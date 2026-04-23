"""docs-sync: detect drift between Rust source code and Markdown docs.

The public API is intentionally small:

* :class:`docs_sync.report.Finding` — a single drift observation.
* :class:`docs_sync.report.Severity` — severity level.
* :class:`docs_sync.report.Report` — aggregated run output.
* :func:`docs_sync.cli.main` — CLI entry point.
"""

from __future__ import annotations

from .report import Finding, Report, Severity

__all__ = ["Finding", "Report", "Severity"]
__version__ = "0.1.0"
