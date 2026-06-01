//! mathlib — A small math library demonstrating modules, visibility, and features.
//!
//! Public API is controlled via `pub use` re-exports.

pub mod arithmetic;

#[cfg(feature = "trig")]
pub mod trig;

#[cfg(feature = "stats")]
pub mod stats;

// Re-export commonly used items at crate root.
pub use arithmetic::{add, divide, multiply, subtract};

#[cfg(feature = "trig")]
pub use trig::{cos, sin};

#[cfg(feature = "stats")]
pub use stats::{mean, median};
