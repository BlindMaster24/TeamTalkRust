#![cfg(feature = "mock")]

use std::sync::Arc;

use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;
use teamtalk::events::ConnectionState;
use teamtalk::types::{UserId, VideoCodec, VideoFormat};

fn make_client() -> (Arc<MockBackend>, Client) {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);
    (backend, client)
}

#[test]
fn get_video_capture_devices_returns_empty() {
    let (_backend, client) = make_client();
    let devices = client.get_video_capture_devices();
    assert!(devices.is_empty());
}

#[test]
fn init_video_capture_device_routes_to_backend() {
    let (_backend, client) = make_client();
    let format = VideoFormat::default();
    assert!(client.init_video_capture_device("test", &format));
}

#[test]
fn close_video_capture_device_routes_to_backend() {
    let (_backend, client) = make_client();
    assert!(client.close_video_capture_device());
}

#[test]
fn start_video_transmission_routes_to_backend() {
    let (_backend, client) = make_client();
    let codec = VideoCodec::default();
    assert!(client.start_video_transmission(&codec));
}

#[test]
fn stop_video_transmission_routes_to_backend() {
    let (_backend, client) = make_client();
    assert!(client.stop_video_transmission());
}

#[test]
fn acquire_video_frame_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let result = client.acquire_video_frame(UserId(1));
    assert!(result.is_none());
}

#[test]
fn acquire_video_frame_guard_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let guard = client.acquire_video_frame_guard(UserId(1));
    assert!(guard.is_none());
}

#[test]
fn release_video_frame_null_returns_false() {
    let (_backend, client) = make_client();
    assert!(!unsafe { client.release_video_frame(std::ptr::null_mut()) });
}
