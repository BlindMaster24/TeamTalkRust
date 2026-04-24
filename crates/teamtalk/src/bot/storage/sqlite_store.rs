//! [`StateStore`] implementation backed by SQLite.
//!
//! Stores all entries in a single `bot_state` table with columns
//! `(key PRIMARY KEY, value, expires_at)`. TTL is evaluated against
//! `datetime('now')` on every read so callers do not need a background
//! sweeper.

use super::StateStore;
use std::collections::HashMap;
use std::time::Duration;

/// [`StateStore`] implementation that stores entries in a SQLite
/// database (in-memory or file-backed).
pub struct SqliteStateStore {
    conn: rusqlite::Connection,
}

impl SqliteStateStore {
    /// Opens an in-memory SQLite database and creates the schema.
    pub fn in_memory() -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        let mut store = Self { conn };
        store.ensure_schema()?;
        Ok(store)
    }

    /// Opens a file-backed SQLite database and creates the schema.
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
