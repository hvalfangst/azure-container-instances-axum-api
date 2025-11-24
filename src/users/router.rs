use std::{
    sync::{Arc, Mutex},
    collections::HashMap
};
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Router,
    Json
};
use serde_json::{json, Value};
use crate::users::{
    model::{UpsertUser, User, validate_email},
    service::{create_user, get_user_by_email, delete_user_by_email, update_user_by_email},
};

// - - - - - - - - - - - [ROUTES] - - - - - - - - - - -

pub fn users_routes(shared_hashmap: Arc<Mutex<HashMap<String, User>>>) -> Router {
    Router::new()
        .route("/users", axum::routing::post(create_user_handler))
        .route("/users/:email", axum::routing::get(get_user_handler))
        .route("/users/:email", axum::routing::put(update_user_handler))
        .route("/users/:email", axum::routing::delete(delete_user_handler))
        .with_state(shared_hashmap)
}

// - - - - - - - - - - - [HANDLERS] - - - - - - - - - - -

pub async fn create_user_handler(
    State(shared_hashmap): State<Arc<Mutex<HashMap<String, User>>>>,
    Json(request): Json<UpsertUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    if !validate_email(&request) {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error": "Invalid input for field 'email'"}))));
    }

    match create_user(request, &shared_hashmap).await {
        None => Err((StatusCode::ALREADY_REPORTED, Json(json!({"error": "User with associated email already exists!"})))),
        Some(created_user) => Ok((StatusCode::CREATED, Json(created_user)))
    }
}

pub async fn get_user_handler(
    State(shared_hashmap): State<Arc<Mutex<HashMap<String, User>>>>,
    path: Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let email = path.0;

    match get_user_by_email(&email, &shared_hashmap).await {
        Some(user) => Ok((StatusCode::OK, Json(user))),
        _ => Err((StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))
    }
}

pub async fn update_user_handler(
    State(shared_hashmap): State<Arc<Mutex<HashMap<String, User>>>>,
    path: Path<String>,
    Json(request): Json<UpsertUser>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let email = path.0;

    match update_user_by_email(&email, request, &shared_hashmap).await {
        Some(retrieved_user) => Ok((StatusCode::OK, Json(retrieved_user))),
        _ => Err((StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))
    }
}

pub async fn delete_user_handler(
    State(shared_hashmap): State<Arc<Mutex<HashMap<String, User>>>>,
    path: Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let email = path.0;

    match delete_user_by_email(&email, &shared_hashmap).await {
        Some(_user) => Ok((StatusCode::OK, Json(json!({"message": "User has been deleted"})))),
        _ => Err((StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))
    }
}
