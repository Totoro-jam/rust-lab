//! Application entry point using `anyhow` for error propagation.
//!
//! Demonstrates `.context()` to add human-readable context to errors.

use anyhow::{Context, Result};
use rustlab03::config;

fn run() -> Result<()> {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        // Default demo config
        String::from("host=127.0.0.1\nport=9090\ndebug=false\napp_name=demo")
    });

    // If the argument looks like a file path, try to read it
    let content = if input.contains('=') {
        input
    } else {
        std::fs::read_to_string(&input)
            .with_context(|| format!("failed to read config file: {input}"))?
    };

    let cfg = config::parse_config(&content).context("failed to parse config")?;

    println!("Loaded config:");
    println!("  host:  {}", cfg.host);
    println!("  port:  {}", cfg.port);
    println!("  debug: {}", cfg.debug);
    if !cfg.extras.is_empty() {
        println!("  extras: {:?}", cfg.extras);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
