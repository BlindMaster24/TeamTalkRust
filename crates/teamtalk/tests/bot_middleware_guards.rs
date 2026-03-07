#![cfg(all(feature = "bot", feature = "mock"))]

use std::sync::Arc;
use teamtalk::client::backend::MockBackend;
use teamtalk::events::Event;
use teamtalk::mock::MockMessage;
use teamtalk::types::{ChannelId, UserId};
use teamtalk::{
    Client, HandlerResult, MemoryStateStore, RequireChannelMessage, RequireCommand,
    RequirePrivateMessage, Router, StateStore,
};
use teamtalk_sys::TextMsgType;

fn mock_client() -> Client {
    let backend = Arc::new(MockBackend::new());
    Client::with_backend(backend).expect("mock client")
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
