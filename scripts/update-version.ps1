$ErrorActionPreference = "Stop"

$cargoToml = "crates/teamtalk/Cargo.toml"
$versionLine = Get-Content $cargoToml | Where-Object { $_ -match '^version\s*=\s*"' } | Select-Object -First 1
if (-not $versionLine) {
    throw "Version not found in $cargoToml"
}
$version = ($versionLine -replace '^version\s*=\s*"', '') -replace '"\s*$', ''

$files = @(
    "README.md",
    "docs/getting-started.md",
    "docs/features.md"
)

foreach ($file in $files) {
    if (-not (Test-Path $file)) {
        throw "File not found: $file"
    }
    $content = Get-Content $file -Raw
    $content = $content -replace 'teamtalk\s*=\s*"\d+\.\d+\.\d+"', ('teamtalk = "' + $version + '"')
    $content = $content -replace 'teamtalk\s*=\s*\{\s*version\s*=\s*"\d+\.\d+\.\d+"', ('teamtalk = { version = "' + $version + '"')
    Set-Content $file -Value $content
}
