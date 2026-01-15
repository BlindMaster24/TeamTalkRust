param(
    [string]$PrevRef = "",
    [string]$Package = "teamtalk",
    [string]$OutputDir = "reports"
)

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
if ([string]::IsNullOrWhiteSpace($PrevRef)) {
    $PrevRef = (git -C $root tag --sort=-creatordate | Select-Object -First 1)
}

if ([string]::IsNullOrWhiteSpace($PrevRef)) {
    Write-Error "No git tag found. Pass -PrevRef vX.Y.Z"
    exit 1
}

$publicApi = Get-Command cargo-public-api -ErrorAction SilentlyContinue
if (-not $publicApi) {
    Write-Error "cargo-public-api is not installed. Install with: cargo install cargo-public-api"
    exit 1
}

$reports = Join-Path $root $OutputDir
New-Item -ItemType Directory -Force -Path $reports | Out-Null

$prevFile = Join-Path $reports "public-api-$PrevRef.txt"
$headFile = Join-Path $reports "public-api-HEAD.txt"
$diffFile = Join-Path $reports "public-api-diff-$PrevRef..HEAD.txt"

Push-Location $root
try {
    git checkout $PrevRef | Out-Null
    cargo public-api --package $Package > $prevFile
    git checkout - | Out-Null
    cargo public-api --package $Package > $headFile
} finally {
    Pop-Location
}

$diff = Compare-Object -ReferenceObject (Get-Content $prevFile) -DifferenceObject (Get-Content $headFile)
$diff | ForEach-Object { "{0} {1}" -f $_.SideIndicator, $_.InputObject } | Set-Content $diffFile

"Prev: $prevFile"
"Head: $headFile"
"Diff: $diffFile"