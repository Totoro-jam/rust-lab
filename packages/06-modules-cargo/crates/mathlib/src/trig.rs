//! Trigonometry functions (behind "trig" feature flag).

use std::f64::consts::PI;

pub fn sin(degrees: f64) -> f64 {
    (degrees * PI / 180.0).sin()
}

pub fn cos(degrees: f64) -> f64 {
    (degrees * PI / 180.0).cos()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
