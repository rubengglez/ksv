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
        Some(Commands::Get { key }) => match kvs.get(key.to_string()) {
            Ok(value) => {
                let message = value.or(Some("Key not found".to_owned()));
                println!("{}", message.unwrap());
                exit(0);
            }
            Err(error) => {
                println!("{}", error);
                exit(-1);
            }
        },
        Some(Commands::Set { key, value }) => match kvs.set(key.to_string(), value.to_string()) {
            Ok(_) => exit(0),
            Err(error) => {
                println!("{}", error);
                exit(-1);
            }
        },
        Some(Commands::Rm { key }) => match kvs.remove(key.to_string()) {
            Ok(_) => exit(0),
            Err(_) => {
                println!("Key not found");
                exit(-1);
            }
        },
        None => Err(Errors::NoCommand),
    }
}
