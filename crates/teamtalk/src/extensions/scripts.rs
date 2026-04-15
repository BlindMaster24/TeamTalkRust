#[cfg(feature = "scripts")]
use crate::client::Message;
#[cfg(feature = "scripts")]
use crate::events::Event;
#[cfg(feature = "scripts")]
use crate::types::{
    Channel, FileTransfer, ServerProperties, ServerStatistics, TextMessage, User, UserAccount,
};
#[cfg(feature = "scripts")]
use mlua::{HookTriggers, Lua, Table, Value, VmState};
#[cfg(feature = "scripts")]
use std::collections::HashMap;
#[cfg(feature = "scripts")]
use std::fs;
#[cfg(feature = "scripts")]
use std::path::{Path, PathBuf};
#[cfg(feature = "scripts")]
use std::time::{Duration, Instant};

#[cfg(feature = "scripts")]
pub struct ScriptManager {
    lua: Lua,
    scripts: HashMap<String, ScriptEntry>,
    max_exec_time: Option<Duration>,
    hook_instruction_count: u32,
    init_error: Option<mlua::Error>,
}

#[cfg(feature = "scripts")]
struct ScriptEntry {
    path: PathBuf,
    globals: Vec<String>,
}

#[cfg(feature = "scripts")]
impl ScriptManager {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        let mut manager = Self {
            lua: Lua::new(),
            scripts: HashMap::new(),
            max_exec_time: None,
            hook_instruction_count: 50_000,
            init_error: None,
        };
        if let Err(err) = manager.register_builtin_api() {
            manager.init_error = Some(err);
        }
        manager
    }

    pub fn set_timeout(&mut self, max_exec_time: Duration) {
        self.max_exec_time = Some(max_exec_time);
    }

    pub fn clear_timeout(&mut self) {
        self.max_exec_time = None;
    }

    pub fn set_hook_instruction_count(&mut self, count: u32) {
        self.hook_instruction_count = count.max(1);
    }

    pub fn load_script(&mut self, name: &str, path: impl AsRef<Path>) -> mlua::Result<()> {
        self.ensure_ready()?;
        let path = path.as_ref().to_path_buf();
        let contents = fs::read_to_string(&path)?;
        let globals_before = self.collect_global_keys()?;
        let result = self.with_script_name(name, || {
            self.with_timeout(|| self.lua.load(&contents).exec())
        });
        if let Err(err) = result {
            return Err(Self::wrap_error(name, "load_script", &err));
        }
        let globals_after = self.collect_global_keys()?;
        let globals = diff_globals(&globals_before, &globals_after);
        self.scripts
            .insert(name.to_string(), ScriptEntry { path, globals });
        Ok(())
    }

    pub fn reload_script(&mut self, name: &str) -> mlua::Result<()> {
        self.ensure_ready()?;
        let path = self
            .scripts
            .get(name)
            .ok_or_else(|| mlua::Error::RuntimeError("script not found".into()))?
            .path
            .clone();
        self.unload_script(name)?;
        self.load_script(name, path)
    }

    pub fn unload_script(&mut self, name: &str) -> mlua::Result<()> {
        self.ensure_ready()?;
        let entry = self
            .scripts
            .remove(name)
            .ok_or_else(|| mlua::Error::RuntimeError("script not found".into()))?;
        let globals = self.lua.globals();
        for key in entry.globals {
            let _ = globals.raw_remove(key);
        }
        self.remove_registered_commands(name)?;
        Ok(())
    }

    pub fn call_command(&self, command: &str, args: &[String]) -> mlua::Result<bool> {
        self.ensure_ready()?;
        let globals = self.lua.globals();
        let handlers: Value = globals.get("commands")?;
        let Value::Table(handlers) = handlers else {
            return Ok(false);
        };
        let func: Value = handlers.get(command)?;
        let Value::Function(func) = func else {
            return Ok(false);
        };
        let args_table = self.lua.create_table()?;
        for (idx, arg) in args.iter().enumerate() {
            args_table.set(idx + 1, arg.clone())?;
        }
        let result = self
            .with_timeout(|| func.call::<bool>(args_table))
            .map_err(|err| Self::wrap_error(command, "call_command", &err))?;
        Ok(result)
    }

    pub fn register_fn<A, R, F>(&mut self, name: &str, func: F) -> mlua::Result<()>
    where
        F: for<'lua> Fn(&'lua Lua, A) -> mlua::Result<R> + Send + 'static,
        A: mlua::FromLuaMulti,
        R: mlua::IntoLuaMulti,
    {
        self.ensure_ready()?;
        let f = self.lua.create_function(func)?;
        let globals = self.lua.globals();
        globals.set(name, f)?;
        Ok(())
    }

    pub fn handle_event(&self, event: Event, message: &Message) -> mlua::Result<bool> {
        self.ensure_ready()?;
        let globals = self.lua.globals();
        let mut handled = false;
        if let Ok(Value::Function(func)) = globals.get::<Value>("on_event") {
            let event_table = self.event_table(event, message)?;
            let result = self
                .with_timeout(|| func.call::<bool>(event_table))
                .map_err(|err| Self::wrap_error(event_name(event), "on_event", &err))?;
            handled |= result;
        }
        if let Ok(Value::Table(table)) = globals.get::<Value>("events") {
            let key = event_name(event);
            if let Ok(Value::Function(func)) = table.get::<Value>(key) {
                let event_table = self.event_table(event, message)?;
                let result = self
                    .with_timeout(|| func.call::<bool>(event_table))
                    .map_err(|err| Self::wrap_error(key, "event", &err))?;
                handled |= result;
            }
        }
        Ok(handled)
    }

    fn with_timeout<F, R>(&self, func: F) -> mlua::Result<R>
    where
        F: FnOnce() -> mlua::Result<R>,
    {
        let Some(max_exec_time) = self.max_exec_time else {
            return func();
        };
        let start = Instant::now();
        let triggers = HookTriggers::new().every_nth_instruction(self.hook_instruction_count);
        self.lua.set_hook(triggers, move |_lua, _debug| {
            if start.elapsed() > max_exec_time {
                Err(mlua::Error::RuntimeError("script timeout".into()))
            } else {
                Ok(VmState::Continue)
            }
        })?;
        let result = func();
        let _ = self
            .lua
            .set_hook(HookTriggers::new(), |_lua, _debug| Ok(VmState::Continue));
        result
    }

    fn with_script_name<F, R>(&self, name: &str, func: F) -> mlua::Result<R>
    where
        F: FnOnce() -> mlua::Result<R>,
    {
        let globals = self.lua.globals();
        globals.set("_SCRIPT_NAME", name)?;
        let result = func();
        let _ = globals.raw_remove("_SCRIPT_NAME");
        result
    }

    fn collect_global_keys(&self) -> mlua::Result<Vec<String>> {
        self.ensure_ready()?;
        let globals = self.lua.globals();
        let mut keys = Vec::new();
        for pair in globals.pairs::<Value, Value>() {
            let (key, _) = pair?;
            if let Value::String(value) = key {
                keys.push(value.to_str()?.to_string());
            }
        }
        Ok(keys)
    }

    fn register_builtin_api(&mut self) -> mlua::Result<()> {
        let func = self
            .lua
            .create_function(|lua, (name, func): (String, mlua::Function)| {
                let globals = lua.globals();
                let commands = if let Value::Table(table) = globals.get::<Value>("commands")? {
                    table
                } else {
                    let table = lua.create_table()?;
                    globals.set("commands", table.clone())?;
                    table
                };
                commands.set(name.clone(), func)?;
                if let Ok(Value::String(script)) = globals.get::<Value>("_SCRIPT_NAME") {
                    let by_script = if let Value::Table(table) =
                        globals.get::<Value>("__tt_commands_by_script")?
                    {
                        table
                    } else {
                        let table = lua.create_table()?;
                        globals.set("__tt_commands_by_script", table.clone())?;
                        table
                    };
                    let key = script.to_str()?.to_string();
                    let list = if let Value::Table(table) = by_script.get::<Value>(key.clone())? {
                        table
                    } else {
                        let table = lua.create_table()?;
                        by_script.set(key, table.clone())?;
                        table
                    };
                    let idx = list.len()? + 1;
                    list.set(idx, name)?;
                }
                Ok(())
            })?;
        let globals = self.lua.globals();
        globals.set("register_command", func)?;
        Ok(())
    }

    fn remove_registered_commands(&self, name: &str) -> mlua::Result<()> {
        self.ensure_ready()?;
        let globals = self.lua.globals();
        let Value::Table(by_script) = globals.get::<Value>("__tt_commands_by_script")? else {
            return Ok(());
        };
        let Ok(Value::Table(list)) = by_script.get::<Value>(name) else {
            return Ok(());
        };
        let Value::Table(commands) = globals.get::<Value>("commands")? else {
            return Ok(());
        };
        for pair in list.sequence_values::<String>() {
            let cmd = pair?;
            let _ = commands.raw_remove(cmd);
        }
        let _ = by_script.raw_remove(name);
        Ok(())
    }

    fn wrap_error(name: &str, context: &str, err: &mlua::Error) -> mlua::Error {
        mlua::Error::RuntimeError(format!("lua {context} error ({name}): {err}"))
    }

    fn ensure_ready(&self) -> mlua::Result<()> {
        if let Some(err) = &self.init_error {
            return Err(mlua::Error::RuntimeError(format!("lua init error: {err}")));
        }
        Ok(())
    }
}

