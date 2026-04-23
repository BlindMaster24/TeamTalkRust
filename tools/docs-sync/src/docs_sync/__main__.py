"""Run docs-sync as a module: ``python -m docs_sync``."""

from __future__ import annotations

from .cli import main

if __name__ == "__main__":
    raise SystemExit(main())
