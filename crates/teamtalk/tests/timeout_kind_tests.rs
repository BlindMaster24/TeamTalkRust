//! Integration tests for the typed timeout classification on
//! [`teamtalk::events::Error::Timeout`].

use teamtalk::events::{Error, FfiError, TimeoutKind};

#[test]
fn timeout_constructor_produces_timeout_variant() {
    let err = Error::timeout(TimeoutKind::Command);
    assert!(matches!(
        err,
        Error::Timeout {
            kind: TimeoutKind::Command
        }
    ));
}

#[test]
fn timeout_kind_accessor_returns_kind_for_timeout() {
    let kinds = [
        TimeoutKind::Command,
        TimeoutKind::Connect,
        TimeoutKind::Login,
        TimeoutKind::Join,
        TimeoutKind::Transfer,
        TimeoutKind::ServerConfig,
        TimeoutKind::Other,
    ];
    for kind in kinds {
        let err = Error::timeout(kind);
        assert_eq!(err.timeout_kind(), Some(kind));
    }
}

#[test]
fn timeout_kind_accessor_returns_none_for_non_timeout_errors() {
    assert!(Error::InitFailed.timeout_kind().is_none());
    assert!(Error::ConnectFailed.timeout_kind().is_none());
    assert!(Error::AuthFailed.timeout_kind().is_none());
    assert!(Error::InvalidParam.timeout_kind().is_none());
    assert!(Error::MissingLoginParams.timeout_kind().is_none());
    assert!(Error::MissingReconnectParams.timeout_kind().is_none());
    assert!(
        Error::IoError {
            message: "x".into(),
        }
        .timeout_kind()
        .is_none()
    );
    assert!(
        Error::CommandFailed {
            code: 1001,
            message: "x".into(),
        }
        .timeout_kind()
        .is_none()
    );
    assert!(
        Error::ClientError {
            code: 10000,
            message: "x".into(),
        }
        .timeout_kind()
        .is_none()
    );
    assert!(Error::Ffi(FfiError::NullPointer).timeout_kind().is_none());
}

#[test]
fn display_includes_kind_name() {
    let msg = format!("{}", Error::timeout(TimeoutKind::Login));
    assert!(
        msg.contains("login"),
        "Timeout Display should include kind name, got: {msg}"
    );
    assert!(
        msg.contains("timed out"),
        "Timeout Display should include the base phrase, got: {msg}"
    );
}

#[test]
fn display_distinguishes_each_kind() {
    let login = format!("{}", Error::timeout(TimeoutKind::Login));
    let transfer = format!("{}", Error::timeout(TimeoutKind::Transfer));
    assert_ne!(login, transfer);
}

#[test]
fn timeout_kind_name_is_snake_case_stable() {
    assert_eq!(TimeoutKind::Command.name(), "command");
    assert_eq!(TimeoutKind::Connect.name(), "connect");
    assert_eq!(TimeoutKind::Login.name(), "login");
    assert_eq!(TimeoutKind::Join.name(), "join");
    assert_eq!(TimeoutKind::Transfer.name(), "transfer");
    assert_eq!(TimeoutKind::ServerConfig.name(), "server_config");
    assert_eq!(TimeoutKind::Other.name(), "other");
}

#[test]
fn timeout_kind_display_matches_name() {
    let all = [
        TimeoutKind::Command,
        TimeoutKind::Connect,
        TimeoutKind::Login,
        TimeoutKind::Join,
        TimeoutKind::Transfer,
        TimeoutKind::ServerConfig,
        TimeoutKind::Other,
    ];
    for kind in all {
        assert_eq!(format!("{kind}"), kind.name());
    }
}

#[test]
fn names_are_unique() {
    let all = [
        TimeoutKind::Command,
        TimeoutKind::Connect,
        TimeoutKind::Login,
        TimeoutKind::Join,
        TimeoutKind::Transfer,
        TimeoutKind::ServerConfig,
        TimeoutKind::Other,
    ];
    let mut names: Vec<_> = all.iter().map(|k| k.name()).collect();
    names.sort_unstable();
    let len = names.len();
    names.dedup();
    assert_eq!(len, names.len(), "TimeoutKind names must be unique");
}

#[test]
fn derives_are_available() {
    // Compile-time assertions that the derived traits exist.
    fn assert_impls<T: Copy + Clone + std::fmt::Debug + Eq + std::hash::Hash>() {}
    assert_impls::<TimeoutKind>();

    // Equality and hash work.
    let a = TimeoutKind::Command;
    let b = TimeoutKind::Command;
    let c = TimeoutKind::Login;
    assert_eq!(a, b);
    assert_ne!(a, c);
    let mut set = std::collections::HashSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);
    assert_eq!(set.len(), 2);
}
