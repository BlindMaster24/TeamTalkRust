//! Typed mapping of TeamTalk SDK error codes.
//!
//! The SDK surfaces command and internal errors as an integer
//! `nErrorNo` on `ClientErrorMsg` (and through the `CMD_ERROR` /
//! `INTERNAL_ERROR` client events). Rust callers previously had to
//! compare against raw `i32` constants, which is error-prone.
//!
//! [`SdkErrorCode`] provides a typed enum with one variant per
//! documented `CMDERR_*` and `INTERR_*` constant. Unknown codes are
//! preserved via the [`SdkErrorCode::Unknown`] fallback so new SDK
//! releases do not silently lose information.
//!
//! The enum is `#[non_exhaustive]`, matching project convention, so
//! a later SDK update can add a new variant without breaking
//! downstream match statements.

use teamtalk_sys as ffi;

/// Typed TeamTalk SDK error code.
///
/// Produced from the raw `i32` carried by [`Error::CommandFailed`]
/// or [`Error::ClientError`] via `From<i32>`. Each named variant
/// mirrors one `CMDERR_*` or `INTERR_*` constant in
/// `teamtalk_sys::ClientError`; unrecognised codes map to
/// [`Self::Unknown`] so that forward compatibility with newer SDK
/// builds is preserved.
///
/// [`Error::CommandFailed`]: crate::events::Error::CommandFailed
/// [`Error::ClientError`]: crate::events::Error::ClientError
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SdkErrorCode {
    /// `CMDERR_SUCCESS` (0) — command succeeded. Present so
    /// round-tripping `0` through `From<i32>` is lossless; never
    /// observed as a `CommandFailed` error in practice.
    Success,
    /// `CMDERR_SYNTAX_ERROR` (1000).
    SyntaxError,
    /// `CMDERR_UNKNOWN_COMMAND` (1001).
    UnknownCommand,
    /// `CMDERR_MISSING_PARAMETER` (1002).
    MissingParameter,
    /// `CMDERR_INCOMPATIBLE_PROTOCOLS` (1003).
    IncompatibleProtocols,
    /// `CMDERR_UNKNOWN_AUDIOCODEC` (1004).
    UnknownAudioCodec,
    /// `CMDERR_INVALID_USERNAME` (1005).
    InvalidUsername,
    /// `CMDERR_INCORRECT_CHANNEL_PASSWORD` (2001).
    IncorrectChannelPassword,
    /// `CMDERR_INVALID_ACCOUNT` (2002).
    InvalidAccount,
    /// `CMDERR_MAX_SERVER_USERS_EXCEEDED` (2003).
    MaxServerUsersExceeded,
    /// `CMDERR_MAX_CHANNEL_USERS_EXCEEDED` (2004).
    MaxChannelUsersExceeded,
    /// `CMDERR_SERVER_BANNED` (2005).
    ServerBanned,
    /// `CMDERR_NOT_AUTHORIZED` (2006).
    NotAuthorized,
    /// `CMDERR_MAX_DISKUSAGE_EXCEEDED` (2008).
    MaxDiskUsageExceeded,
    /// `CMDERR_INCORRECT_OP_PASSWORD` (2010).
    IncorrectOpPassword,
    /// `CMDERR_AUDIOCODEC_BITRATE_LIMIT_EXCEEDED` (2011).
    AudioCodecBitrateLimitExceeded,
    /// `CMDERR_MAX_LOGINS_PER_IPADDRESS_EXCEEDED` (2012).
    MaxLoginsPerIpAddressExceeded,
    /// `CMDERR_MAX_CHANNELS_EXCEEDED` (2013).
    MaxChannelsExceeded,
    /// `CMDERR_COMMAND_FLOOD` (2014).
    CommandFlood,
    /// `CMDERR_CHANNEL_BANNED` (2015).
    ChannelBanned,
    /// `CMDERR_MAX_FILETRANSFERS_EXCEEDED` (2016).
    MaxFileTransfersExceeded,
    /// `CMDERR_NOT_LOGGEDIN` (3000).
    NotLoggedIn,
    /// `CMDERR_ALREADY_LOGGEDIN` (3001).
    AlreadyLoggedIn,
    /// `CMDERR_NOT_IN_CHANNEL` (3002).
    NotInChannel,
    /// `CMDERR_ALREADY_IN_CHANNEL` (3003).
    AlreadyInChannel,
    /// `CMDERR_CHANNEL_ALREADY_EXISTS` (3004).
    ChannelAlreadyExists,
    /// `CMDERR_CHANNEL_NOT_FOUND` (3005).
    ChannelNotFound,
    /// `CMDERR_USER_NOT_FOUND` (3006).
    UserNotFound,
    /// `CMDERR_BAN_NOT_FOUND` (3007).
    BanNotFound,
    /// `CMDERR_FILETRANSFER_NOT_FOUND` (3008).
    FileTransferNotFound,
    /// `CMDERR_OPENFILE_FAILED` (3009).
    OpenFileFailed,
    /// `CMDERR_ACCOUNT_NOT_FOUND` (3010).
    AccountNotFound,
    /// `CMDERR_FILE_NOT_FOUND` (3011).
    FileNotFound,
    /// `CMDERR_FILE_ALREADY_EXISTS` (3012).
    FileAlreadyExists,
    /// `CMDERR_FILESHARING_DISABLED` (3013).
    FileSharingDisabled,
    /// `CMDERR_CHANNEL_HAS_USERS` (3015).
    ChannelHasUsers,
    /// `CMDERR_LOGINSERVICE_UNAVAILABLE` (3016).
    LoginServiceUnavailable,
    /// `CMDERR_CHANNEL_CANNOT_BE_HIDDEN` (3017).
    ChannelCannotBeHidden,
    /// `INTERR_SNDINPUT_FAILURE` (10000).
    SoundInputFailure,
    /// `INTERR_SNDOUTPUT_FAILURE` (10001).
    SoundOutputFailure,
    /// `INTERR_AUDIOCODEC_INIT_FAILED` (10002).
    AudioCodecInitFailed,
    /// `INTERR_SPEEXDSP_INIT_FAILED` (10003). Also reported as
    /// `INTERR_AUDIOPREPROCESSOR_INIT_FAILED` by some SDK versions;
    /// both aliases share the same numeric value.
    AudioPreprocessorInitFailed,
    /// `INTERR_TTMESSAGE_QUEUE_OVERFLOW` (10004).
    MessageQueueOverflow,
    /// `INTERR_SNDEFFECT_FAILURE` (10005).
    SoundEffectFailure,
    /// Code returned by the SDK that is not known to this version
    /// of the wrapper. The raw `i32` is preserved so callers can
    /// still log or forward it.
    Unknown(i32),
}

