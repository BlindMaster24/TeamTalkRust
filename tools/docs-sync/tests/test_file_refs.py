"""Tests for the file_refs checker."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.checkers.file_refs import run
from docs_sync.report import Severity

from .conftest import CARGO_TOML_DEFAULT

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

    from docs_sync.config import Config


def test_existing_path_is_ok(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "docs/dev.md": "See `crates/teamtalk/Cargo.toml` for details.\n",
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []


def test_missing_path_flagged(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "docs/dev.md": "See `docs/missing.md` for details.\n",
        },
    )
    cfg = make_config(root)
    findings = run(cfg)
    assert len(findings) == 1
    assert findings[0].severity == Severity.WARNING
    assert findings[0].actual == "docs/missing.md"


def test_unknown_prefix_ignored(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "docs/dev.md": "inline code like `Foo::bar` is not a path.\n",
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []


def test_anchor_stripped_when_checking(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "docs/dev.md": "See `docs/dev.md#section` for details.\n",
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []
