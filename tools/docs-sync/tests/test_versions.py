"""Tests for the versions checker."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.checkers.versions import fix, run
from docs_sync.report import Severity

from .conftest import CARGO_TOML_DEFAULT

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

    from docs_sync.config import Config


def _prepare(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
    doc_body: str,
) -> tuple[Path, Config]:
    root = make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "docs/bot.md": doc_body,
        },
    )
    cfg = make_config(root)
    return root, cfg


def test_no_findings_when_versions_match(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    _, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = { version = "6.0.0", features = ["bot"] }\n',
    )
    assert run(cfg) == []


def test_plain_version_drift_reported(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    _, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = "4.0.0"\n',
    )
    [finding] = run(cfg)
    assert finding.severity == Severity.ERROR
    assert finding.rule == "version-mismatch"
    assert finding.expected == "6.0.0"
    assert finding.actual == "4.0.0"


def test_nested_version_drift_reported(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    _, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = { version = "4.0.0", features = ["bot"] }\n',
    )
    [finding] = run(cfg)
    assert finding.actual == "4.0.0"
    assert finding.file == "docs/bot.md"


def test_fix_updates_in_place(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = { version = "4.0.0", features = ["bot"] }\n',
    )
    edits = fix(cfg)
    assert edits == 1
    after = (root / "docs/bot.md").read_text(encoding="utf-8")
    assert '"6.0.0"' in after
    assert run(cfg) == []


def test_fix_is_idempotent(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    _, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = { version = "6.0.0", features = ["bot"] }\n',
    )
    assert fix(cfg) == 0


def test_exclude_honors_migration_docs(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "docs/migrations/2-to-3.md": 'teamtalk = "3.0.0"\n',
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []


def test_literal_placeholder_is_ignored(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    """Regression: documentation using ``"X.Y.Z"`` as a literal placeholder
    must not be rewritten. Only concrete numeric semver strings are
    considered.
    """
    root, cfg = _prepare(
        make_workspace,
        make_config,
        'prose using `teamtalk = "X.Y.Z"` as a placeholder\n'
        'and `teamtalk = { version = "X.Y.Z", ... }` too\n',
    )
    assert run(cfg) == []
    assert fix(cfg) == 0
    after = (root / "docs/bot.md").read_text(encoding="utf-8")
    assert "X.Y.Z" in after


def test_fix_handles_multiple_matches_on_one_line(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    """Regression: two matches on the same line must not corrupt output.

    Earlier the offsets were computed on the original line but applied to
    the progressively-mutated ``new_line``, so the second rewrite on a
    line with a length-changing replacement would land in the wrong
    place. Rewriting right-to-left keeps every match offset valid.
    """
    root, cfg = _prepare(
        make_workspace,
        make_config,
        'teamtalk = "4.0.0" and teamtalk = "4.0.0"\n',
    )
    edits = fix(cfg)
    assert edits == 2
    after = (root / "docs/bot.md").read_text(encoding="utf-8")
    assert after == 'teamtalk = "6.0.0" and teamtalk = "6.0.0"\n'
    assert run(cfg) == []
