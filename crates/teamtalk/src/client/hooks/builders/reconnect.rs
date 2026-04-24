//! Hook registrations for the automatic reconnect / auto-login /
//! auto-join recovery pipeline and for internal SDK errors.

use crate::client::hooks::ClientHooks;
use crate::client::{Client, Message};

impl ClientHooks {
    /// Registers a handler for internal error events.
    #[must_use]
    pub fn on_internal_error(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_internal_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for reconnecting notifications.
    #[must_use]
    pub fn on_reconnecting(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_reconnecting = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic reconnect attempt.
    #[must_use]
    pub fn on_before_reconnect(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_reconnect = Some(Box::new(hook));
        self
    }

    /// Registers a handler after an automatic reconnect succeeds.
    #[must_use]
    pub fn on_after_reconnect(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_after_reconnect = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic reconnect gives up.
    #[must_use]
    pub fn on_reconnect_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_reconnect_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic login retry.
    #[must_use]
    pub fn on_before_auto_login(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_auto_login = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic login gives up.
    #[must_use]
    pub fn on_auto_login_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_login_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler before an automatic join retry.
    #[must_use]
    pub fn on_before_auto_join(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_before_auto_join = Some(Box::new(hook));
        self
    }

    /// Registers a handler when automatic join gives up.
    #[must_use]
    pub fn on_auto_join_failed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_join_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler after full in-session recovery reaches Joined.
    #[must_use]
    pub fn on_auto_recover_completed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_auto_recover_completed = Some(Box::new(hook));
        self
    }
}
