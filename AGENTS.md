# Repository Guidelines

## Project Structure & Module Organization
- `crates/teamtalk/` is the high-level Rust SDK; core modules live under `src/client/`, `src/events.rs`, `src/types.rs`, and `src/utils/`.
- `crates/teamtalk/tests/` holds integration tests; `crates/teamtalk/examples/` holds runnable examples.
- `crates/teamtalk-sys/` contains bindgen-based FFI and loads `TeamTalk.h`.
- `TEAMTALK_DLL/` stores downloaded SDK binaries and headers (git-ignored); `qtTeamTalk/` is the upstream Qt client source.
- `docs/` contains user guides; `README.md` links to docs and entry points.

## Build, Test, and Development Commands
- `cargo build` builds the workspace (`teamtalk`, `teamtalk-sys`).
- `cargo check --workspace --all-targets` runs fast type checks used in CI.
- `cargo fmt --all -- --check` enforces formatting; `cargo fmt --all` applies it.
- `cargo clippy --workspace --all-targets -- -D warnings` enforces lint rules.
- `cargo test --workspace --all-targets --all-features` runs the full test matrix.
- `cargo doc --no-deps --all-features` builds API docs; `cargo doc --no-deps --all-features --open` opens them.
- `scripts/build-docs.ps1` and `scripts/build-docs.sh` run the docs build locally.

## Coding Style & Naming Conventions
- Rust 2024 edition; follow rustfmt defaults and keep clippy clean.
- Event-driven only: react to `client.poll()` events and avoid `thread::sleep` for control flow.
- Use strong IDs like `UserId` and `ChannelId`; avoid raw `i32` in public APIs.
- Deliver professional, optimized code with descriptive names and no inline comments in library code or examples.
- Auto-reconnect is opt-in; keep manual reconnect logic out of examples unless explicitly required.
- New optional functionality must be feature-gated in `crates/teamtalk/Cargo.toml`.

## Feature Flags
- `dispatch`: event dispatcher wrapper for deterministic event routing.
- `async`: async client wrapper backed by `futures`.
- `logging`: tracing hook for client events.
- `mock`: in-memory test client built on `dispatch`.
- `offline`: disable SDK downloads; require a pre-populated `TEAMTALK_DLL/`.

## Documentation Guidelines
- Keep user-facing documentation in `README.md` and `docs/`.
- Put contributor-only guidance in `docs/developer.md`.
- When adding a feature, update:
  - `crates/teamtalk/Cargo.toml` for feature flags.
  - `docs/features.md` for the feature list.
  - `README.md` only if onboarding or default usage changes.
  - `docs/developer.md` if module layout or behavior changes.
  - `docs/developer.md` if new tests or commands are required.
  - `docs/README.md` if new docs pages are added or removed.
- When removing a feature or doc page, delete its entry from `docs/README.md` and `docs/features.md`.
- Keep user docs free of implementation details; put internal mechanics in `docs/developer.md`.

## Agent Documentation Workflow
- Before editing docs, re-read `AGENTS.md` and confirm current expectations.
- If the user adds new requirements, ask whether they should be recorded in `AGENTS.md`.
- If you learn a new permanent rule or workflow, ask the user to confirm adding it to `AGENTS.md`.
- If the user requests changes that conflict with existing rules, ask which rule to keep and update `AGENTS.md` accordingly.
- If you remove or add docs pages, update `docs/README.md` and mention it in your response.
- If you add or remove feature flags, update `docs/features.md` and confirm the list with the user.

## Doc Placement Rules
- `README.md`: high-level overview, quick start, installation, links to docs.
- `docs/README.md`: user-facing documentation index only.
- `docs/getting-started.md`: onboarding flow with a minimal working example.
- `docs/features.md`: user-facing feature list and how to enable features.
- `docs/configuration.md`: runtime setup, SDK binaries, networking, TLS usage.
- `docs/developer.md`: contributor guidance, testing commands, architecture notes, doc build steps.

## Required Prompts
- Ask before adding new docs pages outside `docs/`.
- Ask whether to update `AGENTS.md` when the user introduces new permanent requirements.
- Never use `git add .`; always stage only the files relevant to the task.

## Definition of Done
- Code changes compile with `cargo check --workspace --all-targets`.
- Formatting passes `cargo fmt --all -- --check`.
- Linting passes `cargo clippy --workspace --all-targets -- -D warnings`.
- Tests pass with `cargo test --workspace --all-targets --all-features` unless the user says otherwise.
- Docs build passes `cargo doc --no-deps --all-features`.

## Documentation Change Response Template
- Summarize which docs pages were updated.
- Call out any links or indices adjusted in `docs/README.md`.
- Mention if `AGENTS.md` was updated due to new rules.

## Versioning and Changelog
- Use semantic versioning for `crates/teamtalk/Cargo.toml`.
- Record user-facing changes in `docs/changelog.md` under the version header.
- Update version references in `README.md`, `docs/getting-started.md`, and `docs/features.md` when the crate version changes.
- Use `scripts/update-version.ps1` or `scripts/update-version.sh` to sync version references.
- Keep `docs/changelog.md` limited to user-facing changes; keep CI/CD details in `docs/developer.md`.

## Testing Guidelines
- Place tests under `crates/<crate>/tests` or `#[cfg(test)]` modules.
- Prefer focused unit tests and feature-gated tests for optional modules.
- Name tests by behavior, for example `recording_start_on_command`.

## Commit & Pull Request Guidelines
- Use Conventional Commit style: `feat:`, `fix:`, `docs:`, `chore:`.
- PRs should explain intent, list key changes, and include commands run.

## Security & Configuration Tips
- The loader downloads SDK binaries from `https://bearware.dk`; use `--features offline` and `TEAMTALK_DLL/` when network access is restricted.

