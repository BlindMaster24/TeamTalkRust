//! Core client type and message wrapper.
use crate::events::{ConnectionState, Error, Event, Result};
#[cfg(feature = "scripts")]
use crate::extensions::scripts::ScriptManager;
use crate::types::ClientId;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
pub use teamtalk_sys as ffi;

use super::bus;
#[cfg(feature = "state")]
use super::cache;
use super::hooks;

mod init;

static NEXT_CLIENT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug)]
pub struct TTPtr(pub *mut ffi::TTInstance);

unsafe impl Send for TTPtr {}
unsafe impl Sync for TTPtr {}

pub struct Client {
    /// Optional client name used by the SDK.
    pub name: Option<String>,
    pub(crate) ptr: TTPtr,
    pub(crate) id: ClientId,
    pub(crate) backend: Arc<dyn super::backend::TeamTalkBackend>,
    pub(crate) label: Mutex<Option<String>>,
    pub(crate) state: Mutex<ConnectionState>,
    pub(crate) hooks: Mutex<hooks::ClientHooks>,
    pub(crate) hooks_revision: AtomicU64,
    pub(crate) bus: Mutex<bus::EventBus>,
    pub(crate) bus_revision: AtomicU64,
    #[cfg(feature = "scripts")]
    pub(crate) scripts: Mutex<Option<ScriptManager>>,
    pub(crate) auto_reconnect: Mutex<AutoReconnectState>,
    #[cfg(feature = "state")]
    pub(crate) cache: Mutex<cache::CacheState>,
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

/// A split interface for handling client events (polling).
pub struct ClientEvents(pub Arc<Client>);

impl ClientEvents {
    /// Polls the client for the next event.
    pub fn poll(&self, timeout_ms: i32) -> Option<(Event, Message)> {
        self.0.poll(timeout_ms)
    }
}

/// A split interface for issuing client commands.
#[derive(Clone)]
pub struct ClientCommands(pub Arc<Client>);

impl std::ops::Deref for ClientCommands {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Client {
    pub(crate) fn with_initialized_backend(
        backend: Arc<dyn super::backend::TeamTalkBackend>,
        ptr: *mut ffi::TTInstance,
    ) -> Result<Self> {
        if ptr.is_null() {
            Err(Error::InitFailed)
        } else {
            Ok(Self {
                name: None,
                ptr: TTPtr(ptr),
                id: ClientId(NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed)),
                backend,
                label: Mutex::new(None),
                state: Mutex::new(ConnectionState::Idle),
                hooks: Mutex::new(hooks::ClientHooks::default()),
                hooks_revision: AtomicU64::new(0),
                bus: Mutex::new(bus::EventBus::default()),
                bus_revision: AtomicU64::new(0),
                #[cfg(feature = "scripts")]
                scripts: Mutex::new(None),
                auto_reconnect: Mutex::new(AutoReconnectState::default()),
                #[cfg(feature = "state")]
                cache: Mutex::new(cache::CacheState::default()),
            })
        }
    }

    /// Creates a new polling client and loads the SDK.
    pub fn new() -> Result<Self> {
        init::new_client()
    }

    /// Splits the client into event polling and command execution parts.
    pub fn split(self) -> (ClientEvents, ClientCommands) {
        let shared = Arc::new(self);
        (ClientEvents(shared.clone()), ClientCommands(shared))
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
        unsafe { init::new_client_with_hwnd(hwnd, msg) }
    }

    #[cfg(windows)]
    /// Swaps the window handle used by the client.
    ///
    /// # Safety
    /// - `hwnd` must be a valid window handle for the lifetime of the client.
    /// - The previous window handle must no longer be in use by this client.
    pub unsafe fn swap_hwnd(&self, hwnd: ffi::HWND) -> bool {
        unsafe { ffi::api().TT_SwapTeamTalkHWND(self.ptr.0, hwnd) == 1 }
    }

    pub(crate) fn backend(&self) -> &dyn super::backend::TeamTalkBackend {
        self.backend.as_ref()
    }

    #[cfg(feature = "mock")]
    pub fn with_backend(backend: Arc<dyn super::backend::TeamTalkBackend>) -> Result<Self> {
        init::new_client_with_backend(backend)
    }

