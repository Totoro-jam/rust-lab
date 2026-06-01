//! Shape trait and implementations demonstrating Rust's trait system.

use std::f64::consts::PI;
use std::fmt;

/// A trait for geometric shapes.
pub trait Shape: fmt::Debug {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    /// Default implementation using area.
    fn description(&self) -> String {
        format!("{:?} with area {:.2}", self, self.area())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn square(side: f64) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

// --- Generic functions ---

/// Returns the shape with the largest area (static dispatch via generics).
pub fn largest_area<T: Shape>(items: &[T]) -> Option<&T> {
    items
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
}

/// Accepts any Shape via `impl Trait` syntax (static dispatch).
pub fn print_area(shape: &impl Shape) -> String {
    format!("area = {:.2}", shape.area())
}

/// Sums areas of a heterogeneous collection (dynamic dispatch).
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// Returns a shape without revealing the concrete type.
pub fn make_unit_shape(circle: bool) -> Box<dyn Shape> {
    if circle {
        Box::new(Circle::new(1.0))
    } else {
        Box::new(Rectangle::square(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_and_perimeter() {
        let c = Circle::new(1.0);
        assert!((c.area() - PI).abs() < 1e-10);
        assert!((c.perimeter() - 2.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn rectangle_area() {
        let r = Rectangle::new(3.0, 4.0);
        assert!((r.area() - 12.0).abs() < 1e-10);
        assert!((r.perimeter() - 14.0).abs() < 1e-10);
    }

    #[test]
    fn largest_area_generic() {
        let shapes = vec![Circle::new(1.0), Circle::new(2.0), Circle::new(0.5)];
        let biggest = largest_area(&shapes).unwrap();
        assert_eq!(biggest.radius, 2.0);
    }

    #[test]
    fn trait_objects_heterogeneous() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle::new(1.0)),
            Box::new(Rectangle::new(2.0, 3.0)),
            Box::new(Triangle::new(3.0, 4.0, 5.0)),
        ];
        let total = total_area(&shapes);
        let expected = PI + 6.0 + 6.0; // circle(r=1) + rect(2x3) + triangle(3,4,5)
        assert!((total - expected).abs() < 1e-10);
    }

    #[test]
    fn default_method() {
        let c = Circle::new(2.0);
        let desc = c.description();
        assert!(desc.contains("area"));
    }
}
