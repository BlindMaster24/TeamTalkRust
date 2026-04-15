//! Server management APIs.
use super::Client;
use crate::types::{ChannelId, CommandId, ServerProperties, User, UserId};
use std::time::{Duration, Instant};
#[cfg(windows)]
use teamtalk_sys as ffi;

fn can_issue_logged_in_command(state: crate::events::ConnectionState) -> bool {
    matches!(
        state,
        crate::events::ConnectionState::LoggedIn
            | crate::events::ConnectionState::Joining(_)
            | crate::events::ConnectionState::Joined(_)
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
    /// Waits for command success or command error for a specific command id.
    pub fn wait_for_command(
        &self,
        cmd_id: CommandId,
        timeout_ms: i32,
    ) -> Result<(), crate::events::Error> {
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "command rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::CmdSuccess if cmd_id == message.source() => {
                            return Ok(());
                        }
                        crate::events::Event::CmdError if cmd_id == message.source() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
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
                    crate::events::Event::CmdSuccess if cmd_id == message.source() => return Ok(()),
                    crate::events::Event::CmdError if cmd_id == message.source() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Returns current server properties.
    pub fn get_server_properties(&self) -> Option<ServerProperties> {
        self.backend().get_server_properties(self.ptr.0)
    }

    /// Returns all users on the server.
    pub fn get_server_users(&self) -> Vec<User> {
        self.backend().get_server_users(self.ptr.0)
    }

    /// Bans an IP address.
    pub fn ban_ip(&self, ip: &str, ban_type: i32) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_ban_ip_address(self.ptr.0, ip, ban_type))
    }

    /// Returns client statistics.
    pub fn get_client_statistics(&self) -> Option<crate::types::ClientStatistics> {
        self.backend().get_client_statistics(self.ptr.0)
    }

    /// Requests a list of bans.
    pub fn list_bans(&self, channel_id: ChannelId, index: i32, count: i32) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(
            self.backend()
                .do_list_bans(self.ptr.0, channel_id.0, index, count),
        )
    }

    /// Requests a list of bans and waits for the matching list to complete.
    pub fn list_bans_and_wait(
        &self,
        channel_id: ChannelId,
        index: i32,
        count: i32,
        timeout_ms: i32,
    ) -> Result<Vec<crate::types::BannedUser>, crate::events::Error> {
        let cmd_id = self.list_bans(channel_id, index, count);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "ban list command rejected in current state".to_string(),
            });
        }
        let mut items = Vec::new();
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::BannedUser => {
                            if let Some(entry) = message.banned_user() {
                                items.push(entry);
                            }
                        }
                        crate::events::Event::CmdSuccess if cmd_id == message.source() => {
                            return Ok(items);
                        }
                        crate::events::Event::CmdError if cmd_id == message.source() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "ban list command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
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
                    crate::events::Event::BannedUser => {
                        if let Some(entry) = message.banned_user() {
                            items.push(entry);
                        }
                    }
                    crate::events::Event::CmdSuccess if cmd_id == message.source() => {
                        return Ok(items);
                    }
                    crate::events::Event::CmdError if cmd_id == message.source() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "ban list command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Updates server properties.
    pub fn update_server(&self, props: &ServerProperties) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_update_server(self.ptr.0, props))
    }

    /// Updates server properties and waits for the updated server state event.
    pub fn update_server_and_wait(
        &self,
        props: &ServerProperties,
        timeout_ms: i32,
    ) -> Result<ServerProperties, crate::events::Error> {
        let cmd_id = self.update_server(props);
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "server update command rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::ServerUpdate => {
                            if let Some(updated) = message.server_properties() {
                                return Ok(updated);
                            }
                        }
                        crate::events::Event::CmdError if cmd_id == message.source() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "server update command failed".to_string(),
                            });
                        }
                        _ => {}
                    }
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
                    crate::events::Event::ServerUpdate => {
                        if let Some(updated) = message.server_properties() {
                            return Ok(updated);
                        }
                    }
                    crate::events::Event::CmdError if cmd_id == message.source() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "server update command failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Saves the server configuration.
    pub fn save_server_config(&self) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_save_config(self.ptr.0))
    }

    /// Saves the server configuration and waits for command completion.
    pub fn save_server_config_and_wait(&self, timeout_ms: i32) -> Result<(), crate::events::Error> {
        let cmd_id = self.save_server_config();
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "save server config command rejected in current state".to_string(),
            });
        }
        self.wait_for_command(cmd_id, timeout_ms)
    }

    /// Returns the root channel ID.
    pub fn get_root_channel_id(&self) -> ChannelId {
        self.backend().get_root_channel_id(self.ptr.0)
    }

    /// Requests server statistics.
    pub fn query_server_stats(&self) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(self.backend().do_query_server_stats(self.ptr.0))
    }

    /// Requests server statistics and waits for the statistics event or command error.
    pub fn query_server_stats_and_wait(
        &self,
        timeout_ms: i32,
    ) -> Result<crate::client::Message, crate::events::Error> {
        let cmd_id = self.query_server_stats();
        if !cmd_id.is_ok() {
            return Err(crate::events::Error::CommandFailed {
                code: 0,
                message: "server statistics query rejected in current state".to_string(),
            });
        }
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50) {
                    match event {
                        crate::events::Event::ServerStatistics => return Ok(message),
                        crate::events::Event::CmdError if cmd_id == message.source() => {
                            return Err(crate::events::Error::CommandFailed {
                                code: message.source(),
                                message: "server statistics query failed".to_string(),
                            });
                        }
                        _ => {}
                    }
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
                    crate::events::Event::ServerStatistics => return Ok(message),
                    crate::events::Event::CmdError if cmd_id == message.source() => {
                        return Err(crate::events::Error::CommandFailed {
                            code: message.source(),
                            message: "server statistics query failed".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Pings the server and waits for processing events.
    pub fn ping(&self) -> CommandId {
        CommandId(self.backend().do_ping(self.ptr.0))
    }

    /// Queries the max payload for a user.
    ///
    /// Note: current TeamTalk SDK versions only support querying the
    /// server payload (`user_id = 0`). For that default path use
    /// `query_server_max_payload()`.
    pub fn query_max_payload(&self, user_id: UserId) -> bool {
        self.backend().query_max_payload(self.ptr.0, user_id.0)
    }

    /// Queries the max payload supported by the server (`user_id = 0`).
    pub fn query_server_max_payload(&self) -> bool {
        self.backend().query_max_payload(self.ptr.0, 0)
    }

    /// Pumps a message into the Windows message loop (Windows only).
    #[cfg(windows)]
    pub fn pump_message(&self, event: ffi::ClientEvent, id: i32) -> bool {
        unsafe { ffi::api().TT_PumpMessage(self.ptr.0, event, id) == 1 }
    }

    /// Quits the TeamTalk client (for standalone apps).
    pub fn quit(&self) -> CommandId {
        CommandId(self.backend().do_quit(self.ptr.0))
    }
}
