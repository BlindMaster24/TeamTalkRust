use teamtalk_sys as ffi;

/// Server properties snapshot.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ServerProperties {
    pub name: String,
    pub motd: String,
    pub motd_raw: String,
    pub max_users: i32,
    pub max_login_attempts: i32,
    pub max_logins_per_ip: i32,
    pub max_voice_tx: i32,
    pub max_video_tx: i32,
    pub max_media_tx: i32,
    pub max_desktop_tx: i32,
    pub max_total_tx: i32,
    pub auto_save: bool,
    pub tcp_port: i32,
    pub udp_port: i32,
    pub user_timeout: i32,
    pub version: String,
    pub protocol_version: String,
    pub login_delay: i32,
    pub access_token: String,
    pub log_events: u32,
}

impl From<ffi::ServerProperties> for ServerProperties {
    fn from(p: ffi::ServerProperties) -> Self {
        Self {
            name: crate::utils::strings::to_string(&p.szServerName),
            motd: crate::utils::strings::to_string(&p.szMOTD),
            motd_raw: crate::utils::strings::to_string(&p.szMOTDRaw),
            max_users: p.nMaxUsers,
            max_login_attempts: p.nMaxLoginAttempts,
            max_logins_per_ip: p.nMaxLoginsPerIPAddress,
            max_voice_tx: p.nMaxVoiceTxPerSecond,
            max_video_tx: p.nMaxVideoCaptureTxPerSecond,
            max_media_tx: p.nMaxMediaFileTxPerSecond,
            max_desktop_tx: p.nMaxDesktopTxPerSecond,
            max_total_tx: p.nMaxTotalTxPerSecond,
            auto_save: p.bAutoSave != 0,
            tcp_port: p.nTcpPort,
            udp_port: p.nUdpPort,
            user_timeout: p.nUserTimeout,
            version: crate::utils::strings::to_string(&p.szServerVersion),
            protocol_version: crate::utils::strings::to_string(&p.szServerProtocolVersion),
            login_delay: p.nLoginDelayMSec,
            access_token: crate::utils::strings::to_string(&p.szAccessToken),
            log_events: p.uServerLogEvents,
        }
    }
}

