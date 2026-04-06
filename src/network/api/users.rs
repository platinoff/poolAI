//! User management API endpoints
//!
//! Provides endpoints for managing users:
//! - List users
//! - Create, get, update, delete users

use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json as AxumJson, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::{api_json_error, check_permission};
use crate::network::auth::{auth_middleware, Claims, UserRole};

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
pub fn create_users_routes() -> Router<ApiContext> {
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

async fn users_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let manager = ctx.user_manager.clone();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        let (s, j) = api_json_error(
            "USER_MANAGER_UNAVAILABLE",
            format!("User manager not initialized: {}", e),
            Some(
                ErrorContext::new("users_list")
                    .with_hint("Check system startup sequence and user manager wiring."),
            ),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    match manager.list_users().await {
        Ok(users) => AxumJson(users).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "LIST_USERS_FAILED",
                format!("Failed to list users: {}", e),
                Some(ErrorContext::new("users_list")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn user_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UserCreateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = ctx.user_manager.clone();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        let (s, j) = api_json_error(
            "USER_MANAGER_UNAVAILABLE",
            format!("User manager not initialized: {}", e),
            Some(
                ErrorContext::new("user_create")
                    .with_hint("Check system startup sequence and user manager wiring."),
            ),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    match manager
        .create_user(req.username, req.password, req.role)
        .await
    {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "CREATE_USER_FAILED",
                format!("Failed to create user: {}", e),
                Some(
                    ErrorContext::new("user_create")
                        .with_hint("Verify username uniqueness and request parameters."),
                ),
                StatusCode::BAD_REQUEST,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn user_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let manager = ctx.user_manager.clone();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for user id: {}", id),
                Some(ErrorContext::new("user_get").with_resource("user_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match manager.get_user(user_id).await {
        Ok(Some(user)) => AxumJson(user).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "USER_NOT_FOUND",
                format!("User not found: {}", id),
                Some(ErrorContext::new("user_get").with_resource("user_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => {
            let (s, j) = api_json_error(
                "GET_USER_FAILED",
                format!("Failed to retrieve user: {}", e),
                Some(ErrorContext::new("user_get").with_resource("user_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn user_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<UserUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = ctx.user_manager.clone();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for user id: {}", id),
                Some(ErrorContext::new("user_update").with_resource("user_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match manager
        .update_user(user_id, req.username, req.password, req.role, req.active)
        .await
    {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "UPDATE_USER_FAILED",
                format!("Failed to update user: {}", e),
                Some(
                    ErrorContext::new("user_update")
                        .with_resource("user_id", &id)
                        .with_hint("Verify user ID and update parameters."),
                ),
                StatusCode::BAD_REQUEST,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn user_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = ctx.user_manager.clone();
    let user_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for user id: {}", id),
                Some(ErrorContext::new("user_delete").with_resource("user_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
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
        Err(e) => {
            let (s, j) = api_json_error(
                "DELETE_USER_FAILED",
                format!("Failed to delete user: {}", e),
                Some(
                    ErrorContext::new("user_delete")
                        .with_resource("user_id", &id)
                        .with_hint("Verify user ID and ensure the user exists."),
                ),
                StatusCode::BAD_REQUEST,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}
