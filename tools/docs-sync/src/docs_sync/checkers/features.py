"""Detect unknown feature names referenced in documentation snippets."""

from __future__ import annotations

import re
from pathlib import Path
from typing import TYPE_CHECKING

from docs_sync.discovery import iter_doc_files, read_lines
from docs_sync.manifest import load_manifest
from docs_sync.report import Finding, Severity

if TYPE_CHECKING:
    from docs_sync.config import Config

# Matches ``features = ["a", "b"]`` on a single line; good enough for
# Cargo-style doc snippets. Multi-line feature arrays aren't idiomatic for
# embedded TOML examples.
_FEATURES_RE = re.compile(r"features\s*=\s*\[(?P<list>[^\]]*)\]")
_ITEM_RE = re.compile(r'"([^"]+)"')


def run(config: Config) -> list[Finding]:
    """Return findings for doc snippets listing unknown feature names."""
    if not config.features.enabled:
        return []
    root = Path(config.sources.workspace_root).resolve()
    manifest = load_manifest(root / config.sources.cargo_manifest)
    known = manifest.features
    findings: list[Finding] = []
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        for line_no, line in enumerate(read_lines(root / rel), start=1):
            match = _FEATURES_RE.search(line)
            if match is None:
                continue
            items = _ITEM_RE.findall(match.group("list"))
            for item in items:
                if item in known:
                    continue
                findings.append(
                    Finding(
                        checker="features",
                        severity=Severity.ERROR,
                        file=str(rel),
                        line=line_no,
                        rule="unknown-feature",
                        message=(
                            f"feature {item!r} is not defined in {config.sources.cargo_manifest}"
                        ),
                        expected=", ".join(sorted(known)),
                        actual=item,
                    ),
                )
    return findings
