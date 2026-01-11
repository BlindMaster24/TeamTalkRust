Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$pattern = '`docs/|`crates/'
$files = @("README.md")
$files += Get-ChildItem -Path "docs" -Recurse -Filter "*.md" | ForEach-Object { $_.FullName }

$matches = Select-String -Path $files -Pattern $pattern -AllMatches
if ($matches) {
  $matches | ForEach-Object { "$($_.Path):$($_.LineNumber):$($_.Line.Trim())" }
  throw "Found raw docs/crates paths in prose. Use Markdown links instead."
}
