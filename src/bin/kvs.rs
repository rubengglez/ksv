use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::errors::Errors;
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        /// key
        key: String,
    },
    Set {
        /// key
        key: String,
        /// value
        value: String,
    },
    Rm {
        /// key
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut kvs = KvStore::new();

    match &cli.command {
        Some(Commands::Get { key: _ }) => {
            eprintln!("unimplemented");
        }
        Some(Commands::Set { key, value }) => return kvs.set(key.to_string(), value.to_string()),
        Some(Commands::Rm { key }) => return kvs.remove(key.to_string()),
        None => return Err(Errors::NoCommand),
    }
    exit(-1);
}
