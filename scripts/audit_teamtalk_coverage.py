from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Iterable

HEADER_PATH = Path("TEAMTALK_DLL/TeamTalk.h")
DOCS_PATH = Path("TEAMTALK_DLL/Documentation/C-API")
BINDINGS_PATH = Path("crates/teamtalk-sys")
GENERATED_BINDINGS_GLOB = "target/debug/build/teamtalk-sys-*/out/bindings.rs"
SRC_PATH = Path("crates/teamtalk/src")
TESTS_PATH = Path("crates/teamtalk/tests")
USER_DOCS_PATH = Path("docs")
DEV_DOC_PATH = Path("docs/dev.md")
README_PATH = Path("README.md")
DEFAULT_JSON = Path("target/teamtalk-coverage-audit.json")
DEFAULT_MD = Path("docs/teamtalk-coverage.md")

CONSTANT_PREFIXES = (
    "TT_CLASSROOM_",
    "TT_DESKTOPINPUT_",
    "TT_SOUNDDEVICE_ID_",
    "TT_TRANSMITUSERS_",
)
CONSTANT_SYMBOLS = {
    "TT_CHANNELID_MAX",
    "TT_CHANNELS_OPERATOR_MAX",
    "TT_LOCAL_TX_USERID",
    "TT_LOCAL_USERID",
    "TT_MEDIAPLAYBACK_OFFSET_IGNORE",
    "TT_MUXED_USERID",
    "TT_SAMPLERATES_MAX",
    "TT_TRANSMITQUEUE_MAX",
    "TT_TRANSMITUSERS_FREEFORALL",
    "TT_TRANSMITUSERS_MAX",
    "TT_USERID_MAX",
    "TT_VIDEOFORMATS_MAX",
}
HIGH_LEVEL_WRAPPERS_WANTED = {
    "TT_AcquireUserDesktopWindow",
    "TT_AcquireUserDesktopWindowEx",
    "TT_CloseDesktopWindow",
    "TT_Connect",
    "TT_ConnectEx",
    "TT_ConnectSysID",
    "TT_DesktopInput_Execute",
    "TT_DesktopInput_KeyTranslate",
    "TT_Disconnect",
    "TT_DoBanUser",
    "TT_DoChannelOp",
    "TT_DoChannelOpEx",
    "TT_DoChangeStatus",
    "TT_DoJoinChannelByID",
    "TT_DoKickUser",
    "TT_DoLoginEx",
    "TT_DoLogout",
    "TT_DoMoveUser",
    "TT_DoTextMessage",
    "TT_GetChannel",
    "TT_GetFlags",
    "TT_GetMediaFileInfo",
    "TT_GetMyChannelID",
    "TT_GetMyUserID",
    "TT_GetMyUserRights",
    "TT_GetUser",
    "TT_HotKey_GetKeyString",
    "TT_HotKey_InstallTestHook",
    "TT_HotKey_IsActive",
    "TT_HotKey_Register",
    "TT_HotKey_RemoveTestHook",
    "TT_HotKey_Unregister",
    "TT_InitLocalPlayback",
    "TT_InitTeamTalk",
    "TT_InitTeamTalkPoll",
    "TT_PaintDesktopWindow",
    "TT_PaintDesktopWindowEx",
    "TT_PaintVideoFrame",
    "TT_PaintVideoFrameEx",
    "TT_Palette_GetColorTable",
    "TT_ReleaseUserDesktopWindow",
    "TT_ReleaseUserMediaVideoFrame",
    "TT_ReleaseUserVideoCaptureFrame",
    "TT_SendDesktopCursorPosition",
    "TT_SendDesktopInput",
    "TT_SendDesktopWindow",
    "TT_SendDesktopWindowFromHWND",
    "TT_StartStreamingMediaFileToChannel",
    "TT_StartStreamingMediaFileToChannelEx",
    "TT_StopLocalPlayback",
    "TT_StopStreamingMediaFileToChannel",
    "TT_UpdateLocalPlayback",
    "TT_UpdateStreamingMediaFileToChannel",
    "TT_Windows_GetDesktopActiveHWND",
    "TT_Windows_GetDesktopHWND",
    "TT_Windows_GetDesktopWindowHWND",
    "TT_Windows_GetWindow",
    "TT_Mixer_SetWaveInSelected",
    "TT_Mixer_GetWaveInSelected",
    "TT_Mixer_SetWaveInVolume",
    "TT_Mixer_GetWaveInVolume",
}

@dataclass
class SymbolRecord:
    symbol: str
    in_bindings: bool
    in_src: bool
    in_tests: bool
    in_docs: bool
    category: str
    wrapper_expected: bool
    status: str
    rationale: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--json", default=str(DEFAULT_JSON))
    parser.add_argument("--markdown", default=str(DEFAULT_MD))
    return parser.parse_args()


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def strip_comments(text: str) -> str:
    text = re.sub(r"/\*.*-\*/", "", text, flags=re.S)
    text = re.sub(r"//.*", "", text)
    return text


def extract_symbols(text: str) -> list[str]:
    return sorted(set(re.findall(r"\bTT_[A-Za-z0-9_]+\b", text)))


def text_hits(root: Path) -> dict[Path, str]:
    hits: dict[Path, str] = {}
    if not root.exists():
        return hits
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        try:
            hits[path] = read_text(path)
        except Exception:
            continue
    return hits


