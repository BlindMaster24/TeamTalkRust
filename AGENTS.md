# Repository Guidelines

## Project Structure & Module Organization
- `crates/teamtalk/` is the high-level Rust SDK; core modules live under `src/client/`, `src/events.rs`, `src/types/`, `src/dispatch/`, `src/loader/`, and `src/utils/`.
- `crates/teamtalk/tests/` holds integration tests; `crates/teamtalk/examples/` holds runnable examples.
- `crates/teamtalk-sys/` contains bindgen-based FFI and loads `TeamTalk.h`.
- `TEAMTALK_DLL/` stores downloaded SDK runtime files, import/static libs, `TeamTalk.h`, and `Documentation/` (git-ignored); `qtTeamTalk/` is the upstream Qt client source.
- `docs/` contains user guides; `README.md` links to docs and entry points.

## Current Module Baseline
- `client` is directory-first:
  - `src/client/core/` for runtime lifecycle, polling, reconnect state/recovery, and debug internals.
  - `src/client/connection/` for connect/reconnect/auto-reconnect/keep-alive concerns.
  - `src/client/audio/` for device operations and audio-block streaming APIs.
  - `src/client/hooks/` for registration, dispatch pipeline, and hook types.
  - `src/client/recording/synced/` for synced recording session/writer/helpers.
  - `src/client/users/` for account/auth plus moderation/text/subscription APIs.
  - `src/client/desktop.rs` for desktop sharing, input translation, and cursor operations; routes through `TeamTalkBackend` with safe `DesktopWindowView` accessor on guard types.
  - `src/client/video.rs` for video capture and transmission; routes through `TeamTalkBackend` with safe `VideoFrameView` accessor on guard types.
  - `src/client/media.rs` for media file streaming, local playback, and palette queries; routes through `TeamTalkBackend`.
  - `src/client/files.rs` for channel file listing; routes through `TeamTalkBackend`.
  - `src/client/recorder.rs` (feature-gated `mock`) for `EventRecorder`/`EventReplayer`/`RecordedEvent` with serde roundtrip support.
  - `src/client/backend.rs` defines `TeamTalkBackend` trait with both `#[cfg(feature = "mock")]` (public + sealed) and `#[cfg(not(feature = "mock"))]` (pub(crate)) blocks that must stay in sync.
  - `src/client/backend_mock.rs` provides `MockBackend` with stub implementations and `push_raw_message` for injecting test events.
- `types` is directory-first:
  - `src/types/entities/` for high-level domain entities and conversion surfaces.
  - focused modules (`audio.rs`, `channels.rs`, `users.rs`, `server.rs`, etc.) for cohesive type groups.
- `dispatch` is split into `src/dispatch/{mod,types,source,dispatcher}.rs`.
- `loader` is split into `src/loader/{mod,versions,download}.rs`.
- `extensions` provides plugin infrastructure:
  - `src/extensions/plugins.rs` for `TeamTalkPlugin` trait, `PluginFlow`, `PluginError`, and `PluginManager` (register, load, dispatch, unload).
- `bot/router` is split into `src/bot/router/{mod,builder,dispatch,help,helpers}.rs`.
- `bot` currently includes:
  - `src/bot/fsm.rs` for dialog/session state, timeout policy, metadata, and flow helpers.
  - `src/bot/middleware.rs` for function middleware, guards, and rate limiting.
  - `src/bot/permissions.rs` for rights-based permission presets on top of TeamTalk account rights.
  - `src/bot/storage.rs` plus Redis/SQLite adapters for bot state backends; StateStore v2 adds `exists`, `set_with_ttl`, `keys`, `remove_prefix`, `get_many`, `set_many` with TTL support in Memory (Instant+Entry), Redis (SCAN/SETEX), and SQLite (expires_at column).
  - `src/bot/scheduler.rs` for sync job scheduling with named jobs and `every_named`/`after` variants.
  - `src/bot/scheduler_async.rs` (feature-gated `async`) for `AsyncScheduler` with tokio and futures runtime integration.
  - `src/bot/runtime_async.rs` integrates `tokio::select!` and `futures::select!` into the async bot run loop.
- `events.rs` includes `FfiError` enum for typed FFI failure classification; `Error::Ffi` variant exposes it in the public error surface.

## Module Structure Guidelines (for this repo)
- Prefer small, focused modules; split files when a module grows beyond ~400-600 lines or mixes multiple responsibilities.
- Keep public API surface shallow: expose through `lib.rs` and module `mod.rs`, hide internals in submodules with `pub(crate)`.
- Group by domain (client, events, types, utils) over technical layers; new features should follow existing domain boundaries.
- If a domain grows, split into a folder with `mod.rs` and focused files (for example `client/connection/*`, `client/audio/*`, `types/entities/*`).
- Avoid circular dependencies between modules; if needed, extract shared types into `types` or shared helpers into `utils`.
- Keep FFI wrappers in `teamtalk-sys`; keep safe abstractions in `teamtalk` (no unsafe in high-level modules).
- Examples live in `crates/teamtalk/examples/`; integration tests in `crates/teamtalk/tests/`.
- Prefer `*_ex` variants for high-level APIs to expose full parameters, as per project rules.
- Prefer one public `impl Client` surface per domain module and move internal helpers into sibling private files.
- Keep cross-file access narrow: default private, then `pub(super)`, then `pub(crate)` only when required.
- Do not move public items between modules without preserving re-export paths from the parent `mod.rs`.
- For structural refactors, enforce behavior parity with existing tests before and after file moves.
- Keep bot authorization logic aligned with TeamTalk account rights:
  - prefer `UserRights`, `Client::my_user_rights()`, and rights-based guards for authorization;
  - treat `RequireUserType` as a cache-based classification helper, not the default authorization path.

## Refactor Practices (required)
- Structural refactor means no API/semantic changes unless explicitly requested.
- Preserve public type/function paths through parent module re-exports where needed.
- Split by behavior boundaries, not by arbitrary line count:
  - lifecycle/state
  - command dispatch/ffi calls
  - conversion/mapping helpers
  - event pipeline/filters
- Keep unsafe and FFI boundary logic localized and documented in the narrowest module possible.
- During decomposition, update imports first, then move code, then run `fmt`, `clippy`, `test`.
- If a split causes temporary compile failures, fix module wiring before any behavior changes.

