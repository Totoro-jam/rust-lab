//! rustlab10 — HTTP: axum (server) + reqwest (client)
//!
//! A simple CRUD REST API with in-memory storage.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A todo item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// Request body for creating/updating a todo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

/// Shared application state.
#[derive(Debug, Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<HashMap<u64, Todo>>>,
    pub next_id: Arc<Mutex<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds the axum Router with all routes.
pub fn app() -> Router {
    let state = AppState::new();
    app_with_state(state)
}

/// Builds the Router with a given state (useful for testing).
pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(state)
}

async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();
    let list: Vec<Todo> = todos.values().cloned().collect();
    Json(list)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let todo = Todo {
        id,
        title: input.title,
        completed: false,
    };

    state.todos.lock().unwrap().insert(id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}

async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.lock().unwrap();
    todos
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.lock().unwrap();
    match todos.get_mut(&id) {
        Some(todo) => {
            todo.title = input.title;
            Ok(Json(todo.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut todos = state.todos.lock().unwrap();
    if todos.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
