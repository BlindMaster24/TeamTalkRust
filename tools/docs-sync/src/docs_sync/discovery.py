"""File-discovery helpers shared by checkers."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Iterable
    from pathlib import Path


def iter_doc_files(
    root: Path,
    include: Iterable[str],
    exclude: Iterable[str],
) -> list[Path]:
    """Collect Markdown files matching ``include`` minus ``exclude``.

    Args:
        root: Repository root.
        include: Glob patterns to include (relative to ``root``).
        exclude: Glob patterns to exclude (relative to ``root``).

    Returns:
        Sorted list of repository-relative :class:`Path` instances.
    """
    included: set[Path] = set()
    for pattern in include:
        for match in root.glob(pattern):
            if match.is_file():
                included.add(match.resolve())
    excluded: set[Path] = set()
    for pattern in exclude:
        for match in root.glob(pattern):
            if match.is_file():
                excluded.add(match.resolve())
    result = sorted(included - excluded)
    return [p.relative_to(root.resolve()) for p in result]


def read_lines(path: Path) -> list[str]:
    """Read ``path`` as UTF-8 text and split into lines without newline chars.

    Args:
        path: Path to read.

    Returns:
        List of lines without line separators.
    """
    return path.read_text(encoding="utf-8").splitlines()
