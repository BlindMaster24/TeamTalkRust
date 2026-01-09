# Developer Notes

This section is for contributors and maintainers.

## Where to Update Documentation

- `README.md`: short overview, install, quick start, and links to guides.
- `docs/README.md`: documentation index and entry points.
- `docs/features.md`: feature flags and how to enable them.
- `docs/getting-started.md`: onboarding example and minimal workflow.
- `docs/configuration.md`: SDK binaries, networking, and TLS setup.
- `docs/developer.md`: contributor-only guidance.

## When Adding a Feature

1. Add or update the feature flag in `crates/teamtalk/Cargo.toml`.
2. Update `docs/features.md` with the new flag and a one-line description.
3. Update `README.md` only if the feature affects onboarding or default usage.
4. Update the Architecture Overview below if it changes core behavior or modules.
5. Add or update examples under `crates/teamtalk/examples/` or `crates/teamtalk/my/`.
6. Update `docs/README.md` if you add or remove guides.

## Documentation Build

- API reference: `cargo doc --no-deps --all-features --open`
- Scripts: `scripts/build-docs.ps1`, `scripts/build-docs.sh`

## Publishing

- Update the version in `crates/teamtalk/Cargo.toml`.
- Run the full Definition of Done checks from `AGENTS.md`.
- Publish with `cargo publish` inside `crates/teamtalk`.
- docs.rs builds automatically after publish and uses `all-features = true`.

## CI/CD

- CI runs formatting, linting, checks, tests, and docs build.
- The publish job runs on version tags.

## Testing

Run the full test suite with all features enabled:

```bash
cargo test --workspace --all-targets --all-features
```

Quick checks used in CI:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
```

Use the `mock` feature for deterministic event-driven tests without a running
TeamTalk server.

## Architecture Overview

The SDK wraps the TeamTalk C API with a polling client and typed structures.
The design emphasizes:

- Event-driven flow via `Client::poll()`.
- Strongly typed IDs (`UserId`, `ChannelId`) to avoid mixing values.
- Explicit conversion between Rust structs and TeamTalk FFI types.

### Modules

- `client`: main client and per-domain operations (users, channels, audio).
- `types`: strongly typed data structures and conversion helpers.
- `events`: event and error types emitted by polling.
- `utils`: string and math helpers for working with TeamTalk types.

## Documentation Style

- Keep user-facing docs focused on usage and configuration.
- Keep developer details in this file.
- Avoid internal implementation details in user guides.