    #[cfg(feature = "mock")]
    pub fn mock_set_connection_state_for_tests(&self, state: ConnectionState) {
        self.set_connection_state(state);
    }

    #[cfg(feature = "mock")]
    pub fn mock_set_pending_commands_for_tests(&self, login: Option<i32>, join: Option<i32>) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.pending_login_cmd = login;
        auto.pending_join_cmd = join;
    }

    #[cfg(feature = "mock")]
    pub fn mock_pending_commands_for_tests(&self) -> (Option<i32>, Option<i32>) {
        let auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        (auto.pending_login_cmd, auto.pending_join_cmd)
    }

    #[cfg(feature = "mock")]
    pub fn mock_last_channel_password_for_tests(&self) -> Option<String> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_channel_password
            .clone()
    }

    #[cfg(feature = "mock")]
    pub fn mock_apply_event_for_tests(&self, event: Event, source: i32) {
        let mut raw = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        raw.nSource = source;
        let message = Message::from_raw(event, raw);
        self.update_state_for_event(event, &message);
    }

    /// Sets the client name used for login.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets a human-friendly label for the client instance.
    pub fn with_label(self, label: &str) -> Self {
        *self.label.lock().unwrap_or_else(|e| e.into_inner()) = Some(label.to_string());
        self
    }

    /// Returns the client instance id.
    pub fn id(&self) -> ClientId {
        self.id
    }

    /// Returns the client label, if set.
    pub fn label(&self) -> Option<String> {
        self.label.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    /// Sets or clears the client label.
    pub fn set_label(&self, label: Option<&str>) {
        *self.label.lock().unwrap_or_else(|e| e.into_inner()) =
            label.map(|value| value.to_string());
    }

    /// Returns the current connection state.
    pub fn connection_state(&self) -> ConnectionState {
        *self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Creates a subscription for a specific event type.
    pub fn on_event(&self, event: Event) -> bus::SubscriptionBuilder<'_> {
        bus::SubscriptionBuilder::new(self, Some(event))
    }

    /// Creates a subscription for all events.
    pub fn on_any(&self) -> bus::SubscriptionBuilder<'_> {
        bus::SubscriptionBuilder::new(self, None)
    }

    /// Removes an event subscription.
    pub fn unsubscribe_event(&self, id: bus::EventSubscriptionId) -> bool {
        let removed = self
            .bus
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .unsubscribe(id);
        if removed {
            self.bus_revision.fetch_add(1, Ordering::Relaxed);
        }
        removed
    }

    /// Clears all event subscriptions.
    pub fn clear_event_subscriptions(&self) {
        let mut bus = self.bus.lock().unwrap_or_else(|e| e.into_inner());
        if bus.len() > 0 {
            bus.clear();
            self.bus_revision.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Removes all subscriptions in the specified group.
    pub fn unsubscribe_event_group(&self, group: impl AsRef<str>) -> usize {
        let group = bus::EventSubscriptionGroup::new(group.as_ref());
        let removed = self
            .bus
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .unsubscribe_group(&group);
        if removed > 0 {
            self.bus_revision.fetch_add(1, Ordering::Relaxed);
        }
        removed
    }

    /// Returns the number of active event subscriptions.
    pub fn event_subscription_count(&self) -> usize {
        self.bus.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// Replaces the current hook set.
    pub fn set_hooks(&self, hooks: hooks::ClientHooks) {
        *self.hooks.lock().unwrap_or_else(|e| e.into_inner()) = hooks;
        self.hooks_revision.fetch_add(1, Ordering::Relaxed);
    }

    /// Clears all hooks.
    pub fn clear_hooks(&self) {
        *self.hooks.lock().unwrap_or_else(|e| e.into_inner()) = hooks::ClientHooks::default();
        self.hooks_revision.fetch_add(1, Ordering::Relaxed);
    }

    #[cfg(feature = "scripts")]
    pub fn enable_scripts(&self) {
        let mut scripts = self.scripts.lock().unwrap_or_else(|e| e.into_inner());
        if scripts.is_none() {
            *scripts = Some(ScriptManager::new());
        }
    }

    #[cfg(feature = "scripts")]
    pub fn set_script_manager(&self, manager: ScriptManager) {
        *self.scripts.lock().unwrap_or_else(|e| e.into_inner()) = Some(manager);
    }

    #[cfg(feature = "scripts")]
    pub fn clear_scripts(&self) {
        *self.scripts.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }

    #[cfg(feature = "scripts")]
    pub fn scripts_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut ScriptManager) -> R,
    {
        self.scripts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
            .map(f)
    }

    pub(crate) fn set_connection_state(&self, state: ConnectionState) {
        *self.state.lock().unwrap_or_else(|e| e.into_inner()) = state;
    }

    pub(crate) fn invoke_hooks(&self, event: crate::events::Event, msg: &Message) {
        let revision_before = self.hooks_revision.load(Ordering::Relaxed);
        let mut local_hooks = {
            let mut hooks = self.hooks.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *hooks)
        };
        local_hooks.fire(self, event, msg);
        if self.hooks_revision.load(Ordering::Relaxed) == revision_before {
            *self.hooks.lock().unwrap_or_else(|e| e.into_inner()) = local_hooks;
        }
    }

    pub(crate) fn dispatch_bus(&self, event: crate::events::Event, msg: &Message) {
        let revision_before = self.bus_revision.load(Ordering::Relaxed);
        let mut local_bus = {
            let mut bus = self.bus.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *bus)
        };
        local_bus.dispatch(self, event, msg);
        if self.bus_revision.load(Ordering::Relaxed) == revision_before {
            *self.bus.lock().unwrap_or_else(|e| e.into_inner()) = local_bus;
        }
    }

    #[cfg(feature = "scripts")]
    pub(crate) fn dispatch_scripts(&self, event: crate::events::Event, msg: &Message) {
        if let Some(manager) = self
            .scripts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            let _ = manager.handle_event(event, msg);
        }
    }

    pub(crate) fn invoke_joined_hook(&self, channel_id: crate::types::ChannelId) {
        let revision_before = self.hooks_revision.load(Ordering::Relaxed);
        let mut local_hooks = {
            let mut hooks = self.hooks.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *hooks)
        };
        local_hooks.fire_joined(self, channel_id);
        if self.hooks_revision.load(Ordering::Relaxed) == revision_before {
            *self.hooks.lock().unwrap_or_else(|e| e.into_inner()) = local_hooks;
        }
    }

    pub(crate) fn handle_auto_reconnect(&self) {
        match *self.state.lock().unwrap_or_else(|e| e.into_inner()) {
            ConnectionState::Disconnected => self.handle_connect_recovery(),
            ConnectionState::Connected => self.handle_login_recovery(),
            ConnectionState::LoggedIn => self.handle_join_recovery(),
            ConnectionState::Joined(_) => self.handle_recovery_completed(),
            _ => {}
        }
    }

    fn empty_message(event: Event) -> Message {
        Message::from_raw(event, unsafe { std::mem::zeroed::<ffi::TTMessage>() })
    }

    fn handle_connect_recovery(&self) {
        let params = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled {
                return;
            }

            let params: super::connection::ConnectParamsOwned = match auto.params.as_ref() {
                Some(params) => params.clone(),
                None => return,
            };

            let handler: &mut super::connection::ReconnectHandler = match auto.handler.as_mut() {
                Some(handler) => handler,
                None => return,
            };

            if handler.can_attempt() {
                params
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                drop(auto);
                let failed_event = Event::ReconnectFailed { attempts };
                let msg = Self::empty_message(failed_event);
                self.invoke_hooks(failed_event, &msg);

                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                auto.enabled = false;
                auto.handler = None;
                auto.login_handler = None;
                auto.join_handler = None;
                return;
            } else {
                return;
            }
        };

        let _ = self.disconnect();
        if self.has_connection_flags() {
            return;
        }

        let (attempt, delay) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled {
                return;
            }

            let handler = match auto.handler.as_mut() {
                Some(handler) => handler,
                None => return,
            };
            if !handler.can_attempt() {
                return;
            }

            let attempt = handler.attempts() + 1;
            let delay = handler.current_delay();
            handler.record_attempt();
            auto.force_disconnect = false;
            auto.login_gave_up = false;
            auto.join_gave_up = false;
            auto.recovery_completed = false;
            (attempt, delay)
        };

        let before_event = Event::BeforeReconnect { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);
        self.invoke_hooks(Event::Reconnecting { attempt, delay }, &msg);
        let _ = self.connect(&params.host, params.tcp, params.udp, params.encrypted);
    }

    fn handle_login_recovery(&self) {
        let (params, attempt, delay, gave_up_now) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.login_gave_up {
                return;
            }

            let params = match auto.login.as_ref() {
                Some(params) => params.clone(),
                None => return,
            };

            let login_config = auto.workflow.login.clone();
            let handler = auto
                .login_handler
                .get_or_insert_with(|| super::connection::ReconnectHandler::new(login_config));
            if handler.can_attempt() {
                let attempt = handler.attempts() + 1;
                let delay = handler.current_delay();
                handler.record_attempt();
                (params, attempt, delay, false)
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                let delay = handler.current_delay();
                auto.login_gave_up = true;
                (params, attempts, delay, true)
            } else {
                return;
            }
        };

        if gave_up_now {
            let event = Event::AutoLoginFailed { attempts: attempt };
            let msg = Self::empty_message(event);
            self.invoke_hooks(event, &msg);
            return;
        }

        let before_event = Event::BeforeAutoLogin { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);

        let cmd_id = self.login(
            &params.nickname,
            &params.username,
            &params.password,
            &params.client_name,
        );
        if cmd_id <= 0 {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_login_cmd = None;
            drop(auto);
            self.set_connection_state(ConnectionState::Connected);
            let failed = Event::AutoLoginFailed { attempts: attempt };
            let msg = Self::empty_message(failed);
            self.invoke_hooks(failed, &msg);
        } else {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_login_cmd = Some(cmd_id);
        }
    }

    fn handle_join_recovery(&self) {
        let (channel, password, attempt, delay, gave_up_now) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.join_gave_up {
                return;
            }

            let channel = match auto.last_channel {
                Some(channel) => channel,
                None => return,
            };
            let password = auto.last_channel_password.clone();

            let join_config = auto.workflow.join.clone();
            let handler = auto
                .join_handler
                .get_or_insert_with(|| super::connection::ReconnectHandler::new(join_config));
            if handler.can_attempt() {
                let attempt = handler.attempts() + 1;
                let delay = handler.current_delay();
                handler.record_attempt();
                (channel, password, attempt, delay, false)
            } else if handler.exhausted() {
                let attempts = handler.attempts();
                let delay = handler.current_delay();
                auto.join_gave_up = true;
                (channel, password, attempts, delay, true)
            } else {
                return;
            }
        };

        if gave_up_now {
            let event = Event::AutoJoinFailed { attempts: attempt };
            let msg = Self::empty_message(event);
            self.invoke_hooks(event, &msg);
            return;
        }

        let before_event = Event::BeforeAutoJoin { attempt, delay };
        let msg = Self::empty_message(before_event);
        self.invoke_hooks(before_event, &msg);

        let cmd_id = self.join_channel(channel, password.as_deref().unwrap_or(""));
        if cmd_id <= 0 {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_join_cmd = None;
            drop(auto);
            self.set_connection_state(ConnectionState::LoggedIn);
            let failed = Event::AutoJoinFailed { attempts: attempt };
            let msg = Self::empty_message(failed);
            self.invoke_hooks(failed, &msg);
        } else {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            auto.pending_join_cmd = Some(cmd_id);
        }
    }

    fn handle_recovery_completed(&self) {
        let (reconnect_attempts, login_attempts, join_attempts) = {
            let mut auto = self
                .auto_reconnect
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if !auto.enabled || auto.recovery_completed {
                return;
            }

            auto.recovery_completed = true;
            auto.login_gave_up = false;
            auto.join_gave_up = false;
            let reconnect_attempts = auto
                .handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            let login_attempts = auto
                .login_handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            let join_attempts = auto
                .join_handler
                .as_ref()
                .map_or(0, |handler| handler.attempts());
            if let Some(handler) = auto.login_handler.as_mut() {
                handler.reset();
            }
            if let Some(handler) = auto.join_handler.as_mut() {
                handler.reset();
            }
            (reconnect_attempts, login_attempts, join_attempts)
        };

        let event = Event::AutoRecoverCompleted {
            reconnect_attempts,
            login_attempts,
            join_attempts,
        };
        let msg = Self::empty_message(event);
        self.invoke_hooks(event, &msg);
    }

    /// Sends a debug input tone to the SDK.
    pub fn dbg_set_input_tone(&self, stream_types: u32, freq: i32) -> bool {
        unsafe { ffi::api().TT_DBG_SetSoundInputTone(self.ptr.0, stream_types, freq) == 1 }
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
    pub(crate) enabled: bool,
    pub(crate) handler: Option<super::connection::ReconnectHandler>,
    pub(crate) login_handler: Option<super::connection::ReconnectHandler>,
    pub(crate) join_handler: Option<super::connection::ReconnectHandler>,
    pub(crate) params: Option<super::connection::ConnectParamsOwned>,
    pub(crate) last_channel: Option<crate::types::ChannelId>,
    pub(crate) last_channel_password: Option<String>,
    pub(crate) login: Option<super::users::LoginParams>,
    pub(crate) workflow: super::connection::ReconnectWorkflowConfig,
    pub(crate) login_gave_up: bool,
    pub(crate) join_gave_up: bool,
    pub(crate) recovery_completed: bool,
    pub(crate) pending_login_cmd: Option<i32>,
    pub(crate) pending_join_cmd: Option<i32>,
    pub(crate) extra_events: Vec<crate::events::Event>,
    pub(crate) force_disconnect: bool,
}

