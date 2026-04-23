//! Integration tests for the `SecretString` credential wrapper
//! and its adoption on `LoginParams`.

use teamtalk::LoginParams;
use teamtalk::types::SecretString;

#[test]
fn secret_string_exposes_inner_string() {
    let s = SecretString::from("hunter2");
    assert_eq!(s.expose_secret(), "hunter2");
    assert_eq!(s.len(), 7);
    assert!(!s.is_empty());
}

#[test]
fn secret_string_default_is_empty() {
    let s = SecretString::default();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.expose_secret(), "");
}

#[test]
fn secret_string_zeroize_in_place_empties_buffer() {
    let mut s = SecretString::from("hunter2");
    s.zeroize_in_place();
    assert!(s.is_empty());
    assert_eq!(s.expose_secret(), "");
}

#[test]
fn debug_does_not_leak_contents() {
    let s = SecretString::from("supersecret-password");
    let dbg = format!("{s:?}");
    assert!(
        !dbg.contains("supersecret-password"),
        "Debug should not include the plaintext, got: {dbg}"
    );
    assert!(
        dbg.contains("redacted") || dbg.contains("SecretString"),
        "Debug should mark the field as redacted, got: {dbg}"
    );
}

#[test]
fn display_does_not_leak_contents() {
    let s = SecretString::from("supersecret-password");
    let disp = format!("{s}");
    assert!(
        !disp.contains("supersecret-password"),
        "Display should not include the plaintext, got: {disp}"
    );
    assert!(
        disp.contains("redacted"),
        "Display should announce redaction, got: {disp}"
    );
}

#[test]
fn equality_is_content_based() {
    let a = SecretString::from("abc");
    let b = SecretString::from("abc");
    let c = SecretString::from("abd");
    let d = SecretString::from("ab");
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
}

#[test]
fn conversions_from_common_types() {
    let from_string: SecretString = "pw".to_string().into();
    let from_str: SecretString = "pw".into();
    let from_cow: SecretString = std::borrow::Cow::Borrowed("pw").into();
    let from_ref_string: SecretString = (&"pw".to_string()).into();

    assert_eq!(from_string.expose_secret(), "pw");
    assert_eq!(from_str.expose_secret(), "pw");
    assert_eq!(from_cow.expose_secret(), "pw");
    assert_eq!(from_ref_string.expose_secret(), "pw");
}

#[test]
fn clone_preserves_value() {
    let original = SecretString::from("shared");
    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(cloned.expose_secret(), "shared");
}

#[test]
fn login_params_password_is_secret_string() {
    let params = LoginParams::new("nick", "user", "pw", "client");
    assert_eq!(params.password.expose_secret(), "pw");
    assert_eq!(params.nickname, "nick");
    assert_eq!(params.username, "user");
    assert_eq!(params.client_name, "client");
}

#[test]
fn login_params_debug_does_not_leak_password() {
    let params = LoginParams::new("nick", "user", "hunter2", "client");
    let dbg = format!("{params:?}");
    assert!(
        !dbg.contains("hunter2"),
        "LoginParams Debug leaked password: {dbg}"
    );
    // Nickname/username are not secret — they should still be visible.
    assert!(dbg.contains("nick"));
    assert!(dbg.contains("user"));
}

#[test]
fn login_params_accepts_string_and_str_passwords() {
    let owned = String::from("pw-owned");
    let p1 = LoginParams::new("n", "u", owned.clone(), "c");
    let p2 = LoginParams::new("n", "u", "pw-borrowed", "c");
    let p3 = LoginParams::new("n", "u", SecretString::from("pw-typed"), "c");

    assert_eq!(p1.password.expose_secret(), "pw-owned");
    assert_eq!(p2.password.expose_secret(), "pw-borrowed");
    assert_eq!(p3.password.expose_secret(), "pw-typed");
}

#[test]
fn empty_secret_and_empty_debug() {
    let empty = SecretString::new();
    assert!(empty.is_empty());
    let dbg = format!("{empty:?}");
    assert!(
        !dbg.is_empty(),
        "Debug should still produce SOME output for an empty secret"
    );
    assert!(dbg.contains("redacted") || dbg.contains("SecretString"));
}
