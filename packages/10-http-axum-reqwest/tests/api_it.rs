//! Integration tests using axum's built-in test utilities.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use rustlab10::{app, Todo};
use tower::ServiceExt;

#[tokio::test]
async fn create_and_get_todo() {
    let app = app();

    // Create a todo
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/todos")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title": "Learn Rust"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let todo: Todo = serde_json::from_slice(&body).unwrap();
    assert_eq!(todo.title, "Learn Rust");
    assert!(!todo.completed);
    assert_eq!(todo.id, 1);
}

#[tokio::test]
async fn get_nonexistent_returns_404() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos/999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn crud_full_cycle() {
    let app = app();

    // Create
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/todos")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title": "item1"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // List
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/todos")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let todos: Vec<Todo> = serde_json::from_slice(&body).unwrap();
    assert_eq!(todos.len(), 1);

    // Update
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/todos/1")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title": "updated"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let todo: Todo = serde_json::from_slice(&body).unwrap();
    assert_eq!(todo.title, "updated");

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/todos/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Confirm deleted
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/todos/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
