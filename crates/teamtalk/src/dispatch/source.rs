use super::*;

pub trait EventSource {
    /// Polls for the next event.
    fn poll(&mut self, timeout_ms: i32) -> Option<(Event, Message)>;
    /// Returns the underlying client if available.
    fn client(&self) -> Option<&Client>;
}

impl EventSource for Client {
    fn poll(&mut self, timeout_ms: i32) -> Option<(Event, Message)> {
        Client::poll(self, timeout_ms)
    }

    fn client(&self) -> Option<&Client> {
        Some(self)
    }
}

impl EventSource for &Client {
    fn poll(&mut self, timeout_ms: i32) -> Option<(Event, Message)> {
        (*self).poll(timeout_ms)
    }

    fn client(&self) -> Option<&Client> {
        Some(*self)
    }
}

pub(super) type HandlerFn = Box<dyn for<'a> FnMut(EventContext<'a>) -> DispatchFlow + Send>;

pub(super) struct HandlerEntry {
    pub(super) handler: HandlerFn,
}

pub(super) struct ReconnectState {
    params: ConnectParamsOwned,
    handler: ReconnectHandler,
    extra_events: Vec<Event>,
}

impl ReconnectState {
    pub(super) fn new(settings: ReconnectSettings) -> Self {
        Self {
            handler: ReconnectHandler::new(settings.config),
            params: settings.params,
            extra_events: settings.extra_events,
        }
    }

    pub(super) fn on_event(&mut self, client: Option<&Client>, event: &Event) {
        if matches!(event, Event::ConnectSuccess) {
            self.handler.mark_connected();
        }
        if event.is_reconnect_needed_with(&self.extra_events) {
            self.handler.mark_disconnected();
            if let Some(client) = client {
                let params = self.params.as_params();
                let _ = client.handle_reconnect(&params, &mut self.handler);
            }
        }
    }
}
