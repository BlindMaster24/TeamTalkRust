/// Speex narrowband minimum bitrate.
pub const SPEEX_NB_MIN_BITRATE: i32 = 2150;
/// Speex narrowband maximum bitrate.
pub const SPEEX_NB_MAX_BITRATE: i32 = 24_600;
/// Speex wideband minimum bitrate.
pub const SPEEX_WB_MIN_BITRATE: i32 = 3950;
/// Speex wideband maximum bitrate.
pub const SPEEX_WB_MAX_BITRATE: i32 = 42_200;
/// Speex ultra-wideband minimum bitrate.
pub const SPEEX_UWB_MIN_BITRATE: i32 = 4150;
/// Speex ultra-wideband maximum bitrate.
pub const SPEEX_UWB_MAX_BITRATE: i32 = 44_000;

/// Opus minimum bitrate.
pub const OPUS_MIN_BITRATE: i32 = 6000;
/// Opus maximum bitrate.
pub const OPUS_MAX_BITRATE: i32 = 510_000;
/// Opus minimum frame size in ms.
pub const OPUS_MIN_FRAMESIZE: i32 = 2;
/// Opus maximum frame size in ms.
pub const OPUS_MAX_FRAMESIZE: i32 = 60;

/// Maximum WebRTC gain value.
pub const WEBRTC_GAIN_MAX: f32 = 49.9;

/// Desktop input: left mouse button.
pub const MOUSE_LEFT: u32 = 0x1000;
/// Desktop input: right mouse button.
pub const MOUSE_RIGHT: u32 = 0x1001;
/// Desktop input: middle mouse button.
pub const MOUSE_MIDDLE: u32 = 0x1002;

/// `TeamTalk` fixed string length.
pub const TT_STRLEN: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Subscription mask for user streams.
pub struct Subscriptions(pub(crate) u32);

impl Subscriptions {
    /// No subscriptions.
    pub const NONE: u32 = 0x0000_0000;
    /// User messages subscription flag.
    pub const USER_MSG: u32 = 0x0000_0001;
    /// Channel messages subscription flag.
    pub const CHANNEL_MSG: u32 = 0x0000_0002;
    /// Broadcast messages subscription flag.
    pub const BROADCAST_MSG: u32 = 0x0000_0004;
    /// Custom messages subscription flag.
    pub const CUSTOM_MSG: u32 = 0x0000_0008;
    /// Voice subscription flag.
    pub const VOICE: u32 = 0x0000_0010;
    /// Video capture subscription flag.
    pub const VIDEOCAPTURE: u32 = 0x0000_0020;
    /// Desktop subscription flag.
    pub const DESKTOP: u32 = 0x0000_0040;
    /// Desktop input subscription flag.
    pub const DESKTOPINPUT: u32 = 0x0000_0080;
    /// Media file subscription flag.
    pub const MEDIAFILE: u32 = 0x0000_0100;
    /// Intercept user messages flag.
    pub const INTERCEPT_USER_MSG: u32 = 0x0001_0000;
    /// Intercept channel messages flag.
    pub const INTERCEPT_CHANNEL_MSG: u32 = 0x0002_0000;
    /// Intercept custom messages flag.
    pub const INTERCEPT_CUSTOM_MSG: u32 = 0x0008_0000;
    /// Intercept voice streams flag.
    pub const INTERCEPT_VOICE: u32 = 0x0010_0000;
    /// Intercept video capture streams flag.
    pub const INTERCEPT_VIDEOCAPTURE: u32 = 0x0020_0000;
    /// Intercept desktop streams flag.
    pub const INTERCEPT_DESKTOP: u32 = 0x0040_0000;
    /// Intercept media file streams flag.
    pub const INTERCEPT_MEDIAFILE: u32 = 0x0100_0000;
    /// All subscription flags.
    pub const ALL: u32 = 0xFFFF_FFFF;

