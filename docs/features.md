# Feature Flags

Feature flags enable optional capabilities. Default features include `tls-native`.

Enable features in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "1.1.0", features = ["dispatch", "async"] }
```

## Available Features

- `dispatch`: event dispatcher with handler routing and reconnect support.
- `async`: async wrapper without a worker thread (single-threaded polling).
- `logging`: event logging integration.
- `mock`: in-memory event source for tests.
- `offline`: disables SDK downloads; requires `TEAMTALK_DLL/` to be present.
- `scripts`: Lua scripting support for extensions.
- `plugins`: native plugin loading for extensions.
- `tls-native`: system TLS via the native OS backend (default).
- `tls-rustls`: pure Rust TLS for builds without OpenSSL.

To use Rustls only:

```bash
cargo build --no-default-features --features tls-rustls
```

## Built-In Helpers (No Feature Flags)

- Connection state tracking via `ConnectionState` and `Client::connection_state`.
- Hooks API via `ClientHooks` for event callbacks.
- Poll helpers: `Client::poll_until` and `Client::wait_for`.
- Convenience APIs: `Client::join_root`, `Subscriptions::all_audio`, `all_text`, `all_control`.
- Recording guard: `RecordSession` for safe start/stop of channel recording.    
- Managed recording: `RecordingSession`, `RecordingOptions`, `RecordingTarget`.  
- Auto-rotation: `RecordingSession::rotate_if_needed` with size/time limits.    
- Per-user recording: `UserRecordingSession`, `UserRecordingOptions`.           
- Audio block streaming: `stream_audio_blocks`, sinks (`CallbackSink`, `WriterSink`, `UdpSink`).
- Synced per-user recording: `SyncedUserRecordingSession` with `PcmS16Le`/`WavS16Le` output and silence policies.
- Auto-reconnect support via `enable_auto_reconnect` and `connect_remember`.
- Auto-login and rejoin using stored `LoginParams` and remembered channels.
- Typed errors with SDK code + message.
- Env helpers: `ConnectParamsOwned::from_env` and `LoginParams::from_env`.
- Multi-client tracking: `ClientId`, labels, and `ClientRegistry`.
- Multi-client manager: `ClientManager` with event channel and health snapshots.
- Cache helpers: user/channel caches with `refresh_*` and `cached_*` accessors.
- Server info snapshot: `Client::server_info` from cached properties and stats.
- Message builder: `MessageBuilder` for outgoing text messages.
- Event subscriptions: `Client::on_event`, `Client::on_any`, filters by user/channel/nickname/username/text type, and grouped removal via `unsubscribe_event_group`.
- Audio profiles: `AudioDeviceProfile` with `apply_audio_profile`.
- Reconnect hooks: `BeforeReconnect`, `AfterReconnect`, `ReconnectFailed`.
- Hybrid extensions: Lua scripts (`scripts`) and native plugins (`plugins`). See [docs/extensions.md](extensions.md).




