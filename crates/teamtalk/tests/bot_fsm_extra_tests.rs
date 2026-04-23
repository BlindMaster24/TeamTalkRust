#![cfg(feature = "mock")]
//! Additional `DialogState` / `DialogFlow` / `DialogMachine` coverage that
//! pins down behaviour not exercised by `bot_fsm.rs`.

use std::thread;
use std::time::Duration;

use teamtalk::bot::{DialogFlow, DialogMachine, DialogState, DialogStatus, DialogTimeoutPolicy};
use teamtalk::types::UserId;
use teamtalk::{MemoryStateStore, StateStore};

fn user() -> UserId {
    UserId(42)
}

#[test]
fn dialog_state_defaults_to_active_and_no_deadline() {
    let state = DialogState::new("d", "s");
    assert!(state.is_active());
    assert!(!state.is_paused());
    assert!(!state.is_expired());
    assert_eq!(state.deadline_unix_ms, None);
    assert!(state.metadata.is_empty());
}

#[test]
fn dialog_state_is_expired_at_past_deadline() {
    let state = DialogState::new("d", "s").with_deadline_unix_ms(100);
    assert!(state.is_expired_at(200));
    assert!(state.is_expired_at(100)); // boundary: deadline <= now is expired
    assert!(!state.is_expired_at(50));
}

#[test]
fn dialog_state_default_timeout_policy_is_clear() {
    let state = DialogState::new("d", "s");
    assert_eq!(state.timeout_policy(), DialogTimeoutPolicy::Clear);
}

#[test]
fn dialog_state_with_timeout_policy_is_round_tripped_through_encode_decode() {
    let state = DialogState::new("d", "s").with_timeout_policy(DialogTimeoutPolicy::Pause);
    assert_eq!(state.timeout_policy(), DialogTimeoutPolicy::Pause);
    let decoded = DialogState::decode(&state.encode()).expect("decode");
    assert_eq!(decoded.timeout_policy(), DialogTimeoutPolicy::Pause);
}

#[test]
fn dialog_state_metadata_round_trips_including_order() {
    let state = DialogState::new("d", "s").with_metadata(vec![("a", "1"), ("b", "2"), ("c", "3")]);
    let decoded = DialogState::decode(&state.encode()).expect("decode");
    let decoded_meta: Vec<(String, String)> = decoded
        .metadata
        .iter()
        .filter(|(k, _)| !k.starts_with("__"))
        .cloned()
        .collect();
    assert_eq!(
        decoded_meta,
        vec![
            ("a".into(), "1".into()),
            ("b".into(), "2".into()),
            ("c".into(), "3".into()),
        ]
    );
}

#[test]
fn dialog_state_set_metadata_replaces_existing_value_without_growing() {
    let mut state = DialogState::new("d", "s");
    state.set_metadata("k", "v1");
    state.set_metadata("k", "v2");
    assert_eq!(state.metadata("k"), Some("v2"));
    assert_eq!(
        state.metadata.iter().filter(|(k, _)| k == "k").count(),
        1,
        "replacing a metadata key must not create a second entry"
    );
}

#[test]
fn dialog_state_remove_metadata_returns_value_and_removes_pair() {
    let mut state = DialogState::new("d", "s");
    state.set_metadata("k", "v");
    assert_eq!(state.remove_metadata("k"), Some("v".into()));
    assert_eq!(state.metadata("k"), None);
    assert_eq!(state.remove_metadata("k"), None);
}

#[test]
fn dialog_state_decode_legacy_pipe_format_is_accepted() {
    let state = DialogState::decode("signup|step-1").expect("legacy format");
    assert_eq!(state.dialog, "signup");
    assert_eq!(state.step, "step-1");
    assert!(state.is_active());
    assert_eq!(state.deadline_unix_ms, None);
}

#[test]
fn dialog_state_decode_rejects_malformed_input() {
    assert!(DialogState::decode("not-a-real-encoding").is_none());
    assert!(DialogState::decode("|").is_none());
    assert!(DialogState::decode("dialog|").is_none());
    assert!(DialogState::decode("|step").is_none());
}

#[test]
fn dialog_flow_navigation_handles_first_last_and_missing_steps() {
    let flow = DialogFlow::new("onboarding", "start")
        .step("a")
        .step("b")
        .step("c");
    assert!(flow.is_start_step("start"));
    assert!(!flow.is_start_step("a"));
    assert!(flow.contains_step("start"));
    assert!(flow.contains_step("a"));
    assert!(!flow.contains_step("z"));
    assert_eq!(flow.next_step("start"), Some("a"));
    assert_eq!(flow.next_step("a"), Some("b"));
    assert_eq!(flow.next_step("c"), None);
    assert_eq!(flow.previous_step("a"), Some("start"));
    assert_eq!(flow.previous_step("b"), Some("a"));
    assert_eq!(flow.previous_step("start"), None);
    assert!(flow.is_terminal_step("c"));
    assert!(!flow.is_terminal_step("b"));
}

