use errors::Errors;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, LineWriter};
use std::{collections::HashMap, fs::File, path::PathBuf};

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

    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore::new())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Commands::Set(key.clone(), value.clone());
        self.write(command)?;
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        for line in self.reader()?.lines() {
            let serialized_value = line?;
            let command = serde_json::from_str::<Commands>(&serialized_value)?;
            match command {
                Commands::Set(key, value) => {
                    self.store.insert(key, value);
                }
                Commands::Remove(key) => {
                    self.store.remove(&key);
                }
            }
        }
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        for line in self.reader()?.lines() {
            let serialized_value = line?;
            let command = serde_json::from_str::<Commands>(&serialized_value)?;
            match command {
                Commands::Set(key, value) => {
                    self.store.insert(key, value);
                }
                Commands::Remove(key) => {
                    self.store.remove(&key);
                }
            }
        }

        match self.store.get(&key) {
            Some(_) => {
                let command = Commands::Remove(key.clone());
                self.write(command)?;
                self.store.remove(&key);
                Ok(())
            }
            None => Err(Errors::ImpossibleRemoveKeyDoesNotExist),
        }
    }

    fn reader(&self) -> Result<BufReader<File>> {
        let file = OpenOptions::new().read(true).open("db.txt")?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    fn writer(&self) -> Result<LineWriter<File>> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("db.txt")?;
        let writer = LineWriter::new(file);
        Ok(writer)
    }

    fn write(&self, command: Commands) -> Result<()> {
        let serialized_command = serde_json::to_string(&command)?;
        let mut writer = self.writer()?;
        writer.write_all(format!("{}\n", serialized_command).as_bytes())?;
        Ok(())
    }
}
