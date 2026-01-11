#[cfg(feature = "scripts")]
use mlua::{Lua, Value};
#[cfg(feature = "scripts")]
use std::collections::HashMap;
#[cfg(feature = "scripts")]
use std::fs;
#[cfg(feature = "scripts")]
use std::path::{Path, PathBuf};

#[cfg(feature = "scripts")]
pub struct ScriptManager {
    lua: Lua,
    scripts: HashMap<String, ScriptEntry>,
}

#[cfg(feature = "scripts")]
struct ScriptEntry {
    path: PathBuf,
}

#[cfg(feature = "scripts")]
impl ScriptManager {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            scripts: HashMap::new(),
        }
    }

    pub fn load_script(&mut self, name: &str, path: impl AsRef<Path>) -> mlua::Result<()> {
        let path = path.as_ref().to_path_buf();
        let contents = fs::read_to_string(&path)?;
        self.lua.load(&contents).exec()?;
        self.scripts.insert(name.to_string(), ScriptEntry { path });
        Ok(())
    }

    pub fn reload_script(&mut self, name: &str) -> mlua::Result<()> {
        let entry = self
            .scripts
            .get(name)
            .ok_or_else(|| mlua::Error::RuntimeError("script not found".into()))?;
        let contents = fs::read_to_string(&entry.path)?;
        self.lua.load(&contents).exec()?;
        Ok(())
    }

    pub fn unload_script(&mut self, name: &str) -> mlua::Result<()> {
        self.scripts
            .remove(name)
            .ok_or_else(|| mlua::Error::RuntimeError("script not found".into()))?;
        Ok(())
    }

    pub fn call_command(&self, command: &str, args: &[String]) -> mlua::Result<bool> {
        let globals = self.lua.globals();
        let handlers: Value = globals.get("commands")?;
        let handlers = match handlers {
            Value::Table(table) => table,
            _ => return Ok(false),
        };
        let func: Value = handlers.get(command)?;
        let func = match func {
            Value::Function(func) => func,
            _ => return Ok(false),
        };
        let args_table = self.lua.create_table()?;
        for (idx, arg) in args.iter().enumerate() {
            args_table.set(idx + 1, arg.clone())?;
        }
        let result: bool = func.call(args_table)?;
        Ok(result)
    }
}

#[cfg(feature = "scripts")]
impl Default for ScriptManager {
    fn default() -> Self {
        Self::new()
    }
}
