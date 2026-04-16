#![cfg(feature = "mock")]

use std::sync::Arc;

use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;
use teamtalk::client::ffi;
use teamtalk::client::media::MediaFilePlayback;
use teamtalk::events::ConnectionState;
use teamtalk::types::{PlaybackSessionId, UserId};

fn make_client() -> (Arc<MockBackend>, Client) {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("client");
    client.mock_set_connection_state_for_tests(ConnectionState::LoggedIn);
    (backend, client)
}

#[test]
fn get_media_file_info_returns_none_when_not_set() {
    let (_backend, client) = make_client();
    let result = client.get_media_file_info("test.wav");
    assert!(result.is_none());
}

#[test]
fn get_palette_color_returns_none_when_not_set() {
    let (_backend, client) = make_client();
    let result = client.get_palette_color(ffi::BitmapFormat::BMP_RGB8_PALETTE, 0);
    assert!(result.is_none());
}

#[test]
fn start_streaming_media_file_to_channel_routes() {
    let (_backend, client) = make_client();
    assert!(client.start_streaming_media_file_to_channel("test.wav", None));
}

#[test]
fn start_streaming_media_file_to_channel_ex_routes() {
    let (_backend, client) = make_client();
    let playback = MediaFilePlayback::default();
    assert!(client.start_streaming_media_file_to_channel_ex("test.wav", &playback, None));
}

#[test]
fn update_streaming_media_file_to_channel_routes() {
    let (_backend, client) = make_client();
    let playback = MediaFilePlayback::default();
    assert!(client.update_streaming_media_file_to_channel(&playback, None));
}

#[test]
fn stop_streaming_media_file_to_channel_routes() {
    let (_backend, client) = make_client();
    assert!(client.stop_streaming_media_file_to_channel());
}

#[test]
fn init_local_playback_returns_session_id() {
    let (_backend, client) = make_client();
    let playback = MediaFilePlayback::default();
    let session = client.init_local_playback("test.wav", &playback);
    assert_eq!(session, PlaybackSessionId(0));
}

#[test]
fn update_local_playback_routes() {
    let (_backend, client) = make_client();
    let playback = MediaFilePlayback::default();
    assert!(client.update_local_playback(PlaybackSessionId(1), &playback));
}

#[test]
fn stop_local_playback_routes() {
    let (_backend, client) = make_client();
    assert!(client.stop_local_playback(PlaybackSessionId(1)));
}

#[test]
fn acquire_user_media_video_frame_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let result = client.acquire_user_media_video_frame(UserId(1));
    assert!(result.is_none());
}

#[test]
fn acquire_user_media_video_frame_guard_returns_none_when_no_data() {
    let (_backend, client) = make_client();
    let guard = client.acquire_user_media_video_frame_guard(UserId(1));
    assert!(guard.is_none());
}

#[test]
fn release_user_media_video_frame_null_returns_false() {
    let (_backend, client) = make_client();
    assert!(!unsafe { client.release_user_media_video_frame(std::ptr::null_mut()) });
}
