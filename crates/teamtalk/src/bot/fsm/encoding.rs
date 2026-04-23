//! Shared constants, time helpers, and the netstring wire format used
//! by [`crate::bot::DialogState::encode`] / [`crate::bot::DialogState::decode`].
//!
//! The dialog encoding is a sequence of [netstrings][dj-netstring] of the
//! form `len:value,`. This makes the format self-delimiting, forward-
//! compatible (old decoders can be replaced by version-aware ones by
//! bumping [`DIALOG_ENCODING_VERSION`]), and trivially safe to embed
//! inside the arbitrary `String` values accepted by [`StateStore`][1]
//! backends.
//!
//! [dj-netstring]: https://cr.yp.to/proto/netstrings.txt
//! [1]: crate::bot::StateStore

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Current dialog wire-format version; bump on any breaking encoding
/// change so [`super::state::DialogState::decode`] can fall back to the
/// legacy `dialog|step` path for values stored before netstrings were
/// introduced.
pub(super) const DIALOG_ENCODING_VERSION: &str = "v2";

/// Metadata key under which [`super::state::DialogState`] stores the
/// per-session id. Internal; never exposed to user metadata.
pub(super) const INTERNAL_SESSION_KEY: &str = "__session";

/// Metadata key under which [`super::state::DialogState`] stores the
/// encoded [`super::status::DialogTimeoutPolicy`]. Internal; never
/// exposed to user metadata.
pub(super) const INTERNAL_TIMEOUT_POLICY_KEY: &str = "__timeout_policy";

static NEXT_DIALOG_SESSION: AtomicU64 = AtomicU64::new(1);

pub(super) fn netstring(value: &str) -> String {
    format!("{}:{value},", value.len())
}

pub(super) fn parse_netstring<'a>(input: &mut &'a str) -> Option<&'a str> {
    let colon = input.find(':')?;
    let len = input[..colon].parse::<usize>().ok()?;
    let start = colon + 1;
    let end = start.checked_add(len)?;
    let trailing = end.checked_add(1)?;
    if input.len() < trailing || input.as_bytes().get(end).copied()? != b',' {
        return None;
    }
    let value = &input[start..end];
    *input = &input[trailing..];
    Some(value)
}

pub(super) fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

pub(super) fn duration_to_millis(duration: Duration) -> u64 {
    duration.as_millis().try_into().unwrap_or(u64::MAX)
}

pub(super) fn generate_session_id() -> String {
    format!(
        "{}-{}",
        now_unix_ms(),
        NEXT_DIALOG_SESSION.fetch_add(1, Ordering::Relaxed)
    )
}