impl SdkErrorCode {
    /// Returns the raw SDK integer code for this variant.
    ///
    /// This is the inverse of `From<i32>` and round-trips through
    /// it for every known variant: `SdkErrorCode::from(code.as_i32()) == code`.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::SyntaxError => 1000,
            Self::UnknownCommand => 1001,
            Self::MissingParameter => 1002,
            Self::IncompatibleProtocols => 1003,
            Self::UnknownAudioCodec => 1004,
            Self::InvalidUsername => 1005,
            Self::IncorrectChannelPassword => 2001,
            Self::InvalidAccount => 2002,
            Self::MaxServerUsersExceeded => 2003,
            Self::MaxChannelUsersExceeded => 2004,
            Self::ServerBanned => 2005,
            Self::NotAuthorized => 2006,
            Self::MaxDiskUsageExceeded => 2008,
            Self::IncorrectOpPassword => 2010,
            Self::AudioCodecBitrateLimitExceeded => 2011,
            Self::MaxLoginsPerIpAddressExceeded => 2012,
            Self::MaxChannelsExceeded => 2013,
            Self::CommandFlood => 2014,
            Self::ChannelBanned => 2015,
            Self::MaxFileTransfersExceeded => 2016,
            Self::NotLoggedIn => 3000,
            Self::AlreadyLoggedIn => 3001,
            Self::NotInChannel => 3002,
            Self::AlreadyInChannel => 3003,
            Self::ChannelAlreadyExists => 3004,
            Self::ChannelNotFound => 3005,
            Self::UserNotFound => 3006,
            Self::BanNotFound => 3007,
            Self::FileTransferNotFound => 3008,
            Self::OpenFileFailed => 3009,
            Self::AccountNotFound => 3010,
            Self::FileNotFound => 3011,
            Self::FileAlreadyExists => 3012,
            Self::FileSharingDisabled => 3013,
            Self::ChannelHasUsers => 3015,
            Self::LoginServiceUnavailable => 3016,
            Self::ChannelCannotBeHidden => 3017,
            Self::SoundInputFailure => 10000,
            Self::SoundOutputFailure => 10001,
            Self::AudioCodecInitFailed => 10002,
            Self::AudioPreprocessorInitFailed => 10003,
            Self::MessageQueueOverflow => 10004,
            Self::SoundEffectFailure => 10005,
            Self::Unknown(code) => code,
        }
    }

    /// Returns a short, stable, lower-snake-case name for this
    /// variant.
    ///
    /// Useful for structured logging and metrics where a string
    /// label is preferred over the numeric code.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::SyntaxError => "syntax_error",
            Self::UnknownCommand => "unknown_command",
            Self::MissingParameter => "missing_parameter",
            Self::IncompatibleProtocols => "incompatible_protocols",
            Self::UnknownAudioCodec => "unknown_audio_codec",
            Self::InvalidUsername => "invalid_username",
            Self::IncorrectChannelPassword => "incorrect_channel_password",
            Self::InvalidAccount => "invalid_account",
            Self::MaxServerUsersExceeded => "max_server_users_exceeded",
            Self::MaxChannelUsersExceeded => "max_channel_users_exceeded",
            Self::ServerBanned => "server_banned",
            Self::NotAuthorized => "not_authorized",
            Self::MaxDiskUsageExceeded => "max_disk_usage_exceeded",
            Self::IncorrectOpPassword => "incorrect_op_password",
            Self::AudioCodecBitrateLimitExceeded => "audio_codec_bitrate_limit_exceeded",
            Self::MaxLoginsPerIpAddressExceeded => "max_logins_per_ip_address_exceeded",
            Self::MaxChannelsExceeded => "max_channels_exceeded",
            Self::CommandFlood => "command_flood",
            Self::ChannelBanned => "channel_banned",
            Self::MaxFileTransfersExceeded => "max_file_transfers_exceeded",
            Self::NotLoggedIn => "not_logged_in",
            Self::AlreadyLoggedIn => "already_logged_in",
            Self::NotInChannel => "not_in_channel",
            Self::AlreadyInChannel => "already_in_channel",
            Self::ChannelAlreadyExists => "channel_already_exists",
            Self::ChannelNotFound => "channel_not_found",
            Self::UserNotFound => "user_not_found",
            Self::BanNotFound => "ban_not_found",
            Self::FileTransferNotFound => "file_transfer_not_found",
            Self::OpenFileFailed => "open_file_failed",
            Self::AccountNotFound => "account_not_found",
            Self::FileNotFound => "file_not_found",
            Self::FileAlreadyExists => "file_already_exists",
            Self::FileSharingDisabled => "file_sharing_disabled",
            Self::ChannelHasUsers => "channel_has_users",
            Self::LoginServiceUnavailable => "login_service_unavailable",
            Self::ChannelCannotBeHidden => "channel_cannot_be_hidden",
            Self::SoundInputFailure => "sound_input_failure",
            Self::SoundOutputFailure => "sound_output_failure",
            Self::AudioCodecInitFailed => "audio_codec_init_failed",
            Self::AudioPreprocessorInitFailed => "audio_preprocessor_init_failed",
            Self::MessageQueueOverflow => "message_queue_overflow",
            Self::SoundEffectFailure => "sound_effect_failure",
            Self::Unknown(_) => "unknown",
        }
    }

    /// Returns `true` if this code originates from the
    /// `CMDERR_*` protocol-level namespace (codes `1000..=3999`).
    #[must_use]
    pub const fn is_command_error(self) -> bool {
        matches!(self.as_i32(), 1000..=3999)
    }

    /// Returns `true` if this code originates from the
    /// `INTERR_*` client-internal namespace (codes `10000..=19999`).
    #[must_use]
    pub const fn is_internal_error(self) -> bool {
        matches!(self.as_i32(), 10000..=19999)
    }

    /// Returns `true` if the code is a known named variant (not
    /// [`Self::Unknown`]).
    #[must_use]
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

