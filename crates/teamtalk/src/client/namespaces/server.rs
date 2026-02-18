//! Server management namespace.
use super::define_namespace;
#[cfg(feature = "async")]
use crate::events::Result;
use crate::types::{ChannelId, ClientStatistics, ServerProperties, User};

define_namespace!(ServerNamespace);

impl ServerNamespace {
    /// Returns current server properties.
    pub fn properties(&self) -> Option<ServerProperties> {
        self.client.get_server_properties()
    }

    /// Returns all users on the server.
    pub fn users(&self) -> Vec<User> {
        self.client.get_server_users()
    }

    /// Bans an IP address.
    pub fn ban_ip(&self, ip: &str, ban_type: i32) -> i32 {
        self.client.ban_ip(ip, ban_type)
    }

    /// Returns client statistics.
    pub fn client_statistics(&self) -> Option<ClientStatistics> {
        self.client.get_client_statistics()
    }

    /// Requests a list of bans.
    pub fn list_bans(&self, channel_id: ChannelId, index: i32, count: i32) -> i32 {
        self.client.list_bans(channel_id, index, count)
    }

    /// Updates server properties.
    pub fn update(&self, props: &ServerProperties) -> i32 {
        self.client.update_server(props)
    }

    /// Saves the server configuration.
    pub fn save_config(&self) -> i32 {
        self.client.save_server_config()
    }

    /// Returns the root channel ID.
    pub fn root_channel_id(&self) -> ChannelId {
        self.client.get_root_channel_id()
    }

    /// Requests server statistics.
    pub fn query_stats(&self) -> i32 {
        self.client.query_server_stats()
    }

    /// Pings the server.
    pub fn ping(&self) -> i32 {
        self.client.ping()
    }

    /// Queries the max payload supported by the server.
    pub fn query_max_payload(&self) -> bool {
        self.client.query_server_max_payload()
    }

    /// Quits the TeamTalk client.
    pub fn quit(&self) -> i32 {
        self.client.quit()
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;
#[cfg(feature = "async")]
use crate::events::Event;

#[cfg(feature = "async")]
define_async_namespace!(AsyncServerNamespace);

#[cfg(feature = "async")]
impl AsyncServerNamespace {
    /// Returns current server properties (from cache).
    pub fn properties(&self) -> Option<ServerProperties> {
        self.client.get_server_properties()
    }

    /// Updates server properties and waits for confirmation.
    #[cfg(feature = "async")]
    pub async fn update(&self, props: &ServerProperties) -> Result<ServerProperties> {
        self.client
            .execute_command(Event::ServerUpdate, || self.client.update_server(props))
            .await
    }

    /// Pings the server.
    pub fn ping(&self) -> i32 {
        self.client.ping()
    }

    /// Returns client statistics.
    pub fn client_statistics(&self) -> Option<ClientStatistics> {
        self.client.get_client_statistics()
    }
}
