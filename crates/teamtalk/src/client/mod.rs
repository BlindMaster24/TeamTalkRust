//! Core client type and message wrapper.
use crate::events::{ConnectionState, Error, Event, Result};
use std::cell::{Cell, RefCell};
pub use teamtalk_sys as ffi;

pub mod audio;
pub mod channels;
pub mod connection;
pub mod core;
pub mod desktop;
pub mod encryption;
pub mod files;
pub mod hooks;
pub mod hotkeys;
pub mod media;
pub mod recording;
pub mod server;
pub mod system;
pub mod users;
pub mod video;

pub use connection::{ConnectParams, ConnectParamsOwned, ReconnectConfig, ReconnectHandler};
pub use hooks::ClientHooks;

pub struct Client {
    /// Optional client name used by the SDK.
    pub name: Option<String>,
    ptr: *mut ffi::TTInstance,
    state: Cell<ConnectionState>,
    hooks: RefCell<ClientHooks>,
    auto_reconnect: RefCell<AutoReconnectState>,
}

unsafe impl Send for Client {}

impl Client {
    /// Creates a new polling client and loads the SDK.
    pub fn new() -> Result<Self> {
        crate::init()?;
        let ptr = unsafe { ffi::api().TT_InitTeamTalkPoll() };
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr,
                state: Cell::new(ConnectionState::Idle),
                hooks: RefCell::new(ClientHooks::default()),
                auto_reconnect: RefCell::new(AutoReconnectState::default()),
            })
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    /// Creates a client bound to a Windows message window.
    ///
    /// # Safety
    /// The caller must ensure `hwnd` and `msg` are valid for the target window.
    pub unsafe fn with_hwnd(hwnd: ffi::HWND, msg: u32) -> Result<Self> {
        crate::init()?;
        let ptr = unsafe { ffi::api().TT_InitTeamTalk(hwnd, msg) };
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr,
                state: Cell::new(ConnectionState::Idle),
                hooks: RefCell::new(ClientHooks::default()),
                auto_reconnect: RefCell::new(AutoReconnectState::default()),
            })
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[cfg(windows)]
    /// Swaps the window handle used by the client.
    ///
    /// # Safety
    /// The caller must ensure `hwnd` is valid for the target window.
    pub unsafe fn swap_hwnd(&self, hwnd: ffi::HWND) -> bool {
        unsafe { ffi::api().TT_SwapTeamTalkHWND(self.ptr, hwnd) == 1 }
    }

    /// Sets the client name used for login.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Returns the current connection state.
    pub fn connection_state(&self) -> ConnectionState {
        self.state.get()
    }

    /// Replaces the current hook set.
    pub fn set_hooks(&self, hooks: ClientHooks) {
        *self.hooks.borrow_mut() = hooks;
    }

    /// Clears all hooks.
    pub fn clear_hooks(&self) {
        *self.hooks.borrow_mut() = ClientHooks::default();
    }

    pub(crate) fn set_connection_state(&self, state: ConnectionState) {
        self.state.set(state);
    }

    pub(crate) fn invoke_hooks(&self, event: crate::events::Event, msg: &Message) {
        self.hooks.borrow_mut().fire(self, event, msg);
    }

    pub(crate) fn invoke_joined_hook(&self, channel_id: crate::types::ChannelId) {
        self.hooks.borrow_mut().fire_joined(self, channel_id);
    }

    pub(crate) fn handle_auto_reconnect(&self) {
        if self.state.get() != ConnectionState::Disconnected {
            return;
        }

        let mut auto = self.auto_reconnect.borrow_mut();
        if !auto.enabled {
            return;
        }

        let params = match auto.params.as_ref() {
            Some(params) => params.clone(),
            None => return,
        };

        let handler = match auto.handler.as_mut() {
            Some(handler) => handler,
            None => return,
        };

        if handler.can_attempt() {
            handler.record_attempt();
            let attempt = handler.attempts();
            let delay = handler.current_delay();
            let msg = Message::from_raw(unsafe { std::mem::zeroed::<ffi::TTMessage>() });
            self.invoke_hooks(Event::Reconnecting { attempt, delay }, &msg);
            let _ = self.connect(&params.host, params.tcp, params.udp, params.encrypted);
        }
    }

    /// Sends a debug input tone to the SDK.
    pub fn dbg_set_input_tone(&self, stream_types: u32, freq: i32) -> bool {
        unsafe { ffi::api().TT_DBG_SetSoundInputTone(self.ptr, stream_types, freq) == 1 }
    }

    /// Writes a debug tone into an audio file.
    pub fn dbg_write_audio_file_tone(&self, file_path: &str, freq: i32) -> bool {
        let mut info = unsafe { std::mem::zeroed::<ffi::MediaFileInfo>() };
        let p = crate::utils::ToTT::tt(file_path);
        unsafe {
            std::ptr::copy_nonoverlapping(
                p.as_ptr(),
                info.szFileName.as_mut_ptr(),
                p.len().min(511),
            );
            ffi::api().TT_DBG_WriteAudioFileTone(&info, freq) == 1
        }
    }

    /// Returns the SDK-reported size for a TeamTalk type.
    pub fn dbg_sizeof(n_type: ffi::TTType) -> i32 {
        unsafe { ffi::api().TT_DBG_SIZEOF(n_type) }
    }

    /// Returns a data pointer for a TeamTalk message.
    pub fn dbg_get_data_ptr(msg: &mut ffi::TTMessage) -> *mut std::ffi::c_void {
        unsafe { ffi::api().TT_DBG_GETDATAPTR(msg) }
    }
}

#[derive(Default)]
pub(crate) struct AutoReconnectState {
    enabled: bool,
    handler: Option<ReconnectHandler>,
    params: Option<ConnectParamsOwned>,
}

/// Wrapper around a raw TeamTalk message.
pub struct Message(ffi::TTMessage);

impl Message {
    /// Wraps a raw TeamTalk message.
    pub(crate) fn from_raw(raw: ffi::TTMessage) -> Self {
        Self(raw)
    }

    /// Returns the source user id for the message.
    pub fn source(&self) -> i32 {
        self.0.nSource
    }

    /// Returns the text message payload if present.
    pub fn text(&self) -> Option<crate::types::TextMessage> {
        unsafe {
            Some(crate::types::TextMessage::from(
                self.0.__bindgen_anon_1.textmessage,
            ))
        }
    }

    /// Returns the user payload if present.
    pub fn user(&self) -> Option<crate::types::User> {
        unsafe { Some(crate::types::User::from(self.0.__bindgen_anon_1.user)) }
    }

    /// Returns the user account payload if present.
    pub fn account(&self) -> Option<crate::types::UserAccount> {
        unsafe {
            Some(crate::types::UserAccount::from(
                self.0.__bindgen_anon_1.useraccount,
            ))
        }
    }

    /// Returns the raw TeamTalk message.
    pub fn raw(&self) -> &ffi::TTMessage {
        &self.0
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            ffi::api().TT_CloseTeamTalk(self.ptr);
        }
    }
}
