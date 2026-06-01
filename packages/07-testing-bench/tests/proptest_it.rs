//! Property-based tests with proptest.

use proptest::prelude::*;
use rustlab07::{add, is_palindrome, reverse};

proptest! {
    #[test]
    fn add_is_commutative(a in -10000i64..10000, b in -10000i64..10000) {
        prop_assert_eq!(add(a, b), add(b, a));
    }

    #[test]
    fn add_zero_identity(a in -10000i64..10000) {
        prop_assert_eq!(add(a, 0), a);
    }

    #[test]
    fn reverse_twice_is_identity(s in "[a-zA-Z0-9]{0,100}") {
        prop_assert_eq!(reverse(&reverse(&s)), s);
    }

    #[test]
    fn palindrome_of_reversed_concat(s in "[a-z]{1,50}") {
        // s + reverse(s) is always a palindrome
        let palindrome = format!("{}{}", s, reverse(&s));
        prop_assert!(is_palindrome(&palindrome));
    }
}
