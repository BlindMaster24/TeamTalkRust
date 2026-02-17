$ErrorActionPreference = "Stop"

& ./scripts/update-version.ps1

$targets = @(
    "README.md",
    "docs/getting-started.md",
    "docs/features.md"
)

$diff = git diff --name-only -- $targets
if ($LASTEXITCODE -ne 0) {
    throw "Failed to check version reference diffs."
}

if ($diff) {
    Write-Host "Version references are out of sync with crates/teamtalk/Cargo.toml."
    Write-Host "Run scripts/update-version.ps1 and commit the result."
    git --no-pager diff -- $targets
    exit 1
}
