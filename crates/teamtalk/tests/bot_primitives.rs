#![cfg(feature = "bot")]

use teamtalk::{MemoryStateStore, StateStore, parse_command};

#[test]
fn parse_command_splits_name_and_args() {
    let cmd = parse_command("/help ping pong", &['/', '!']).expect("expected command");
    assert_eq!(cmd.prefix, '/');
    assert_eq!(cmd.name, "help");
    assert_eq!(cmd.arg(0), Some("ping"));
    assert_eq!(cmd.arg(1), Some("pong"));
}

#[test]
fn parse_command_rejects_non_prefixed_input() {
    assert!(parse_command("help", &['/']).is_none());
}

#[test]
fn memory_store_roundtrip() {
    let mut store = MemoryStateStore::new();
    store.set("dialog:user:10".to_owned(), "awaiting_code".to_owned());
    assert_eq!(
        store.get("dialog:user:10").as_deref(),
        Some("awaiting_code")
    );
    assert_eq!(
        store.remove("dialog:user:10").as_deref(),
        Some("awaiting_code")
    );
    assert!(store.get("dialog:user:10").is_none());
}
