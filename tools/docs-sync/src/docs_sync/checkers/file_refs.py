"""Detect in-doc file-path references pointing at missing files."""

from __future__ import annotations

import re
from pathlib import Path
from typing import TYPE_CHECKING

from docs_sync.discovery import iter_doc_files, read_lines
from docs_sync.report import Finding, Severity

if TYPE_CHECKING:
    from docs_sync.config import Config

_INLINE_CODE_RE = re.compile(r"`(?P<path>(?:[\w./-]+))`")
_ANGLE_LINK_RE = re.compile(r"<(?P<path>(?:[\w./-]+))>")


def _iter_candidates(line: str) -> list[tuple[int, str]]:
    """Extract candidate file-path strings from inline code and angle links.

    Returns a list of ``(column, path)`` tuples.
    """
    hits: list[tuple[int, str]] = [
        (match.start("path"), match.group("path")) for match in _INLINE_CODE_RE.finditer(line)
    ]
    hits.extend(
        (match.start("path"), match.group("path")) for match in _ANGLE_LINK_RE.finditer(line)
    )
    return hits


def _looks_like_filepath(candidate: str, prefixes: list[str]) -> bool:
    """Heuristic: starts with a known prefix and contains ``/``."""
    return (
        any(candidate.startswith(p) for p in prefixes)
        and "/" in candidate
        and not candidate.endswith("/")
    )


def _exists(root: Path, candidate: str) -> bool:
    """Check if ``candidate`` exists under ``root``.

    Strips a ``#anchor`` suffix before checking.
    """
    target = candidate.split("#", 1)[0]
    if not target:
        return False
    return (root / target).exists()


def run(config: Config) -> list[Finding]:
    """Return findings for inline-code paths that don't exist on disk."""
    if not config.file_refs.enabled:
        return []
    root = Path(config.sources.workspace_root).resolve()
    findings: list[Finding] = []
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        for line_no, line in enumerate(read_lines(root / rel), start=1):
            for _column, candidate in _iter_candidates(line):
                if not _looks_like_filepath(candidate, config.file_refs.prefixes):
                    continue
                if _exists(root, candidate):
                    continue
                findings.append(
                    Finding(
                        checker="file_refs",
                        severity=Severity.WARNING,
                        file=str(rel),
                        line=line_no,
                        rule="missing-path",
                        message=f"referenced path {candidate!r} does not exist",
                        expected=None,
                        actual=candidate,
                    ),
                )
    return findings
