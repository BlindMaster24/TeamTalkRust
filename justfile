set positional-arguments

# =========================
# Core
# =========================

# Show all available tasks.
default:
    @just --list

# Verify local tooling versions used by tasks.
env-check:
    cargo --version
    rustup --version
    gh --version
    python --version

# Show installed Rust toolchains and active override.
rustup-show:
    rustup show

# Show available Cargo subcommands.
cargo-list:
    cargo --list

# =========================
# Quality / CI
# =========================

# Run formatter in check mode.
fmt:
    cargo fmt --all -- --check

# Apply formatter.
fmt-fix:
    cargo fmt --all

# Type-check workspace and all targets with all features.
check:
    cargo check --workspace --all-targets --all-features

# Type-check workspace without all-features.
check-fast:
    cargo check --workspace --all-targets

# Type-check workspace with one explicit feature.
check-feature feature:
    cargo check --workspace --all-targets --features {{feature}}

# Run clippy with warnings denied (all features).
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run clippy without all-features.
clippy-fast:
    cargo clippy --workspace --all-targets -- -D warnings

# Run clippy with one explicit feature.
clippy-feature feature:
    cargo clippy --workspace --all-targets --features {{feature}} -- -D warnings

# Apply clippy auto-fixes where possible.
clippy-fix:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

# Run full test matrix for workspace.
test:
    cargo test --workspace --all-targets --all-features

# Run tests without all-features.
test-fast:
    cargo test --workspace --all-targets

# Run tests with one explicit feature.
test-feature feature:
    cargo test --workspace --all-targets --features {{feature}}

# Run a single integration test target.
test-one target:
    cargo test --test {{target}}

# Run tests by name filter.
test-filter filter:
    cargo test --workspace --all-targets --all-features {{filter}}

# Run examples as test targets.
examples:
    cargo test --workspace --examples --all-features

# Run benchmark suite.
bench:
    cargo bench

# Run Miri tests (requires nightly + miri component).
miri-test:
    cargo +nightly miri test

# Full Definition of Done.
dod: fmt check clippy test doc

# CI-like full check for bash users.
ci: dod doc-links version-check

# CI-like full check for PowerShell users.
ci-ps: dod doc-links-ps version-check-ps

# Fast local sanity pass.
quick: fmt check-fast test-fast

# Full QA profile used before release.
qa-full: ci

# =========================
# Docs
# =========================

# Build docs with all features.
doc:
    cargo doc --no-deps --all-features

# Open local docs in browser.
doc-open:
    cargo doc --no-deps --all-features --open

# Build docs helper script (bash).
docs-build:
    bash ./scripts/build-docs.sh

# Build docs helper script (PowerShell).
docs-build-ps:
    ./scripts/build-docs.ps1

# Validate markdown links (bash).
doc-links:
    bash ./scripts/check-doc-links.sh

# Validate markdown links (PowerShell).
doc-links-ps:
    ./scripts/check-doc-links.ps1

# Verify version refs in docs (bash).
version-check:
    bash ./scripts/check-version-refs.sh

# Verify version refs in docs (PowerShell).
version-check-ps:
    ./scripts/check-version-refs.ps1

# Sync version refs in docs (bash).
version-sync:
    bash ./scripts/update-version.sh

# Sync version refs in docs (PowerShell).
version-sync-ps:
    ./scripts/update-version.ps1

# =========================
# Dependencies
# =========================

# Show latest published version of a crate from crates.io.
search crate:
    cargo search {{crate}} --limit 1

# Show crate metadata and latest version.
info crate:
    cargo info {{crate}}

# Show outdated dependencies (root deps only).
deps-outdated:
    cargo outdated --workspace --root-deps-only

# Show full outdated dependency report.
deps-outdated-all:
    cargo outdated --workspace

# Update dependency requirements within compatible ranges (patch/minor).
deps-upgrade-compatible:
    cargo upgrade --manifest-path crates/teamtalk/Cargo.toml
    cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml

# Update dependency requirements including major versions.
deps-upgrade-major:
    cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible allow --pinned allow
    cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible allow --pinned allow

# Preview major dependency upgrades without applying them.
deps-major-report:
    cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible allow --pinned allow --dry-run
    cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible allow --pinned allow --dry-run

# Refresh lockfile to newest allowed versions.
deps-update-lock:
    cargo update

# Update lockfile for one package.
deps-update-one crate:
    cargo update -p {{crate}}

# Full dependency refresh for patch/minor policy.
deps-refresh-compatible: deps-upgrade-compatible deps-update-lock

# Full dependency refresh including majors.
deps-refresh-major: deps-upgrade-major deps-update-lock

# Refresh patch/minor deps and run CI checks.
deps-safe-cycle: deps-refresh-compatible ci

# Refresh major deps and run CI checks.
deps-major-cycle: deps-refresh-major ci

# =========================
# Release / GitHub
# =========================

# List release-plz PRs and latest version tags.
release-status:
    gh pr list --search "head:release-plz/" --state open --limit 20
    gh release list --limit 10

# Trigger release-plz manual workflow in dry-run mode.
release-dry:
    gh workflow run release-plz.yml -f dry_run=true

# Trigger release-plz manual workflow in publish mode.
release-run:
    gh workflow run release-plz.yml -f dry_run=false

# Watch latest release-plz workflow run.
release-watch:
    gh run watch $(gh run list --workflow "Release-plz" --limit 1 --json databaseId --jq '.[0].databaseId')

# Show open PRs.
pr-open:
    gh pr list --state open --limit 30

# Show open release-plz PRs only.
pr-release-open:
    gh pr list --search "head:release-plz/" --state open --limit 20

# Show recent workflow runs.
runs-list:
    gh run list --limit 30

# Show recent failed workflow runs.
runs-fail:
    gh run list --status failure --limit 20

# =========================
# Workspace
# =========================

# Remove build artifacts.
clean:
    cargo clean

# Clean and rebuild checks.
rebuild: clean check

# Install common CLI tools used by this repository.
tools-install:
    cargo install just cargo-edit cargo-outdated cargo-llvm-cov

# Run TeamTalk header+docs audit pass.
audit-pass:
    python scripts/audit_teamtalk_coverage.py --root .
