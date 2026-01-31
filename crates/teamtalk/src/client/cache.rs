use super::Client;
use crate::events::Event;
use crate::types::{Channel, ChannelId, ServerProperties, ServerStatistics, User, UserId};
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct CacheState {
    users: HashMap<UserId, User>,
    channels: HashMap<ChannelId, Channel>,
    server_props: Option<ServerProperties>,
    server_stats: Option<ServerStatistics>,
    users_auto: bool,
    channels_auto: bool,
}

/// Aggregated server snapshot from cached data.
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub properties: Option<ServerProperties>,
    pub statistics: Option<ServerStatistics>,
    pub users: Vec<User>,
    pub channels: Vec<Channel>,
}

impl Client {
    /// Enables the user cache. When auto-sync is true, events update the cache.
    pub fn enable_user_cache(&self, auto_sync: bool) {
        let mut cache = self.cache.borrow_mut();
        cache.users_auto = auto_sync;
    }

    /// Enables the channel cache. When auto-sync is true, events update the cache.
    pub fn enable_channel_cache(&self, auto_sync: bool) {
        let mut cache = self.cache.borrow_mut();
        cache.channels_auto = auto_sync;
    }

    /// Clears the cached users.
    pub fn clear_user_cache(&self) {
        self.cache.borrow_mut().users.clear();
    }

    /// Clears the cached channels.
    pub fn clear_channel_cache(&self) {
        self.cache.borrow_mut().channels.clear();
    }

    /// Refreshes the user cache from the server.
    pub fn refresh_user_cache(&self) -> Vec<User> {
        let users = self.get_server_users();
        let mut cache = self.cache.borrow_mut();
        cache.users = users.iter().map(|u| (u.id, u.clone())).collect();
        users
    }

    /// Refreshes the channel cache from the server.
    pub fn refresh_channel_cache(&self) -> Vec<Channel> {
        let channels = self.get_server_channels();
        let mut cache = self.cache.borrow_mut();
        cache.channels = channels.iter().map(|c| (c.id, c.clone())).collect();
        channels
    }

    /// Returns a cached user, if present.
    pub fn cached_user(&self, user_id: UserId) -> Option<User> {
        self.cache.borrow().users.get(&user_id).cloned()
    }

    /// Returns all cached users.
    pub fn cached_users(&self) -> Vec<User> {
        self.cache.borrow().users.values().cloned().collect()
    }

    /// Returns a cached user by username, if present.
    pub fn cached_user_by_username(&self, username: &str) -> Option<User> {
        self.cache
            .borrow()
            .users
            .values()
            .find(|u| u.username == username)
            .cloned()
    }

    /// Returns a cached channel, if present.
    pub fn cached_channel(&self, channel_id: ChannelId) -> Option<Channel> {
        self.cache.borrow().channels.get(&channel_id).cloned()
    }

    /// Returns all cached channels.
    pub fn cached_channels(&self) -> Vec<Channel> {
        self.cache.borrow().channels.values().cloned().collect()
    }

    /// Returns a cached channel by name, if present.
    pub fn cached_channel_by_name(&self, name: &str) -> Option<Channel> {
        self.cache
            .borrow()
            .channels
            .values()
            .find(|c| c.name == name)
            .cloned()
    }

    /// Returns a cached channel by path, if present.
    pub fn cached_channel_by_path(&self, path: &str) -> Option<Channel> {
        self.cache
            .borrow()
            .channels
            .values()
            .find(|c| self.get_channel_path(c.id) == path)
            .cloned()
    }

    /// Returns the last cached server properties and statistics.
    pub fn server_info(&self) -> ServerInfo {
        let cache = self.cache.borrow();
        ServerInfo {
            properties: cache.server_props.clone(),
            statistics: cache.server_stats.clone(),
            users: cache.users.values().cloned().collect(),
            channels: cache.channels.values().cloned().collect(),
        }
    }

