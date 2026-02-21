//! Connection and reconnect helpers.
use super::Client;
use crate::events::{ConnectionState, Error};
use crate::utils::backoff::ExponentialBackoff;
use std::env;
use std::time::{Duration, Instant};
use teamtalk_sys as ffi;

/// Reconnect policy configuration.
#[derive(Clone)]
pub struct ReconnectConfig {
    pub max_attempts: u32,
    pub min_delay: Duration,
    pub max_delay: Duration,
    pub stability_threshold: Duration,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_attempts: u32::MAX,
            min_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(60),
            stability_threshold: Duration::from_secs(10),
        }
    }
}

/// Full in-session recovery workflow configuration.
///
/// The default keeps login and join retries aligned with reconnect defaults.
#[derive(Clone)]
pub struct ReconnectWorkflowConfig {
    pub login: ReconnectConfig,
    pub join: ReconnectConfig,
}

impl Default for ReconnectWorkflowConfig {
    fn default() -> Self {
        let defaults = ReconnectConfig::default();
        Self {
            login: defaults.clone(),
            join: defaults,
        }
    }
}

pub struct ReconnectHandler {
    pub config: ReconnectConfig,
    backoff: ExponentialBackoff,
    attempts: u32,
    last_attempt: Option<Instant>,
    connected_at: Option<Instant>,
}

impl ReconnectHandler {
    /// Creates a new reconnect handler.
    pub fn new(config: ReconnectConfig) -> Self {
        let backoff = ExponentialBackoff::new(config.min_delay, config.max_delay, 1.6, 1.0);
        Self {
            config,
            backoff,
            attempts: 0,
            last_attempt: None,
            connected_at: None,
        }
    }

    /// Marks the client as connected.
    pub fn mark_connected(&mut self) {
        self.connected_at = Some(Instant::now());
    }

    /// Marks the client as disconnected.
    pub fn mark_disconnected(&mut self) {
        if let Some(at) = self.connected_at
            && at.elapsed() >= self.config.stability_threshold
        {
            self.attempts = 0;
            self.backoff.reset();
        }
        self.connected_at = None;
    }

    /// Returns true when a reconnect attempt is allowed.
    pub fn can_attempt(&self) -> bool {
        if self.attempts >= self.config.max_attempts {
            return false;
        }
        match self.last_attempt {
            Some(last) => last.elapsed() >= self.backoff.current_delay(),
            None => true,
        }
    }

    /// Records a reconnect attempt.
    pub fn record_attempt(&mut self) {
        self.last_attempt = Some(Instant::now());
        self.attempts += 1;
        self.backoff.next_delay();
    }

    /// Resets attempts and backoff state.
    pub fn reset(&mut self) {
        self.attempts = 0;
        self.last_attempt = None;
        self.connected_at = None;
        self.backoff.reset();
    }

    /// Returns the current backoff delay.
    pub fn current_delay(&self) -> Duration {
        self.backoff.current_delay()
    }

    /// Returns the number of attempts.
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Returns true when no more attempts are allowed.
    pub fn exhausted(&self) -> bool {
        self.attempts >= self.config.max_attempts
    }
}

#[derive(Debug, Clone)]
/// Borrowed connection parameters.
pub struct ConnectParams<'a> {
    pub host: &'a str,
    pub tcp: i32,
    pub udp: i32,
    pub encrypted: bool,
}

#[derive(Debug, Clone)]
/// Owned connection parameters.
pub struct ConnectParamsOwned {
    pub host: String,
    pub tcp: i32,
    pub udp: i32,
    pub encrypted: bool,
}

impl ConnectParamsOwned {
    pub fn new(host: impl Into<String>, tcp: i32, udp: i32, encrypted: bool) -> Self {
        Self {
            host: host.into(),
            tcp,
            udp,
            encrypted,
        }
    }

