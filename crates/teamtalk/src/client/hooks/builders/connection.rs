//! Hook registrations for connection- and command-lifecycle events.
//!
//! Events in this module are emitted by the TeamTalk SDK while the
//! client is establishing, maintaining, or losing a connection, and
//! for command-processing notifications that are not tied to a
//! specific command result.

use crate::client::hooks::ClientHooks;
use crate::client::{Client, Message};

impl ClientHooks {
    /// Registers a handler for successful connections.
    #[must_use]
    pub fn on_connect_success(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_success = Some(Box::new(hook));
        self
    }

    /// Registers a handler for failed connections.
    #[must_use]
    pub fn on_connect_failed(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_failed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for connection encryption errors.
    #[must_use]
    pub fn on_connect_crypt_error(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connect_crypt_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for max payload updates.
    #[must_use]
    pub fn on_connect_max_payload_updated(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_connect_max_payload_updated = Some(Box::new(hook));
        self
    }

    /// Registers a handler for connection loss.
    #[must_use]
    pub fn on_connection_lost(mut self, hook: impl FnMut(&Client) + Send + 'static) -> Self {
        self.on_connection_lost = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command processing notifications.
    #[must_use]
    pub fn on_cmd_processing(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_cmd_processing = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command errors.
    #[must_use]
    pub fn on_cmd_error(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_cmd_error = Some(Box::new(hook));
        self
    }

    /// Registers a handler for command success notifications.
    #[must_use]
    pub fn on_cmd_success(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_cmd_success = Some(Box::new(hook));
        self
    }
}
