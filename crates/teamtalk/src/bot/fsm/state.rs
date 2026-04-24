//! Per-user dialog state record plus its netstring wire encoding.

use super::encoding::{
    DIALOG_ENCODING_VERSION, INTERNAL_SESSION_KEY, INTERNAL_TIMEOUT_POLICY_KEY, duration_to_millis,
    netstring, now_unix_ms, parse_netstring,
};
use super::status::{DialogStatus, DialogTimeoutPolicy};
use std::time::Duration;

/// Persistent dialog state for a single user.
///
/// Stored as the encoded value in a [`crate::bot::StateStore`] keyed
/// by `<prefix>:<user_id>`. The encoding is a sequence of netstrings
/// so it round-trips cleanly through any string-based backend and can
/// be extended with additional metadata without breaking older
/// readers (see [`DIALOG_ENCODING_VERSION`]).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DialogState {
    /// Dialog name (first axis of the state).
    pub dialog: String,
    /// Current step within the dialog.
    pub step: String,
    /// Current lifecycle status.
    pub status: DialogStatus,
    /// Optional absolute deadline in milliseconds since the Unix epoch.
    pub deadline_unix_ms: Option<u64>,
    /// Arbitrary user metadata plus a few internal keys (session id,
    /// timeout policy). Kept as `Vec<(String, String)>` rather than a
    /// map to preserve insertion order across encode/decode.
    pub metadata: Vec<(String, String)>,
}

impl DialogState {
    /// Builds a fresh [`DialogStatus::Active`] state with no deadline
    /// and no metadata.
    pub fn new(dialog: impl Into<String>, step: impl Into<String>) -> Self {
        Self {
            dialog: dialog.into(),
            step: step.into(),
            status: DialogStatus::Active,
            deadline_unix_ms: None,
            metadata: Vec::new(),
        }
    }

    /// Sets an absolute deadline in milliseconds since the Unix epoch.
    #[must_use]
    pub fn with_deadline_unix_ms(mut self, deadline_unix_ms: u64) -> Self {
        self.deadline_unix_ms = Some(deadline_unix_ms);
        self
    }

