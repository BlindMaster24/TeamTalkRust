#!/usr/bin/env python3
import argparse
import re
import subprocess
from pathlib import Path
from datetime import datetime, timezone

SEARCH_TARGETS = [
    "TEAMTALK_DLL/Documentation/C-API",
    "crates/teamtalk-sys",
    "crates/teamtalk/src",
    "crates/teamtalk/tests",
    "docs",
    "README.md",
]

MAX_LINES_PER_TARGET = 8


def run_rg(root: Path, pattern: str, rel: str) -> list[str]:
    target = root / rel
    if not target.exists():
        return []
    cmd = ["rg", "-n", "-F", pattern, str(target)]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode not in (0, 1):
        return []
    return [line for line in proc.stdout.splitlines() if line.strip()]


def strip_c_comments(text: str) -> str:
    # Remove block comments first, then line comments.
    no_block = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//.*", "", no_block)


def extract_tt_symbols(header_text: str) -> list[str]:
    code_only = strip_c_comments(header_text)
    symbols = set(re.findall(r"\bTT_[A-Za-z0-9_]+\b", code_only))
    return sorted(symbols)


def now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%SZ")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    header = root / "TEAMTALK_DLL" / "TeamTalk.h"
    out = root / "plan_requirements_scan.md"

    if not header.exists():
        out.write_text("# Requirement Scan\n\nTeamTalk.h not found.\n", encoding="utf-8")
        print(f"wrote {out}")
        return 0

    header_text = header.read_text(encoding="utf-8", errors="replace")
    symbols = extract_tt_symbols(header_text)

    lines: list[str] = []
    lines.append("# Requirement Scan")
    lines.append("")
    lines.append(f"Generated: {now_iso()}")
    lines.append(f"TeamTalk symbols discovered: {len(symbols)}")
    lines.append("")

    uncovered_wrapper = []
    uncovered_tests = []

    for sym in symbols:
        lines.append(f"## `{sym}`")
        hit_map: dict[str, list[str]] = {}
        for target in SEARCH_TARGETS:
            hits = run_rg(root, sym, target)
            hit_map[target] = hits
            lines.append(f"### {target}")
            if hits:
                lines.append("```text")
                lines.extend(hits[:MAX_LINES_PER_TARGET])
                if len(hits) > MAX_LINES_PER_TARGET:
                    lines.append(f"... ({len(hits) - MAX_LINES_PER_TARGET} more)")
                lines.append("```")
            else:
                lines.append("No matches")
            lines.append("")

        if not hit_map["crates/teamtalk/src"] and not hit_map["crates/teamtalk-sys"]:
            uncovered_wrapper.append(sym)
        if not hit_map["crates/teamtalk/tests"]:
            uncovered_tests.append(sym)

    lines.append("## Coverage Summary")
    lines.append("")
    lines.append(f"- Symbols without wrapper/sys mapping: {len(uncovered_wrapper)}")
    if uncovered_wrapper:
        lines.append("- List: " + ", ".join(uncovered_wrapper[:120]))
    lines.append(f"- Symbols without direct tests reference: {len(uncovered_tests)}")
    if uncovered_tests:
        lines.append("- List: " + ", ".join(uncovered_tests[:120]))
    lines.append("")
    lines.append("## Next Actions")
    lines.append("- Convert uncovered symbols into plan.md findings with disposition.")
    lines.append("- Implement wrappers/tests/docs or mark explicit defer reasons.")

    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
