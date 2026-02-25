# Feature Flags

Feature flags enable optional capabilities. Default features include `tls-native`.

Enable features in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "4.0.0", features = ["dispatch", "async"] }
```

## Available Features

- `dispatch`: event dispatcher with handler routing and reconnect support.
- `async`: async wrapper with stream helpers (`next_event`, `wait_for_event`,
  `wait_for_predicate`, `wait_for_data`) and explicit shutdown (`shutdown`,
  `into_client`).
- `async-tokio`: Tokio wake integration for the async wrapper (requires a Tokio runtime).
- `logging`: enables `tracing` integration for SDK logs (including loader logs).
- `mock`: in-memory event source for tests.
- `offline`: disables SDK downloads; requires `TEAMTALK_DLL/` to be present.
- `scripts`: Lua scripting support for extensions.
- `plugins`: native plugin loading for extensions.
- `state`: in-memory state store helpers (`store_snapshot`, `store_user`, `store_channel`).
- `bot`: high-level bot framework (router, commands, middleware, scheduler, state store).
- `bot-macros`: attribute macros for bot handlers
  (`#[teamtalk_command]`, `#[teamtalk_event]`, `#[teamtalk_middleware]`).
- `bot-redis`: Redis-backed bot state store adapter (`RedisStateStore`).
- `bot-sqlite`: SQLite-backed bot state store adapter (`SqliteStateStore`).
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
- High-level wait helpers: `Client::login_and_wait` and `Client::join_channel_and_wait`.
- Convenience APIs: `Client::join_root`, `Subscriptions::all_audio`, `all_text`, `all_control`.
- Recording guard: `RecordSession` for safe start/stop of channel recording.    
- Managed recording: `RecordingSession`, `RecordingOptions`, `RecordingTarget`.  
- Auto-rotation: `RecordingSession::rotate_if_needed` with size/time limits.    
- Per-user recording: `UserRecordingSession`, `UserRecordingOptions`.           
- Audio block streaming: `stream_audio_blocks`, sinks (`CallbackSink`, `WriterSink`, `UdpSink`).
- Synced per-user recording: `SyncedUserRecordingSession` with `PcmS16Le`/`WavS16Le` output and silence policies.
- Auto-reconnect support via `enable_auto_reconnect`, `enable_auto_reconnect_with_events`, and `connect_remember`.
- Auto-reconnect trigger helpers: `set_auto_reconnect_events`, `add_auto_reconnect_event`, `remove_auto_reconnect_event`.
- Manual reconnect helpers with disconnect barrier: `reconnect`, `reconnect_with_params`, `reconnect_ex`, `reconnect_sys_id`.
- Auto-login and rejoin using stored `LoginParams` and remembered channels (including passwords set via `join_channel`).
- Full in-session recovery via `enable_full_auto_reconnect` (connect + login + join workflow).
- Per-phase retry policy via `ReconnectWorkflowConfig` (`login` and `join` configs).
- Explicit auto-join state helpers: `set_last_channel` and `clear_last_channel`.
- Keep-alive helpers: `ping` and `set_client_keep_alive_and_ping`.
- Typed errors with SDK code + message.
- Env helpers: `ConnectParamsOwned::from_env` and `LoginParams::from_env`.
- Multi-client tracking: `ClientId`, labels, and `ClientRegistry`.
- Multi-client manager: `ClientManager` with event channel and health snapshots.
- Cache helpers: user/channel caches with `refresh_*` and `cached_*` accessors.
- Server info snapshot: `Client::server_info` from cached properties and stats.
- State store facade: `enable_state_store`, `store_snapshot`, `store_user`, `store_channel`.
- Message builder: `MessageBuilder` for outgoing text messages.
- Typed event payload facade: `Message::data` and `try_as_*` helpers.
- Async typed waits: `AsyncClient::wait_for_data` and
  `AsyncClient::wait_for_data_timeout` (`async-tokio`).
- Bot primitives: `Bot`, `BotBuilder`, `Router`, `Context`, `Middleware`, `Scheduler`,
  command parsing (`parse_command`), and in-memory state (`MemoryStateStore`).
- Bot app facade: `BotApp` for single-point runtime wiring and sync/async entry methods.
- Command args helpers: `Args::get`, `Args::require`, and `Args::rest`.
- Declarative command schemas: `CommandPattern` with `Router::on_command_pattern*`
  and `Router::try_on_command_pattern*` for argument-count validation.
- Auto help responses: `Router::with_auto_help`, `with_help_command`,
  `with_auto_help_command`, `with_help_header`, `with_help_footer`, and
  `without_auto_help`.
- Built-in bot middleware: `CommandOnly` and `RateLimitBySource`.
- Function middleware adapters: `FnMiddleware`, `Router::use_middleware_fn`, and
  `Router::use_middleware_hooks`.
- Unknown command strategy: `UnknownCommandPolicy` (`Ignore` or automatic reply text).
- Dialog/FSM helpers: `DialogMachine`, `DialogState`, and `Context::dialog_*` helpers.
- Dialog flow helper: `DialogFlow` for reusable dialog definitions.
- Scene-style dialog routing: `Router::on_dialog_step`, `Router::on_dialog`,
  and checked transitions via `Context::dialog_start_checked` and
  `Context::dialog_advance_checked`.
- Bot context reply helpers: `reply`, `reply_private`, and `reply_channel`.
- Bot wait helpers: `wait_for_event`, `wait_text_from`, and `wait_command_from_sender`.
- Scoped state helpers: `user_state_*`, `channel_state_*`, and `global_state_*`.
- Async bot runtime (requires `bot` + `async`): `AsyncBot` and `AsyncBotBuilder`.
- Async bot wait helpers: `AsyncBot::wait_for_event`, `wait_for_predicate`, and `wait_for_data`
  (plus timeout variants with `async-tokio`).
- Outgoing long text is chunked automatically and sent as multipart
  `TextMessage.bMore`; avoid manual splitting unless you need custom behavior.
- Event subscriptions: `Client::on_event`, `Client::on_any`, filters by user/channel/nickname/username/text type, and grouped removal via `unsubscribe_event_group`.
- Audio profiles: `AudioDeviceProfile` with `apply_audio_profile`.
- Reconnect hooks: `BeforeReconnect`, `AfterReconnect`, `ReconnectFailed`.
- Auto phase hooks/events: `BeforeAutoLogin`, `AutoLoginFailed`, `BeforeAutoJoin`, `AutoJoinFailed`, `AutoRecoverCompleted`.
- Hybrid extensions: Lua scripts (`scripts`) and native plugins (`plugins`). See [docs/extensions.md](extensions.md).
