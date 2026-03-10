//! Server management APIs.
use super::Client;
use crate::types::{ChannelId, ServerProperties, User, UserId};
use teamtalk_sys as ffi;

fn can_issue_logged_in_command(state: crate::events::ConnectionState) -> bool {
    matches!(
        state,
        crate::events::ConnectionState::LoggedIn
            | crate::events::ConnectionState::Joining(_)
            | crate::events::ConnectionState::Joined(_)
    )
}

impl Client {
    /// Returns current server properties.
    pub fn get_server_properties(&self) -> Option<ServerProperties> {
        self.backend().get_server_properties(self.ptr.0)
    }

    /// Returns all users on the server.
    pub fn get_server_users(&self) -> Vec<User> {
        self.backend().get_server_users(self.ptr.0)
    }

    /// Bans an IP address.
    pub fn ban_ip(&self, ip: &str, ban_type: i32) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend().do_ban_ip_address(self.ptr.0, ip, ban_type)
    }

    /// Returns client statistics.
    pub fn get_client_statistics(&self) -> Option<crate::types::ClientStatistics> {
        self.backend().get_client_statistics(self.ptr.0)
    }

    /// Requests a list of bans.
    pub fn list_bans(&self, channel_id: ChannelId, index: i32, count: i32) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend()
            .do_list_bans(self.ptr.0, channel_id.0, index, count)
    }

    /// Updates server properties.
    pub fn update_server(&self, props: &ServerProperties) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend().do_update_server(self.ptr.0, props)
    }

    /// Saves the server configuration.
    pub fn save_server_config(&self) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        self.backend().do_save_config(self.ptr.0)
    }

    /// Returns the root channel ID.
    pub fn get_root_channel_id(&self) -> ChannelId {
        self.backend().get_root_channel_id(self.ptr.0)
    }

    /// Requests server statistics.
    pub fn query_server_stats(&self) -> i32 {
        if !can_issue_logged_in_command(self.connection_state()) {
            return 0;
        }
        unsafe { ffi::api().TT_DoQueryServerStats(self.ptr.0) }
    }

    /// Pings the server and waits for processing events.
    pub fn ping(&self) -> i32 {
        unsafe { ffi::api().TT_DoPing(self.ptr.0) }
    }

    /// Queries the max payload for a user.
    ///
    /// Note: current TeamTalk SDK versions only support querying the
    /// server payload (`user_id = 0`). For that default path use
    /// `query_server_max_payload()`.
    pub fn query_max_payload(&self, user_id: UserId) -> bool {
        unsafe { ffi::api().TT_QueryMaxPayload(self.ptr.0, user_id.0) == 1 }
    }

    /// Queries the max payload supported by the server (`user_id = 0`).
    pub fn query_server_max_payload(&self) -> bool {
        unsafe { ffi::api().TT_QueryMaxPayload(self.ptr.0, 0) == 1 }
    }

    /// Pumps a message into the Windows message loop (Windows only).
    #[cfg(windows)]
    pub fn pump_message(&self, event: ffi::ClientEvent, id: i32) -> bool {
        unsafe { ffi::api().TT_PumpMessage(self.ptr.0, event, id) == 1 }
    }

    /// Quits the TeamTalk client (for standalone apps).
    pub fn quit(&self) -> i32 {
        unsafe { ffi::api().TT_DoQuit(self.ptr.0) }
    }
}
