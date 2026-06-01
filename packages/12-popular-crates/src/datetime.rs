//! Date and time handling with `chrono`.
//!
//! Demonstrates parsing, formatting, duration arithmetic,
//! and timezone conversions.

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};

/// Parses an ISO 8601 datetime string into a UTC DateTime.
pub fn parse_iso8601(input: &str) -> Option<DateTime<Utc>> {
    input.parse::<DateTime<Utc>>().ok()
}

/// Formats a UTC DateTime as a human-readable string.
pub fn format_human(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Calculates the number of days between two date strings (YYYY-MM-DD).
pub fn days_between(from: &str, to: &str) -> Option<i64> {
    let d1 = NaiveDate::parse_from_str(from, "%Y-%m-%d").ok()?;
    let d2 = NaiveDate::parse_from_str(to, "%Y-%m-%d").ok()?;
    Some((d2 - d1).num_days())
}

/// Adds a number of days to a date string, returns the new date.
pub fn add_days(date: &str, days: i64) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let new_date = d + Duration::days(days);
    Some(new_date.format("%Y-%m-%d").to_string())
}

/// Parses a datetime string and returns the day of the week.
pub fn day_of_week(date: &str) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    Some(d.format("%A").to_string())
}

/// Checks if a year is a leap year using chrono.
pub fn is_leap_year(year: i32) -> bool {
    NaiveDate::from_ymd_opt(year, 2, 29).is_some()
}

/// Parses a custom datetime format.
pub fn parse_custom(input: &str, format: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(input, format).ok()
}

/// Creates a UTC DateTime from components.
pub fn from_components(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(year, month, day, hour, min, 0)
        .single()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_format_iso8601() {
        let dt = parse_iso8601("2024-06-15T10:30:00Z").unwrap();
        assert_eq!(format_human(dt), "2024-06-15 10:30:00 UTC");
    }

    #[test]
    fn parse_invalid_iso() {
        assert!(parse_iso8601("not a date").is_none());
    }

    #[test]
    fn days_between_dates() {
        assert_eq!(days_between("2024-01-01", "2024-01-31"), Some(30));
        assert_eq!(days_between("2024-03-01", "2024-02-01"), Some(-29));
    }

    #[test]
    fn add_days_to_date() {
        assert_eq!(add_days("2024-01-30", 2), Some("2024-02-01".into()));
        assert_eq!(add_days("2024-12-31", 1), Some("2025-01-01".into()));
    }

    #[test]
    fn day_of_week_check() {
        assert_eq!(day_of_week("2024-01-01"), Some("Monday".into()));
    }

    #[test]
    fn leap_year_check() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn custom_format_parse() {
        let dt = parse_custom("15/06/2024 10:30", "%d/%m/%Y %H:%M").unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-06-15");
    }

    #[test]
    fn from_components_valid() {
        let dt = from_components(2024, 6, 15, 10, 30).unwrap();
        assert_eq!(format_human(dt), "2024-06-15 10:30:00 UTC");
    }

    #[test]
    fn from_components_invalid() {
        assert!(from_components(2024, 13, 1, 0, 0).is_none());
    }
}