/// Wrapper around a raw TeamTalk message with its originating event.
pub struct Message {
    event: crate::events::Event,
    raw: ffi::TTMessage,
}

/// Typed payload extracted from a TeamTalk message.
pub enum EventData {
    TextMessage(crate::types::TextMessage),
    Channel(crate::types::Channel),
    ServerProperties(crate::types::ServerProperties),
    ServerStatistics(crate::types::ServerStatistics),
    FileTransfer(crate::types::FileTransfer),
    User(crate::types::User),
    UserAccount(crate::types::UserAccount),
    ErrorMessage(crate::types::ErrorMessage),
}

impl Message {
    fn has_tt_type(&self, expected: ffi::TTType) -> bool {
        self.raw.ttType == expected
    }

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
        if matches!(self.event, crate::events::Event::TextMessage)
            && self.has_tt_type(ffi::TTType::__TEXTMESSAGE)
        {
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
        ) && self.has_tt_type(ffi::TTType::__CHANNEL)
        {
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
        if matches!(self.event, crate::events::Event::ServerUpdate)
            && self.has_tt_type(ffi::TTType::__SERVERPROPERTIES)
        {
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
        if matches!(self.event, crate::events::Event::ServerStatistics)
            && self.has_tt_type(ffi::TTType::__SERVERSTATISTICS)
        {
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
        if matches!(self.event, crate::events::Event::FileTransfer)
            && self.has_tt_type(ffi::TTType::__FILETRANSFER)
        {
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
                | crate::events::Event::MySelfKicked
                | crate::events::Event::UserFirstVoiceStreamPacket
        ) && self.has_tt_type(ffi::TTType::__USER)
        {
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
        ) && self.has_tt_type(ffi::TTType::__USERACCOUNT)
        {
            unsafe {
                Some(crate::types::UserAccount::from(
                    self.raw.__bindgen_anon_1.useraccount,
                ))
            }
        } else {
            None
        }
    }

    /// Returns the SDK error payload if present.
    pub fn error_message(&self) -> Option<crate::types::ErrorMessage> {
        if matches!(
            self.event,
            crate::events::Event::ConnectCryptError
                | crate::events::Event::CmdError
                | crate::events::Event::InternalError
        ) && self.has_tt_type(ffi::TTType::__CLIENTERRORMSG)
        {
            unsafe {
                Some(crate::types::ErrorMessage::from(
                    self.raw.__bindgen_anon_1.clienterrormsg,
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

    /// Returns typed payload for the current event, if available.
    pub fn data(&self) -> Option<EventData> {
        self.text()
            .map(EventData::TextMessage)
            .or_else(|| self.channel().map(EventData::Channel))
            .or_else(|| self.server_properties().map(EventData::ServerProperties))
            .or_else(|| self.server_statistics().map(EventData::ServerStatistics))
            .or_else(|| self.file_transfer().map(EventData::FileTransfer))
            .or_else(|| self.user().map(EventData::User))
            .or_else(|| self.account().map(EventData::UserAccount))
            .or_else(|| self.error_message().map(EventData::ErrorMessage))
    }

    /// Returns the text message payload when this message carries it.
    pub fn try_as_text_message(&self) -> Option<crate::types::TextMessage> {
        self.text()
    }

    /// Returns the user payload when this message carries it.
    pub fn try_as_user(&self) -> Option<crate::types::User> {
        self.user()
    }

    /// Returns the channel payload when this message carries it.
    pub fn try_as_channel(&self) -> Option<crate::types::Channel> {
        self.channel()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.backend.close(self.ptr.0);
    }
}

impl Client {
    /// Returns the raw TeamTalk instance pointer.
    pub fn raw_ptr(&self) -> *mut ffi::TTInstance {
        self.ptr.0
    }

    /// Returns the SDK version string.
    pub fn version() -> String {
        let _ = crate::init();
        unsafe {
            let ptr = ffi::api().TT_GetVersion();
            if ptr.is_null() {
                "Unknown".to_string()
            } else {
                crate::utils::strings::from_tt(ptr)
            }
        }
    }

    /// Polls the client for the next event.
    pub fn poll(&self, timeout_ms: i32) -> Option<(Event, Message)> {
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        let t = timeout_ms;
        if unsafe { ffi::api().TT_GetMessage(self.ptr.0, &mut msg, &t) } == 1 {
            let event = Event::from(msg.nClientEvent);
            let message = Message::from_raw(event, msg);
            self.update_state_for_event(event, &message);
            #[cfg(feature = "state")]
            self.update_cache_for_event(event, &message);
            self.invoke_hooks(event, &message);
            self.dispatch_bus(event, &message);
            #[cfg(feature = "scripts")]
            self.dispatch_scripts(event, &message);
            #[cfg(feature = "logging")]
            tracing::trace!(
                event = ?event,
                source = message.source(),
                state = ?self.connection_state(),
                "teamtalk poll event"
            );
            self.handle_auto_reconnect();
            Some((event, message))
        } else {
            self.handle_auto_reconnect();
            None
        }
    }

    /// Polls until the predicate matches or the timeout expires.
    pub fn poll_until<F>(&self, timeout_ms: i32, mut predicate: F) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        use std::time::{Duration, Instant};
        if timeout_ms < 0 {
            loop {
                if let Some((event, msg)) = self.poll(timeout_ms)
                    && predicate(event, &msg)
                {
                    return Some((event, msg));
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return None;
            }
            let wait_ms = remaining.as_millis().min(i32::MAX as u128) as i32;
            if let Some((event, msg)) = self.poll(wait_ms)
                && predicate(event, &msg)
            {
                return Some((event, msg));
            }
        }
    }

    /// Polls until a specific event arrives or the timeout expires.
    pub fn wait_for(&self, event: Event, timeout_ms: i32) -> Option<Message> {
        self.poll_until(timeout_ms, |incoming, _| incoming == event)
            .map(|(_, msg)| msg)
    }

    /// Polls until a specific event arrives or the timeout expires.
    pub fn poll_until_event(&self, event: Event, timeout_ms: i32) -> Option<Message> {
        self.wait_for(event, timeout_ms)
    }

    fn update_state_for_event(&self, event: Event, msg: &Message) {
        #[cfg(feature = "logging")]
        let prev_state = self.connection_state();
        match event {
            Event::ConnectSuccess => {
                self.set_connection_state(ConnectionState::Connected);
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                if let Some(handler) = auto.handler.as_mut() {
                    handler.mark_connected();
                }
                auto.login_gave_up = false;
                auto.join_gave_up = false;
                if auto.enabled {
                    let msg =
                        Message::from_raw(event, unsafe { std::mem::zeroed::<ffi::TTMessage>() });
                    let attempts = auto.handler.as_ref().map(|h| h.attempts()).unwrap_or(0);
                    drop(auto);
                    if attempts > 0 {
                        self.invoke_hooks(Event::AfterReconnect { attempt: attempts }, &msg);
                    }
                } else {
                    drop(auto);
                }
            }
            Event::ConnectFailed | Event::ConnectionLost | Event::ConnectCryptError => {
                self.set_connection_state(ConnectionState::Disconnected);
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                if let Some(handler) = auto.handler.as_mut() {
                    handler.mark_disconnected();
                }
                auto.pending_login_cmd = None;
                auto.pending_join_cmd = None;
            }
            Event::MySelfLoggedIn => {
                self.set_connection_state(ConnectionState::LoggedIn);
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                auto.pending_login_cmd = None;
                if let Some(handler) = auto.login_handler.as_mut() {
                    handler.mark_connected();
                }
            }
            Event::MySelfLoggedOut => {
                self.set_connection_state(ConnectionState::Connected);
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                auto.pending_join_cmd = None;
                if let Some(handler) = auto.join_handler.as_mut() {
                    handler.mark_disconnected();
                }
            }
            Event::UserJoined => {
                if let Some(user) = msg.user()
                    && user.id == self.my_id()
                {
                    self.set_connection_state(ConnectionState::Joined(user.channel_id));
                    let mut auto = self
                        .auto_reconnect
                        .lock()
                        .unwrap_or_else(|e| e.into_inner());
                    auto.pending_join_cmd = None;
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_connected();
                    }
                    self.invoke_joined_hook(user.channel_id);
                }
            }
            Event::UserLeft => {
                if let Some(user) = msg.user()
                    && user.id == self.my_id()
                {
                    self.set_connection_state(ConnectionState::LoggedIn);
                    let mut auto = self
                        .auto_reconnect
                        .lock()
                        .unwrap_or_else(|e| e.into_inner());
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                }
            }
            Event::MySelfKicked => {
                let next_state = kicked_next_state(msg.source());
                self.set_connection_state(next_state);
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                auto.pending_join_cmd = None;
                if matches!(next_state, ConnectionState::Connected) {
                    auto.pending_login_cmd = None;
                }
                if let Some(handler) = auto.join_handler.as_mut() {
                    handler.mark_disconnected();
                }
                if matches!(next_state, ConnectionState::Connected)
                    && let Some(handler) = auto.login_handler.as_mut()
                {
                    handler.mark_disconnected();
                }
            }
            Event::CmdError => {
                let source = msg.source();
                let mut next_state = None;
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                if auto.pending_login_cmd == Some(source) {
                    auto.pending_login_cmd = None;
                    if let Some(handler) = auto.login_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                    next_state = Some(ConnectionState::Connected);
                } else if auto.pending_join_cmd == Some(source) {
                    auto.pending_join_cmd = None;
                    if let Some(handler) = auto.join_handler.as_mut() {
                        handler.mark_disconnected();
                    }
                    next_state = Some(ConnectionState::LoggedIn);
                }
                drop(auto);
                if let Some(state) = next_state {
                    self.set_connection_state(state);
                }
            }
            Event::CmdSuccess => {
                let source = msg.source();
                let mut auto = self
                    .auto_reconnect
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                if auto.pending_login_cmd == Some(source) {
                    auto.pending_login_cmd = None;
                }
                if auto.pending_join_cmd == Some(source) {
                    auto.pending_join_cmd = None;
                }
            }
            _ => {}
        }

        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if auto.enabled && event.is_reconnect_needed_with(&auto.extra_events) {
            auto.force_disconnect = true;
            drop(auto);
            self.set_connection_state(ConnectionState::Disconnected);
        }
        #[cfg(feature = "logging")]
        {
            let next_state = self.connection_state();
            if next_state != prev_state {
                tracing::debug!(
                    event = ?event,
                    previous = ?prev_state,
                    current = ?next_state,
                    "connection state transition"
                );
            }
        }
    }

    /// Returns the current client flags.
    pub fn get_flags(&self) -> crate::types::ClientFlags {
        crate::types::ClientFlags::from_raw(self.backend().get_flags(self.ptr.0))
    }

    /// Returns a human-readable error message for a TeamTalk error code.
    pub fn get_error_message(&self, code: i32) -> String {
        use crate::types::TT_STRLEN;
        use crate::utils::strings::tt_buf;
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            ffi::api().TT_GetErrorMessage(code, buf.as_mut_ptr());
            crate::utils::strings::to_string(&buf)
        }
    }

    /// Builds a typed SDK error with the resolved message.
    pub fn client_error(&self, code: i32) -> crate::events::Error {
        crate::events::Error::ClientError {
            code,
            message: self.get_error_message(code),
        }
    }
}

fn kicked_next_state(source: i32) -> ConnectionState {
    if source > 0 {
        ConnectionState::LoggedIn
    } else {
        ConnectionState::Connected
    }
}
