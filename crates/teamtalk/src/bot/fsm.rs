use super::storage::StateStore;
use crate::types::UserId;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DIALOG_ENCODING_VERSION: &str = "v2";
const INTERNAL_SESSION_KEY: &str = "__session";
const INTERNAL_TIMEOUT_POLICY_KEY: &str = "__timeout_policy";
static NEXT_DIALOG_SESSION: AtomicU64 = AtomicU64::new(1);

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogStatus {
    Active,
    Paused,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTimeoutPolicy {
    Clear,
    Pause,
}

impl DialogTimeoutPolicy {
    fn encode(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Pause => "pause",
        }
    }

    fn decode(raw: &str) -> Option<Self> {
        match raw {
            "clear" => Some(Self::Clear),
            "pause" => Some(Self::Pause),
            _ => None,
        }
    }
}

impl DialogStatus {
    fn encode(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
        }
    }

    fn decode(raw: &str) -> Option<Self> {
        match raw {
            "active" => Some(Self::Active),
            "paused" => Some(Self::Paused),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DialogState {
    pub dialog: String,
    pub step: String,
    pub status: DialogStatus,
    pub deadline_unix_ms: Option<u64>,
    pub metadata: Vec<(String, String)>,
}

impl DialogState {
    pub fn new(dialog: impl Into<String>, step: impl Into<String>) -> Self {
        Self {
            dialog: dialog.into(),
            step: step.into(),
            status: DialogStatus::Active,
            deadline_unix_ms: None,
            metadata: Vec::new(),
        }
    }

    pub fn with_deadline_unix_ms(mut self, deadline_unix_ms: u64) -> Self {
        self.deadline_unix_ms = Some(deadline_unix_ms);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.deadline_unix_ms = Some(now_unix_ms().saturating_add(duration_to_millis(timeout)));
        self
    }

    pub fn with_status(mut self, status: DialogStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_timeout_policy(mut self, policy: DialogTimeoutPolicy) -> Self {
        self.set_metadata(INTERNAL_TIMEOUT_POLICY_KEY, policy.encode());
        self
    }

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

    pub fn is_active(&self) -> bool {
        matches!(self.status, DialogStatus::Active)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.status, DialogStatus::Paused)
    }

    pub fn is_expired(&self) -> bool {
        self.is_expired_at(now_unix_ms())
    }

    pub fn is_expired_at(&self, now_unix_ms: u64) -> bool {
        self.deadline_unix_ms
            .is_some_and(|deadline| deadline <= now_unix_ms)
    }

    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata
            .iter()
            .find_map(|(existing, value)| (existing == key).then_some(value.as_str()))
    }

    pub fn session_id(&self) -> Option<&str> {
        self.metadata(INTERNAL_SESSION_KEY)
    }

    pub fn timeout_policy(&self) -> DialogTimeoutPolicy {
        self.metadata(INTERNAL_TIMEOUT_POLICY_KEY)
            .and_then(DialogTimeoutPolicy::decode)
            .unwrap_or(DialogTimeoutPolicy::Clear)
    }

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

    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        let index = self
            .metadata
            .iter()
            .position(|(existing, _)| existing == key)?;
        Some(self.metadata.remove(index).1)
    }

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

pub struct DialogMachine<'a> {
    store: &'a mut dyn StateStore,
    prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogFlow {
    name: String,
    start_step: String,
    steps: Vec<String>,
}

impl DialogFlow {
    pub fn new(name: impl Into<String>, start_step: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start_step: start_step.into(),
            steps: Vec::new(),
        }
    }

    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.steps.push(step.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_step(&self) -> &str {
        &self.start_step
    }

    pub fn steps(&self) -> &[String] {
        &self.steps
    }

    pub fn contains_step(&self, step: &str) -> bool {
        step == self.start_step || self.steps.iter().any(|item| item == step)
    }

    pub fn next_step(&self, step: &str) -> Option<&str> {
        if step == self.start_step {
            return self.steps.first().map(String::as_str);
        }

        self.steps
            .windows(2)
            .find_map(|window| (window[0] == step).then_some(window[1].as_str()))
    }

    pub fn previous_step(&self, step: &str) -> Option<&str> {
        if let Some(first) = self.steps.first()
            && first == step
        {
            return Some(&self.start_step);
        }

        self.steps
            .windows(2)
            .find_map(|window| (window[1] == step).then_some(window[0].as_str()))
    }

    pub fn is_start_step(&self, step: &str) -> bool {
        step == self.start_step
    }

    pub fn is_terminal_step(&self, step: &str) -> bool {
        self.steps.last().is_some_and(|last| last == step)
    }
}

impl<'a> DialogMachine<'a> {
    pub fn new(store: &'a mut dyn StateStore) -> Self {
        Self {
            store,
            prefix: "bot:dialog".to_owned(),
        }
    }

    pub fn with_prefix(store: &'a mut dyn StateStore, prefix: impl Into<String>) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    fn key(&self, source_id: UserId) -> String {
        format!("{}:{}", self.prefix, source_id.raw())
    }

    pub fn start(&mut self, source_id: UserId, dialog: impl Into<String>, step: impl Into<String>) {
        self.start_state(source_id, DialogState::new(dialog, step));
    }

    pub fn start_state(&mut self, source_id: UserId, state: DialogState) {
        self.store
            .set(self.key(source_id), self.prepare_state(state).encode());
    }

    pub fn current(&self, source_id: UserId) -> Option<DialogState> {
        self.store
            .get(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }

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

    pub fn current_active(&mut self, source_id: UserId) -> Option<DialogState> {
        let state = self.current_live(source_id)?;
        state.is_active().then_some(state)
    }

    pub fn is_in(&mut self, source_id: UserId, dialog: &str, step: &str) -> bool {
        self.current_active(source_id)
            .is_some_and(|state| state.dialog == dialog && state.step == step)
    }

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

    pub fn pause(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.status = DialogStatus::Paused)
    }

    pub fn resume(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.status = DialogStatus::Active)
    }

    pub fn set_timeout(&mut self, source_id: UserId, timeout: Duration) -> Option<DialogState> {
        self.update(source_id, |state| {
            state.deadline_unix_ms =
                Some(now_unix_ms().saturating_add(duration_to_millis(timeout)));
        })
    }

    pub fn clear_timeout(&mut self, source_id: UserId) -> Option<DialogState> {
        self.update(source_id, |state| state.deadline_unix_ms = None)
    }

    pub fn set_timeout_policy(
        &mut self,
        source_id: UserId,
        policy: DialogTimeoutPolicy,
    ) -> Option<DialogState> {
        self.update(source_id, |state| {
            state.set_metadata(INTERNAL_TIMEOUT_POLICY_KEY, policy.encode())
        })
    }

    pub fn timeout_policy(&mut self, source_id: UserId) -> Option<DialogTimeoutPolicy> {
        self.current(source_id).map(|state| state.timeout_policy())
    }

    pub fn metadata(&mut self, source_id: UserId, key: &str) -> Option<String> {
        self.current_live(source_id)?
            .metadata(key)
            .map(ToOwned::to_owned)
    }

    pub fn set_metadata(
        &mut self,
        source_id: UserId,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<DialogState> {
        let key = key.into();
        let value = value.into();
        self.update(source_id, move |state| {
            state.set_metadata(key.clone(), value.clone())
        })
    }

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

    pub fn stop(&mut self, source_id: UserId) -> Option<DialogState> {
        self.store
            .remove(&self.key(source_id))
            .and_then(|raw| DialogState::decode(&raw))
    }

    pub fn restart_flow(&mut self, source_id: UserId, flow: &DialogFlow) -> DialogState {
        let state = DialogState::new(flow.name(), flow.start_step());
        self.start_state(source_id, state.clone());
        self.current(source_id).unwrap_or(state)
    }

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

    fn prepare_state(&self, mut state: DialogState) -> DialogState {
        if state.session_id().is_none() {
            state.set_metadata(INTERNAL_SESSION_KEY, generate_session_id());
        }
        state
    }
}

fn netstring(value: &str) -> String {
    format!("{}:{value},", value.len())
}

fn parse_netstring<'a>(input: &mut &'a str) -> Option<&'a str> {
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

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn duration_to_millis(duration: Duration) -> u64 {
    duration.as_millis().try_into().unwrap_or(u64::MAX)
}

fn generate_session_id() -> String {
    format!(
        "{}-{}",
        now_unix_ms(),
        NEXT_DIALOG_SESSION.fetch_add(1, Ordering::Relaxed)
    )
}
