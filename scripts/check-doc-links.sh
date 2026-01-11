#!/usr/bin/env bash
set -euo pipefail

pattern='`docs/|`crates/'
targets=(README.md docs)

if grep -R -n -E "$pattern" "${targets[@]}"; then
  echo "Found raw docs/crates paths in prose. Use Markdown links instead."
  exit 1
fi