impl std::fmt::Display for SdkErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.as_i32())
    }
}

impl From<i32> for SdkErrorCode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Success,
            1000 => Self::SyntaxError,
            1001 => Self::UnknownCommand,
            1002 => Self::MissingParameter,
            1003 => Self::IncompatibleProtocols,
            1004 => Self::UnknownAudioCodec,
            1005 => Self::InvalidUsername,
            2001 => Self::IncorrectChannelPassword,
            2002 => Self::InvalidAccount,
            2003 => Self::MaxServerUsersExceeded,
            2004 => Self::MaxChannelUsersExceeded,
            2005 => Self::ServerBanned,
            2006 => Self::NotAuthorized,
            2008 => Self::MaxDiskUsageExceeded,
            2010 => Self::IncorrectOpPassword,
            2011 => Self::AudioCodecBitrateLimitExceeded,
            2012 => Self::MaxLoginsPerIpAddressExceeded,
            2013 => Self::MaxChannelsExceeded,
            2014 => Self::CommandFlood,
            2015 => Self::ChannelBanned,
            2016 => Self::MaxFileTransfersExceeded,
            3000 => Self::NotLoggedIn,
            3001 => Self::AlreadyLoggedIn,
            3002 => Self::NotInChannel,
            3003 => Self::AlreadyInChannel,
            3004 => Self::ChannelAlreadyExists,
            3005 => Self::ChannelNotFound,
            3006 => Self::UserNotFound,
            3007 => Self::BanNotFound,
            3008 => Self::FileTransferNotFound,
            3009 => Self::OpenFileFailed,
            3010 => Self::AccountNotFound,
            3011 => Self::FileNotFound,
            3012 => Self::FileAlreadyExists,
            3013 => Self::FileSharingDisabled,
            3015 => Self::ChannelHasUsers,
            3016 => Self::LoginServiceUnavailable,
            3017 => Self::ChannelCannotBeHidden,
            10000 => Self::SoundInputFailure,
            10001 => Self::SoundOutputFailure,
            10002 => Self::AudioCodecInitFailed,
            10003 => Self::AudioPreprocessorInitFailed,
            10004 => Self::MessageQueueOverflow,
            10005 => Self::SoundEffectFailure,
            other => Self::Unknown(other),
        }
    }
}

impl From<ffi::ClientError> for SdkErrorCode {
    fn from(value: ffi::ClientError) -> Self {
        Self::from(value as i32)
    }
}
