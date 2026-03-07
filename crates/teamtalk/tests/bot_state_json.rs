#![cfg(all(feature = "bot-serde", feature = "mock"))]

use std::sync::Arc;
use teamtalk::client::backend::MockBackend;
use teamtalk::events::Event;
use teamtalk::mock::MockMessage;
use teamtalk::types::{ChannelId, UserId};
use teamtalk::{Client, Context, MemoryStateStore};
use teamtalk_sys::TextMsgType;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct JsonState {
    name: String,
    count: u32,
}

fn make_context<'a>(
    client: &'a Client,
    message: &'a teamtalk::Message,
    store: &'a mut MemoryStateStore,
) -> Context<'a> {
    Context {
        client,
        event: Event::TextMessage,
        message,
        command: None,
        state: store,
    }
}

fn text_message(text: &str) -> teamtalk::Message {
    MockMessage::text(
        TextMsgType::MSGTYPE_USER,
        UserId(7),
        UserId(1),
        ChannelId(0),
        "alice",
        text,
    )
}

#[test]
fn context_json_helpers_roundtrip_across_scopes() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("mock client");
    let message = text_message("hello");
    let mut store = MemoryStateStore::new();
    let mut ctx = make_context(&client, &message, &mut store);

    let state = JsonState {
        name: "alice".to_owned(),
        count: 2,
    };

    ctx.state_set_json("plain", &state).expect("plain json");
    ctx.user_state_set_json("profile", &state)
        .expect("user json");
    ctx.global_state_set_json("config", &state)
        .expect("global json");
    ctx.dialog_start("wizard", "step1");
    ctx.dialog_state_set_json("payload", &state)
        .expect("dialog json write");

    assert_eq!(
        ctx.state_get_json::<JsonState>("plain")
            .expect("plain json read"),
        Some(state.clone())
    );
    assert_eq!(
        ctx.user_state_get_json::<JsonState>("profile")
            .expect("user json read"),
        Some(state.clone())
    );
    assert_eq!(
        ctx.global_state_get_json::<JsonState>("config")
            .expect("global json read"),
        Some(state.clone())
    );
    assert_eq!(
        ctx.dialog_state_get_json::<JsonState>("payload")
            .expect("dialog json read"),
        Some(state)
    );
}
