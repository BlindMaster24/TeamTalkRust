# docs-sync

Detect drift between TeamTalkRust source code and external Markdown documentation.

Runs as a self-contained uv project:

```bash
uv run --project tools/docs-sync docs-sync --check
uv run --project tools/docs-sync docs-sync --fix
uv run --project tools/docs-sync docs-sync --format all --report target/docs-sync
```

Configuration is loaded from `scripts/docs_sync.toml` at the repository root.

See `docs/docs-sync.md` for the full user guide.
