# TeamTalk SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/teamtalk.svg)](https://crates.io/crates/teamtalk)
[![Documentation](https://docs.rs/teamtalk/badge.svg)](https://docs.rs/teamtalk)
[![CI](https://github.com/BlindMaster24/TeamTalkRust/actions/workflows/ci.yml/badge.svg)](https://github.com/BlindMaster24/TeamTalkRust/actions)
[![License](https://img.shields.io/crates/l/teamtalk.svg)](https://github.com/BlindMaster24/TeamTalkRust)

**TeamTalk SDK for Rust** is a high-level, safety-first wrapper for the BearWare.dk TeamTalk 5 SDK. It provides strict typing and a pure event-driven model for performance and reliability.

## Key Features

- **Pure Event-Driven Architecture:** Reactive model via `client.poll()` with no arbitrary sleeps.
- **Strict Typing:** Strong IDs such as `UserId` and `ChannelId` prevent misuse.
- **Dynamic Runtime Loading:** `loader.rs` downloads SDK binaries when needed.
- **Full API Coverage:** Events, audio, video, desktop, files, and administration.
- **Documentation:** API reference plus guides under `docs/`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
teamtalk = "1.0.1"
```

## Quick Start

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

## Advanced Usage

### Builders

```rust
use teamtalk::types::Channel;

let my_channel = Channel::builder("Music Room")
    .topic("Only Rock 'n' Roll")
    .max_users(50)
    .build();

client.make_channel(&my_channel);
```

### Zero-Allocation Strings

```rust
let mut buf = String::with_capacity(1024);
teamtalk::utils::strings::copy_to_string(&raw_tt_str, &mut buf);
```

## Project Structure

- `crates/teamtalk-sys`: Low-level bindgen bindings to the SDK.
- `crates/teamtalk`: High-level Rust wrapper.
- `docs/`: User guides.

## Philosophy

- Event-driven only.
- Strongly typed IDs for safety.
- Encapsulated FFI with explicit conversion.

## Documentation

- API reference: https://docs.rs/teamtalk
- Guides: `docs/README.md`
- Changelog: `docs/changelog.md`

## License

MIT

