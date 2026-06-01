//! 业务逻辑 + 模块内单元测试。
//!
//! 设计要点:
//!  - `divide` 用自定义错误 enum,显式区分"除零"和"其它"
//!  - `add` / `is_even` 是 `const fn`,常量上下文可调用
//!  - 单元测试与实现放在同一文件 (`#[cfg(test)] mod tests`),
//!    既能访问私有项,又不污染发布二进制

use std::fmt;

/// 计算可能出现的错误。
///
/// 真实项目里用 `thiserror` 写更省事,但第一章先手写,看清结构。
#[derive(Debug, PartialEq, Eq)]
pub enum CalcError {
    DivideByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

/// 两数相加。`const` → 编译期就能算。
pub const fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 整数除法。除数为 0 返回 `Err`,其它情况向 0 截断。
pub fn divide(a: i64, b: i64) -> Result<i64, CalcError> {
    if b == 0 {
        Err(CalcError::DivideByZero)
    } else {
        Ok(a / b)
    }
}

/// 判偶。负数也能正确判断 (Rust 的 `%` 符号跟被除数走)。
pub const fn is_even(n: i64) -> bool {
    n % 2 == 0
}

// ----------------------------------------------------------------------
// 单元测试: 在模块内,可以测私有项。
// `cargo test` 自动跑。
// ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn add_with_zero_and_negatives() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-3, 3), 0);
        assert_eq!(add(-3, -4), -7);
    }

    #[test]
    fn add_is_const() {
        // 在 const 上下文里调用 → 编译期求值
        const X: i64 = add(2, 3);
        assert_eq!(X, 5);
    }

    #[test]
    fn divide_normal() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn divide_by_zero_returns_err() {
        assert_eq!(divide(1, 0), Err(CalcError::DivideByZero));
    }

    #[test]
    fn divide_negatives_truncates_toward_zero() {
        assert_eq!(divide(-7, 2), Ok(-3)); // 不是 -4
    }

    #[test]
    fn is_even_works_for_negatives() {
        assert!(is_even(-4));
        assert!(!is_even(-3));
        assert!(is_even(0));
    }

    #[test]
    fn error_displays_human_message() {
        let e = CalcError::DivideByZero;
        assert_eq!(e.to_string(), "cannot divide by zero");
    }
}
