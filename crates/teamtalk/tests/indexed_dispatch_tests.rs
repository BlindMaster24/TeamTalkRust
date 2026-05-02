#![cfg(feature = "mock")]
//! Integration tests for the indexed event-dispatch path.
//!
//! Both the high-level `Dispatcher` (`crate::dispatch::Dispatcher`) and
//! the `Client` subscription bus (`Client::on_event` / `Client::on_any`)
//! are backed by the same per-discriminant index: registered handlers
//! are bucketed into a `HashMap<Discriminant<Event>, Vec<index>>` plus
//! a wildcard `Vec<index>`, and dispatch merges the two index lists in
//! insertion order.

use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};

use teamtalk::client::backend::MockBackend;
use teamtalk::client::ffi;
use teamtalk::dispatch::{DispatchFlow, Dispatcher};
use teamtalk::mock::{MockClient, MockMessage, MockUserBuilder};
use teamtalk::types::{ChannelId, UserId};
use teamtalk::{Client, Event};

fn push_user_joined(mock: &mut MockClient, id: i32) {
    let user = MockUserBuilder::new(UserId(id))
        .username(&format!("u{id}"))
        .nickname(&format!("n{id}"));
    mock.push_user_joined(user);
}

fn push_text(mock: &mut MockClient, from: i32, to: i32, channel: i32, text: &str) {
    let from_user = format!("u{from}");
    let msg = MockMessage::text(
        ffi::TextMsgType::MSGTYPE_USER,
        UserId(from),
        UserId(to),
        ChannelId(channel),
        &from_user,
        text,
    );
    mock.push_text_message(msg);
}

fn record(buf: &Arc<Mutex<Vec<&'static str>>>, tag: &'static str) -> DispatchFlow {
    buf.lock().unwrap().push(tag);
    DispatchFlow::Continue
}

fn lock_log<'a>(buf: &'a Arc<Mutex<Vec<&'static str>>>) -> MutexGuard<'a, Vec<&'static str>> {
    buf.lock().unwrap()
}

fn mock_client() -> Client {
    let backend = Arc::new(MockBackend::new());
    Client::with_backend(backend).expect("mock client")
}

#[test]
fn specific_handler_only_fires_for_matching_discriminant() {
    let mut mock = MockClient::new();
    push_user_joined(&mut mock, 1);
    push_text(&mut mock, 1, 2, 3, "hi");
    mock.push_event(Event::ConnectSuccess);

    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);

    let mut dispatcher = Dispatcher::new(mock)
        .on_user_joined(move |_| record(&b1, "join"))
        .on_text_message(move |_| record(&b2, "text"));

    dispatcher.step(0);
    dispatcher.step(0);
    dispatcher.step(0);

    assert_eq!(*lock_log(&log_buf), vec!["join", "text"]);
}

#[test]
fn wildcard_handler_fires_for_every_event() {
    let mut mock = MockClient::new();
    push_user_joined(&mut mock, 1);
    mock.push_event(Event::ConnectSuccess);
    mock.push_event(Event::ConnectionLost);

    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let b1 = Arc::clone(&log_buf);
    let mut dispatcher = Dispatcher::new(mock).on_any(move |_| record(&b1, "any"));

    for _ in 0..3 {
        dispatcher.step(0);
    }

    assert_eq!(*lock_log(&log_buf), vec!["any", "any", "any"]);
}

#[test]
fn insertion_order_is_preserved_across_specific_and_wildcard() {
    // Register handlers in an interleaved order:
    //   #0 text  (specific)
    //   #1 any
    //   #2 text  (specific)
    //   #3 any
    //   #4 user  (specific, unrelated for this event)
    // On a TextMessage the expected firing order — by insertion — is:
    // text0, any1, text2, any3. Specific handlers that don't match the
    // event must never fire.
    let mut mock = MockClient::new();
    push_text(&mut mock, 1, 2, 3, "hi");

    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let b0 = Arc::clone(&log_buf);
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);
    let b3 = Arc::clone(&log_buf);
    let b4 = Arc::clone(&log_buf);

    let mut dispatcher = Dispatcher::new(mock)
        .on_text_message(move |_| record(&b0, "text0"))
        .on_any(move |_| record(&b1, "any1"))
        .on_text_message(move |_| record(&b2, "text2"))
        .on_any(move |_| record(&b3, "any3"))
        .on_user_joined(move |_| record(&b4, "user4"));

    dispatcher.step(0);

    assert_eq!(*lock_log(&log_buf), vec!["text0", "any1", "text2", "any3"]);
}

#[test]
fn unmatched_event_fires_only_wildcards() {
    let mut mock = MockClient::new();
    mock.push_event(Event::ConnectionLost);

    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);
    let b3 = Arc::clone(&log_buf);

    let mut dispatcher = Dispatcher::new(mock)
        .on_user_joined(move |_| record(&b1, "user"))
        .on_any(move |_| record(&b2, "any"))
        .on_text_message(move |_| record(&b3, "text"));

    dispatcher.step(0);

    assert_eq!(*lock_log(&log_buf), vec!["any"]);
}

