#[cfg(all(feature = "scripts", feature = "mock"))]
mod scripts_tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use teamtalk::events::Event;
    use teamtalk::extensions::scripts::ScriptManager;
    use teamtalk::mock::MockMessage;
    use teamtalk::utils::ToTT;

    fn copy_tt(src: &str, dst: &mut [teamtalk::client::ffi::TTCHAR]) {
        let tt = src.tt();
        let len = tt.len().min(dst.len());
        dst[..len].copy_from_slice(&tt[..len]);
    }

    fn temp_script(name: &str, contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_nanos();
        path.push(format!("teamtalk_{name}_{nanos}.lua"));
        fs::write(&path, contents).expect("write script");
        path
    }

    #[test]
    fn register_command_and_unload_removes_command() {
        let path = temp_script(
            "register_command",
            r#"
            register_command("ping", function(args)
                return true
            end)
        "#,
        );
        let mut manager = ScriptManager::new();
        manager.load_script("test", &path).expect("load");
        let handled = manager.call_command("ping", &[]).expect("call_command");
        assert!(handled);
        manager.unload_script("test").expect("unload");
        let handled = manager
            .call_command("ping", &[])
            .expect("call_command after unload");
        assert!(!handled);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn timeout_interrupts_long_running_command() {
        let path = temp_script(
            "timeout",
            r#"
            register_command("spin", function(args)
                while true do end
            end)
        "#,
        );
        let mut manager = ScriptManager::new();
        manager.set_timeout(Duration::from_millis(1));
        manager.set_hook_instruction_count(1);
        manager.load_script("timeout", &path).expect("load");
        let err = manager
            .call_command("spin", &[])
            .expect_err("expected timeout");
        let msg = err.to_string();
        assert!(msg.contains("timeout"), "{msg}");
        manager.unload_script("timeout").expect("unload");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn errors_include_context() {
        let path = temp_script(
            "event_error",
            r#"
            function on_event(evt)
                error("boom")
            end
        "#,
        );
        let mut manager = ScriptManager::new();
        manager.load_script("event_error", &path).expect("load");
        let message = MockMessage::empty();
        let err = manager
            .handle_event(Event::None, &message)
            .expect_err("expected error");
        let text = err.to_string();
        assert!(text.contains("lua on_event error (None)"), "{text}");
        manager.unload_script("event_error").expect("unload");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn event_tables_include_new_payloads() {
        let path = temp_script(
            "event_payloads",
            r#"
            function on_event(evt)
                if evt.type == "FileNew" then
                    assert(evt.remote_file ~= nil)
                    assert(evt.remote_file.name == "clip.wav")
                elseif evt.type == "BannedUser" then
                    assert(evt.banned_user ~= nil)
                    assert(evt.banned_user.owner == "owner")
                elseif evt.type == "DesktopInput" then
                    assert(evt.desktop_input ~= nil)
                    assert(evt.desktop_input.key_code == 65)
                elseif evt.type == "StreamMediaFile" then
                    assert(evt.media_file_info ~= nil)
                    assert(evt.media_file_info.name == "stream.ogg")
                elseif evt.type == "AudioInput" then
                    assert(evt.audio_input_progress ~= nil)
                    assert(evt.audio_input_progress.stream_id == 9)
                end
                return false
            end
        "#,
        );
        let mut manager = ScriptManager::new();
        manager.load_script("payloads", &path).expect("load");

        let mut remote_file = teamtalk::client::ffi::RemoteFile {
            nChannelID: 1,
            nFileID: 2,
            ..Default::default()
        };
        copy_tt("clip.wav", &mut remote_file.szFileName);

        let mut banned_user = teamtalk::client::ffi::BannedUser::default();
        copy_tt("owner", &mut banned_user.szOwner);

        let desktop_input = teamtalk::client::ffi::DesktopInput {
            uMousePosX: 1,
            uMousePosY: 2,
            uKeyCode: 65,
            uKeyState: teamtalk::client::ffi::DesktopKeyState::DESKTOPKEYSTATE_DOWN as u32,
        };

        let mut media_file = teamtalk::client::ffi::MediaFileInfo::default();
        copy_tt("stream.ogg", &mut media_file.szFileName);

        let audio_input = teamtalk::client::ffi::AudioInputProgress {
            nStreamID: 9,
            uQueueMSec: 1,
            uElapsedMSec: 2,
        };

        manager
            .handle_event(
                Event::FileNew,
                &MockMessage::remote_file(Event::FileNew, remote_file),
            )
            .expect("file new");
        manager
            .handle_event(Event::BannedUser, &MockMessage::banned_user(banned_user))
            .expect("banned user");
        manager
            .handle_event(
                Event::DesktopInput,
                &MockMessage::desktop_input(desktop_input, 3),
            )
            .expect("desktop input");
        manager
            .handle_event(
                Event::StreamMediaFile,
                &MockMessage::media_file_info(Event::StreamMediaFile, media_file),
            )
            .expect("media file info");
        manager
            .handle_event(
                Event::AudioInput,
                &MockMessage::audio_input_progress(audio_input, 4),
            )
            .expect("audio input");

        manager.unload_script("payloads").expect("unload");
        let _ = fs::remove_file(path);
    }
}
