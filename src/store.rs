use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    pub fn values(&self) -> Vec<String> {
        self.data.values().cloned().collect()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut String)> {
        self.data.iter_mut()
    }
    pub fn drain(&mut self) -> impl Iterator<Item = (String, String)> {
        self.data.drain()
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}