def contains_symbol(files: dict[Path, str], symbol: str) -> bool:
    return any(symbol in content for content in files.values())


def classify(symbol: str) -> tuple[str, bool, str]:
    if symbol.startswith("TT_MacOS_") or symbol == "TT_SendDesktopFromWindowID":
        return "platform-specific", False, "macOS-specific API or target-gated desktop window path"
    if symbol in CONSTANT_SYMBOLS or symbol.startswith(CONSTANT_PREFIXES):
        return "constant-or-macro", False, "header constant or macro-style symbol; no separate high-level wrapper required"
    if symbol in HIGH_LEVEL_WRAPPERS_WANTED:
        return "runtime-api", True, "user-facing runtime API with practical value in the safe layer"
    return "low-level-or-specialized", False, "low-level or specialized API; wrap only when there is concrete user need"


def build_records(root: Path) -> list[SymbolRecord]:
    header_text = strip_comments(read_text(root / HEADER_PATH))
    symbols = extract_symbols(header_text)
    bindings_files = text_hits(root / BINDINGS_PATH)
    for generated in root.glob(GENERATED_BINDINGS_GLOB):
        if generated.is_file():
            try:
                bindings_files[generated] = read_text(generated)
            except Exception:
                pass
    src_files = text_hits(root / SRC_PATH)
    test_files = text_hits(root / TESTS_PATH)
    docs_files = text_hits(root / USER_DOCS_PATH)
    if (root / README_PATH).exists():
        docs_files[root / README_PATH] = read_text(root / README_PATH)
    if (root / DEV_DOC_PATH).exists():
        docs_files[root / DEV_DOC_PATH] = read_text(root / DEV_DOC_PATH)
    records: list[SymbolRecord] = []
    for symbol in symbols:
        category, wrapper_expected, rationale = classify(symbol)
        in_bindings = contains_symbol(bindings_files, symbol)
        in_src = contains_symbol(src_files, symbol)
        in_tests = contains_symbol(test_files, symbol)
        in_docs = contains_symbol(docs_files, symbol)
        if not in_bindings:
            status = "missing-binding"
        elif wrapper_expected and not in_src:
            status = "missing-wrapper"
        elif wrapper_expected and in_src and not in_tests:
            status = "missing-test"
        elif wrapper_expected and in_src and not in_docs:
            status = "missing-doc"
        elif in_src:
            status = "covered"
        else:
            status = "intentional-omission"
        records.append(
            SymbolRecord(
                symbol=symbol,
                in_bindings=in_bindings,
                in_src=in_src,
                in_tests=in_tests,
                in_docs=in_docs,
                category=category,
                wrapper_expected=wrapper_expected,
                status=status,
                rationale=rationale,
            )
        )
    return records


def summary(records: Iterable[SymbolRecord]) -> dict[str, int]:
    items = list(records)
    return {
        "total_symbols": len(items),
        "missing_bindings": sum(r.status == "missing-binding" for r in items),
        "missing_wrappers": sum(r.status == "missing-wrapper" for r in items),
        "missing_tests": sum(r.status == "missing-test" for r in items),
        "missing_docs": sum(r.status == "missing-doc" for r in items),
        "covered": sum(r.status == "covered" for r in items),
        "intentional_omissions": sum(r.status == "intentional-omission" for r in items),
    }


def render_markdown(records: list[SymbolRecord], summary_map: dict[str, int]) -> str:
    sections = {
        "missing-binding": [],
        "missing-wrapper": [],
        "missing-test": [],
        "missing-doc": [],
        "intentional-omission": [],
    }
    for record in records:
        if record.status in sections:
            sections[record.status].append(record)
    lines = [
        "# TeamTalk Coverage Audit",
        "",
        "This report is generated by `scripts/audit_teamtalk_coverage.py`.",
        "",
        "## Summary",
        "",
    ]
    for key, value in summary_map.items():
        lines.append(f"- `{key}`: {value}")
    for title, key in [
        ("Missing bindings", "missing-binding"),
        ("Missing high-level wrappers", "missing-wrapper"),
        ("Missing tests for wrapped APIs", "missing-test"),
        ("Missing docs for wrapped APIs", "missing-doc"),
        ("Intentional omissions", "intentional-omission"),
    ]:
        lines.extend(["", f"## {title}", ""])
        if not sections[key]:
            lines.append("- none")
            continue
        for record in sections[key]:
            lines.append(
                f"- `{record.symbol}`  `{record.category}`  {record.rationale}"
            )
    lines.extend(
        [
            "",
            "## Wrapper Policy",
            "",
            "- Wrap symbols in the high-level SDK when they are user-facing runtime APIs with clear safe semantics and concrete downstream value.",
            "- Keep constants/macros, platform-specific symbols for unsupported targets, and highly specialized low-level utilities as intentional omissions unless a concrete use case appears.",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    root = Path(args.root).resolve()
    records = build_records(root)
    payload = {
        "summary": summary(records),
        "records": [asdict(record) for record in records],
    }
    json_path = (root / args.json).resolve()
    json_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    markdown_path = (root / args.markdown).resolve()
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text(render_markdown(records, payload["summary"]), encoding="utf-8")
    print(json.dumps(payload["summary"], indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
