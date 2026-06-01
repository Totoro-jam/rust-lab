//! calc CLI using clap derive with subcommands.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "calc", version, about = "A calculator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Enable verbose (debug) output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Add two numbers
    Add {
        /// First operand
        a: i64,
        /// Second operand
        b: i64,
    },
    /// Subtract b from a
    Sub { a: i64, b: i64 },
    /// Multiply two numbers
    Mul { a: i64, b: i64 },
    /// Divide a by b
    Divide { a: i64, b: i64 },
    /// Check if a number is even
    Even { n: i64 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing based on --verbose flag
    let filter = if cli.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .init();

    info!("Starting calc CLI");

    let result = match cli.command {
        Command::Add { a, b } => {
            info!(a, b, "performing addition");
            let r = rustlab11::add(a, b)?;
            r.to_string()
        }
        Command::Sub { a, b } => {
            info!(a, b, "performing subtraction");
            let r = rustlab11::subtract(a, b)?;
            r.to_string()
        }
        Command::Mul { a, b } => {
            info!(a, b, "performing multiplication");
            let r = rustlab11::multiply(a, b)?;
            r.to_string()
        }
        Command::Divide { a, b } => {
            info!(a, b, "performing division");
            let r = rustlab11::divide(a, b)?;
            r.to_string()
        }
        Command::Even { n } => {
            info!(n, "checking parity");
            rustlab11::is_even(n).to_string()
        }
    };

    println!("{result}");
    Ok(())
}
