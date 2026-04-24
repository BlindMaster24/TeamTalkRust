//! Hook registrations for session-lifecycle, user-presence, and
//! text-message events.

use crate::client::hooks::ClientHooks;
use crate::client::{Client, Message};
use crate::types::{ChannelId, TextMessage, User};

impl ClientHooks {
    /// Registers a handler for successful login.
    #[must_use]
    pub fn on_logged_in(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_in = Some(Box::new(hook));
        self
    }

    /// Registers a handler for logout.
    #[must_use]
    pub fn on_logged_out(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_logged_out = Some(Box::new(hook));
        self
    }

    /// Registers a handler for being kicked.
    #[must_use]
    pub fn on_myself_kicked(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_myself_kicked = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user login events.
    #[must_use]
    pub fn on_user_logged_in(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_logged_in = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user logout events.
    #[must_use]
    pub fn on_user_logged_out(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_logged_out = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user updates.
    #[must_use]
    pub fn on_user_update(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_update = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel joins.
    #[must_use]
    pub fn on_joined(mut self, hook: impl FnMut(&Client, ChannelId) + Send + 'static) -> Self {
        self.on_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user join event.
    #[must_use]
    pub fn on_user_joined(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_joined = Some(Box::new(hook));
        self
    }

    /// Registers a handler for any user leave event.
    #[must_use]
    pub fn on_user_left(mut self, hook: impl FnMut(&Client, User) + Send + 'static) -> Self {
        self.on_user_left = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel or user text messages.
    #[must_use]
    pub fn on_text_message(
        mut self,
        hook: impl FnMut(&Client, TextMessage) + Send + 'static,
    ) -> Self {
        self.on_text_message = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user state changes.
    #[must_use]
    pub fn on_user_state_change(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_state_change = Some(Box::new(hook));
        self
    }
}