    /// Creates an empty subscription mask.
    #[must_use]
    pub fn new() -> Self {
        Self(Self::NONE)
    }
    /// Creates a subscription mask with all flags set.
    #[must_use]
    pub fn all() -> Self {
        Self(Self::ALL)
    }
    /// Creates a subscription mask with voice and media file streams.
    #[must_use]
    pub fn all_audio() -> Self {
        Self(Self::VOICE | Self::MEDIAFILE)
    }
    /// Creates a subscription mask with user, channel, broadcast, and custom messages.
    #[must_use]
    pub fn all_text() -> Self {
        Self(Self::USER_MSG | Self::CHANNEL_MSG | Self::BROADCAST_MSG | Self::CUSTOM_MSG)
    }
    /// Creates a subscription mask with desktop control streams.
    #[must_use]
    pub fn all_control() -> Self {
        Self(Self::DESKTOP | Self::DESKTOPINPUT)
    }
    /// Creates a subscription mask from raw bits.
    #[must_use]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
    /// Adds a subscription flag.
    pub fn add(&mut self, flag: u32) {
        self.0 |= flag;
    }
    /// Removes a subscription flag.
    pub fn remove(&mut self, flag: u32) {
        self.0 &= !flag;
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

/// Bitmask of user state flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UserState(pub(crate) u32);

impl UserState {
    /// No user state flags set.
    pub const NONE: u32 = 0x0000_0000;
    /// User is talking.
    pub const VOICE: u32 = 0x0000_0001;
    /// User voice is muted.
    pub const MUTE_VOICE: u32 = 0x0000_0002;
    /// User media file is muted.
    pub const MUTE_MEDIAFILE: u32 = 0x0000_0004;
    /// User has desktop sharing active.
    pub const DESKTOP: u32 = 0x0000_0008;
    /// User has video capture active.
    pub const VIDEOCAPTURE: u32 = 0x0000_0010;
    /// User is streaming media file audio.
    pub const MEDIAFILE_AUDIO: u32 = 0x0000_0020;
    /// User is streaming media file video.
    pub const MEDIAFILE_VIDEO: u32 = 0x0000_0040;

    /// Creates a user state from raw bits.
    #[must_use]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
    /// Returns true if the user is talking.
    #[must_use]
    pub fn is_talking(&self) -> bool {
        (self.0 & Self::VOICE) != 0
    }
    /// Returns true if the user is muted.
    #[must_use]
    pub fn is_muted(&self) -> bool {
        (self.0 & Self::MUTE_VOICE) != 0
    }
    /// Returns true if the user has video.
    #[must_use]
    pub fn has_video(&self) -> bool {
        (self.0 & Self::VIDEOCAPTURE) != 0
    }
    /// Returns the raw bitmask.
    #[must_use]
    pub fn raw(&self) -> u32 {
        self.0
    }
}

/// Channel type bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChannelType(pub(crate) u32);

impl ChannelType {
    /// Default channel type.
    pub const DEFAULT: u32 = 0x0000;
    /// Permanent channel flag.
    pub const PERMANENT: u32 = 0x0001;
    /// Solo transmit flag.
    pub const SOLO_TRANSMIT: u32 = 0x0002;
    /// Classroom flag.
    pub const CLASSROOM: u32 = 0x0004;
    /// Operator receive-only flag.
    pub const OPERATOR_RECVONLY: u32 = 0x0008;
    /// Disable voice activation flag.
    pub const NO_VOICEACTIVATION: u32 = 0x0010;
    /// Disable recording flag.
    pub const NO_RECORDING: u32 = 0x0020;
    /// Hidden channel flag.
    pub const HIDDEN: u32 = 0x0040;

    /// Creates from raw bits.
    #[must_use]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }
    /// Returns the raw bitmask.
    #[must_use]
    pub fn raw(&self) -> u32 {
        self.0
    }
}

/// Presence status of a user.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserPresence {
    #[default]
    Available,
    Away,
    Question,
}

/// Gender metadata for a user profile.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserGender {
    Male,
    Female,
    #[default]
    Neutral,
}

/// User status flags and presence metadata.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UserStatus {
    pub presence: UserPresence,
    pub gender: UserGender,
    pub video: bool,
    pub desktop: bool,
    pub streaming: bool,
    pub media_paused: bool,
}

