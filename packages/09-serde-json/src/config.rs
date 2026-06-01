//! Application config deserialization with serde attributes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application configuration demonstrating various serde attributes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Renamed from camelCase in JSON.
    pub app_name: String,

    #[serde(default = "default_port")]
    pub listen_port: u16,

    #[serde(default)]
    pub debug_mode: bool,

    /// Optional field: absent in JSON means None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,

    /// Flattened: database fields appear at the top level in JSON.
    #[serde(flatten)]
    pub database: DatabaseConfig,

    /// Extra fields not explicitly defined.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extras: HashMap<String, serde_json::Value>,
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,

    #[serde(default = "default_db_name")]
    pub db_name: String,
}

fn default_db_name() -> String {
    "app_db".to_string()
}

/// Parses JSON string into AppConfig.
pub fn parse_config(json: &str) -> Result<AppConfig, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serializes AppConfig to a JSON string.
pub fn to_json(config: &AppConfig) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_config() {
        let json = r#"{
            "appName": "my-service",
            "listenPort": 9090,
            "debugMode": true,
            "logFile": "/var/log/app.log",
            "dbHost": "localhost",
            "dbPort": 5432,
            "dbName": "mydb"
        }"#;

        let cfg = parse_config(json).unwrap();
        assert_eq!(cfg.app_name, "my-service");
        assert_eq!(cfg.listen_port, 9090);
        assert!(cfg.debug_mode);
        assert_eq!(cfg.log_file, Some("/var/log/app.log".into()));
        assert_eq!(cfg.database.db_host, "localhost");
        assert_eq!(cfg.database.db_port, 5432);
    }

    #[test]
    fn defaults_applied() {
        let json = r#"{
            "appName": "test",
            "dbHost": "127.0.0.1",
            "dbPort": 3306
        }"#;

        let cfg = parse_config(json).unwrap();
        assert_eq!(cfg.listen_port, 8080);
        assert!(!cfg.debug_mode);
        assert_eq!(cfg.log_file, None);
        assert_eq!(cfg.database.db_name, "app_db");
    }

    #[test]
    fn roundtrip_serialization() {
        let cfg = AppConfig {
            app_name: "roundtrip".into(),
            listen_port: 3000,
            debug_mode: false,
            log_file: None,
            database: DatabaseConfig {
                db_host: "db.local".into(),
                db_port: 5432,
                db_name: "test".into(),
            },
            extras: HashMap::new(),
        };

        let json = to_json(&cfg).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, parsed);
    }
}
