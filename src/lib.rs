use errors::Errors;
use std::{collections::HashMap, path::PathBuf};

pub mod errors;

pub type Result<T> = core::result::Result<T, Errors>;

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
