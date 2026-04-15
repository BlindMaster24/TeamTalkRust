use super::source::{EventSource, HandlerEntry, ReconnectState};
use super::{ClientConfig, DispatchFlow, Event, EventContext, Message};

pub struct Dispatcher<S: EventSource> {
    source: S,
    handlers: Vec<HandlerEntry>,
    poll_timeout_ms: i32,
    reconnect: Option<ReconnectState>,
    stop: bool,
}

impl<S: EventSource> Dispatcher<S> {
    /// Creates a dispatcher with default configuration.
    pub fn new(source: S) -> Self {
        Self::with_config(source, ClientConfig::default())
    }

    /// Creates a dispatcher with a custom configuration.
    pub fn with_config(source: S, config: ClientConfig) -> Self {
        let reconnect = config.reconnect.map(ReconnectState::new);
        Self {
            source,
            handlers: Vec::new(),
            poll_timeout_ms: config.poll_timeout_ms,
            reconnect,
            stop: false,
        }
    }

    /// Returns the underlying event source.
    pub fn source(&self) -> &S {
        &self.source
    }

    /// Returns a mutable reference to the event source.
    pub fn source_mut(&mut self) -> &mut S {
        &mut self.source
    }

    /// Adds a handler for a specific event.
    pub fn add_handler<F>(&mut self, event: Event, handler: F)
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.handlers.push(HandlerEntry {
            event: Some(event),
            handler: Box::new(handler),
        });
    }

    /// Adds a handler which receives all events.
    pub fn add_handler_any<F>(&mut self, handler: F)
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.handlers.push(HandlerEntry {
            event: None,
            handler: Box::new(handler),
        });
    }

    /// Adds a handler and returns the dispatcher for chaining.
    #[must_use]
    pub fn on_event<F>(mut self, event: Event, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.add_handler(event, handler);
        self
    }

    /// Adds a handler for all events and returns the dispatcher for chaining.
    #[must_use]
    pub fn on_any<F>(mut self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.add_handler_any(handler);
        self
    }

    /// Adds a handler for user join events.
    #[must_use]
    pub fn on_user_joined<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::UserJoined, handler)
    }

    /// Adds a handler for user left events.
    #[must_use]
    pub fn on_user_left<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::UserLeft, handler)
    }

    /// Adds a handler for text messages.
    #[must_use]
    pub fn on_text_message<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::TextMessage, handler)
    }

    /// Adds a handler for connection success events.
    #[must_use]
    pub fn on_connect_success<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::ConnectSuccess, handler)
    }

    /// Adds a handler for connection lost events.
    #[must_use]
    pub fn on_connection_lost<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::ConnectionLost, handler)
    }

    /// Adds a handler for connection failure events.
    #[must_use]
    pub fn on_connect_failed<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::ConnectFailed, handler)
    }

    /// Adds a handler for command error events.
    #[must_use]
    pub fn on_command_error<F>(self, handler: F) -> Self
    where
        F: for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send + 'static,
    {
        self.on_event(Event::CmdError, handler)
    }

    /// Requests the dispatcher loop to stop.
    pub fn stop(&mut self) {
        self.stop = true;
    }

    /// Runs the dispatcher loop with the configured timeout.
    pub fn run(&mut self) -> DispatchFlow {
        self.run_with_timeout(self.poll_timeout_ms)
    }

    /// Runs the dispatcher loop with an explicit timeout.
    pub fn run_with_timeout(&mut self, timeout_ms: i32) -> DispatchFlow {
        while !self.stop {
            let flow = self.step(timeout_ms);
            if flow == DispatchFlow::Stop {
                self.stop = true;
            }
        }
        DispatchFlow::Stop
    }

    /// Performs one poll/dispatch step.
    pub fn step(&mut self, timeout_ms: i32) -> DispatchFlow {
        match self.source.poll(timeout_ms) {
            Some((event, message)) => self.process_event(event, &message),
            None => DispatchFlow::Continue,
        }
    }

    fn process_event(&mut self, event: Event, message: &Message) -> DispatchFlow {
        let client = self.source.client();
        if let Some(reconnect) = self.reconnect.as_mut() {
            reconnect.on_event(client, &event);
        }
        #[cfg(feature = "logging")]
        crate::logging::event(&event, message);
        let ctx = EventContext {
            event,
            message,
            client,
        };
        let mut flow = DispatchFlow::Continue;
        for handler in &mut self.handlers {
            if handler.matches(&event) && (handler.handler)(ctx) == DispatchFlow::Stop {
                flow = DispatchFlow::Stop;
            }
        }
        flow
    }
}
