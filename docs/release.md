# Release Process

This repository uses release-plz to automate version bumps, changelog updates,
tag creation, GitHub releases, and crates.io publishing.

## Prerequisites

- Workflow config: [release-plz.yml](../.github/workflows/release-plz.yml)
- SemVer check workflow: [semver.yml](../.github/workflows/semver.yml)
- release-plz config: [release-plz.toml](../release-plz.toml)
- Dependabot config: [dependabot.yml](../.github/dependabot.yml)
- Repository secret: `CRATES_IO_TOKEN`
- Preferred repository secret: `RELEASE_PLZ_TOKEN` (PAT or GitHub App token)
- GitHub Actions default workflow permissions: `Read and write`

## Normal Development Flow

1. Merge feature and fix PRs to `main`.
2. Keep Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`) so release-plz
   can infer the correct semantic version bump and changelog grouping.
3. Keep changelog edits (if any) in the release PR only; avoid manual
   pre-population of changelog sections during feature PRs.

## Automated Release Flow

On every push to `main`:

1. `Release-plz PR` job creates or updates the current open release PR with
   version/changelog changes.
2. The workflow syncs version references in:
   - [README](../README.md)
   - [getting-started.md](getting-started.md)
   - [features.md](features.md)
3. No publish happens on plain pushes to `main`.

On merge of a `release-plz/*` PR into `main`:

1. `Release-plz release` runs and publishes the crate.
2. release-plz creates the tag and GitHub release.

## GitHub Token Policy (Recommended)

This repository supports a token fallback, but for reliable release automation
the preferred setup is:

1. Prefer GitHub App authentication via:
   - `RELEASE_PLZ_APP_ID`
   - `RELEASE_PLZ_APP_PRIVATE_KEY`
2. Keep `RELEASE_PLZ_TOKEN` as emergency fallback.
3. Keep `CRATES_IO_TOKEN` for crates publish.
4. Workflow token resolution order:
   - GitHub App token
   - `RELEASE_PLZ_TOKEN`
   - `GITHUB_TOKEN`

Why this is preferred:

- release-plz may use temporary clones and `git push`, so explicit git auth is
  more reliable than relying on checkout-only credentials.
- default `GITHUB_TOKEN` has workflow-trigger limitations in some setups.
- a dedicated token keeps release permissions explicit and easier to audit.

## Dependency Update Automation

Dependabot is configured for both Cargo and GitHub Actions:

- Runs daily for Cargo and workflow dependencies.
- Includes major updates (grouped separately for Cargo).
- Opens grouped PRs to reduce review noise.
- Works with pinned workflow SHAs by updating to newer pinned commits.
- Auto-assigns and auto-requests review from `BlindMaster24`.
- Auto-merges Dependabot patch/minor PRs after required checks pass.
- Publishes a weekly digest issue with all open dependency PRs.

## Release Notes Section Contract

Release-plz changelog generation is explicitly configured in
[release-plz.toml](../release-plz.toml) via `[changelog].commit_parsers`.
Section order is fixed and stable:

1. Breaking
2. Added
3. Changed
4. Deprecated
5. Removed
6. Fixed
7. Security
8. Docs
9. CI
10. Dependencies
11. Other

Rules:

- Commit grouping is strict-first (Conventional Commit style).
- Any commit not matched by a specific parser is placed in `Other`.
- `Other` is intentionally the last section.
- `release_commits` remains disabled by policy, so release-plz still evaluates all commits for release updates.

## Breaking Change Method (Required)

When a public API break is intentional (or reported by `cargo-semver-checks`),
record it in commit metadata so release-plz categorizes it correctly:

1. Use `!` in commit type/scope, for example: `feat(async)!: ...`.
2. Add a `BREAKING CHANGE:` footer with migration guidance.

Template:

```text
feat(scope)!: short summary

BREAKING CHANGE: what changed, why, and exact migration step.
```

For the current async architecture change, use wording like:

```text
BREAKING CHANGE: AsyncClient no longer implements Sync. Keep AsyncClient in one
runtime/task and use wait helpers plus shutdown/into_client for lifecycle.
```

Include the migration guide in the same footer when available:

```text
BREAKING CHANGE: AsyncClient no longer implements Sync. Keep AsyncClient in one
runtime/task and use wait helpers plus shutdown/into_client for lifecycle.
Migration: https://github.com/BlindMaster24/TeamTalkRust/blob/main/docs/migrations/2-to-3.md
```

For major releases, keep the migration guide URL directly in the breaking
commit summary as well, so release notes can surface it without manual edits.

## SemVer Gate in CI

Pull requests to `main` run a dedicated semver compatibility check for the
public API:

```bash
cargo semver-checks check-release --package teamtalk
```

If this check fails, either:

- make the change additive/non-breaking, or
- keep the break and mark it explicitly with `!` and `BREAKING CHANGE:` so the
  release PR can correctly bump the major version.

## Manual Checks

Use these checks locally before merging:

```powershell
./scripts/check-version-refs.ps1
./scripts/check-doc-links.ps1
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo doc --no-deps --all-features
```

Linux/macOS version-ref check:

```bash
bash ./scripts/check-version-refs.sh
```

## CLI Operations

Use this section when you need to inspect or run release steps manually.

### Install Tools

```bash
cargo install release-plz
gh --version
```

### Local Dry Run (No Push)

Run in the repository root:

```bash
release-plz release-pr --dry-run
```

This validates the computed version bump and changelog changes without opening
or updating a PR.

### Publish Preflight Checklist

Before a real release:

1. Run `release-plz release-pr --dry-run`.
2. Confirm `CRATES_IO_TOKEN` exists and is valid.
3. Confirm workflow permissions are `Read and write`.
4. Confirm release-plz workflow run can complete `release-pr` and `release` jobs.

### Trigger Release Workflow Manually

Use GitHub CLI to start the workflow dispatch job:

```bash
gh workflow run "Release-plz" --ref main
```

Manual publish mode: default `dry_run=false` (real publish).

Manual dry-run (no publish):

```bash
gh workflow run "Release-plz" --ref main -f dry_run=true
```

### Inspect Release Workflow Runs

```bash
gh run list --workflow "Release-plz" --limit 5
gh run view <run-id>
gh run watch <run-id>
```

### Common Troubleshooting

- If release PR cannot be updated, verify Actions permissions are `Read and write`.
- If publish fails, verify `CRATES_IO_TOKEN` exists and is valid.
- If docs version sync fails, run `scripts/check-version-refs.ps1` or
  `bash ./scripts/check-version-refs.sh` locally and fix the diff.
