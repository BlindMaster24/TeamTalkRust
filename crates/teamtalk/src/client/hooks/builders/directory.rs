//! Hook registrations for channel, server, file, account, and ban
//! directory mutations (the "slow-changing state" events delivered
//! after login).

use crate::client::hooks::ClientHooks;
use crate::client::{Client, Message};

impl ClientHooks {
    /// Registers a handler for channel creation events.
    #[must_use]
    pub fn on_channel_created(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_created = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel update events.
    #[must_use]
    pub fn on_channel_updated(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_updated = Some(Box::new(hook));
        self
    }

    /// Registers a handler for channel removal events.
    #[must_use]
    pub fn on_channel_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_channel_removed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for server updates.
    #[must_use]
    pub fn on_server_update(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_server_update = Some(Box::new(hook));
        self
    }

    /// Registers a handler for server statistics updates.
    #[must_use]
    pub fn on_server_statistics(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_server_statistics = Some(Box::new(hook));
        self
    }

    /// Registers a handler for new file events.
    #[must_use]
    pub fn on_file_new(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_file_new = Some(Box::new(hook));
        self
    }

    /// Registers a handler for file removal events.
    #[must_use]
    pub fn on_file_remove(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_file_remove = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account events.
    #[must_use]
    pub fn on_user_account(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_user_account = Some(Box::new(hook));
        self
    }

    /// Registers a handler for banned user events.
    #[must_use]
    pub fn on_banned_user(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_banned_user = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account creation events.
    #[must_use]
    pub fn on_user_account_created(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_account_created = Some(Box::new(hook));
        self
    }

    /// Registers a handler for user account removal events.
    #[must_use]
    pub fn on_user_account_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_account_removed = Some(Box::new(hook));
        self
    }
}
