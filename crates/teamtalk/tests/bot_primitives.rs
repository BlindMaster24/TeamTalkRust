#![cfg(feature = "bot")]

use teamtalk::{Args, CommandPattern, MemoryStateStore, StateStore, parse_command};

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
fn args_helpers_parse_and_rest() {
    let raw = vec!["10".to_owned(), "hello".to_owned(), "world".to_owned()];
    let args = Args::new(&raw);
    assert_eq!(args.get::<i32>(0).expect("parse int"), Some(10));
    assert!(args.get::<bool>(1).is_err());
    assert_eq!(args.rest(1).as_deref(), Some("hello world"));
}

#[test]
fn args_require_returns_error_for_missing_value() {
    let raw = vec!["5".to_owned()];
    let args = Args::new(&raw);
    assert!(args.require::<i32>(1, "/set <value>").is_err());
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

#[test]
fn command_pattern_usage_and_bounds() {
    let pattern = CommandPattern::parse("ban <user> [reason...]").expect("pattern");
    assert_eq!(pattern.command(), "ban");
    assert_eq!(pattern.usage(), "ban <user> [reason...]");
    assert_eq!(pattern.usage_with_prefix('/'), "/ban <user> [reason...]");
    assert_eq!(pattern.min_args(), 1);
    assert_eq!(pattern.max_args(), None);
}
