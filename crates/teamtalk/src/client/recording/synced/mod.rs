use crate::events::{Error, Event, Result};
use crate::types::{User, UserId};
use crate::utils::UnpoisonedMutex;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use teamtalk_sys as ffi;

use super::super::{Client, Message};
use super::RecordingSampleFormat;
use crate::AudioBlockView;
mod utils;
use utils::{
    is_synced_bus_event, render_vars, sanitized_filename, should_warn_missing_audio_subscriptions,
    synced_audio_subscription_mask, unique_recording_path,
};

mod session;
mod writer;

pub use session::{
    SilencePolicy, SyncedUserRecording, SyncedUserRecordingBus, SyncedUserRecordingOptions,
    SyncedUserRecordingSession,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Subscriptions;

    #[test]
    fn synced_mask_is_audio_only() {
        assert_eq!(
            synced_audio_subscription_mask().raw(),
            Subscriptions::all_audio().raw()
        );
    }

    #[test]
    fn warn_when_manual_subscriptions_missing() {
        let user = User::default();
        assert!(should_warn_missing_audio_subscriptions(false, Some(&user)));
    }

    #[test]
    fn no_warn_when_manual_voice_present() {
        let user = User {
            local_subscriptions: Subscriptions::from_raw(Subscriptions::VOICE),
            ..User::default()
        };
        assert!(!should_warn_missing_audio_subscriptions(false, Some(&user)));
    }

    #[test]
    fn no_warn_when_subscribe_audio_enabled() {
        let user = User::default();
        assert!(!should_warn_missing_audio_subscriptions(true, Some(&user)));
    }

    #[test]
    fn sanitize_filename_replaces_path_separators_and_reserved_chars() {
        assert_eq!(
            sanitized_filename("..\\evil/name:bad*file?"),
            "_evil_name_bad_file_"
        );
    }

    #[test]
    fn unique_recording_path_adds_suffix_for_existing_file() {
        let base = std::env::temp_dir().join(format!(
            "teamtalk-synced-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("ts")
                .as_nanos()
        ));
        std::fs::create_dir_all(&base).expect("mkdir");
        let path = base.join("voice.wav");
        std::fs::write(&path, b"x").expect("write");

        let unique = unique_recording_path(&path);
        assert_ne!(unique, path);
        assert_eq!(
            unique.file_name().and_then(|v| v.to_str()),
            Some("voice-1.wav")
        );

        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn synced_bus_filter_includes_connection_events() {
        assert!(is_synced_bus_event(Event::ConnectSuccess));
        assert!(is_synced_bus_event(Event::ConnectionLost));
        assert!(is_synced_bus_event(Event::ConnectFailed));
        assert!(is_synced_bus_event(Event::ConnectCryptError));
        assert!(is_synced_bus_event(Event::AudioBlock));
    }
}