## Library Architecture & API Design (Rust)
- Keep crates layered: `teamtalk-sys` = raw FFI, `teamtalk` = safe, ergonomic SDK.
- Public API should be minimal, stable, and documented; everything else stays `pub(crate)`.
- Prefer newtype wrappers for IDs and handles; avoid raw primitives at boundaries.
- Use traits when you need polymorphism or testability; avoid traits as a default abstraction.
- Prefer composition over deep inheritance‑style trait hierarchies.
- Expose configuration via structs/builders; avoid long parameter lists.
- Make default behavior explicit; optional behavior behind feature flags.
- Favor deterministic behavior; avoid background tasks unless explicitly started.
- Document invariants and safety for any public API or unsafe boundary.
## Documentation Sources & Validation
- If guidance is unclear, check official docs or primary sources (Rust book, std docs, crate docs).
- When the user asks for best practices or “proper” patterns, verify against authoritative references.
- If advice depends on a specific crate, consult its official docs/changelog before suggesting API usage.
- When the user asks to “check blogs/books,” summarize the key points and cite the source domain/title in the response.
- Prefer official references first; use blogs only to supplement or clarify.
- If sources conflict, present both and explain the tradeoff.
## TeamTalk.h Comment Audit Workflow
- Use this workflow when asked to verify SDK behavior from TeamTalk comments or check if wrappers miss important requirements.
- Primary source order:
  - `TEAMTALK_DLL/TeamTalk.h` (canonical API signatures + behavior comments).
  - `TEAMTALK_DLL/Documentation/C-API/` (narrative docs, examples, cross-links).
  - `crates/teamtalk-sys/` generated bindings (verify symbol exposure and type shape).
  - `crates/teamtalk/src/` safe wrappers and event/state logic (verify behavior mapping).
- Why this order:
  - Header comments stay closest to API changes and constraints.
  - Documentation can lag or summarize, while `.h` stays authoritative for exact function contracts.
- Required audit sequence:
  1. Extract target area in `TeamTalk.h` (connection, reconnect, login, channels, keep-alive, events).
  2. Capture exact requirement from comments (preconditions, ordering, retries, pointer validity, ownership, threading).
  3. Map each requirement to Rust wrapper entry points in `crates/teamtalk/src/client/`.
  4. Verify wrapper behavior and state transitions in code (not only method names).
  5. Verify tests exist for the behavior in `crates/teamtalk/tests/`.
  6. Verify user docs reflect behavior (`README.md`, `docs/getting-started.md`, `docs/features.md`, `docs/configuration.md`, `docs/dev.md`, `docs/changelog.md`).
  7. Record gaps as: missing API, wrong behavior, missing tests, missing docs, or unclear error surface.
- Commands to use during audit:
  - `rg -n "<TT_FunctionName|keyword>" TEAMTALK_DLL/TeamTalk.h`
  - `rg -n "<keyword>" TEAMTALK_DLL/Documentation/C-API`
  - `rg -n "<TT_FunctionName|method|event>" crates/teamtalk-sys crates/teamtalk/src crates/teamtalk/tests`
  - `cargo test --workspace --all-targets --all-features` after behavior changes.
- Comment-to-wrapper mapping checklist (must verify each):
  - Precondition enforcement (for example: disconnect-before-reconnect barriers).
  - Return value handling and typed error mapping.
  - Event ordering assumptions and connection-state transitions.
  - Retry/backoff semantics and termination conditions.
  - Security-sensitive handling (in-memory secrets only, no secret logging, no persistent password writes unless explicit config).
  - Platform-specific or feature-flag behavior.
- Deliverable format for audit tasks:
  - "Source requirement" (header comment or C-API section).
  - "Current Rust behavior" (file + function).
  - "Gap/Risk" (if any).
  - "Fix plan" (code/test/docs updates).
- If header and documentation disagree, follow `TeamTalk.h` and note the mismatch explicitly in docs/changelog when it affects users.
## TeamTalk.h Coverage Audit
- There is currently no project-local audit skill under `.codex/skills/`; `.codex/skills/` is effectively empty except for `.gitkeep`.
- The canonical audit entry point is `python scripts/audit_teamtalk_coverage.py --root .`.
- The script writes:
  - `target/teamtalk-coverage-audit.json`
  - `target/teamtalk-coverage-audit.md`
  - `target/teamtalk-coverage-audit.txt`
- Select outputs with `--format`; accepted values are `json`, `md`, `txt`, `all`, or combinations separated by commas, spaces, or both such as `md,txt`, `md txt`, or `json, md`.
- When asked to audit SDK coverage, run the audit manually against:
  - `TEAMTALK_DLL/TeamTalk.h`
  - `TEAMTALK_DLL/Documentation/C-API/`
  - `crates/teamtalk-sys/`
  - `crates/teamtalk/src/`
  - `crates/teamtalk/tests/`
  - `docs/`
- Manual audit workflow:
  1. Extract the target `TT_*` symbols or API group from `TEAMTALK_DLL/TeamTalk.h`.
  2. Verify symbol exposure in `crates/teamtalk-sys/` or generated bindings under `target/debug/build/teamtalk-sys-*/out/bindings.rs`.
  3. Verify whether a safe/high-level wrapper exists in `crates/teamtalk/src/`.
  4. Verify whether integration coverage exists in `crates/teamtalk/tests/`.
  5. Verify whether user-facing docs mention the behavior in `docs/` or `README.md`.
- Preferred shell commands for manual audits:
  - `rg -n "<TT_FunctionName|keyword>" TEAMTALK_DLL/TeamTalk.h`
  - `rg -n "<keyword>" TEAMTALK_DLL/Documentation/C-API`
  - `rg -n "<TT_FunctionName|method|event>" crates/teamtalk-sys crates/teamtalk/src crates/teamtalk/tests docs`
- Coverage interpretation rules:
  - `symbol in header, missing from bindings` = target/bindgen/export gap.
  - `symbol in bindings, missing from src` = candidate high-level wrapper gap.
  - `symbol in src, missing from tests` = likely test gap for user-facing behavior.
  - `symbol in src, missing from docs` = likely docs gap when behavior is user-visible.
  - `wrapped_symbols` = direct `TT_*` hit in `crates/teamtalk/src` for a runtime candidate.
  - `direct_test_hits_for_wrapped_symbols` = direct `TT_*` hit in `crates/teamtalk/tests` for a wrapped runtime candidate.
  - `fully_covered_symbols` = direct `TT_*` hit in source, tests, and docs for a wrapped runtime candidate.
  - `missing_tests` is intentionally stricter than runtime-behavior coverage; treat it as audit triage, not automatic proof of broken functionality.
