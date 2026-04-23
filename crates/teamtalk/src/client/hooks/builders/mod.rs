//! Hook registration builders split by event category.
//!
//! All submodules add `impl ClientHooks` methods to the single
//! [`ClientHooks`] struct defined in the parent module. Each
//! `on_*` method follows the same pattern: take `mut self`, box the
//! provided closure, store it under the matching optional field, and
//! return `self` for chaining.
//!
//! The file was previously a single 574-line monolith; splitting by
//! event category makes it easier to find a hook for a given
//! subsystem (connection vs session vs directory vs media vs
//! reconnect) and shortens each file well under the ~400-600 line
//! budget in `AGENTS.md`.
//!
//! External callers continue to use the builders through the same
//! `ClientHooks::new().on_*(...)` chain; there are no new public
//! items and no re-exports are necessary.
//!
//! [`ClientHooks`]: super::ClientHooks

mod connection;
mod directory;
mod media;
mod reconnect;
mod session;

use super::ClientHooks;
use crate::client::{Client, Message};
use crate::events::Event;

impl ClientHooks {
    /// Registers a handler for every event.
    ///
    /// This is a catch-all hook invoked for every [`Event`] after any
    /// event-specific hook has run. It is intentionally kept in the
    /// top-level `builders/mod.rs` rather than under any subcategory
    /// because it does not correspond to a specific TeamTalk event.
    #[must_use]
    pub fn on_event(mut self, hook: impl FnMut(&Client, Event, &Message) + Send + 'static) -> Self {
        self.on_event = Some(Box::new(hook));
        self
    }
}
