//! Parameterized tests with rstest.

use rstest::rstest;
use rustlab07::{add, divide, fibonacci};

#[rstest]
#[case(0, 0, 0)]
#[case(1, 2, 3)]
#[case(-1, 1, 0)]
#[case(100, -50, 50)]
#[case(i64::MAX, 0, i64::MAX)]
fn add_cases(#[case] a: i64, #[case] b: i64, #[case] expected: i64) {
    assert_eq!(add(a, b), expected);
}

#[rstest]
#[case(10, 2, Some(5))]
#[case(7, 3, Some(2))]
#[case(0, 5, Some(0))]
#[case(1, 0, None)]
#[case(-10, 3, Some(-3))]
fn divide_cases(#[case] a: i64, #[case] b: i64, #[case] expected: Option<i64>) {
    assert_eq!(divide(a, b), expected);
}

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(5, 5)]
#[case(10, 55)]
#[case(20, 6765)]
fn fibonacci_cases(#[case] n: u32, #[case] expected: u64) {
    assert_eq!(fibonacci(n), expected);
}
