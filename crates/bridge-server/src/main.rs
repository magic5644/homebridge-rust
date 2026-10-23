use bridge_core::config::load_config;
use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Minimal home automation bridge CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate private configuration without loading credentials or connecting.
    Check {
        #[arg(long, value_name = "PATH")]
        config: PathBuf,
    },
}

fn main() -> ExitCode {
    let Cli {
        command: Commands::Check { config },
    } = Cli::parse();
    match load_config(&config) {
        Ok(_) => {
            println!("Configuration is valid.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Cannot validate configuration: {error}");
            ExitCode::FAILURE
        }
    }
}
