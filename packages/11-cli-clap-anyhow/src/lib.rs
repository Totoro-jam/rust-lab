//! rustlab11 — CLI: clap + anyhow + tracing
//!
//! Library code for the calculator CLI.

use anyhow::{bail, Context, Result};

/// Adds two numbers.
pub fn add(a: i64, b: i64) -> Result<i64> {
    a.checked_add(b).context("integer overflow in addition")
}

/// Subtracts b from a.
pub fn subtract(a: i64, b: i64) -> Result<i64> {
    a.checked_sub(b).context("integer overflow in subtraction")
}

/// Multiplies two numbers.
pub fn multiply(a: i64, b: i64) -> Result<i64> {
    a.checked_mul(b)
        .context("integer overflow in multiplication")
}

/// Divides a by b.
pub fn divide(a: i64, b: i64) -> Result<i64> {
    if b == 0 {
        bail!("cannot divide by zero");
    }
    Ok(a / b)
}

/// Checks if a number is even.
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}
