//! Event and error types emitted by the `TeamTalk` client.
mod sdk_error;

pub use sdk_error::SdkErrorCode;

use crate::types::ChannelId;
use std::time::Duration;
use teamtalk_sys as ffi;

/// Client event emitted by `Client::poll`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    None,
    ConnectSuccess,
    ConnectCryptError,
    ConnectFailed,
    ConnectionLost,
    ConnectMaxPayloadUpdated,
    CmdProcessing,
    CmdError,
    CmdSuccess,
    MySelfLoggedIn,
    MySelfLoggedOut,
    MySelfKicked,
    UserLoggedIn,
    UserLoggedOut,
    UserUpdate,
    UserJoined,
    UserLeft,
    TextMessage,
    ChannelCreated,
    ChannelUpdated,
    ChannelRemoved,
    ServerUpdate,
    ServerStatistics,
    FileNew,
    FileRemove,
    UserAccount,
    BannedUser,
    UserAccountCreated,
    UserAccountRemoved,
    UserStateChange,
    VideoCaptureFrame,
    MediaFileVideo,
    DesktopWindow,
    DesktopCursor,
    DesktopInput,
    UserRecordMediaFile,
    AudioBlock,
    InternalError,
    VoiceActivation,
    Hotkey,
    HotkeyTest,
    FileTransfer,
    DesktopWindowTransfer,
    StreamMediaFile,
    LocalMediaFile,
    AudioInput,
    UserFirstVoiceStreamPacket,
    SoundDeviceAdded,
    SoundDeviceRemoved,
    SoundDeviceUnplugged,
    SoundDeviceNewDefaultInput,
    SoundDeviceNewDefaultOutput,
    SoundDeviceNewDefaultInputComDevice,
    SoundDeviceNewDefaultOutputComDevice,
    BeforeReconnect {
        attempt: u32,
        delay: Duration,
    },
    Reconnecting {
        attempt: u32,
        delay: Duration,
    },
    AfterReconnect {
        attempt: u32,
    },
    ReconnectFailed {
        attempts: u32,
    },
    BeforeAutoLogin {
        attempt: u32,
        delay: Duration,
    },
    AutoLoginFailed {
        attempts: u32,
    },
    BeforeAutoJoin {
        attempt: u32,
        delay: Duration,
    },
    AutoJoinFailed {
        attempts: u32,
    },
    AutoRecoverCompleted {
        reconnect_attempts: u32,
        login_attempts: u32,
        join_attempts: u32,
    },
    Unknown(ffi::ClientEvent),
}

/// Client connection state derived from commands and events.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionState {
    #[default]
    Idle,
    Connecting,
    Connected,
    LoggingIn,
    LoggedIn,
    Joining(ChannelId),
    Joined(ChannelId),
    Disconnected,
}

