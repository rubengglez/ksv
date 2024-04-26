use std::process::exit;

use clap::{Parser, Subcommand};

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

fn main() {
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
        None => {}
    }
    exit(-1)
}
