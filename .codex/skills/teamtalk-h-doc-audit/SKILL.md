---
name: teamtalk-h-doc-audit
description: Perform full-coverage TeamTalk Rust audits against all TeamTalk.h comments and the entire C-API Documentation tree. Use when checking for any missing wrapper behavior, uncovered events, untested requirements, or doc mismatches; always write/update plan.md first, then iterate until no uncovered gaps remain.
---

# TeamTalk H+Doc Audit

Run this skill when the task is to verify or improve TeamTalk Rust behavior using all header comments and all C-API docs as source requirements.

## Mandatory Loop

1. Create or refresh `plan.md` at repository root.
2. Read `TEAMTALK_DLL/TeamTalk.h` comprehensively in passes.
3. Read `TEAMTALK_DLL/Documentation/C-API/` comprehensively in passes.
4. Map each requirement to `crates/teamtalk/src/` behavior.
5. Implement minimal fixes.
6. Add/update tests in `crates/teamtalk/tests`.
7. Update docs/changelog for user-visible changes.
8. Re-run full scan and continue from unresolved plan items on next invocation.

Always continue from existing `plan.md` if it exists.

## Required Commands

Primary one-command pass:

- `python .codex/skills/teamtalk-h-doc-audit/scripts/run_audit_pass.py --root .`

Start plan file:

- `python .codex/skills/teamtalk-h-doc-audit/scripts/plan_sync.py --root . --mode start`

Refresh status after each pass:

- `python .codex/skills/teamtalk-h-doc-audit/scripts/plan_sync.py --root . --mode update`

Full requirement scan:

- `python .codex/skills/teamtalk-h-doc-audit/scripts/scan_requirements.py --root .`

Sync auto findings from scan into `plan.md`:

- `python .codex/skills/teamtalk-h-doc-audit/scripts/sync_plan_findings.py --root .`

## Source Priority

1. `TEAMTALK_DLL/TeamTalk.h`
2. `TEAMTALK_DLL/Documentation/C-API/`
3. `crates/teamtalk-sys/`
4. `crates/teamtalk/src/`
5. `crates/teamtalk/tests/`
6. `README.md` and `docs/`

If header and docs disagree, follow `TeamTalk.h` and record mismatch.

## Full-Coverage Rule

- Do not limit review to reconnect only.
- Audit all discovered `TT_*` API symbols from `TeamTalk.h`.
- For each symbol, verify:
  - presence in `teamtalk-sys` bindings,
  - safe-wrapper mapping or explicit intentional omission,
  - test coverage or explicit gap entry,
  - user-doc impact if behavior is user-visible.
- Keep iterating until all uncovered symbols are either implemented, tested, documented, or explicitly deferred in `plan.md`.

## High-Risk Areas

- Disconnect-before-reconnect barrier.
- Logged-in preconditions for command wrappers.
- Pending command state reset on errors.
- Event ordering and reconnect trigger deduplication.
- Channel password handling only in memory for current process.
- Any API present in `TeamTalk.h` but missing wrapper/test/docs mapping.

## Deliverable Format Per Finding

- Source requirement.
- Current behavior with file/function.
- Gap/risk.
- Fix plan.
- Test coverage status.
- Disposition: fix now / defer with reason.

## Constraints

- Keep fixes minimal and scoped.
- No tests in `src`; place tests in `crates/teamtalk/tests`.
- Do not persist secrets.
- Keep commit scope clean: code, tests, docs separated when possible.
- Keep `plan.md` timestamp fresh on every pass by running `run_audit_pass.py`.