- Wrapper-needed rules:
  - Treat non-macro, non-platform-specific `TT_*` symbols as runtime API candidates in the audit output.
  - Then triage them manually before adding wrappers; the scanner should over-report candidates rather than silently hide them.
  - Add a high-level wrapper when the symbol is a user-facing runtime API with safe semantics and clear downstream value.
  - The scanner detects macro/constant symbols from `TeamTalk.h` declarations instead of relying on a fixed prefix list.
  - Keep constants/macros as intentional omissions unless a typed Rust surface is missing.
  - Keep platform-specific symbols as intentional omissions on unsupported targets unless the repository explicitly adds target support.
  - Keep specialized low-level utilities omitted until there is a concrete user-facing use case.
- High-risk-first execution order for fixes/tests:
  1. Connection lifecycle and reconnect barriers.
  2. Login/join/channel state transitions.
  3. Kick/ban/moderation and event ordering.
  4. File transfer, media, desktop, and hotkeys.
  5. Lower-risk utility/config APIs and constants.
## Project-Local Multi-Agent Roles (`.codex/agents`)
- This repository defines project-scoped multi-agent roles in:
  - `.codex/config.toml`
  - `.codex/agents/teamtalk-checker.toml`
  - `.codex/agents/teamtalk-reviewer.toml`
- These roles are project-local by design. Do not move them to global `~/.codex` unless explicitly requested.
- Trust requirement (important):
  - Codex loads `.codex/config.toml` only for trusted projects.
  - On Windows, trust entry should use extended path format in user config (`~/.codex/config.toml`), for example:
    - `[projects."\\\\?\\D:\\path\\to\\project"]`
    - `trust_level = "trusted"`
- Current role intent:
  - `teamtalk_checker`: deep contract checker (`TeamTalk.h` primary source, full docs sweep, wrapper/test/docs mapping, gap analysis with P0/P1/P2).
  - `teamtalk_reviewer`: strict engineering review (bugs/regressions/API risks/concurrency/lifecycle/missing tests/docs mismatch), findings-first output.
- Operating model:
  - `agents.max_depth = 1` keeps one-level delegation (no nested sub-agents from child agents).
  - `agents.max_threads = 6` allows bounded parallelism without uncontrolled thread fan-out.
  - Role configs use max quality settings (`gpt-5.3-codex`, `model_reasoning_effort = "xhigh"`, `model_verbosity = "high"`).
- Expected usage:
  - Run both roles in parallel for release-readiness review.
  - Merge findings into one severity-ordered report with concrete file/symbol evidence.
  - If role behavior looks like fallback/default, verify project trust and restart Codex CLI before re-running.
## Context7 MCP Usage (Default)
- Always use Context7 MCP by default when the task involves library/API documentation, code generation, setup steps, configuration steps, or framework-specific usage.
- Do not wait for the user to explicitly request Context7 for these cases.
- First resolve the library id via Context7, then query docs with concrete version-aware prompts.
- Prefer Context7/official docs over memory for API signatures, feature flags, config keys, and migration guidance.
- If Context7 and local code differ, treat local code as source of truth for current repository behavior and document the mismatch.
- If Context7 is unavailable or insufficient, fall back to official upstream docs/changelogs and mention the fallback in the response.
## External Research & Verification (when requested)
- If the user asks to “look it up” or “verify,” do it before answering.
- Use primary sources (official docs, RFCs, upstream repos) for API or behavior claims.
- Record the exact version or date when advice is time‑sensitive.
## Release Hygiene
- Order of operations: code -> tests -> docs -> changelog -> version bump (separate commit).
- Release commit must only contain version + changelog + synced references.
- Release automation uses `release-plz` via `.github/workflows/release-plz.yml` and `release-plz.toml`.
- Release workflow token strategy:
  - Preferred: GitHub App token from `actions/create-github-app-token`.
  - Fallback: `RELEASE_PLZ_TOKEN`, then `GITHUB_TOKEN`.
  - Required secrets for GitHub App mode: `RELEASE_PLZ_APP_ID`, `RELEASE_PLZ_APP_PRIVATE_KEY`.
- `actions/create-github-app-token` update policy:
  - Keep it pinned to a commit SHA in workflow files.
  - Check latest version with:
    - `gh api repos/actions/create-github-app-token/releases/latest --jq '.tag_name, .published_at'`
    - `gh api repos/actions/create-github-app-token/git/ref/tags/<tag> --jq '.object.sha, .object.type'`
  - If the tag points to an annotated tag object, resolve commit with:
    - `gh api repos/actions/create-github-app-token/git/tags/<sha> --jq '.object.sha, .object.type'`
- No unrelated refactors in release commits.
- Current release-plz baseline:
  - `docs/changelog.md` is the canonical source for released notes (versioned sections only).
  - `release_always = false` is enabled in `release-plz.toml`.
  - `semver_check = true` is enabled for `teamtalk`; breaking API changes can force major releases.
  - Keep `release_commits` commented unless explicitly requested; it is an opt-in noise filter.
  - Publish policy is split:
    - `push` to `main` runs only `release-pr` creation/update.
    - publish runs on merged `release-plz/*` PRs and on explicit manual dispatch.
  - Publishing guardrail with `release_always = false`:
    - `release-plz release` publishes only when the current commit is associated with a merged `release-plz/*` PR.
    - Running `release-plz release` manually on arbitrary `main` commits will log: `skipping release: current commit is not from a release PR`.
    - If you need a publish retry, use a merged `release-plz/*` branch/PR flow instead of forcing publish from a random commit.
  - `dry_run` action input safety:
    - In `release-plz/action`, `dry_run` is presence-sensitive in the wrapper script; avoid passing `dry_run: false` explicitly in always-on publish jobs.
    - For non-dry publish jobs, omit the `dry_run` input entirely.
    - For manual workflows, split steps:
      - step A (`if: inputs.dry_run == true`) with `dry_run: true`
      - step B (`if: inputs.dry_run != true`) without `dry_run` input
    - Symptom of misconfiguration: release job succeeds but logs `due to dry, skipping ... creation of tag ... creation of git release`.
  - Post-publish verification checklist:
    - Verify release workflow job success (`gh run list --workflow release-plz.yml`).
    - Verify GitHub release exists (`gh release list --limit 5`).
    - Verify tag exists on remote (`git ls-remote --tags origin | rg "refs/tags/vX.Y.Z"`).
    - If release exists but crate is missing, inspect `Run release-plz release` logs for token/publish lines.
