# Changelog

## Versioning

This project follows semantic versioning. A major release indicates stable
public API with compatibility expectations for downstream users.

## Unreleased

### Added
- `CommandId` newtype plus `MessageBuilder::send_cmd` and `TextMessage::{send_to_user, send_to_channel, send_broadcast, send_private}` helpers.
- Cache lookups by username, channel name, and channel path.
- Channel helpers: `join_channel_path`, `join_channel_path_unprotected`, `join_channel_unprotected`, `leave_to_root`.
- `poll_until_event` helper for event waits.
- Operator helpers: `set_user_operator`, `set_user_operator_ex`, `op_user`, `deop_user`, `op_user_ex`, `deop_user_ex`.
- Text/voice/media mute helpers: `set_user_text_mute`, `mute_user_text`, `unmute_user_text`, `mute_user_voice`, `unmute_user_voice`, `mute_user_media`, `unmute_user_media`.
- Manager helpers: `ClientManager::{wait_cmd, wait_cmd_ok, wait_cmd_any}` and command id tracking in events.
- SDK version pin/override via [SDK_VERSION.txt](../crates/teamtalk/SDK_VERSION.txt) and `TEAMTALK_SDK_VERSION`.
- Loader now copies full SDK `Documentation/` under `TEAMTALK_DLL/Documentation` and validates it against a saved manifest; missing files trigger SDK re-download.
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
- `logout` now returns `0` when no login session is active, avoiding duplicate/logout-outside-session commands.
- `leave_channel` now returns `0` when the client is not currently joining/joined in a channel, avoiding invalid leave commands.
- Auto-reconnect extra event lists now deduplicate by event kind in `enable_auto_reconnect_with_events` and `set_auto_reconnect_events`.
- Auto-join remembers channel passwords set via `join_channel`.
- `clear_last_channel` now clears both remembered channel id and channel password.
- `dispatch_reconnect` example now logs in via stored params and guards repeated joins.
- Reconnect paths now enforce a disconnect barrier before retrying `TT_Connect`, matching TeamTalk C-API reconnect requirements.

### Fixed
- Safer `TTCHAR` handling on non-Windows and expanded safety contracts on unsafe APIs.
- `Client::set_client_keep_alive()` now rejects invalid timeout relationships before calling `TT_SetClientKeepAlive`.

### Docs
- Developer notes moved to [dev.md](dev.md) with updated lefthook guidance and doc links.
- Added `dispatch_reconnect` example showing reconnect with kick handling.
- Added explicit license setup guidance (`set_license` before `connect*`/`login`).

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
