#![cfg(feature = "mock")]
//! Coverage for `bot::Scheduler` registration, enable/disable, removal,
//! tick delivery and `one_shot` vs recurring semantics.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use teamtalk::MemoryStateStore;
use teamtalk::bot::{JobErrorPolicy, Scheduler};
use teamtalk::client::Client;
use teamtalk::client::backend::MockBackend;

fn client() -> Client {
    let backend = Arc::new(MockBackend::new());
    Client::with_backend(backend).expect("mock client")
}

fn store() -> MemoryStateStore {
    MemoryStateStore::new()
}

#[test]
fn scheduler_new_is_empty_and_next_run_delay_is_bounded() {
    let scheduler = Scheduler::new();
    assert!(scheduler.job_names().is_empty());
    // No jobs → fall back to 1h sentinel so callers can idle.
    assert!(scheduler.next_run_delay() >= Duration::from_secs(60));
}

#[test]
fn every_named_registers_job_and_lists_it() {
    let mut scheduler = Scheduler::new();
    scheduler.every_named(
        "heartbeat",
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    assert_eq!(scheduler.job_names(), vec!["heartbeat"]);
    assert_eq!(scheduler.is_enabled("heartbeat"), Some(true));
}

#[test]
fn every_generates_unique_job_names() {
    let mut scheduler = Scheduler::new();
    scheduler.every(
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    scheduler.every(
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    let names = scheduler.job_names();
    assert_eq!(names.len(), 2);
    assert_ne!(names[0], names[1]);
}

#[test]
fn remove_returns_true_only_when_job_existed() {
    let mut scheduler = Scheduler::new();
    scheduler.every_named(
        "a",
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    assert!(scheduler.remove("a"));
    assert!(!scheduler.remove("a"));
    assert!(scheduler.job_names().is_empty());
}

#[test]
fn set_enabled_toggles_job_status() {
    let mut scheduler = Scheduler::new();
    scheduler.every_named(
        "x",
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    scheduler.set_enabled("x", false);
    assert_eq!(scheduler.is_enabled("x"), Some(false));
    scheduler.set_enabled("x", true);
    assert_eq!(scheduler.is_enabled("x"), Some(true));
}

#[test]
fn set_enabled_on_missing_job_is_noop() {
    let mut scheduler = Scheduler::new();
    scheduler.set_enabled("ghost", false);
    assert_eq!(scheduler.is_enabled("ghost"), None);
}

#[test]
fn is_enabled_returns_none_for_missing_job() {
    let scheduler = Scheduler::new();
    assert_eq!(scheduler.is_enabled("ghost"), None);
}

#[test]
fn recurring_job_runs_multiple_times_across_ticks() {
    let mut scheduler = Scheduler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&counter);
    scheduler.every_named(
        "tick",
        Duration::from_millis(10),
        JobErrorPolicy::KeepRunning,
        move |_, _| {
            c.fetch_add(1, Ordering::SeqCst);
            Ok(())
        },
    );

    let client = client();
    let mut state = store();
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(15));
        scheduler.tick(&client, &mut state).unwrap();
    }
    assert!(
        counter.load(Ordering::SeqCst) >= 3,
        "recurring job should have fired at least 3 times"
    );
}

#[test]
fn after_job_is_one_shot_and_disables_itself() {
    let mut scheduler = Scheduler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&counter);
    scheduler.after("once", Duration::from_millis(10), move |_, _| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(())
    });

    let client = client();
    let mut state = store();
    for _ in 0..5 {
        thread::sleep(Duration::from_millis(15));
        scheduler.tick(&client, &mut state).unwrap();
    }
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    // One-shot jobs are fully removed from the scheduler once they finish.
    assert_eq!(scheduler.is_enabled("once"), None);
    assert!(scheduler.job_names().is_empty());
}

#[test]
fn disabled_job_does_not_run_on_tick() {
    let mut scheduler = Scheduler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&counter);
    scheduler.every_named(
        "x",
        Duration::from_millis(10),
        JobErrorPolicy::KeepRunning,
        move |_, _| {
            c.fetch_add(1, Ordering::SeqCst);
            Ok(())
        },
    );
    scheduler.set_enabled("x", false);

    let client = client();
    let mut state = store();
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(15));
        scheduler.tick(&client, &mut state).unwrap();
    }
    assert_eq!(counter.load(Ordering::SeqCst), 0);
}

#[test]
fn next_run_delay_only_considers_enabled_jobs() {
    let mut scheduler = Scheduler::new();
    scheduler.every_named(
        "a",
        Duration::from_millis(50),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    scheduler.every_named(
        "b",
        Duration::from_secs(3600),
        JobErrorPolicy::KeepRunning,
        |_, _| Ok(()),
    );
    scheduler.set_enabled("a", false);
    // Only `b` remains enabled, so the soonest delay should be near 3600s.
    assert!(scheduler.next_run_delay() >= Duration::from_secs(60));
}
