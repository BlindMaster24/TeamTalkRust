# Scenes and Dialogs

Scenes are built on top of `DialogFlow`, `DialogState`, and `Context::dialog_*`
helpers.

## Dialog Flow

`DialogFlow` declares the allowed ordered steps:

```rust
use teamtalk::DialogFlow;

let onboarding = DialogFlow::new("onboarding", "ask_name")
    .step("ask_email")
    .step("done");
```

Helpers on `DialogFlow`:

- `contains_step`
- `next_step`
- `previous_step`
- `is_start_step`
- `is_terminal_step`

These let handlers move through the declared flow instead of hardcoding string
transitions everywhere.

## Lifecycle Operations

`Context` exposes the scene lifecycle:

- `dialog_start`
- `dialog_start_state`
- `dialog_start_flow`
- `dialog_start_checked`
- `dialog_restart_flow`
- `dialog_advance`
- `dialog_advance_checked`
- `dialog_advance_next`
- `dialog_pause`
- `dialog_resume`
- `dialog_cancel`
- `dialog_finish`
- `dialog_stop`

`dialog_cancel`, `dialog_finish`, and `dialog_stop` currently all end the active
scene. Use the name that best matches the handler intent.

## Timeout Handling

Dialogs can carry a deadline through `DialogState::with_timeout`,
`with_deadline_unix_ms`, or `Context::dialog_set_timeout`.

Timeout behavior is controlled by `DialogTimeoutPolicy`:

- `DialogTimeoutPolicy::Clear`: default; expired dialogs are removed.
- `DialogTimeoutPolicy::Pause`: expired dialogs become paused and stay available
  through `dialog_current_live()`.

Use:

- `DialogState::with_timeout_policy`
- `Context::dialog_set_timeout_policy`
- `Context::dialog_timeout_policy`

`dialog_current()` returns only active, non-expired scenes. `dialog_current_live()`
returns a live scene even if it is paused.

## Metadata and Session-Scoped State

There are two ways to keep extra scene data:

1. dialog metadata inside `DialogState`
2. dialog-scoped state in the `StateStore`

Metadata helpers:

- `dialog_metadata`
- `dialog_metadata_parse`
- `dialog_set_metadata`
- `dialog_remove_metadata`

Dialog-scoped store helpers:

- `dialog_state_key`
- `dialog_state_get`
- `dialog_state_parse`
- `dialog_state_set`
- `dialog_state_set_typed`
- `dialog_state_remove`

Dialog-scoped state uses a session-aware key. Restarting the same flow creates a
new dialog session id, so scratch values from the previous run do not collide
with the current one.

## Recommended Pattern

Use this split consistently:

- dialog metadata for small state tied directly to the dialog record
- dialog-scoped store for larger scratch values or typed counters
- user/global state for durable bot data that must outlive the current scene