- Workflow hardening baseline:
  - Keep action refs pinned to commit SHAs in `.github/workflows/release-plz.yml`.
  - Keep concurrency groups on release jobs to avoid duplicate publish/tag races.
  - Keep manual fallback (`workflow_dispatch`) enabled for recovery.
- Changelog writing baseline:
  - Keep entries user-facing only (API/behavior/docs affecting users).
  - Avoid duplicate summary + detailed bullets for the same change.
  - Do not mix CI/process notes into user-facing release notes.
- Release-plz docs map (must read before changing release behavior):
  - `.codex/release-plz/Configuration.txt`: all `release-plz.toml` keys and behavior (`release_always`, `release_commits`, `semver_check`, `[changelog]` options).
  - `.codex/release-plz/Changelog.txt`: changelog generation model and parser rules.
  - `.codex/release-plz/GitHub Action.txt`: Action setup, required permissions, `dry_run`, token model, inputs/outputs.
  - `.codex/release-plz/CLI Usage.txt`: manual commands (`release-pr`, `release`) and local verification flow.
  - `.codex/release-plz/Troubleshooting.txt`: common failure patterns and recommended fixes.
  - `.codex/release-plz/FAQ.txt`, `.codex/release-plz/Extra.txt`: edge-case behavior and advanced options.
- Breaking-change method for release-plz (required when semver-check reports API break):
  - Preferred commit title format: `feat(scope)!: ...` or `fix(scope)!: ...`.
  - Required footer for explicit migration signal: `BREAKING CHANGE: <what changed and how to migrate>`.
  - Keep migration note concise and user-actionable (what changed, why, replacement pattern).
  - Do not hide breaking notes only in PR comments; keep them in commit message and release notes.
  - Important: for `teamtalk` release notes, the breaking commit must touch `crates/teamtalk/**`; docs-only commits (for example only `docs/**`) won't be included in the package changelog block.
  - If you need a migration link in generated changelog, place it in the same breaking commit footer that touches the package: `BREAKING CHANGE: ... Migration: <url>`.
- If release PR shows major bump but changelog has no `Breaking` section:
  - Cause: semver-check detects API break, but changelog groups are still based on commit parser rules.
  - Fix sequence:
    1. Create a new commit touching the target package path (for `teamtalk`: `crates/teamtalk/**`).
    2. Use a breaking commit headline (`type(scope)!: ...`).
    3. Add `BREAKING CHANGE:` footer with migration guidance.
    4. Push to `main` and wait for `Release-plz PR` workflow to update the existing `release-plz/*` PR.
  - Verification:
    - `gh run list --workflow "Release-plz" --limit 5`
    - `gh pr view <number> --json body,title,url`
    - Confirm both:
      - PR body contains semver-check break details.
      - Generated changelog block contains `### Breaking`.
- Practical release-plz troubleshooting order:
  1. Confirm action run status first (workflow success/failure).
  2. If workflow is green but output is unexpected, inspect PR body/changelog block.
  3. Treat semver-check findings as versioning signals, not action failures.
  4. Avoid manual changelog edits in `main`; prefer parser-driven commits so release-plz regenerates deterministically.
  5. For missing tags/releases, inspect `Run release-plz release` logs for:
     - `due to dry, skipping ...`
     - `skipping release: current commit is not from a release PR`
     - token/publish errors.
  6. If publish was skipped due to release-PR association, create/merge a `release-plz/*` branch PR to trigger canonical publish path.
## API Review Checklist (before shipping)
- Backward compatibility assessed (breaking vs additive).
- Deprecation plan and migration notes where needed.
- Feature flags updated and documented.
- Examples updated and compile.
## Error & Logging Rules
- Public error types are stable and documented.
- Error messages must be actionable and avoid leaking secrets.
- Logging should use consistent levels and include context identifiers.
## Backport / Hotfix Rules
- Minimal, targeted fix in a dedicated commit.
- Explain scope and risk in commit body.
- Prefer revert+fix to history rewriting.
## Benchmarking & Performance Practices
- Benchmarks live only under `benches/` and run via `cargo bench`.
- Use stable-friendly tooling (Criterion) unless the project explicitly requires nightly `#[bench]`.
- Run benchmarks in release mode (default `cargo bench`) and keep inputs deterministic.
- Avoid benchmarking in shared CI unless results are stable; prefer local runs with recorded context.
- If performance-sensitive code changes, add or update a benchmark and summarize results in the PR/commit context.
## Miri (UB Detection) Usage
- Miri is an interpreter that detects undefined behavior; useful for FFI and unsafe-heavy code.
- Requires nightly toolchain and component:
  - `rustup toolchain install nightly --component miri`
  - or `rustup +nightly component add miri`
- Run tests under Miri: `cargo +nightly miri test` (same flags as `cargo test`).
- Run a binary under Miri: `cargo +nightly miri run`.
- First run may download extra sysroot; consider `cargo miri setup` to prepare.
- If you switch between Miri and normal builds, run `cargo clean` to avoid mixed artifacts.
- Miri is single-threaded and slow; use sparingly or with nextest if configured.

