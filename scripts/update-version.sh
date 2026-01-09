set -euo pipefail

cargo_toml="crates/teamtalk/Cargo.toml"
version=$(rg '^version\s*=\s*"' -m 1 "$cargo_toml" | sed -E 's/^version\s*=\s*"([^"]+)".*/\1/')

files=(
  "README.md"
  "docs/getting-started.md"
  "docs/features.md"
)

for file in "${files[@]}"; do
  sed -E -i \
    -e "s/teamtalk\\s*=\\s*\"[0-9]+\\.[0-9]+\\.[0-9]+\"/teamtalk = \"$version\"/g" \
    -e "s/teamtalk\\s*=\\s*\\{\\s*version\\s*=\\s*\"[0-9]+\\.[0-9]+\\.[0-9]+\"/teamtalk = { version = \"$version\"/g" \
    "$file"
done
