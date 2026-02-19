set positional-arguments

# Show all available tasks.
default:
    @just --list

# Run formatter in check mode.
fmt:
    cargo fmt --all -- --check

# Apply formatter.
fmt-fix:
    cargo fmt --all

# Type-check workspace and all targets.
check:
    cargo check --workspace --all-targets --all-features

# Type-check workspace without all features.
check-fast:
    cargo check --workspace --all-targets

# Type-check workspace with one explicit feature.
check-feature feature:
    cargo check --workspace --all-targets --features {{feature}}

# Run clippy with warnings denied.
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run clippy without all features.
clippy-fast:
    cargo clippy --workspace --all-targets -- -D warnings

# Run clippy with one explicit feature.
clippy-feature feature:
    cargo clippy --workspace --all-targets --features {{feature}} -- -D warnings

# Run full test matrix for workspace.
test:
    cargo test --workspace --all-targets --all-features

# Run tests without all features.
test-fast:
    cargo test --workspace --all-targets

# Run tests with one explicit feature.
test-feature feature:
    cargo test --workspace --all-targets --features {{feature}}

# Run a single test target.
test-one target:
    cargo test --test {{target}}

# Run a single test name filter.
test-filter filter:
    cargo test --workspace --all-targets --all-features {{filter}}

# Build docs with all features.
doc:
    cargo doc --no-deps --all-features

# Open local docs in browser.
doc-open:
    cargo doc --no-deps --all-features --open

# Validate markdown links (Unix shell).
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

# Build docs bundle script (bash).
docs-build:
    bash ./scripts/build-docs.sh

# Build docs bundle script (PowerShell).
docs-build-ps:
    ./scripts/build-docs.ps1

# Full Definition of Done.
dod: fmt check clippy test doc

# CI-like full check for Linux shell users.
ci: dod doc-links version-check

# CI-like full check for PowerShell users.
ci-ps: dod doc-links-ps version-check-ps

# Very fast local sanity pass.
quick: fmt check-fast test-fast

# Show latest published version of a crate from crates.io.
search crate:
    cargo search {{crate}} --limit 1

# Show crate metadata and latest version.
info crate:
    cargo info {{crate}}

# Update dependency requirements within compatible ranges (patch/minor).
deps-upgrade-compatible:
    cargo upgrade --manifest-path crates/teamtalk/Cargo.toml
    cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml

# Update dependency requirements including major versions.
deps-upgrade-major:
    cargo upgrade --manifest-path crates/teamtalk/Cargo.toml --incompatible allow --pinned allow
    cargo upgrade --manifest-path crates/teamtalk-sys/Cargo.toml --incompatible allow --pinned allow

# Refresh lockfile to newest allowed versions.
deps-update-lock:
    cargo update

# Update lockfile for one package.
deps-update-one crate:
    cargo update -p {{crate}}

# Show outdated dependencies.
deps-outdated:
    cargo outdated --workspace --root-deps-only

# Full dependency refresh for patch/minor policy.
deps-refresh-compatible: deps-upgrade-compatible deps-update-lock

# Full dependency refresh including majors.
deps-refresh-major: deps-upgrade-major deps-update-lock

# Refresh patch/minor deps and run full CI checks.
deps-refresh-compatible-verify: deps-refresh-compatible ci

# Refresh major deps and run full CI checks.
deps-refresh-major-verify: deps-refresh-major ci

# Show installed toolchain and components.
rustup-show:
    rustup show

# Show available Cargo subcommands.
cargo-list:
    cargo --list

# Run TeamTalk header+docs audit pass.
audit-pass:
    python .codex/skills/teamtalk-h-doc-audit/scripts/run_audit_pass.py --root .
