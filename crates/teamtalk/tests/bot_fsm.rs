#![cfg(feature = "bot")]

use std::time::Duration;
use teamtalk::{DialogFlow, DialogMachine, DialogState, DialogStatus, MemoryStateStore};

#[test]
fn dialog_machine_roundtrip() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    fsm.start(42, "onboarding", "ask_name");
    let cur = fsm.current(42).expect("state exists");
    assert_eq!(cur.dialog, "onboarding");
    assert_eq!(cur.step, "ask_name");

    let next = fsm.advance(42, "ask_email").expect("advance state");
    assert_eq!(next.step, "ask_email");

    let stopped = fsm.stop(42).expect("stop state");
    assert_eq!(stopped.dialog, "onboarding");
    assert!(fsm.current(42).is_none());
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
    fsm.start_state(7, state);

    let cur = fsm.current_live(7).expect("live state exists");
    assert_eq!(cur.metadata("locale"), Some("ru"));
    assert_eq!(cur.metadata("mode"), Some("guided"));
    assert!(cur.deadline_unix_ms.is_some());

    let updated = fsm
        .set_metadata(7, "locale", "en")
        .expect("metadata updated");
    assert_eq!(updated.metadata("locale"), Some("en"));

    let (updated, removed) = fsm.remove_metadata(7, "mode").expect("metadata removed");
    assert_eq!(removed.as_deref(), Some("guided"));
    assert_eq!(updated.metadata("mode"), None);
}

#[test]
fn dialog_machine_pause_resume_and_active_state() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    fsm.start(9, "wizard", "step1");
    let paused = fsm.pause(9).expect("paused");
    assert_eq!(paused.status, DialogStatus::Paused);
    assert!(fsm.current_active(9).is_none());
    assert!(fsm.current_live(9).is_some());

    let resumed = fsm.resume(9).expect("resumed");
    assert_eq!(resumed.status, DialogStatus::Active);
    assert_eq!(fsm.current_active(9).expect("active").step, "step1");
}

#[test]
fn dialog_machine_removes_expired_state_from_live_queries() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);

    let expired = DialogState::new("wizard", "step1")
        .with_deadline_unix_ms(1)
        .with_metadata([("key", "value")]);
    fsm.start_state(11, expired);

    assert!(fsm.current_live(11).is_none());
    assert!(fsm.current(11).is_none());
}

#[test]
fn dialog_machine_advances_flow_in_order() {
    let mut store = MemoryStateStore::new();
    let mut fsm = DialogMachine::new(&mut store);
    let flow = DialogFlow::new("wizard", "ask_name")
        .step("ask_email")
        .step("done");

    let restarted = fsm.restart_flow(21, &flow);
    assert_eq!(restarted.dialog, "wizard");
    assert_eq!(restarted.step, "ask_name");

    let next = fsm.advance_flow(21, &flow).expect("advance to ask_email");
    assert_eq!(next.step, "ask_email");

    let next = fsm.advance_flow(21, &flow).expect("advance to done");
    assert_eq!(next.step, "done");

    assert!(fsm.advance_flow(21, &flow).is_none());
}
