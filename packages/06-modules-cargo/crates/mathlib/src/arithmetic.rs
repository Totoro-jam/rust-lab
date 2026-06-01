//! Basic arithmetic operations.

/// Module-private helper (not accessible outside this module).
fn check_divisor(b: f64) -> Result<(), &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(())
    }
}

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    check_divisor(b)?;
    Ok(a / b)
}

/// Only visible within the crate (not to external users).
pub(crate) fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}
