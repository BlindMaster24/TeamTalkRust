# docs-sync

`docs-sync` is a self-contained Python 3.13 tool that detects — and
optionally repairs — drift between the Rust source code of this repository
and the Markdown documentation under [`docs/`](./), the root
[`README.md`](../README.md), and [`AGENTS.md`](../AGENTS.md).

It is packaged as an independent [uv](https://docs.astral.sh/uv/) project
under [`tools/docs-sync/`](../tools/docs-sync/), uses
[Typer](https://typer.tiangolo.com/) for the CLI, is strictly typed under
`mypy --strict`, linted with `ruff` (`select = ["ALL"]`), and tested with
`pytest`. Output is plain ASCII — no colours, no emoji, no Rich markup —
so that screen readers reproduce it verbatim.

## Why

A code base with heavy external Markdown documentation almost always grows
subtle drift over time:

* version numbers inside `teamtalk = { version = "X.Y.Z", ... }` snippets
  fall behind the crate version;
* Cargo feature names change but old names linger in the docs;
* paths such as ``` `docs/developer.md` ``` are renamed and the old
  references are never updated;
* the "Current Module Baseline" section in `AGENTS.md` stops reflecting
  the real source tree;
* code snippets copy-pasted from `examples/` drift from the real file.

`rustdoc` can only check `///` doc comments inside Rust source — it does
not see `.md` files. `docs-sync` fills that gap.

## Quick start

```bash
# Detect drift (exit 1 when any error-severity finding is reported):
just docs-sync

# Run all checkers but exit 0 regardless (useful during migration):
just docs-sync-warn

# Apply automatic fixes where a fixer exists (versions, snippets):
just docs-sync-fix

# Write reports in every format to target/docs-sync.{txt,md,json}:
just docs-sync-report
```

Direct invocation (identical to the recipes above):

```bash
uv run --project tools/docs-sync docs-sync check
uv run --project tools/docs-sync docs-sync check --warn-only
uv run --project tools/docs-sync docs-sync fix
uv run --project tools/docs-sync docs-sync check \
    --format all --report target/docs-sync
```

## Checkers

Every checker is wired through [`tools/docs-sync/src/docs_sync/checkers/`](../tools/docs-sync/src/docs_sync/checkers/)
and is individually togglable via `scripts/docs_sync.toml`.

### `versions` — crate version drift (auto-fixable)

Scans every included Markdown file for `teamtalk = "X.Y.Z"` and
`teamtalk = { version = "X.Y.Z", ... }` and compares the value against the
`[package].version` field of `crates/teamtalk/Cargo.toml`. Mismatches are
reported as errors. Historical snapshots (the release changelog and the
`docs/migrations/**` guides) are excluded by default so references to
older versions are allowed there.

### `features` — unknown feature names

Reads the `[features]` table of `crates/teamtalk/Cargo.toml` and flags any
`features = [...]` array in a docs snippet that mentions a name which is
not defined in the manifest.

### `module_tree` — `AGENTS.md` baseline freshness

Parses the `## Current Module Baseline` section of `AGENTS.md`, extracts
inline-code paths starting with `src/` or `crates/`, and verifies every
path exists under the configured source roots. Missing paths surface as
warnings.

### `file_refs` — dead inline-code path references

Scans inline code spans (``` `crates/…/file.rs` ```, ``` `docs/foo.md`
```, ``` `scripts/bar.sh` ```) across all included docs and flags paths
that no longer exist on disk. Anchor fragments (`#section`) are stripped
before the existence check.

### `snippets` — embedded source-region sync (auto-fixable)

Keeps fenced code blocks in Markdown locked to named regions in real
source files.

In a documentation file:

    <!-- docs-sync:begin src=crates/teamtalk/examples/bot.rs region=setup -->
    ```rust
    let client = Client::new();
    ```
    <!-- docs-sync:end -->

In the source file (any language — the markers are matched as a
substring, so `//`, `#`, `--`, etc. all work):

```rust
// docs-sync:region setup
let client = Client::new();
// docs-sync:endregion setup
```

The checker extracts the region body, strips common leading whitespace,
and compares it to the current fenced block. `docs-sync fix` rewrites the
block in place when it drifts.

## Configuration

Defaults are baked into the tool and are safe for this repository.
Per-repository overrides live in [`scripts/docs_sync.toml`](../scripts/docs_sync.toml).
Every checker exposes at least an `enabled = true/false` toggle; the
`docs` table controls file discovery via glob include/exclude.

```toml
[sources]
cargo_manifest = "crates/teamtalk/Cargo.toml"
workspace_root = "."

[docs]
include = ["docs/**/*.md", "README.md", "AGENTS.md"]
exclude = [
    "docs/changelog.md",
    "docs/migrations/**/*.md",
]

[module_tree]
baseline_heading = "Current Module Baseline"
source_roots = [
    "crates/teamtalk/src",
    "crates/teamtalk-sys/src",
    "crates/teamtalk-macros/src",
]

[file_refs]
prefixes = ["crates/", "docs/", "scripts/", "examples/", "tools/"]
```

## CI integration

A dedicated workflow at
[`.github/workflows/docs-sync.yml`](../.github/workflows/docs-sync.yml) runs
on every push and pull request:

1. `uv sync --project tools/docs-sync --all-groups --frozen` — install
   locked dependencies.
2. `ruff check`, `ruff format --check`, `mypy --strict` — lint and type
   check the tool itself.
3. `pytest` — run the tool's own unit tests.
4. `docs-sync check --warn-only --format all --report target/docs-sync`
   — run the tool against the repository and upload the report as an
   artefact.

The repository-scan step runs in **warn-only** mode during the bootstrap
period so that existing drift does not block unrelated pull requests. Once
the existing findings have been fixed (see
[Bootstrap plan](#bootstrap-plan) below), the `--warn-only` flag will be
removed and the gate becomes strict.

A [`lefthook`](https://github.com/evilmartians/lefthook) `pre-commit`
hook runs the same warn-only check locally if `uv` is installed.

## Reports

`docs-sync check --format all --report <base>` writes three files:

* `<base>.txt` — plain-text human summary (the same string `check` prints
  to stdout when run without `--report`).
* `<base>.md` — Markdown summary suitable for PR comments; each checker
  becomes its own table.
* `<base>.json` — machine-readable report for downstream tooling.

Every finding carries:

| Field       | Purpose                                                      |
|-------------|--------------------------------------------------------------|
| `checker`   | Stable id of the producing checker (e.g. `versions`).        |
| `severity`  | `error`, `warning`, or `info`.                               |
| `file`      | Repository-relative path of the file with the issue.         |
| `line`      | 1-based line number when applicable.                         |
| `rule`      | Stable id of the rule within the checker.                    |
| `message`   | Human-readable description.                                  |
| `expected`  | Value the tool expected, when available (used by `--fix`).   |
| `actual`    | Value observed in the file.                                  |

## Screen-reader friendly output

All output paths are plain 7-bit ASCII. The CLI sets `NO_COLOR=1` and
`TERM=dumb` before any Click or Typer code runs, forces
`rich_markup_mode=None` and `pretty_exceptions_enable=False` on the Typer
app, and never emits ANSI escapes or Unicode box drawing. The JSON and
Markdown reports use only ASCII table borders (`|`, `-`).

## Extending docs-sync

New checkers are added under
[`tools/docs-sync/src/docs_sync/checkers/`](../tools/docs-sync/src/docs_sync/checkers/)
as small modules exposing `run(config) -> list[Finding]` and — when an
automatic fix is possible — `fix(config) -> int`. Register them in
[`tools/docs-sync/src/docs_sync/checkers/__init__.py`](../tools/docs-sync/src/docs_sync/checkers/__init__.py)
and add tests under
[`tools/docs-sync/tests/`](../tools/docs-sync/tests/).

## Bootstrap plan

The tool is intentionally shipped in `--warn-only` mode first so this
pull request is purely additive. Follow-up PRs will:

1. Run `docs-sync fix` and commit the auto-fixable drift (versions and
   snippets).
2. Manually update the remaining `file_refs` warnings (renamed docs,
   deleted examples).
3. Resolve the `module_tree` warnings in `AGENTS.md` (including the
   legacy brace-expansion syntax `src/dispatch/{mod,...}.rs`).
4. Add a `rustdoc-json`-based `api_refs` checker that verifies every
   ``` `Client::foo` ``` reference in the docs maps to a real public item.
5. Remove the `--warn-only` flag from CI and `lefthook`, turning the gate
   strict.
