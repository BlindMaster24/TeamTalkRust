"""Detect and fix stale ``teamtalk`` crate version references in docs."""

from __future__ import annotations

import re
from pathlib import Path
from typing import TYPE_CHECKING

from docs_sync.discovery import iter_doc_files, read_lines
from docs_sync.manifest import load_manifest
from docs_sync.report import Finding, Severity

if TYPE_CHECKING:
    from docs_sync.config import Config

# Matches ``teamtalk = "X.Y.Z"`` or ``teamtalk = { version = "X.Y.Z", ...``.
_PATTERN = re.compile(
    r"""
    \bteamtalk\s*=\s*
    (?:
        "(?P<plain>[^"]+)"
      | \{\s*[^}]*? version\s*=\s*"(?P<nested>[^"]+)"
    )
    """,
    re.VERBOSE,
)


def _iter_matches(lines: list[str]) -> list[tuple[int, int, int, str]]:
    """Yield ``(line_no, span_start, span_end, actual_version)`` tuples.

    ``line_no`` is 1-based. Spans are relative to the line text.
    """
    hits: list[tuple[int, int, int, str]] = []
    for idx, line in enumerate(lines, start=1):
        for match in _PATTERN.finditer(line):
            actual = match.group("plain") or match.group("nested") or ""
            if not actual:
                continue
            group_name = "plain" if match.group("plain") else "nested"
            hits.append(
                (
                    idx,
                    match.start(group_name),
                    match.end(group_name),
                    actual,
                ),
            )
    return hits


def _root(config: Config) -> Path:
    return Path(config.sources.workspace_root).resolve()


def run(config: Config) -> list[Finding]:
    """Return findings for every stale ``teamtalk`` version reference."""
    if not config.versions.enabled:
        return []
    root = _root(config)
    manifest = load_manifest(root / config.sources.cargo_manifest)
    findings: list[Finding] = []
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        lines = read_lines(root / rel)
        for line_no, _start, _end, actual in _iter_matches(lines):
            if actual != manifest.version:
                findings.append(
                    Finding(
                        checker="versions",
                        severity=Severity.ERROR,
                        file=str(rel),
                        line=line_no,
                        rule="version-mismatch",
                        message=(
                            f"teamtalk version reference {actual!r} does not "
                            f"match Cargo.toml ({manifest.version!r})"
                        ),
                        expected=manifest.version,
                        actual=actual,
                    ),
                )
    return findings


def fix(config: Config) -> int:
    """Rewrite stale version references in-place, return count of edits.

    The function is idempotent: running twice on a clean tree edits nothing.
    """
    if not config.versions.enabled:
        return 0
    root = _root(config)
    manifest = load_manifest(root / config.sources.cargo_manifest)
    edits = 0
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        path = root / rel
        original = path.read_text(encoding="utf-8")
        lines = original.splitlines(keepends=True)
        changed = False
        for idx, line in enumerate(lines):
            # Rewrite right-to-left so earlier match offsets stay valid as
            # we mutate ``new_line``. Matches are computed once on the
            # original line and applied in reverse order.
            new_line = line
            for match in reversed(list(_PATTERN.finditer(line))):
                group_name = "plain" if match.group("plain") else "nested"
                actual = match.group(group_name)
                if actual == manifest.version:
                    continue
                start = match.start(group_name)
                end = match.end(group_name)
                new_line = new_line[:start] + manifest.version + new_line[end:]
                edits += 1
            if new_line != line:
                lines[idx] = new_line
                changed = True
        if changed:
            path.write_text("".join(lines), encoding="utf-8")
    return edits
