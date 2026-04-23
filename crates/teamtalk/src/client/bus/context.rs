//! Read-only context passed to event subscribers.
//!
//! `EventContext` is the argument type handed to every closure registered
//! through [`crate::client::bus::SubscriptionBuilder::subscribe`]. It is
//! deliberately a thin borrow over the `(Event, &Message, &Client)` triple
//! so handlers can access everything the dispatch loop knows without
//! copying message payloads.

use crate::client::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, TextMessage, User, UserId};

/// Context for a dispatched client event.
#[derive(Clone, Copy)]
pub struct EventContext<'a> {
    pub(super) event: Event,
    pub(super) message: &'a Message,
    pub(super) client: &'a Client,
}

impl<'a> EventContext<'a> {
    /// Returns the event.
    #[must_use]
    pub fn event(&self) -> Event {
        self.event
    }

    /// Returns the raw message.
    #[must_use]
    pub fn message(&self) -> &'a Message {
        self.message
    }

    /// Returns the client which emitted the event.
    #[must_use]
    pub fn client(&self) -> &'a Client {
        self.client
    }

    /// Returns the user payload if present.
    #[must_use]
    pub fn user(&self) -> Option<User> {
        self.message.user()
    }

    /// Returns the text payload if present.
    #[must_use]
    pub fn text(&self) -> Option<TextMessage> {
        self.message.text()
    }

    /// Returns the source user id if present.
    #[must_use]
    pub fn user_id(&self) -> Option<UserId> {
        self.message
            .user()
            .map(|user| user.id)
            .or_else(|| self.message.text().map(|text| text.from_id))
    }

    /// Returns the channel id if present.
    #[must_use]
    pub fn channel_id(&self) -> Option<ChannelId> {
        self.message
            .user()
            .map(|user| user.channel_id)
            .or_else(|| self.message.text().map(|text| text.channel_id))
    }
}
