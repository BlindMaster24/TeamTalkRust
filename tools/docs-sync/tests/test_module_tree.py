"""Tests for the module_tree checker."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.checkers.module_tree import run
from docs_sync.report import Severity

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

    from docs_sync.config import Config


def test_missing_baseline_path_flagged(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    agents = """\
# AGENTS

## Current Module Baseline

- `src/client/connection/`
- `src/nonexistent.rs`

## Next Section
- ignored
"""
    root = make_workspace(
        {
            "AGENTS.md": agents,
            "crates/teamtalk/src/client/connection/mod.rs": "",
        },
    )
    cfg = make_config(root)
    findings = run(cfg)
    paths = sorted(f.actual for f in findings if f.actual is not None)
    assert paths == ["src/nonexistent.rs"]
    assert findings[0].severity == Severity.WARNING


def test_no_baseline_section_returns_nothing(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace({"AGENTS.md": "# AGENTS\n\nNo baseline here.\n"})
    cfg = make_config(root)
    assert run(cfg) == []


def test_baseline_accepts_directory_form(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "AGENTS.md": "## Current Module Baseline\n- `src/bot/`\n",
            "crates/teamtalk/src/bot/mod.rs": "",
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []
