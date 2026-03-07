# Bot Framework

The `bot` feature provides a high-level framework on top of polling TeamTalk
events. It is designed for command bots, workflow bots, and multi-step dialog
scenes without forcing a specific persistence backend.

Enable it in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "4.0.0", features = ["bot"] }
```

## Core Building Blocks

- `Router`: routes commands, events, and dialog steps.
- `Context`: handler access to client, parsed command, state store, and dialog helpers.
- `Bot` / `BotBuilder`: sync bot runtime.
- `AsyncBot` / `AsyncBotBuilder`: async runtime (`bot` + `async`).
- `BotApp`: one place to wire router, scheduler, state store, and runtime.
- `Scheduler`: periodic jobs that share the same `StateStore`.
- `MemoryStateStore`, `RedisStateStore`, `SqliteStateStore`: persistence options.

## Command and Middleware Model

Use `Router::on_command`, `on_command_pattern`, `on_event`, `on_any`,
`on_dialog_step`, and `on_dialog` to register handlers. Middleware runs before
handlers, can stop dispatch early, and can run post-handler cleanup through
`use_middleware_hooks`.

Recommended layering:

1. command parsing and validation
2. reusable middleware guards
3. scene/dialog handlers
4. scheduler jobs

## State Strategy

`Context` exposes four state scopes:

- global: `global_state_*`
- user: `user_state_*`
- channel: `channel_state_*`
- dialog/session: `dialog_state_*`

Use dialog-scoped state for temporary scene scratch data. Dialog state keys are
session-aware, so restarting the same dialog creates a new namespace instead of
reusing old scene-local values.

Typed helpers are available for all scopes:

- `state_parse`, `state_set_typed`
- `user_state_parse`, `user_state_set_typed`
- `channel_state_parse`, `channel_state_set_typed`
- `global_state_parse`, `global_state_set_typed`
- `dialog_state_parse`, `dialog_state_set_typed`
- `dialog_metadata_parse`

These helpers use `FromStr` and `Display`, so they work well for integers,
booleans, enums with manual parsing, and similar lightweight types without
adding serialization dependencies.

## Scene Lifecycle

For multi-step conversations, use `DialogFlow`, `DialogState`, and `Context`'s
dialog helpers. The framework supports:

- explicit start, restart, advance, cancel, and finish
- pause/resume
- metadata attached to the dialog state
- dialog-local scratch storage
- optional timeout with policy control

See [scenes.md](scenes.md) for the full scene model.
