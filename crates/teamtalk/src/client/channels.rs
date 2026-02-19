//! Channel management APIs.
use super::Client;
use crate::events::ConnectionState;
use crate::types::{Channel, ChannelId, UserId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

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

impl Client {
    /// Returns available channels from the server.
    pub fn get_server_channels(&self) -> Vec<Channel> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetServerChannels(self.ptr.0, std::ptr::null_mut(), &mut count);
            let mut channels = vec![std::mem::zeroed::<ffi::Channel>(); count as usize];
            if ffi::api().TT_GetServerChannels(self.ptr.0, channels.as_mut_ptr(), &mut count) == 1 {
                channels.into_iter().map(Channel::from).collect()
            } else {
                vec![]
            }
        }
    }

    /// Returns a channel by id.
    pub fn get_channel(&self, id: ChannelId) -> Option<Channel> {
        self.backend().get_channel(self.ptr.0, id.0)
    }

    /// Returns a channel path by id.
    pub fn get_channel_path(&self, id: ChannelId) -> Option<String> {
        use crate::types::TT_STRLEN;
        use crate::utils::strings::tt_buf;
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_GetChannelPath(self.ptr.0, id.0, buf.as_mut_ptr()) == 1 {
                Some(crate::utils::strings::to_string(&buf))
            } else {
                None
            }
        }
    }

    /// Returns a channel id from a path.
    pub fn get_channel_id_from_path(&self, path: &str) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetChannelIDFromPath(self.ptr.0, path.tt().as_ptr()) })
    }

    /// Joins a channel.
    pub fn join_channel(&self, id: ChannelId, password: &str) -> i32 {
        if !can_start_join(self.connection_state()) {
            return 0;
        }
        let cmd_id = self
            .backend()
            .do_join_channel_by_id(self.ptr.0, id.0, password);
        if cmd_id > 0 {
            let mut auto = self.auto_reconnect.lock().unwrap();
            auto.last_channel = Some(id);
            if password.is_empty() {
                auto.last_channel_password = None;
            } else {
                auto.last_channel_password = Some(password.to_string());
            }
            auto.join_gave_up = false;
            self.set_connection_state(ConnectionState::Joining(id));
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
        if cmd_id <= 0 {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "join command rejected in current state".to_string(),
            });
        }
        let waited = self.poll_until(timeout_ms, |event, msg| match event {
            crate::events::Event::UserJoined => msg
                .user()
                .map(|user| user.id == self.my_id())
                .unwrap_or(false),
            crate::events::Event::CmdError => msg.source() == cmd_id,
            _ => false,
        });
        let Some((event, message)) = waited else {
            return Err(crate::events::Error::Timeout);
        };
        if matches!(event, crate::events::Event::CmdError) {
            return Err(crate::events::Error::CommandFailed {
                code: message.source(),
                message: "join command failed".to_string(),
            });
        }
        Ok(message)
    }

    /// Joins a channel by id without a password.
    pub fn join_channel_unprotected(&self, channel_id: ChannelId) -> i32 {
        self.join_channel(channel_id, "")
    }

    /// Joins a channel path.
    pub fn join_channel_path(&self, path: &str, password: &str) -> i32 {
        let id = self.get_channel_id_from_path(path);
        if id.0 > 0 {
            self.join_channel(id, password)
        } else {
            0
        }
    }

    /// Joins a channel path without a password.
    pub fn join_channel_path_unprotected(&self, path: &str) -> i32 {
        self.join_channel_path(path, "")
    }

    /// Leaves the current channel.
    pub fn leave_channel(&self) -> i32 {
        if !can_leave_channel_in_state(self.connection_state()) {
            return 0;
        }
        let cmd_id = self.backend().do_leave_channel(self.ptr.0);
        if cmd_id > 0 {
            let mut auto = self.auto_reconnect.lock().unwrap();
            auto.last_channel = None;
            auto.last_channel_password = None;
            auto.pending_join_cmd = None;
            auto.join_gave_up = false;
        }
        cmd_id
    }

    /// Creates a new channel.
    pub fn make_channel(&self, channel: &Channel) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoMakeChannel(self.ptr.0, &channel.to_ffi()) }
    }

    /// Updates an existing channel.
    pub fn update_channel(&self, channel: &Channel) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoUpdateChannel(self.ptr.0, &channel.to_ffi()) }
    }

    /// Removes a channel.
    pub fn remove_channel(&self, id: ChannelId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoRemoveChannel(self.ptr.0, id.0) }
    }

    /// Moves a user to a different channel.
    pub fn move_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoMoveUser(self.ptr.0, user_id.0, channel_id.0) }
    }

    /// Checks if a user is an operator in a channel.
    pub fn is_channel_operator(&self, user_id: UserId, channel_id: ChannelId) -> bool {
        unsafe { ffi::api().TT_IsChannelOperator(self.ptr.0, user_id.0, channel_id.0) == 1 }
    }

    /// Joins the root channel.
    pub fn join_root(&self) -> i32 {
        let root = ChannelId(unsafe { ffi::api().TT_GetRootChannelID(self.ptr.0) });
        self.join_channel(root, "")
    }

    /// Leaves the current channel and joins the root channel.
    pub fn leave_to_root(&self) -> i32 {
        let _ = self.leave_channel();
        self.join_root()
    }

    /// Returns the channel ID where the current user is.
    pub fn my_channel_id(&self) -> ChannelId {
        self.backend().get_my_channel_id(self.ptr.0)
    }

    /// Returns users in a channel.
    pub fn get_channel_users(&self, channel_id: ChannelId) -> Vec<crate::types::User> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetChannelUsers(
                self.ptr.0,
                channel_id.0,
                std::ptr::null_mut(),
                &mut count,
            );
            let mut users = vec![std::mem::zeroed::<ffi::User>(); count as usize];
            if ffi::api().TT_GetChannelUsers(
                self.ptr.0,
                channel_id.0,
                users.as_mut_ptr(),
                &mut count,
            ) == 1
            {
                users.into_iter().map(crate::types::User::from).collect()
            } else {
                vec![]
            }
        }
    }
}
