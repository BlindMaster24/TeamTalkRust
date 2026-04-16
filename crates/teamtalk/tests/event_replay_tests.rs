#![cfg(feature = "mock")]

use teamtalk::EventRecorder;
use teamtalk::EventReplayer;
use teamtalk::RecordedEvent;
use teamtalk::events::Event;

#[test]
fn recorder_stores_events() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    recorder.record(Event::UserLoggedIn, 5);
    assert_eq!(recorder.len(), 2);
    assert!(!recorder.is_empty());
    let events = recorder.events();
    assert_eq!(events[0].event, "ConnectSuccess");
    assert_eq!(events[0].source, 0);
    assert_eq!(events[1].event, "UserLoggedIn");
    assert_eq!(events[1].source, 5);
}

#[test]
fn recorder_json_roundtrip() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    recorder.record(Event::UserJoined, 42);
    let json = recorder.to_json().unwrap();
    let restored = EventRecorder::from_json(&json).unwrap();
    assert_eq!(restored.len(), 2);
    assert_eq!(restored.events()[0].event, "ConnectSuccess");
    assert_eq!(restored.events()[1].source, 42);
}

#[test]
fn replayer_from_recorder() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    recorder.record(Event::CmdSuccess, 1);
    let replayer = EventReplayer::from_recorder(recorder);
    assert_eq!(replayer.remaining(), 2);
}

#[test]
fn replayer_from_json() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    let json = recorder.to_json().unwrap();
    let replayer = EventReplayer::from_json(&json).unwrap();
    assert_eq!(replayer.remaining(), 1);
}

#[test]
fn replayer_replay_next_drains() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    recorder.record(Event::UserLoggedIn, 5);
    let backend = teamtalk::client::backend::MockBackend::new();
    let mut replayer = EventReplayer::from_recorder(recorder);
    assert!(replayer.replay_next(&backend));
    assert!(replayer.replay_next(&backend));
    assert!(!replayer.replay_next(&backend));
    assert_eq!(replayer.remaining(), 0);
}

#[test]
fn replayer_replay_all() {
    let mut recorder = EventRecorder::new();
    recorder.record(Event::ConnectSuccess, 0);
    recorder.record(Event::UserLoggedIn, 5);
    recorder.record(Event::CmdSuccess, 1);
    let backend = teamtalk::client::backend::MockBackend::new();
    let mut replayer = EventReplayer::from_recorder(recorder);
    replayer.replay_all(&backend);
    assert_eq!(replayer.remaining(), 0);
}

#[test]
fn recorded_event_clone_eq() {
    let e = RecordedEvent {
        event: "ConnectSuccess".to_string(),
        source: 0,
    };
    let e2 = e.clone();
    assert_eq!(e.event, e2.event);
    assert_eq!(e.source, e2.source);
}

#[test]
fn recorder_default_is_empty() {
    let recorder = EventRecorder::default();
    assert!(recorder.is_empty());
    assert_eq!(recorder.len(), 0);
}