    pub fn from_env() -> Self {
        let host = env::var("TT_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let tcp = env::var("TT_TCP")
            .ok()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(10333);
        let udp = env::var("TT_UDP")
            .ok()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(10333);
        let encrypted = env::var("TT_ENCRYPTED")
            .ok()
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false);
        Self::new(host, tcp, udp, encrypted)
    }
}

impl<'a> From<&ConnectParams<'a>> for ConnectParamsOwned {
    fn from(params: &ConnectParams<'a>) -> Self {
        Self::new(params.host, params.tcp, params.udp, params.encrypted)
    }
}

fn reset_auto_recovery_handlers(auto: &mut super::core::AutoReconnectState) {
    auto.login_handler = Some(ReconnectHandler::new(auto.workflow.login.clone()));
    auto.join_handler = Some(ReconnectHandler::new(auto.workflow.join.clone()));
    auto.login_gave_up = false;
    auto.join_gave_up = false;
    auto.recovery_completed = false;
    auto.pending_login_cmd = None;
    auto.pending_join_cmd = None;
}

fn validate_client_keep_alive(keep_alive: &crate::types::ClientKeepAlive) -> Result<(), Error> {
    if keep_alive.lost_ms <= 0 {
        return Err(Error::InvalidParam);
    }
    if keep_alive.udp_interval_ms <= 0 {
        return Err(Error::InvalidParam);
    }
    if keep_alive.lost_ms <= keep_alive.udp_interval_ms {
        return Err(Error::InvalidParam);
    }
    if keep_alive.tcp_interval_ms > 0 && keep_alive.lost_ms <= keep_alive.tcp_interval_ms {
        return Err(Error::InvalidParam);
    }
    Ok(())
}

fn dedupe_events(events: Vec<crate::events::Event>) -> Vec<crate::events::Event> {
    let mut unique = Vec::with_capacity(events.len());
    for event in events {
        if unique
            .iter()
            .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(&event))
        {
            continue;
        }
        unique.push(event);
    }
    unique
}

fn ensure_connect_not_busy(has_connection_flags: bool) -> Result<(), Error> {
    if has_connection_flags {
        return Err(Error::CommandFailed {
            code: -1,
            message: "Connect refused: client is already connecting or connected".to_string(),
        });
    }
    Ok(())
}

impl Client {
    fn disconnect_connect_barrier_if_needed(&self) {
        if matches!(self.connection_state(), ConnectionState::Disconnected) {
            let _ = self.disconnect();
        }
    }

    fn ensure_connect_allowed(&self) -> Result<(), Error> {
        ensure_connect_not_busy(self.has_connection_flags())
    }

    fn disconnect_reconnect_barrier(&self) -> Result<(), Error> {
        let _ = self.disconnect();
        if self.has_connection_flags() {
            return Err(Error::CommandFailed {
                code: -1,
                message: "Reconnect barrier failed: client still has connection flags".to_string(),
            });
        }
        Ok(())
    }

