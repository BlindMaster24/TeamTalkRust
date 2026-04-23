//! Mutating dialog façade over a [`StateStore`].
//!
//! [`DialogMachine`] borrows a `&mut dyn StateStore` and treats it as
//! a keyed bag of [`DialogState`] values, one per [`UserId`]. All
//! dialog lifecycle operations (start, advance, pause/resume, expire,
//! timeout-policy enforcement) live here.

use super::encoding::{
    INTERNAL_SESSION_KEY, INTERNAL_TIMEOUT_POLICY_KEY, duration_to_millis, generate_session_id,
    now_unix_ms,
};
use super::flow::DialogFlow;
use super::state::DialogState;
use super::status::{DialogStatus, DialogTimeoutPolicy};
use crate::bot::storage::StateStore;
use crate::types::UserId;
use std::time::Duration;

/// Mutating façade that stores dialog state per-user in a
/// [`StateStore`].
pub struct DialogMachine<'a> {
    store: &'a mut dyn StateStore,
    prefix: String,
}

impl<'a> DialogMachine<'a> {
    /// Creates a machine that stores entries under the default
    /// `bot:dialog` key prefix.
    pub fn new(store: &'a mut dyn StateStore) -> Self {
        Self {
            store,
            prefix: "bot:dialog".to_owned(),
        }
    }

    /// Creates a machine with a custom key prefix. Useful when
    /// multiple independent dialog machines share the same store.
    #[must_use]
    pub fn with_prefix(store: &'a mut dyn StateStore, prefix: impl Into<String>) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    fn key(&self, source_id: UserId) -> String {
        format!("{}:{}", self.prefix, source_id.raw())
    }

    /// Starts a fresh dialog for `source_id` at the given step.
    pub fn start(&mut self, source_id: UserId, dialog: impl Into<String>, step: impl Into<String>) {
        self.start_state(source_id, DialogState::new(dialog, step));
    }

    /// Starts a dialog at the given pre-built [`DialogState`].
    pub fn start_state(&mut self, source_id: UserId, state: DialogState) {
        self.store
            .set(self.key(source_id), Self::prepare_state(state).encode());
    }

    /// Returns the stored state for `source_id` without applying any
    /// expiration or timeout-policy side effects.
    #[allow(clippy::must_use_candidate)]
    pub fn current(&self, source_id: UserId) -> Option<DialogState> {
        self.store
            .get(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }

    /// Returns the stored state for `source_id`, applying the
    /// [`DialogTimeoutPolicy`] if its deadline has expired.
    pub fn current_live(&mut self, source_id: UserId) -> Option<DialogState> {
        let mut state = self.current(source_id)?;
        if state.is_expired() {
            match state.timeout_policy() {
                DialogTimeoutPolicy::Clear => {
                    let _ = self.stop(source_id);
                    return None;
                }
                DialogTimeoutPolicy::Pause => {
                    state.deadline_unix_ms = None;
                    state.status = DialogStatus::Paused;
                    self.store.set(self.key(source_id), state.encode());
                }
            }
        }
        Some(state)
    }

    /// Same as [`Self::current_live`] but only returns `Some` when the
    /// resulting state is [`DialogStatus::Active`].
    pub fn current_active(&mut self, source_id: UserId) -> Option<DialogState> {
        let state = self.current_live(source_id)?;
        state.is_active().then_some(state)
    }

    /// Returns `true` when the current active state has the given
    /// dialog/step pair.
    #[must_use]
    pub fn is_in(&mut self, source_id: UserId, dialog: &str, step: &str) -> bool {
        self.current_active(source_id)
            .is_some_and(|state| state.dialog == dialog && state.step == step)
    }

    /// Advances to the next step, marking the state as
    /// [`DialogStatus::Active`].
    pub fn advance(
        &mut self,
        source_id: UserId,
        next_step: impl Into<String>,
    ) -> Option<DialogState> {
        let mut state = self.current_live(source_id)?;
        state.step = next_step.into();
        state.status = DialogStatus::Active;
        self.store.set(self.key(source_id), state.encode());
        Some(state)
    }

    /// Marks the state as [`DialogStatus::Paused`].
    pub fn pause(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.status = DialogStatus::Paused)
    }

