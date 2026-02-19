# Release Process

This repository uses release-plz to automate version bumps, changelog updates,
tag creation, GitHub releases, and crates.io publishing.

## Prerequisites

- Workflow config: [release-plz.yml](../.github/workflows/release-plz.yml)
- release-plz config: [release-plz.toml](../release-plz.toml)
- Dependabot config: [dependabot.yml](../.github/dependabot.yml)
- Repository secret: `CRATES_IO_TOKEN`
- GitHub Actions default workflow permissions: `Read and write`

## Normal Development Flow

1. Merge feature and fix PRs to `main`.
2. Keep [changelog.md](changelog.md) updated under `## Unreleased` for all
   user-facing changes.
3. Keep Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`) so release-plz
   can infer the correct semantic version bump.
4. Keep `## Unreleased` concise and up to date; release-plz moves it into the
   next version section automatically.

## Automated Release Flow

On every push to `main`:

1. `Release-plz PR` job creates or updates a release PR with version/changelog
   changes.
2. The workflow syncs version references in:
   - [README](../README.md)
   - [getting-started.md](getting-started.md)
   - [features.md](features.md)
3. After merge of the release PR, `Release-plz release` publishes the crate and
   creates the tag/release.

## Dependency Update Automation

Dependabot is configured for both Cargo and GitHub Actions:

- Runs daily for Cargo and workflow dependencies.
- Includes major updates (grouped separately for Cargo).
- Opens grouped PRs to reduce review noise.
- Works with pinned workflow SHAs by updating to newer pinned commits.

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

### Trigger Release Workflow Manually

Use GitHub CLI to start the workflow dispatch job:

```bash
gh workflow run "Release-plz" --ref main
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
