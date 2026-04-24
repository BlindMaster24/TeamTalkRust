//! [`StateStore`] implementation backed by Redis.
//!
//! Every key is stored under a configurable namespace prefix (default
//! `teamtalk:bot`) so multiple bots can share a single Redis instance
//! without key collisions. TTL uses `SETEX`; enumeration uses `SCAN`
//! with `MATCH`.

use super::StateStore;
use std::time::Duration;

/// [`StateStore`] implementation that stores entries in Redis under a
/// shared key prefix.
pub struct RedisStateStore {
    conn: crate::utils::UnpoisonedMutex<redis::Connection>,
    key_prefix: String,
}

impl RedisStateStore {
    /// Connects to Redis using the default `teamtalk:bot` key prefix.
    pub fn connect(url: &str) -> Result<Self, redis::RedisError> {
        Self::connect_with_prefix(url, "teamtalk:bot")
    }

    /// Connects to Redis with an explicit key prefix.
    pub fn connect_with_prefix(url: &str, prefix: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn: crate::utils::UnpoisonedMutex::new(conn),
            key_prefix: prefix.to_owned(),
        })
    }

    fn key(&self, key: &str) -> String {
        format!("{}:{key}", self.key_prefix)
    }

    fn prefixed_key_pattern(&self, prefix: &str) -> String {
        format!("{}:{prefix}*", self.key_prefix)
    }
}

impl StateStore for RedisStateStore {
    fn get(&self, key: &str) -> Option<String> {
        use redis::Commands;
        let full = self.key(key);
        let mut conn = self.conn.lock();
        conn.get(full).ok()
    }

    fn set(&mut self, key: String, value: String) {
        use redis::Commands;
        let full = self.key(&key);
        let mut conn = self.conn.lock();
        let _: redis::RedisResult<()> = conn.set(full, value);
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        use redis::Commands;
        let full = self.key(key);
        let mut conn = self.conn.lock();
        let old = conn.get(full.clone()).ok();
        let _: redis::RedisResult<usize> = conn.del(full);
        old
    }

    fn exists(&self, key: &str) -> bool {
        use redis::Commands;
        let full = self.key(key);
        let mut conn = self.conn.lock();
        conn.exists(full).unwrap_or(false)
    }

    fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        use redis::Commands;
        let full = self.key(&key);
        let secs = ttl.as_secs();
        let mut conn = self.conn.lock();
        let _: redis::RedisResult<()> = conn.set_ex(full, value, secs);
    }

    fn keys(&self, prefix: &str) -> Vec<String> {
        use redis::Commands;
        let pattern = self.prefixed_key_pattern(prefix);
        let prefix_len = self.key_prefix.len() + 1;
        let mut conn = self.conn.lock();
        let mut result = Vec::new();
        let mut iter: redis::Iter<'_, String> = match conn.scan_match(&pattern) {
            Ok(it) => it,
            Err(_) => return result,
        };
        while let Some(Ok(full_key)) = iter.next() {
            if full_key.len() > prefix_len {
                result.push(full_key[prefix_len..].to_owned());
            }
        }
        result
    }

    fn remove_prefix(&mut self, prefix: &str) -> usize {
        use redis::Commands;
        let pattern = self.prefixed_key_pattern(prefix);
        let mut conn = self.conn.lock();
        let mut keys_to_delete: Vec<String> = Vec::new();
        let mut iter: redis::Iter<'_, String> = match conn.scan_match(&pattern) {
            Ok(it) => it,
            Err(_) => return 0,
        };
        while let Some(Ok(full_key)) = iter.next() {
            keys_to_delete.push(full_key);
        }
        let count = keys_to_delete.len();
        if !keys_to_delete.is_empty() {
            let mut pipe = redis::Pipeline::with_capacity(keys_to_delete.len());
            pipe.atomic();
            for k in &keys_to_delete {
                pipe.del(k);
            }
            let _: redis::RedisResult<()> = pipe.query(&mut *conn);
        }
        count
    }

    fn get_many(&self, keys: &[&str]) -> Vec<Option<String>> {
        use redis::Commands;
        let full_keys: Vec<String> = keys.iter().map(|k| self.key(k)).collect();
        let mut conn = self.conn.lock();
        let values: Vec<Option<String>> = conn.mget(&full_keys).unwrap_or_default();
        values
    }

    fn set_many(&mut self, pairs: Vec<(String, String)>) {
        let mut conn = self.conn.lock();
        let mut pipe = redis::Pipeline::with_capacity(pairs.len());
        pipe.atomic();
        for (k, v) in pairs {
            let full = self.key(&k);
            pipe.set(full, v);
        }
        let _: redis::RedisResult<()> = pipe.query(&mut *conn);
    }
}