impl UserStatus {
    /// Creates a new user status.
    #[must_use]
    pub fn new(
        presence: UserPresence,
        gender: UserGender,
        video: bool,
        desktop: bool,
        streaming: bool,
        media_paused: bool,
    ) -> Self {
        Self {
            presence,
            gender,
            video,
            desktop,
            streaming,
            media_paused,
        }
    }
    /// Creates a status from raw bit flags.
    #[must_use]
    pub fn from_bits(bits: u32) -> Self {
        let presence = match bits & 0xFF {
            1 => UserPresence::Away,
            2 => UserPresence::Question,
            _ => UserPresence::Available,
        };
        let gender = if (bits & 0x1000) != 0 {
            UserGender::Neutral
        } else if (bits & 0x100) != 0 {
            UserGender::Female
        } else {
            UserGender::Male
        };
        Self {
            presence,
            gender,
            video: (bits & 0x200) != 0,
            desktop: (bits & 0x400) != 0,
            streaming: (bits & 0x800) != 0,
            media_paused: (bits & 0x2000) != 0,
        }
    }
    /// Converts the status into raw bit flags.
    #[must_use]
    pub fn to_bits(&self) -> u32 {
        let mut bits = match self.presence {
            UserPresence::Available => 0,
            UserPresence::Away => 1,
            UserPresence::Question => 2,
        };
        match self.gender {
            UserGender::Male => {}
            UserGender::Female => bits |= 0x100,
            UserGender::Neutral => bits |= 0x1000,
        }
        if self.video {
            bits |= 0x200;
        }
        if self.desktop {
            bits |= 0x400;
        }
        if self.streaming {
            bits |= 0x800;
        }
        if self.media_paused {
            bits |= 0x2000;
        }
        bits
    }
}

/// User rights bitmask from `TeamTalk`'s account/server model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UserRights(pub(crate) u32);

impl UserRights {
    pub const NONE: Self = Self(0x0000_0000);
    pub const MULTI_LOGIN: Self = Self(0x0000_0001);
    pub const VIEW_ALL_USERS: Self = Self(0x0000_0002);
    pub const CREATE_TEMPORARY_CHANNEL: Self = Self(0x0000_0004);
    pub const MODIFY_CHANNELS: Self = Self(0x0000_0008);
    pub const TEXTMESSAGE_BROADCAST: Self = Self(0x0000_0010);
    pub const KICK_USERS: Self = Self(0x0000_0020);
    pub const BAN_USERS: Self = Self(0x0000_0040);
    pub const MOVE_USERS: Self = Self(0x0000_0080);
    pub const OPERATOR_ENABLE: Self = Self(0x0000_0100);
    pub const UPLOAD_FILES: Self = Self(0x0000_0200);
    pub const DOWNLOAD_FILES: Self = Self(0x0000_0400);
    pub const UPDATE_SERVERPROPERTIES: Self = Self(0x0000_0800);
    pub const TRANSMIT_VOICE: Self = Self(0x0000_1000);
    pub const TRANSMIT_VIDEOCAPTURE: Self = Self(0x0000_2000);
    pub const TRANSMIT_DESKTOP: Self = Self(0x0000_4000);
    pub const TRANSMIT_DESKTOPINPUT: Self = Self(0x0000_8000);
    pub const TRANSMIT_MEDIAFILE_AUDIO: Self = Self(0x0001_0000);
    pub const TRANSMIT_MEDIAFILE_VIDEO: Self = Self(0x0002_0000);
    pub const TRANSMIT_MEDIAFILE: Self = Self(0x0001_0000 | 0x0002_0000);
    pub const LOCKED_NICKNAME: Self = Self(0x0004_0000);
    pub const LOCKED_STATUS: Self = Self(0x0008_0000);
    pub const RECORD_VOICE: Self = Self(0x0010_0000);
    pub const VIEW_HIDDEN_CHANNELS: Self = Self(0x0020_0000);
    pub const TEXTMESSAGE_USER: Self = Self(0x0040_0000);
    pub const TEXTMESSAGE_CHANNEL: Self = Self(0x0080_0000);

    #[must_use]
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub fn raw(self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn has_any(self, rights: Self) -> bool {
        (self.0 & rights.0) != 0
    }

    #[must_use]
    pub fn has_all(self, rights: Self) -> bool {
        (self.0 & rights.0) == rights.0
    }
}

impl std::ops::BitOr for UserRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for UserRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
