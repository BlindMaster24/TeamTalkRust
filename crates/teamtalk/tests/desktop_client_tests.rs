#![cfg(feature = "mock")]

use std::sync::Arc;

use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;
use teamtalk::client::ffi;
use teamtalk::events::ConnectionState;
use teamtalk::types::UserId;

fn make_client() -> (Arc<MockBackend>, Client) {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);
    (backend, client)
}

#[test]
fn close_desktop_window_routes_to_backend() {
    let (_backend, client) = make_client();
    assert!(client.close_desktop_window());
}

#[test]
fn send_desktop_cursor_position_routes_to_backend() {
    let (_backend, client) = make_client();
    assert!(client.send_desktop_cursor_position(100, 200));
}

#[test]
fn send_desktop_input_routes_to_backend() {
    let (_backend, client) = make_client();
    let input = ffi::DesktopInput::default();
    assert!(client.send_desktop_input(UserId(1), &input));
}

#[test]
fn send_desktop_inputs_empty_returns_false() {
    let (_backend, client) = make_client();
    let inputs: Vec<teamtalk::types::DesktopInput> = vec![];
    assert!(!client.send_desktop_inputs(UserId(1), &inputs));
}

#[test]
fn send_desktop_window_routes_to_backend() {
    let (_backend, client) = make_client();
    let window = ffi::DesktopWindow::default();
    let result = client.send_desktop_window(&window, ffi::BitmapFormat::BMP_RGB32);
    assert_eq!(result, 0);
}

#[test]
fn acquire_user_desktop_window_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let result = client.acquire_user_desktop_window(UserId(1));
    assert!(result.is_none());
}

#[test]
fn acquire_user_desktop_window_guard_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let guard = client.acquire_user_desktop_window_guard(UserId(1));
    assert!(guard.is_none());
}

#[test]
fn acquire_user_desktop_window_ex_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let result = client.acquire_user_desktop_window_ex(UserId(1), ffi::BitmapFormat::BMP_RGB32);
    assert!(result.is_none());
}

#[test]
fn acquire_user_desktop_window_guard_ex_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let guard =
        client.acquire_user_desktop_window_guard_ex(UserId(1), ffi::BitmapFormat::BMP_RGB32);
    assert!(guard.is_none());
}

#[test]
fn desktop_input_key_translate_empty_returns_none() {
    let (_backend, client) = make_client();
    let inputs: Vec<teamtalk::types::DesktopInput> = vec![];
    let result =
        client.desktop_input_key_translate(ffi::TTKeyTranslate::TTKEY_NO_TRANSLATE, &inputs);
    assert!(result.is_none());
}

#[test]
fn execute_desktop_input_empty_returns_negative() {
    let (_backend, client) = make_client();
    let inputs: Vec<teamtalk::types::DesktopInput> = vec![];
    assert_eq!(client.execute_desktop_input(&inputs), -1);
}

#[test]
fn release_user_desktop_window_null_returns_false() {
    let (_backend, client) = make_client();
    assert!(!unsafe { client.release_user_desktop_window(std::ptr::null_mut()) });
}
