use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Minimal home automation bridge CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check configuration readability (schema validation is not implemented yet).
    Check {
        #[arg(long, value_name = "PATH")]
        config: PathBuf,
    },
}

fn main() -> ExitCode {
    let Cli {
        command: Commands::Check { config },
    } = Cli::parse();
    match fs::read_to_string(&config) {
        Ok(_) => {
            println!("Configuration is readable; schema validation is not implemented yet.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Cannot read configuration '{}': {error}", config.display());
            ExitCode::FAILURE
        }
    }
}
