#!/usr/bin/env python3
import argparse
from datetime import datetime, timezone
from pathlib import Path

TEMPLATE = """# TeamTalk Audit Plan

Last updated: {ts}

## Current focus
- Audit TeamTalk.h comments and C-API docs against Rust wrappers.

## Work queue
- [ ] Extract new/unchecked requirements from TeamTalk.h.
- [ ] Cross-check C-API docs for the same area.
- [ ] Map to wrappers in crates/teamtalk/src.
- [ ] Validate tests in crates/teamtalk/tests.
- [ ] Implement minimal fixes.
- [ ] Update docs/changelog if user-visible.

## Findings

### Open
- None yet.

### Closed
- None yet.

## Next invocation instructions
1. Re-open this file first.
2. Continue from first unchecked queue item.
3. Append new findings under Open/Closed.
4. Run scan script and required cargo checks after edits.
"""


def now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%SZ")


def update_timestamp(text: str) -> str:
    lines = text.splitlines()
    out = []
    replaced = False
    for line in lines:
        if line.startswith("Last updated:"):
            out.append(f"Last updated: {now_iso()}")
            replaced = True
        else:
            out.append(line)
    if not replaced:
        out.insert(1, f"Last updated: {now_iso()}")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    ap.add_argument("--mode", choices=["start", "update"], default="start")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    plan = root / "plan.md"

    if args.mode == "start":
        if plan.exists():
            plan.write_text(update_timestamp(plan.read_text(encoding="utf-8")), encoding="utf-8")
            print(f"updated {plan}")
        else:
            plan.write_text(TEMPLATE.format(ts=now_iso()), encoding="utf-8")
            print(f"created {plan}")
    else:
        if not plan.exists():
            plan.write_text(TEMPLATE.format(ts=now_iso()), encoding="utf-8")
            print(f"created {plan}")
        else:
            plan.write_text(update_timestamp(plan.read_text(encoding="utf-8")), encoding="utf-8")
            print(f"updated {plan}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
