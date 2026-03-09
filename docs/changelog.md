# Changelog

## [5.0.0](https://github.com/BlindMaster24/TeamTalkRust/compare/teamtalk-v4.0.0...teamtalk-v5.0.0) - 2026-03-08

### Added
- *(client)* add win32 desktop and hotkey guards
- *(client)* fill DLL event and desktop gaps
- *(bot)* add rights-based permissions layer
- *(bot)* expand guards, aliases, and macros
- *(bot)* add guards, aliases, and serde state
- *(bot)* complete scene lifecycle and guides
- *(bot)* add flow-aware scene helpers
- *(bot)* enrich dialog state machine lifecycle

### Changed
- *(client)* split users module by responsibility
- *(client)* decompose connection, audio, and users modules
- *(core)* split backend mock, dispatch, scripts tables, and loader download
- *(client)* split core recovery and debug internals
- *(teamtalk)* decompose router, core message, and synced recording
- *(types)* split entities into domain submodules
- *(teamtalk)* split hooks, loader versions, and types entities
- *(client)* split core runtime/state handling
- *(types)* extract base and preprocess modules
- *(types)* split users_channels and move id definitions
- *(loader)* move loader into module directory
- *(recording)* separate synced utilities from session flow
- *(bot)* split router helpers from dispatch pipeline
- *(client)* decompose core and hooks internals
- *(types)* split monolithic types module into submodules

### Fixed
- *(client)* recover stalled auto reconnect phases

### Docs
- *(audit)* update coverage workflow and desktop helpers

### Other
- *(bot)* cover guards and json defaults

## [0.1.2](https://github.com/BlindMaster24/TeamTalkRust/compare/teamtalk-macros-v0.1.1...teamtalk-macros-v0.1.2) - 2026-03-08

### Added
- *(bot)* expand guards, aliases, and macros

## [4.0.0](https://github.com/BlindMaster24/TeamTalkRust/compare/teamtalk-v3.1.0...teamtalk-v4.0.0) - 2026-03-01

### Breaking
- *(audio)* [**breaking**] add sample_index migration note — AudioBlockView gained public field sample_index. If you construct AudioBlockView with struct literals, set sample_index explicitly (for example sample_index: 0 when no offset is tracked).

### Added
- *(audio)* expose sample_index in audio block view
- *(examples)* add voice stream and segment recording samples
- *(bot)* add command schemas, fn middleware, and scene routing
- *(loader)* support remote latest pin with URL override

### Changed
- *(loader)* unify internal logging helper

### Fixed
- *(recording)* harden synced lifecycle and file safety
- *(recording)* harden synced audio block capture

## [0.1.1](https://github.com/BlindMaster24/TeamTalkRust/compare/teamtalk-macros-v0.1.0...teamtalk-macros-v0.1.1) - 2026-03-01

### Added
- *(bot)* add command schemas, fn middleware, and scene routing

## [3.1.0](https://github.com/BlindMaster24/TeamTalkRust/compare/teamtalk-v3.0.0...teamtalk-v3.1.0) - 2026-02-21

### Added
- *(bot)* add optional bot macros and async wait parity
- *(bot)* add dialog flow and wait helpers in context
- *(bot)* add BotApp facade for sync and async runtimes
- *(bot)* add typed args, reply helpers, and unknown command policy

### Changed
- *(client)* remove remaining lock unwrap panics
- *(client)* recover from poisoned mutexes
- *(client)* avoid lock-held callback dispatch

### Fixed
- *(deps)* switch macros dep to workspace source
- *(async)* harden shutdown wake and DLL load guard

### Dependencies
- *(deps)* refresh workspace dependency versions

