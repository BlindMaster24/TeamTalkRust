//! Bot state storage backends.
//!
//! Defines the [`StateStore`] trait consumed by the bot runtime and
//! ships three implementations:
//!
//! * [`MemoryStateStore`] - always available, in-process `HashMap` with
//!   TTL semantics via [`std::time::Instant`].
//! * [`RedisStateStore`] - gated behind the `bot-redis` feature;
//!   prefixes every key with a shared namespace and uses `SETEX`/`SCAN`
//!   for TTL and enumeration.
//! * [`SqliteStateStore`] - gated behind the `bot-sqlite` feature;
//!   stores entries in a single `bot_state` table with an optional
//!   `expires_at` column evaluated against `datetime('now')`.
//!
//! Each backend is kept in its own file so the feature-gated code
//! (together with its `use redis::...` / `use rusqlite::...`) stays
//! localized and does not pull dependencies into the always-on
//! trait surface.

mod memory;
#[cfg(feature = "bot-redis")]
mod redis_store;
#[cfg(feature = "bot-sqlite")]
mod sqlite_store;

use std::time::Duration;

/// Pluggable bot state store.
///
/// The bot runtime depends only on this trait; the concrete backend
/// (in-memory, Redis, SQLite, or a user-supplied implementation) is
/// chosen at construction time.
pub trait StateStore: Send {
    /// Returns the current value for `key`, if present and not
    /// expired.
    fn get(&self, key: &str) -> Option<String>;

    /// Stores `value` at `key` without any expiration.
    fn set(&mut self, key: String, value: String);

    /// Removes `key` and returns its previous value, if any.
    fn remove(&mut self, key: &str) -> Option<String>;

    /// Returns `true` if `key` is present (and not expired).
    fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Stores `value` at `key` with a TTL. Default implementation
    /// falls back to [`StateStore::set`] - backends without native
    /// TTL support should override.
    fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        let _ = ttl;
        self.set(key, value);
    }

    /// Returns all keys that start with `prefix`. Default returns an
    /// empty vector for backends that do not support enumeration.
    fn keys(&self, prefix: &str) -> Vec<String> {
        let _ = prefix;
        Vec::new()
    }

    /// Removes every key that starts with `prefix`, returning the
    /// number of entries removed. Default is a no-op.
    fn remove_prefix(&mut self, prefix: &str) -> usize {
        let _ = prefix;
        0
    }

    /// Returns values for `keys` in the same order, using `None` for
    /// missing entries. Default implementation fans out to
    /// [`StateStore::get`].
    fn get_many(&self, keys: &[&str]) -> Vec<Option<String>> {
        keys.iter().map(|k| self.get(k)).collect()
    }

    /// Stores every `(key, value)` pair. Default implementation fans
    /// out to [`StateStore::set`]; backends supporting pipelines or
    /// transactions should override.
    fn set_many(&mut self, pairs: Vec<(String, String)>) {
        for (k, v) in pairs {
            self.set(k, v);
        }
    }
}

pub use memory::MemoryStateStore;
#[cfg(feature = "bot-redis")]
pub use redis_store::RedisStateStore;
#[cfg(feature = "bot-sqlite")]
pub use sqlite_store::SqliteStateStore;
