#![cfg(feature = "mock")]
//! Additional high-level `Dispatcher` behaviour tests. Complements
//! `dispatch_tests.rs` and `indexed_dispatch_tests.rs` by exercising the
//! public builder surface, shortcut handlers and flow propagation.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use teamtalk::Event;
use teamtalk::dispatch::{DispatchFlow, Dispatcher};
use teamtalk::mock::{MockClient, MockUserBuilder};
use teamtalk::types::UserId;

fn bump(
    counter: &Arc<AtomicUsize>,
) -> impl FnMut(teamtalk::dispatch::EventContext<'_>) -> DispatchFlow + Send + 'static + use<> {
    let c = Arc::clone(counter);
    move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        DispatchFlow::Continue
    }
}

#[test]
fn dispatcher_step_returns_none_flow_when_queue_empty() {
    let mock = MockClient::new();
    let mut dispatcher = Dispatcher::new(mock).on_any(|_| DispatchFlow::Continue);
    // No events queued: step should report Continue and not trigger any handler.
    assert!(matches!(dispatcher.step(0), DispatchFlow::Continue));
}

#[test]
fn dispatcher_on_event_filters_by_event_variant() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    mock.push_event(Event::ConnectFailed);
    mock.push_event(Event::ConnectionLost);

    let success = Arc::new(AtomicUsize::new(0));
    let lost = Arc::new(AtomicUsize::new(0));
    let mut dispatcher = Dispatcher::new(mock)
        .on_event(Event::ConnectSuccess, bump(&success))
        .on_event(Event::ConnectionLost, bump(&lost));

    for _ in 0..3 {
        dispatcher.step(0);
    }
    assert_eq!(success.load(Ordering::SeqCst), 1);
    assert_eq!(lost.load(Ordering::SeqCst), 1);
}

#[test]
fn dispatcher_on_any_fires_for_every_event_type() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    mock.push_event(Event::ConnectFailed);
    mock.push_event(Event::ConnectionLost);

    let total = Arc::new(AtomicUsize::new(0));
    let mut dispatcher = Dispatcher::new(mock).on_any(bump(&total));
    for _ in 0..3 {
        dispatcher.step(0);
    }
    assert_eq!(total.load(Ordering::SeqCst), 3);
}

#[test]
fn dispatcher_stop_flow_from_any_handler_stops_step() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    let mut dispatcher = Dispatcher::new(mock).on_any(|_| DispatchFlow::Stop);
    assert!(matches!(dispatcher.step(0), DispatchFlow::Stop));
}

#[test]
fn dispatcher_connect_shortcut_handlers_fire_for_expected_events() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    mock.push_event(Event::ConnectionLost);
    mock.push_event(Event::ConnectFailed);
    mock.push_event(Event::CmdError);

    let ok = Arc::new(AtomicUsize::new(0));
    let lost = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let cmd_err = Arc::new(AtomicUsize::new(0));

    let mut dispatcher = Dispatcher::new(mock)
        .on_connect_success(bump(&ok))
        .on_connection_lost(bump(&lost))
        .on_connect_failed(bump(&failed))
        .on_command_error(bump(&cmd_err));

    for _ in 0..4 {
        dispatcher.step(0);
    }

    assert_eq!(ok.load(Ordering::SeqCst), 1);
    assert_eq!(lost.load(Ordering::SeqCst), 1);
    assert_eq!(failed.load(Ordering::SeqCst), 1);
    assert_eq!(cmd_err.load(Ordering::SeqCst), 1);
}

#[test]
fn dispatcher_user_joined_shortcut_receives_user_payload() {
    let mut mock = MockClient::new();
    mock.push_user_joined(
        MockUserBuilder::new(UserId(7))
            .username("alice")
            .nickname("a"),
    );
    let seen = Arc::new(AtomicUsize::new(0));
    let captured_id = Arc::new(AtomicUsize::new(0));

    let seen_c = Arc::clone(&seen);
    let captured_c = Arc::clone(&captured_id);
    let mut dispatcher = Dispatcher::new(mock).on_user_joined(move |ctx| {
        seen_c.fetch_add(1, Ordering::SeqCst);
        if let Some(user) = ctx.message().user() {
            captured_c.store(user.id.0 as usize, Ordering::SeqCst);
        }
        DispatchFlow::Continue
    });
    dispatcher.step(0);

    assert_eq!(seen.load(Ordering::SeqCst), 1);
    assert_eq!(captured_id.load(Ordering::SeqCst), 7);
}

