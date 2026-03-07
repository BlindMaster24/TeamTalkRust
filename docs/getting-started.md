# Getting Started

This crate is a high-level Rust wrapper over the TeamTalk 5 SDK. The client is
event-driven and uses polling through `Client::poll()`.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
teamtalk = "4.0.0"
```

For the latest development version from `main`:

```toml
[dependencies]
teamtalk = { git = "https://github.com/BlindMaster24/TeamTalkRust.git", branch = "main" }
```

Quick add:

```bash
cargo add teamtalk
```

Dev install via cargo:

```bash
cargo add teamtalk --git https://github.com/BlindMaster24/TeamTalkRust.git --branch main
```

## Basic Flow

1. Initialize the SDK.
2. Connect and login.
3. Poll events and react.

Example:

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;
use teamtalk::types::UserId;
use teamtalk_sys as ffi;

fn main() -> teamtalk::Result<()> {
    teamtalk::init()?;
    let client = Client::new()?;
    client.connect("127.0.0.1", 10333, 10333, false)?;
    loop {
        if let Some((event, _msg)) = client.poll(100) {
            match event {
                Event::ConnectSuccess => {
                    client.login("RustBot", "guest", "guest", "TeamTalkRust");
                }
                Event::MySelfLoggedIn => {
                    client.join_channel(ChannelId(1), "");
                }
                Event::ConnectionLost | Event::ConnectFailed => break,
                _ => {}
            }
        }
    }
    Ok(())
}
```

Note: message payloads are event-specific. Only call the accessor that matches
the event you received (for example, `msg.text()` for `Event::TextMessage`).
Other accessors return `None`.

For SDK and command errors (`Event::ConnectCryptError`, `Event::CmdError`,
`Event::InternalError`), use `msg.error_message()` to read error code/text from
the `TTMessage` payload.

For `Event::MySelfKicked`, `msg.user()` now returns the kicker user when the
SDK includes it in the event payload.

If you use a TeamTalk license key, call `teamtalk::set_license(...)` before
creating `Client`.

If you plan to use auto-reconnect, prefer `connect_remember` and
`login_remember` so the client can restore state after reconnect. For protected
channels, call `join_channel` once with the password or use `set_last_channel`
to store the channel and password explicitly.

For manual reconnect flows, use `reconnect`, `reconnect_ex`, or
`reconnect_sys_id`. These helpers apply a disconnect barrier first. Direct
`connect*` calls now return `CommandFailed` if the client is already connecting
or connected.

`login` now has a duplicate-call guard: if login/join state is already in
progress, it returns `0` and skips issuing a duplicate SDK login command.

For outgoing text, prefer a single high-level call (`send_to_user`,
`send_to_channel`, `send_to_all`) for one logical message. The client handles
multipart chunking (`TextMessage.bMore`) for long messages. Avoid manual
splitting unless you need custom behavior, since repeated sends may trigger
server flood protection.

When querying UDP payload limits, prefer `query_server_max_payload()`. The
current TeamTalk SDK only supports server query mode (`user_id = 0`).

## Event Bus Helpers

Manual `match` is still available, but the built-in subscription API can route events for you:

```rust
let _subscription = client
    .on_event(Event::TextMessage)
    .filter_user(UserId(42))
    .filter_channel(ChannelId(1))
    .filter_text_type(ffi::TextMsgType::MSGTYPE_USER)
    .group("cli-watchers")
    .subscribe(|ctx| {
        if let Some(text) = ctx.text() {
            println!("{} says: {}", text.from_username, text.text);
        }
    });

loop {
    let _ = client.poll(100); // dispatches the handler above
}
```

Store the returned `EventSubscriptionId` to drop the handler later, or use
`client.unsubscribe_event_group("cli-watchers")` to remove a whole group in one call.

## Async Flow

Enable async support in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "4.0.0", features = ["async"] }
```

Use the async wrapper when you want stream-style event handling:

```rust
use teamtalk::{Client, Event};

fn main() -> teamtalk::Result<()> {
    let client = Client::new()?;
    let mut stream = client.into_async();

    futures::executor::block_on(async {
        let _ = stream
            .wait_for_predicate(|event, _| {
                matches!(event, Event::ConnectionLost | Event::ConnectFailed)
            })
            .await;
    });

    stream.shutdown();
    let _client = stream.into_client();
    Ok(())
}
```

For Tokio timeout helpers (`wait_for_event_timeout`, `wait_for_data_timeout`),
enable `async-tokio` in addition to `async`.

## Bot Context Helpers

With the `bot` feature, handlers can use context helpers instead of manual
message plumbing:

```rust
use std::time::Duration;
use teamtalk::{
    CommandPattern, DialogFlow, DialogState, DialogTimeoutPolicy, HandlerResult,
    RequirePrivateMessage, Router,
};

