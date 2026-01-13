# Getting Started

This crate is a high-level Rust wrapper over the TeamTalk 5 SDK. The client is
event-driven and uses polling through `Client::poll()`.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
teamtalk = "1.1.0"
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


