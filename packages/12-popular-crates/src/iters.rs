//! Extended iterator combinators with `itertools`.
//!
//! Demonstrates chunk, unique, interleave, join, sorted, group_by,
//! and other combinators beyond what std provides.

use itertools::Itertools;

/// Groups consecutive equal elements and returns (element, count) pairs.
pub fn run_length_encode<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)> {
    items
        .iter()
        .dedup_with_count()
        .map(|(count, item)| (item.clone(), count))
        .collect()
}

/// Returns unique elements preserving first-seen order.
pub fn unique_stable(items: &[i32]) -> Vec<i32> {
    items.iter().copied().unique().collect()
}

/// Joins elements into a string with a separator.
pub fn join_with(items: &[&str], sep: &str) -> String {
    items.iter().join(sep)
}

/// Chunks a slice into groups of `size`, returning collected vectors.
pub fn chunk_data(items: &[i32], size: usize) -> Vec<Vec<i32>> {
    items
        .iter()
        .copied()
        .chunks(size)
        .into_iter()
        .map(|c| c.collect())
        .collect()
}

/// Returns all combinations of 2 elements.
pub fn pairs(items: &[i32]) -> Vec<(i32, i32)> {
    items
        .iter()
        .copied()
        .combinations(2)
        .map(|c| (c[0], c[1]))
        .collect()
}

/// Interleaves two slices element-by-element.
pub fn interleave_slices(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().interleave(b.iter()).copied().collect()
}

/// Finds the min and max in one pass using itertools.
pub fn min_max(items: &[i32]) -> Option<(i32, i32)> {
    items.iter().copied().minmax().into_option()
}

/// Cartesian product of two slices.
pub fn cartesian(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter()
        .copied()
        .cartesian_product(b.iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rle_encode() {
        let data = [1, 1, 2, 2, 2, 3, 1, 1];
        let encoded = run_length_encode(&data);
        assert_eq!(encoded, [(1, 2), (2, 3), (3, 1), (1, 2)]);
    }

    #[test]
    fn unique_preserves_order() {
        let data = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let unique = unique_stable(&data);
        assert_eq!(unique, [3, 1, 4, 5, 9, 2, 6]);
    }

    #[test]
    fn join_strings() {
        assert_eq!(join_with(&["a", "b", "c"], ", "), "a, b, c");
        assert_eq!(join_with(&["x"], "-"), "x");
        assert_eq!(join_with(&[], ", "), "");
    }

    #[test]
    fn chunk_into_groups() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        let chunks = chunk_data(&data, 3);
        assert_eq!(chunks, [vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn combination_pairs() {
        let data = [1, 2, 3];
        let result = pairs(&data);
        assert_eq!(result, [(1, 2), (1, 3), (2, 3)]);
    }

    #[test]
    fn interleave_two_slices() {
        let result = interleave_slices(&[1, 3, 5], &[2, 4, 6]);
        assert_eq!(result, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn interleave_uneven() {
        let result = interleave_slices(&[1, 3], &[2, 4, 6, 8]);
        assert_eq!(result, [1, 2, 3, 4, 6, 8]);
    }

    #[test]
    fn min_max_single_pass() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5, 9]), Some((1, 9)));
        assert_eq!(min_max(&[42]), Some((42, 42)));
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn cartesian_product() {
        let result = cartesian(&[1, 2], &[10, 20]);
        assert_eq!(result, [(1, 10), (1, 20), (2, 10), (2, 20)]);
    }
}