#[path = "scripts_tables.rs"]
mod tables;
impl Default for ScriptManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "scripts")]
fn event_name(event: Event) -> &'static str {
    match event {
        Event::None => "None",
        Event::ConnectSuccess => "ConnectSuccess",
        Event::ConnectCryptError => "ConnectCryptError",
        Event::ConnectFailed => "ConnectFailed",
        Event::ConnectionLost => "ConnectionLost",
        Event::ConnectMaxPayloadUpdated => "ConnectMaxPayloadUpdated",
        Event::CmdProcessing => "CmdProcessing",
        Event::CmdError => "CmdError",
        Event::CmdSuccess => "CmdSuccess",
        Event::MySelfLoggedIn => "MySelfLoggedIn",
        Event::MySelfLoggedOut => "MySelfLoggedOut",
        Event::MySelfKicked => "MySelfKicked",
        Event::UserLoggedIn => "UserLoggedIn",
        Event::UserLoggedOut => "UserLoggedOut",
        Event::UserUpdate => "UserUpdate",
        Event::UserJoined => "UserJoined",
        Event::UserLeft => "UserLeft",
        Event::TextMessage => "TextMessage",
        Event::ChannelCreated => "ChannelCreated",
        Event::ChannelUpdated => "ChannelUpdated",
        Event::ChannelRemoved => "ChannelRemoved",
        Event::ServerUpdate => "ServerUpdate",
        Event::ServerStatistics => "ServerStatistics",
        Event::FileNew => "FileNew",
        Event::FileRemove => "FileRemove",
        Event::UserAccount => "UserAccount",
        Event::BannedUser => "BannedUser",
        Event::UserAccountCreated => "UserAccountCreated",
        Event::UserAccountRemoved => "UserAccountRemoved",
        Event::UserStateChange => "UserStateChange",
        Event::VideoCaptureFrame => "VideoCaptureFrame",
        Event::MediaFileVideo => "MediaFileVideo",
        Event::DesktopWindow => "DesktopWindow",
        Event::DesktopCursor => "DesktopCursor",
        Event::DesktopInput => "DesktopInput",
        Event::UserRecordMediaFile => "UserRecordMediaFile",
        Event::AudioBlock => "AudioBlock",
        Event::InternalError => "InternalError",
        Event::VoiceActivation => "VoiceActivation",
        Event::Hotkey => "Hotkey",
        Event::HotkeyTest => "HotkeyTest",
        Event::FileTransfer => "FileTransfer",
        Event::DesktopWindowTransfer => "DesktopWindowTransfer",
        Event::StreamMediaFile => "StreamMediaFile",
        Event::LocalMediaFile => "LocalMediaFile",
        Event::AudioInput => "AudioInput",
        Event::UserFirstVoiceStreamPacket => "UserFirstVoiceStreamPacket",
        Event::SoundDeviceAdded => "SoundDeviceAdded",
        Event::SoundDeviceRemoved => "SoundDeviceRemoved",
        Event::SoundDeviceUnplugged => "SoundDeviceUnplugged",
        Event::SoundDeviceNewDefaultInput => "SoundDeviceNewDefaultInput",
        Event::SoundDeviceNewDefaultOutput => "SoundDeviceNewDefaultOutput",
        Event::SoundDeviceNewDefaultInputComDevice => "SoundDeviceNewDefaultInputComDevice",
        Event::SoundDeviceNewDefaultOutputComDevice => "SoundDeviceNewDefaultOutputComDevice",
        Event::BeforeReconnect { .. } => "BeforeReconnect",
        Event::Reconnecting { .. } => "Reconnecting",
        Event::AfterReconnect { .. } => "AfterReconnect",
        Event::ReconnectFailed { .. } => "ReconnectFailed",
        Event::BeforeAutoLogin { .. } => "BeforeAutoLogin",
        Event::AutoLoginFailed { .. } => "AutoLoginFailed",
        Event::BeforeAutoJoin { .. } => "BeforeAutoJoin",
        Event::AutoJoinFailed { .. } => "AutoJoinFailed",
        Event::AutoRecoverCompleted { .. } => "AutoRecoverCompleted",
        Event::Unknown(_) => "Unknown",
    }
}

#[cfg(feature = "scripts")]
fn diff_globals(before: &[String], after: &[String]) -> Vec<String> {
    let mut added = Vec::new();
    let excluded = [
        "_SCRIPT_NAME",
        "register_command",
        "commands",
        "__tt_commands_by_script",
    ];
    for key in after {
        if excluded.iter().any(|value| *value == key) {
            continue;
        }
        if !before.iter().any(|existing| existing == key) {
            added.push(key.clone());
        }
    }
    added
}
