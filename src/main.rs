use std::process::ExitCode;
use std::process::Termination;

use clap::{Parser, Subcommand};

// extern crate bean_rs;
use bean_rs::balance;
use bean_rs::error::BeanError;
use bean_rs::utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display account balances
    Balance { path: String },
    /// Check for errors and quit
    Check { path: String },
}

#[derive(Debug)]
struct CliError {}

impl Termination for CliError {
    fn report(self) -> ExitCode {
        ExitCode::FAILURE
    }
}

fn set_exit(errs: &[BeanError]) -> ExitCode {
    if errs.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Balance { path } => {
            let (bals, errs) = balance(path);
            utils::print_errors(&errs);
            utils::print_bals(bals);
            set_exit(&errs)
        }
        Commands::Check { path } => {
            let (_, errs) = balance(path);
            utils::print_errors(&errs);
            set_exit(&errs)
        }
    }
}
