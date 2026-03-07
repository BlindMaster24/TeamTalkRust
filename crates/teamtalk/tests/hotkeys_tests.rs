#![cfg(all(feature = "mock", windows))]

use std::sync::Arc;
use teamtalk::Client;
use teamtalk::client::backend::MockBackend;

#[test]
fn hotkeys_require_hwnd_client() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("mock client");

    assert!(!client.register_hotkey(1, &[0x41]));
    assert!(!client.unregister_hotkey(1));
    assert!(!client.is_hotkey_active(1));
    assert!(!client.remove_hotkey_test_hook());
}
