#![allow(unused_imports)]
#![allow(dead_code)]

mod shared;
mod solutions;
mod utils;

use clap::{Parser, Subcommand};
use tracing::{debug, error, info, trace, warn, Level};

use solutions::day01;
use solutions::day02;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Controls the verbosity of the program.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Solve day 01 problems
    Day01(day01::cli::CliArgs),

    /// Solve day 02 problems
    Day02(day02::cli::CliArgs),
}

pub fn main() {
    let cli = Cli::parse();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_level(true)
        // .without_time()
        .with_max_level(match cli.verbosity {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Tracing subscriber should set the global default.");

    match &cli.command {
        SubCommands::Day01(args) => {
            day01::solver::solve(args);
        }
        SubCommands::Day02(args) => {
            day02::solver::solve(args);
        }
    }
}
