# Feature Flags

Feature flags enable optional capabilities. Defaults are empty.

Enable features in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "1.0.1", features = ["dispatch", "async"] }
```

## Available Features

- `dispatch`: event dispatcher with handler routing and reconnect support.
- `async`: async wrapper with a worker thread.
- `logging`: event logging integration.
- `mock`: in-memory event source for tests.
- `offline`: disables SDK downloads; requires `TEAMTALK_DLL/` to be present.

## Built-In Helpers (No Feature Flags)

- Connection state tracking via `ConnectionState` and `Client::connection_state`.
- Hooks API via `ClientHooks` for event callbacks.
- Poll helpers: `Client::poll_until` and `Client::wait_for`.
- Convenience APIs: `Client::join_root`, `Subscriptions::all_audio`, `all_text`, `all_control`.
- Recording guard: `RecordSession` for safe start/stop of channel recording.
- Auto-reconnect support via `enable_auto_reconnect` and `connect_remember`.
- Auto-login and rejoin using stored `LoginParams` and remembered channels.
- Typed errors with SDK code + message.
- Env helpers: `ConnectParamsOwned::from_env` and `LoginParams::from_env`.
- Multi-client tracking: `ClientId`, labels, and `ClientRegistry`.