    /// Marks the state as [`DialogStatus::Active`].
    pub fn resume(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.status = DialogStatus::Active)
    }

    /// Sets a new relative deadline for the stored dialog.
    pub fn set_timeout(&mut self, source_id: UserId, timeout: Duration) -> Option<DialogState> {
        self.update(source_id, |state| {
            state.deadline_unix_ms =
                Some(now_unix_ms().saturating_add(duration_to_millis(timeout)));
        })
    }

    /// Clears any deadline on the stored dialog.
    pub fn clear_timeout(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.deadline_unix_ms = None)
    }

    /// Sets the [`DialogTimeoutPolicy`] on the stored dialog.
    pub fn set_timeout_policy(
        &mut self,
        source_id: UserId,
        policy: DialogTimeoutPolicy,
    ) -> Option<DialogState> {
        self.update(source_id, |state| {
            state.set_metadata(INTERNAL_TIMEOUT_POLICY_KEY, policy.encode());
        })
    }

    /// Returns the current [`DialogTimeoutPolicy`] for `source_id`.
    pub fn timeout_policy(&mut self, source_id: UserId) -> Option<DialogTimeoutPolicy> {
        self.current(source_id).map(|state| state.timeout_policy())
    }

    /// Returns the metadata value at `key` for the current live state.
    pub fn metadata(&mut self, source_id: UserId, key: &str) -> Option<String> {
        self.current_live(source_id)?
            .metadata(key)
            .map(ToOwned::to_owned)
    }

    /// Sets metadata on the stored dialog.
    pub fn set_metadata(
        &mut self,
        source_id: UserId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<DialogState> {
        let key = key.into();
        let value = value.into();
        self.update(source_id, move |state| {
            state.set_metadata(key.clone(), value.clone());
        })
    }

    /// Removes metadata at `key` from the stored dialog.
    pub fn remove_metadata(
        &mut self,
        source_id: UserId,
        key: &str,
    ) -> Option<(DialogState, Option<String>)> {
        let mut removed = None;
        let state = self.update(source_id, |state| {
            removed = state.remove_metadata(key);
        })?;
        Some((state, removed))
    }

    /// Removes the dialog for `source_id` and returns its last state.
    pub fn stop(&mut self, source_id: UserId) -> Option<DialogState> {
        self.store
            .remove(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }

    /// Resets the dialog to `flow.start_step()`, returning the new
    /// stored state.
    #[must_use]
    pub fn restart_flow(&mut self, source_id: UserId, flow: &DialogFlow) -> DialogState {
        let state = DialogState::new(flow.name(), flow.start_step());
        self.start_state(source_id, state.clone());
        self.current(source_id).unwrap_or(state)
    }

    /// Advances the dialog to the next step of `flow`, or returns
    /// `None` if the current dialog name does not match or the step
    /// has no successor.
    pub fn advance_flow(&mut self, source_id: UserId, flow: &DialogFlow) -> Option<DialogState> {
        let current = self.current_live(source_id)?;
        if !current.dialog.eq_ignore_ascii_case(flow.name()) {
            return None;
        }
        let next = flow.next_step(&current.step)?;
        self.advance(source_id, next)
    }

    fn update<F>(&mut self, source_id: UserId, mut update: F) -> Option<DialogState>
    where
        F: FnMut(&mut DialogState),
    {
        let mut state = self.current_live(source_id)?;
        update(&mut state);
        self.store.set(self.key(source_id), state.encode());
        Some(state)
    }

    fn prepare_state(mut state: DialogState) -> DialogState {
        if state.session_id().is_none() {
            state.set_metadata(INTERNAL_SESSION_KEY, generate_session_id());
        }
        state
    }
}
