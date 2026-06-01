//! Data parallelism with rayon.
//!
//! rayon turns sequential iterators into parallel ones with `.par_iter()`.
//! The thread pool is managed automatically.

use rayon::prelude::*;

/// Sums squares of numbers in parallel.
pub fn parallel_sum_of_squares(data: &[i64]) -> i64 {
    data.par_iter().map(|&x| x * x).sum()
}

/// Filters and transforms in parallel, returning sorted results.
pub fn parallel_filter_map(data: &[i64], threshold: i64) -> Vec<i64> {
    let mut result: Vec<i64> = data
        .par_iter()
        .filter(|&&x| x > threshold)
        .map(|&x| x * 2)
        .collect();
    result.sort();
    result
}

/// Parallel sort (in-place).
pub fn parallel_sort(data: &mut [i64]) {
    data.par_sort();
}

/// Finds any element matching a predicate in parallel.
pub fn parallel_find(data: &[i64], target: i64) -> Option<i64> {
    data.par_iter().find_any(|&&x| x == target).copied()
}

/// Parallel word count: splits text into chunks and counts in parallel.
pub fn parallel_word_count(texts: &[&str]) -> usize {
    texts
        .par_iter()
        .map(|text| text.split_whitespace().count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_squares() {
        let data: Vec<i64> = (1..=100).collect();
        let result = parallel_sum_of_squares(&data);
        let expected: i64 = (1..=100).map(|x: i64| x * x).sum();
        assert_eq!(result, expected);
    }

    #[test]
    fn filter_map_parallel() {
        let data = vec![1, 5, 3, 8, 2, 9, 4, 7, 6];
        let result = parallel_filter_map(&data, 5);
        assert_eq!(result, vec![12, 14, 16, 18]);
    }

    #[test]
    fn sort_parallel() {
        let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
        parallel_sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn find_parallel() {
        let data: Vec<i64> = (0..1000).collect();
        assert_eq!(parallel_find(&data, 42), Some(42));
        assert_eq!(parallel_find(&data, 9999), None);
    }

    #[test]
    fn word_count_parallel() {
        let texts = vec!["hello world", "foo bar baz", "one"];
        assert_eq!(parallel_word_count(&texts), 6);
    }
}
