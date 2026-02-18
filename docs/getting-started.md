# Getting Started

This crate is a high-level Rust wrapper over the TeamTalk 5 SDK. The client is
event-driven and uses polling through `Client::poll()`.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
teamtalk = "1.3.0"
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
2. Connect and login using organized namespaces.
3. Poll events and react.

Example (Synchronous):

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;

fn main() -> teamtalk::Result<()> {
    teamtalk::init()?;
    let client = Client::new()?;
    
    // Commands are organized into logical namespaces
    client.connect("127.0.0.1", 10333, 10333, false)?;
    
    loop {
        if let Some((event, msg)) = client.poll(100) {
            match event {
                Event::ConnectSuccess => {
                    client.users().login("RustBot", "guest", "guest", "TeamTalkRust");
                }
                Event::MySelfLoggedIn => {
                    client.channels().join(ChannelId(1), "");
                }
                Event::TextMessage => {
                    if let Some(text) = msg.extract::<teamtalk::types::TextMessage>() {
                        println!("{}: {}", text.from_username, text.text);
                    }
                }
                Event::ConnectionLost | Event::ConnectFailed => break,
                _ => {}
            }
        }
    }
    Ok(())
}
```

### Async Flow (Professional)

For asynchronous projects, `AsyncClient` provides methods that wait for server confirmation:

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;

#[tokio::main]
async fn main() -> teamtalk::Result<()> {
    teamtalk::init()?;
    let client = Client::new()?;
    let async_client = client.clone().into_async();

    async_client.connect("127.0.0.1", 10333, 10333, false).await?;
    
    // Robust login: waits for MySelfLoggedIn OR CmdError
    let me = async_client.users().login("RustBot", "guest", "guest", "TeamTalkRust").await?;
    println!("Logged in as UserId({})", me.id.0);

    async_client.channels().join(ChannelId(1), "").await?;
    println!("Joined channel!");

    Ok(())
}
```

## Working with Payloads

Namespaces return results, but when polling events manually, you can use the ergonomic `extract` method to get typed data:

```rust
// Use the universal extractor
if let Some(user) = msg.extract::<User>() { 
    println!("User {} is here", user.nickname);
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

For manual reconnect flows, use `reconnect`, `reconnect_with_params`, or
`reconnect_ex`. These helpers apply a disconnect barrier first. Direct
`connect*` calls now return `CommandFailed` if the client is already connecting
or connected.

Login commands now have duplicate-call guards: if login/join state is already in
progress, it skip issuing a duplicate SDK login command.

For outgoing text, use the `users()` namespace helpers:

```rust
client.users().send_to_user(UserId(42), "Hello!");
```

The client handles multipart chunking (`TextMessage.bMore`) for long messages. 

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

