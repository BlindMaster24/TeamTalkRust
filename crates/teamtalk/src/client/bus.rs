use super::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, TextMessage, User, UserId};
use std::mem;

type Predicate = Box<dyn FnMut(&EventContext) -> bool + Send>;
type Handler = Box<dyn FnMut(EventContext) + Send>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Identifier for an event subscription.
pub struct EventSubscriptionId(u64);

/// Context for a dispatched client event.
#[derive(Clone, Copy)]
pub struct EventContext<'a> {
    event: Event,
    message: &'a Message,
    client: &'a Client,
}

impl<'a> EventContext<'a> {
    /// Returns the event.
    pub fn event(&self) -> Event {
        self.event
    }

    /// Returns the raw message.
    pub fn message(&self) -> &'a Message {
        self.message
    }

    /// Returns the client which emitted the event.
    pub fn client(&self) -> &'a Client {
        self.client
    }

    /// Returns the user payload if present.
    pub fn user(&self) -> Option<User> {
        self.message.user()
    }

    /// Returns the text payload if present.
    pub fn text(&self) -> Option<TextMessage> {
        self.message.text()
    }

    /// Returns the source user id if present.
    pub fn user_id(&self) -> Option<UserId> {
        self.message
            .user()
            .map(|user| user.id)
            .or_else(|| self.message.text().map(|text| text.from_id))
    }

    /// Returns the channel id if present.
    pub fn channel_id(&self) -> Option<ChannelId> {
        self.message
            .user()
            .map(|user| user.channel_id)
            .or_else(|| self.message.text().map(|text| text.channel_id))
    }
}

#[derive(Default)]
pub(crate) struct EventBus {
    next_id: u64,
    subscriptions: Vec<Subscription>,
}

impl EventBus {
    pub(crate) fn subscribe(
        &mut self,
        event: Option<Event>,
        user_id: Option<UserId>,
        channel_id: Option<ChannelId>,
        predicate: Option<Predicate>,
        handler: Handler,
    ) -> EventSubscriptionId {
        self.next_id += 1;
        let id = EventSubscriptionId(self.next_id);
        self.subscriptions.push(Subscription {
            id,
            event,
            user_id,
            channel_id,
            predicate,
            handler,
        });
        id
    }

    pub(crate) fn unsubscribe(&mut self, id: EventSubscriptionId) -> bool {
        let before = self.subscriptions.len();
        self.subscriptions.retain(|sub| sub.id != id);
        before != self.subscriptions.len()
    }

    pub(crate) fn clear(&mut self) {
        self.subscriptions.clear();
    }

    pub(crate) fn len(&self) -> usize {
        self.subscriptions.len()
    }

    pub(crate) fn dispatch(&mut self, client: &Client, event: Event, message: &Message) {
        for sub in self.subscriptions.iter_mut() {
            if !sub.matches(client, event, message) {
                continue;
            }
            let ctx = EventContext {
                event,
                message,
                client,
            };
            (sub.handler)(ctx);
        }
    }
}

struct Subscription {
    id: EventSubscriptionId,
    event: Option<Event>,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    predicate: Option<Predicate>,
    handler: Handler,
}

impl Subscription {
    fn matches(&mut self, client: &Client, event: Event, message: &Message) -> bool {
        if let Some(filter) = self.event {
            if mem::discriminant(&filter) != mem::discriminant(&event) {
                return false;
            }
        }
        if let Some(user_id) = self.user_id {
            let match_user = message
                .user()
                .map(|user| user.id == user_id)
                .or_else(|| message.text().map(|text| text.from_id == user_id))
                .unwrap_or(false);
            if !match_user {
                return false;
            }
        }
        if let Some(channel_id) = self.channel_id {
            let match_channel = message
                .user()
                .map(|user| user.channel_id == channel_id)
                .or_else(|| message.text().map(|text| text.channel_id == channel_id))
                .unwrap_or(false);
            if !match_channel {
                return false;
            }
        }
        if let Some(predicate) = self.predicate.as_mut() {
            let ctx = EventContext {
                event,
                message,
                client,
            };
            if !(predicate)(&ctx) {
                return false;
            }
        }
        true
    }
}

/// Builder for event subscriptions.
pub struct SubscriptionBuilder<'a> {
    client: &'a Client,
    event: Option<Event>,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    predicate: Option<Predicate>,
}

impl<'a> SubscriptionBuilder<'a> {
    pub(crate) fn new(client: &'a Client, event: Option<Event>) -> Self {
        Self {
            client,
            event,
            user_id: None,
            channel_id: None,
            predicate: None,
        }
    }

    /// Filters by a specific user id.
    pub fn filter_user(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Filters by a specific channel id.
    pub fn filter_channel(mut self, channel_id: ChannelId) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    /// Filters by a custom predicate.
    pub fn filter(mut self, predicate: impl FnMut(&EventContext) -> bool + Send + 'static) -> Self {
        self.predicate = Some(Box::new(predicate));
        self
    }

    /// Registers the subscription and returns its id.
    pub fn subscribe(
        self,
        handler: impl FnMut(EventContext) + Send + 'static,
    ) -> EventSubscriptionId {
        self.client.bus.borrow_mut().subscribe(
            self.event,
            self.user_id,
            self.channel_id,
            self.predicate,
            Box::new(handler),
        )
    }
}
