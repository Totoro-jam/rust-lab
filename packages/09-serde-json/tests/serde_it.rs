//! Integration tests for serde JSON handling.

use rustlab09::api::{ApiResponse, Event, User, UserType};
use rustlab09::config::{parse_config, AppConfig};

#[test]
fn config_missing_optional_fields() {
    let json = r#"{
        "appName": "minimal",
        "dbHost": "db",
        "dbPort": 5432
    }"#;
    let cfg = parse_config(json).unwrap();
    assert_eq!(cfg.app_name, "minimal");
    assert_eq!(cfg.log_file, None);
    assert_eq!(cfg.listen_port, 8080);
}

#[test]
fn config_extra_fields_ignored() {
    let json = r#"{
        "appName": "test",
        "dbHost": "localhost",
        "dbPort": 3306,
        "unknownField": "should not crash"
    }"#;
    let cfg = parse_config(json).unwrap();
    assert_eq!(cfg.app_name, "test");
}

#[test]
fn api_response_roundtrip() {
    let user = User {
        id: 1,
        user_name: "test".into(),
        email: "test@test.com".into(),
        is_active: true,
        user_type: UserType::Member,
    };

    let resp = ApiResponse::ok(user.clone());
    let json = serde_json::to_string(&resp).unwrap();
    let parsed: ApiResponse<User> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.data.unwrap(), user);
}

#[test]
fn event_all_variants_parse() {
    let events_json = [
        r#"{"type": "user_created", "user_id": 1, "username": "a"}"#,
        r#"{"type": "user_deleted", "user_id": 2}"#,
        r#"{"type": "message_sent", "from": 1, "to": 2, "content": "hi"}"#,
    ];

    let events: Vec<Event> = events_json
        .iter()
        .map(|j| serde_json::from_str(j).unwrap())
        .collect();

    assert_eq!(events.len(), 3);
    assert!(matches!(events[0], Event::UserCreated { .. }));
    assert!(matches!(events[1], Event::UserDeleted { .. }));
    assert!(matches!(events[2], Event::MessageSent { .. }));
}

#[test]
fn null_vs_absent() {
    // log_file explicitly null
    let json = r#"{
        "appName": "test",
        "logFile": null,
        "dbHost": "h",
        "dbPort": 1
    }"#;
    let cfg: AppConfig = serde_json::from_str(json).unwrap();
    assert_eq!(cfg.log_file, None);
}