let onboarding = DialogFlow::new("onboarding", "ask_name").step("ask_email");
let ban_pattern = CommandPattern::parse("ban <user_id> [reason...]")?;
let onboarding_start = onboarding.clone();
let onboarding_name = onboarding.clone();
let onboarding_email = onboarding.clone();

let router = Router::new()
    .with_auto_help_command("commands")
    .alias_command("begin", "start")
    .with_unknown_command_suggestions(3)
    .use_middleware(RequirePrivateMessage)
    .use_middleware_fn(|ctx| {
        Ok(if ctx.command.is_some() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        })
    })
    .on_command_pattern_with_help(
        ban_pattern,
        "Ban user by id with optional reason",
        |ctx| {
            let args = ctx.args().expect("command args");
            let user_id: i32 = args.require(0, "/ban <user_id> [reason...]")?;
            let _reason = args.rest(1).unwrap_or_else(|| "no reason".to_owned());

            ctx.user_state_set("last_ban_target", user_id.to_string());
            let _ = ctx.reply_private("Command accepted");
            Ok(HandlerResult::Continue)
        },
    )
    .on_command("start", move |ctx| {
        let state = DialogState::new(onboarding_start.name(), onboarding_start.start_step())
            .with_timeout(Duration::from_secs(300))
            .with_timeout_policy(DialogTimeoutPolicy::Pause)
            .with_metadata([("locale", "en"), ("mode", "guided")]);
        ctx.dialog_start_state(state);
        let _ = ctx.reply_private("What is your name?");
        Ok(HandlerResult::Continue)
    })
    .on_dialog_step("onboarding", "ask_name", move |ctx| {
        let Some(name) = ctx.text() else {
            return Ok(HandlerResult::Continue);
        };
        ctx.user_state_set("name", name);
        let _ = ctx.dialog_set_metadata("name", name);
        let _ = ctx.dialog_state_set_typed("started", true);
        ctx.dialog_advance_next(&onboarding_name)?;
        let _ = ctx.reply_private("Now send your email:");
        Ok(HandlerResult::Continue)
    })
    .on_dialog_step("onboarding", "ask_email", move |ctx| {
        let Some(email) = ctx.text() else {
            return Ok(HandlerResult::Continue);
        };
        ctx.user_state_set("email", &email);
        ctx.user_state_set_typed("completed_steps", 2_u32);
        let locale = ctx
            .dialog_metadata("locale")
            .unwrap_or_else(|| "en".to_owned());
        let _ = ctx.dialog_advance_next(&onboarding_email)?;
        let started = ctx
            .dialog_state_parse::<bool>("started")?
            .unwrap_or(false);
        let current_step = ctx.dialog_step().unwrap_or_else(|| "unknown".to_owned());
        let completed_steps = ctx
            .user_state_parse::<u32>("completed_steps")?
            .unwrap_or(0);
        let _ = ctx.reply_private("Onboarding complete.");
        let _ = ctx.reply_private(&format!(
            "Stored locale: {locale}; started: {started}; step: {current_step}; completed: {completed_steps}"
        ));
        let _ = ctx.dialog_finish();
        Ok(HandlerResult::Continue)
    });
```

`dialog_current()` returns only active, non-expired dialog state. Use
`dialog_current_live()` when you also need paused dialogs, and use
`dialog_pause`, `dialog_resume`, `dialog_set_timeout`, and
`dialog_set_timeout_policy` for explicit lifecycle control. Use
`dialog_advance_next(&flow)` when the next step should follow the declared
`DialogFlow` order, and `dialog_state_*` / `*_parse` helpers when you want
typed scene-local scratch data without manually building state keys.

For the full bot framework surface, see [bot.md](bot.md). For scene lifecycle,
timeouts, metadata, and session-scoped state, see [scenes.md](scenes.md).

You can wire bot runtime components through `BotApp`:

```rust
use teamtalk::{BotApp, BotConfig, HandlerResult, Router};

let router = Router::new().on_command("ping", |ctx| {
    let _ = ctx.reply_private("pong");
    Ok(HandlerResult::Continue)
});

let client = teamtalk::Client::new()?;
BotApp::new()
    .with_router(router)
    .with_config(BotConfig::new().poll_timeout_ms(100))
    .run_sync(client)?;
```

## Bot Macros (Optional)

Enable `bot-macros` when you want attribute-based handler registration:

```toml
[dependencies]
teamtalk = { version = "4.0.0", features = ["bot", "bot-macros"] }
```

```rust
use teamtalk::{teamtalk_command, teamtalk_middleware, HandlerResult, Router};

#[teamtalk_command("ping")]
fn ping(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    let _ = ctx.reply_private("pong");
    Ok(HandlerResult::Continue)
}

#[teamtalk_middleware]
fn command_only(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    Ok(if ctx.command.is_some() {
        HandlerResult::Continue
    } else {
        HandlerResult::Stop
    })
}

let router = register_command_only(register_ping(Router::new()));
```