impl ServerProperties {
    /// Creates a new server properties instance.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        motd: impl Into<String>,
        motd_raw: impl Into<String>,
        max_users: i32,
        max_login_attempts: i32,
        max_logins_per_ip: i32,
        max_voice_tx: i32,
        max_video_tx: i32,
        max_media_tx: i32,
        max_desktop_tx: i32,
        max_total_tx: i32,
        auto_save: bool,
        tcp_port: i32,
        udp_port: i32,
        user_timeout: i32,
        version: impl Into<String>,
        protocol_version: impl Into<String>,
        login_delay: i32,
        access_token: impl Into<String>,
        log_events: u32,
    ) -> Self {
        Self {
            name: name.into(),
            motd: motd.into(),
            motd_raw: motd_raw.into(),
            max_users,
            max_login_attempts,
            max_logins_per_ip,
            max_voice_tx,
            max_video_tx,
            max_media_tx,
            max_desktop_tx,
            max_total_tx,
            auto_save,
            tcp_port,
            udp_port,
            user_timeout,
            version: version.into(),
            protocol_version: protocol_version.into(),
            login_delay,
            access_token: access_token.into(),
            log_events,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::ServerProperties {
        let mut raw = ffi::ServerProperties::default();
        let name = crate::utils::ToTT::tt(&self.name);
        let motd_raw = crate::utils::ToTT::tt(&self.motd_raw);
        unsafe {
            let n_len = name.len().min(511);
            std::ptr::copy_nonoverlapping(name.as_ptr(), raw.szServerName.as_mut_ptr(), n_len);
            let m_len = motd_raw.len().min(511);
            std::ptr::copy_nonoverlapping(motd_raw.as_ptr(), raw.szMOTDRaw.as_mut_ptr(), m_len);
        }
        raw.nMaxUsers = self.max_users;
        raw.nMaxLoginAttempts = self.max_login_attempts;
        raw.nMaxLoginsPerIPAddress = self.max_logins_per_ip;
        raw.nMaxVoiceTxPerSecond = self.max_voice_tx;
        raw.nMaxVideoCaptureTxPerSecond = self.max_video_tx;
        raw.nMaxMediaFileTxPerSecond = self.max_media_tx;
        raw.nMaxDesktopTxPerSecond = self.max_desktop_tx;
        raw.nMaxTotalTxPerSecond = self.max_total_tx;
        raw.bAutoSave = i32::from(self.auto_save);
        raw.nTcpPort = self.tcp_port;
        raw.nUdpPort = self.udp_port;
        raw.nUserTimeout = self.user_timeout;
        raw.nLoginDelayMSec = self.login_delay;
        raw.uServerLogEvents = self.log_events;
        raw
    }
}

/// Client statistics snapshot.
#[non_exhaustive]
pub struct ClientStatistics {
    pub udp_ping: i32,
    pub tcp_ping: i32,
    pub tcp_server_silence_sec: i32,
    pub udp_server_silence_sec: i32,
    pub udp_sent: i64,
    pub udp_recv: i64,
    pub voice_sent: i64,
    pub voice_recv: i64,
    pub video_sent: i64,
    pub video_recv: i64,
    pub media_audio_sent: i64,
    pub media_audio_recv: i64,
    pub media_video_sent: i64,
    pub media_video_recv: i64,
    pub desktop_sent: i64,
    pub desktop_recv: i64,
}

impl From<ffi::ClientStatistics> for ClientStatistics {
    fn from(s: ffi::ClientStatistics) -> Self {
        Self {
            udp_ping: s.nUdpPingTimeMs,
            tcp_ping: s.nTcpPingTimeMs,
            tcp_server_silence_sec: s.nTcpServerSilenceSec,
            udp_server_silence_sec: s.nUdpServerSilenceSec,
            udp_sent: s.nUdpBytesSent,
            udp_recv: s.nUdpBytesRecv,
            voice_sent: s.nVoiceBytesSent,
            voice_recv: s.nVoiceBytesRecv,
            video_sent: s.nVideoCaptureBytesSent,
            video_recv: s.nVideoCaptureBytesRecv,
            media_audio_sent: s.nMediaFileAudioBytesSent,
            media_audio_recv: s.nMediaFileAudioBytesRecv,
            media_video_sent: s.nMediaFileVideoBytesSent,
            media_video_recv: s.nMediaFileVideoBytesRecv,
            desktop_sent: s.nDesktopBytesSent,
            desktop_recv: s.nDesktopBytesRecv,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Client flag bitmask.
pub struct ClientFlags(u32);

impl ClientFlags {
    /// Client is closed.
    pub const CLOSED: u32 = 0x0000_0000;
    /// Sound input is ready.
    pub const SNDINPUT_READY: u32 = 0x0000_0001;
    /// Sound output is ready.
    pub const SNDOUTPUT_READY: u32 = 0x0000_0002;
    /// Sound input/output duplex is ready.
    pub const SNDINOUTPUT_DUPLEX: u32 = 0x0000_0004;
    /// Voice activation is enabled.
    pub const SNDINPUT_VOICEACTIVATED: u32 = 0x0000_0008;
    /// Voice input is active.
    pub const SNDINPUT_VOICEACTIVE: u32 = 0x0000_0010;
    /// Output is muted.
    pub const SNDOUTPUT_MUTE: u32 = 0x0000_0020;
    /// Auto 3D positioning is enabled.
    pub const SNDOUTPUT_AUTO3DPOSITION: u32 = 0x0000_0040;
    /// Video capture is ready.
    pub const VIDEOCAPTURE_READY: u32 = 0x0000_0080;
    /// Transmitting voice.
    pub const TX_VOICE: u32 = 0x0000_0100;
    /// Transmitting video.
    pub const TX_VIDEOCAPTURE: u32 = 0x0000_0200;
    /// Transmitting desktop.
    pub const TX_DESKTOP: u32 = 0x0000_0400;
    /// Desktop sharing is active.
    pub const DESKTOP_ACTIVE: u32 = 0x0000_0800;
    /// Muxing audio file.
    pub const MUX_AUDIOFILE: u32 = 0x0000_1000;
    /// Currently connecting.
    pub const CONNECTING: u32 = 0x0000_2000;
    /// Connected.
    pub const CONNECTED: u32 = 0x0000_4000;
    /// Connecting or connected.
    pub const CONNECTION: u32 = Self::CONNECTING | Self::CONNECTED;
    /// Authorized.
    pub const AUTHORIZED: u32 = 0x0000_8000;
    /// Streaming audio.
    pub const STREAM_AUDIO: u32 = 0x0001_0000;
    /// Streaming video.
    pub const STREAM_VIDEO: u32 = 0x0002_0000;

    /// Creates from raw bits.
    #[must_use]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
    /// Returns true if the flag is present.
    #[must_use]
    pub fn has(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }
    /// Returns the raw bitmask.
    #[must_use]
    pub fn raw(&self) -> u32 {
        self.0
    }
}
