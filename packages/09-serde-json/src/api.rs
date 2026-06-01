//! API response types with tagged enums.

use serde::{Deserialize, Serialize};

/// A generic API response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// User data from an API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub user_name: String,
    pub email: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "type")]
    pub user_type: UserType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Admin,
    Member,
    Guest,
}

/// An event type demonstrating internally tagged enums.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    UserCreated { user_id: u64, username: String },
    UserDeleted { user_id: u64 },
    MessageSent { from: u64, to: u64, content: String },
}

/// Parse a JSON API response containing a User.
pub fn parse_user_response(json: &str) -> Result<ApiResponse<User>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Parse an event from JSON.
pub fn parse_event(json: &str) -> Result<Event, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_user_ok_response() {
        let json = r#"{
            "success": true,
            "data": {
                "id": 42,
                "userName": "alice",
                "email": "alice@example.com",
                "isActive": true,
                "type": "admin"
            }
        }"#;

        let resp = parse_user_response(json).unwrap();
        assert!(resp.success);
        let user = resp.data.unwrap();
        assert_eq!(user.id, 42);
        assert_eq!(user.user_name, "alice");
        assert_eq!(user.user_type, UserType::Admin);
    }

    #[test]
    fn parse_error_response() {
        let json = r#"{
            "success": false,
            "error": "not found"
        }"#;

        let resp: ApiResponse<User> = serde_json::from_str(json).unwrap();
        assert!(!resp.success);
        assert_eq!(resp.error, Some("not found".into()));
        assert!(resp.data.is_none());
    }

    #[test]
    fn tagged_enum_variants() {
        let json = r#"{"type": "user_created", "user_id": 1, "username": "bob"}"#;
        let event = parse_event(json).unwrap();
        assert!(matches!(event, Event::UserCreated { user_id: 1, .. }));

        let json = r#"{"type": "message_sent", "from": 1, "to": 2, "content": "hi"}"#;
        let event = parse_event(json).unwrap();
        assert!(matches!(event, Event::MessageSent { from: 1, to: 2, .. }));
    }

    #[test]
    fn serialize_event() {
        let event = Event::UserDeleted { user_id: 99 };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"user_deleted\""));
        assert!(json.contains("\"user_id\":99"));
    }
}