#[test]
fn dialog_machine_assigns_unique_session_ids_per_start() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start(UserId(1), "flow", "s");
    let first = machine
        .current(UserId(1))
        .unwrap()
        .session_id()
        .map(String::from);
    machine.start(UserId(2), "flow", "s");
    let second = machine
        .current(UserId(2))
        .unwrap()
        .session_id()
        .map(String::from);
    assert!(first.is_some() && second.is_some());
    assert_ne!(first, second);
}

#[test]
fn dialog_machine_advance_live_clears_paused_status() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start_state(
        user(),
        DialogState::new("d", "s1").with_status(DialogStatus::Paused),
    );
    let advanced = machine.advance(user(), "s2").expect("advanced");
    assert_eq!(advanced.step, "s2");
    assert!(
        advanced.is_active(),
        "advancing must re-activate a paused dialog"
    );
}

#[test]
fn dialog_machine_current_live_with_clear_policy_stops_expired_dialog() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start_state(
        user(),
        DialogState::new("d", "s")
            .with_deadline_unix_ms(1)
            .with_timeout_policy(DialogTimeoutPolicy::Clear),
    );
    assert!(machine.current_live(user()).is_none());
    assert!(
        machine.current(user()).is_none(),
        "Clear policy must have removed the persisted state"
    );
}

#[test]
fn dialog_machine_current_live_with_pause_policy_flips_to_paused() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start_state(
        user(),
        DialogState::new("d", "s")
            .with_deadline_unix_ms(1)
            .with_timeout_policy(DialogTimeoutPolicy::Pause),
    );
    let live = machine.current_live(user()).expect("state is preserved");
    assert!(live.is_paused());
    assert_eq!(live.deadline_unix_ms, None);
    // Subsequent calls should keep seeing the paused state.
    assert!(machine.current(user()).unwrap().is_paused());
    assert!(machine.current_active(user()).is_none());
}

#[test]
fn dialog_machine_set_and_clear_timeout_update_deadline() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start(user(), "d", "s");
    let with_timeout = machine
        .set_timeout(user(), Duration::from_secs(60))
        .unwrap();
    assert!(with_timeout.deadline_unix_ms.is_some());
    let cleared = machine.clear_timeout(user()).unwrap();
    assert_eq!(cleared.deadline_unix_ms, None);
}

#[test]
fn dialog_machine_metadata_helpers_persist_across_reloads() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start(user(), "d", "s");
    machine.set_metadata(user(), "k", "v");
    assert_eq!(machine.metadata(user(), "k"), Some("v".into()));
    let (state, removed) = machine.remove_metadata(user(), "k").unwrap();
    assert_eq!(removed, Some("v".into()));
    assert!(state.metadata("k").is_none());
    assert_eq!(machine.metadata(user(), "k"), None);
}

#[test]
fn dialog_machine_with_prefix_uses_custom_key_namespace() {
    let mut store = MemoryStateStore::new();
    {
        let mut machine = DialogMachine::with_prefix(&mut store, "svc:flow");
        machine.start(user(), "d", "s");
    }
    let keys = store.keys("svc:flow:");
    assert_eq!(
        keys.len(),
        1,
        "prefix should namespace the dialog storage key"
    );
    assert!(store.keys("bot:dialog").is_empty());
}

#[test]
fn dialog_machine_is_in_ignores_paused_state() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    machine.start_state(
        user(),
        DialogState::new("d", "s").with_status(DialogStatus::Paused),
    );
    assert!(!machine.is_in(user(), "d", "s"));
    machine.resume(user());
    assert!(machine.is_in(user(), "d", "s"));
}

#[test]
fn dialog_machine_restart_flow_produces_fresh_session_id() {
    let mut store = MemoryStateStore::new();
    let mut machine = DialogMachine::new(&mut store);
    let flow = DialogFlow::new("wizard", "start").step("a").step("b");
    machine.start(user(), "wizard", "start");
    let before = machine
        .current(user())
        .and_then(|s| s.session_id().map(String::from));
    // Ensure the subsequent session id is strictly different (generator counter).
    thread::sleep(Duration::from_millis(2));
    let after_state = machine.restart_flow(user(), &flow);
    assert_eq!(after_state.step, "start");
    let after = after_state.session_id().map(String::from);
    assert!(before.is_some() && after.is_some() && before != after);
}