    /// Sets a relative deadline by adding `timeout` to the current
    /// wall-clock time.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.deadline_unix_ms = Some(now_unix_ms().saturating_add(duration_to_millis(timeout)));
        self
    }

    /// Sets the initial [`DialogStatus`].
    #[must_use]
    pub fn with_status(mut self, status: DialogStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the [`DialogTimeoutPolicy`] on the state.
    #[must_use]
    pub fn with_timeout_policy(mut self, policy: DialogTimeoutPolicy) -> Self {
        self.set_metadata(INTERNAL_TIMEOUT_POLICY_KEY, policy.encode());
        self
    }

    /// Replaces the metadata vector with the given `(key, value)`
    /// pairs.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.metadata = metadata
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }

    /// Returns `true` if the state is [`DialogStatus::Active`].
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self.status, DialogStatus::Active)
    }

    /// Returns `true` if the state is [`DialogStatus::Paused`].
    #[must_use]
    pub fn is_paused(&self) -> bool {
        matches!(self.status, DialogStatus::Paused)
    }

    /// Returns `true` if the state has a deadline that is in the past
    /// at the current wall-clock time.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.is_expired_at(now_unix_ms())
    }

    /// Returns `true` if the state has a deadline that is less than or
    /// equal to `now_unix_ms`.
    #[must_use]
    pub fn is_expired_at(&self, now_unix_ms: u64) -> bool {
        self.deadline_unix_ms
            .is_some_and(|deadline| deadline <= now_unix_ms)
    }

    /// Returns the metadata value stored at `key`, if any.
    #[allow(clippy::must_use_candidate)]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata
            .iter()
            .find_map(|(existing, value)| (existing == key).then_some(value.as_str()))
    }

    /// Returns the internal per-state session id if set.
    #[allow(clippy::must_use_candidate)]
    pub fn session_id(&self) -> Option<&str> {
        self.metadata(INTERNAL_SESSION_KEY)
    }

    /// Returns the effective [`DialogTimeoutPolicy`]
    /// (defaulting to [`DialogTimeoutPolicy::Clear`]).
    #[must_use]
    pub fn timeout_policy(&self) -> DialogTimeoutPolicy {
        self.metadata(INTERNAL_TIMEOUT_POLICY_KEY)
            .and_then(DialogTimeoutPolicy::decode)
            .unwrap_or(DialogTimeoutPolicy::Clear)
    }

    /// Inserts or overwrites the metadata value at `key`.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value = value.into();
        if let Some((_, existing)) = self
            .metadata
            .iter_mut()
            .find(|(existing, _)| *existing == key)
        {
            *existing = value;
            return;
        }
        self.metadata.push((key, value));
    }

    /// Removes the metadata entry at `key`, returning its value.
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        let index = self
            .metadata
            .iter()
            .position(|(existing, _)| existing == key)?;
        Some(self.metadata.remove(index).1)
    }

    /// Encodes the state as a sequence of netstrings suitable for
    /// storing in a [`crate::bot::StateStore`].
    #[must_use]
    pub fn encode(&self) -> String {
        let mut fields = Vec::with_capacity(6 + self.metadata.len() * 2);
        fields.push(netstring(DIALOG_ENCODING_VERSION));
        fields.push(netstring(self.status.encode()));
        fields.push(netstring(
            &self
                .deadline_unix_ms
                .map(|value| value.to_string())
                .unwrap_or_default(),
        ));
        fields.push(netstring(&self.dialog));
        fields.push(netstring(&self.step));
        fields.push(netstring(&self.metadata.len().to_string()));
        for (key, value) in &self.metadata {
            fields.push(netstring(key));
            fields.push(netstring(value));
        }
        fields.concat()
    }

    /// Decodes a state previously produced by [`Self::encode`], with
    /// a best-effort fallback to the legacy `dialog|step` pipe
    /// encoding used before [`DIALOG_ENCODING_VERSION`].
    #[allow(clippy::must_use_candidate)]
    pub fn decode(raw: &str) -> Option<Self> {
        if !raw.contains(':') {
            let (dialog, step) = raw.split_once('|')?;
            if dialog.is_empty() || step.is_empty() {
                return None;
            }
            return Some(Self::new(dialog, step));
        }

        let mut remainder = raw;
        let Some(version) = parse_netstring(&mut remainder) else {
            let (dialog, step) = raw.split_once('|')?;
            if dialog.is_empty() || step.is_empty() {
                return None;
            }
            return Some(Self::new(dialog, step));
        };
        if version != DIALOG_ENCODING_VERSION {
            let (dialog, step) = raw.split_once('|')?;
            if dialog.is_empty() || step.is_empty() {
                return None;
            }
            return Some(Self::new(dialog, step));
        }

        let status = DialogStatus::decode(parse_netstring(&mut remainder)?)?;
        let deadline_unix_ms = {
            let value = parse_netstring(&mut remainder)?;
            if value.is_empty() {
                None
            } else {
                Some(value.parse::<u64>().ok()?)
            }
        };
        let dialog = parse_netstring(&mut remainder)?.to_owned();
        let step = parse_netstring(&mut remainder)?.to_owned();
        if dialog.is_empty() || step.is_empty() {
            return None;
        }
        let metadata_len = parse_netstring(&mut remainder)?.parse::<usize>().ok()?;
        let mut metadata = Vec::with_capacity(metadata_len);
        for _ in 0..metadata_len {
            let key = parse_netstring(&mut remainder)?.to_owned();
            let value = parse_netstring(&mut remainder)?.to_owned();
            metadata.push((key, value));
        }
        if !remainder.is_empty() {
            return None;
        }

        Some(Self {
            dialog,
            step,
            status,
            deadline_unix_ms,
            metadata,
        })
    }
}
