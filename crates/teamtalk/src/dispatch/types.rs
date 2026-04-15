use super::*;

/// Owned connection parameters for reconnect workflows.
#[derive(Clone)]
pub struct ConnectParamsOwned {
    pub host: String,
    pub tcp: i32,
    pub udp: i32,
    pub encrypted: bool,
}

impl ConnectParamsOwned {
    /// Creates owned connection parameters.
    pub fn new(host: impl Into<String>, tcp: i32, udp: i32, encrypted: bool) -> Self {
        Self {
            host: host.into(),
            tcp,
            udp,
            encrypted,
        }
    }

    /// Returns a borrowed `ConnectParams` view.
    pub fn as_params(&self) -> ConnectParams<'_> {
        ConnectParams {
            host: &self.host,
            tcp: self.tcp,
            udp: self.udp,
            encrypted: self.encrypted,
        }
    }
}

impl From<ConnectParams<'_>> for ConnectParamsOwned {
    fn from(params: ConnectParams<'_>) -> Self {
        Self::new(params.host, params.tcp, params.udp, params.encrypted)
    }
}

#[derive(Clone)]
/// Reconnect settings for dispatch flows.
pub struct ReconnectSettings {
    pub params: ConnectParamsOwned,
    pub config: ReconnectConfig,
    pub extra_events: Vec<Event>,
}

impl ReconnectSettings {
    /// Creates reconnect settings with default event set.
    pub fn new(params: ConnectParamsOwned, config: ReconnectConfig) -> Self {
        Self {
            params,
            config,
            extra_events: Vec::new(),
        }
    }

    /// Adds extra events which should trigger reconnection.
    pub fn with_extra_events(mut self, extra_events: Vec<Event>) -> Self {
        self.extra_events = extra_events;
        self
    }
}

#[derive(Clone)]
/// Configuration for the dispatcher runtime.
pub struct ClientConfig {
    pub poll_timeout_ms: i32,
    pub reconnect: Option<ReconnectSettings>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            poll_timeout_ms: 100,
            reconnect: None,
        }
    }
}

impl ClientConfig {
    /// Creates a configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the poll timeout in milliseconds.
    pub fn poll_timeout_ms(mut self, timeout_ms: i32) -> Self {
        self.poll_timeout_ms = timeout_ms;
        self
    }

    /// Enables reconnect using provided connection parameters.
    pub fn reconnect(mut self, params: ConnectParamsOwned, config: ReconnectConfig) -> Self {
        self.reconnect = Some(ReconnectSettings::new(params, config));
        self
    }

    /// Enables reconnect and adds extra events which trigger reconnect.
    pub fn reconnect_with_events(
        mut self,
        params: ConnectParamsOwned,
        config: ReconnectConfig,
        extra_events: Vec<Event>,
    ) -> Self {
        self.reconnect =
            Some(ReconnectSettings::new(params, config).with_extra_events(extra_events));
        self
    }

    pub fn without_reconnect(mut self) -> Self {
        self.reconnect = None;
        self
    }
}

/// Controls dispatcher loop flow.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DispatchFlow {
    Continue,
    Stop,
}

/// Event context passed into dispatcher handlers.
#[derive(Clone, Copy)]
pub struct EventContext<'a> {
    pub(super) event: Event,
    pub(super) message: &'a Message,
    pub(super) client: Option<&'a Client>,
}

impl<'a> EventContext<'a> {
    /// Returns the event.
    pub fn event(&self) -> Event {
        self.event
    }

    /// Returns the raw message.
    pub fn message(&self) -> &Message {
        self.message
    }

    /// Returns the client if the source provides one.
    pub fn client(&self) -> Option<&Client> {
        self.client
    }
}
