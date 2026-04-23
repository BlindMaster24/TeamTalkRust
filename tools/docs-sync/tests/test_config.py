"""Tests for docs-sync config loading."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.config import Config, load

if TYPE_CHECKING:
    from pathlib import Path


def test_missing_file_returns_defaults(tmp_path: Path) -> None:
    cfg = load(tmp_path / "missing.toml")
    assert isinstance(cfg, Config)
    assert cfg.sources.cargo_manifest.endswith("Cargo.toml")


def test_partial_override(tmp_path: Path) -> None:
    path = tmp_path / "cfg.toml"
    path.write_text(
        """
[sources]
cargo_manifest = "x/y/Cargo.toml"

[module_tree]
enabled = false
source_roots = ["foo/src"]

[file_refs]
prefixes = ["foo/"]
""",
        encoding="utf-8",
    )
    cfg = load(path)
    assert cfg.sources.cargo_manifest == "x/y/Cargo.toml"
    assert cfg.module_tree.enabled is False
    assert cfg.module_tree.source_roots == ["foo/src"]
    assert cfg.file_refs.prefixes == ["foo/"]
    # unrelated defaults untouched
    assert cfg.versions.enabled is True
    assert cfg.features.enabled is True


def test_none_returns_defaults() -> None:
    cfg = load(None)
    assert isinstance(cfg, Config)
