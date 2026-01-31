# Changelog

## Versioning

This project follows semantic versioning. A major release indicates stable
public API with compatibility expectations for downstream users.

## Unreleased

### Added
- `CommandId` newtype for command results.
- `TextMessage` helpers for common send targets.
- Cache lookups by username, channel name, and channel path.
- `join_channel_path` convenience helper.
- `poll_until_event` helper for event waits.
- Operator convenience helpers with and without operator password.
- Text mute helper using text subscriptions.
- `ClientManager::wait_cmd` helper and command ids in manager events.

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
