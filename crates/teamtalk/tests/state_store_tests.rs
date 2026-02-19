#![cfg(all(feature = "mock", feature = "state"))]

use std::sync::Arc;

use teamtalk::Client;
use teamtalk::client::backend::MockBackend;

#[test]
fn state_store_starts_empty() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    client.enable_state_store(true);

    let snapshot = client.store_snapshot();
    assert!(snapshot.users.is_empty());
    assert!(snapshot.channels.is_empty());
    assert!(snapshot.properties.is_none());
    assert!(snapshot.statistics.is_none());
}
