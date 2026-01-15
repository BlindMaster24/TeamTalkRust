param(
    [ValidateSet("md","json")]
    [string]$Format = "md",
    [string]$BindingsPath = "",
    [string]$Root = ""
)

if ([string]::IsNullOrWhiteSpace($Root)) {
    $Root = (Resolve-Path (Join-Path $PSScriptRoot ".."))
}

$targetRoot = Join-Path $Root "target"
if ([string]::IsNullOrWhiteSpace($BindingsPath)) {
    $candidates = Get-ChildItem -Path $targetRoot -Filter bindings.rs -Recurse -File -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending
    if (-not $candidates) {
        Write-Error "bindings.rs not found under $targetRoot"
        exit 1
    }
    $BindingsPath = $candidates[0].FullName
}

$bindingsContent = Get-Content -Raw $BindingsPath
$sysFns = [regex]::Matches($bindingsContent, "pub fn (TT_[A-Za-z0-9_]+)\b") |
    ForEach-Object { $_.Groups[1].Value } |
    Sort-Object -Unique
$sysAllFns = [regex]::Matches($bindingsContent, "pub fn ([A-Za-z0-9_]+)\b") |
    ForEach-Object { $_.Groups[1].Value } |
    Sort-Object -Unique
$sysTypes = [regex]::Matches($bindingsContent, "pub (struct|enum|type) ([A-Za-z0-9_]+)\b") |
    ForEach-Object { $_.Groups[2].Value } |
    Sort-Object -Unique
$sysConsts = [regex]::Matches($bindingsContent, "pub const ([A-Za-z0-9_]+)\b") |
    ForEach-Object { $_.Groups[1].Value } |
    Sort-Object -Unique
$sysStatics = [regex]::Matches($bindingsContent, "pub static ([A-Za-z0-9_]+)\b") |
    ForEach-Object { $_.Groups[1].Value } |
    Sort-Object -Unique

$srcRoot = Join-Path $Root "crates\teamtalk\src"
$srcFiles = Get-ChildItem -Path $srcRoot -Filter *.rs -Recurse -File
$usedFns = foreach ($file in $srcFiles) {
    $content = Get-Content -Raw $file.FullName
    [regex]::Matches($content, "\bTT_[A-Za-z0-9_]+\b") | ForEach-Object { $_.Value }
}
$usedFns = $usedFns | Sort-Object -Unique
$usedAllFns = foreach ($file in $srcFiles) {
    $content = Get-Content -Raw $file.FullName
    [regex]::Matches($content, "\b[A-Za-z0-9_]+\s*\(") |
        ForEach-Object { $_.Value.TrimEnd("(").Trim() }
}
$usedAllFns = $usedAllFns | Sort-Object -Unique
$definedTypes = foreach ($file in $srcFiles) {
    $content = Get-Content -Raw $file.FullName
    [regex]::Matches($content, "pub (struct|enum|type) ([A-Za-z0-9_]+)\b") |
        ForEach-Object { $_.Groups[2].Value }
}
$definedTypes = $definedTypes | Sort-Object -Unique
$definedConsts = foreach ($file in $srcFiles) {
    $content = Get-Content -Raw $file.FullName
    [regex]::Matches($content, "pub const ([A-Za-z0-9_]+)\b") |
        ForEach-Object { $_.Groups[1].Value }
}
$definedConsts = $definedConsts | Sort-Object -Unique
$definedStatics = foreach ($file in $srcFiles) {
    $content = Get-Content -Raw $file.FullName
    [regex]::Matches($content, "pub static ([A-Za-z0-9_]+)\b") |
        ForEach-Object { $_.Groups[1].Value }
}
$definedStatics = $definedStatics | Sort-Object -Unique

$missing = $sysFns | Where-Object { $usedFns -notcontains $_ }
$missingAllFns = $sysAllFns | Where-Object { $usedAllFns -notcontains $_ }
$missingTypes = $sysTypes | Where-Object { $definedTypes -notcontains $_ }
$missingConsts = $sysConsts | Where-Object { $definedConsts -notcontains $_ }
$missingStatics = $sysStatics | Where-Object { $definedStatics -notcontains $_ }

if ($Format -eq "json") {
    $payload = [ordered]@{
        bindings = $BindingsPath
        sys_count = $sysFns.Count
        used_count = $usedFns.Count
        missing_count = $missing.Count
        missing = $missing
        sys_all_fns_count = $sysAllFns.Count
        used_all_fns_count = $usedAllFns.Count
        missing_all_fns_count = $missingAllFns.Count
        missing_all_fns = $missingAllFns
        sys_types_count = $sysTypes.Count
        defined_types_count = $definedTypes.Count
        missing_types_count = $missingTypes.Count
        missing_types = $missingTypes
        sys_consts_count = $sysConsts.Count
        defined_consts_count = $definedConsts.Count
        missing_consts_count = $missingConsts.Count
        missing_consts = $missingConsts
        sys_statics_count = $sysStatics.Count
        defined_statics_count = $definedStatics.Count
        missing_statics_count = $missingStatics.Count
        missing_statics = $missingStatics
    }
    $payload | ConvertTo-Json -Depth 4
    exit 0
}

"# TeamTalk sys vs teamtalk" 
"" 
"Bindings: $BindingsPath" 
"" 
"- sys functions: $($sysFns.Count)" 
"- used in teamtalk: $($usedFns.Count)" 
"- missing in teamtalk: $($missing.Count)" 
"- sys types: $($sysTypes.Count)" 
"- defined in teamtalk: $($definedTypes.Count)" 
"- missing types in teamtalk: $($missingTypes.Count)" 
"- sys consts: $($sysConsts.Count)" 
"- defined consts in teamtalk: $($definedConsts.Count)" 
"- missing consts in teamtalk: $($missingConsts.Count)" 
"- sys statics: $($sysStatics.Count)" 
"- defined statics in teamtalk: $($definedStatics.Count)" 
"- missing statics in teamtalk: $($missingStatics.Count)" 
"- sys functions (all): $($sysAllFns.Count)" 
"- used functions (all): $($usedAllFns.Count)" 
"- missing functions (all): $($missingAllFns.Count)" 
"" 
"## Missing TT_* functions" 
"" 
foreach ($name in $missing) { "- $name" }
"" 
"## Missing sys types" 
"" 
foreach ($name in $missingTypes) { "- $name" }
"" 
"## Missing sys consts" 
"" 
foreach ($name in $missingConsts) { "- $name" }
"" 
"## Missing sys statics" 
"" 
foreach ($name in $missingStatics) { "- $name" }
"" 
"## Missing sys functions (all)" 
"" 
foreach ($name in $missingAllFns) { "- $name" }
