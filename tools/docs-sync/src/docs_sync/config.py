"""Configuration loading for docs-sync."""

from __future__ import annotations

import tomllib
from dataclasses import dataclass, field
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path


@dataclass
class SourcesConfig:
    """Filesystem sources for the checkers."""

    cargo_manifest: str = "crates/teamtalk/Cargo.toml"
    workspace_root: str = "."


@dataclass
class DocsConfig:
    """Documentation file globs."""

    include: list[str] = field(
        default_factory=lambda: ["docs/**/*.md", "README.md", "AGENTS.md"],
    )
    exclude: list[str] = field(
        default_factory=lambda: [
            "docs/changelog.md",
            "docs/migrations/**/*.md",
        ],
    )


@dataclass
class VersionsConfig:
    """Version-drift detection settings."""

    enabled: bool = True


@dataclass
class FeaturesConfig:
    """Feature-drift detection settings."""

    enabled: bool = True


@dataclass
class ModuleTreeConfig:
    """Module-tree freshness settings.

    Compares the paths mentioned in ``AGENTS.md`` Current Module Baseline
    section against the actual directory tree.
    """

    enabled: bool = True
    agents_md: str = "AGENTS.md"
    baseline_heading: str = "Current Module Baseline"
    source_roots: list[str] = field(
        default_factory=lambda: [
            "crates/teamtalk/src",
            "crates/teamtalk-sys/src",
            "crates/teamtalk-macros/src",
        ],
    )


@dataclass
class FileRefsConfig:
    """Detects in-doc file-path references pointing at missing files."""

    enabled: bool = True
    prefixes: list[str] = field(
        default_factory=lambda: [
            "crates/",
            "docs/",
            "scripts/",
            "examples/",
            "tools/",
        ],
    )


@dataclass
class SnippetsConfig:
    """Snippet-embedding sync settings.

    Markers in docs (HTML comments, invisible in rendering)::

        <!-- docs-sync:begin src=crates/teamtalk/examples/foo.rs region=setup -->
        ```rust
        let x = 1;
        ```
        <!-- docs-sync:end -->

    Markers in source files::

        // docs-sync:region setup
        let x = 1;
        // docs-sync:endregion setup
    """

    enabled: bool = True


@dataclass
class Config:
    """Top-level docs-sync configuration."""

    sources: SourcesConfig = field(default_factory=SourcesConfig)
    docs: DocsConfig = field(default_factory=DocsConfig)
    versions: VersionsConfig = field(default_factory=VersionsConfig)
    features: FeaturesConfig = field(default_factory=FeaturesConfig)
    module_tree: ModuleTreeConfig = field(default_factory=ModuleTreeConfig)
    file_refs: FileRefsConfig = field(default_factory=FileRefsConfig)
    snippets: SnippetsConfig = field(default_factory=SnippetsConfig)


def load(path: Path | None) -> Config:
    """Load a :class:`Config` from ``path`` or return defaults.

    Args:
        path: Path to a ``docs_sync.toml`` file, or ``None``.

    Returns:
        A :class:`Config` built from the file when present, otherwise the
        built-in defaults. A missing file is not an error.
    """
    if path is None or not path.exists():
        return Config()
    raw = tomllib.loads(path.read_text(encoding="utf-8"))
    cfg = Config()
    if "sources" in raw:
        section = raw["sources"]
        cfg.sources = SourcesConfig(
            cargo_manifest=section.get(
                "cargo_manifest",
                cfg.sources.cargo_manifest,
            ),
            workspace_root=section.get(
                "workspace_root",
                cfg.sources.workspace_root,
            ),
        )
    if "docs" in raw:
        section = raw["docs"]
        cfg.docs = DocsConfig(
            include=list(section.get("include", cfg.docs.include)),
            exclude=list(section.get("exclude", cfg.docs.exclude)),
        )
    if "versions" in raw:
        cfg.versions = VersionsConfig(
            enabled=bool(raw["versions"].get("enabled", True)),
        )
    if "features" in raw:
        cfg.features = FeaturesConfig(
            enabled=bool(raw["features"].get("enabled", True)),
        )
    if "module_tree" in raw:
        section = raw["module_tree"]
        cfg.module_tree = ModuleTreeConfig(
            enabled=bool(section.get("enabled", True)),
            agents_md=section.get("agents_md", cfg.module_tree.agents_md),
            baseline_heading=section.get(
                "baseline_heading",
                cfg.module_tree.baseline_heading,
            ),
            source_roots=list(
                section.get("source_roots", cfg.module_tree.source_roots),
            ),
        )
    if "file_refs" in raw:
        section = raw["file_refs"]
        cfg.file_refs = FileRefsConfig(
            enabled=bool(section.get("enabled", True)),
            prefixes=list(section.get("prefixes", cfg.file_refs.prefixes)),
        )
    if "snippets" in raw:
        section = raw["snippets"]
        cfg.snippets = SnippetsConfig(
            enabled=bool(section.get("enabled", True)),
        )
    return cfg
