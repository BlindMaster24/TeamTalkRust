#![cfg(feature = "mock")]

use std::thread;
use std::time::Duration;
use teamtalk::MemoryStateStore;
use teamtalk::StateStore;

#[test]
fn exists_returns_true_for_present_key() {
    let mut store = MemoryStateStore::new();
    store.set("foo".into(), "bar".into());
    assert!(store.exists("foo"));
}

#[test]
fn exists_returns_false_for_missing_key() {
    let store = MemoryStateStore::new();
    assert!(!store.exists("missing"));
}

#[test]
fn set_with_ttl_stores_value_accessible_before_expiry() {
    let mut store = MemoryStateStore::new();
    store.set_with_ttl("ttl_key".into(), "ttl_val".into(), Duration::from_secs(60));
    assert_eq!(store.get("ttl_key"), Some("ttl_val".into()));
    assert!(store.exists("ttl_key"));
}

#[test]
fn set_with_ttl_expires_after_duration() {
    let mut store = MemoryStateStore::new();
    store.set_with_ttl("ephemeral".into(), "gone".into(), Duration::from_millis(50));
    assert_eq!(store.get("ephemeral"), Some("gone".into()));
    thread::sleep(Duration::from_millis(80));
    assert_eq!(store.get("ephemeral"), None);
    assert!(!store.exists("ephemeral"));
}

#[test]
fn keys_returns_matching_prefix() {
    let mut store = MemoryStateStore::new();
    store.set("u:1:name".into(), "alice".into());
    store.set("u:1:age".into(), "30".into());
    store.set("u:2:name".into(), "bob".into());
    store.set("g:config".into(), "val".into());
    let mut result = store.keys("u:1:");
    result.sort();
    assert_eq!(result, vec!["u:1:age", "u:1:name"]);
}

#[test]
fn keys_excludes_expired_entries() {
    let mut store = MemoryStateStore::new();
    store.set("active".into(), "yes".into());
    store.set_with_ttl("stale".into(), "no".into(), Duration::from_millis(50));
    thread::sleep(Duration::from_millis(80));
    let mut result = store.keys("");
    result.sort();
    assert_eq!(result, vec!["active"]);
}

#[test]
fn remove_prefix_deletes_matching_keys_and_returns_count() {
    let mut store = MemoryStateStore::new();
    store.set("u:1:a".into(), "1".into());
    store.set("u:1:b".into(), "2".into());
    store.set("u:2:c".into(), "3".into());
    let count = store.remove_prefix("u:1:");
    assert_eq!(count, 2);
    assert_eq!(store.get("u:1:a"), None);
    assert_eq!(store.get("u:1:b"), None);
    assert_eq!(store.get("u:2:c"), Some("3".into()));
}

#[test]
fn get_many_returns_values_for_keys() {
    let mut store = MemoryStateStore::new();
    store.set("a".into(), "1".into());
    store.set("b".into(), "2".into());
    let result = store.get_many(&["a", "b", "c"]);
    assert_eq!(result, vec![Some("1".into()), Some("2".into()), None]);
}

#[test]
fn set_many_sets_multiple_pairs() {
    let mut store = MemoryStateStore::new();
    store.set_many(vec![("x".into(), "10".into()), ("y".into(), "20".into())]);
    assert_eq!(store.get("x"), Some("10".into()));
    assert_eq!(store.get("y"), Some("20".into()));
}

#[test]
fn remove_returns_value_for_non_expired() {
    let mut store = MemoryStateStore::new();
    store.set("rm".into(), "val".into());
    assert_eq!(store.remove("rm"), Some("val".into()));
    assert_eq!(store.get("rm"), None);
}

#[test]
fn ttl_expired_entry_not_in_keys() {
    let mut store = MemoryStateStore::new();
    store.set("keep".into(), "yes".into());
    store.set_with_ttl("expire".into(), "no".into(), Duration::from_millis(30));
    thread::sleep(Duration::from_millis(60));
    let keys = store.keys("");
    assert_eq!(keys, vec!["keep"]);
}
