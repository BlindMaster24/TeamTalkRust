use super::Client;
use crate::utils::{ToTT, backoff::ExponentialBackoff};
use std::time::{Duration, Instant};
use teamtalk_sys as ffi;

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

    pub fn mark_connected(&mut self) {
        self.connected_at = Some(Instant::now());
    }

    pub fn mark_disconnected(&mut self) {
        if let Some(at) = self.connected_at
            && at.elapsed() >= self.config.stability_threshold
        {
            self.attempts = 0;
            self.backoff.reset();
        }
        self.connected_at = None;
    }

    pub fn can_attempt(&self) -> bool {
        if self.attempts >= self.config.max_attempts {
            return false;
        }
        match self.last_attempt {
            Some(last) => last.elapsed() >= self.backoff.current_delay(),
            None => true,
        }
    }

    pub fn record_attempt(&mut self) {
        self.last_attempt = Some(Instant::now());
        self.attempts += 1;
        self.backoff.next_delay();
    }
}

#[derive(Debug, Clone)]
pub struct ConnectParams<'a> {
    pub host: &'a str,
    pub tcp: i32,
    pub udp: i32,
    pub encrypted: bool,
}

impl Client {
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
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    pub fn connect_auto(&self, host: &str, tcp: i32, udp: i32) -> Result<(), crate::events::Error> {
        self.connect(host, tcp, udp, false)
    }

    pub fn is_connected(&self) -> bool {
        let flags = unsafe { ffi::api().TT_GetFlags(self.ptr) };
        (flags & ffi::ClientFlag::CLIENT_CONNECTED as u32) != 0
    }

    pub fn is_connecting(&self) -> bool {
        let flags = unsafe { ffi::api().TT_GetFlags(self.ptr) };
        (flags & ffi::ClientFlag::CLIENT_CONNECTING as u32) != 0
    }

    pub fn handle_reconnect(&self, params: &ConnectParams, handler: &mut ReconnectHandler) -> bool {
        if handler.can_attempt() {
            let _ = self.disconnect();
            handler.record_attempt();
            let _ = self.connect(params.host, params.tcp, params.udp, params.encrypted);
        }

        true
    }

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
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

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
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    pub fn disconnect(&self) -> Result<(), crate::events::Error> {
        if unsafe { ffi::api().TT_Disconnect(self.ptr) == 1 } {
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed(-1))
        }
    }

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

    pub fn get_client_keep_alive(&self) -> Option<crate::types::ClientKeepAlive> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientKeepAlive>() };
        if unsafe { ffi::api().TT_GetClientKeepAlive(self.ptr, &mut raw) } == 1 {
            Some(crate::types::ClientKeepAlive::from(raw))
        } else {
            None
        }
    }
}
