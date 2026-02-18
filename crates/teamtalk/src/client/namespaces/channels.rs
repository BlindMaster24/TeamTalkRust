//! Channel management namespace.
use super::define_namespace;
#[cfg(feature = "async")]
use crate::events::Result;
use crate::types::{Channel, ChannelId, User, UserId};

define_namespace!(ChannelsNamespace);

impl ChannelsNamespace {
    /// Returns available channels from the server.
    pub fn list(&self) -> Vec<Channel> {
        self.client.get_server_channels()
    }

    /// Returns a channel by id.
    pub fn get(&self, id: ChannelId) -> Option<Channel> {
        self.client.get_channel(id)
    }

    /// Returns a channel path by id.
    pub fn get_path(&self, id: ChannelId) -> Option<String> {
        self.client.get_channel_path(id)
    }

    /// Returns a channel id from a path.
    pub fn get_id_from_path(&self, path: &str) -> ChannelId {
        self.client.get_channel_id_from_path(path)
    }

    /// Joins a channel.
    pub fn join(&self, id: ChannelId, password: &str) -> i32 {
        self.client.join_channel(id, password)
    }

    /// Joins a channel by id without a password.
    pub fn join_unprotected(&self, channel_id: ChannelId) -> i32 {
        self.client.join_channel_unprotected(channel_id)
    }

    /// Joins a channel path.
    pub fn join_path(&self, path: &str, password: &str) -> i32 {
        self.client.join_channel_path(path, password)
    }

    /// Joins a channel path without a password.
    pub fn join_path_unprotected(&self, path: &str) -> i32 {
        self.client.join_channel_path_unprotected(path)
    }

    /// Leaves the current channel.
    pub fn leave(&self) -> i32 {
        self.client.leave_channel()
    }

    /// Creates a new channel.
    pub fn create(&self, channel: &Channel) -> i32 {
        self.client.make_channel(channel)
    }

    /// Updates an existing channel.
    pub fn update(&self, channel: &Channel) -> i32 {
        self.client.update_channel(channel)
    }

    /// Removes a channel.
    pub fn remove(&self, id: ChannelId) -> i32 {
        self.client.remove_channel(id)
    }

    /// Moves a user to a different channel.
    pub fn move_user(&self, user_id: UserId, channel_id: ChannelId) -> i32 {
        self.client.move_user(user_id, channel_id)
    }

    /// Checks if a user is an operator in a channel.
    pub fn is_operator(&self, user_id: UserId, channel_id: ChannelId) -> bool {
        self.client.is_channel_operator(user_id, channel_id)
    }

    /// Joins the root channel.
    pub fn join_root(&self) -> i32 {
        self.client.join_root()
    }

    /// Leaves the current channel and joins the root channel.
    pub fn leave_to_root(&self) -> i32 {
        self.client.leave_to_root()
    }

    /// Returns the channel ID where the current user is.
    pub fn my_id(&self) -> ChannelId {
        self.client.my_channel_id()
    }

    /// Returns users in a channel.
    pub fn users(&self, channel_id: ChannelId) -> Vec<User> {
        self.client.get_channel_users(channel_id)
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;
#[cfg(feature = "async")]
use crate::events::Event;

#[cfg(feature = "async")]
define_async_namespace!(AsyncChannelsNamespace);

#[cfg(feature = "async")]
impl AsyncChannelsNamespace {
    /// Returns available channels (from cache).
    pub fn list(&self) -> Vec<Channel> {
        self.client.get_server_channels()
    }

    /// Joins a channel and waits for success.
    #[cfg(feature = "async")]
    pub async fn join(&self, id: ChannelId, password: &str) -> Result<Channel> {
        self.client
            .execute_command(Event::UserJoined, || self.client.join_channel(id, password))
            .await
    }

    /// Leaves the current channel and waits for success.
    #[cfg(feature = "async")]
    pub async fn leave(&self) -> Result<()> {
        self.client
            .execute_void_command(Event::UserLeft, || self.client.leave_channel())
            .await
    }

    /// Creates a new channel and waits for success.
    #[cfg(feature = "async")]
    pub async fn create(&self, channel: &Channel) -> Result<Channel> {
        self.client
            .execute_command(Event::ChannelCreated, || self.client.make_channel(channel))
            .await
    }

    /// Removes a channel and waits for success.
    #[cfg(feature = "async")]
    pub async fn remove(&self, id: ChannelId) -> Result<Channel> {
        self.client
            .execute_command(Event::ChannelRemoved, || self.client.remove_channel(id))
            .await
    }

    /// Moves a user and waits for success.
    #[cfg(feature = "async")]
    pub async fn move_user(&self, user_id: UserId, channel_id: ChannelId) -> Result<User> {
        self.client
            .execute_command(Event::UserJoined, || {
                self.client.move_user(user_id, channel_id)
            })
            .await
    }

    /// Returns the channel ID where the current user is.
    pub fn my_id(&self) -> ChannelId {
        self.client.my_channel_id()
    }
}
