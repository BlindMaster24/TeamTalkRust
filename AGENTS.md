# Repository Guidelines

## Project Structure & Module Organization
- `crates/teamtalk/` is the high-level Rust SDK; core modules live under `src/client/`, `src/events.rs`, `src/types.rs`, and `src/utils/`.
- `crates/teamtalk/tests/` holds integration tests; `crates/teamtalk/examples/` holds runnable examples.
- `crates/teamtalk-sys/` contains bindgen-based FFI and loads `TeamTalk.h`.
- `TEAMTALK_DLL/` stores downloaded SDK runtime files, import/static libs, `TeamTalk.h`, and `Documentation/` (git-ignored); `qtTeamTalk/` is the upstream Qt client source.
- `docs/` contains user guides; `README.md` links to docs and entry points.
## Module Structure Guidelines (for this repo)
- Prefer small, focused modules; split files when a module grows beyond ~400–600 lines or mixes multiple responsibilities.
- Keep public API surface shallow: expose through `lib.rs` and module `mod.rs`, hide internals in submodules with `pub(crate)`.
- Group by domain (client, events, types, utils) over technical layers; new features should follow existing domain boundaries.
- If a type group grows, split into a folder with `mod.rs` and separate files (e.g., `types/voice.rs`, `types/channels.rs`).
- Avoid circular dependencies between modules; if needed, extract shared types into `types` or shared helpers into `utils`.
- Keep FFI wrappers in `teamtalk-sys`; keep safe abstractions in `teamtalk` (no unsafe in high-level modules).
- Examples live in `crates/teamtalk/examples/`; integration tests in `crates/teamtalk/tests/`.
- Prefer `*_ex` variants for high-level APIs to expose full parameters, as per project rules.
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
- Order of operations: code → tests → docs → changelog → version bump (separate commit).
- Release commit must only contain version + changelog + synced references.
- No unrelated refactors in release commits.
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
- Safe non-major policy:
  - Update manifest constraints without major jumps: `cargo upgrade --workspace --incompatible false`.
  - Then refresh lockfile: `cargo update`.
  - Then run DoD checks (`fmt`, `check`, `clippy`, `test`, `doc`, doc links).
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

## Documentation Updates from a Specific Commit
- When asked to update docs from a baseline commit, diff from that commit to `HEAD`.
- Summarize all user-facing changes in `docs/changelog.md` under `Unreleased`.
- Update affected guides (README + docs/) only when behavior or usage changed.
- Replace raw `docs/` or `crates/` paths in prose with Markdown links.

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
- Changelog scope: include only user-facing changes (API, behavior, new features, fixes, user docs). Exclude internal commit policy, linting rules, refactors, and test-only changes unless they impact users.
- Changelog completeness: for any release, enumerate all user-facing changes since the last release baseline (or chosen baseline commit); do not shorten or omit relevant user-facing items.
- Release checklist for changes since a baseline commit:
  - List commits: `git log --oneline <baseline>..HEAD`.
  - Summarize diff scope: `git diff --stat <baseline>..HEAD`.
  - Draft changelog groups (Added/Changed/Fixed/Docs/Tests/Chore) using those results.
- Update version references in `README.md`, `docs/getting-started.md`, and `docs/features.md` when the crate version changes.
- Use `scripts/update-version.ps1` or `scripts/update-version.sh` to sync version references.
- Keep `docs/changelog.md` limited to user-facing changes; keep CI/CD details in `docs/developer.md`.
- Every user-facing change must be added to `docs/changelog.md` under `Unreleased` in the same PR/commit set.
- Version bumps must be in a dedicated commit, even if requested alongside other changes.
- Release workflow: update `docs/changelog.md` by moving `Unreleased` items under the new version header, then update versions in `crates/teamtalk/Cargo.toml` and docs, run the Definition of Done checks, commit the bump separately, tag `vX.Y.Z`, and push commits + tag.
 - Release commands (Windows PowerShell):
   - Baseline diff: `git log --oneline <baseline>..HEAD` and `git diff --stat <baseline>..HEAD`
   - Update changelog: edit `docs/changelog.md` (move Unreleased -> X.Y.Z).
   - Version sync: `scripts\\update-version.ps1 X.Y.Z`
   - Required checks:
     - `cargo fmt --all`
     - `cargo check --workspace --all-targets`
     - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
     - `cargo test --workspace --all-targets --all-features`
     - `cargo doc --no-deps --all-features`
     - `scripts\\check-doc-links.ps1`
   - Commit release bump: `git add README.md docs/changelog.md docs/features.md docs/getting-started.md crates/teamtalk/Cargo.toml` then `git commit -m "chore: release X.Y.Z"`
   - Tag and push: `git tag vX.Y.Z` then `git push` and `git push --tags`

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
- Version bump flow (separate commit only): update `docs/changelog.md` (move Unreleased), update `crates/teamtalk/Cargo.toml` version, sync references via `scripts/update-version.*`, run DoD checks, commit with a `chore:` message, tag `vX.Y.Z`, then push commits + tag.
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
