//! Core client type and message wrapper.
use crate::events::{ConnectionState, Error, Event, Result};
#[cfg(feature = "scripts")]
use crate::extensions::scripts::ScriptManager;
use crate::types::ClientId;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
pub use teamtalk_sys as ffi;

pub mod audio;
#[cfg(feature = "mock")]
pub mod backend;
#[cfg(not(feature = "mock"))]
pub(crate) mod backend;
pub mod bus;
pub mod cache;
pub mod channels;
pub mod connection;
pub mod core;
pub mod desktop;
pub mod encryption;
pub mod files;
pub mod hooks;
pub mod hotkeys;
pub mod manager;
pub mod media;
pub mod recording;
pub mod registry;
pub mod server;
pub mod system;
pub mod users;
pub mod video;

pub use bus::{EventContext, EventSubscriptionGroup, EventSubscriptionId, SubscriptionBuilder};
pub use cache::ServerInfo;
pub use connection::{ConnectParams, ConnectParamsOwned, ReconnectConfig, ReconnectHandler};
pub use hooks::ClientHooks;
pub use manager::{ClientEvent, ClientHealth, ClientManager};
pub use registry::{ClientInfo, ClientRegistry};

static NEXT_CLIENT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Client {
    /// Optional client name used by the SDK.
    pub name: Option<String>,
    ptr: *mut ffi::TTInstance,
    id: ClientId,
    backend: Arc<dyn backend::TeamTalkBackend>,
    label: RefCell<Option<String>>,
    state: Cell<ConnectionState>,
    hooks: RefCell<ClientHooks>,
    bus: RefCell<bus::EventBus>,
    #[cfg(feature = "scripts")]
    scripts: RefCell<Option<ScriptManager>>,
    auto_reconnect: RefCell<AutoReconnectState>,
    cache: RefCell<cache::CacheState>,
}

