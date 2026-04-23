"""Shared pytest fixtures for docs-sync tests."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from docs_sync.config import Config

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path


@pytest.fixture
def make_workspace(tmp_path: Path) -> Callable[[dict[str, str]], Path]:
    """Factory that writes a file tree and returns the root path.

    The input maps repository-relative paths to UTF-8 file contents. Parent
    directories are created on demand.
    """

    def _make(files: dict[str, str]) -> Path:
        for rel, content in files.items():
            target = tmp_path / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content, encoding="utf-8")
        return tmp_path

    return _make


@pytest.fixture
def make_config() -> Callable[[Path], Config]:
    """Factory that builds a :class:`Config` pointing at ``root``."""

    def _make(root: Path) -> Config:
        cfg = Config()
        cfg.sources.workspace_root = str(root)
        return cfg

    return _make


CARGO_TOML_DEFAULT = """\
[package]
name = "teamtalk"
version = "6.0.0"
edition = "2024"

[features]
default = []
async = []
dispatch = []
bot = []
"""
