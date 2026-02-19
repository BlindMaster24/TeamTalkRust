#![cfg(feature = "async")]

#[cfg(all(feature = "mock", not(feature = "async-tokio")))]
use std::sync::Arc;
#[cfg(all(feature = "mock", not(feature = "async-tokio")))]
use teamtalk::Client;
use teamtalk::async_api::AsyncConfig;
#[cfg(all(feature = "mock", not(feature = "async-tokio")))]
use teamtalk::client::backend::MockBackend;

#[test]
fn async_config_defaults() {
    let cfg = AsyncConfig::default();
    assert_eq!(cfg.poll_timeout_ms, 100);
    assert!(cfg.buffer > 0);
}

#[test]
fn async_config_builder() {
    let cfg = AsyncConfig::new().poll_timeout_ms(5).buffer(12);
    assert_eq!(cfg.poll_timeout_ms, 5);
    assert_eq!(cfg.buffer, 12);
}

#[cfg(all(feature = "mock", not(feature = "async-tokio")))]
#[test]
fn async_client_next_event_returns_none_after_stop() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let mut stream = client.into_async();
    stream.stop();
    let next = futures::executor::block_on(stream.next_event());
    assert!(next.is_none());
}
