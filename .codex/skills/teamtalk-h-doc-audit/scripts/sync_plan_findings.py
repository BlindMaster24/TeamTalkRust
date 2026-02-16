#!/usr/bin/env python3
import argparse
import re
from pathlib import Path

AUTO_START = "<!-- AUTO-FINDINGS:START -->"
AUTO_END = "<!-- AUTO-FINDINGS:END -->"


def parse_scan(scan_text: str) -> tuple[int, list[str], int, list[str]]:
    wrapper_count = 0
    wrapper_list: list[str] = []
    tests_count = 0
    tests_list: list[str] = []

    m = re.search(r"- Symbols without wrapper/sys mapping:\s+(\d+)", scan_text)
    if m:
        wrapper_count = int(m.group(1))
    m = re.search(
        r"- Symbols without wrapper/sys mapping:.*?\n- List:\s+(.+?)(?:\n- Symbols without direct tests reference:|\n\n)",
        scan_text,
        flags=re.S,
    )
    if m:
        wrapper_list = [x.strip() for x in m.group(1).replace("\n", " ").split(",") if x.strip()]

    m = re.search(r"- Symbols without direct tests reference:\s+(\d+)", scan_text)
    if m:
        tests_count = int(m.group(1))
    m = re.search(
        r"- Symbols without direct tests reference:.*?\n- List:\s+(.+?)(?:\n\n##|\Z)",
        scan_text,
        flags=re.S,
    )
    if m:
        tests_list = [x.strip() for x in m.group(1).replace("\n", " ").split(",") if x.strip()]

    return wrapper_count, wrapper_list, tests_count, tests_list


def build_auto_block(
    wrapper_count: int, wrapper_list: list[str], tests_count: int, tests_list: list[str]
) -> str:
    lines: list[str] = []
    lines.append(AUTO_START)
    lines.append(
        f"- [AUTO][P0] Wrapper/sys uncovered symbols: {wrapper_count}."
    )
    if wrapper_count > 0:
        preview = ", ".join(wrapper_list[:20])
        lines.append(f"  Top symbols: `{preview}`.")
        lines.append(
            "  Disposition: classify each as alias/intentional omission/missing binding/missing safe wrapper."
        )
    else:
        lines.append("  Status: no uncovered wrapper/sys symbols in current scan.")

    lines.append(
        f"- [AUTO][INFO] Symbols without direct tests reference: {tests_count}."
    )
    if tests_count > 0:
        preview = ", ".join(tests_list[:20])
        lines.append(f"  Top symbols: `{preview}`.")
        lines.append(
            "  Disposition: informational backlog metric (function-level heuristic); prioritize only high-risk paths."
        )
    else:
        lines.append("  Status: all symbols referenced in tests.")
    lines.append(AUTO_END)
    return "\n".join(lines)


def insert_or_replace_auto_block(plan_text: str, auto_block: str) -> str:
    if AUTO_START in plan_text and AUTO_END in plan_text:
        pattern = re.compile(
            re.escape(AUTO_START) + r".*?" + re.escape(AUTO_END), flags=re.S
        )
        return pattern.sub(auto_block, plan_text)

    marker = "### Open"
    idx = plan_text.find(marker)
    if idx == -1:
        return plan_text.rstrip() + "\n\n## Findings\n\n### Open\n" + auto_block + "\n"

    insert_pos = idx + len(marker)
    return plan_text[:insert_pos] + "\n" + auto_block + plan_text[insert_pos:]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    plan = root / "plan.md"
    scan = root / "plan_requirements_scan.md"

    if not scan.exists():
        raise SystemExit("plan_requirements_scan.md not found; run scan_requirements.py first")
    if not plan.exists():
        raise SystemExit("plan.md not found; run plan_sync.py first")

    scan_text = scan.read_text(encoding="utf-8", errors="replace")
    plan_text = plan.read_text(encoding="utf-8", errors="replace")

    wrapper_count, wrapper_list, tests_count, tests_list = parse_scan(scan_text)
    auto_block = build_auto_block(wrapper_count, wrapper_list, tests_count, tests_list)
    updated = insert_or_replace_auto_block(plan_text, auto_block)

    plan.write_text(updated, encoding="utf-8")
    print(f"updated {plan}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