impl From<ffi::ClientEvent> for Event {
    fn from(event: ffi::ClientEvent) -> Self {
        match event {
            ffi::ClientEvent::CLIENTEVENT_NONE => Event::None,
            ffi::ClientEvent::CLIENTEVENT_CON_SUCCESS => Event::ConnectSuccess,
            ffi::ClientEvent::CLIENTEVENT_CON_CRYPT_ERROR => Event::ConnectCryptError,
            ffi::ClientEvent::CLIENTEVENT_CON_FAILED => Event::ConnectFailed,
            ffi::ClientEvent::CLIENTEVENT_CON_LOST => Event::ConnectionLost,
            ffi::ClientEvent::CLIENTEVENT_CON_MAX_PAYLOAD_UPDATED => {
                Event::ConnectMaxPayloadUpdated
            }
            ffi::ClientEvent::CLIENTEVENT_CMD_PROCESSING => Event::CmdProcessing,
            ffi::ClientEvent::CLIENTEVENT_CMD_ERROR => Event::CmdError,
            ffi::ClientEvent::CLIENTEVENT_CMD_SUCCESS => Event::CmdSuccess,
            ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_LOGGEDIN => Event::MySelfLoggedIn,
            ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_LOGGEDOUT => Event::MySelfLoggedOut,
            ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_KICKED => Event::MySelfKicked,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_LOGGEDIN => Event::UserLoggedIn,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_LOGGEDOUT => Event::UserLoggedOut,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_UPDATE => Event::UserUpdate,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_JOINED => Event::UserJoined,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_LEFT => Event::UserLeft,
            ffi::ClientEvent::CLIENTEVENT_CMD_USER_TEXTMSG => Event::TextMessage,
            ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_NEW => Event::ChannelCreated,
            ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_UPDATE => Event::ChannelUpdated,
            ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_REMOVE => Event::ChannelRemoved,
            ffi::ClientEvent::CLIENTEVENT_CMD_SERVER_UPDATE => Event::ServerUpdate,
            ffi::ClientEvent::CLIENTEVENT_CMD_SERVERSTATISTICS => Event::ServerStatistics,
            ffi::ClientEvent::CLIENTEVENT_CMD_FILE_NEW => Event::FileNew,
            ffi::ClientEvent::CLIENTEVENT_CMD_FILE_REMOVE => Event::FileRemove,
            ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT => Event::UserAccount,
            ffi::ClientEvent::CLIENTEVENT_CMD_BANNEDUSER => Event::BannedUser,
            ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_NEW => Event::UserAccountCreated,
            ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_REMOVE => Event::UserAccountRemoved,
            ffi::ClientEvent::CLIENTEVENT_USER_STATECHANGE => Event::UserStateChange,
            ffi::ClientEvent::CLIENTEVENT_USER_VIDEOCAPTURE => Event::VideoCaptureFrame,
            ffi::ClientEvent::CLIENTEVENT_USER_MEDIAFILE_VIDEO => Event::MediaFileVideo,
            ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPWINDOW => Event::DesktopWindow,
            ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPCURSOR => Event::DesktopCursor,
            ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPINPUT => Event::DesktopInput,
            ffi::ClientEvent::CLIENTEVENT_USER_RECORD_MEDIAFILE => Event::UserRecordMediaFile,
            ffi::ClientEvent::CLIENTEVENT_USER_AUDIOBLOCK => Event::AudioBlock,
            ffi::ClientEvent::CLIENTEVENT_INTERNAL_ERROR => Event::InternalError,
            ffi::ClientEvent::CLIENTEVENT_VOICE_ACTIVATION => Event::VoiceActivation,
            ffi::ClientEvent::CLIENTEVENT_HOTKEY => Event::Hotkey,
            ffi::ClientEvent::CLIENTEVENT_HOTKEY_TEST => Event::HotkeyTest,
            ffi::ClientEvent::CLIENTEVENT_FILETRANSFER => Event::FileTransfer,
            ffi::ClientEvent::CLIENTEVENT_DESKTOPWINDOW_TRANSFER => Event::DesktopWindowTransfer,
            ffi::ClientEvent::CLIENTEVENT_STREAM_MEDIAFILE => Event::StreamMediaFile,
            ffi::ClientEvent::CLIENTEVENT_LOCAL_MEDIAFILE => Event::LocalMediaFile,
            ffi::ClientEvent::CLIENTEVENT_AUDIOINPUT => Event::AudioInput,
            ffi::ClientEvent::CLIENTEVENT_USER_FIRSTVOICESTREAMPACKET => {
                Event::UserFirstVoiceStreamPacket
            }
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_ADDED => Event::SoundDeviceAdded,
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_REMOVED => Event::SoundDeviceRemoved,
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_UNPLUGGED => Event::SoundDeviceUnplugged,
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT => {
                Event::SoundDeviceNewDefaultInput
            }
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT => {
                Event::SoundDeviceNewDefaultOutput
            }
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT_COMDEVICE => {
                Event::SoundDeviceNewDefaultInputComDevice
            }
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT_COMDEVICE => {
                Event::SoundDeviceNewDefaultOutputComDevice
            }
            #[allow(unreachable_patterns)]
            c => Event::Unknown(c),
        }
    }
}

impl Event {
    /// Returns true when the event indicates a reconnect should be attempted.
    #[must_use]
    pub fn is_reconnect_needed(&self) -> bool {
        matches!(
            self,
            Event::ConnectionLost | Event::ConnectFailed | Event::ConnectCryptError
        )
    }

    /// Returns true when the event indicates a reconnect should be attempted,
    /// including any additional custom events.
    #[must_use]
    pub fn is_reconnect_needed_with(&self, extra: &[Event]) -> bool {
        if self.is_reconnect_needed() {
            return true;
        }
        for extra_event in extra {
            if std::mem::discriminant(self) == std::mem::discriminant(extra_event) {
                return true;
            }
        }
        false
    }
}

