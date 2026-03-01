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
- **Opt-in Structured Logging:** enable `logging` and configure `tracing_subscriber` in your app.
- **Full API Coverage:** Events, audio, video, desktop, files, and administration.
- **Documentation:** API reference plus guides under [docs](docs/README.md).

## Installation

Add this to your `Cargo.toml`:

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

## Task Runner (Optional)

This repository includes a root [`justfile`](justfile) with shortcuts for
checks, docs, dependencies, and release operations.

Install optional tooling:

```bash
cargo install just cargo-edit cargo-outdated cargo-llvm-cov
```

Common commands:

```bash
just quick
just ci
just deps-safe-cycle
just release-status
```

## Quick Start

```rust
use teamtalk::{Client, Event};
use teamtalk::types::ChannelId;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    teamtalk::init()?;
    // If you use a TeamTalk license key, set it before creating Client.
    // teamtalk::set_license("Company Name", "license-key")?;
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

If you use a TeamTalk license key, call `teamtalk::set_license(...)` before
creating `Client`.

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

### Async Event Stream

- `into_async()` starts a worker-backed async stream over events.
- Use `wait_for_event`, `wait_for_predicate`, and `wait_for_data` to avoid manual loops.
- For explicit lifecycle, call `shutdown()` and then `into_client()` when done.
- With `async-tokio`, timeout helpers are available (`wait_for_event_timeout`, `wait_for_data_timeout`).
## Project Structure

- [crates/teamtalk](crates/teamtalk/): High-level safe SDK (`client`, `events`, `types`, `utils`).
- [crates/teamtalk/tests](crates/teamtalk/tests/): Integration tests for public API behavior.
- [crates/teamtalk/examples](crates/teamtalk/examples/): Runnable usage examples.
- [crates/teamtalk-sys](crates/teamtalk-sys/): Raw FFI bindings generated from `TeamTalk.h`.
- [crates/teamtalk-macros](crates/teamtalk-macros/): Optional proc-macros used by high-level APIs.
- [docs](docs/README.md): User-facing guides and release process docs.
- `TEAMTALK_DLL/`: Downloaded TeamTalk SDK runtime and C-API documentation (local workspace asset, git-ignored in CI).
- `qtTeamTalk/`: Upstream TeamTalk Qt client source snapshot used for reference (optional local mirror).

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