    pub(crate) fn update_cache_for_event(&self, event: Event, msg: &super::Message) {
        let mut cache = self.cache.borrow_mut();
        match event {
            Event::UserLoggedIn | Event::UserUpdate | Event::UserJoined | Event::UserLeft => {
                if cache.users_auto
                    && let Some(user) = msg.user()
                {
                    cache.users.insert(user.id, user);
                }
            }
            Event::UserLoggedOut => {
                if cache.users_auto
                    && let Some(user) = msg.user()
                {
                    cache.users.remove(&user.id);
                }
            }
            Event::ChannelCreated | Event::ChannelUpdated => {
                if cache.channels_auto
                    && let Some(channel) = msg.channel()
                {
                    cache.channels.insert(channel.id, channel);
                }
            }
            Event::ChannelRemoved => {
                if cache.channels_auto
                    && let Some(channel) = msg.channel()
                {
                    cache.channels.remove(&channel.id);
                }
            }
            Event::ServerUpdate => {
                cache.server_props = msg.server_properties();
            }
            Event::ServerStatistics => {
                cache.server_stats = msg.server_statistics();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {
    use crate::client::backend::MockBackend;
    use crate::client::{Client, Message};
    use crate::events::Event;
    use crate::types::{Channel, ChannelId, ServerProperties, ServerStatistics, UserId};
    use teamtalk_sys as ffi;

    fn message_from_user(user_id: i32, event: Event) -> Message {
        let mut user = unsafe { std::mem::zeroed::<ffi::User>() };
        user.nUserID = user_id;
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nSource = user_id;
        msg.ttType = ffi::TTType::__USER;
        msg.__bindgen_anon_1.user = user;
        Message::from_raw(event, msg)
    }

    fn message_from_channel(channel: Channel, event: Event) -> Message {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nSource = channel.id.0;
        msg.ttType = ffi::TTType::__CHANNEL;
        msg.__bindgen_anon_1.channel = channel.to_ffi();
        Message::from_raw(event, msg)
    }

    fn message_from_server_props(props: ServerProperties) -> Message {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.ttType = ffi::TTType::__SERVERPROPERTIES;
        msg.__bindgen_anon_1.serverproperties = props.to_ffi();
        Message::from_raw(Event::ServerUpdate, msg)
    }

    fn message_from_server_stats(stats: ServerStatistics) -> Message {
        let raw = ffi::ServerStatistics {
            nTotalBytesTX: stats.total_tx,
            nTotalBytesRX: stats.total_rx,
            nVoiceBytesTX: stats.voice_tx,
            nVoiceBytesRX: stats.voice_rx,
            nVideoCaptureBytesTX: stats.video_tx,
            nVideoCaptureBytesRX: stats.video_rx,
            nMediaFileBytesTX: stats.media_tx,
            nMediaFileBytesRX: stats.media_rx,
            nDesktopBytesTX: stats.desktop_tx,
            nDesktopBytesRX: stats.desktop_rx,
            nUsersServed: stats.users_served,
            nUsersPeak: stats.users_peak,
            nFilesTx: stats.files_tx,
            nFilesRx: stats.files_rx,
            nUptimeMSec: stats.uptime_ms,
        };
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.ttType = ffi::TTType::__SERVERSTATISTICS;
        msg.__bindgen_anon_1.serverstatistics = raw;
        Message::from_raw(Event::ServerStatistics, msg)
    }

    fn test_channel(id: i32, name: &str) -> Channel {
        let mut channel = Channel::builder(name).build();
        channel.id = ChannelId(id);
        channel
    }

    fn test_props() -> ServerProperties {
        ServerProperties {
            name: "srv".to_string(),
            motd: "motd".to_string(),
            motd_raw: "motd".to_string(),
            max_users: 10,
            max_login_attempts: 3,
            max_logins_per_ip: 2,
            max_voice_tx: 1,
            max_video_tx: 1,
            max_media_tx: 1,
            max_desktop_tx: 1,
            max_total_tx: 1,
            auto_save: true,
            tcp_port: 10333,
            udp_port: 10333,
            user_timeout: 60,
            version: "v".to_string(),
            protocol_version: "p".to_string(),
            login_delay: 0,
            access_token: "token".to_string(),
            log_events: 0,
        }
    }

    fn test_stats() -> ServerStatistics {
        ServerStatistics {
            total_tx: 1,
            total_rx: 2,
            voice_tx: 3,
            voice_rx: 4,
            video_tx: 5,
            video_rx: 6,
            media_tx: 7,
            media_rx: 8,
            desktop_tx: 9,
            desktop_rx: 10,
            users_served: 11,
            users_peak: 12,
            files_tx: 13,
            files_rx: 14,
            uptime_ms: 15,
        }
    }

    #[test]
    fn cache_updates_users_and_channels() {
        let backend = std::sync::Arc::new(MockBackend::new());
        let client = Client::with_backend(backend).expect("client");
        client.enable_user_cache(true);
        client.enable_channel_cache(true);

        let user_msg = message_from_user(1, Event::UserLoggedIn);
        client.update_cache_for_event(Event::UserLoggedIn, &user_msg);
        assert!(client.cached_user(UserId(1)).is_some());

        let channel = test_channel(5, "lobby");
        let channel_msg = message_from_channel(channel.clone(), Event::ChannelCreated);
        client.update_cache_for_event(Event::ChannelCreated, &channel_msg);
        assert!(client.cached_channel(channel.id).is_some());

        client.update_cache_for_event(Event::UserLoggedOut, &user_msg);
        assert!(client.cached_user(UserId(1)).is_none());

        client.update_cache_for_event(Event::ChannelRemoved, &channel_msg);
        assert!(client.cached_channel(channel.id).is_none());
    }

    #[test]
    fn cache_updates_server_info() {
        let backend = std::sync::Arc::new(MockBackend::new());
        let client = Client::with_backend(backend).expect("client");

        let props = test_props();
        let props_msg = message_from_server_props(props.clone());
        client.update_cache_for_event(Event::ServerUpdate, &props_msg);

        let stats = test_stats();
        let stats_msg = message_from_server_stats(stats.clone());
        client.update_cache_for_event(Event::ServerStatistics, &stats_msg);

        let info = client.server_info();
        assert!(info.properties.is_some());
        assert!(info.statistics.is_some());
        assert_eq!(info.properties.unwrap().name, props.name);
        assert_eq!(info.statistics.unwrap().total_tx, stats.total_tx);
    }
}
