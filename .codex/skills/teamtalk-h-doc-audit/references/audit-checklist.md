# References for TeamTalk H+Doc Audit

Use this file as a quick navigation map during audits.

## Canonical sources

- `TEAMTALK_DLL/TeamTalk.h`
- `TEAMTALK_DLL/Documentation/C-API/`

## Rust mapping targets

- `crates/teamtalk-sys/`
- `crates/teamtalk/src/client/`
- `crates/teamtalk/tests/`

## Fast checks

- `rg -n "TT_Connect|TT_Disconnect|TT_DoLogin|TT_DoJoinChannel|CMDERR_" TEAMTALK_DLL/TeamTalk.h`
- `rg -n "reconnect|kick|disconnect|login|join" crates/teamtalk/src crates/teamtalk/tests`
- `cargo test -p teamtalk --tests`

## Review output template

- Source requirement:
- Current behavior:
- Gap/Risk:
- Fix plan:
- Test status:
