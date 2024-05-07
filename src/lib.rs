use std::{collections::HashMap, error::Error, path::PathBuf};

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::default(),
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore::new())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }
}
