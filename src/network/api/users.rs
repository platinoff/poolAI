//! User management API endpoints
//!
//! Provides endpoints for managing users:
//! - List users
//! - Create, get, update, delete users

use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json as AxumJson, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, get_global_user_manager, Claims, UserRole};

#[derive(Deserialize)]
struct UserCreateRequest {
    username: String,
    password: String,
    role: UserRole,
}

#[derive(Deserialize)]
struct UserUpdateRequest {
    username: Option<String>,
    password: Option<String>,
    role: Option<UserRole>,
    active: Option<bool>,
}

/// Create user management routes
pub fn create_users_routes() -> Router {
    Router::new()
        .route("/users", get(users_list_handler))
        .route(
            "/users",
            post(user_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/users/{id}", get(user_get_handler))
        .route(
            "/users/{id}",
            put(user_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/users/{id}",
            delete(user_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
}

async fn users_list_handler() -> impl IntoResponse {
    let manager = get_global_user_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": format!("User manager not initialized. Context: User manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.list_users().await {
        Ok(users) => AxumJson(users).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to list users. Context: Cannot retrieve user list. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn user_create_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<UserCreateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = get_global_user_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": format!("User manager not initialized. Context: Cannot create user - user manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.create_user(req.username, req.password, req.role).await {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create user. Context: Cannot create new user with specified parameters. Suggestion: Verify username uniqueness and parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn user_get_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = get_global_user_manager();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_user(user_id).await {
        Ok(Some(user)) => AxumJson(user).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!("User not found. Context: Cannot find user with specified ID. Suggestion: Verify user ID and ensure user exists. User ID: '{}'", id)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to retrieve user. Context: Cannot retrieve user information. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn user_update_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<UserUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = get_global_user_manager();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager
        .update_user(user_id, req.username, req.password, req.role, req.active)
        .await
    {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": format!("Failed to update user. Context: Cannot update user with specified parameters. Suggestion: Verify user ID and update parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn user_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = get_global_user_manager();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.delete_user(user_id).await {
        Ok(()) => (
            StatusCode::OK,
            AxumJson(serde_json::json!({
                "message": "User deleted successfully"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": format!("Failed to delete user. Context: Cannot delete user. Suggestion: Verify user ID and ensure user exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}
