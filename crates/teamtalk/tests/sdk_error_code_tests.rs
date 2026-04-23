//! Integration tests for the typed SDK-error-code mapping.

use teamtalk::events::{Error, FfiError, SdkErrorCode};
use teamtalk_sys::ClientError;

#[test]
fn from_i32_covers_every_documented_client_error_variant() {
    let known = [
        ClientError::CMDERR_SUCCESS,
        ClientError::CMDERR_SYNTAX_ERROR,
        ClientError::CMDERR_UNKNOWN_COMMAND,
        ClientError::CMDERR_MISSING_PARAMETER,
        ClientError::CMDERR_INCOMPATIBLE_PROTOCOLS,
        ClientError::CMDERR_UNKNOWN_AUDIOCODEC,
        ClientError::CMDERR_INVALID_USERNAME,
        ClientError::CMDERR_INCORRECT_CHANNEL_PASSWORD,
        ClientError::CMDERR_INVALID_ACCOUNT,
        ClientError::CMDERR_MAX_SERVER_USERS_EXCEEDED,
        ClientError::CMDERR_MAX_CHANNEL_USERS_EXCEEDED,
        ClientError::CMDERR_SERVER_BANNED,
        ClientError::CMDERR_NOT_AUTHORIZED,
        ClientError::CMDERR_MAX_DISKUSAGE_EXCEEDED,
        ClientError::CMDERR_INCORRECT_OP_PASSWORD,
        ClientError::CMDERR_AUDIOCODEC_BITRATE_LIMIT_EXCEEDED,
        ClientError::CMDERR_MAX_LOGINS_PER_IPADDRESS_EXCEEDED,
        ClientError::CMDERR_MAX_CHANNELS_EXCEEDED,
        ClientError::CMDERR_COMMAND_FLOOD,
        ClientError::CMDERR_CHANNEL_BANNED,
        ClientError::CMDERR_MAX_FILETRANSFERS_EXCEEDED,
        ClientError::CMDERR_NOT_LOGGEDIN,
        ClientError::CMDERR_ALREADY_LOGGEDIN,
        ClientError::CMDERR_NOT_IN_CHANNEL,
        ClientError::CMDERR_ALREADY_IN_CHANNEL,
        ClientError::CMDERR_CHANNEL_ALREADY_EXISTS,
        ClientError::CMDERR_CHANNEL_NOT_FOUND,
        ClientError::CMDERR_USER_NOT_FOUND,
        ClientError::CMDERR_BAN_NOT_FOUND,
        ClientError::CMDERR_FILETRANSFER_NOT_FOUND,
        ClientError::CMDERR_OPENFILE_FAILED,
        ClientError::CMDERR_ACCOUNT_NOT_FOUND,
        ClientError::CMDERR_FILE_NOT_FOUND,
        ClientError::CMDERR_FILE_ALREADY_EXISTS,
        ClientError::CMDERR_FILESHARING_DISABLED,
        ClientError::CMDERR_CHANNEL_HAS_USERS,
        ClientError::CMDERR_LOGINSERVICE_UNAVAILABLE,
        ClientError::CMDERR_CHANNEL_CANNOT_BE_HIDDEN,
        ClientError::INTERR_SNDINPUT_FAILURE,
        ClientError::INTERR_SNDOUTPUT_FAILURE,
        ClientError::INTERR_AUDIOCODEC_INIT_FAILED,
        ClientError::INTERR_SPEEXDSP_INIT_FAILED,
        ClientError::INTERR_TTMESSAGE_QUEUE_OVERFLOW,
        ClientError::INTERR_SNDEFFECT_FAILURE,
    ];
    for raw in known {
        let code = SdkErrorCode::from(raw as i32);
        assert!(
            code.is_known(),
            "known FFI code {} mapped to Unknown",
            raw as i32,
        );
        assert_eq!(code.as_i32(), raw as i32, "round-trip mismatch");
    }
}

#[test]
fn unknown_codes_are_preserved_via_unknown_variant() {
    let code = SdkErrorCode::from(424_242);
    assert_eq!(code, SdkErrorCode::Unknown(424_242));
    assert!(!code.is_known());
    assert_eq!(code.as_i32(), 424_242);
    assert_eq!(code.name(), "unknown");
}

#[test]
fn negative_codes_become_unknown_and_round_trip() {
    let code = SdkErrorCode::from(-7);
    assert_eq!(code, SdkErrorCode::Unknown(-7));
    assert_eq!(code.as_i32(), -7);
}

