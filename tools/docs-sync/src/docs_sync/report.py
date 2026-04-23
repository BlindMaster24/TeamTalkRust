"""Finding / Severity / Report primitives for docs-sync."""

from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field
from enum import StrEnum
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path


class Severity(StrEnum):
    """Severity level of a single docs-sync finding."""

    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


@dataclass(frozen=True)
class Finding:
    """A single drift/mismatch detected by a checker.

    Attributes:
        checker: Stable id of the producing checker (e.g. ``versions``).
        severity: :class:`Severity` value.
        file: Repository-relative path of the file with the issue.
        message: Human-readable description.
        rule: Stable id of the rule within the checker (e.g.
            ``version-mismatch``).
        line: 1-based line number, or ``None`` if not applicable.
        expected: Optional expected value (used by ``--fix``).
        actual: Optional actual value observed in the file.
    """

    checker: str
    severity: Severity
    file: str
    message: str
    rule: str
    line: int | None = None
    expected: str | None = None
    actual: str | None = None


@dataclass
class Report:
    """Aggregated result of a docs-sync run."""

    findings: list[Finding] = field(default_factory=list)
    scanned_files: int = 0

    def add(self, finding: Finding) -> None:
        """Append a finding to the report."""
        self.findings.append(finding)

    def extend(self, findings: list[Finding]) -> None:
        """Append multiple findings to the report."""
        self.findings.extend(findings)

    def errors(self) -> list[Finding]:
        """Return only findings with :class:`Severity.ERROR`."""
        return [f for f in self.findings if f.severity == Severity.ERROR]

    def warnings(self) -> list[Finding]:
        """Return only findings with :class:`Severity.WARNING`."""
        return [f for f in self.findings if f.severity == Severity.WARNING]

    def to_text(self) -> str:
        """Render the report as a human-readable text summary."""
        if not self.findings:
            return f"docs-sync: OK ({self.scanned_files} files scanned)\n"
        lines = [
            f"docs-sync: {len(self.findings)} finding(s) "
            f"({len(self.errors())} error, {len(self.warnings())} warning) "
            f"across {self.scanned_files} file(s)",
            "",
        ]
        for f in self.findings:
            loc = f"{f.file}:{f.line}" if f.line is not None else f.file
            lines.append(
                f"[{f.severity.value}] [{f.checker}/{f.rule}] {loc}: {f.message}",
            )
            if f.expected is not None or f.actual is not None:
                lines.append(f"    expected: {f.expected!r}")
                lines.append(f"    actual:   {f.actual!r}")
        return "\n".join(lines) + "\n"

    def to_markdown(self) -> str:
        """Render the report as a Markdown summary for PR comments."""
        if not self.findings:
            return f"# docs-sync\n\nOK -- {self.scanned_files} file(s) scanned.\n"
        sections: dict[str, list[Finding]] = {}
        for f in self.findings:
            sections.setdefault(f.checker, []).append(f)
        out = [
            "# docs-sync",
            "",
            f"{len(self.findings)} finding(s) "
            f"({len(self.errors())} error, {len(self.warnings())} warning) "
            f"across {self.scanned_files} file(s).",
            "",
        ]
        for checker, findings in sorted(sections.items()):
            out.extend([f"## {checker}", ""])
            out.append("| Severity | Rule | File:Line | Message |")
            out.append("|---|---|---|---|")
            for f in findings:
                loc = f"`{f.file}:{f.line}`" if f.line is not None else f"`{f.file}`"
                msg = f.message.replace("|", r"\|")
                out.append(
                    f"| {f.severity.value} | `{f.rule}` | {loc} | {msg} |",
                )
            out.append("")
        return "\n".join(out) + "\n"

    def to_json(self) -> str:
        """Render the report as JSON."""
        payload = {
            "scanned_files": self.scanned_files,
            "findings": [{**asdict(f), "severity": f.severity.value} for f in self.findings],
        }
        return json.dumps(payload, indent=2, sort_keys=True) + "\n"

    def write_reports(self, base: Path, formats: list[str]) -> None:
        """Write the report to ``base`` in the chosen formats.

        Args:
            base: Path stem for the output files (without extension).
            formats: Subset of ``{"json", "md", "txt"}``.
        """
        base.parent.mkdir(parents=True, exist_ok=True)
        if "json" in formats:
            base.with_suffix(".json").write_text(self.to_json(), encoding="utf-8")
        if "md" in formats:
            base.with_suffix(".md").write_text(self.to_markdown(), encoding="utf-8")
        if "txt" in formats:
            base.with_suffix(".txt").write_text(self.to_text(), encoding="utf-8")