### Other
- *(teamtalk)* release v3.1.0 ([#21](https://github.com/BlindMaster24/TeamTalkRust/pull/21))

## [0.1.0](https://github.com/BlindMaster24/TeamTalkRust/releases/tag/teamtalk-macros-v0.1.0) - 2026-02-21

### Added
- *(bot)* add optional bot macros and async wait parity

## [3.0.0](https://github.com/BlindMaster24/TeamTalkRust/compare/v2.0.0...v3.0.0) - 2026-02-20

### Breaking
- *(async)* [**breaking**] add migration link to AsyncClient threading docs — AsyncClient no longer implements Sync. Keep AsyncClient in one runtime/task and coordinate via wait helpers plus shutdown/into_client. Migration: https://github.com/BlindMaster24/TeamTalkRust/blob/main/docs/migrations/2-to-3.md
- *(async)* [**breaking**] document AsyncClient Sync removal in API docs — AsyncClient no longer implements Sync. Keep AsyncClient in one runtime/task and coordinate via wait helpers and shutdown/into_client.

### Added
- *(bot)* add dialog state machine helpers
- *(bot)* add command and rate-limit middleware
- *(bot)* add redis and sqlite state store adapters
- *(bot)* add command groups and unknown fallback
- *(bot)* add sync and async bot framework layer
- *(async)* move stream polling to worker queue

### Docs
- *(async)* refresh stream examples and usage guides

## [2.0.0](https://github.com/BlindMaster24/TeamTalkRust/compare/v1.3.0...v2.0.0) - 2026-02-19

### Added
- add state store and high-level wait helpers

## 1.3.0

### Added
- `CommandId` newtype plus `MessageBuilder::send_cmd` and `TextMessage::{send_to_user, send_to_channel, send_broadcast, send_private}` helpers.
- Cache lookups by username, channel name, and channel path.
- Channel helpers: `join_channel_path`, `join_channel_path_unprotected`, `join_channel_unprotected`, `leave_to_root`.
- `poll_until_event` helper for event waits.
- Operator helpers: `set_user_operator`, `set_user_operator_ex`, `op_user`, `deop_user`, `op_user_ex`, `deop_user_ex`.
- Text/voice/media mute helpers: `set_user_text_mute`, `mute_user_text`, `unmute_user_text`, `mute_user_voice`, `unmute_user_voice`, `mute_user_media`, `unmute_user_media`.
- Manager helpers: `ClientManager::{wait_cmd, wait_cmd_ok, wait_cmd_any}` and command id tracking in events.
- SDK version pin/override via [SDK_VERSION.txt](../crates/teamtalk/SDK_VERSION.txt) and `TEAMTALK_SDK_VERSION`.
- Loader now validates `TEAMTALK_DLL/Documentation/C-API` against a saved
  manifest; missing files trigger SDK re-download and restore that C-API docs
  subtree.
- `async-tokio` feature for Tokio wake integration in the async wrapper.
- `Client::split()` method returning `ClientEvents` (polling) and `ClientCommands` (execution) for concurrent usage.
- Auto-reconnect extra events via `enable_auto_reconnect_with_events` and `set_auto_reconnect_events`.
- Explicit auto-join state helper: `set_last_channel(ChannelId, Option<&str>)`.
- Auto-reconnect event list helpers: `add_auto_reconnect_event` and `remove_auto_reconnect_event`.
- `reconnect_protected_channel` example showing reconnect + login + protected auto-join flow.
- `async_tokio_event_stream` example showing async polling with Tokio wake integration.
- In-session full recovery API: `enable_full_auto_reconnect` and `ReconnectWorkflowConfig` (separate login/join retry policies).
- Auto-recovery phase events/hooks: `BeforeAutoLogin`, `AutoLoginFailed`, `BeforeAutoJoin`, `AutoJoinFailed`, `AutoRecoverCompleted`.
- `ClientFlags::CONNECTION` helper bit (`CONNECTING | CONNECTED`) for connection-state checks.
- Keep-alive helpers: `Client::ping` and `Client::set_client_keep_alive_and_ping`.
- Manual reconnect helpers: `reconnect`, `reconnect_with_params`, `reconnect_ex`, `reconnect_sys_id`.
- `ClientStatistics` now includes TCP/UDP server silence seconds.
- `Message::error_message()` accessor for `ConnectCryptError`/`CmdError`/`InternalError`.
- `Client::query_server_max_payload()` helper for SDK-supported payload query mode.
- `teamtalk::set_license(name, key)` helper to set license information before `Client::new()`.

### Changed
- `Client` is now thread-safe (`Send` + `Sync`) and uses internal locking.
- `Client::send_text` and `send_to_*` now send long text as multipart messages using `TextMessage.bMore` instead of truncating to a single packet.
- Message payload accessors now verify `TTMessage.ttType` before decoding unions.
- `Message::user()` now decodes `Event::MySelfKicked` when kicker payload is present.
- `ClientEvent` now carries `command_id` for multi-client command tracking.
- `connect`, `connect_ex`, and `connect_sys_id` now fail fast with `CommandFailed` when the client is already connecting or connected.
- `login` now fails fast (returns `0`) when called while a login/join flow is already in progress.
- `Event::MySelfKicked` now updates state using `TTMessage.nSource` semantics:
  channel kick (`nSource > 0`) -> `LoggedIn`, server kick (`nSource <= 0`) -> `Connected`.
- `join_channel` now returns `0` when a previous join is still in progress (`ConnectionState::Joining(_)`), avoiding duplicate join commands.
- `join_channel` now starts only from `LoggedIn` state, avoiding `CMDERR_ALREADY_IN_CHANNEL` retries when already joined.
- `logout` now returns `0` when no login session is active, avoiding duplicate/logout-outside-session commands.
- `leave_channel` now returns `0` when the client is not currently joining/joined in a channel, avoiding invalid leave commands.
- `CmdError` now resets login/join recovery states even when auto-reconnect is disabled, preventing stale `LoggingIn`/`Joining` states after failed commands.
- Auto-reconnect extra event lists now deduplicate by event kind in `enable_auto_reconnect_with_events` and `set_auto_reconnect_events`.
- `set_encryption_context` now returns `false` unless called before connect/login state (`Idle` or `Disconnected`), matching TeamTalk C-API sequencing.
- Auto-join remembers channel passwords set via `join_channel`.
- `clear_last_channel` now clears both remembered channel id and channel password.
- `dispatch_reconnect` example now logs in via stored params and guards repeated joins.
- Manual and auto reconnect paths now enforce a disconnect barrier before retrying reconnect operations, matching the TeamTalk C-API reconnect requirements around `TT_Connect`.
- Manual `connect`, `connect_ex`, and `connect_sys_id` now apply a disconnect barrier when called from `Disconnected` state, matching TeamTalk C-API reconnect guidance.
- Logged-in command wrappers now fail fast with `0` when the client is not authorized yet (`Connected`/`Idle`), instead of forwarding and waiting for `CMDERR_NOT_LOGGEDIN` (`change_nickname`, `set_status*`, text, file, subscribe, channel-admin/user-admin commands).
- Server admin command wrappers now also fail fast with `0` before login (`ban_ip`, `list_bans`, `update_server`, `save_server_config`, `query_server_stats`), matching C-API `CMDERR_NOT_LOGGEDIN` preconditions.

### Fixed
- Safer `TTCHAR` handling on non-Windows and expanded safety contracts on unsafe APIs.
- `Client::set_client_keep_alive()` now rejects invalid timeout relationships before calling `TT_SetClientKeepAlive`.

### Docs
- Developer notes moved to [dev.md](dev.md) with updated lefthook guidance and doc links.
- Added `dispatch_reconnect` example showing reconnect with kick handling.
- Added explicit license setup guidance (`teamtalk::set_license` before `Client::new()`).
- README quick start now documents license call ordering, and `connect_login` supports `TT_LICENSE_NAME`/`TT_LICENSE_KEY`.

## 1.2.0

### Added
- Reconnect hooks: `BeforeReconnect`, `AfterReconnect`, `ReconnectFailed`.
- User/channel caches with auto-sync and refresh helpers.
- `MessageBuilder` for outgoing text messages.
- `AudioDeviceProfile` with `apply_audio_profile`.
- `ServerInfo` cache snapshot for properties and statistics.
- Event bus subscriptions and filters.
- Recording session management and rotation policies.
- Synced user recording and audio streaming helpers.
- Lua scripting event handlers and host function registration.

### Changed
- `Message` payload accessors now return `None` when the event does not match.
- `Client` is single-threaded (`!Send`); keep it on one thread or wrap it yourself.

### Fixed
- Avoid panics on non-UTF8 DLL paths during SDK init.
- Synced user recording returns IO errors instead of panicking on file creation.
- Registry operations no longer panic on poisoned mutexes.
- Windows: gate HWND backend APIs to avoid cross-platform build failures.

## 1.1.0

### Added
- Subscription presets: `Subscriptions::all_audio`, `all_text`, `all_control`.
- Client helpers: `poll_until`, `wait_for`, and `join_root`.
- Recording guard: `RecordSession` for safe start/stop of channel recording.
- Connection state tracking via `ConnectionState` and `Client::connection_state`.
- Hooks API: `ClientHooks` with per-event handlers.
- Auto-reconnect configuration and remembered connection parameters.
- Auto-login and rejoin using stored login data.
- Typed errors with SDK code and message.
- Env-based helpers for connection and login parameters.
- Multi-client tracking with `ClientId`, labels, and `ClientRegistry`.
- Multi-client scheduling via `ClientManager` with health snapshots.
- Hybrid extensions via Lua scripts and native plugins.
- TLS selection via `tls-native` (default) and `tls-rustls`.
- Tests for subscription presets.

### Changed
- Error variants now carry code + message payloads.

### Breaking
- `Error::CommandFailed` and `Error::ClientError` now include `code` and `message` fields.

## 1.0.0

### Added
- Full rustdoc coverage for the public API.
- User documentation set in [docs/](README.md) with onboarding and configuration guides.
- Feature-gated modules: `dispatch`, `async`, `logging`, `mock`, `offline`.
- Comprehensive test suite with feature coverage.
- Documentation build automation and scripts.

### Changed
- Loader supports explicit offline mode with `offline` feature flag.
- README updated with professional structure and links to guides.
