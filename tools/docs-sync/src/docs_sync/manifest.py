"""Parse the TeamTalk Rust crate manifest for docs-sync."""

from __future__ import annotations

import tomllib
from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path


@dataclass(frozen=True)
class Manifest:
    """Selected fields from ``crates/teamtalk/Cargo.toml``."""

    version: str
    features: frozenset[str]


def load_manifest(path: Path) -> Manifest:
    """Load version and features from a Cargo manifest.

    Args:
        path: Path to ``Cargo.toml``.

    Returns:
        :class:`Manifest` with version string and feature name set.

    Raises:
        FileNotFoundError: If ``path`` does not exist.
        KeyError: If required fields are missing.
    """
    raw = tomllib.loads(path.read_text(encoding="utf-8"))
    package = raw.get("package", {})
    version = package.get("version")
    if not isinstance(version, str):
        msg = f"missing [package].version in {path}"
        raise KeyError(msg)
    features_section = raw.get("features", {}) or {}
    features = frozenset(features_section.keys())
    return Manifest(version=version, features=features)