#[test]
fn dispatcher_multiple_specific_handlers_for_same_event_all_fire_in_order() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);

    let order = Arc::new(std::sync::Mutex::new(Vec::<u32>::new()));
    let o1 = Arc::clone(&order);
    let o2 = Arc::clone(&order);
    let o3 = Arc::clone(&order);

    let mut dispatcher = Dispatcher::new(mock)
        .on_event(Event::ConnectSuccess, move |_| {
            o1.lock().unwrap().push(1);
            DispatchFlow::Continue
        })
        .on_event(Event::ConnectSuccess, move |_| {
            o2.lock().unwrap().push(2);
            DispatchFlow::Continue
        })
        .on_event(Event::ConnectSuccess, move |_| {
            o3.lock().unwrap().push(3);
            DispatchFlow::Continue
        });
    dispatcher.step(0);

    assert_eq!(order.lock().unwrap().as_slice(), &[1, 2, 3]);
}

#[test]
fn dispatcher_specific_and_wildcard_interleave_in_insertion_order() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);

    let order = Arc::new(std::sync::Mutex::new(Vec::<&'static str>::new()));
    let a = Arc::clone(&order);
    let b = Arc::clone(&order);
    let c = Arc::clone(&order);

    let mut dispatcher = Dispatcher::new(mock)
        .on_event(Event::ConnectSuccess, move |_| {
            a.lock().unwrap().push("spec");
            DispatchFlow::Continue
        })
        .on_any(move |_| {
            b.lock().unwrap().push("any1");
            DispatchFlow::Continue
        })
        .on_event(Event::ConnectSuccess, move |_| {
            c.lock().unwrap().push("spec2");
            DispatchFlow::Continue
        });
    dispatcher.step(0);

    assert_eq!(order.lock().unwrap().as_slice(), &["spec", "any1", "spec2"]);
}

#[test]
fn dispatcher_stop_from_first_handler_still_surfaces_stop_and_lets_siblings_run() {
    // Dispatcher semantics: all matching handlers fire for a single event, but
    // a `Stop` from any of them causes `step()` to return `Stop` so the caller
    // can exit the run loop after the current event finishes dispatching.
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);

    let later = Arc::new(AtomicUsize::new(0));
    let later_c = Arc::clone(&later);
    let mut dispatcher = Dispatcher::new(mock)
        .on_event(Event::ConnectSuccess, |_| DispatchFlow::Stop)
        .on_event(Event::ConnectSuccess, move |_| {
            later_c.fetch_add(1, Ordering::SeqCst);
            DispatchFlow::Continue
        });
    assert!(matches!(dispatcher.step(0), DispatchFlow::Stop));
    assert_eq!(later.load(Ordering::SeqCst), 1);
}

#[test]
fn dispatcher_new_starts_with_no_handlers_and_drains_events_silently() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    mock.push_event(Event::ConnectFailed);
    let mut dispatcher = Dispatcher::new(mock);
    for _ in 0..2 {
        assert!(matches!(dispatcher.step(0), DispatchFlow::Continue));
    }
}

#[test]
fn dispatcher_text_message_shortcut_fires_only_for_text_messages() {
    use teamtalk::client::ffi;
    use teamtalk::mock::MockMessage;
    use teamtalk::types::ChannelId;

    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectSuccess);
    let msg = MockMessage::text(
        ffi::TextMsgType::MSGTYPE_USER,
        UserId(1),
        UserId(2),
        ChannelId(3),
        "alice",
        "hi",
    );
    mock.push_text_message(msg);
    mock.push_event(Event::ConnectionLost);

    let text_hits = Arc::new(AtomicUsize::new(0));
    let hits = Arc::clone(&text_hits);
    let mut dispatcher = Dispatcher::new(mock).on_text_message(move |_| {
        hits.fetch_add(1, Ordering::SeqCst);
        DispatchFlow::Continue
    });
    for _ in 0..3 {
        dispatcher.step(0);
    }
    assert_eq!(text_hits.load(Ordering::SeqCst), 1);
}
