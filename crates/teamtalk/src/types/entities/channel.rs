use crate::types::{AudioCodec, AudioConfig, ChannelId, ChannelType, UserId};
use teamtalk_sys as ffi;

/// Channel definition and configuration.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Channel {
    pub id: ChannelId,
    pub parent_id: ChannelId,
    pub name: String,
    pub topic: String,
    pub channel_type: ChannelType,
    pub has_password: bool,
    pub user_data: i32,
    pub disk_quota: i64,
    pub max_users: i32,
    pub audio_codec: AudioCodec,
    pub audio_cfg: AudioConfig,
    pub transmit_users: Vec<(UserId, u32)>,
    pub transmit_users_queue: Vec<UserId>,
    pub queue_delay_ms: i32,
    pub timeout_voice_ms: i32,
    pub timeout_media_ms: i32,
}

impl Channel {
    /// Creates a channel builder with a name.
    #[must_use]
    pub fn builder(name: &str) -> ChannelBuilder {
        ChannelBuilder::new(name)
    }
}

/// Builder for channel configuration.
pub struct ChannelBuilder {
    inner: Channel,
}

impl ChannelBuilder {
    /// Creates a new builder with the channel name.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            inner: Channel {
                id: ChannelId(0),
                parent_id: ChannelId(0),
                name: name.to_string(),
                topic: String::new(),
                channel_type: ChannelType::default(),
                has_password: false,
                user_data: 0,
                disk_quota: 0,
                max_users: 0,
                audio_codec: AudioCodec::default(),
                audio_cfg: AudioConfig::default(),
                transmit_users: Vec::new(),
                transmit_users_queue: Vec::new(),
                queue_delay_ms: 0,
                timeout_voice_ms: 0,
                timeout_media_ms: 0,
            },
        }
    }

    /// Sets the parent channel id.
    #[must_use]
    pub fn parent(mut self, id: ChannelId) -> Self {
        self.inner.parent_id = id;
        self
    }

    /// Sets the channel topic.
    #[must_use]
    pub fn topic(mut self, topic: &str) -> Self {
        self.inner.topic = topic.to_string();
        self
    }

    /// Sets the channel type flags.
    #[must_use]
    pub fn channel_type(mut self, t: ChannelType) -> Self {
        self.inner.channel_type = t;
        self
    }

    /// Sets the maximum number of users.
    #[must_use]
    pub fn max_users(mut self, max: i32) -> Self {
        self.inner.max_users = max;
        self
    }

    /// Sets the audio codec configuration.
    #[must_use]
    pub fn codec(mut self, codec: AudioCodec) -> Self {
        self.inner.audio_codec = codec;
        self
    }

    /// Builds the channel.
    #[must_use]
    pub fn build(self) -> Channel {
        self.inner
    }
}

impl From<ffi::Channel> for Channel {
    fn from(c: ffi::Channel) -> Self {
        let mut transmit_users = Vec::new();
        for i in 0..128 {
            let id = c.transmitUsers[i][0];
            if id == 0 {
                break;
            }
            transmit_users.push((UserId(id), c.transmitUsers[i][1] as u32));
        }
        let transmit_users_queue = c
            .transmitUsersQueue
            .iter()
            .take_while(|&&id| id != 0)
            .map(|&id| UserId(id))
            .collect();
        Self {
            id: ChannelId(c.nChannelID),
            parent_id: ChannelId(c.nParentID),
            name: crate::utils::strings::to_string(&c.szName),
            topic: crate::utils::strings::to_string(&c.szTopic),
            channel_type: ChannelType::from_raw(c.uChannelType),
            has_password: c.bPassword != 0,
            user_data: c.nUserData,
            disk_quota: c.nDiskQuota,
            max_users: c.nMaxUsers,
            audio_codec: AudioCodec::from(c.audiocodec),
            audio_cfg: AudioConfig::from(c.audiocfg),
            transmit_users,
            transmit_users_queue,
            queue_delay_ms: c.nTransmitUsersQueueDelayMSec,
            timeout_voice_ms: c.nTimeOutTimerVoiceMSec,
            timeout_media_ms: c.nTimeOutTimerMediaFileMSec,
        }
    }
}

impl Channel {
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::Channel {
        let mut raw = ffi::Channel {
            nChannelID: self.id.raw(),
            nParentID: self.parent_id.raw(),
            ..Default::default()
        };
        let n = crate::utils::ToTT::tt(&self.name);
        let t = crate::utils::ToTT::tt(&self.topic);
        unsafe {
            let n_len = n.len().min(511);
            std::ptr::copy_nonoverlapping(n.as_ptr(), raw.szName.as_mut_ptr(), n_len);
            let t_len = t.len().min(511);
            std::ptr::copy_nonoverlapping(t.as_ptr(), raw.szTopic.as_mut_ptr(), t_len);
        }
        raw.uChannelType = self.channel_type.raw();
        raw.nUserData = self.user_data;
        raw.nDiskQuota = self.disk_quota;
        raw.nMaxUsers = self.max_users;
        raw.audiocodec = self.audio_codec.to_ffi();
        raw.audiocfg = self.audio_cfg.to_ffi();
        raw.nTransmitUsersQueueDelayMSec = self.queue_delay_ms;
        raw.nTimeOutTimerVoiceMSec = self.timeout_voice_ms;
        raw.nTimeOutTimerMediaFileMSec = self.timeout_media_ms;
        for (i, (uid, types)) in self.transmit_users.iter().take(128).enumerate() {
            raw.transmitUsers[i][0] = uid.raw();
            raw.transmitUsers[i][1] = *types as i32;
        }
        for (i, id) in self.transmit_users_queue.iter().take(16).enumerate() {
            raw.transmitUsersQueue[i] = id.raw();
        }
        raw
    }
}
