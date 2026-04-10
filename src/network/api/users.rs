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

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::core::user_manager::UserInfo;
use crate::network::api::common::{check_permission, HttpAppError};
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

async fn users_list_handler(
    State(ctx): State<ApiContext>,
) -> Result<AxumJson<Vec<UserInfo>>, HttpAppError> {
    let manager = ctx.user_manager.clone();

    if let Err(e) = manager.initialize().await {
        return Err(HttpAppError::new(AppError::RestError {
            code: "USER_MANAGER_UNAVAILABLE",
            message: format!("User manager not initialized: {}", e),
        })
        .with_context(
            ErrorContext::new("users_list")
                .with_hint("Check system startup sequence and user manager wiring."),
        )
        .with_status(StatusCode::SERVICE_UNAVAILABLE));
    }

    match manager.list_users().await {
        Ok(users) => Ok(AxumJson(users)),
        Err(e) => Err(HttpAppError::new(AppError::RestError {
            code: "LIST_USERS_FAILED",
            message: format!("Failed to list users: {}", e),
        })
        .with_context(ErrorContext::new("users_list"))),
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
        return HttpAppError::new(AppError::RestError {
            code: "USER_MANAGER_UNAVAILABLE",
            message: format!("User manager not initialized: {}", e),
        })
        .with_context(
            ErrorContext::new("user_create")
                .with_hint("Check system startup sequence and user manager wiring."),
        )
        .with_status(StatusCode::SERVICE_UNAVAILABLE)
        .into_response();
    }

    match manager
        .create_user(req.username, req.password, req.role)
        .await
    {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => HttpAppError::new(AppError::RestError {
            code: "CREATE_USER_FAILED",
            message: format!("Failed to create user: {}", e),
        })
        .with_context(
            ErrorContext::new("user_create")
                .with_hint("Verify username uniqueness and request parameters."),
        )
        .with_status(StatusCode::BAD_REQUEST)
        .into_response(),
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
            return HttpAppError::new(AppError::RestError {
                code: "INVALID_UUID",
                message: format!("Invalid UUID format for user id: {}", id),
            })
            .with_context(ErrorContext::new("user_get").with_resource("user_id", &id))
            .with_status(StatusCode::BAD_REQUEST)
            .into_response();
        }
    };

    match manager.get_user(user_id).await {
        Ok(Some(user)) => AxumJson(user).into_response(),
        Ok(None) => HttpAppError::new(AppError::RestError {
            code: "USER_NOT_FOUND",
            message: format!("User not found: {}", id),
        })
        .with_context(ErrorContext::new("user_get").with_resource("user_id", &id))
        .with_status(StatusCode::NOT_FOUND)
        .into_response(),
        Err(e) => HttpAppError::new(AppError::RestError {
            code: "GET_USER_FAILED",
            message: format!("Failed to retrieve user: {}", e),
        })
        .with_context(ErrorContext::new("user_get").with_resource("user_id", &id))
        .into_response(),
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
            return HttpAppError::new(AppError::RestError {
                code: "INVALID_UUID",
                message: format!("Invalid UUID format for user id: {}", id),
            })
            .with_context(ErrorContext::new("user_update").with_resource("user_id", &id))
            .with_status(StatusCode::BAD_REQUEST)
            .into_response();
        }
    };

    match manager
        .update_user(user_id, req.username, req.password, req.role, req.active)
        .await
    {
        Ok(user) => AxumJson(user).into_response(),
        Err(e) => HttpAppError::new(AppError::RestError {
            code: "UPDATE_USER_FAILED",
            message: format!("Failed to update user: {}", e),
        })
        .with_context(
            ErrorContext::new("user_update")
                .with_resource("user_id", &id)
                .with_hint("Verify user ID and update parameters."),
        )
        .with_status(StatusCode::BAD_REQUEST)
        .into_response(),
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
            return HttpAppError::new(AppError::RestError {
                code: "INVALID_UUID",
                message: format!("Invalid UUID format for user id: {}", id),
            })
            .with_context(ErrorContext::new("user_delete").with_resource("user_id", &id))
            .with_status(StatusCode::BAD_REQUEST)
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
        Err(e) => HttpAppError::new(AppError::RestError {
            code: "DELETE_USER_FAILED",
            message: format!("Failed to delete user: {}", e),
        })
        .with_context(
            ErrorContext::new("user_delete")
                .with_resource("user_id", &id)
                .with_hint("Verify user ID and ensure the user exists."),
        )
        .with_status(StatusCode::BAD_REQUEST)
        .into_response(),
    }
}
