# Getting Started

This crate is a high-level Rust wrapper over the TeamTalk 5 SDK. The client is
event-driven and uses polling through `Client::poll()`.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
teamtalk = "1.0.1"
```

For the latest development version from `main`:

```toml
[dependencies]
teamtalk = { git = "https://github.com/BlindMaster24/TeamTalkRust.git", branch = "main" }
```

## Basic Flow

1. Initialize the SDK.
2. Connect and login.
3. Poll events and react.

Example:

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;

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

