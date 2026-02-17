# Release Process

This repository uses release-plz to automate version bumps, changelog updates,
tag creation, GitHub releases, and crates.io publishing.

## Prerequisites

- Workflow config: [release-plz.yml](../.github/workflows/release-plz.yml)
- release-plz config: [release-plz.toml](../release-plz.toml)
- Repository secret: `CRATES_IO_TOKEN`
- GitHub Actions default workflow permissions: `Read and write`

## Normal Development Flow

1. Merge feature and fix PRs to `main`.
2. Keep [changelog.md](changelog.md) updated under `## Unreleased` for all
   user-facing changes.
3. Keep Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`) so release-plz
   can infer the correct semantic version bump.

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
