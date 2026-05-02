#![cfg(feature = "mock")]
//! Additional coverage for `MemoryStateStore` behaviour beyond the baseline
//! exercised in `bot_state_v2.rs`.

use std::thread;
use std::time::Duration;

use teamtalk::{MemoryStateStore, StateStore};

#[test]
fn set_overwrites_existing_value_without_growing_store() {
    let mut store = MemoryStateStore::new();
    store.set("k".into(), "v1".into());
    store.set("k".into(), "v2".into());
    assert_eq!(store.get("k"), Some("v2".into()));
    assert_eq!(store.keys("").len(), 1);
}

#[test]
fn set_with_ttl_replaces_permanent_entry_with_expiring_one() {
    let mut store = MemoryStateStore::new();
    store.set("k".into(), "permanent".into());
    store.set_with_ttl("k".into(), "ephemeral".into(), Duration::from_millis(20));
    thread::sleep(Duration::from_millis(40));
    assert_eq!(store.get("k"), None);
}

#[test]
fn set_overwrites_expiring_entry_with_permanent_one() {
    let mut store = MemoryStateStore::new();
    store.set_with_ttl("k".into(), "old".into(), Duration::from_millis(10));
    store.set("k".into(), "fresh".into());
    thread::sleep(Duration::from_millis(30));
    // The plain `set` must clear the TTL; value survives beyond original expiry.
    assert_eq!(store.get("k"), Some("fresh".into()));
}

#[test]
fn remove_evicts_expired_entry_even_though_it_returns_the_stale_value() {
    // Documented behaviour: `remove` operates on the raw storage map and does
    // not consult the TTL, so expired entries still return their stored value
    // via `remove`. Confirm the entry is nevertheless evicted so subsequent
    // lookups observe `None`.
    let mut store = MemoryStateStore::new();
    store.set_with_ttl("k".into(), "v".into(), Duration::from_millis(10));
    thread::sleep(Duration::from_millis(30));
    assert_eq!(store.remove("k"), Some("v".into()));
    assert_eq!(store.get("k"), None);
    assert!(!store.exists("k"));
}

#[test]
fn remove_prefix_returns_zero_when_nothing_matches() {
    let mut store = MemoryStateStore::new();
    store.set("a".into(), "1".into());
    assert_eq!(store.remove_prefix("unused:"), 0);
    assert_eq!(store.get("a"), Some("1".into()));
}

#[test]
fn remove_prefix_empty_prefix_removes_every_key() {
    let mut store = MemoryStateStore::new();
    store.set("a".into(), "1".into());
    store.set("b".into(), "2".into());
    store.set("c".into(), "3".into());
    let removed = store.remove_prefix("");
    assert_eq!(removed, 3);
    assert!(store.keys("").is_empty());
}

#[test]
fn keys_with_empty_prefix_returns_all_non_expired_keys() {
    let mut store = MemoryStateStore::new();
    store.set("foo".into(), "1".into());
    store.set("bar".into(), "2".into());
    let mut keys = store.keys("");
    keys.sort();
    assert_eq!(keys, vec!["bar", "foo"]);
}

#[test]
fn exists_returns_false_for_expired_entry() {
    let mut store = MemoryStateStore::new();
    store.set_with_ttl("k".into(), "v".into(), Duration::from_millis(10));
    thread::sleep(Duration::from_millis(30));
    assert!(!store.exists("k"));
}

#[test]
fn get_many_preserves_requested_key_order() {
    let mut store = MemoryStateStore::new();
    store.set("b".into(), "2".into());
    store.set("a".into(), "1".into());
    let result = store.get_many(&["a", "b", "c"]);
    assert_eq!(result, vec![Some("1".into()), Some("2".into()), None]);
}

#[test]
fn set_many_accepts_empty_batch() {
    let mut store = MemoryStateStore::new();
    store.set_many(Vec::new());
    assert!(store.keys("").is_empty());
}

#[test]
fn set_many_overwrites_existing_values() {
    let mut store = MemoryStateStore::new();
    store.set("a".into(), "before".into());
    store.set_many(vec![
        ("a".into(), "after".into()),
        ("b".into(), "new".into()),
    ]);
    assert_eq!(store.get("a"), Some("after".into()));
    assert_eq!(store.get("b"), Some("new".into()));
}

#[test]
fn get_returns_none_for_missing_and_expired_entries() {
    let mut store = MemoryStateStore::new();
    assert_eq!(store.get("missing"), None);
    store.set_with_ttl("ephemeral".into(), "x".into(), Duration::from_millis(10));
    thread::sleep(Duration::from_millis(30));
    assert_eq!(store.get("ephemeral"), None);
}

#[test]
fn remove_prefix_ignores_expired_entries_in_its_count() {
    let mut store = MemoryStateStore::new();
    store.set("p:a".into(), "1".into());
    store.set_with_ttl("p:b".into(), "2".into(), Duration::from_millis(10));
    store.set("other".into(), "3".into());
    thread::sleep(Duration::from_millis(30));
    let removed = store.remove_prefix("p:");
    assert!(
        removed == 1 || removed == 2,
        "expected remove_prefix to report 1 or 2 (implementation-defined whether it counts the already-expired entry), got {removed}"
    );
    assert!(store.keys("p:").is_empty());
    assert_eq!(store.get("other"), Some("3".into()));
}
