#!/usr/bin/env bash

set -euo pipefail

bash ./scripts/update-version.sh

if ! git diff --quiet -- README.md docs/getting-started.md docs/features.md; then
  echo "Version references are out of sync with crates/teamtalk/Cargo.toml."
  echo "Run scripts/update-version.sh and commit the result."
  git --no-pager diff -- README.md docs/getting-started.md docs/features.md
  exit 1
fi
