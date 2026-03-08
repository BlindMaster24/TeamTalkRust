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

/// Watchdog timeouts for in-progress recovery phases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconnectPhaseTimeouts {
    /// Maximum time to wait for `ConnectSuccess`, `ConnectFailed`, or `ConnectCryptError`.
    pub connect: Option<Duration>,
    /// Maximum time to wait for login completion after a login command is accepted.
    pub login: Option<Duration>,
    /// Maximum time to wait for channel join completion after a join command is accepted.
    pub join: Option<Duration>,
}

impl ReconnectPhaseTimeouts {
    /// Disables phase watchdogs for all built-in recovery phases.
    pub const fn disabled() -> Self {
        Self {
            connect: None,
            login: None,
            join: None,
        }
    }
}

impl Default for ReconnectPhaseTimeouts {
    fn default() -> Self {
        let timeout = Some(Duration::from_secs(15));
        Self {
            connect: timeout,
            login: timeout,
            join: timeout,
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
    auto.clear_phase_tracking();
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

fn validate_phase_timeouts(timeouts: &ReconnectPhaseTimeouts) -> Result<(), Error> {
    for timeout in [timeouts.connect, timeouts.login, timeouts.join] {
        if matches!(timeout, Some(value) if value.is_zero()) {
            return Err(Error::InvalidParam);
        }
    }
    Ok(())
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

mod auto_reconnect;
mod connect_ops;
mod keep_alive;