#[test]
fn is_command_vs_internal_error_classification() {
    assert!(SdkErrorCode::NotLoggedIn.is_command_error());
    assert!(!SdkErrorCode::NotLoggedIn.is_internal_error());

    assert!(SdkErrorCode::SoundInputFailure.is_internal_error());
    assert!(!SdkErrorCode::SoundInputFailure.is_command_error());

    // Success is neither a command- nor internal-range error.
    assert!(!SdkErrorCode::Success.is_command_error());
    assert!(!SdkErrorCode::Success.is_internal_error());

    // Unknown codes are classified by their numeric range.
    assert!(SdkErrorCode::Unknown(2099).is_command_error());
    assert!(SdkErrorCode::Unknown(10_500).is_internal_error());
    assert!(!SdkErrorCode::Unknown(42).is_command_error());
    assert!(!SdkErrorCode::Unknown(42).is_internal_error());
}

#[test]
fn display_is_name_and_numeric_code() {
    let s = format!("{}", SdkErrorCode::NotAuthorized);
    assert!(s.contains("not_authorized"));
    assert!(s.contains("2006"));

    let s = format!("{}", SdkErrorCode::Unknown(9999));
    assert!(s.contains("unknown"));
    assert!(s.contains("9999"));
}

#[test]
fn from_ffi_client_error_matches_from_i32() {
    let from_enum = SdkErrorCode::from(ClientError::CMDERR_NOT_AUTHORIZED);
    let from_int = SdkErrorCode::from(ClientError::CMDERR_NOT_AUTHORIZED as i32);
    assert_eq!(from_enum, from_int);
    assert_eq!(from_enum, SdkErrorCode::NotAuthorized);
}

#[test]
fn error_sdk_code_on_command_failed() {
    let err = Error::CommandFailed {
        code: ClientError::CMDERR_INVALID_ACCOUNT as i32,
        message: "bad creds".into(),
    };
    assert_eq!(err.sdk_code(), Some(SdkErrorCode::InvalidAccount));
}

#[test]
fn error_sdk_code_on_client_error() {
    let err = Error::ClientError {
        code: ClientError::INTERR_SNDINPUT_FAILURE as i32,
        message: "no mic".into(),
    };
    assert_eq!(err.sdk_code(), Some(SdkErrorCode::SoundInputFailure));
}

#[test]
fn error_sdk_code_on_ffi_sdk_error() {
    let err = Error::Ffi(FfiError::SdkError {
        code: ClientError::CMDERR_COMMAND_FLOOD as i32,
        message: "slow down".into(),
    });
    assert_eq!(err.sdk_code(), Some(SdkErrorCode::CommandFlood));
}

#[test]
fn error_sdk_code_none_for_non_sdk_errors() {
    assert!(Error::Timeout.sdk_code().is_none());
    assert!(Error::InitFailed.sdk_code().is_none());
    assert!(Error::ConnectFailed.sdk_code().is_none());
    assert!(Error::AuthFailed.sdk_code().is_none());
    assert!(Error::InvalidParam.sdk_code().is_none());
    assert!(Error::MissingLoginParams.sdk_code().is_none());
    assert!(Error::MissingReconnectParams.sdk_code().is_none());
    assert!(
        Error::IoError {
            message: "x".into(),
        }
        .sdk_code()
        .is_none()
    );
    assert!(Error::Ffi(FfiError::NullPointer).sdk_code().is_none());
    assert!(Error::Ffi(FfiError::BoolFalse).sdk_code().is_none());
}

#[test]
fn unknown_sdk_code_preserves_raw_code_on_command_failed() {
    let err = Error::CommandFailed {
        code: 9999,
        message: "future".into(),
    };
    let code = err.sdk_code().expect("code");
    assert_eq!(code, SdkErrorCode::Unknown(9999));
    assert_eq!(code.as_i32(), 9999);
}

