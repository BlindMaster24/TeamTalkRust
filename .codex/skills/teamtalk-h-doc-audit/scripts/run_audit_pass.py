#!/usr/bin/env python3
import argparse
import subprocess
from pathlib import Path


def run(cmd: list[str], cwd: Path) -> None:
    proc = subprocess.run(cmd, cwd=str(cwd))
    if proc.returncode != 0:
        raise SystemExit(proc.returncode)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    scripts = root / ".codex" / "skills" / "teamtalk-h-doc-audit" / "scripts"

    plan_sync = scripts / "plan_sync.py"
    scan = scripts / "scan_requirements.py"
    sync_findings = scripts / "sync_plan_findings.py"

    for required in (plan_sync, scan, sync_findings):
        if not required.exists():
            raise SystemExit(f"missing script: {required}")

    # Always refresh timestamp first.
    run(["python", str(plan_sync), "--root", str(root), "--mode", "update"], root)
    # Full symbol scan from TeamTalk.h and docs.
    run(["python", str(scan), "--root", str(root)], root)
    # Sync auto findings into plan.md.
    run(["python", str(sync_findings), "--root", str(root)], root)
    # Refresh timestamp after plan mutation to guarantee latest value.
    run(["python", str(plan_sync), "--root", str(root), "--mode", "update"], root)

    print("audit pass complete")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
