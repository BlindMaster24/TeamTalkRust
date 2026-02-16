# TeamTalk Audit Plan

Last updated: 2026-02-16 20:35:41Z

## Current focus
- Audit TeamTalk.h comments and C-API docs against Rust wrappers.

## Work queue
- [x] Extract new/unchecked requirements from TeamTalk.h.
- [x] Cross-check C-API docs for the same area.
- [x] Map to wrappers in crates/teamtalk/src.
- [x] Triage uncovered symbols from plan_requirements_scan.md.
- [x] Validate targeted tests in crates/teamtalk/tests (mock reconnect scope).
- [x] Implement minimal fixes for reconnect disconnect/connect barrier observability.
- [x] Update docs/changelog if user-visible (no user-visible API behavior changes in this pass).

## Findings

### Open
<!-- AUTO-FINDINGS:START -->
- [AUTO][P0] Wrapper/sys uncovered symbols: 0.
  Status: no uncovered wrapper/sys symbols in current scan.
- [AUTO][INFO] Symbols without direct tests reference: 202.
  Top symbols: `TT_AcquireUserAudioBlock, TT_AcquireUserDesktopWindow, TT_AcquireUserDesktopWindowEx, TT_AcquireUserMediaVideoFrame, TT_AcquireUserVideoCaptureFrame, TT_AutoPositionUsers, TT_CancelFileTransfer, TT_CloseDesktopWindow, TT_CloseSoundDuplexDevices, TT_CloseSoundInputDevice, TT_CloseSoundLoopbackTest, TT_CloseSoundOutputDevice, TT_CloseTeamTalk, TT_CloseVideoCaptureDevice, TT_Connect, TT_ConnectEx, TT_ConnectSysID, TT_DBG_GETDATAPTR, TT_DBG_SIZEOF, TT_DBG_SetSoundInputTone`.
  Disposition: informational backlog metric (function-level heuristic); prioritize only high-risk paths.
<!-- AUTO-FINDINGS:END -->
- None.

### Closed
- Source requirement: `TeamTalk.h:3302` (`CLIENTEVENT_CMD_MYSELF_KICKED`) channel-kick vs server-kick state split.
  Current behavior: `kicked_next_state()` maps positive source to `LoggedIn`, otherwise `Connected`.
  Gap/Risk: previously no direct tests for source-based state split and pending command reset.
  Fix plan: added tests `myself_kicked_from_channel_keeps_connected_state` and `myself_kicked_from_server_resets_to_connected_and_clears_pending_cmds` in `crates/teamtalk/tests/client_logic_tests.rs`.
  Test coverage status: `cargo test -p teamtalk --features mock --test client_logic_tests` passed.
  Disposition: fixed.

- Source requirement: `CmdError` should clear matching pending login/join command and restore state.
  Current behavior: `update_state_for_event()` handles this in `crates/teamtalk/src/client/core.rs`.
  Gap/Risk: previously no direct regression test for cmd-id keyed resets.
  Fix plan: added tests `cmd_error_for_pending_login_returns_to_connected_and_clears_login_pending` and `cmd_error_for_pending_join_returns_to_logged_in_and_clears_join_pending`.
  Test coverage status: `cargo test -p teamtalk --features mock --test client_logic_tests` passed.
  Disposition: fixed.

- Source requirement: full-scan test-coverage signal should avoid false positives from constants and wrapper indirection.
  Current behavior: scan now focuses on callable `TT_*` functions and accepts wrapper-method references in tests.
  Gap/Risk: previously scan overstated uncovered tests by requiring raw `TT_*` references.
  Fix plan: updated `scan_requirements.py` function extraction and wrapper-to-test heuristic; marked result as informational backlog metric in auto-findings.
  Test coverage status: `run_audit_pass.py` completes with updated counts and preserved wrapper/sys uncovered = 0.
  Disposition: fixed (metric quality).

- Source requirement: `TeamTalk.h:3209` requires disconnect barrier before reconnect connect call.
  Current behavior: `Client::reconnect*()` executes barrier then connect via backend abstraction.
  Gap/Risk: previously no integration assertion for call order.
  Fix plan: added backend-level connect/disconnect/get_flags abstraction and mock call log, plus reconnect barrier tests in `crates/teamtalk/tests/client_logic_tests.rs`.
  Test coverage status: `cargo test -p teamtalk --features mock reconnect_` passed.
  Disposition: fixed for direct reconnect APIs.

- Source requirement: uncovered wrapper/sys mapping from full scan (`plan_requirements_scan.md` summary).
  Current behavior: after scanner fix to ignore comment-only tokens, wrapper/sys uncovered count is `0`.
  Gap/Risk: previous 12-symbol gap was false-positive from comment references and typo aliases in docs/comments.
  Fix plan: scanner now strips C comments before symbol extraction.
  Disposition: fixed in audit tooling.

## Next invocation instructions
1. Re-open this file first.
2. Continue from first unchecked queue item.
3. Append new findings under Open/Closed.
4. Run scan script and required cargo checks after edits.
