#![cfg(feature = "bot")]

use teamtalk::{DialogFlow, DialogMachine, MemoryStateStore};

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
}
