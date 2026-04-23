"""Ensure the ``AGENTS.md`` module baseline matches the actual source tree."""

from __future__ import annotations

import re
from pathlib import Path
from typing import TYPE_CHECKING

from docs_sync.report import Finding, Severity

if TYPE_CHECKING:
    from docs_sync.config import Config

# Matches inline code spans inside the baseline section, e.g.
# ``src/client/connection/`` or ``src/bot/fsm.rs``.
_INLINE_PATH_RE = re.compile(r"`(?P<path>(?:src|crates)/[^`\s]+)`")


def _extract_baseline_paths(
    agents_md: Path,
    baseline_heading: str,
) -> set[str]:
    """Parse baseline paths from the AGENTS.md section until next ``## ``."""
    if not agents_md.exists():
        return set()
    lines = agents_md.read_text(encoding="utf-8").splitlines()
    in_section = False
    paths: set[str] = set()
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("## "):
            heading = stripped.removeprefix("## ").strip()
            in_section = heading.lower() == baseline_heading.lower()
            continue
        if not in_section:
            continue
        for match in _INLINE_PATH_RE.finditer(line):
            paths.add(match.group("path"))
    return paths


def _collect_actual_paths(root: Path, source_roots: list[str]) -> set[str]:
    """Collect every ``.rs`` file and its parent directories as a flat set."""
    results: set[str] = set()
    for source_root in source_roots:
        base = root / source_root
        if not base.exists():
            continue
        for path in base.rglob("*.rs"):
            rel = path.relative_to(root).as_posix()
            results.add(rel)
            for parent in path.relative_to(root).parents:
                parent_str = parent.as_posix()
                if parent_str in {".", ""}:
                    break
                results.add(parent_str + "/")
    return results


def _normalise(path: str) -> str:
    """Normalise a documented path to a comparable form."""
    # AGENTS.md uses workspace-relative paths like ``src/client`` without
    # leading ``crates/teamtalk/``. We accept both.
    if path.startswith("src/"):
        return "crates/teamtalk/" + path
    return path


def run(config: Config) -> list[Finding]:
    """Return findings for baseline paths missing from the real tree."""
    if not config.module_tree.enabled:
        return []
    root = Path(config.sources.workspace_root).resolve()
    agents_md = root / config.module_tree.agents_md
    documented = _extract_baseline_paths(
        agents_md,
        config.module_tree.baseline_heading,
    )
    if not documented:
        return []
    actual = _collect_actual_paths(root, config.module_tree.source_roots)
    findings: list[Finding] = []
    for path in sorted(documented):
        normalised = _normalise(path)
        trailing_variant = normalised.rstrip("/") + "/"
        if normalised in actual or trailing_variant in actual:
            continue
        findings.append(
            Finding(
                checker="module_tree",
                severity=Severity.WARNING,
                file=config.module_tree.agents_md,
                rule="missing-from-tree",
                message=(
                    f"baseline path {path!r} not found under {config.module_tree.source_roots}"
                ),
                expected=None,
                actual=path,
            ),
        )
    return findings
