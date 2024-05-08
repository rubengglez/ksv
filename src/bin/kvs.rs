use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::errors::Errors;
use kvs::Result;

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

    match &cli.command {
        Some(Commands::Get { key: _ }) => {
            eprintln!("unimplemented");
        }
        Some(Commands::Set { key: _, value: _ }) => {
            eprintln!("unimplemented");
        }
        Some(Commands::Rm { key: _ }) => {
            eprintln!("unimplemented");
        }
        None => return Err(Errors::NoCommand),
    }
    exit(-1);
}
