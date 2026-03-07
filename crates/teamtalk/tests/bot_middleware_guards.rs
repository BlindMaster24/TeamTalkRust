#![cfg(all(feature = "bot", feature = "mock"))]

use std::sync::Arc;
use teamtalk::client::backend::MockBackend;
use teamtalk::events::Event;
use teamtalk::mock::MockMessage;
use teamtalk::types::{ChannelId, UserId};
use teamtalk::{
    Client, HandlerResult, MemoryStateStore, RequireChannelMessage, RequireCommand,
    RequireCommandPrefix, RequirePrivateMessage, RequireUserIds, RequireUserType, Router,
    StateStore,
};
use teamtalk_sys::{TextMsgType, User};

fn mock_client_with_backend() -> (Client, Arc<MockBackend>) {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend.clone()).expect("mock client");
    (client, backend)
}

fn mock_client() -> Client {
    mock_client_with_backend().0
}

fn text_message(msg_type: TextMsgType, text: &str, channel_id: ChannelId) -> teamtalk::Message {
    MockMessage::text(msg_type, UserId(7), UserId(1), channel_id, "alice", text)
}

#[test]
fn require_private_message_blocks_channel_dispatch() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequirePrivateMessage)
        .on_any(|ctx| {
            ctx.user_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let message = text_message(TextMsgType::MSGTYPE_CHANNEL, "hello", ChannelId(10));
    router
        .dispatch(&client, Event::TextMessage, &message, &mut store)
        .expect("dispatch");

    assert_eq!(store.get("u:7:ran"), None);
}

#[test]
fn require_channel_message_allows_channel_dispatch() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequireChannelMessage)
        .on_any(|ctx| {
            ctx.channel_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let message = text_message(TextMsgType::MSGTYPE_CHANNEL, "hello", ChannelId(10));
    router
        .dispatch(&client, Event::TextMessage, &message, &mut store)
        .expect("dispatch");

    assert_eq!(store.get("c:10:ran"), Some("yes".to_owned()));
}

#[test]
fn require_command_matches_specific_command() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequireCommand::new("ping"))
        .on_any(|ctx| {
            ctx.user_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let ping = text_message(TextMsgType::MSGTYPE_USER, "/ping", ChannelId(0));
    router
        .dispatch(&client, Event::TextMessage, &ping, &mut store)
        .expect("ping dispatch");
    assert_eq!(store.get("u:7:ran"), Some("yes".to_owned()));

    let other = text_message(TextMsgType::MSGTYPE_USER, "/pong", ChannelId(0));
    store.remove("u:7:ran");
    router
        .dispatch(&client, Event::TextMessage, &other, &mut store)
        .expect("pong dispatch");
    assert_eq!(store.get("u:7:ran"), None);
}

#[test]
fn require_command_prefix_matches_expected_prefix() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequireCommandPrefix::new('!'))
        .on_any(|ctx| {
            ctx.user_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let bang = text_message(TextMsgType::MSGTYPE_USER, "!ping", ChannelId(0));
    router
        .dispatch(&client, Event::TextMessage, &bang, &mut store)
        .expect("bang dispatch");
    assert_eq!(store.get("u:7:ran"), Some("yes".to_owned()));

    let slash = text_message(TextMsgType::MSGTYPE_USER, "/ping", ChannelId(0));
    store.remove("u:7:ran");
    router
        .dispatch(&client, Event::TextMessage, &slash, &mut store)
        .expect("slash dispatch");
    assert_eq!(store.get("u:7:ran"), None);
}

#[test]
fn require_user_ids_blocks_unlisted_sender() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequireUserIds::new(vec![UserId(7)]))
        .on_any(|ctx| {
            ctx.user_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let allowed = text_message(TextMsgType::MSGTYPE_USER, "/ping", ChannelId(0));
    router
        .dispatch(&client, Event::TextMessage, &allowed, &mut store)
        .expect("allowed dispatch");
    assert_eq!(store.get("u:7:ran"), Some("yes".to_owned()));

    let other = MockMessage::text(
        TextMsgType::MSGTYPE_USER,
        UserId(8),
        UserId(1),
        ChannelId(0),
        "bob",
        "/ping",
    );
    store.remove("u:7:ran");
    router
        .dispatch(&client, Event::TextMessage, &other, &mut store)
        .expect("other dispatch");
    assert_eq!(store.get("u:7:ran"), None);
}

#[test]
fn require_user_type_matches_sender_type() {
    let (client, backend) = mock_client_with_backend();
    let mut store = MemoryStateStore::new();
    let mut router = Router::new()
        .use_middleware(RequireUserType::new(vec![42]))
        .on_any(|ctx| {
            ctx.user_state_set("ran", "yes");
            Ok(HandlerResult::Continue)
        });

    let mut raw = unsafe { std::mem::zeroed::<User>() };
    raw.nUserID = 7;
    raw.uUserType = 42;
    backend.set_user(raw);

    let message = text_message(TextMsgType::MSGTYPE_USER, "/ping", ChannelId(0));
    router
        .dispatch(&client, Event::TextMessage, &message, &mut store)
        .expect("dispatch");
    assert_eq!(store.get("u:7:ran"), Some("yes".to_owned()));
}
