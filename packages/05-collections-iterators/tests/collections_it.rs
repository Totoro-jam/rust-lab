//! Integration tests for collections and iterators.

use rustlab05::fibonacci::Fibonacci;
use rustlab05::word_freq::{sorted_frequencies, top_n, word_frequencies};

#[test]
fn word_freq_punctuation_handling() {
    let text = "Hello, hello! World... world?";
    let freq = word_frequencies(text);
    assert_eq!(freq["hello"], 2);
    assert_eq!(freq["world"], 2);
    assert_eq!(freq.len(), 2);
}

#[test]
fn collect_into_result() {
    let strings = ["1", "2", "3", "4"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    assert_eq!(numbers.unwrap(), vec![1, 2, 3, 4]);

    let bad_strings = ["1", "oops", "3"];
    let result: Result<Vec<i32>, _> = bad_strings.iter().map(|s| s.parse::<i32>()).collect();
    assert!(result.is_err());
}

#[test]
fn fibonacci_overflow_stops() {
    let count = Fibonacci::new().count();
    // Fibonacci stops when u64 would overflow
    assert!(count > 80 && count < 100);
}

#[test]
fn iterator_chaining() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: i32 = data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
    // 4 + 16 + 36 + 64 + 100 = 220
    assert_eq!(result, 220);
}

#[test]
fn top_n_and_sorted() {
    let text = "rust rust rust go go python";
    let freq = word_frequencies(text);
    let top = top_n(&freq, 2);
    assert_eq!(top[0].0, "rust");
    assert_eq!(top[1].0, "go");

    let sorted = sorted_frequencies(text);
    let first_key = sorted.keys().next().unwrap();
    assert_eq!(first_key, "go"); // alphabetically first
}
