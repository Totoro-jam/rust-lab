//! UUID generation with the `uuid` crate.
//!
//! Demonstrates v4 (random) UUID generation, parsing, and formatting.

use uuid::Uuid;

/// Generates a random (v4) UUID.
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

/// Parses a UUID from a string.
pub fn parse_id(input: &str) -> Option<Uuid> {
    Uuid::parse_str(input).ok()
}

/// Returns the hyphenated string form of a UUID.
pub fn format_id(id: &Uuid) -> String {
    id.to_string()
}

/// Checks if a string is a valid UUID.
pub fn is_valid_uuid(input: &str) -> bool {
    Uuid::parse_str(input).is_ok()
}

/// Generates N unique UUIDs.
pub fn generate_batch(n: usize) -> Vec<Uuid> {
    (0..n).map(|_| Uuid::new_v4()).collect()
}

/// Returns the UUID version number.
pub fn uuid_version(id: &Uuid) -> usize {
    id.get_version_num()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_is_v4() {
        let id = generate_id();
        assert_eq!(uuid_version(&id), 4);
    }

    #[test]
    fn generate_unique() {
        let a = generate_id();
        let b = generate_id();
        assert_ne!(a, b);
    }

    #[test]
    fn parse_valid_uuid() {
        let input = "550e8400-e29b-41d4-a716-446655440000";
        let id = parse_id(input).unwrap();
        assert_eq!(format_id(&id), input);
    }

    #[test]
    fn parse_invalid_uuid() {
        assert!(parse_id("not-a-uuid").is_none());
        assert!(parse_id("").is_none());
    }

    #[test]
    fn roundtrip_format_parse() {
        let original = generate_id();
        let formatted = format_id(&original);
        let parsed = parse_id(&formatted).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn is_valid_uuid_check() {
        assert!(is_valid_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!is_valid_uuid("not-valid"));
    }

    #[test]
    fn batch_unique() {
        let batch = generate_batch(100);
        let unique: std::collections::HashSet<_> = batch.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn uuid_string_length() {
        let id = generate_id();
        assert_eq!(format_id(&id).len(), 36);
    }
}
