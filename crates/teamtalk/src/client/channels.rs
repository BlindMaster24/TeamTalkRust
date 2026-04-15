//! Channel management APIs.
use super::Client;
use crate::events::ConnectionState;
use crate::types::{Channel, ChannelId, CommandId, UserId};
use std::time::{Duration, Instant};

fn can_start_join(state: ConnectionState) -> bool {
    matches!(state, ConnectionState::LoggedIn)
}

fn can_leave_channel_in_state(state: ConnectionState) -> bool {
    matches!(
        state,
        ConnectionState::Joining(_) | ConnectionState::Joined(_)
    )
}

fn can_issue_logged_in_command(state: ConnectionState) -> bool {
    matches!(
        state,
        ConnectionState::LoggedIn | ConnectionState::Joining(_) | ConnectionState::Joined(_)
    )
}

fn wait_slice(deadline: Instant) -> i32 {
    deadline
        .saturating_duration_since(Instant::now())
        .min(Duration::from_millis(50))
        .as_millis()
        .min(i32::MAX as u128) as i32
}

impl Client {
    /// Returns available channels from the server.
    #[must_use]
    pub fn get_server_channels(&self) -> Vec<Channel> {
        self.backend().get_server_channels(self.ptr.0)
    }

    /// Returns a channel by id.
    pub fn get_channel(&self, id: ChannelId) -> Option<Channel> {
        self.backend().get_channel(self.ptr.0, id)
    }

    /// Returns a channel path by id.
    pub fn get_channel_path(&self, id: ChannelId) -> Option<String> {
        self.backend().get_channel_path(self.ptr.0, id)
    }

    /// Returns a channel by path.
    pub fn get_channel_by_path(&self, path: &str) -> Option<Channel> {
        let id = self.get_channel_id_from_path(path);
        (id.raw() > 0).then(|| self.get_channel(id)).flatten()
    }

    /// Returns a channel id from a path.
    #[must_use]
    pub fn get_channel_id_from_path(&self, path: &str) -> ChannelId {
        self.backend().get_channel_id_from_path(self.ptr.0, path)
    }

    /// Joins a channel.
    #[must_use]
    pub fn join_channel(&self, id: ChannelId, password: &str) -> CommandId {
        if !can_start_join(self.connection_state()) {
            return CommandId::ZERO;
        }
        let cmd_id = CommandId(
            self.backend()
                .do_join_channel_by_id(self.ptr.0, id, password),
        );
        if cmd_id.is_ok() {
            let mut auto = self.auto_reconnect.lock();

            auto.last_channel = Some(id);
            if password.is_empty() {
                auto.last_channel_password = None;
            } else {
                auto.last_channel_password = Some(password.to_string());
            }
            auto.join_gave_up = false;
            self.set_connection_state(ConnectionState::Joining(id));
            drop(auto);
            self.mark_join_phase_started(cmd_id.raw());
        }
        cmd_id
    }

    /// Joins a channel and waits for join completion or command error.
    pub fn join_channel_and_wait(
        &self,
        id: ChannelId,
        password: &str,
        timeout_ms: i32,
    ) -> Result<super::Message, crate::events::Error> {
        let cmd_id = self.join_channel(id, password);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "join command rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::UserJoined
                            if message.user().is_some_and(|user| user.id == self.my_id()) =>
                        {
                            return Ok(message);
                        }
                        crate::events::Event::CmdError if cmd_id == message.source() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "join command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
                if self.connection_state() != ConnectionState::Joining(id) {
                    return Err(crate::events::Error::CommandFailed {
                        code: 0,
                        message: "join phase aborted before completion".to_string(),
                    });
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(crate::events::Error::Timeout);
            }
            if let Some((event, message)) = self.poll(wait_ms) {
                match event {
                    crate::events::Event::UserJoined
                        if message.user().is_some_and(|user| user.id == self.my_id()) =>
                    {
                        return Ok(message);
                    }
                    crate::events::Event::CmdError if cmd_id == message.source() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "join command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
            if self.connection_state() != ConnectionState::Joining(id) {
                return Err(crate::events::Error::CommandFailed {
                    code: 0,
                    message: "join phase aborted before completion".to_string(),
                });
            }
        }
    }

    /// Joins a channel by id without a password.
    #[must_use]
    pub fn join_channel_unprotected(&self, channel_id: ChannelId) -> CommandId {
        self.join_channel(channel_id, "")
    }

    /// Joins a channel path.
    #[must_use]
    pub fn join_channel_path(&self, path: &str, password: &str) -> CommandId {
        let id = self.get_channel_id_from_path(path);
        if id.raw() > 0 {
            self.join_channel(id, password)
        } else {
            CommandId::ZERO
        }
    }

    /// Joins a channel path without a password.
    #[must_use]
    pub fn join_channel_path_unprotected(&self, path: &str) -> CommandId {
        self.join_channel_path(path, "")
    }

    /// Leaves the current channel.
    #[must_use]
    pub fn leave_channel(&self) -> CommandId {
        if !can_leave_channel_in_state(self.connection_state()) {
            return CommandId::ZERO;
        }
        let cmd_id = CommandId(self.backend().do_leave_channel(self.ptr.0));
        if cmd_id.is_ok() {
            let mut auto = self.auto_reconnect.lock();

            auto.last_channel = None;
            auto.last_channel_password = None;
            auto.clear_join_phase();
            auto.join_gave_up = false;
        }
        cmd_id
    }

    /// Creates a new channel.
    #[must_use]
    pub fn make_channel(&self, channel: &Channel) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_make_channel(self.ptr.0, channel))
    }

    /// Updates an existing channel.
    #[must_use]
    pub fn update_channel(&self, channel: &Channel) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_update_channel(self.ptr.0, channel))
    }

    /// Removes a channel.
    #[must_use]
    pub fn remove_channel(&self, id: ChannelId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_remove_channel(self.ptr.0, id))
    }

    /// Moves a user to a different channel.
    #[must_use]
    pub fn move_user(&self, user_id: UserId, channel_id: ChannelId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_move_user(self.ptr.0, user_id, channel_id))
    }

    /// Checks if a user is an operator in a channel.
    #[must_use]
    pub fn is_channel_operator(&self, user_id: UserId, channel_id: ChannelId) -> bool {
        self.backend()
            .is_channel_operator(self.ptr.0, user_id, channel_id)
    }

    /// Joins the root channel.
    #[must_use]
    pub fn join_root(&self) -> CommandId {
        let root = self.backend().get_root_channel_id(self.ptr.0);
        self.join_channel(root, "")
    }

    /// Leaves the current channel and joins the root channel.
    #[must_use]
    pub fn leave_to_root(&self) -> CommandId {
        let _ = self.leave_channel();
        self.join_root()
    }

    /// Returns the channel ID where the current user is.
    #[must_use]
    pub fn my_channel_id(&self) -> ChannelId {
        self.backend().get_my_channel_id(self.ptr.0)
    }

    /// Returns the current channel for the local user, if any.
    pub fn my_channel(&self) -> Option<Channel> {
        let id = self.my_channel_id();
        (id.raw() > 0).then(|| self.get_channel(id)).flatten()
    }

    /// Returns users in a channel.
    #[must_use]
    pub fn get_channel_users(&self, channel_id: ChannelId) -> Vec<crate::types::User> {
        self.backend().get_channel_users(self.ptr.0, channel_id)
    }
}
