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
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` enforces lint rules.
- `cargo test --workspace --all-targets --all-features` runs the full test matrix.
- `cargo doc --no-deps --all-features` builds API docs; `cargo doc --no-deps --all-features --open` opens them.
- `scripts/build-docs.ps1` and `scripts/build-docs.sh` run the docs build locally.
- `scripts/check-doc-links.ps1` and `scripts/check-doc-links.sh` validate that docs paths are proper links.
- Coverage (optional but preferred for test-heavy changes):
  - Install once: `cargo install cargo-llvm-cov`
  - Ensure toolchain component: `rustup component add llvm-tools-preview`
  - Summary: `cargo llvm-cov --workspace --all-targets --all-features --summary-only`
  - LCOV report: `cargo llvm-cov --workspace --all-targets --all-features --lcov --output-path target/coverage.lcov`
- Rustup tooling:
  - Help: `rustup --help` and `rustup <command> --help`
  - Status summary: `rustup show`
  - Update toolchains: `rustup update`
  - Toolchains list: `rustup toolchain list`
  - Components list: `rustup component list` and `rustup component list --installed`
  - Targets list: `rustup target list` and `rustup target list --installed`
  - Overrides: `rustup override list`, `rustup override set <toolchain>`, `rustup override unset`

## Coding Style & Naming Conventions
- Rust 2024 edition; follow rustfmt defaults and keep clippy clean.
- Event-driven only: react to `client.poll()` events and avoid `thread::sleep` for control flow.
- Use strong IDs like `UserId` and `ChannelId`; avoid raw `i32` in public APIs.
- Deliver professional, optimized code with descriptive names and no inline comments in library code or examples.
- Auto-reconnect is opt-in; keep manual reconnect logic out of examples unless explicitly required.
- New optional functionality must be feature-gated in `crates/teamtalk/Cargo.toml`.
- Linting is enforced via workspace rules in `Cargo.toml`; do not relax lint levels without user approval.
- Backend abstractions exist only for logic-heavy paths that need mocking. Thin
  FFI wrappers stay direct unless a specific testable logic layer requires a
  backend hook.
- High-level APIs must have a matching `*_ex` variant that exposes full parameters without defaults.
- Auto-features must be off by default and require explicit opt-in to enable.
- Prefer explicit start/stop calls over implicit background behavior.
- Keep public APIs deterministic; avoid time-based side effects unless explicitly configured.

## Feature Flags
- `dispatch`: event dispatcher wrapper for deterministic event routing.
- `async`: async client wrapper backed by `futures`.
- `logging`: tracing hook for client events.
- `mock`: in-memory test client built on `dispatch`.
- `offline`: disable SDK downloads; require a pre-populated `TEAMTALK_DLL/`.
- `scripts`: Lua scripting support for extensions.
- `plugins`: native plugin loading for extensions.
- `tls-native`: system TLS via the native OS backend (default).
- `tls-rustls`: pure Rust TLS for builds without OpenSSL.

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
- Keep inline `docs/...` or `crates/...` paths out of prose; use Markdown links.
- Use neutral, direct language; avoid subjective or marketing terms like "production".
- Keep paragraphs short (1-3 sentences) and prefer lists for steps.
- Split large topics into subfolders under `docs/` with a small `README.md` index.
- User docs should explain both "what" and "why" in short, practical terms.
- When introducing new high-level APIs, document both the default and `*_ex` variants.
- Document all auto-features with explicit opt-in instructions and default state.

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
- Local pre-commit hooks use `lefthook.yml`; Windows overrides can use `lefthook-local.yml` (see `lefthook-local.example.yml`).

## Definition of Done
- Code changes compile with `cargo check --workspace --all-targets`.
- Formatting passes `cargo fmt --all -- --check`.
- Linting passes `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Tests pass with `cargo test --workspace --all-targets --all-features` unless the user says otherwise.
- Docs build passes `cargo doc --no-deps --all-features`.
- Docs link checks pass (`scripts/check-doc-links.ps1` or `scripts/check-doc-links.sh`).

## Lint and Tool Discovery
- Definition of Done (DoD) means the checklist above must pass before commit/push.
- Show clippy help and flags: `cargo clippy --help`.
- List all lint names: `cargo clippy -- -W help`.
- Explain a lint: `cargo clippy --explain clippy::lint_name`.

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
- Every user-facing change must be added to `docs/changelog.md` under `Unreleased` in the same PR/commit set.
- Version bumps must be in a dedicated commit, even if requested alongside other changes.
- Release workflow: update `docs/changelog.md` by moving `Unreleased` items under the new version header, then update versions in `crates/teamtalk/Cargo.toml` and docs, run the Definition of Done checks, commit the bump separately, tag `vX.Y.Z`, and push commits + tag.

## Testing Guidelines
- Place tests under `crates/<crate>/tests` or `#[cfg(test)]` modules.
- Prefer focused unit tests and feature-gated tests for optional modules.
- Name tests by behavior, for example `recording_start_on_command`.
- Add at least one usage example for every new high-level API, even if no tests are added.
- When adding or expanding tests, run the full test matrix and (if requested) coverage commands above.

## Commit & Pull Request Guidelines
- Use Conventional Commit style: `feat:`, `fix:`, `docs:`, `chore:`.
- PRs should explain intent, list key changes, and include commands run.
- Before committing, run required checks in the Definition of Done.
- Commit workflow: review `git status`, stage only relevant files, review `git diff --staged`, then commit and push after user confirmation.

## Security & Configuration Tips
- The loader downloads SDK binaries from `https://bearware.dk`; use `--features offline` and `TEAMTALK_DLL/` when network access is restricted.

