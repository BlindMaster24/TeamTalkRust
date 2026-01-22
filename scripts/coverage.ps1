#!/usr/bin/env pwsh
$ErrorActionPreference = 'Stop'

$ignore = 'teamtalk-sys|crates/teamtalk/src/client/(audio|video|connection|hooks|server|files|media|desktop|system|hotkeys|manager|registry)\.rs'

cargo llvm-cov --workspace --all-targets --all-features --summary-only --ignore-filename-regex $ignore
