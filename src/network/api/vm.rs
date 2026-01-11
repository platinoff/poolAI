//! VM instance management API endpoints
//!
//! Provides endpoints for managing VM instances:
//! - List VM instances
//! - Create, update, delete VM instances
//! - Start, stop, restart VM instances
//! - Get VM instance health and resource usage
//! - Check resource limits support

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
use crate::network::auth::Claims;
use crate::vm;

#[derive(Deserialize)]
struct VmCreateRequest {
    name: String,
    resources: vm::VmResources,
    isolation: Option<vm::VmIsolation>,
}

#[derive(Deserialize)]
struct VmUpdateRequest {
    name: Option<String>,
    resources: Option<vm::VmResources>,
    isolation: Option<vm::VmIsolation>,
}

/// Create VM management routes
pub fn create_vm_routes() -> Router {
    Router::new()
        .route("/vm/instances", get(vm_instances_handler))
        .route(
            "/vm/instances",
            post(vm_instance_create_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/instances/{id}",
            put(vm_instance_update_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/instances/{id}",
            delete(vm_instance_delete_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/instances/{id}/start",
            post(vm_instance_start_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/instances/{id}/stop",
            post(vm_instance_stop_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/instances/{id}/restart",
            post(vm_instance_restart_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/vm/instances/{id}/health", get(vm_instance_health_handler))
        .route(
            "/vm/instances/{id}/resources",
            get(vm_instance_resources_handler),
        )
        .route(
            "/vm/resource-limits-supported",
            get(vm_resource_limits_supported_handler),
        )
}

async fn vm_instances_handler() -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let instances = manager.list_instances().await;
    AxumJson(instances).into_response()
}

async fn vm_instance_resources_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_instance_resource_usage(uuid).await {
        Ok(usage) => AxumJson(usage).into_response(),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": e.to_string()
            });
            (StatusCode::NOT_FOUND, AxumJson(error_response)).into_response()
        }
    }
}

async fn vm_resource_limits_supported_handler() -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let supported = manager.is_resource_limits_supported();
    AxumJson(serde_json::json!({
        "supported": supported
    }))
    .into_response()
}

async fn vm_instance_create_handler(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<VmCreateRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let isolation = payload.isolation.unwrap_or(vm::VmIsolation::ProcessSandbox);
    let instance_name = payload.name.clone();

    match manager
        .create_instance(payload.name, payload.resources, isolation)
        .await
    {
        Ok(instance) => AxumJson(instance).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create VM instance. Context: Cannot create new VM instance with specified configuration. Suggestion: Verify resource limits, isolation settings, and system capacity. Instance name: '{}', Error: {}", instance_name, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_update_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<VmUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager
        .update_instance(uuid, payload.name, payload.resources, payload.isolation, None)
        .await
    {
        Ok(instance) => AxumJson(instance).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to update VM instance. Context: Cannot update VM instance with new configuration. Suggestion: Verify instance exists, is in a valid state for updates, and check resource availability. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: delete:all or write:vm
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.delete_instance(uuid).await {
        Ok(_) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} deleted successfully", id)
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to delete VM instance. Context: Cannot delete VM instance. Suggestion: Ensure instance is stopped, check for active resources, and verify permissions. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_start_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.start_instance(uuid).await {
        Ok(_) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} started successfully", id)
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to start VM instance. Context: Cannot start VM instance. Suggestion: Verify instance exists, check resource availability, and ensure isolation settings are valid. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_stop_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.stop_instance(uuid).await {
        Ok(_) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} stopped successfully", id)
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to stop VM instance. Context: Cannot stop VM instance. Suggestion: Verify instance is running, check for blocking operations, and ensure proper shutdown sequence. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_restart_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.restart_instance(uuid).await {
        Ok(_) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} restarted successfully", id)
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to restart VM instance. Context: Cannot restart VM instance. Suggestion: Verify instance exists, ensure proper shutdown before restart, and check resource availability. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_health_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_instance_health(uuid).await {
        Ok(Some(status)) => match status {
            crate::runtime::health::HealthStatus::Healthy => {
                AxumJson(serde_json::json!({ "status": "healthy" })).into_response()
            }
            crate::runtime::health::HealthStatus::Unhealthy(reason) => {
                AxumJson(serde_json::json!({
                    "status": "unhealthy",
                    "reason": reason
                }))
                .into_response()
            }
            crate::runtime::health::HealthStatus::Unknown => {
                AxumJson(serde_json::json!({ "status": "unknown" })).into_response()
            }
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": "Health check not registered for this instance"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to retrieve VM instance health. Context: Cannot get health status for VM instance. Suggestion: Verify instance ID exists, ensure health monitor is registered for this instance, and check health monitor status. Instance ID: '{}', Error: {}", id, e)
            })),
        )
            .into_response(),
    }
}