/// Error type used across `TeamTalk` operations.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Init failed")]
    InitFailed,
    #[error("Command failed: {code} ({message})")]
    CommandFailed {
        /// Raw SDK error code. Use [`Error::sdk_code`] to obtain a
        /// typed [`SdkErrorCode`] view without losing unknown codes.
        code: i32,
        /// Human-readable error message from the SDK.
        message: String,
    },
    #[error("Connection failed")]
    ConnectFailed,
    #[error("Auth failed")]
    AuthFailed,
    #[error("Invalid parameter")]
    InvalidParam,
    #[error("Missing reconnect parameters")]
    MissingReconnectParams,
    #[error("Missing login parameters")]
    MissingLoginParams,
    #[error("SDK Error: {code} ({message})")]
    ClientError {
        /// Raw SDK error code. Use [`Error::sdk_code`] to obtain a
        /// typed [`SdkErrorCode`] view without losing unknown codes.
        code: i32,
        /// Human-readable error message from the SDK.
        message: String,
    },
    #[error("IO error: {message}")]
    IoError { message: String },
    #[error("Operation timed out: {kind}")]
    Timeout {
        /// Classification of which kind of operation hit the timeout,
        /// so callers can branch on `Command` vs `Join` vs `Transfer`
        /// etc. without string-matching on error messages.
        kind: TimeoutKind,
    },
    #[error("FFI error: {0}")]
    Ffi(#[from] FfiError),
}

impl Error {
    /// Returns the typed [`SdkErrorCode`] carried by
    /// [`Error::CommandFailed`], [`Error::ClientError`], or
    /// [`Error::Ffi`] (via [`FfiError::SdkError`]).
    ///
    /// Returns `None` for errors that do not originate from the
    /// SDK integer code space (for example [`Error::Timeout`] or
    /// [`Error::IoError`]). Unknown codes are still returned — they
    /// map to [`SdkErrorCode::Unknown`], preserving the raw `i32`
    /// so callers never silently lose information about a new or
    /// out-of-range SDK code.
    #[must_use]
    pub fn sdk_code(&self) -> Option<SdkErrorCode> {
        match self {
            Self::CommandFailed { code, .. } | Self::ClientError { code, .. } => {
                Some(SdkErrorCode::from(*code))
            }
            Self::Ffi(FfiError::SdkError { code, .. }) => Some(SdkErrorCode::from(*code)),
            _ => None,
        }
    }

    /// Constructs an [`Error::Timeout`] with the given classification.
    ///
    /// Shorthand for `Error::Timeout { kind }` that keeps the call
    /// sites readable (`Error::timeout(TimeoutKind::Command)`).
    #[must_use]
    pub const fn timeout(kind: TimeoutKind) -> Self {
        Self::Timeout { kind }
    }

    /// Returns the [`TimeoutKind`] carried by an [`Error::Timeout`],
    /// or [`None`] for any other variant.
    #[must_use]
    pub fn timeout_kind(&self) -> Option<TimeoutKind> {
        if let Self::Timeout { kind } = *self {
            Some(kind)
        } else {
            None
        }
    }
}

/// Categorises which blocking `*_and_wait` call hit its deadline.
///
/// Produced by [`Error::Timeout`] so callers can differentiate
/// between, for example, a slow login from a slow file transfer
/// without scraping the error message.
///
/// Marked `#[non_exhaustive]` so new kinds can be introduced in a
/// minor release; callers must always include a `_ =>` arm.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeoutKind {
    /// Generic command completion timeout (`wait_for_command`,
    /// `list_*_and_wait`, `create_user_account_and_wait`, etc.).
    Command,
    /// TCP/UDP connect step timed out before `ConnectSuccess`.
    Connect,
    /// `login_and_wait` did not observe `MySelfLoggedIn` in time.
    Login,
    /// `join_channel_and_wait` did not reach `JoinedChannel`.
    Join,
    /// File transfer did not reach a terminal state in time.
    Transfer,
    /// `update_server_and_wait` / `save_server_config_and_wait`.
    ServerConfig,
    /// Timeout that does not fall into any of the categorised kinds
    /// yet. Prefer adding a typed variant in a follow-up when the
    /// call site is stable.
    Other,
}

impl TimeoutKind {
    /// Returns a short, stable, lower-snake-case name for this
    /// variant (useful for structured logging and metrics).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Connect => "connect",
            Self::Login => "login",
            Self::Join => "join",
            Self::Transfer => "transfer",
            Self::ServerConfig => "server_config",
            Self::Other => "other",
        }
    }
}

impl std::fmt::Display for TimeoutKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, thiserror::Error)]
pub enum FfiError {
    #[error("SDK error {code}: {message}")]
    SdkError { code: i32, message: String },
    #[error("operation returned false")]
    BoolFalse,
    #[error("null pointer returned")]
    NullPointer,
}

/// Convenience result type for `TeamTalk` operations.
pub type Result<T> = std::result::Result<T, Error>;
