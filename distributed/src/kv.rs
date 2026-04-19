//! Replicated key-value store

use alloc::collections::BTreeMap;
use alloc::string::String;

pub struct KeyValueStore {
    data: BTreeMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}
