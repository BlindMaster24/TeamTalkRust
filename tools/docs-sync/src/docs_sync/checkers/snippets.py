"""Snippet-embedding sync.

Keeps fenced code blocks in Markdown in sync with named regions in source.

Markers in docs (HTML comments are invisible in rendering)::

    <!-- docs-sync:begin src=crates/teamtalk/examples/foo.rs region=setup -->
    ```rust
    let x = 1;
    ```
    <!-- docs-sync:end -->

Markers in source (single-line comments, any supported language)::

    // docs-sync:region setup
    let x = 1;
    // docs-sync:endregion setup
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

from docs_sync.discovery import iter_doc_files
from docs_sync.report import Finding, Severity

if TYPE_CHECKING:
    from docs_sync.config import Config

_BEGIN_RE = re.compile(
    r"<!--\s*docs-sync:begin\s+src=(?P<src>\S+)\s+region=(?P<region>\S+?)\s*-->",
)
_END_RE = re.compile(r"<!--\s*docs-sync:end\s*-->")
_FENCE_OPEN_RE = re.compile(r"^(?P<indent>\s*)```(?P<info>.*)$")
_FENCE_CLOSE_RE = re.compile(r"^(?P<indent>\s*)```\s*$")


@dataclass(frozen=True)
class _Block:
    """A single ``docs-sync:begin``/``end`` block within a doc file."""

    begin_line: int  # 1-based line of the begin marker
    end_line: int  # 1-based line of the end marker
    src: str
    region: str
    fence_start: int  # 1-based line of opening fence (0 = missing)
    fence_end: int  # 1-based line of closing fence (0 = missing)
    fence_info: str  # language tag from the opening fence
    body: list[str]  # body lines between the fences


def _find_blocks(lines: list[str]) -> list[_Block]:
    """Parse ``docs-sync:begin`` / ``docs-sync:end`` blocks from ``lines``."""
    blocks: list[_Block] = []
    i = 0
    total = len(lines)
    while i < total:
        begin = _BEGIN_RE.search(lines[i])
        if begin is None:
            i += 1
            continue
        begin_line = i + 1
        src = begin.group("src")
        region = begin.group("region")
        j = i + 1
        fence_start = 0
        fence_end = 0
        fence_info = ""
        body: list[str] = []
        while j < total:
            if _END_RE.search(lines[j]):
                end_line = j + 1
                blocks.append(
                    _Block(
                        begin_line=begin_line,
                        end_line=end_line,
                        src=src,
                        region=region,
                        fence_start=fence_start,
                        fence_end=fence_end,
                        fence_info=fence_info,
                        body=body,
                    ),
                )
                i = j + 1
                break
            if fence_start == 0:
                match = _FENCE_OPEN_RE.match(lines[j])
                if match is not None and match.group("info").strip():
                    fence_start = j + 1
                    fence_info = match.group("info")
                    j += 1
                    continue
            elif fence_end == 0:
                if _FENCE_CLOSE_RE.match(lines[j]) is not None:
                    fence_end = j + 1
                else:
                    body.append(lines[j])
            j += 1
        else:
            # no end marker found
            i += 1
    return blocks


_REGION_BEGIN_RE = re.compile(
    r"(?:^|\s)docs-sync:region\s+(?P<region>\S+)",
)
_REGION_END_RE = re.compile(
    r"(?:^|\s)docs-sync:endregion\s+(?P<region>\S+)",
)


def _extract_region(src: Path, region: str) -> list[str] | None:
    """Extract the body of a named region from a source file.

    Returns ``None`` if the file or region is missing. Leading common
    indentation is stripped for clean embedding.
    """
    if not src.exists():
        return None
    lines = src.read_text(encoding="utf-8").splitlines()
    start = -1
    end = -1
    for idx, line in enumerate(lines):
        begin = _REGION_BEGIN_RE.search(line)
        if begin is not None and begin.group("region") == region:
            start = idx + 1
            continue
        if start != -1:
            end_match = _REGION_END_RE.search(line)
            if end_match is not None and end_match.group("region") == region:
                end = idx
                break
    if start == -1 or end == -1:
        return None
    body = lines[start:end]
    # Strip common leading whitespace
    non_blank = [line for line in body if line.strip()]
    if non_blank:
        common = min(len(line) - len(line.lstrip(" ")) for line in non_blank)
        if common > 0:
            body = [line[common:] if len(line) >= common else line for line in body]
    return body


def _root(config: Config) -> Path:
    return Path(config.sources.workspace_root).resolve()


def run(config: Config) -> list[Finding]:
    """Return findings for docs whose snippet bodies drifted from source."""
    if not config.snippets.enabled:
        return []
    root = _root(config)
    findings: list[Finding] = []
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        lines = (root / rel).read_text(encoding="utf-8").splitlines()
        for block in _find_blocks(lines):
            expected = _extract_region(root / block.src, block.region)
            if expected is None:
                findings.append(
                    Finding(
                        checker="snippets",
                        severity=Severity.ERROR,
                        file=str(rel),
                        line=block.begin_line,
                        rule="missing-region",
                        message=(f"source region {block.region!r} not found in {block.src!r}"),
                        expected=None,
                        actual=f"{block.src}#{block.region}",
                    ),
                )
                continue
            if block.fence_start == 0 or block.fence_end == 0:
                findings.append(
                    Finding(
                        checker="snippets",
                        severity=Severity.ERROR,
                        file=str(rel),
                        line=block.begin_line,
                        rule="missing-fence",
                        message=("docs-sync block is missing a fenced code block"),
                        expected=None,
                        actual=None,
                    ),
                )
                continue
            if block.body != expected:
                findings.append(
                    Finding(
                        checker="snippets",
                        severity=Severity.ERROR,
                        file=str(rel),
                        line=block.fence_start,
                        rule="content-drift",
                        message=(
                            f"snippet body drifted from {block.src}#"
                            f"{block.region} (run docs-sync --fix)"
                        ),
                        expected="\n".join(expected),
                        actual="\n".join(block.body),
                    ),
                )
    return findings


def fix(config: Config) -> int:
    """Rewrite drifted snippet bodies in-place. Returns count of edits."""
    if not config.snippets.enabled:
        return 0
    root = _root(config)
    edits = 0
    for rel in iter_doc_files(root, config.docs.include, config.docs.exclude):
        path = root / rel
        text = path.read_text(encoding="utf-8")
        # Preserve trailing newline separately for clean re-joining.
        trailing = "\n" if text.endswith("\n") else ""
        lines = text.splitlines()
        blocks = _find_blocks(lines)
        if not blocks:
            continue
        # Apply in reverse so earlier indices stay valid.
        changed = False
        for block in reversed(blocks):
            if block.fence_start == 0 or block.fence_end == 0:
                continue
            expected = _extract_region(root / block.src, block.region)
            if expected is None or block.body == expected:
                continue
            # Replace the inclusive body slice between the two fences.
            body_start = block.fence_start  # 0-based line after opening fence
            body_end = block.fence_end - 1  # 0-based line of closing fence
            lines = lines[:body_start] + expected + lines[body_end:]
            changed = True
            edits += 1
        if changed:
            path.write_text("\n".join(lines) + trailing, encoding="utf-8")
    return edits
