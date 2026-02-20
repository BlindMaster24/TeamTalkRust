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