#[test]
fn stop_flow_propagates_from_any_slot() {
    let mut mock = MockClient::new();
    push_user_joined(&mut mock, 1);

    let mut dispatcher = Dispatcher::new(mock)
        .on_user_joined(|_| DispatchFlow::Continue)
        .on_any(|_| DispatchFlow::Stop)
        .on_user_joined(|_| DispatchFlow::Continue);

    assert!(matches!(dispatcher.step(0), DispatchFlow::Stop));
}

// --- Client subscription bus ------------------------------------------------

#[test]
fn client_bus_specific_and_wildcard_dispatch_in_insertion_order() {
    let client = mock_client();
    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

    let b0 = Arc::clone(&log_buf);
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);
    let b3 = Arc::clone(&log_buf);

    let _id0 = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b0.lock().unwrap().push("join0"));
    let _id1 = client
        .on_any()
        .subscribe(move |_| b1.lock().unwrap().push("any1"));
    let _id2 = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b2.lock().unwrap().push("join2"));
    let _id3 = client
        .on_event(Event::TextMessage)
        .subscribe(move |_| b3.lock().unwrap().push("text3"));

    client.mock_dispatch_bus_for_tests(Event::UserJoined, 0);
    assert_eq!(
        *lock_log(&log_buf),
        vec!["join0", "any1", "join2"],
        "UserJoined must fire join0 (#0), any1 (#1), join2 (#2) in insertion order, skipping text3"
    );

    lock_log(&log_buf).clear();
    client.mock_dispatch_bus_for_tests(Event::TextMessage, 0);
    assert_eq!(*lock_log(&log_buf), vec!["any1", "text3"]);

    lock_log(&log_buf).clear();
    client.mock_dispatch_bus_for_tests(Event::ConnectSuccess, 0);
    assert_eq!(*lock_log(&log_buf), vec!["any1"]);
}

#[test]
fn client_bus_unsubscribe_rebuilds_indexes() {
    let client = mock_client();
    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

    let b0 = Arc::clone(&log_buf);
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);

    let id0 = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b0.lock().unwrap().push("join0"));
    let _id1 = client
        .on_any()
        .subscribe(move |_| b1.lock().unwrap().push("any1"));
    let _id2 = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b2.lock().unwrap().push("join2"));
    assert_eq!(client.event_subscription_count(), 3);

    client.mock_dispatch_bus_for_tests(Event::UserJoined, 0);
    assert_eq!(*lock_log(&log_buf), vec!["join0", "any1", "join2"]);

    assert!(client.unsubscribe_event(id0));
    assert_eq!(client.event_subscription_count(), 2);

    lock_log(&log_buf).clear();
    client.mock_dispatch_bus_for_tests(Event::UserJoined, 0);
    assert_eq!(*lock_log(&log_buf), vec!["any1", "join2"]);
}

#[test]
fn client_bus_unsubscribe_group_removes_from_all_buckets() {
    let client = mock_client();
    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

    let b0 = Arc::clone(&log_buf);
    let b1 = Arc::clone(&log_buf);
    let b2 = Arc::clone(&log_buf);
    let b3 = Arc::clone(&log_buf);

    let _ = client
        .on_event(Event::UserJoined)
        .group("feature-x")
        .subscribe(move |_| b0.lock().unwrap().push("join0"));
    let _ = client
        .on_any()
        .group("feature-x")
        .subscribe(move |_| b1.lock().unwrap().push("any1"));
    let _ = client
        .on_event(Event::TextMessage)
        .group("feature-x")
        .subscribe(move |_| b2.lock().unwrap().push("text2"));
    let _ = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b3.lock().unwrap().push("join3-other"));

    assert_eq!(client.event_subscription_count(), 4);
    let removed = client.unsubscribe_event_group("feature-x");
    assert_eq!(removed, 3);
    assert_eq!(client.event_subscription_count(), 1);

    client.mock_dispatch_bus_for_tests(Event::UserJoined, 0);
    client.mock_dispatch_bus_for_tests(Event::TextMessage, 0);
    client.mock_dispatch_bus_for_tests(Event::ConnectSuccess, 0);
    assert_eq!(*lock_log(&log_buf), vec!["join3-other"]);
}

#[test]
fn client_bus_clear_subscriptions_resets_both_buckets() {
    let client = mock_client();
    let log_buf: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let b0 = Arc::clone(&log_buf);
    let b1 = Arc::clone(&log_buf);

    let _ = client
        .on_event(Event::UserJoined)
        .subscribe(move |_| b0.lock().unwrap().push("join"));
    let _ = client
        .on_any()
        .subscribe(move |_| b1.lock().unwrap().push("any"));
    assert_eq!(client.event_subscription_count(), 2);

    client.clear_event_subscriptions();
    assert_eq!(client.event_subscription_count(), 0);

    client.mock_dispatch_bus_for_tests(Event::UserJoined, 0);
    client.mock_dispatch_bus_for_tests(Event::TextMessage, 0);
    assert!(lock_log(&log_buf).is_empty());
}
