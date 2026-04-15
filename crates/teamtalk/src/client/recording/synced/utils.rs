use crate::events::Event;
use crate::types::{Subscriptions, User, UserId};
use std::path::{Path, PathBuf};

pub(super) fn synced_audio_subscription_mask() -> Subscriptions {
    Subscriptions::all_audio()
}

pub(super) fn is_synced_bus_event(event: Event) -> bool {
    matches!(
        event,
        Event::UserJoined
            | Event::UserLeft
            | Event::AudioBlock
            | Event::ConnectSuccess
            | Event::ConnectionLost
            | Event::ConnectFailed
            | Event::ConnectCryptError
    )
}

pub(super) fn should_warn_missing_audio_subscriptions(
    subscribe_audio: bool,
    user: Option<&User>,
) -> bool {
    if subscribe_audio {
        return false;
    }
    let Some(current_user) = user else {
        return false;
    };
    !current_user.local_subscriptions.has(Subscriptions::VOICE)
        && !current_user
            .local_subscriptions
            .has(Subscriptions::MEDIAFILE)
}

pub(super) fn render_vars(template: &str, user_id: UserId, username: &str) -> String {
    template
        .replace("%user_id%", &user_id.raw().to_string())
        .replace("%username%", username)
}

pub(super) fn sanitized_filename(raw: &str) -> String {
    const FORBIDDEN: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut sanitized = String::with_capacity(raw.len());
    for ch in raw.chars() {
        let replacement = if ch.is_control() || FORBIDDEN.contains(&ch) {
            '_'
        } else {
            ch
        };
        sanitized.push(replacement);
    }
    let trimmed = sanitized.trim_matches([' ', '.']);
    if trimmed.is_empty() {
        "user".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(super) fn unique_recording_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }

    let stem = path
        .file_stem()
        .and_then(|v| v.to_str())
        .unwrap_or("recording");
    let ext = path.extension().and_then(|v| v.to_str());
    for idx in 1.. {
        let filename = match ext {
            Some(ext) if !ext.is_empty() => format!("{stem}-{idx}.{ext}"),
            _ => format!("{stem}-{idx}"),
        };
        let candidate = path.with_file_name(filename);
        if !candidate.exists() {
            return candidate;
        }
    }

    path.to_path_buf()
}
