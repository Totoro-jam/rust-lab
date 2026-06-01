//! 集成测试: 跟外部使用者一样,只调公开 API。
//!
//! 与 `src/calc.rs` 末尾的 `#[cfg(test)] mod tests` 不同 —
//! 集成测试不能访问 crate 私有项。这是好事:逼着你把 API 设得能用。

use rustlab01::calc::{self, CalcError};

#[test]
fn end_to_end_arithmetic() {
    assert_eq!(calc::add(2, 3), 5);
    assert_eq!(calc::divide(20, 4), Ok(5));
    assert!(calc::is_even(0));
}

#[test]
fn divide_by_zero_surface() {
    let err = calc::divide(1, 0).unwrap_err();
    assert_eq!(err, CalcError::DivideByZero);
    assert_eq!(err.to_string(), "cannot divide by zero");
}
