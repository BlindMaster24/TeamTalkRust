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

# Run clippy with warnings denied.
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run full test matrix for workspace.
test:
    cargo test --workspace --all-targets --all-features

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

# Full Definition of Done.
dod: fmt check clippy test doc

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

# Full dependency refresh for patch/minor policy.
deps-refresh-compatible: deps-upgrade-compatible deps-update-lock

# Full dependency refresh including majors.
deps-refresh-major: deps-upgrade-major deps-update-lock