## Build, Test, and Development Commands
- `cargo build` builds the workspace (`teamtalk`, `teamtalk-sys`).
- `cargo check --workspace --all-targets` runs fast type checks used in CI.
- `cargo fmt --all -- --check` enforces formatting; `cargo fmt --all` applies it.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` enforces lint rules.
- `cargo test --workspace --all-targets --all-features` runs the full test matrix.
- `cargo nextest run --workspace --all-features` is the preferred strict local runner.
- `cargo nextest run --profile ci --workspace --all-features` is the preferred CI-style runner.
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
- Cargo cache management (`cargo-cache`):
  - `cargo-cache` manages `~/.cargo/` (global registry, git checkouts, installed binaries).
  - `cargo cache -a` — show size breakdown (registry index, crate archives, source checkouts, git dbs, binaries).
  - `cargo cache --autoclean` — remove crate source checkouts and old archive versions not needed by any local lockfile; safe default.
  - `cargo cache --autoclean-expensive` — same, but scans all workspace Cargo.lock files on the system; slower but more thorough.
  - `cargo cache --clean-ci` — aggressive cleanup: git checkouts, crate source checkouts, and stale registry archives; use when disk space is critical.
  - `cargo cache --list` — show installed binaries and their sizes.
  - `cargo cache --fsck` — verify registry integrity; fix broken entries.
  - Typical usage: run `cargo cache --autoclean` after `cargo clean` or when disk is full; `cargo cache -a` to inspect before cleaning.
  - The tool is global (operates on `~/.cargo/`, not `target/`); `cargo clean` handles `target/` separately.
## Task Runner (`just`)
- The repo ships a root `justfile` as a convenience CLI for common workflows.
- `just` is optional; every task must still be runnable with direct `cargo`/`gh`/scripts commands.
- Install recommended tooling:
  - `cargo install just cargo-edit cargo-outdated cargo-llvm-cov cargo-nextest cargo-cache`
- The `justfile` sets `windows-shell` to PowerShell so recipes work on Windows without a separate `sh` installation.
- Discover tasks with `just --list`.
- Prefer `just` commands first for repeatable local flows; if `just` or a required subcommand is unavailable, use the equivalent manual commands.
- Core `just` command map (preferred):
  - Environment/tooling:
    - `just env-check`
    - `just tools-install`
    - `just rustup-show`
    - `just cargo-list`
  - Quality:
    - `just quick`
    - `just quick-nextest`
    - `just qa-full`
    - `just qa-nextest`
  - Repository nextest config lives in `.config/nextest.toml` with a strict default profile and a softer `ci` profile.
    - `just dod`
    - `just ci`
    - `just ci-nextest`
    - `just check-feature <feature>`
    - `just clippy-feature <feature>`
    - `just test-feature <feature>`
    - `just test-nextest`
    - `just test-nextest-feature <feature>`
    - `just test-one <target>`
    - `just test-filter <pattern>`
    - `just examples`
    - `just examples-feature <feature>`
    - `just bench`
    - `just miri-test`
  - Docs/version refs:
    - `just doc`
    - `just doc-open`
    - `just docs-build`
    - `just doc-links`
    - `just coverage`
    - `just version-check`
    - `just version-sync`
  - Dependencies:
    - `just deps-outdated`
    - `just deps-outdated-all`
    - `just deps-major-report`
    - `just deps-refresh-compatible`
    - `just deps-refresh-major`
    - `just deps-safe-cycle`
    - `just deps-major-cycle`
    - `just deps-update-one <crate>`
  - Release/GitHub:
    - `just release-status`
    - `just release-dry`
    - `just release-run`
    - `just release-watch`
    - `just pr-open`
    - `just pr-release-open`
    - `just runs-list`
    - `just runs-fail`
  - Workspace:
    - `just clean`
    - `just rebuild`
- For unknown/new commands, run `just --list` and then execute the matching recipe.
- Use direct commands as fallback when `just` is unavailable.
### Detailed `just` Usage and Fallback Matrix
- Execution order policy:
  - Prefer `just` recipe first.
  - If `just` is missing: run equivalent manual command(s).
  - If recipe exists but dependent CLI is missing (for example `gh`, `cargo-outdated`, nightly+miri), either:
    1) install missing tool via `just tools-install` (or direct install), or
    2) continue with available checks and clearly report which optional step was skipped.
- Daily operations (recommended sequence):
  1. `just env-check` (verify toolchain + CLI presence).
  2. `just quick-nextest` (fast health check during active development).
  3. `just test-nextest-feature <feature>` when working in one feature slice.
  4. `just release-status` when release state/PR state matters.
- Weekly maintenance (recommended sequence):
  1. `just deps-outdated`
  2. `just deps-safe-cycle`
  3. `just runs-fail`
- Pre-release operations (recommended sequence):
  1. `just qa-nextest`
  2. `just release-dry`
  3. `just release-watch`
- Release-day operations (explicit publish path):
  1. `just release-status`
  2. `just release-run`
  3. `just release-watch`
- Fallback equivalents for critical recipes:
  - `just quick-nextest` ->
    - `cargo fmt --all -- --check`
    - `cargo check --workspace --all-targets`
    - `cargo nextest run --workspace`
  - `just qa-nextest` / `just ci-nextest` ->
    - `cargo fmt --all -- --check`
    - `cargo check --workspace --all-targets --all-features`
    - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    - `cargo nextest run --profile ci --workspace --all-features`
    - `cargo test --workspace --examples --all-features`
    - `cargo doc --no-deps --all-features`
    - `bash ./scripts/check-doc-links.sh` (or `./scripts/check-doc-links.ps1`)
    - `bash ./scripts/check-version-refs.sh` (or `./scripts/check-version-refs.ps1`)
  - `just deps-refresh-compatible` ->
    - `cargo upgrade --manifest-path crates/teamtalk/Cargo.toml`
    - `cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml`
    - `cargo update`
  - `just deps-refresh-major` ->
    - `cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible allow --pinned allow`
    - `cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible allow --pinned allow`
    - `cargo update`
  - `just release-dry` ->
    - `gh workflow run release-plz.yml -f dry_run=true`
  - `just release-run` ->
    - `gh workflow run release-plz.yml -f dry_run=false`
  - `just release-watch` ->
    - `gh run watch $(gh run list --workflow "Release-plz" --limit 1 --json databaseId --jq '.[0].databaseId')`
- Expected tool prerequisites by recipe family:
  - Cargo-only recipes: Rust toolchain + workspace dependencies.
  - `deps-outdated*`: `cargo-outdated` installed.
  - `miri-test`: nightly toolchain + `miri` component installed.
  - Release/GitHub recipes: authenticated `gh` CLI and repository access.
- Failure handling policy:
  - Never silently skip a failed step.
  - Report exact failing recipe and command output summary.
  - For optional recipes (bench, miri, outdated), state "optional check skipped" with reason.
  - For required release/quality recipes, stop and fix before proceeding to commit/push.
- Command selection policy by intent:
  - "quick local confidence" -> `just quick`
  - "full ready-to-push quality gate" -> `just qa-full`
  - "dependency refresh without majors" -> `just deps-safe-cycle`
  - "dependency refresh including majors" -> `just deps-major-cycle`
  - "check release pipeline without publish" -> `just release-dry` + `just release-watch`
  - "perform publish run" -> `just release-run` + `just release-watch`
- Windows/Linux shell policy:
  - Use PowerShell variants when available on Windows (`*-ps`).
  - Use bash variants in Linux/macOS and CI.
  - Keep command separators platform-correct (`;` in PowerShell).
## Command Invocation Conventions
- When a task requires multiple shell commands, run them sequentially and keep each command explicit.
- In PowerShell, use `;` as the command separator (not `&&`).
- Prefer one logical check per command for readable logs (for example: `cargo fmt`, then `cargo check`, then `cargo test`).
- For long workflows, record the exact command order in commit/PR notes so reruns are reproducible.
- For skill invocations in chat, call one skill per line and wait for completion before triggering the next step.
- If a step is destructive (force-push, branch delete, reset), require explicit user confirmation before running.
## Cargo Help & Command Discovery
- `cargo --list` lists all installed Cargo subcommands (including third-party ones like `clippy`, `fmt`, `llvm-cov`, `sqlx`).
- `cargo help` shows general usage and built-in commands.
- `cargo help <command>` shows detailed help for a specific command (e.g. `cargo help test`, `cargo help doc`).
## Cargo Dependency Tooling (`cargo-edit` + crates.io search)
- `cargo search <name>` searches crates.io and shows the latest published version(s). Use `--limit N` to see more results.
- `cargo info <crate>` shows crate metadata (latest version, features, repository, docs link, and more).
- `cargo add <crate>` / `cargo remove <crate>` come from `cargo-edit` and modify `Cargo.toml` directly.
- `cargo upgrade` (from `cargo-edit`) updates dependency requirements in `Cargo.toml`; by default it can include major updates.
- Install tooling: `cargo install cargo-edit`; verify commands with `cargo --list`.
- Full workspace manifests in this repo:
  - `crates/teamtalk/Cargo.toml`
  - `crates/teamtalk-sys/Cargo.toml`
  - `crates/teamtalk-macros/Cargo.toml`
- Safe non-major policy:
  - Verify latest published crate versions first with `cargo search <crate> --limit 1`.
  - Update manifest constraints without major jumps:
    - `cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible false`
    - `cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible false`
    - `cargo upgrade --manifest-path crates/teamtalk-macros/Cargo.toml --incompatible false`
  - Then refresh lockfile: `cargo update`.
  - Then run DoD checks (`fmt`, `check`, `clippy`, `test`, `doc`, doc links).
- Full major-refresh policy:
  - `cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible allow --pinned allow`
  - `cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible allow --pinned allow`
  - `cargo upgrade --manifest-path crates/teamtalk-macros/Cargo.toml --incompatible allow --pinned allow`
  - `cargo update`
- `cargo update` alone does not change `Cargo.toml`; it only updates `Cargo.lock` within already-allowed semver ranges.
- Prefer pinning dependency features explicitly in `Cargo.toml`; avoid implicit default features unless intentional.
- Before upgrading dependencies, inspect current tree with `cargo tree` and re-check after upgrade for changed transitive graph.
- Keep dependency updates in a dedicated commit when possible.
## Core Cargo Commands (practical)
- `cargo build` builds the workspace.
- `cargo check --workspace --all-targets` runs fast type checks (DoD).
- `cargo fmt --all -- --check` verifies formatting (DoD).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` runs lints (DoD).
- `cargo test --workspace --all-targets --all-features` runs tests (DoD).
- `cargo doc --no-deps --all-features` builds docs (DoD).
- `cargo doc --no-deps --all-features --open` opens docs locally.
- `cargo add <crate>` / `cargo remove <crate>` manage dependencies.
- `cargo update` updates the lockfile.
- `cargo tree` shows dependency graph.
## Cargo Cheatsheet by Task
- **Quick sanity (local):**
  - `cargo fmt --all -- --check`
  - `cargo check --workspace --all-targets`
