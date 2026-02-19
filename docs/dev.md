# Developer Notes

This section is for contributors and maintainers.

## Where to Update Documentation

- `README.md`: short overview, install, quick start, and links to guides.
- [docs/README.md](README.md): documentation index and entry points.
- [docs/features.md](features.md): feature flags and how to enable them.
- [docs/getting-started.md](getting-started.md): onboarding example and minimal workflow.
- [docs/configuration.md](configuration.md): SDK binaries, networking, and TLS setup.
- [docs/tls.md](tls.md): TLS backend selection and troubleshooting.
- [docs/release.md](release.md): release automation and publishing flow.
- [docs/dev.md](dev.md): contributor-only guidance.

## When Adding a Feature

1. Add or update the feature flag in [crates/teamtalk/Cargo.toml](../crates/teamtalk/Cargo.toml).
2. Update [docs/features.md](features.md) with the new flag and a one-line description.
3. Update `README.md` only if the feature affects onboarding or default usage.
4. Update the Architecture Overview below if it changes core behavior or modules.
5. Add or update examples under [crates/teamtalk/examples](../crates/teamtalk/examples/).
6. Update [docs/README.md](README.md) if you add or remove guides.

## Documentation Build

- API reference: `cargo doc --no-deps --all-features --open`
- Scripts: `scripts/build-docs.ps1`, `scripts/build-docs.sh`

## Local Pre-Commit Checks

- Install lefthook using the official guide for your OS (includes manual installs):
  [https://lefthook.dev/installation/](https://lefthook.dev/installation/)
- Enable hooks: `lefthook install`
- Default hooks run formatting, linting, type checks, and doc-link checks.
- On Windows, if you don't have a bash-compatible shell for `scripts/check-doc-links.sh`, copy `lefthook-local.example.yml` to `lefthook-local.yml` and override the doc-link command:
  - Example file: [lefthook-local.example.yml](../lefthook-local.example.yml)

```yml
pre-commit:
  commands:
    doc-links:
      run: pwsh -File scripts/check-doc-links.ps1
```

## Publishing

- Release flow is automated by [`.github/workflows/release-plz.yml`](../.github/workflows/release-plz.yml).
- On push to `main`, release-plz creates or updates a release PR with version bump and changelog updates.
- The release PR also syncs [README](../README.md),
  [getting-started.md](getting-started.md), and [features.md](features.md) to
  the new crate version.
- After the release PR is merged, release-plz creates the tag/release and publishes to crates.io.
- Keep commit messages in Conventional Commits format so release-plz can infer
  semver bumps and changelog sections for the release PR.
- docs.rs builds automatically after publish and uses `all-features = true`.

## CI/CD

- CI runs formatting, linting, checks, tests, docs build, and link validation.
- CI also verifies docs version references are in sync with
  [crates/teamtalk/Cargo.toml](../crates/teamtalk/Cargo.toml).
- Release and publish are handled by the release-plz workflow.

## Testing

Run the full test suite with all features enabled:

```bash
cargo test --workspace --all-targets --all-features
```

Quick checks used in CI:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-targets
```

Doc link checks used in CI:

```bash
scripts/check-doc-links.sh
```

Coverage (logic-only; excludes thin FFI wrappers):

```bash
scripts/coverage.sh
scripts/coverage.ps1
```

The coverage scripts exclude thin FFI wrapper modules and the `teamtalk-sys`
crate so the reported percentage reflects logic we can validate without the
SDK or a live server.

Use the `mock` feature for deterministic event-driven tests without a running
TeamTalk server.

Backend abstractions are intentionally limited to logic-heavy areas that need
mocking (recording, scripting, channels). Thin FFI wrappers stay direct to
avoid unnecessary indirection. If you add a new mockable logic layer, route it
through the backend; if you add a thin wrapper, keep it direct.

## Architecture Overview

The SDK wraps the TeamTalk C API with a polling client and typed structures.
The design emphasizes:

- Event-driven flow via `Client::poll()`.
- Strongly typed IDs (`UserId`, `ChannelId`) to avoid mixing values.
- Explicit conversion between Rust structs and TeamTalk FFI types.
- `Client` is thread-safe (`Send` + `Sync`) and uses internal locking.
- For concurrent usage, prefer `Client::split()` and keep polling on one thread
  while sending commands from another.

### Modules

- `client`: main client and per-domain operations (users, channels, audio).
- `types`: strongly typed data structures and conversion helpers.
- `events`: event and error types emitted by polling.
- `utils`: string and math helpers for working with TeamTalk types.

## Documentation Style

- Keep user-facing docs focused on usage and configuration.
- Keep developer details in this file.
- Avoid internal implementation details in user guides.

## Recommended Release Profile (for Apps)

For applications that depend on this SDK, you can use a release profile like:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

Avoid putting these settings into the SDK itself; keep them in your app crate.

For development, the default dev profile is typically fine:

```toml
[profile.dev]
opt-level = 0
debug = true
```
