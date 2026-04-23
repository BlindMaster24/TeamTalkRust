"""Tests for the snippets checker."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.checkers.snippets import fix, run
from docs_sync.report import Severity

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

    from docs_sync.config import Config

_SRC = """\
fn outer() {
    // docs-sync:region setup
    let client = Client::new();
    client.connect();
    // docs-sync:endregion setup
}
"""


def _doc(body: str) -> str:
    return (
        "# Example\n"
        "<!-- docs-sync:begin src=examples/setup.rs region=setup -->\n"
        "```rust\n"
        f"{body}"
        "```\n"
        "<!-- docs-sync:end -->\n"
    )


def test_matching_snippet_is_ok(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "examples/setup.rs": _SRC,
            "docs/bot.md": _doc("let client = Client::new();\nclient.connect();\n"),
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []


def test_drift_flagged(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "examples/setup.rs": _SRC,
            "docs/bot.md": _doc("let client = OldClient::new();\n"),
        },
    )
    cfg = make_config(root)
    findings = run(cfg)
    assert len(findings) == 1
    assert findings[0].severity == Severity.ERROR
    assert findings[0].rule == "content-drift"


def test_fix_rewrites_drifted_snippet(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "examples/setup.rs": _SRC,
            "docs/bot.md": _doc("let client = OldClient::new();\n"),
        },
    )
    cfg = make_config(root)
    edits = fix(cfg)
    assert edits == 1
    assert run(cfg) == []
    updated = (root / "docs/bot.md").read_text(encoding="utf-8")
    assert "Client::new()" in updated
    assert "OldClient" not in updated


def test_missing_region_reported(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "examples/setup.rs": "fn outer() {}\n",
            "docs/bot.md": _doc("let x = 1;\n"),
        },
    )
    cfg = make_config(root)
    findings = run(cfg)
    assert len(findings) == 1
    assert findings[0].rule == "missing-region"
