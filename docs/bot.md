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

Useful built-in middleware:

- preferred for authorization:
  - `RequireClientRightsAny`
  - `RequireClientRightsAll`
- `CommandOnly`
- `RateLimitBySource`
- `RequirePrivateMessage`
- `RequireChannelMessage`
- `RequireCommand`
- `RequireCommandPrefix`
- `RequireUserIds`
- `RequireUserType` (sender-cache-based, use only when cached `user_type` is the signal you want)

For guard patterns and rate limiting, see [guards.md](guards.md).

For common rights bundles, use `Permissions::moderator()`,
`Permissions::file_manager()`, `Permissions::channel_admin()`,
`Permissions::media_sender()`, `Permissions::desktop_controller()`, and
`Permissions::server_admin()` / `Permissions::admin()`.

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

If you enable `bot-serde`, you also get JSON-backed helpers:

- `state_get_json`, `state_set_json`
- `state_get_json_or_default`
- `user_state_get_json`, `user_state_set_json`
- `user_state_get_json_or_default`
- `channel_state_get_json`, `channel_state_set_json`
- `channel_state_get_json_or_default`
- `global_state_get_json`, `global_state_set_json`
- `global_state_get_json_or_default`
- `dialog_state_get_json`, `dialog_state_set_json`
- `dialog_state_get_json_or_default`

That is the preferred option when scene/user state is a structured payload.

## Scene Lifecycle

For multi-step conversations, use `DialogFlow`, `DialogState`, and `Context`'s
dialog helpers. The framework supports:

- explicit start, restart, advance, cancel, and finish
- pause/resume
- metadata attached to the dialog state
- dialog-local scratch storage
- optional timeout with policy control

The router also supports:

- command aliases through `alias_command`
- unknown-command suggestions through `with_unknown_command_suggestions`

The bot macros crate provides `#[teamtalk_command]`, `#[teamtalk_command_help]`,
`#[teamtalk_event]`, and `#[teamtalk_middleware]` to keep handler registration
compact.

Macros support aliases:

```rust
#[teamtalk_command("ping", "p")]
fn ping_handler(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    let _ = ctx.reply_private("pong");
    Ok(HandlerResult::Continue)
}

#[teamtalk_command_help("help", "Show help", "h")]
fn help_handler(ctx: &mut teamtalk::Context<'_>) -> teamtalk::Result<HandlerResult> {
    Ok(HandlerResult::Continue)
}
```

See [scenes.md](scenes.md) for the full scene model.
