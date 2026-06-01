//! Integration tests for mathlib.

#[test]
fn reexported_arithmetic() {
    assert_eq!(mathlib::add(2.0, 3.0), 5.0);
    assert_eq!(mathlib::subtract(10.0, 4.0), 6.0);
    assert_eq!(mathlib::multiply(3.0, 7.0), 21.0);
    assert_eq!(mathlib::divide(10.0, 2.0), Ok(5.0));
}

#[test]
fn divide_by_zero() {
    assert_eq!(mathlib::divide(1.0, 0.0), Err("division by zero"));
}

#[cfg(feature = "trig")]
#[test]
fn trig_functions() {
    let sin_90 = mathlib::sin(90.0);
    assert!((sin_90 - 1.0).abs() < 1e-10);

    let cos_0 = mathlib::cos(0.0);
    assert!((cos_0 - 1.0).abs() < 1e-10);
}

#[cfg(feature = "stats")]
#[test]
fn stats_mean_median() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(mathlib::mean(&data), Some(3.0));
    assert_eq!(mathlib::median(&data), Some(3.0));

    let even_data = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(mathlib::median(&even_data), Some(2.5));
}
