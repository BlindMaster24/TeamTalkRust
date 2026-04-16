use crate::Client;
use crate::client::Message;
use crate::events::Event;
use libloading::Library;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginFlow {
    Continue,
    Stop,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("plugin init failed: {0}")]
    InitFailed(String),
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("library error: {0}")]
    LibraryError(String),
}

pub trait TeamTalkPlugin: Send {
    fn on_load(&mut self, _client: &Client) -> Result<(), PluginError> {
        Ok(())
    }
    fn on_unload(&mut self) {}
    fn on_event(&mut self, _event: Event, _message: &Message) -> PluginFlow {
        PluginFlow::Continue
    }
    fn on_command(&mut self, _command: &str, _args: &[String]) -> PluginFlow {
        PluginFlow::Continue
    }
}

struct PluginEntry {
    plugin: Box<dyn TeamTalkPlugin>,
    path: Option<PathBuf>,
    _lib: Option<Library>,
}

type PluginInitFn = unsafe extern "C" fn() -> i32;
type PluginHandleEventFn =
    unsafe extern "C" fn(teamtalk_sys::ClientEvent, *const teamtalk_sys::TTMessage) -> i32;
type PluginShutdownFn = unsafe extern "C" fn();

struct DynamicPlugin {
    _lib: Library,
    handle_event_fn: Option<PluginHandleEventFn>,
    shutdown_fn: Option<PluginShutdownFn>,
}

impl DynamicPlugin {
    fn new(lib: Library) -> Result<Self, PluginError> {
        unsafe {
            let init_fn: Result<libloading::Symbol<PluginInitFn>, _> = lib.get(b"tt_plugin_init");
            if let Ok(sym) = init_fn {
                let ret = sym();
                if ret != 0 {
                    return Err(PluginError::InitFailed(format!(
                        "tt_plugin_init returned {ret}"
                    )));
                }
            }
            let handle_event_fn = lib
                .get(b"tt_plugin_handle_event")
                .ok()
                .map(|sym: libloading::Symbol<PluginHandleEventFn>| *sym);
            let shutdown_fn = lib
                .get(b"tt_plugin_shutdown")
                .ok()
                .map(|sym: libloading::Symbol<PluginShutdownFn>| *sym);
            Ok(Self {
                _lib: lib,
                handle_event_fn,
                shutdown_fn,
            })
        }
    }
}

impl TeamTalkPlugin for DynamicPlugin {
    fn on_load(&mut self, _client: &Client) -> Result<(), PluginError> {
        Ok(())
    }

    fn on_unload(&mut self) {
        if let Some(shutdown_fn) = self.shutdown_fn {
            unsafe {
                shutdown_fn();
            }
        }
    }

    fn on_event(&mut self, _event: Event, message: &Message) -> PluginFlow {
        if let Some(handle_fn) = self.handle_event_fn {
            unsafe {
                let raw_event = message.raw().nClientEvent;
                let ret = handle_fn(raw_event, message.raw());
                if ret != 0 {
                    return PluginFlow::Stop;
                }
            }
        }
        PluginFlow::Continue
    }
}

pub struct PluginManager {
    plugins: HashMap<String, PluginEntry>,
}

impl PluginManager {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, name: &str, plugin: Box<dyn TeamTalkPlugin>) {
        self.plugins.insert(
            name.to_string(),
            PluginEntry {
                plugin,
                path: None,
                _lib: None,
            },
        );
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), PluginError> {
        let path_buf = path.to_path_buf();
        let lib = unsafe {
            Library::new(&path_buf).map_err(|e| PluginError::LibraryError(e.to_string()))?
        };
        let name = path_buf
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let dynamic = DynamicPlugin::new(lib)?;
        self.plugins.insert(
            name.clone(),
            PluginEntry {
                plugin: Box::new(dynamic),
                path: Some(path_buf),
                _lib: None,
            },
        );
        Ok(())
    }

    pub fn load(&mut self, name: &str, path: impl AsRef<Path>) -> Result<(), String> {
        let path_ref = path.as_ref();
        let path_buf = path_ref.to_path_buf();
        let lib = unsafe { Library::new(&path_buf).map_err(|e| e.to_string())? };
        let dynamic = DynamicPlugin::new(lib).map_err(|e| e.to_string())?;
        self.plugins.insert(
            name.to_string(),
            PluginEntry {
                plugin: Box::new(dynamic),
                path: Some(path_buf),
                _lib: None,
            },
        );
        Ok(())
    }

    pub fn unload(&mut self, name: &str) -> Result<(), PluginError> {
        let mut entry = self
            .plugins
            .remove(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
        entry.plugin.on_unload();
        Ok(())
    }

    pub fn reload(&mut self, name: &str) -> Result<(), PluginError> {
        let path = self
            .plugins
            .get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?
            .path
            .clone();
        let path = path.ok_or_else(|| {
            PluginError::LibraryError(format!("cannot reload static plugin '{name}'"))
        })?;
        self.unload(name)?;
        self.load_plugin(&path)
    }

    pub fn dispatch_event(&mut self, event: Event, message: &Message) -> PluginFlow {
        for entry in self.plugins.values_mut() {
            let flow = entry.plugin.on_event(event, message);
            if flow == PluginFlow::Stop {
                return PluginFlow::Stop;
            }
        }
        PluginFlow::Continue
    }

    pub fn dispatch_command(&mut self, command: &str, args: &[String]) -> PluginFlow {
        for entry in self.plugins.values_mut() {
            let flow = entry.plugin.on_command(command, args);
            if flow == PluginFlow::Stop {
                return PluginFlow::Stop;
            }
        }
        PluginFlow::Continue
    }

    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
