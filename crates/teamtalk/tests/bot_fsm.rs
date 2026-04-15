#![cfg(feature = "bot")]

use std::time::Duration;
use teamtalk::types::UserId;
use teamtalk::{
    DialogFlow, DialogMachine, DialogState, DialogStatus, DialogTimeoutPolicy, MemoryStateStore,
};

#[test]
fn dialog_machine_roundtrip() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    fsm.start(UserId(42), "onboarding", "ask_name");
    let cur = fsm.current(UserId(42)).expect("state exists");
    assert_eq!(cur.dialog, "onboarding");
    assert_eq!(cur.step, "ask_name");

    let next = fsm.advance(UserId(42), "ask_email").expect("advance state");
    assert_eq!(next.step, "ask_email");

    let stopped = fsm.stop(UserId(42)).expect("stop state");
    assert_eq!(stopped.dialog, "onboarding");
    assert!(fsm.current(UserId(42)).is_none());
}

#[test]
fn dialog_flow_contains_start_and_steps() {
    let flow = DialogFlow::new("onboarding", "ask_name")
        .step("ask_email")
        .step("done");
    assert_eq!(flow.name(), "onboarding");
    assert_eq!(flow.start_step(), "ask_name");
    assert!(flow.contains_step("ask_name"));
    assert!(flow.contains_step("ask_email"));
    assert!(!flow.contains_step("missing"));
    assert_eq!(flow.next_step("ask_name"), Some("ask_email"));
    assert_eq!(flow.next_step("ask_email"), Some("done"));
    assert_eq!(flow.next_step("done"), None);
    assert_eq!(flow.previous_step("ask_email"), Some("ask_name"));
    assert_eq!(flow.previous_step("done"), Some("ask_email"));
    assert!(flow.is_start_step("ask_name"));
    assert!(flow.is_terminal_step("done"));
}

#[test]
fn dialog_state_decodes_legacy_format() {
    let state = DialogState::decode("legacy|step1").expect("legacy state");
    assert_eq!(state.dialog, "legacy");
    assert_eq!(state.step, "step1");
    assert_eq!(state.status, DialogStatus::Active);
    assert_eq!(state.deadline_unix_ms, None);
    assert!(state.metadata.is_empty());
}

#[test]
fn dialog_machine_preserves_metadata_and_timeout_state() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    let state = DialogState::new("wizard", "ask_name")
        .with_timeout(Duration::from_secs(30))
        .with_metadata([("locale", "ru"), ("mode", "guided")]);
    fsm.start_state(UserId(7), state);

    let cur = fsm.current_live(UserId(7)).expect("live state exists");
    assert_eq!(cur.metadata("locale"), Some("ru"));
    assert_eq!(cur.metadata("mode"), Some("guided"));
    assert!(cur.deadline_unix_ms.is_some());
    assert!(cur.session_id().is_some());

    let updated = fsm
        .set_metadata(UserId(7), "locale", "en")
        .expect("metadata updated");
    assert_eq!(updated.metadata("locale"), Some("en"));

    let (updated, removed) = fsm
        .remove_metadata(UserId(7), "mode")
        .expect("metadata removed");
    assert_eq!(removed.as_deref(), Some("guided"));
    assert_eq!(updated.metadata("mode"), None);
}

#[test]
fn dialog_machine_pause_resume_and_active_state() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    fsm.start(UserId(9), "wizard", "step1");
    let paused = fsm.pause(UserId(9)).expect("paused");
    assert_eq!(paused.status, DialogStatus::Paused);
    assert!(fsm.current_active(UserId(9)).is_none());
    assert!(fsm.current_live(UserId(9)).is_some());

    let resumed = fsm.resume(UserId(9)).expect("resumed");
    assert_eq!(resumed.status, DialogStatus::Active);
    assert_eq!(fsm.current_active(UserId(9)).expect("active").step, "step1");
}

#[test]
fn dialog_machine_removes_expired_state_from_live_queries() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    let expired = DialogState::new("wizard", "step1")
        .with_deadline_unix_ms(1)
        .with_metadata([("key", "value")]);
    fsm.start_state(UserId(11), expired);

    assert!(fsm.current_live(UserId(11)).is_none());
    assert!(fsm.current(UserId(11)).is_none());
}

#[test]
fn dialog_machine_pauses_when_timeout_policy_requests_pause() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    let state = DialogState::new("wizard", "step1")
        .with_deadline_unix_ms(1)
        .with_timeout_policy(DialogTimeoutPolicy::Pause);
    fsm.start_state(UserId(12), state);

    let paused = fsm.current_live(UserId(12)).expect("paused state kept");
    assert_eq!(paused.status, DialogStatus::Paused);
    assert_eq!(paused.timeout_policy(), DialogTimeoutPolicy::Pause);
    assert_eq!(paused.deadline_unix_ms, None);
    assert!(fsm.current_active(UserId(12)).is_none());
}

#[test]
fn dialog_machine_advances_flow_in_order() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);
    let flow = DialogFlow::new("wizard", "ask_name")
        .step("ask_email")
        .step("done");

    let restarted = fsm.restart_flow(UserId(21), &flow);
    assert_eq!(restarted.dialog, "wizard");
    assert_eq!(restarted.step, "ask_name");

    let next = fsm
        .advance_flow(UserId(21), &flow)
        .expect("advance to ask_email");
    assert_eq!(next.step, "ask_email");

    let next = fsm
        .advance_flow(UserId(21), &flow)
        .expect("advance to done");
    assert_eq!(next.step, "done");

    assert!(fsm.advance_flow(UserId(21), &flow).is_none());
}

#[test]
fn dialog_machine_restart_flow_rotates_session_id() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);
    let flow = DialogFlow::new("wizard", "ask_name").step("done");

    fsm.start_state(UserId(30), DialogState::new("wizard", "ask_name"));
    let first = fsm
        .current(UserId(30))
        .and_then(|state| state.session_id().map(str::to_owned));

    let restarted = fsm.restart_flow(UserId(30), &flow);
    assert_eq!(restarted.step, "ask_name");
    let second = restarted.session_id().map(str::to_owned);

    assert!(first.is_some());
    assert!(second.is_some());
    assert_ne!(first, second);
}