impl Client {
    /// Creates a new polling client and loads the SDK.
    pub fn new() -> Result<Self> {
        crate::init()?;
        let backend: Arc<dyn backend::TeamTalkBackend> = Arc::new(backend::FfiBackend);
        let ptr = backend.init_poll();
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr,
                id: ClientId(NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed)),
                backend,
                label: RefCell::new(None),
                state: Cell::new(ConnectionState::Idle),
                hooks: RefCell::new(ClientHooks::default()),
                bus: RefCell::new(bus::EventBus::default()),
                #[cfg(feature = "scripts")]
                scripts: RefCell::new(None),
                auto_reconnect: RefCell::new(AutoReconnectState::default()),
                cache: RefCell::new(cache::CacheState::default()),
            })
        }
    }

    #[cfg(windows)]
    /// Creates a client bound to a Windows message window.
    ///
    /// # Safety
    /// - `hwnd` must be a valid window handle for the lifetime of the client.
    /// - `msg` must be a valid message ID routed to `hwnd`.
    /// - The caller must ensure the window's message loop stays alive while the
    ///   client is in use.
    pub unsafe fn with_hwnd(hwnd: ffi::HWND, msg: u32) -> Result<Self> {
        crate::init()?;
        let backend: Arc<dyn backend::TeamTalkBackend> = Arc::new(backend::FfiBackend);
        let ptr = backend.init_hwnd(hwnd, msg);
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr,
                id: ClientId(NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed)),
                backend,
                label: RefCell::new(None),
                state: Cell::new(ConnectionState::Idle),
                hooks: RefCell::new(ClientHooks::default()),
                bus: RefCell::new(bus::EventBus::default()),
                #[cfg(feature = "scripts")]
                scripts: RefCell::new(None),
                auto_reconnect: RefCell::new(AutoReconnectState::default()),
                cache: RefCell::new(cache::CacheState::default()),
            })
        }
    }

    #[cfg(windows)]
    /// Swaps the window handle used by the client.
    ///
    /// # Safety
    /// - `hwnd` must be a valid window handle for the lifetime of the client.
    /// - The previous window handle must no longer be in use by this client.
    pub unsafe fn swap_hwnd(&self, hwnd: ffi::HWND) -> bool {
        unsafe { ffi::api().TT_SwapTeamTalkHWND(self.ptr, hwnd) == 1 }
    }

    pub(crate) fn backend(&self) -> &dyn backend::TeamTalkBackend {
        self.backend.as_ref()
    }

    #[cfg(feature = "mock")]
    pub fn with_backend(backend: Arc<dyn backend::TeamTalkBackend>) -> Result<Self> {
        let ptr = backend.init_poll();
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr,
                id: ClientId(NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed)),
                backend,
                label: RefCell::new(None),
                state: Cell::new(ConnectionState::Idle),
                hooks: RefCell::new(ClientHooks::default()),
                bus: RefCell::new(bus::EventBus::default()),
                #[cfg(feature = "scripts")]
                scripts: RefCell::new(None),
                auto_reconnect: RefCell::new(AutoReconnectState::default()),
                cache: RefCell::new(cache::CacheState::default()),
            })
        }
    }

    /// Sets the client name used for login.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets a human-friendly label for the client instance.
    pub fn with_label(self, label: &str) -> Self {
        *self.label.borrow_mut() = Some(label.to_string());
        self
    }

    /// Returns the client instance id.
    pub fn id(&self) -> ClientId {
        self.id
    }

    /// Returns the client label, if set.
    pub fn label(&self) -> Option<String> {
        self.label.borrow().clone()
    }

    /// Sets or clears the client label.
    pub fn set_label(&self, label: Option<&str>) {
        *self.label.borrow_mut() = label.map(|value| value.to_string());
    }

    /// Returns the current connection state.
    pub fn connection_state(&self) -> ConnectionState {
        self.state.get()
    }

    /// Creates a subscription for a specific event type.
    pub fn on_event(&self, event: Event) -> SubscriptionBuilder<'_> {
        SubscriptionBuilder::new(self, Some(event))
    }

    /// Creates a subscription for all events.
    pub fn on_any(&self) -> SubscriptionBuilder<'_> {
        SubscriptionBuilder::new(self, None)
    }

    /// Removes an event subscription.
    pub fn unsubscribe_event(&self, id: EventSubscriptionId) -> bool {
        self.bus.borrow_mut().unsubscribe(id)
    }

    /// Clears all event subscriptions.
    pub fn clear_event_subscriptions(&self) {
        self.bus.borrow_mut().clear();
    }

    /// Removes all subscriptions in the specified group.
    pub fn unsubscribe_event_group(&self, group: impl AsRef<str>) -> usize {
        let group = EventSubscriptionGroup::new(group.as_ref());
        self.bus.borrow_mut().unsubscribe_group(&group)
    }

    /// Returns the number of active event subscriptions.
    pub fn event_subscription_count(&self) -> usize {
        self.bus.borrow().len()
    }

    /// Replaces the current hook set.
    pub fn set_hooks(&self, hooks: ClientHooks) {
        *self.hooks.borrow_mut() = hooks;
    }

    /// Clears all hooks.
    pub fn clear_hooks(&self) {
        *self.hooks.borrow_mut() = ClientHooks::default();
    }

    #[cfg(feature = "scripts")]
    pub fn enable_scripts(&self) {
        let mut scripts = self.scripts.borrow_mut();
        if scripts.is_none() {
            *scripts = Some(ScriptManager::new());
        }
    }

    #[cfg(feature = "scripts")]
    pub fn set_script_manager(&self, manager: ScriptManager) {
        *self.scripts.borrow_mut() = Some(manager);
    }

    #[cfg(feature = "scripts")]
    pub fn clear_scripts(&self) {
        *self.scripts.borrow_mut() = None;
    }

    #[cfg(feature = "scripts")]
    pub fn scripts_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut ScriptManager) -> R,
    {
        self.scripts.borrow_mut().as_mut().map(f)
    }

    pub(crate) fn set_connection_state(&self, state: ConnectionState) {
        self.state.set(state);
    }

    pub(crate) fn invoke_hooks(&self, event: crate::events::Event, msg: &Message) {
        self.hooks.borrow_mut().fire(self, event, msg);
    }

    pub(crate) fn dispatch_bus(&self, event: crate::events::Event, msg: &Message) {
        self.bus.borrow_mut().dispatch(self, event, msg);
    }

    #[cfg(feature = "scripts")]
    pub(crate) fn dispatch_scripts(&self, event: crate::events::Event, msg: &Message) {
        if let Some(manager) = self.scripts.borrow().as_ref() {
            let _ = manager.handle_event(event, msg);
        }
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
            let attempt = handler.attempts() + 1;
            let delay = handler.current_delay();
            let before_event = Event::BeforeReconnect { attempt, delay };
            let msg = Message::from_raw(before_event, unsafe {
                std::mem::zeroed::<ffi::TTMessage>()
            });
            self.invoke_hooks(before_event, &msg);
            handler.record_attempt();
            self.invoke_hooks(Event::Reconnecting { attempt, delay }, &msg);
            let _ = self.connect(&params.host, params.tcp, params.udp, params.encrypted);
        } else {
            let attempts = handler.attempts();
            let failed_event = Event::ReconnectFailed { attempts };
            let msg = Message::from_raw(failed_event, unsafe {
                std::mem::zeroed::<ffi::TTMessage>()
            });
            self.invoke_hooks(failed_event, &msg);
            auto.enabled = false;
            auto.handler = None;
        }
    }

    pub(crate) fn handle_auto_login(&self) {
        if self.state.get() != ConnectionState::Connected {
            return;
        }

        let auto = self.auto_reconnect.borrow();
        if !auto.enabled {
            return;
        }

        let params = match auto.login.as_ref() {
            Some(params) => params,
            None => return,
        };

        let _ = self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        );
    }

    pub(crate) fn handle_auto_join(&self) {
        if self.state.get() != ConnectionState::LoggedIn {
            return;
        }

        let auto = self.auto_reconnect.borrow();
        if !auto.enabled {
            return;
        }

        let channel = match auto.last_channel {
            Some(channel) => channel,
            None => return,
        };

        let _ = self.join_channel(channel, "");
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
    last_channel: Option<crate::types::ChannelId>,
    login: Option<crate::client::users::LoginParams>,
}

