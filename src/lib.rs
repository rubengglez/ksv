use errors::Errors;
use serde::{Deserialize, Serialize};
use serde_json::{Result as OtherResult, Value};
use std::{collections::HashMap, path::PathBuf};

pub mod errors;

pub type Result<T> = anyhow::Result<T, Errors>;

#[derive(Serialize, Deserialize)]
enum Commands {
    Set(String, String),
    Remove(String),
}

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
        let command = Commands::Set(key.clone(), value.clone());
        let serialized_command = serde_json::to_string(&command)?;
        self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Commands::Remove(key.clone());
        let serialized_command = serde_json::to_string(&command)?;
        self.store.remove(&key);
        Ok(())
    }
}