    /// Enables automatic reconnection using the provided config.
    pub fn enable_auto_reconnect(&self, config: ReconnectConfig) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = true;
        auto.handler = Some(ReconnectHandler::new(config));
        auto.extra_events.clear();
        auto.force_disconnect = false;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Enables automatic reconnection and adds extra events that trigger reconnect.
    pub fn enable_auto_reconnect_with_events(
        &self,
        config: ReconnectConfig,
        extra_events: Vec<crate::events::Event>,
    ) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = true;
        auto.handler = Some(ReconnectHandler::new(config));
        auto.extra_events = dedupe_events(extra_events);
        auto.force_disconnect = false;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Disables automatic reconnection.
    pub fn disable_auto_reconnect(&self) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.enabled = false;
        auto.handler = None;
        auto.login_handler = None;
        auto.join_handler = None;
        auto.login_gave_up = false;
        auto.join_gave_up = false;
        auto.recovery_completed = false;
        auto.pending_login_cmd = None;
        auto.pending_join_cmd = None;
        auto.extra_events.clear();
        auto.force_disconnect = false;
    }

    /// Sets per-phase retry behavior for in-session recovery.
    pub fn set_reconnect_workflow_config(&self, workflow: ReconnectWorkflowConfig) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.workflow = workflow;
        reset_auto_recovery_handlers(&mut auto);
    }

    /// Returns per-phase retry behavior for in-session recovery.
    pub fn reconnect_workflow_config(&self) -> ReconnectWorkflowConfig {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .workflow
            .clone()
    }

    /// Enables full in-session recovery and stores connect/login params in memory.
    pub fn enable_full_auto_reconnect(
        &self,
        connect_config: ReconnectConfig,
        workflow: ReconnectWorkflowConfig,
        connect_params: ConnectParamsOwned,
        login_params: super::users::LoginParams,
    ) {
        self.enable_auto_reconnect(connect_config);
        self.set_reconnect_workflow_config(workflow);
        self.set_reconnect_params(connect_params);
        self.set_login_params(login_params);
    }

    /// Returns true if automatic reconnection is enabled.
    pub fn auto_reconnect_enabled(&self) -> bool {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .enabled
    }

    /// Stores connection parameters for automatic reconnection.
    pub fn set_reconnect_params(&self, params: ConnectParamsOwned) {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .params = Some(params);
    }

    /// Returns the stored reconnection parameters, if any.
    pub fn reconnect_params(&self) -> Option<ConnectParamsOwned> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .params
            .clone()
    }

    /// Sets extra events that should trigger automatic reconnection.
    pub fn set_auto_reconnect_events(&self, extra_events: Vec<crate::events::Event>) {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .extra_events = dedupe_events(extra_events);
    }

    /// Adds one extra event that should trigger automatic reconnection.
    pub fn add_auto_reconnect_event(&self, event: crate::events::Event) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if auto
            .extra_events
            .iter()
            .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(&event))
        {
            return;
        }
        auto.extra_events.push(event);
    }

    /// Removes one extra event trigger by event kind.
    pub fn remove_auto_reconnect_event(&self, event: crate::events::Event) -> bool {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let before = auto.extra_events.len();
        auto.extra_events
            .retain(|existing| std::mem::discriminant(existing) != std::mem::discriminant(&event));
        auto.extra_events.len() != before
    }

    /// Returns the extra events that trigger automatic reconnection.
    pub fn auto_reconnect_events(&self) -> Vec<crate::events::Event> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .extra_events
            .clone()
    }

    /// Returns the last remembered channel, if any.
    pub fn last_channel(&self) -> Option<crate::types::ChannelId> {
        self.auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .last_channel
    }

    /// Remembers the channel and optional password used for auto-join after reconnect.
    pub fn set_last_channel(&self, channel: crate::types::ChannelId, password: Option<&str>) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.last_channel = Some(channel);
        auto.last_channel_password = match password {
            Some(value) if !value.is_empty() => Some(value.to_string()),
            _ => None,
        };
        auto.join_gave_up = false;
        auto.pending_join_cmd = None;
    }

    /// Clears the remembered channel.
    pub fn clear_last_channel(&self) {
        let mut auto = self
            .auto_reconnect
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        auto.last_channel = None;
        auto.last_channel_password = None;
        auto.pending_join_cmd = None;
        auto.join_gave_up = false;
    }

    /// Connects and remembers the parameters for automatic reconnection.
    pub fn connect_remember(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.set_reconnect_params(ConnectParamsOwned::new(host, tcp, udp, encrypted));
        self.connect(host, tcp, udp, encrypted)
    }

    /// Connects using the provided parameters.
    pub fn connect_with_params(
        &self,
        params: &ConnectParamsOwned,
    ) -> Result<(), crate::events::Error> {
        self.connect(&params.host, params.tcp, params.udp, params.encrypted)
    }

    pub fn connect_from_env(&self) -> Result<(), crate::events::Error> {
        let params = ConnectParamsOwned::from_env();
        self.connect_remember(&params.host, params.tcp, params.udp, params.encrypted)
    }

    /// Connects to a TeamTalk server.
    pub fn connect(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect(self.ptr.0, host, tcp, udp, encrypted);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Connects without encryption.
    pub fn connect_auto(&self, host: &str, tcp: i32, udp: i32) -> Result<(), crate::events::Error> {
        self.connect(host, tcp, udp, false)
    }

    /// Returns true when the client is connected.
    pub fn is_connected(&self) -> bool {
        let flags = self.backend().get_flags(self.ptr.0);
        (flags & ffi::ClientFlag::CLIENT_CONNECTED as u32) != 0
    }

    /// Returns true when the client is attempting to connect.
    pub fn is_connecting(&self) -> bool {
        let flags = self.backend().get_flags(self.ptr.0);
        (flags & ffi::ClientFlag::CLIENT_CONNECTING as u32) != 0
    }

    pub(crate) fn has_connection_flags(&self) -> bool {
        self.get_flags().has(crate::types::ClientFlags::CONNECTION)
    }

    /// Handles reconnect logic using provided parameters.
    pub fn handle_reconnect(&self, params: &ConnectParams, handler: &mut ReconnectHandler) -> bool {
        if !handler.can_attempt() {
            return true;
        }

        if self.disconnect_reconnect_barrier().is_err() {
            return true;
        }

        handler.record_attempt();
        let _ = self.connect(params.host, params.tcp, params.udp, params.encrypted);
        true
    }

    /// Reconnects to a TeamTalk server using a disconnect barrier first.
    pub fn reconnect(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect(host, tcp, udp, encrypted)
    }

    /// Reconnects using the provided parameters.
    pub fn reconnect_with_params(
        &self,
        params: &ConnectParamsOwned,
    ) -> Result<(), crate::events::Error> {
        self.reconnect(&params.host, params.tcp, params.udp, params.encrypted)
    }

    /// Connects with a custom system id string.
    pub fn connect_sys_id(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect_sys_id(self.ptr.0, host, tcp, udp, encrypted, sys_id);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Reconnects with a custom system id string.
    pub fn reconnect_sys_id(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect_sys_id(host, tcp, udp, encrypted, sys_id)
    }

    /// Connects with a custom bind IP.
    pub fn connect_ex(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect_ex(self.ptr.0, host, tcp, udp, bind_ip, encrypted);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Reconnects with a custom bind IP.
    pub fn reconnect_ex(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect_ex(host, tcp, udp, bind_ip, encrypted)
    }

    /// Disconnects from the server.
    pub fn disconnect(&self) -> Result<(), crate::events::Error> {
        if self.backend().disconnect(self.ptr.0) {
            self.set_connection_state(ConnectionState::Disconnected);
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Disconnect failed".to_string(),
            })
        }
    }

    /// Sets client keep-alive parameters.
    pub fn set_client_keep_alive(
        &self,
        keep_alive: &crate::types::ClientKeepAlive,
    ) -> Result<(), crate::events::Error> {
        validate_client_keep_alive(keep_alive)?;
        if unsafe { ffi::api().TT_SetClientKeepAlive(self.ptr.0, &keep_alive.to_ffi()) == 1 } {
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Set keep-alive failed".to_string(),
            })
        }
    }

    /// Sets keep-alive parameters and issues a ping to restart timers.
    pub fn set_client_keep_alive_and_ping(
        &self,
        keep_alive: &crate::types::ClientKeepAlive,
    ) -> Result<i32, crate::events::Error> {
        self.set_client_keep_alive(keep_alive)?;
        let cmd_id = self.ping();
        if cmd_id > 0 {
            Ok(cmd_id)
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Ping failed".to_string(),
            })
        }
    }

    /// Returns client keep-alive parameters.
    pub fn get_client_keep_alive(&self) -> Option<crate::types::ClientKeepAlive> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientKeepAlive>() };
        if unsafe { ffi::api().TT_GetClientKeepAlive(self.ptr.0, &mut raw) } == 1 {
            Some(crate::types::ClientKeepAlive::from(raw))
        } else {
            None
        }
    }
}
