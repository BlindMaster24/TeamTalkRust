# Changelog

## Versioning

This project follows semantic versioning. A major release indicates stable
public API with compatibility expectations for downstream users.

## Unreleased
### Added
- Subscription presets: `Subscriptions::all_audio`, `all_text`, `all_control`.
- Client helpers: `poll_until`, `wait_for`, and `join_root`.
- Recording guard: `RecordSession` for safe start/stop of channel recording.
- Connection state tracking via `ConnectionState` and `Client::connection_state`.
- Hooks API: `ClientHooks` with per-event handlers.
- Auto-reconnect configuration and remembered connection parameters.
- Tests for subscription presets.

## 1.0.0

### Added
- Full rustdoc coverage for the public API.
- User documentation set in `docs/` with onboarding and configuration guides.
- Feature-gated modules: `dispatch`, `async`, `logging`, `mock`, `offline`.
- Comprehensive test suite with feature coverage.
- Documentation build automation and scripts.

### Changed
- Loader supports explicit offline mode with `offline` feature flag.
- README updated with professional structure and links to guides.
