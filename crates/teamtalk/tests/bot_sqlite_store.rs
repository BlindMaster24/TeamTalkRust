#![cfg(feature = "bot-sqlite")]

use teamtalk::{SqliteStateStore, StateStore};

#[test]
fn sqlite_store_roundtrip() {
    let mut store = SqliteStateStore::in_memory().expect("sqlite in-memory");
    store.set("dialog:user:1".to_owned(), "await_code".to_owned());
    assert_eq!(store.get("dialog:user:1").as_deref(), Some("await_code"));
    assert_eq!(store.remove("dialog:user:1").as_deref(), Some("await_code"));
    assert!(store.get("dialog:user:1").is_none());
}
