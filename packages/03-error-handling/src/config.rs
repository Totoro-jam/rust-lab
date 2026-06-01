//! Config parser with typed errors using `thiserror`.
//!
//! Parses a simple `key=value` config format. Demonstrates multiple
//! error variants, `From` impls via `#[from]`, and the `?` operator.

use std::collections::HashMap;
use std::num::ParseIntError;

/// Errors that can occur while parsing a config file.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error on line {line}: {message}")]
    Parse { line: usize, message: String },

    #[error("missing required key: {0}")]
    MissingKey(String),

    #[error("invalid integer for key '{key}': {source}")]
    InvalidInt {
        key: String,
        #[source]
        source: ParseIntError,
    },
}

/// Parsed configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub extras: HashMap<String, String>,
}

/// Parses config text in `key=value` format (one pair per line).
/// Lines starting with `#` are comments; blank lines are skipped.
pub fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let map = parse_key_values(input)?;

    let host = map
        .get("host")
        .ok_or_else(|| ConfigError::MissingKey("host".into()))?
        .clone();

    let port_str = map
        .get("port")
        .ok_or_else(|| ConfigError::MissingKey("port".into()))?;

    let port: u16 = port_str.parse().map_err(|e| ConfigError::InvalidInt {
        key: "port".into(),
        source: e,
    })?;

    let debug = map
        .get("debug")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let extras: HashMap<String, String> = map
        .into_iter()
        .filter(|(k, _)| k != "host" && k != "port" && k != "debug")
        .collect();

    Ok(Config {
        host,
        port,
        debug,
        extras,
    })
}

fn parse_key_values(input: &str) -> Result<HashMap<String, String>, ConfigError> {
    let mut map = HashMap::new();

    for (idx, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let (key, value) = trimmed.split_once('=').ok_or_else(|| ConfigError::Parse {
            line: idx + 1,
            message: format!("expected 'key=value', got: {trimmed:?}"),
        })?;

        map.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let input = "host=localhost\nport=8080\ndebug=true\n";
        let cfg = parse_config(input).unwrap();
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
        assert!(cfg.debug);
    }

    #[test]
    fn missing_key_error() {
        let input = "host=localhost\n";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::MissingKey(ref k) if k == "port"));
    }

    #[test]
    fn invalid_port_error() {
        let input = "host=localhost\nport=abc\n";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidInt { ref key, .. } if key == "port"));
    }

    #[test]
    fn parse_error_on_malformed_line() {
        let input = "host=localhost\nbadline\nport=80\n";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err, ConfigError::Parse { line: 2, .. }));
    }

    #[test]
    fn comments_and_blanks_skipped() {
        let input = "# comment\n\nhost = 0.0.0.0\nport = 3000\n";
        let cfg = parse_config(input).unwrap();
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 3000);
    }
}
