use super::Client;
use crate::events::Event;
use crate::utils::{ToTT, backoff::ExponentialBackoff};
use std::time::Duration;
use teamtalk_sys as ffi;

#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    pub max_attempts: u32,
    pub min_delay: Duration,
    pub max_delay: Duration,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_attempts: 10,
            min_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
        }
    }
}

pub struct ReconnectHandler {
    pub config: ReconnectConfig,
    backoff: ExponentialBackoff,
    attempts: u32,
}

impl ReconnectHandler {
    pub fn new(config: ReconnectConfig) -> Self {
        let backoff = ExponentialBackoff::new(config.min_delay, config.max_delay, 2.0, 0.1);
        Self {
            config,
            backoff,
            attempts: 0,
        }
    }

    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.attempts >= self.config.max_attempts {
            return None;
        }
        self.attempts += 1;
        Some(self.backoff.next_delay())
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
        self.backoff.reset();
    }
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

    pub fn handle_reconnect(
        &self,
        event: Event,
        host: &str,
        tcp: i32,
        ignore_events: &[Event],
        handler: Option<&mut ReconnectHandler>,
    ) -> bool {
        if ignore_events.contains(&event) {
            return false;
        }

        if event.is_reconnect_needed()
            && let Some(delay) = handler.and_then(|h| h.next_delay())
        {
            std::thread::sleep(delay);
            return self.connect(host, tcp, tcp, false).is_ok();
        }
        false
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
