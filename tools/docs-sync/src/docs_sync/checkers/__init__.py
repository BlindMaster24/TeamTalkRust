"""Individual docs-sync checkers."""

from __future__ import annotations

from collections.abc import Callable

from docs_sync.config import Config
from docs_sync.report import Finding

from . import features, file_refs, module_tree, snippets, versions

CheckerFn = Callable[[Config], list[Finding]]

ALL_CHECKERS: dict[str, CheckerFn] = {
    "versions": versions.run,
    "features": features.run,
    "module_tree": module_tree.run,
    "file_refs": file_refs.run,
    "snippets": snippets.run,
}

FixerFn = Callable[[Config], int]

ALL_FIXERS: dict[str, FixerFn] = {
    "versions": versions.fix,
    "snippets": snippets.fix,
}

__all__ = ["ALL_CHECKERS", "ALL_FIXERS", "CheckerFn", "FixerFn"]