- **Full DoD (before push/release):**
  - `cargo fmt --all -- --check`
  - `cargo check --workspace --all-targets`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-targets --all-features`
  - `cargo doc --no-deps --all-features`
  - `scripts/check-doc-links.ps1` (Windows) or `scripts/check-doc-links.sh` (Unix)
- **Docs only:**
  - `cargo doc --no-deps --all-features --open`
  - `scripts/build-docs.ps1` / `scripts/build-docs.sh`
- **Dependency inspection:**
  - `cargo tree`
  - `cargo metadata`

## SDK Version Overrides
- The loader pins to `crates/teamtalk/SDK_VERSION.txt` by default.
- Set `TEAMTALK_SDK_VERSION` to override (for example `v5.19` or `latest`).
- Precedence: env var > `SDK_VERSION.txt` > latest from BearWare.
- Loader payload in `TEAMTALK_DLL/`: `TeamTalk5.dll`/`libTeamTalk5.so`, `TeamTalk5.lib`/`libTeamTalk5.a`, `TeamTalk.h`, `Documentation/`, and `TEAMTALK_DOCUMENTATION_MANIFEST.txt`.
- If any required SDK binary or documentation file is missing, the loader restores the selected version automatically (online mode).
- `offline` feature disables downloads and requires `TEAMTALK_DLL/` to be pre-populated.

## Coding Style & Naming Conventions
- Rust 2024 edition; follow rustfmt defaults and keep clippy clean.
- Event-driven only: react to `client.poll()` events and avoid `thread::sleep` for control flow.
- Use strong IDs like `UserId` and `ChannelId`; avoid raw `i32` in public APIs.
- Deliver professional, optimized code with descriptive names and no inline comments in library code or examples.
- Auto-reconnect is opt-in; keep manual reconnect logic out of examples unless explicitly required.
- New optional functionality must be feature-gated in `crates/teamtalk/Cargo.toml`.
- Linting is enforced via workspace rules in `Cargo.toml`; do not relax lint levels without user approval.
- Prefer `UnpoisonedMutex<T>` from `crate::utils` over `std::sync::Mutex<T>` for internal mutable state. Do not use `.lock().unwrap_or_else(|e| e.into_inner())`; use `UnpoisonedMutex::lock()` instead.
- All public enums must have `#[non_exhaustive]`, except FFI-mapping enums that mirror `TeamTalk.h` constants 1:1 (e.g. `AudioCodec`, `AudioPreprocessor`, `FileTransferStatus`). Adding a new enum variant without `#[non_exhaustive]` is a semver-breaking change; the attribute prevents this.
- Backend abstractions exist only for logic-heavy paths that need mocking. Thin
  FFI wrappers stay direct unless a specific testable logic layer requires a
  backend hook.
- High-level APIs must have a matching `*_ex` variant that exposes full parameters without defaults.
- Auto-features must be off by default and require explicit opt-in to enable.
- Prefer explicit start/stop calls over implicit background behavior.
- Keep public APIs deterministic; avoid time-based side effects unless explicitly configured.

