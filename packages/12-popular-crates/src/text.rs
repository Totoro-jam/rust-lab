//! Regular expressions with the `regex` crate.
//!
//! Demonstrates pattern matching, captures, find, replace,
//! and compiled regex for performance.

use regex::Regex;

/// Checks if a string is a valid email (simplified pattern).
pub fn is_valid_email(input: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(input)
}

/// Extracts all numbers from a string.
pub fn extract_numbers(input: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

/// Parses a date string "YYYY-MM-DD" and returns (year, month, day).
pub fn parse_date(input: &str) -> Option<(i32, u32, u32)> {
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    let caps = re.captures(input)?;
    let year: i32 = caps[1].parse().ok()?;
    let month: u32 = caps[2].parse().ok()?;
    let day: u32 = caps[3].parse().ok()?;
    Some((year, month, day))
}

/// Replaces all whitespace sequences with a single space.
pub fn normalize_whitespace(input: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(input, " ").to_string()
}

/// Extracts key-value pairs from "key=value" formatted lines.
pub fn extract_key_values(input: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"(\w+)\s*=\s*(.+)").unwrap();
    re.captures_iter(input)
        .map(|cap| (cap[1].to_string(), cap[2].trim().to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("first.last@domain.org"));
        assert!(!is_valid_email("not-an-email"));
        assert!(!is_valid_email("@missing.com"));
        assert!(!is_valid_email("user@.com"));
    }

    #[test]
    fn extract_numbers_from_text() {
        let nums = extract_numbers("I have 3 cats and -2 dogs, plus 100 fish");
        assert_eq!(nums, [3, -2, 100]);
    }

    #[test]
    fn extract_numbers_empty() {
        let nums = extract_numbers("no numbers here");
        assert!(nums.is_empty());
    }

    #[test]
    fn parse_date_valid() {
        assert_eq!(parse_date("2024-01-15"), Some((2024, 1, 15)));
        assert_eq!(parse_date("1999-12-31"), Some((1999, 12, 31)));
    }

    #[test]
    fn parse_date_invalid() {
        assert_eq!(parse_date("not-a-date"), None);
        assert_eq!(parse_date("2024/01/15"), None);
    }

    #[test]
    fn normalize_spaces() {
        assert_eq!(
            normalize_whitespace("hello   world\t\tfoo\nbar"),
            "hello world foo bar"
        );
    }

    #[test]
    fn key_value_extraction() {
        let input = "name = Alice\nage = 30\ncity = Tokyo";
        let pairs = extract_key_values(input);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0], ("name".into(), "Alice".into()));
        assert_eq!(pairs[1], ("age".into(), "30".into()));
    }
}
