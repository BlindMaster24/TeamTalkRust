"""Command-line entry point for docs-sync (Typer, plain text, no colors).

The CLI is intentionally screen-reader friendly: all output is plain ASCII,
no emoji, no ANSI colour codes, no Rich markup, no Unicode box drawing.
Typer's Rich integration is disabled entirely via ``rich_markup_mode=None``
and ``pretty_exceptions_enable=False``.
"""

from __future__ import annotations

import os
import sys
from pathlib import Path
from typing import Annotated

import click
import typer

from . import config as config_mod
from .checkers import ALL_CHECKERS, ALL_FIXERS
from .discovery import iter_doc_files
from .report import Report

_ALL_CHECKER_NAMES: list[str] = sorted(ALL_CHECKERS)
_FORMATS: tuple[str, ...] = ("txt", "md", "json")


def _new_app() -> typer.Typer:
    """Create the Typer app with all colour/formatting disabled."""
    # Force NO_COLOR before any Typer/Click output is emitted so even the
    # built-in help/usage messages stay plain.
    os.environ.setdefault("NO_COLOR", "1")
    os.environ.setdefault("TERM", "dumb")
    return typer.Typer(
        name="docs-sync",
        add_completion=False,
        no_args_is_help=False,
        rich_markup_mode=None,
        pretty_exceptions_enable=False,
        help=(
            "Detect (and optionally fix) drift between TeamTalkRust source "
            "code and external Markdown documentation. All output is plain "
            "text; no colours, no emoji, no Rich formatting."
        ),
    )


app = _new_app()


def _parse_formats(value: str) -> list[str]:
    """Parse a ``--format`` value into a list of known format names."""
    raw = [item.strip() for item in value.replace(",", " ").split() if item.strip()]
    if not raw:
        return ["txt"]
    if "all" in raw:
        return list(_FORMATS)
    unknown = [item for item in raw if item not in _FORMATS]
    if unknown:
        msg = f"unknown format(s): {', '.join(unknown)}"
        raise typer.BadParameter(msg)
    # preserve user order but de-dup
    seen: set[str] = set()
    result: list[str] = []
    for item in raw:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result


def _run_checkers(
    cfg: config_mod.Config,
    checker_names: list[str] | None,
) -> Report:
    """Run the configured checkers and return the aggregated report."""
    names = checker_names or _ALL_CHECKER_NAMES
    report = Report()
    for name in names:
        report.extend(ALL_CHECKERS[name](cfg))
    report.scanned_files = len(
        iter_doc_files(
            Path(cfg.sources.workspace_root).resolve(),
            cfg.docs.include,
            cfg.docs.exclude,
        ),
    )
    return report


def _run_fixers(
    cfg: config_mod.Config,
    checker_names: list[str] | None,
) -> int:
    """Run the fixers for the requested checkers and return total edits."""
    names = checker_names or list(ALL_FIXERS)
    total = 0
    for name in names:
        fixer = ALL_FIXERS.get(name)
        if fixer is None:
            continue
        total += fixer(cfg)
    return total


def _load_config(root: Path, config_path: Path) -> config_mod.Config:
    cfg_path = config_path if config_path.is_absolute() else root / config_path
    cfg = config_mod.load(cfg_path)
    cfg.sources.workspace_root = str(root)
    return cfg


@app.command("check")
def cmd_check(
    root: Annotated[
        Path | None,
        typer.Option(
            "--root",
            help="Repository root. Defaults to the current directory.",
        ),
    ] = None,
    config: Annotated[
        Path,
        typer.Option(
            "--config",
            help="Path to docs_sync.toml (relative paths resolve to --root).",
        ),
    ] = Path("scripts/docs_sync.toml"),
    checker: Annotated[
        list[str] | None,
        typer.Option(
            "--checker",
            help=(
                "Restrict the run to these checkers. Repeat to allow "
                "multiple. Default: all checkers enabled in the config."
            ),
        ),
    ] = None,
    fmt: Annotated[
        str,
        typer.Option(
            "--format",
            help=(
                "Report formats: any of txt, md, json, or all. Accepts a "
                "comma- or space-separated list."
            ),
        ),
    ] = "txt",
    report: Annotated[
        Path | None,
        typer.Option(
            "--report",
            help=(
                "Write reports to <report>.{txt,md,json}. When unset, the "
                "text report is printed to stdout."
            ),
        ),
    ] = None,
    warn_only: Annotated[
        bool,
        typer.Option(
            "--warn-only/--strict",
            help=("Exit 0 regardless of findings. Useful while migrating to a strict CI gate."),
        ),
    ] = False,
) -> None:
    """Detect drift and exit non-zero when any error-severity finding exists."""
    if checker is not None:
        for name in checker:
            if name not in _ALL_CHECKER_NAMES:
                msg = f"unknown checker {name!r}; known: {_ALL_CHECKER_NAMES}"
                raise typer.BadParameter(msg)
    formats = _parse_formats(fmt)
    resolved_root = (root or Path.cwd()).resolve()
    cfg = _load_config(resolved_root, config)
    run_report = _run_checkers(cfg, checker)
    if report is not None:
        run_report.write_reports(report, formats)
    if report is None or "txt" in formats:
        sys.stdout.write(run_report.to_text())
    if warn_only:
        raise typer.Exit(code=0)
    raise typer.Exit(code=1 if run_report.errors() else 0)


@app.command("fix")
def cmd_fix(
    root: Annotated[
        Path | None,
        typer.Option(
            "--root",
            help="Repository root. Defaults to the current directory.",
        ),
    ] = None,
    config: Annotated[
        Path,
        typer.Option(
            "--config",
            help="Path to docs_sync.toml (relative paths resolve to --root).",
        ),
    ] = Path("scripts/docs_sync.toml"),
    checker: Annotated[
        list[str] | None,
        typer.Option(
            "--checker",
            help=("Restrict the run to these fixers. Repeat to allow multiple."),
        ),
    ] = None,
) -> None:
    """Apply automatic fixes for supported checkers (versions, snippets)."""
    if checker is not None:
        for name in checker:
            if name not in _ALL_CHECKER_NAMES:
                msg = f"unknown checker {name!r}; known: {_ALL_CHECKER_NAMES}"
                raise typer.BadParameter(msg)
    resolved_root = (root or Path.cwd()).resolve()
    cfg = _load_config(resolved_root, config)
    edits = _run_fixers(cfg, checker)
    sys.stdout.write(f"docs-sync: applied {edits} fix(es)\n")


def main(argv: list[str] | None = None) -> int:
    """Programmatic entry point used by ``[project.scripts]``.

    Returns the process exit code. The function never raises ``SystemExit``
    so it can be called from tests without an ``except`` guard.
    """
    try:
        result = app(args=argv, standalone_mode=False)
    except click.exceptions.UsageError as exc:
        sys.stderr.write(f"error: {exc.format_message()}\n")
        return exc.exit_code
    except click.exceptions.ClickException as exc:
        sys.stderr.write(f"error: {exc.format_message()}\n")
        return exc.exit_code
    except typer.Exit as exc:
        return int(exc.exit_code)
    except SystemExit as exc:  # pragma: no cover - defensive
        code = exc.code if isinstance(exc.code, int) else 1
        return int(code)
    if isinstance(result, int):
        return result
    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
