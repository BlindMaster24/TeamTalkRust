"""Tests for the Report primitives."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from docs_sync.report import Finding, Report, Severity

if TYPE_CHECKING:
    from pathlib import Path


def _finding(severity: Severity = Severity.ERROR, **kw: object) -> Finding:
    defaults: dict[str, object] = {
        "checker": "versions",
        "severity": severity,
        "file": "README.md",
        "message": "mismatch",
        "rule": "version-mismatch",
        "line": 32,
        "expected": "6.0.0",
        "actual": "4.0.0",
    }
    defaults.update(kw)
    return Finding(**defaults)  # type: ignore[arg-type]


def test_empty_report_is_ok() -> None:
    report = Report()
    report.scanned_files = 3
    assert "OK" in report.to_text()
    assert "OK" in report.to_markdown()
    assert json.loads(report.to_json())["findings"] == []


def test_report_classifies_errors_and_warnings() -> None:
    report = Report()
    report.add(_finding(severity=Severity.ERROR))
    report.add(_finding(severity=Severity.WARNING))
    report.add(_finding(severity=Severity.INFO))
    assert len(report.errors()) == 1
    assert len(report.warnings()) == 1
    assert len(report.findings) == 3


def test_report_text_is_plain_ascii() -> None:
    report = Report()
    report.scanned_files = 1
    report.add(_finding())
    text = report.to_text()
    assert text.isascii()
    assert "[error]" in text
    assert "versions/version-mismatch" in text
    assert "README.md:32" in text


def test_report_markdown_is_plain_ascii() -> None:
    """Regression: the Markdown report must be pure 7-bit ASCII on every
    code path so that screen readers render it verbatim. Previously the
    ``OK`` line used a U+2014 em dash.
    """
    empty = Report()
    empty.scanned_files = 2
    assert empty.to_markdown().isascii()

    populated = Report()
    populated.scanned_files = 1
    populated.add(_finding())
    assert populated.to_markdown().isascii()


def test_report_markdown_escapes_pipes() -> None:
    report = Report()
    report.add(_finding(message="a | b"))
    md = report.to_markdown()
    assert r"a \| b" in md


def test_report_json_roundtrip() -> None:
    report = Report()
    report.scanned_files = 2
    report.add(_finding())
    payload = json.loads(report.to_json())
    assert payload["scanned_files"] == 2
    [finding] = payload["findings"]
    assert finding["severity"] == "error"
    assert finding["line"] == 32


def test_report_write_formats(tmp_path: Path) -> None:
    report = Report()
    report.add(_finding())
    base = tmp_path / "sub" / "out"
    report.write_reports(base, ["txt", "md", "json"])
    assert base.with_suffix(".txt").exists()
    assert base.with_suffix(".md").exists()
    assert base.with_suffix(".json").exists()
