use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

pub trait StateStore: Send {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn remove(&mut self, key: &str) -> Option<String>;

    fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        let _ = ttl;
        self.set(key, value);
    }

    fn keys(&self, prefix: &str) -> Vec<String> {
        let _ = prefix;
        Vec::new()
    }

    fn remove_prefix(&mut self, prefix: &str) -> usize {
        let _ = prefix;
        0
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

struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self) -> bool {
        self.expires_at.is_some_and(|t| Instant::now() >= t)
    }
}

#[derive(Default)]
pub struct MemoryStateStore {
    inner: HashMap<String, Entry>,
}

impl MemoryStateStore {
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

#[cfg(feature = "bot-redis")]
pub struct RedisStateStore {
    conn: crate::utils::UnpoisonedMutex<redis::Connection>,
    key_prefix: String,
}

#[cfg(feature = "bot-redis")]
impl RedisStateStore {
    pub fn connect(url: &str) -> Result<Self, redis::RedisError> {
        Self::connect_with_prefix(url, "teamtalk:bot")
    }

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

#[cfg(feature = "bot-redis")]
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

#[cfg(feature = "bot-sqlite")]
pub struct SqliteStateStore {
    conn: rusqlite::Connection,
}

#[cfg(feature = "bot-sqlite")]
impl SqliteStateStore {
    pub fn in_memory() -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        let mut store = Self { conn };
        store.ensure_schema()?;
        Ok(store)
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(path)?;
        let mut store = Self { conn };
        store.ensure_schema()?;
        Ok(store)
    }

    fn ensure_schema(&mut self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS bot_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                expires_at TEXT
            )",
            [],
        )?;
        let has_expires: bool = self
            .conn
            .prepare("SELECT expires_at FROM bot_state LIMIT 0")
            .is_ok();
        if !has_expires {
            self.conn
                .execute("ALTER TABLE bot_state ADD COLUMN expires_at TEXT", [])?;
        }
        Ok(())
    }
}

#[cfg(feature = "bot-sqlite")]
impl StateStore for SqliteStateStore {
    fn get(&self, key: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT value FROM bot_state WHERE key = ?1 AND (expires_at IS NULL OR expires_at > datetime('now'))",
                [key],
                |row| row.get::<_, String>(0),
            )
            .ok()
    }

    fn set(&mut self, key: String, value: String) {
        let _ = self.conn.execute(
            "INSERT INTO bot_state(key, value, expires_at) VALUES(?1, ?2, NULL)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, expires_at = NULL",
            rusqlite::params![key, value],
        );
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        let old = self.get(key);
        let _ = self
            .conn
            .execute("DELETE FROM bot_state WHERE key = ?1", [key]);
        old
    }

    fn exists(&self, key: &str) -> bool {
        self.conn
            .query_row(
                "SELECT 1 FROM bot_state WHERE key = ?1 AND (expires_at IS NULL OR expires_at > datetime('now'))",
                [key],
                |row| row.get::<_, i32>(0),
            )
            .is_ok()
    }

    fn set_with_ttl(&mut self, key: String, value: String, ttl: Duration) {
        let expires_at = format!("datetime('now', '+{} seconds')", ttl.as_secs());
        let _ = self.conn.execute(
            "INSERT INTO bot_state(key, value, expires_at) VALUES(?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, expires_at = excluded.expires_at",
            rusqlite::params![key, value, expires_at],
        );
    }

    fn keys(&self, prefix: &str) -> Vec<String> {
        let pattern = format!("{prefix}%");
        let mut stmt = self
            .conn
            .prepare(
                "SELECT key FROM bot_state WHERE key LIKE ?1 AND (expires_at IS NULL OR expires_at > datetime('now'))",
            )
            .ok();
        let Some(ref mut stmt) = stmt else {
            return Vec::new();
        };
        let rows = stmt.query_map([pattern], |row| row.get::<_, String>(0));
        rows.ok()
            .map(|r| r.filter_map(|v| v.ok()).collect())
            .unwrap_or_default()
    }

    fn remove_prefix(&mut self, prefix: &str) -> usize {
        let pattern = format!("{prefix}%");
        self.conn
            .execute("DELETE FROM bot_state WHERE key LIKE ?1", [pattern])
            .unwrap_or(0)
    }

    fn get_many(&self, keys: &[&str]) -> Vec<Option<String>> {
        if keys.is_empty() {
            return Vec::new();
        }
        let placeholders: Vec<String> = keys
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect();
        let sql = format!(
            "SELECT key, value FROM bot_state WHERE key IN ({}) AND (expires_at IS NULL OR expires_at > datetime('now'))",
            placeholders.join(",")
        );
        let params: Vec<&str> = keys.to_vec();
        let mut stmt = match self.conn.prepare(&sql) {
            Ok(s) => s,
            Err(_) => return keys.iter().map(|_| None).collect(),
        };
        let found: HashMap<String, String> = stmt
            .query_map(rusqlite::params_from_iter(params), |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok((key, value))
            })
            .ok()
            .map(|rows| rows.filter_map(|r| r.ok()).collect())
            .unwrap_or_default();
        keys.iter().map(|k| found.get(*k).cloned()).collect()
    }

    fn set_many(&mut self, pairs: Vec<(String, String)>) {
        let tx = self.conn.transaction().ok();
        let Some(tx) = tx else {
            return;
        };
        for (k, v) in &pairs {
            let _ = tx.execute(
                "INSERT INTO bot_state(key, value, expires_at) VALUES(?1, ?2, NULL)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, expires_at = NULL",
                rusqlite::params![k, v],
            );
        }
        let _ = tx.commit();
    }
}
