//! Additional coverage for `Event`, `Error`, `FfiError` and `ConnectionState`
//! helpers that are not exercised elsewhere.

use std::time::Duration;
use teamtalk::events::{ConnectionState, Error, Event, FfiError};
use teamtalk::types::ChannelId;

#[test]
fn event_is_reconnect_needed_matches_documented_variants() {
    assert!(Event::ConnectionLost.is_reconnect_needed());
    assert!(Event::ConnectFailed.is_reconnect_needed());
    assert!(Event::ConnectCryptError.is_reconnect_needed());
    assert!(!Event::ConnectSuccess.is_reconnect_needed());
    assert!(!Event::TextMessage.is_reconnect_needed());
    assert!(!Event::UserJoined.is_reconnect_needed());
}

#[test]
fn event_is_reconnect_needed_with_accepts_extra_by_discriminant() {
    let extra = [Event::MySelfKicked];
    assert!(Event::MySelfKicked.is_reconnect_needed_with(&extra));
    assert!(Event::ConnectionLost.is_reconnect_needed_with(&extra));
    assert!(!Event::UserLoggedIn.is_reconnect_needed_with(&extra));
}

#[test]
fn event_is_reconnect_needed_with_ignores_payload_differences() {
    // Parametric variants compare by discriminant regardless of payload.
    let extra = [Event::BeforeReconnect {
        attempt: 1,
        delay: Duration::from_secs(1),
    }];
    let probe = Event::BeforeReconnect {
        attempt: 99,
        delay: Duration::from_millis(3),
    };
    assert!(probe.is_reconnect_needed_with(&extra));
}

#[test]
fn event_is_reconnect_needed_with_handles_empty_extras_correctly() {
    assert!(!Event::TextMessage.is_reconnect_needed_with(&[]));
    assert!(Event::ConnectionLost.is_reconnect_needed_with(&[]));
}

#[test]
fn connection_state_default_is_idle() {
    assert_eq!(ConnectionState::default(), ConnectionState::Idle);
}

#[test]
fn connection_state_joining_and_joined_are_distinct_for_different_channels() {
    let a = ConnectionState::Joining(ChannelId(1));
    let b = ConnectionState::Joining(ChannelId(2));
    assert_ne!(a, b);
}

#[test]
fn error_display_includes_command_code_and_message() {
    let err = Error::CommandFailed {
        code: 42,
        message: "nope".to_string(),
    };
    let rendered = err.to_string();
    assert!(rendered.contains("42"));
    assert!(rendered.contains("nope"));
    assert!(rendered.starts_with("Command failed"));
}

#[test]
fn error_display_covers_stateless_variants() {
    assert_eq!(Error::InitFailed.to_string(), "Init failed");
    assert_eq!(Error::ConnectFailed.to_string(), "Connection failed");
    assert_eq!(Error::AuthFailed.to_string(), "Auth failed");
    assert_eq!(Error::InvalidParam.to_string(), "Invalid parameter");
    assert_eq!(
        Error::MissingLoginParams.to_string(),
        "Missing login parameters"
    );
    assert_eq!(
        Error::MissingReconnectParams.to_string(),
        "Missing reconnect parameters"
    );
    assert_eq!(Error::Timeout.to_string(), "Operation timed out");
}

#[test]
fn error_io_and_client_variants_carry_messages() {
    let io = Error::IoError {
        message: "read fail".into(),
    };
    assert!(io.to_string().contains("read fail"));
    let sdk = Error::ClientError {
        code: 9,
        message: "inner".into(),
    };
    assert!(sdk.to_string().contains("9"));
    assert!(sdk.to_string().contains("inner"));
}

#[test]
fn ffi_error_display_differentiates_variants() {
    assert_eq!(FfiError::BoolFalse.to_string(), "operation returned false");
    assert_eq!(FfiError::NullPointer.to_string(), "null pointer returned");
    let sdk = FfiError::SdkError {
        code: 7,
        message: "boom".into(),
    };
    assert!(sdk.to_string().contains("7"));
    assert!(sdk.to_string().contains("boom"));
}

#[test]
fn ffi_error_converts_to_error_via_from_impl() {
    let ffi = FfiError::BoolFalse;
    let err: Error = ffi.into();
    assert!(matches!(err, Error::Ffi(FfiError::BoolFalse)));
    // Display cascades through the `#[from]` variant.
    let rendered = err.to_string();
    assert!(rendered.starts_with("FFI error"));
    assert!(rendered.contains("operation returned false"));
}

#[test]
fn ffi_error_sdk_variant_roundtrips_payload_via_clone() {
    let original = FfiError::SdkError {
        code: 12,
        message: "hi".into(),
    };
    let cloned = original.clone();
    match cloned {
        FfiError::SdkError { code, message } => {
            assert_eq!(code, 12);
            assert_eq!(message, "hi");
        }
        _ => panic!("expected SdkError"),
    }
}

#[test]
fn event_derives_copy_and_eq_for_stateless_variants() {
    let a = Event::ConnectSuccess;
    let b = a; // Copy
    assert_eq!(a, b);
    assert_ne!(Event::ConnectSuccess, Event::ConnectFailed);
}

#[test]
fn event_unknown_carries_ffi_client_event() {
    use teamtalk::client::ffi;
    let e = Event::Unknown(ffi::ClientEvent::CLIENTEVENT_NONE);
    match e {
        Event::Unknown(code) => assert_eq!(code, ffi::ClientEvent::CLIENTEVENT_NONE),
        _ => panic!("expected Unknown"),
    }
}
