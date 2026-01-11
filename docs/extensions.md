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

### Script format

Create a Lua file that defines a `commands` table. Each entry is a function
that returns `true` when it handled the command.

```lua
commands = {
  start = function(args)
    return true
  end,
  stop = function(args)
    return true
  end
}
```

### Host usage

```rust
use teamtalk::extensions::scripts::ScriptManager;

let mut manager = ScriptManager::new();
manager.load_script("commands", "scripts/commands.lua")?;
manager.call_command("start", &["channel".to_string()])?;
manager.reload_script("commands")?;
```

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