#[test]
fn names_are_unique_across_variants() {
    let codes = [
        SdkErrorCode::Success,
        SdkErrorCode::SyntaxError,
        SdkErrorCode::UnknownCommand,
        SdkErrorCode::MissingParameter,
        SdkErrorCode::IncompatibleProtocols,
        SdkErrorCode::UnknownAudioCodec,
        SdkErrorCode::InvalidUsername,
        SdkErrorCode::IncorrectChannelPassword,
        SdkErrorCode::InvalidAccount,
        SdkErrorCode::MaxServerUsersExceeded,
        SdkErrorCode::MaxChannelUsersExceeded,
        SdkErrorCode::ServerBanned,
        SdkErrorCode::NotAuthorized,
        SdkErrorCode::MaxDiskUsageExceeded,
        SdkErrorCode::IncorrectOpPassword,
        SdkErrorCode::AudioCodecBitrateLimitExceeded,
        SdkErrorCode::MaxLoginsPerIpAddressExceeded,
        SdkErrorCode::MaxChannelsExceeded,
        SdkErrorCode::CommandFlood,
        SdkErrorCode::ChannelBanned,
        SdkErrorCode::MaxFileTransfersExceeded,
        SdkErrorCode::NotLoggedIn,
        SdkErrorCode::AlreadyLoggedIn,
        SdkErrorCode::NotInChannel,
        SdkErrorCode::AlreadyInChannel,
        SdkErrorCode::ChannelAlreadyExists,
        SdkErrorCode::ChannelNotFound,
        SdkErrorCode::UserNotFound,
        SdkErrorCode::BanNotFound,
        SdkErrorCode::FileTransferNotFound,
        SdkErrorCode::OpenFileFailed,
        SdkErrorCode::AccountNotFound,
        SdkErrorCode::FileNotFound,
        SdkErrorCode::FileAlreadyExists,
        SdkErrorCode::FileSharingDisabled,
        SdkErrorCode::ChannelHasUsers,
        SdkErrorCode::LoginServiceUnavailable,
        SdkErrorCode::ChannelCannotBeHidden,
        SdkErrorCode::SoundInputFailure,
        SdkErrorCode::SoundOutputFailure,
        SdkErrorCode::AudioCodecInitFailed,
        SdkErrorCode::AudioPreprocessorInitFailed,
        SdkErrorCode::MessageQueueOverflow,
        SdkErrorCode::SoundEffectFailure,
    ];
    let mut names: Vec<_> = codes.iter().map(|c| c.name()).collect();
    names.sort_unstable();
    let len = names.len();
    names.dedup();
    assert_eq!(len, names.len(), "duplicate SdkErrorCode::name() values");
}

#[test]
fn round_trip_as_i32_for_every_known_variant() {
    let codes = [
        SdkErrorCode::Success,
        SdkErrorCode::SyntaxError,
        SdkErrorCode::UnknownCommand,
        SdkErrorCode::MissingParameter,
        SdkErrorCode::IncompatibleProtocols,
        SdkErrorCode::UnknownAudioCodec,
        SdkErrorCode::InvalidUsername,
        SdkErrorCode::IncorrectChannelPassword,
        SdkErrorCode::InvalidAccount,
        SdkErrorCode::MaxServerUsersExceeded,
        SdkErrorCode::MaxChannelUsersExceeded,
        SdkErrorCode::ServerBanned,
        SdkErrorCode::NotAuthorized,
        SdkErrorCode::MaxDiskUsageExceeded,
        SdkErrorCode::IncorrectOpPassword,
        SdkErrorCode::AudioCodecBitrateLimitExceeded,
        SdkErrorCode::MaxLoginsPerIpAddressExceeded,
        SdkErrorCode::MaxChannelsExceeded,
        SdkErrorCode::CommandFlood,
        SdkErrorCode::ChannelBanned,
        SdkErrorCode::MaxFileTransfersExceeded,
        SdkErrorCode::NotLoggedIn,
        SdkErrorCode::AlreadyLoggedIn,
        SdkErrorCode::NotInChannel,
        SdkErrorCode::AlreadyInChannel,
        SdkErrorCode::ChannelAlreadyExists,
        SdkErrorCode::ChannelNotFound,
        SdkErrorCode::UserNotFound,
        SdkErrorCode::BanNotFound,
        SdkErrorCode::FileTransferNotFound,
        SdkErrorCode::OpenFileFailed,
        SdkErrorCode::AccountNotFound,
        SdkErrorCode::FileNotFound,
        SdkErrorCode::FileAlreadyExists,
        SdkErrorCode::FileSharingDisabled,
        SdkErrorCode::ChannelHasUsers,
        SdkErrorCode::LoginServiceUnavailable,
        SdkErrorCode::ChannelCannotBeHidden,
        SdkErrorCode::SoundInputFailure,
        SdkErrorCode::SoundOutputFailure,
        SdkErrorCode::AudioCodecInitFailed,
        SdkErrorCode::AudioPreprocessorInitFailed,
        SdkErrorCode::MessageQueueOverflow,
        SdkErrorCode::SoundEffectFailure,
    ];
    for code in codes {
        assert_eq!(SdkErrorCode::from(code.as_i32()), code);
    }
}