/// Wrapper around a raw TeamTalk message with its originating event.
pub struct Message {
    event: crate::events::Event,
    raw: ffi::TTMessage,
}

impl Message {
    /// Wraps a raw TeamTalk message.
    pub(crate) fn from_raw(event: crate::events::Event, raw: ffi::TTMessage) -> Self {
        Self { event, raw }
    }

    /// Returns the originating event for this message.
    pub fn event(&self) -> crate::events::Event {
        self.event
    }

    /// Returns the source user id for the message.
    pub fn source(&self) -> i32 {
        self.raw.nSource
    }

    /// Returns the text message payload if present.
    pub fn text(&self) -> Option<crate::types::TextMessage> {
        if matches!(self.event, crate::events::Event::TextMessage) {
            unsafe {
                Some(crate::types::TextMessage::from(
                    self.raw.__bindgen_anon_1.textmessage,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the channel payload if present.
    pub fn channel(&self) -> Option<crate::types::Channel> {
        if matches!(
            self.event,
            crate::events::Event::ChannelCreated
                | crate::events::Event::ChannelUpdated
                | crate::events::Event::ChannelRemoved
        ) {
            unsafe {
                Some(crate::types::Channel::from(
                    self.raw.__bindgen_anon_1.channel,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the server properties payload if present.
    pub fn server_properties(&self) -> Option<crate::types::ServerProperties> {
        if matches!(self.event, crate::events::Event::ServerUpdate) {
            unsafe {
                Some(crate::types::ServerProperties::from(
                    self.raw.__bindgen_anon_1.serverproperties,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the server statistics payload if present.
    pub fn server_statistics(&self) -> Option<crate::types::ServerStatistics> {
        if matches!(self.event, crate::events::Event::ServerStatistics) {
            unsafe {
                Some(crate::types::ServerStatistics::from(
                    self.raw.__bindgen_anon_1.serverstatistics,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the file transfer payload if present.
    pub fn file_transfer(&self) -> Option<crate::types::FileTransfer> {
        if matches!(self.event, crate::events::Event::FileTransfer) {
            unsafe {
                Some(crate::types::FileTransfer::from(
                    self.raw.__bindgen_anon_1.filetransfer,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the user payload if present.
    pub fn user(&self) -> Option<crate::types::User> {
        if matches!(
            self.event,
            crate::events::Event::UserLoggedIn
                | crate::events::Event::UserLoggedOut
                | crate::events::Event::UserUpdate
                | crate::events::Event::UserJoined
                | crate::events::Event::UserLeft
                | crate::events::Event::UserStateChange
                | crate::events::Event::UserFirstVoiceStreamPacket
        ) {
            unsafe { Some(crate::types::User::from(self.raw.__bindgen_anon_1.user)) }
        } else {
            None
        }
    }

    /// Returns the user account payload if present.
    pub fn account(&self) -> Option<crate::types::UserAccount> {
        if matches!(
            self.event,
            crate::events::Event::UserAccount
                | crate::events::Event::UserAccountCreated
                | crate::events::Event::UserAccountRemoved
        ) {
            unsafe {
                Some(crate::types::UserAccount::from(
                    self.raw.__bindgen_anon_1.useraccount,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the raw TeamTalk message.
    pub fn raw(&self) -> &ffi::TTMessage {
        &self.raw
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.backend.close(self.ptr);
    }
}
