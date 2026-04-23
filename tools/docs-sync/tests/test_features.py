"""Tests for the features checker."""

from __future__ import annotations

from typing import TYPE_CHECKING

from docs_sync.checkers.features import run
from docs_sync.report import Severity

from .conftest import CARGO_TOML_DEFAULT

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path

    from docs_sync.config import Config


def test_unknown_feature_reported(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "docs/features.md": (
                'teamtalk = { version = "6.0.0", features = ["async", "fake"] }\n'
            ),
        },
    )
    cfg = make_config(root)
    findings = run(cfg)
    assert len(findings) == 1
    assert findings[0].severity == Severity.ERROR
    assert findings[0].rule == "unknown-feature"
    assert findings[0].actual == "fake"


def test_known_features_pass(
    make_workspace: Callable[[dict[str, str]], Path],
    make_config: Callable[[Path], Config],
) -> None:
    root = make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "README.md": ('teamtalk = { version = "6.0.0", features = ["async", "bot"] }\n'),
        },
    )
    cfg = make_config(root)
    assert run(cfg) == []
