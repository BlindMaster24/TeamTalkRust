//! Internal event bus: fluent subscription builder and dispatch storage.
//!
//! Public surface (handler-facing):
//!
//! * [`EventContext`] - read-only view passed to every subscribed closure.
//! * [`EventSubscriptionId`] / [`EventSubscriptionGroup`] - opaque handles
//!   used to cancel individual subscriptions or whole groups.
//! * [`SubscriptionBuilder`] - fluent filter + register entry point
//!   returned by [`crate::client::Client::on_event`] /
//!   [`crate::client::Client::on_any`].
//!
//! Internal surface (`pub(crate)`): [`EventBus`] plus its
//! `subscribe`/`unsubscribe`/`dispatch` plumbing live in
//! [`subscription`] and are reached through the `bus::` path from
//! `client::core` so the storage stays invisible to downstream crates.

mod context;
mod subscription;

pub use context::EventContext;
pub(crate) use subscription::EventBus;
pub use subscription::{EventSubscriptionGroup, EventSubscriptionId, SubscriptionBuilder};
