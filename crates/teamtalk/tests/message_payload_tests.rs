#![cfg(feature = "mock")]

use teamtalk::client::EventData;
use teamtalk::events::Event;
use teamtalk::mock::MockMessage;
use teamtalk::types::UserId;
use teamtalk::utils::strings::ToTT;
use teamtalk_sys as ffi;

fn copy_tt(src: &str, dst: &mut [ffi::TTCHAR]) {
    let tt = src.tt();
    let len = tt.len().min(dst.len());
    dst[..len].copy_from_slice(&tt[..len]);
}

#[test]
fn file_event_exposes_remote_file_payload() {
    let mut raw = ffi::RemoteFile {
        nChannelID: 9,
        nFileID: 11,
        nFileSize: 512,
        ..Default::default()
    };
    copy_tt("clip.wav", &mut raw.szFileName);
    copy_tt("alice", &mut raw.szUsername);
    copy_tt("now", &mut raw.szUploadTime);

    let message = MockMessage::remote_file(Event::FileNew, raw);
    let file = message.try_as_remote_file().expect("remote file");
    assert_eq!(file.channel_id.0, 9);
    assert_eq!(file.id.0, 11);
    assert_eq!(file.name, "clip.wav");
    assert!(matches!(message.data(), Some(EventData::RemoteFile(_))));
}

#[test]
fn banned_user_event_exposes_banned_user_payload() {
    let mut raw = ffi::BannedUser {
        uBanTypes: 3,
        ..Default::default()
    };
    copy_tt("1.2.3.4", &mut raw.szIPAddress);
    copy_tt("moderator", &mut raw.szOwner);

    let message = MockMessage::banned_user(raw);
    let banned = message.try_as_banned_user().expect("banned user");
    assert_eq!(banned.ip, "1.2.3.4");
    assert_eq!(banned.owner, "moderator");
    assert!(matches!(message.data(), Some(EventData::BannedUser(_))));
}

#[test]
fn desktop_input_event_exposes_desktop_input_payload() {
    let raw = ffi::DesktopInput {
        uMousePosX: 12,
        uMousePosY: 34,
        uKeyCode: 0x41,
        uKeyState: ffi::DesktopKeyState::DESKTOPKEYSTATE_DOWN as u32,
    };

    let message = MockMessage::desktop_input(raw, 7);
    let input = message.try_as_desktop_input().expect("desktop input");
    assert_eq!(input.mouse_pos_x, 12);
    assert_eq!(input.mouse_pos_y, 34);
    assert_eq!(input.key_code, 0x41);
    assert!(matches!(message.data(), Some(EventData::DesktopInput(_))));
}

#[test]
fn media_file_event_exposes_media_file_info_payload() {
    let mut raw = ffi::MediaFileInfo {
        nStatus: ffi::MediaFileStatus::MFS_STARTED,
        ..Default::default()
    };
    raw.audioFmt.nSampleRate = 48_000;
    raw.videoFmt.nWidth = 640;
    raw.uDurationMSec = 999;
    copy_tt("movie.ogg", &mut raw.szFileName);

    let message = MockMessage::media_file_info(Event::StreamMediaFile, raw);
    let info = message.try_as_media_file_info().expect("media file info");
    assert_eq!(info.name, "movie.ogg");
    assert_eq!(info.audio_fmt.nSampleRate, 48_000);
    assert_eq!(info.video_fmt.nWidth, 640);
    assert!(matches!(message.data(), Some(EventData::MediaFileInfo(_))));
}

#[test]
fn audio_input_event_exposes_audio_input_progress_payload() {
    let raw = ffi::AudioInputProgress {
        nStreamID: 44,
        uQueueMSec: 55,
        uElapsedMSec: 66,
    };

    let message = MockMessage::audio_input_progress(raw, UserId(5).0);
    let progress = message
        .try_as_audio_input_progress()
        .expect("audio input progress");
    assert_eq!(progress.stream_id, 44);
    assert_eq!(progress.queue_ms, 55);
    assert_eq!(progress.elapsed_ms, 66);
    assert!(matches!(
        message.data(),
        Some(EventData::AudioInputProgress(_))
    ));
}
