#![cfg(all(feature = "bot", feature = "mock"))]

use std::sync::Arc;
use teamtalk::client::backend::MockBackend;
use teamtalk::events::Event;
use teamtalk::mock::MockMessage;
use teamtalk::types::{ChannelId, UserId};
use teamtalk::{
    Client, DialogFlow, DialogMachine, HandlerResult, MemoryStateStore, Router, StateStore,
};
use teamtalk_sys::TextMsgType;

fn mock_client() -> Client {
    let backend = Arc::new(MockBackend::new());
    Client::with_backend(backend).expect("mock client")
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
fn router_scene_flow_advances_and_uses_dialog_scoped_state() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let flow = DialogFlow::new("wizard", "ask_name").step("ask_email");
    let flow_start = flow.clone();
    let flow_next = flow.clone();

    let mut router = Router::new()
        .on_command("start", move |ctx| {
            ctx.dialog_restart_flow(&flow_start)?;
            assert!(ctx.dialog_state_set_typed("attempts", 1_u32));
            Ok(HandlerResult::Continue)
        })
        .on_dialog_step("wizard", "ask_name", move |ctx| {
            let attempts = ctx.dialog_state_parse::<u32>("attempts")?.unwrap_or(0);
            assert_eq!(attempts, 1);
            assert!(ctx.dialog_state_set_typed("attempts", attempts + 1));
            let _ = ctx.dialog_set_metadata("name", ctx.text().unwrap_or_default());
            ctx.dialog_advance_next(&flow_next)?;
            Ok(HandlerResult::Continue)
        });

    let start = text_message("/start");
    router
        .dispatch(&client, Event::TextMessage, &start, &mut store)
        .expect("start dispatch");

    let reply = text_message("Alice");
    router
        .dispatch(&client, Event::TextMessage, &reply, &mut store)
        .expect("dialog dispatch");

    let mut machine = DialogMachine::new(&mut store);
    let state = machine.current_live(UserId(7)).expect("dialog state");
    assert_eq!(state.dialog, "wizard");
    assert_eq!(state.step, "ask_email");
    assert_eq!(state.metadata("name"), Some("Alice"));

    let session = state.session_id().expect("session");
    let attempts_key = format!("d:7:wizard:{session}:attempts");
    assert_eq!(store.get(&attempts_key), Some("2".to_owned()));
}

#[test]
fn router_scene_restart_rotates_dialog_scoped_state_namespace() {
    let client = mock_client();
    let mut store = MemoryStateStore::new();
    let flow = DialogFlow::new("wizard", "ask_name");
    let flow_restart = flow.clone();

    let mut router = Router::new().on_command("start", move |ctx| {
        let state = ctx.dialog_restart_flow(&flow_restart)?;
        assert!(ctx.dialog_state_set(
            "marker",
            format!("session:{}", state.session_id().unwrap_or("missing"))
        ));
        Ok(HandlerResult::Continue)
    });

    let start = text_message("/start");
    router
        .dispatch(&client, Event::TextMessage, &start, &mut store)
        .expect("first start");
    let first_session = DialogMachine::new(&mut store)
        .current(UserId(7))
        .and_then(|state| state.session_id().map(str::to_owned))
        .expect("first session");

    router
        .dispatch(&client, Event::TextMessage, &start, &mut store)
        .expect("second start");
    let second_session = DialogMachine::new(&mut store)
        .current(UserId(7))
        .and_then(|state| state.session_id().map(str::to_owned))
        .expect("second session");

    assert_ne!(first_session, second_session);
    assert_eq!(
        store.get(&format!("d:7:wizard:{first_session}:marker")),
        Some(format!("session:{first_session}"))
    );
    assert_eq!(
        store.get(&format!("d:7:wizard:{second_session}:marker")),
        Some(format!("session:{second_session}"))
    );
}
