//! Subscription storage, filter matching, and the fluent builder surface.
//!
//! This module owns the in-memory event-bus state ([`EventBus`] and the
//! per-subscription record [`Subscription`]) together with the public
//! [`SubscriptionBuilder`] that callers use to register handlers through
//! [`crate::client::Client::on_event`] / [`crate::client::Client::on_any`].
//!
//! Dispatch is indexed by `mem::Discriminant<Event>` so firing an event
//! only touches subscriptions that can possibly match it (event-specific
//! subscribers for this discriminant plus wildcard subscribers), instead
//! of scanning every registered subscription. Insertion order is
//! preserved across the specific-vs-wildcard partition by merging the
//! two index lists on the fly.

use super::context::EventContext;
use crate::client::{Client, Message};
use crate::events::Event;
use crate::types::{ChannelId, UserId};
use std::collections::HashMap;
use std::mem::{self, Discriminant};
use std::sync::atomic::Ordering;
use teamtalk_sys as ffi;

type Predicate = Box<dyn FnMut(&EventContext) -> bool + Send>;
type Handler = Box<dyn FnMut(EventContext) + Send>;

/// Identifier for an event subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventSubscriptionId(u64);

/// Identifier for a subscription group.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventSubscriptionGroup(String);

impl EventSubscriptionGroup {
    /// Creates a group identifier from a string.
    pub fn new(group: impl Into<String>) -> Self {
        Self(group.into())
    }
}

#[derive(Default)]
pub(crate) struct EventBus {
    next_id: u64,
    subscriptions: Vec<Subscription>,
    /// Indices into `subscriptions` keyed by the `Event` discriminant
    /// the subscription is filtering on. Each inner `Vec<usize>` is
    /// kept in ascending (insertion) order.
    by_event: HashMap<Discriminant<Event>, Vec<usize>>,
    /// Indices of subscriptions registered without an `event` filter
    /// (wildcard). Kept in ascending (insertion) order.
    any_indices: Vec<usize>,
}

#[derive(Default)]
pub(crate) struct SubscriptionConfig {
    event: Option<Event>,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    username: Option<String>,
    nickname: Option<String>,
    text_type: Option<ffi::TextMsgType>,
    group: Option<EventSubscriptionGroup>,
    predicate: Option<Predicate>,
}

impl EventBus {
    pub(crate) fn subscribe(
        &mut self,
        config: SubscriptionConfig,
        handler: Handler,
    ) -> EventSubscriptionId {
        self.next_id += 1;
        let id = EventSubscriptionId(self.next_id);
        let event_filter = config.event.as_ref().map(mem::discriminant);
        let index = self.subscriptions.len();
        self.subscriptions.push(Subscription {
            id,
            event_filter,
            user_id: config.user_id,
            channel_id: config.channel_id,
            username: config.username,
            nickname: config.nickname,
            text_type: config.text_type,
            group: config.group,
            predicate: config.predicate,
            handler,
        });
        match event_filter {
            Some(d) => self.by_event.entry(d).or_default().push(index),
            None => self.any_indices.push(index),
        }
        id
    }

    pub(crate) fn unsubscribe(&mut self, id: EventSubscriptionId) -> bool {
        let before = self.subscriptions.len();
        self.subscriptions.retain(|sub| sub.id != id);
        if self.subscriptions.len() == before {
            return false;
        }
        self.rebuild_indices();
        true
    }

    pub(crate) fn clear(&mut self) {
        self.subscriptions.clear();
        self.by_event.clear();
        self.any_indices.clear();
    }

    pub(crate) fn unsubscribe_group(&mut self, group: &EventSubscriptionGroup) -> usize {
        let before = self.subscriptions.len();
        self.subscriptions
            .retain(|sub| sub.group.as_ref() != Some(group));
        let removed = before.saturating_sub(self.subscriptions.len());
        if removed > 0 {
            self.rebuild_indices();
        }
        removed
    }

    pub(crate) fn len(&self) -> usize {
        self.subscriptions.len()
    }

