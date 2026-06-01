//! Statistics functions (behind "stats" feature flag).

use crate::arithmetic;

pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    #[allow(clippy::manual_is_multiple_of)]
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

pub fn variance(values: &[f64]) -> Option<f64> {
    let avg = mean(values)?;
    let sum_sq: f64 = values.iter().map(|x| (x - avg) * (x - avg)).sum();
    Some(sum_sq / values.len() as f64)
}

pub fn std_dev(values: &[f64]) -> Option<f64> {
    variance(values).map(|v| v.sqrt())
}

/// Uses pub(crate) `abs` from arithmetic module.
pub fn abs_deviation(values: &[f64]) -> Option<f64> {
    let avg = mean(values)?;
    let sum: f64 = values.iter().map(|x| arithmetic::abs(x - avg)).sum();
    Some(sum / values.len() as f64)
}
