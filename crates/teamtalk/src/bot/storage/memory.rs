//! In-process [`StateStore`] backed by a `HashMap` with per-entry TTL.

use super::StateStore;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self) -> bool {
        self.expires_at.is_some_and(|t| Instant::now() >= t)
    }
}

/// [`StateStore`] implementation that keeps everything in an in-process
/// `HashMap`. TTL is tracked via [`Instant`] and checked lazily on
/// every read.
#[derive(Default)]
pub struct MemoryStateStore {
    inner: HashMap<String, Entry>,
}

impl MemoryStateStore {
    /// Creates an empty in-memory store.
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl StateStore for MemoryStateStore {
    fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).and_then(|e| {
            if e.is_expired() {
                None
            } else {
                Some(e.value.clone())
            }
        })
    }

    fn set(&mut self, key: String, value: String) {
        self.inner.insert(
            key,
            Entry {
                value,
                expires_at: None,
            },
        );
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.inner.remove(key).map(|e| e.value)
    }

    fn exists(&self, key: &str) -> bool {
        self.inner.get(key).is_some_and(|e| !e.is_expired())
    }

    fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        self.inner.insert(
            key,
            Entry {
                value,
                expires_at: Some(Instant::now() + ttl),
            },
        );
    }

    fn keys(&self, prefix: &str) -> Vec<String> {
        self.inner
            .iter()
            .filter(|(_, e)| !e.is_expired())
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, _)| k.clone())
            .collect()
    }

    fn remove_prefix(&mut self, prefix: &str) -> usize {
        let before = self.inner.len();
        self.inner.retain(|k, _| !k.starts_with(prefix));
        before - self.inner.len()
    }

    fn get_many(&self, keys: &[&str]) -> Vec<Option<String>> {
        keys.iter().map(|k| self.get(k)).collect()
    }

    fn set_many(&mut self, pairs: Vec<(String, String)>) {
        for (k, v) in pairs {
            self.set(k, v);
        }
    }
}