    pub(crate) fn dispatch(&mut self, client: &Client, event: Event, message: &Message) {
        let d = mem::discriminant(&event);
        let empty: Vec<usize> = Vec::new();
        let specific: &[usize] = self.by_event.get(&d).unwrap_or(&empty);
        let any: &[usize] = &self.any_indices;
        let subs = &mut self.subscriptions;

        let mut si = 0;
        let mut ai = 0;
        while si < specific.len() || ai < any.len() {
            let pick = match (specific.get(si), any.get(ai)) {
                (Some(&s), Some(&a)) => {
                    if s <= a {
                        si += 1;
                        s
                    } else {
                        ai += 1;
                        a
                    }
                }
                (Some(&s), None) => {
                    si += 1;
                    s
                }
                (None, Some(&a)) => {
                    ai += 1;
                    a
                }
                (None, None) => break,
            };
            let sub = &mut subs[pick];
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

    fn rebuild_indices(&mut self) {
        self.by_event.clear();
        self.any_indices.clear();
        for (i, sub) in self.subscriptions.iter().enumerate() {
            match sub.event_filter {
                Some(d) => self.by_event.entry(d).or_default().push(i),
                None => self.any_indices.push(i),
            }
        }
    }
}

struct Subscription {
    id: EventSubscriptionId,
    event_filter: Option<Discriminant<Event>>,
    user_id: Option<UserId>,
    channel_id: Option<ChannelId>,
    username: Option<String>,
    nickname: Option<String>,
    text_type: Option<ffi::TextMsgType>,
    group: Option<EventSubscriptionGroup>,
    predicate: Option<Predicate>,
    handler: Handler,
}

impl Subscription {
    fn matches(&mut self, client: &Client, event: Event, message: &Message) -> bool {
        let user = message.user();
        let text = message.text();

        // Discriminant filter is already satisfied by the bucket in
        // `EventBus::dispatch`, so it is not re-checked here.
        if let Some(user_id) = self.user_id {
            let match_user = user
                .as_ref()
                .map(|user| user.id == user_id)
                .or_else(|| text.as_ref().map(|text| text.from_id == user_id))
                .unwrap_or(false);
            if !match_user {
                return false;
            }
        }
        if let Some(channel_id) = self.channel_id {
            let match_channel = user
                .as_ref()
                .map(|user| user.channel_id == channel_id)
                .or_else(|| text.as_ref().map(|text| text.channel_id == channel_id))
                .unwrap_or(false);
            if !match_channel {
                return false;
            }
        }
        if let Some(ref username) = self.username {
            let matches_username = text
                .as_ref()
                .is_some_and(|text| text.from_username == *username);
            if !matches_username {
                return false;
            }
        }
        if let Some(ref nickname) = self.nickname {
            let matches_nickname = user.as_ref().is_some_and(|u| u.nickname == *nickname);
            if !matches_nickname {
                return false;
            }
        }
        if let Some(text_type) = self.text_type {
            let matches_type = text.as_ref().is_some_and(|text| text.msg_type == text_type);
            if !matches_type {
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
    username: Option<String>,
    nickname: Option<String>,
    text_type: Option<ffi::TextMsgType>,
    group: Option<EventSubscriptionGroup>,
    predicate: Option<Predicate>,
}

impl<'a> SubscriptionBuilder<'a> {
    pub(crate) fn new(client: &'a Client, event: Option<Event>) -> Self {
        Self {
            client,
            event,
            user_id: None,
            channel_id: None,
            username: None,
            nickname: None,
            text_type: None,
            group: None,
            predicate: None,
        }
    }

    /// Filters by a specific user id.
    #[must_use]
    pub fn filter_user(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Filters by a specific channel id.
    #[must_use]
    pub fn filter_channel(mut self, channel_id: ChannelId) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    /// Filters by a text message type.
    #[must_use]
    pub fn filter_text_type(mut self, msg_type: ffi::TextMsgType) -> Self {
        self.text_type = Some(msg_type);
        self
    }

    /// Filters by the sender username in text events.
    #[must_use]
    pub fn filter_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Filters by nickname (user events).
    #[must_use]
    pub fn filter_nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }

    /// Attaches the subscription to a group id for bulk removal.
    #[must_use]
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(EventSubscriptionGroup::new(group.into()));
        self
    }

    /// Filters by a custom predicate.
    #[must_use]
    pub fn filter(mut self, predicate: impl FnMut(&EventContext) -> bool + Send + 'static) -> Self {
        self.predicate = Some(Box::new(predicate));
        self
    }

    /// Registers the subscription and returns its id.
    pub fn subscribe(
        self,
        handler: impl FnMut(EventContext) + Send + 'static,
    ) -> EventSubscriptionId {
        let config = SubscriptionConfig {
            event: self.event,
            user_id: self.user_id,
            channel_id: self.channel_id,
            username: self.username,
            nickname: self.nickname,
            text_type: self.text_type,
            group: self.group,
            predicate: self.predicate,
        };
        let id = self.client.bus.lock().subscribe(config, Box::new(handler));
        self.client.bus_revision.fetch_add(1, Ordering::Relaxed);
        id
    }
}
