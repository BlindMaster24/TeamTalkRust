"""End-to-end tests for the Typer CLI."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from docs_sync.cli import main

from .conftest import CARGO_TOML_DEFAULT

if TYPE_CHECKING:
    from collections.abc import Callable
    from pathlib import Path


@pytest.fixture
def ready_workspace(
    make_workspace: Callable[[dict[str, str]], Path],
) -> Path:
    return make_workspace(
        {
            "crates/teamtalk/Cargo.toml": CARGO_TOML_DEFAULT,
            "scripts/docs_sync.toml": "",
            "docs/bot.md": 'teamtalk = "4.0.0"\n',
        },
    )


def test_check_returns_non_zero_on_drift(
    ready_workspace: Path,
    capsys: pytest.CaptureFixture[str],
) -> None:
    exit_code = main(
        [
            "check",
            "--root",
            str(ready_workspace),
        ],
    )
    captured = capsys.readouterr()
    assert exit_code == 1
    assert "version-mismatch" in captured.out
    assert captured.out.isascii()


def test_warn_only_returns_zero(ready_workspace: Path) -> None:
    exit_code = main(
        [
            "check",
            "--root",
            str(ready_workspace),
            "--warn-only",
        ],
    )
    assert exit_code == 0


def test_fix_then_check_is_clean(ready_workspace: Path) -> None:
    assert (
        main(
            [
                "fix",
                "--root",
                str(ready_workspace),
            ],
        )
        == 0
    )
    assert (
        main(
            [
                "check",
                "--root",
                str(ready_workspace),
            ],
        )
        == 0
    )


def test_format_all_writes_all_files(
    ready_workspace: Path,
    tmp_path: Path,
) -> None:
    report_base = tmp_path / "report"
    main(
        [
            "check",
            "--root",
            str(ready_workspace),
            "--format",
            "all",
            "--report",
            str(report_base),
            "--warn-only",
        ],
    )
    assert report_base.with_suffix(".txt").exists()
    assert report_base.with_suffix(".md").exists()
    assert report_base.with_suffix(".json").exists()


def test_unknown_format_rejected(ready_workspace: Path) -> None:
    exit_code = main(
        [
            "check",
            "--root",
            str(ready_workspace),
            "--format",
            "html",
        ],
    )
    # Typer returns a non-zero exit code for BadParameter.
    assert exit_code != 0
