#!/usr/bin/env python3
import argparse
import re
import subprocess
from pathlib import Path
from datetime import datetime, timezone
from collections import defaultdict

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
    # Audit callable API surface only (functions), not constants/macros.
    symbols = set(re.findall(r"\b(TT_[A-Za-z0-9_]+)\s*\(", code_only))
    return sorted(symbols)


def build_symbol_wrapper_map(root: Path) -> dict[str, set[str]]:
    src_root = root / "crates" / "teamtalk" / "src"
    mapping: dict[str, set[str]] = defaultdict(set)
    if not src_root.exists():
        return {}

    fn_re = re.compile(r"\bpub(?:\([^)]*\))?\s+fn\s+([A-Za-z0-9_]+)\s*\(")
    sym_re = re.compile(r"\bTT_[A-Za-z0-9_]+\b")

    for rs in src_root.rglob("*.rs"):
        text = rs.read_text(encoding="utf-8", errors="replace")
        lines = text.splitlines()
        for i, line in enumerate(lines):
            symbols = sym_re.findall(line)
            if not symbols:
                continue
            fn_name = None
            for j in range(i, max(-1, i - 80), -1):
                m = fn_re.search(lines[j])
                if m:
                    fn_name = m.group(1)
                    break
            if fn_name:
                for sym in symbols:
                    mapping[sym].add(fn_name)
    return mapping


def load_tests_blob(root: Path) -> str:
    tests_root = root / "crates" / "teamtalk" / "tests"
    if not tests_root.exists():
        return ""
    return "\n".join(
        rs.read_text(encoding="utf-8", errors="replace") for rs in tests_root.rglob("*.rs")
    )


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
    symbol_wrapper_map = build_symbol_wrapper_map(root)
    tests_blob = load_tests_blob(root)

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
        wrappers = sorted(symbol_wrapper_map.get(sym, set()))
        wrapper_test_hits = [
            fn for fn in wrappers if f"{fn}(" in tests_blob or f".{fn}(" in tests_blob
        ]
        if not hit_map["crates/teamtalk/tests"] and not wrapper_test_hits:
            uncovered_tests.append(sym)

    lines.append("## Coverage Summary")
    lines.append("")
    lines.append(f"- Symbols without wrapper/sys mapping: {len(uncovered_wrapper)}")
    if uncovered_wrapper:
        lines.append("- List: " + ", ".join(uncovered_wrapper[:120]))
    lines.append(f"- Symbols without direct tests reference: {len(uncovered_tests)}")
    if uncovered_tests:
        lines.append("- List: " + ", ".join(uncovered_tests[:120]))
    lines.append("- Test heuristic: raw symbol match OR mapped wrapper method match in tests.")
    lines.append("")
    lines.append("## Next Actions")
    lines.append("- Convert uncovered symbols into plan.md findings with disposition.")
    lines.append("- Implement wrappers/tests/docs or mark explicit defer reasons.")

    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
