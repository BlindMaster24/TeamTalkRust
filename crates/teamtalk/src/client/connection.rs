//! Connection and reconnect helpers.
use super::Client;
use crate::events::ConnectionState;
use crate::utils::{ToTT, backoff::ExponentialBackoff};
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
}

#[derive(Debug, Clone)]
/// Borrowed connection parameters.
pub struct ConnectParams<'a> {
    pub host: &'a str,
    pub tcp: i32,
    pub udp: i32,
    pub encrypted: bool,
}

impl Client {
    /// Connects to a TeamTalk server.
    pub fn connect(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        let ok = unsafe {
            ffi::api().TT_Connect(
                self.ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                0,
                0,
                if encrypted { 1 } else { 0 },
            ) == 1
        };
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
        let flags = unsafe { ffi::api().TT_GetFlags(self.ptr) };
        (flags & ffi::ClientFlag::CLIENT_CONNECTED as u32) != 0
    }

    /// Returns true when the client is attempting to connect.
    pub fn is_connecting(&self) -> bool {
        let flags = unsafe { ffi::api().TT_GetFlags(self.ptr) };
        (flags & ffi::ClientFlag::CLIENT_CONNECTING as u32) != 0
    }

    /// Handles reconnect logic using provided parameters.
    pub fn handle_reconnect(&self, params: &ConnectParams, handler: &mut ReconnectHandler) -> bool {
        if handler.can_attempt() {
            let _ = self.disconnect();
            handler.record_attempt();
            let _ = self.connect(params.host, params.tcp, params.udp, params.encrypted);
        }

        true
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
        let ok = unsafe {
            ffi::api().TT_ConnectSysID(
                self.ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                0,
                0,
                if encrypted { 1 } else { 0 },
                sys_id.tt().as_ptr(),
            ) == 1
        };
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
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
        let ok = unsafe {
            ffi::api().TT_ConnectEx(
                self.ptr,
                host.tt().as_ptr(),
                tcp,
                udp,
                bind_ip.tt().as_ptr(),
                0,
                0,
                if encrypted { 1 } else { 0 },
            ) == 1
        };
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Disconnects from the server.
    pub fn disconnect(&self) -> Result<(), crate::events::Error> {
        if unsafe { ffi::api().TT_Disconnect(self.ptr) == 1 } {
            self.set_connection_state(ConnectionState::Disconnected);
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed(-1))
        }
    }

    /// Sets client keep-alive parameters.
    pub fn set_client_keep_alive(
        &self,
        keep_alive: &crate::types::ClientKeepAlive,
    ) -> Result<(), crate::events::Error> {
        if unsafe { ffi::api().TT_SetClientKeepAlive(self.ptr, &keep_alive.to_ffi()) == 1 } {
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed(-1))
        }
    }

    /// Returns client keep-alive parameters.
    pub fn get_client_keep_alive(&self) -> Option<crate::types::ClientKeepAlive> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientKeepAlive>() };
        if unsafe { ffi::api().TT_GetClientKeepAlive(self.ptr, &mut raw) } == 1 {
            Some(crate::types::ClientKeepAlive::from(raw))
        } else {
            None
        }
    }
}
