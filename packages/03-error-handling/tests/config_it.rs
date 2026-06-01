//! Integration tests for config parsing and error types.

use rustlab03::config::{parse_config, ConfigError};

#[test]
fn full_config_with_extras() {
    let input = "\
# App config
host = example.com
port = 443
debug = true
timeout = 30
";
    let cfg = parse_config(input).unwrap();
    assert_eq!(cfg.host, "example.com");
    assert_eq!(cfg.port, 443);
    assert!(cfg.debug);
    assert_eq!(cfg.extras.get("timeout").unwrap(), "30");
}

#[test]
fn error_display_messages() {
    let input = "host=localhost\nport=not_a_number\n";
    let err = parse_config(input).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("invalid integer"));
    assert!(msg.contains("port"));
}

#[test]
fn error_is_correct_variant() {
    let input = "port=80\n";
    let err = parse_config(input).unwrap_err();
    assert!(matches!(err, ConfigError::MissingKey(ref k) if k == "host"));
}
