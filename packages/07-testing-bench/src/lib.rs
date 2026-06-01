//! rustlab07 — Testing & Benchmarking
//!
//! Provides functions to test with proptest, rstest, and benchmark with criterion.

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use rustlab07::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Divides a by b, returning None if b is zero.
///
/// # Examples
///
/// ```
/// use rustlab07::divide;
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(1, 0), None);
/// ```
pub fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Reverses a string.
///
/// # Examples
///
/// ```
/// use rustlab07::reverse;
/// assert_eq!(reverse("hello"), "olleh");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Checks if a string is a palindrome.
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = cleaned.to_lowercase();
    lower == lower.chars().rev().collect::<String>()
}

/// Fibonacci (iterative).
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn divide_by_zero_returns_none() {
        assert_eq!(divide(5, 0), None);
    }

    #[test]
    fn palindrome_detection() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn fibonacci_values() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
