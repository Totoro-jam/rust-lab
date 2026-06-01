//! Integration tests for traits and generics.

use rustlab04::shapes::{make_unit_shape, total_area, Circle, Rectangle, Shape, Triangle};

#[test]
fn dynamic_dispatch_collection() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(5.0)),
        Box::new(Rectangle::new(10.0, 2.0)),
        Box::new(Triangle::new(3.0, 4.0, 5.0)),
    ];

    assert_eq!(shapes.len(), 3);
    let total = total_area(&shapes);
    assert!(total > 0.0);
}

#[test]
fn make_unit_shape_factory() {
    let circle = make_unit_shape(true);
    let square = make_unit_shape(false);

    assert!((circle.area() - std::f64::consts::PI).abs() < 1e-10);
    assert!((square.area() - 1.0).abs() < 1e-10);
}

#[test]
fn impl_trait_print_area() {
    let r = Rectangle::new(4.0, 5.0);
    let output = rustlab04::shapes::print_area(&r);
    assert!(output.contains("20.00"));
}