## Feature Flags
- `dispatch`: event dispatcher wrapper for deterministic event routing; `FfiError` enum for typed FFI failure classification.
- `async`: async client wrapper backed by `futures` and `futures-timer`.
- `logging`: tracing hook for client events.
- `mock`: in-memory test client built on `dispatch`; includes `EventRecorder`/`EventReplayer` for capture-replay testing.
- `offline`: disable SDK downloads; require a pre-populated `TEAMTALK_DLL/`.
- `scripts`: Lua scripting support for extensions.
- `plugins`: native plugin loading via `TeamTalkPlugin` trait, `PluginFlow`, and `PluginManager`.
- `tls-native`: system TLS via the native OS backend (default).
- `tls-rustls`: pure Rust TLS for builds without OpenSSL.

## Documentation Guidelines
- Keep user-facing documentation in `README.md` and `docs/`.
- Put contributor-only guidance in `docs/dev.md`.
- When adding a feature, update:
  - `crates/teamtalk/Cargo.toml` for feature flags.
  - `docs/features.md` for the feature list.
  - `README.md` only if onboarding or default usage changes.
  - `docs/dev.md` if module layout or behavior changes.
  - `docs/dev.md` if new tests or commands are required.
  - `docs/README.md` if new docs pages are added or removed.
- When removing a feature or doc page, delete its entry from `docs/README.md` and `docs/features.md`.
- Keep user docs free of implementation details; put internal mechanics in `docs/dev.md`.
- Keep inline `docs/...` or `crates/...` paths out of prose; use Markdown links.
- Use neutral, direct language; avoid subjective or marketing terms like "production".
- Keep paragraphs short (1-3 sentences) and prefer lists for steps.
- Split large topics into subfolders under `docs/` with a small `README.md` index.
- Do not add `docs/migrations/README.md`; keep migration docs as explicit versioned files (for example `docs/migrations/2-to-3.md`) and link them directly.
- User docs should explain both "what" and "why" in short, practical terms.
- When introducing new high-level APIs, document both the default and `*_ex` variants.
- Document all auto-features with explicit opt-in instructions and default state.
- For bot authorization docs:
  - present rights-based guards (`RequireClientRightsAny`, `RequireClientRightsAll`, `Permissions::*`) before `RequireUserType`;
  - explain that `RequireUserType` depends on local sender cache state and is secondary to account-rights checks.

## Documentation Updates from a Specific Commit
- When asked to update docs from a baseline commit, diff from that commit to `HEAD`.
- Summarize all user-facing changes in the next release section of `docs/changelog.md`.
- Update affected guides (README + docs/) only when behavior or usage changed.
- Replace raw `docs/` or `crates/` paths in prose with Markdown links.

## Agent Documentation Workflow
- Before editing docs, re-read `AGENTS.md` and confirm current expectations.
- Before any release/versioning/release-plz task, re-read `docs/release.md` fully.
- Before any release-plz setup/configuration/troubleshooting task, fully read everything under `.codex/release-plz/`; do not skip files and re-read when needed.
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
- `docs/dev.md`: contributor guidance, testing commands, architecture notes, doc build steps.

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
- Changelog scope: include only user-facing changes (API, behavior, new features, fixes, user docs). Exclude internal commit policy, linting rules, refactors, and test-only changes unless they impact users.
- Changelog completeness: for any release, enumerate all user-facing changes since the last release baseline (or chosen baseline commit); do not shorten or omit relevant user-facing items.
- Release checklist for changes since a baseline commit:
  - List commits: `git log --oneline <baseline>..HEAD`.
  - Summarize diff scope: `git diff --stat <baseline>..HEAD`.
  - Draft changelog groups (Added/Changed/Fixed/Docs/Tests/Chore) using those results.
- Update version references in `README.md`, `docs/getting-started.md`, and `docs/features.md` when the crate version changes.
- Use `scripts/update-version.ps1` or `scripts/update-version.sh` to sync version references.
- Use `scripts/check-version-refs.ps1` or `scripts/check-version-refs.sh` to verify refs are already synced.
- Keep `docs/changelog.md` limited to user-facing changes; keep CI/CD details in `docs/dev.md`.
- Every user-facing change must be represented by release-plz output in the next release section.
- Version bumps must be in a dedicated commit, even if requested alongside other changes.
- Release workflow:
  - Push normal feature/fix PRs with Conventional Commits; release-plz generates changelog sections.
  - On push to `main`, release-plz opens/updates a release PR (version + changelog).
  - release-plz workflow then syncs version refs in `README.md`, `docs/getting-started.md`, and `docs/features.md`.
  - After release PR merge, release-plz tags and publishes the release.
- Manual verification commands:
  - `bash ./scripts/check-version-refs.sh` (Linux/macOS)
  - `./scripts/check-version-refs.ps1` (Windows PowerShell)

## Testing Guidelines
- Place tests under `crates/<crate>/tests` only.
- Do not add `#[cfg(test)]` unit-test modules inside library source files.
- Prefer focused unit tests and feature-gated tests for optional modules.
- Prefer `rstest` for repetitive scenario matrices and fixtures instead of hand-written copy-paste test cases.
- Prefer `proptest` for roundtrip and invariant-heavy surfaces (typed flags, conversions, parser/state invariants).
- Name tests by behavior, for example `recording_start_on_command`.
- Add at least one usage example for every new high-level API, even if no tests are added.
- When adding or expanding tests, run the full test matrix and (if requested) coverage commands above.
- Bot/scenes/permissions changes since `ff082e8` are covered by:
  - `crates/teamtalk/tests/bot_fsm.rs`
  - `crates/teamtalk/tests/bot_scene_integration.rs`
  - `crates/teamtalk/tests/bot_middleware_guards.rs`
  - `crates/teamtalk/tests/bot_state_json.rs`
  - `crates/teamtalk/tests/bot_state_v2.rs`
- Mock-based client module tests:
  - `crates/teamtalk/tests/desktop_client_tests.rs`
  - `crates/teamtalk/tests/video_client_tests.rs`
  - `crates/teamtalk/tests/media_client_tests.rs`
  - `crates/teamtalk/tests/event_replay_tests.rs`
- Matching examples for this surface include:
  - `crates/teamtalk/examples/bot_dialog.rs`
  - `crates/teamtalk/examples/bot_macros.rs`
  - `crates/teamtalk/examples/bot_permissions.rs`
  - `crates/teamtalk/examples/plugin_loader.rs`

