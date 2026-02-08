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
- **Documentation:** API reference plus guides under [docs](docs/README.md).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
teamtalk = "1.2.0"
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

### Text Message Sending

- Use one high-level call (`send_to_user` / `send_to_channel` / `send_to_all`) for a logical message.
- Do not manually split long text unless you explicitly need custom behavior; repeated manual sends can look like spam and may trigger TeamTalk server flood protection.
- Current `teamtalk` from git `main` chunks long text automatically and sends multipart messages via `TextMessage.bMore`.

### In-Session Auto Recovery

- `enable_full_auto_reconnect(...)` enables reconnect + auto-login + auto-join as a single in-session workflow.
- Recovery state is memory-only: connect/login/channel credentials are reused while the process runs, but not persisted across restarts.
- Per-phase retry tuning is available via `ReconnectWorkflowConfig` (`login` and `join` policies).
- New hooks/events expose phase progress and failures: `BeforeAutoLogin`, `AutoLoginFailed`, `BeforeAutoJoin`, `AutoJoinFailed`, `AutoRecoverCompleted`.
## Project Structure

- [crates/teamtalk-sys](crates/teamtalk-sys/): Low-level bindgen bindings to the SDK.
- [crates/teamtalk](crates/teamtalk/): High-level Rust wrapper.
- [crates/teamtalk/examples](crates/teamtalk/examples/): Runnable examples.
- [docs](docs/README.md): User guides.

## Philosophy

- Event-driven only.
- Strongly typed IDs for safety.
- Encapsulated FFI with explicit conversion.

## Documentation

- API reference: https://docs.rs/teamtalk
- Guides: [docs](docs/README.md)
- Changelog: [changelog](docs/changelog.md)

## License

MIT

