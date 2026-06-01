//! Integration tests for popular crates module.

use rustlab12::buffers::{build_message, read_message};
use rustlab12::datetime::{add_days, days_between, is_leap_year};
use rustlab12::ids::{generate_batch, generate_id, is_valid_uuid, parse_id};
use rustlab12::iters::{cartesian, chunk_data, interleave_slices, min_max, unique_stable};
use rustlab12::parallel::{parallel_filter_map, parallel_sort, parallel_sum_of_squares};
use rustlab12::text::{extract_numbers, is_valid_email, normalize_whitespace};

#[test]
fn parallel_large_dataset() {
    let data: Vec<i64> = (1..=10_000).collect();
    let sum = parallel_sum_of_squares(&data);
    assert!(sum > 0);
}

#[test]
fn parallel_filter_and_sort() {
    let data: Vec<i64> = (1..=20).collect();
    let result = parallel_filter_map(&data, 15);
    assert_eq!(result, vec![32, 34, 36, 38, 40]);
}

#[test]
fn parallel_sort_large() {
    let mut data: Vec<i64> = (0..1000).rev().collect();
    parallel_sort(&mut data);
    assert_eq!(data, (0..1000).collect::<Vec<_>>());
}

#[test]
fn regex_email_validation() {
    assert!(is_valid_email("test@example.com"));
    assert!(!is_valid_email("bad@"));
}

#[test]
fn regex_number_extraction() {
    let nums = extract_numbers("temp is -5 to 35 degrees");
    assert_eq!(nums, vec![-5, 35]);
}

#[test]
fn regex_whitespace_normalize() {
    let result = normalize_whitespace("  too   many   spaces  ");
    assert_eq!(result, " too many spaces ");
}

#[test]
fn chrono_date_arithmetic() {
    assert_eq!(days_between("2024-01-01", "2024-12-31"), Some(365));
    assert_eq!(add_days("2024-02-28", 1), Some("2024-02-29".into()));
    assert_eq!(add_days("2023-02-28", 1), Some("2023-03-01".into()));
}

#[test]
fn chrono_leap_years() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2100));
    assert!(is_leap_year(2400));
}

#[test]
fn itertools_combinations() {
    let chunks = chunk_data(&[1, 2, 3, 4, 5], 2);
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0], vec![1, 2]);
    assert_eq!(chunks[2], vec![5]);
}

#[test]
fn itertools_unique_and_interleave() {
    let unique = unique_stable(&[1, 2, 1, 3, 2, 4]);
    assert_eq!(unique, vec![1, 2, 3, 4]);

    let mixed = interleave_slices(&[1, 3], &[2, 4]);
    assert_eq!(mixed, vec![1, 2, 3, 4]);
}

#[test]
fn itertools_min_max_and_cartesian() {
    assert_eq!(min_max(&[5, 2, 8, 1, 9]), Some((1, 9)));
    let product = cartesian(&[1, 2], &[3]);
    assert_eq!(product, vec![(1, 3), (2, 3)]);
}

#[test]
fn uuid_roundtrip() {
    let id = generate_id();
    let s = id.to_string();
    let parsed = parse_id(&s).unwrap();
    assert_eq!(id, parsed);
    assert!(is_valid_uuid(&s));
}

#[test]
fn uuid_batch_unique() {
    let batch = generate_batch(50);
    let set: std::collections::HashSet<_> = batch.iter().collect();
    assert_eq!(set.len(), 50);
}

#[test]
fn bytes_message_protocol() {
    let msg = build_message(0xDEAD, b"data");
    assert_eq!(msg.len(), 8);

    let (id, payload) = read_message(msg).unwrap();
    assert_eq!(id, 0xDEAD);
    assert_eq!(&payload[..], b"data");
}