## Commit & Pull Request Guidelines
- Use Conventional Commit style: `feat:`, `fix:`, `docs:`, `chore:`.
- PRs should explain intent, list key changes, and include commands run.
- Before committing, run required checks in the Definition of Done.
- Commit workflow: review `git status`, stage only relevant files, review `git diff --staged`, then commit and push after user confirmation.
- If any uncertainty about what changed, review `git diff` and `git diff --staged` before naming or creating commits.
- When preparing changes requested “since a commit”, inspect the actual diff (`git diff <base>..HEAD`) and read the relevant commits (not just their titles).
- **Commit splitting policy (strict):**
  - One commit per concern: code, tests, docs, and config changes are separate commits unless they must land together to keep builds green.
  - If a change touches multiple subsystems (e.g., client + sys crate), split by subsystem unless one commit is required for correctness.
  - Each commit must be explainable in one sentence and be buildable (or clearly marked why not in the body).
- **API change definition (project):**
  - Any change to `pub` API, feature flags, user-visible behavior, or documented usage counts as an API change.
  - API changes require matching docs updates and changelog entry in the same PR/commit set.
- **Docs update scope (wide):**
  - When API changes land, re-audit README + all `docs/` pages for accuracy.
  - Do not whitelist specific files; verify the entire `docs/` directory for correctness.
- **Formatting commits:**
  - Formatting-only changes are allowed only when required by rustfmt or CI.
  - If required, keep formatting-only edits in a dedicated commit, never mixed with behavior changes.
- **Staging checklist (before commit):**
  - `git status -sb` to confirm scope.
  - `git diff` to review all working-tree changes.
  - `git diff --staged` to confirm only intended files are staged.
  - Ensure the staged diff matches the commit message intent.
- **Commit message rules (practical):**
  - Summary ≤ 72 chars, imperative, one intent.
  - Use body when rationale, tradeoffs, or migrations are involved.
  - For multi-step work, include a short body line describing the step.
- **Docs accuracy rule:**
  - If docs mention an API/behavior, confirm it exists in code or update docs.
  - Never leave docs describing removed/renamed APIs.
- **Example usage rule:**
  - If a new public API is added, add/update an example or doc snippet that compiles.
- **Safety sanity checks (before commit):**
  - Search for accidental debug artifacts (`TODO`, `FIXME`, `println!`, `dbg!`, `unwrap` in library code).
  - Ensure no secrets, tokens, or local paths are introduced in diffs.
  - Avoid committing generated artifacts unless explicitly required.
- **Dependency updates:**
  - Use a dedicated commit for dependency bumps unless required by the same change.
  - Document user-visible dependency impacts in `docs/changelog.md` when relevant.
  - Dependabot policy for this repository:
    - `.github/dependabot.yml` runs daily for Cargo and GitHub Actions.
    - Cargo updates are grouped into `cargo-patch-minor` and `cargo-major`.
    - GitHub Actions updates are grouped under `github-actions-all`.
    - Dependabot PRs auto-assign and auto-request review from `BlindMaster24`.
    - Patch/minor Dependabot PRs are auto-merge candidates via `.github/workflows/dependabot-automerge.yml`.
    - Weekly digest issue is maintained via `.github/workflows/dependency-digest.yml`.
    - Keep workflow action refs pinned to SHAs so Dependabot updates explicit SHAs.
- **Refactor discipline:**
  - Refactors must not change behavior; if they do, split or document and test.
  - Keep refactor commits small and isolated from feature work.
- **Review discipline:**
  - Before requesting review or pushing, re-scan the diff for scope creep and unintended changes.
  - If a change is risky or user-visible, add a brief commit body explaining impact and mitigation.
  - Use explicit test evidence in PR/commit context (commands run + outcomes).
- **Test discipline:**
  - Match tests to behavior: new behavior → new/updated tests; bugfix → regression test when feasible.
  - Prefer focused unit tests; use integration tests only for public API or feature interactions.
  - Avoid network/IO tests unless explicitly required; mock or isolate.
  - If tests are skipped, explain why and how to validate manually.
- Keep commits production-grade: one logical change per commit. If a task spans multiple concerns, split into 2-5 focused commits (or more if justified).
- If asked to push, still confirm that commits are scoped correctly before pushing.
- Version bumps are always a dedicated commit. Never combine a version bump with other changes, even if requested.
- Version bumps are handled by release-plz; keep release PR commits scoped to version/changelog/docs sync only.
- Commit hygiene:
  - Keep diffs minimal; avoid drive‑by refactors unless explicitly requested.
  - Separate behavior changes from refactors and from docs-only edits.
  - If a change needs tests and doc updates, prefer: code -> tests -> docs (separate commits if they stand alone).
  - Never mix formatting-only edits with behavior changes unless rustfmt is required.
  - Avoid mixing unrelated files; each commit should have a single intent you can explain in one sentence.
- Commit sizing:
  - Prefer smaller, reviewable commits; break large changes into 2-5 commits with clean boundaries.
  - Do not split a single cohesive change across multiple commits just to hit a number.
  - When in doubt, split by module or by feature boundary (e.g., core logic vs. tests vs. docs).
- Commit messages:
  - Use imperative mood and <= 72 characters in the summary.
  - Use a body when rationale or migration guidance matters.
  - Include references to breaking changes and how to migrate.
 - Validation before push:
  - Default: rely on lefthook for DoD checks on commit.
  - If lefthook is disabled or skipped, run the full DoD before pushing.
  - Confirm staged diff matches intent; never push partial/unfinished work.
  - Prefer clean history: avoid "WIP" commits in main history.
  - Use a dedicated commit for dependency updates unless they are required by the same change.
  - If a revert is needed, use `revert:` with a clear reason rather than rewriting history.
  - Do not push failing CI unless explicitly approved; fix or revert first.
  - Keep public API changes in their own commit when possible and document them.
  - Avoid squashing unrelated commits; preserve logical boundaries for auditability.
  - If backporting or cherry-picking, note it in the commit body.
  - Keep CI green; if it breaks, prioritize fix or revert before new changes.
  - For hotfixes, isolate the minimal fix in a single commit and document impact.
  - Avoid hidden behavior changes; surface them in commit body and changelog.
  - Do not reword or squash commits after review starts unless explicitly requested.
  - Use `fixup!`/`squash!` only on private branches; never leave them in main history.
  - Ensure commit order matches dependency order (foundations before dependents).
  - When splitting work, keep each commit buildable or clearly mark why not.
  - For multi-step changes, include a short commit body explaining the step in the series.
  - If a commit changes user-visible behavior, add or update tests in the same or next commit.
  - Prefer deterministic commits: avoid relying on local environment state in commit content.

## Security & Configuration Tips
- The loader downloads SDK binaries from `https://bearware.dk`; use `--features offline` and `TEAMTALK_DLL/` when network access is restricted.
