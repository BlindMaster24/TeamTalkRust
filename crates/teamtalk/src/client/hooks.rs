use super::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, TextMessage, User};

type EventHook = Box<dyn FnMut(&Client, Event, &Message) + Send>;
type ClientHook = Box<dyn FnMut(&Client) + Send>;
type ChannelHook = Box<dyn FnMut(&Client, ChannelId) + Send>;
type UserHook = Box<dyn FnMut(&Client, User) + Send>;
type TextHook = Box<dyn FnMut(&Client, TextMessage) + Send>;

/// Event hooks for reacting to client activity.
#[derive(Default)]
pub struct ClientHooks {
    on_event: Option<EventHook>,
    on_connect_success: Option<ClientHook>,
    on_connect_failed: Option<ClientHook>,
    on_connection_lost: Option<ClientHook>,
    on_logged_in: Option<ClientHook>,
    on_logged_out: Option<ClientHook>,
    on_joined: Option<ChannelHook>,
    on_user_joined: Option<UserHook>,
    on_user_left: Option<UserHook>,
    on_text_message: Option<TextHook>,
}

impl ClientHooks {
    /// Registers a handler for every event.
    pub fn on_event(mut self, hook: impl FnMut(&Client, Event, &Message) + Send + 'static) -> Self {
        self.on_event = Some(Box::new(hook));
        self
    }

    /// Registers a handler for successful connections.
    pub fn on_connect_success(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_success = Some(Box::new(hook));
        self
    }

    /// Registers a handler for failed connections.
    pub fn on_connect_failed(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for connection loss.
    pub fn on_connection_lost(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connection_lost = Some(Box::new(hook));
        self
    }

    /// Registers a handler for successful login.
    pub fn on_logged_in(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_in = Some(Box::new(hook));
        self
    }

    /// Registers a handler for logout.
    pub fn on_logged_out(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_out = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel joins.
    pub fn on_joined(mut self, hook: impl FnMut(&Client, ChannelId) + Send + 'static) -> Self {
        self.on_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user join event.
    pub fn on_user_joined(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user leave event.
    pub fn on_user_left(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_left = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel or user text messages.
    pub fn on_text_message(
        mut self,
        hook: impl FnMut(&Client, TextMessage) + Send + 'static,
    ) -> Self {
        self.on_text_message = Some(Box::new(hook));
        self
    }

    pub(crate) fn fire(&mut self, client: &Client, event: Event, msg: &Message) {
        match event {
            Event::ConnectSuccess => {
                if let Some(hook) = self.on_connect_success.as_mut() {
                    hook(client);
                }
            }
            Event::ConnectFailed => {
                if let Some(hook) = self.on_connect_failed.as_mut() {
                    hook(client);
                }
            }
            Event::ConnectionLost => {
                if let Some(hook) = self.on_connection_lost.as_mut() {
                    hook(client);
                }
            }
            Event::MySelfLoggedIn => {
                if let Some(hook) = self.on_logged_in.as_mut() {
                    hook(client);
                }
            }
            Event::MySelfLoggedOut => {
                if let Some(hook) = self.on_logged_out.as_mut() {
                    hook(client);
                }
            }
            Event::UserJoined => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_joined.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::UserLeft => {
                if let Some(user) = msg.user()
                    && let Some(hook) = self.on_user_left.as_mut()
                {
                    hook(client, user);
                }
            }
            Event::TextMessage => {
                if let Some(text) = msg.text()
                    && let Some(hook) = self.on_text_message.as_mut()
                {
                    hook(client, text);
                }
            }
            _ => {}
        }

        if let Some(hook) = self.on_event.as_mut() {
            hook(client, event, msg);
        }
    }

    pub(crate) fn fire_joined(&mut self, client: &Client, channel_id: ChannelId) {
        if let Some(hook) = self.on_joined.as_mut() {
            hook(client, channel_id);
        }
    }
}
