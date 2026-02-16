#!/usr/bin/env python3
import argparse
import subprocess
from pathlib import Path


def run(step: str, cmd: list[str], cwd: Path) -> None:
    print(f"[{step}] {' '.join(cmd)}")
    proc = subprocess.run(cmd, cwd=str(cwd), capture_output=True, text=True)
    if proc.stdout:
        for line in proc.stdout.splitlines():
            if line.strip():
                print(f"[{step}] {line}")
    if proc.stderr:
        for line in proc.stderr.splitlines():
            if line.strip():
                print(f"[{step}][stderr] {line}")
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

    print("[audit] start")
    # Always refresh timestamp first.
    run(
        "1/4 plan timestamp (pre-scan)",
        ["python", str(plan_sync), "--root", str(root), "--mode", "update"],
        root,
    )
    # Full symbol scan from TeamTalk.h and docs.
    run("2/4 requirement scan", ["python", str(scan), "--root", str(root)], root)
    # Sync auto findings into plan.md.
    run("3/4 findings sync", ["python", str(sync_findings), "--root", str(root)], root)
    # Refresh timestamp after plan mutation to guarantee latest value.
    run(
        "4/4 plan timestamp (post-sync)",
        ["python", str(plan_sync), "--root", str(root), "--mode", "update"],
        root,
    )

    print("[audit] complete")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
