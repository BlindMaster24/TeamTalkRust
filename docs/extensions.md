# Extensions and Commands

This library supports a hybrid extension model:

- **Lua scripts** (feature: `scripts`) for fast iteration and hot reload.
- **Native plugins** (feature: `plugins`) for maximum performance.

Use either approach or both together.

## Enable Features

```toml
[dependencies]
teamtalk = { version = "1.0.1", features = ["scripts", "plugins"] }
```

## Lua Scripts

### What you get

- Hot reload without restarting the process.
- Simple command handlers in a single Lua file.
- Safe default: scripts only see the functions you expose.
- Optional event handlers for all client events.

### Script format

Create a Lua file that defines a `commands` table. Each entry is a function
that returns `true` when it handled the command. You can also define `on_event`
or an `events` table to handle client events.

```lua
commands = {
  start = function(args)
    return true
  end,
  stop = function(args)
    return true
  end
}

function on_event(ev)
  if ev.type == "TextMessage" and ev.text ~= nil then
    return false
  end
  return false
end

events = {
  UserJoined = function(ev)
    return false
  end
}
```

### Host usage

```rust
use teamtalk::extensions::scripts::ScriptManager;

let mut manager = ScriptManager::new();
manager.load_script("commands", "crates/teamtalk/examples/scripts/commands.lua")?;
manager.call_command("start", &["channel".to_string()])?;
manager.reload_script("commands")?;
```

### Registering host functions

```rust
manager.register_fn("ping", |_, msg: String| Ok(format!("pong: {}", msg)))?;
```

### Event payloads

Event handlers receive a table with fields:

- `type`: event name.
- `source`: source user id.
- `text`: table when a text message exists.
- `user`: table when a user payload exists.
- `channel`: table when a channel payload exists.
- `file_transfer`: table when a file transfer payload exists.
- `server_properties`: table when server properties exist.
- `server_statistics`: table when server statistics exist.
- `account`: table when a user account payload exists.

### Reload strategy

- Call `reload_script(name)` when the file changes.
- You can build a watcher in your app and call reload automatically.

## Native Plugins

### What you get

- Full Rust performance.
- Can hold state, share logic, and register event handlers.

### Required export

Your plugin library must export a `tt_plugin_init` symbol:

```rust
#[no_mangle]
pub extern "C" fn tt_plugin_init() {
    // init code here
}
```

### Host usage

```rust
use teamtalk::extensions::plugins::PluginManager;

let mut manager = PluginManager::new();
manager.load("sample", "plugins/sample_plugin.dll")?;
manager.reload("sample")?;
manager.unload("sample")?;
```

### Plugin crate layout

Create a separate crate and build a dynamic library:

```toml
[lib]
crate-type = ["cdylib"]
```

### Notes for Windows/Linux

- Windows: `.dll`
- Linux: `.so`

## Picking a Strategy

- Use **Lua scripts** for commands and quick changes.
- Use **native plugins** for heavy processing or long-lived state.
