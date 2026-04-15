use std::collections::HashMap;

pub trait StateStore: Send {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn remove(&mut self, key: &str) -> Option<String>;
}

#[derive(Default)]
pub struct MemoryStateStore {
    inner: HashMap<String, String>,
}

impl MemoryStateStore {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl StateStore for MemoryStateStore {
    fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.inner.remove(key)
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
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}

#[cfg(feature = "bot-sqlite")]
impl StateStore for SqliteStateStore {
    fn get(&self, key: &str) -> Option<String> {
        self.conn
            .query_row("SELECT value FROM bot_state WHERE key = ?1", [key], |row| {
                row.get::<_, String>(0)
            })
            .ok()
    }

    fn set(&mut self, key: String, value: String) {
        let _ = self.conn.execute(
            "INSERT INTO bot_state(key, value) VALUES(?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
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
}
