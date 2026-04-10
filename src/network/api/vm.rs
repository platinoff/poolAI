//! VM instance management API endpoints
//!
//! Provides endpoints for managing VM instances:
//! - List VM instances
//! - Create, update, delete VM instances
//! - Start, stop, restart VM instances
//! - Get VM instance health and resource usage
//! - Check resource limits support

use axum::{
    extract::{Extension, Json, Path, State},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json as AxumJson, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::network::api::common::{check_permission, HttpAppError};
use crate::network::auth::Claims;
use crate::services::vm_service::{
    VmMutationError, VmResourceUsageError, VmService, VmServiceError,
};
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
pub fn create_vm_routes() -> Router<ApiContext> {
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
        // VM Templates endpoints
        .route("/vm/templates", get(vm_templates_handler))
        .route(
            "/vm/templates",
            post(vm_template_create_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/vm/templates/{id}", get(vm_template_get_handler))
        .route(
            "/vm/templates/{id}",
            put(vm_template_update_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/templates/{id}",
            delete(vm_template_delete_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        // VM Networks endpoints
        .route("/vm/networks", get(vm_networks_handler))
        .route(
            "/vm/networks",
            post(vm_network_create_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/vm/networks/{id}", get(vm_network_get_handler))
        .route(
            "/vm/networks/{id}",
            put(vm_network_update_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route(
            "/vm/networks/{id}",
            delete(vm_network_delete_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
}

fn vm_manager_unavailable() -> HttpAppError {
    HttpAppError::new(AppError::SubsystemUnavailable(
        crate::services::vm_service::VM_MANAGER_UNAVAILABLE_MESSAGE.to_string(),
    ))
    .with_context(
        ErrorContext::new("vm")
            .with_resource("vm_manager", "default")
            .with_hint("Initialize the VM manager during startup."),
    )
}

fn vm_bad_request_uuid(id: &str) -> HttpAppError {
    HttpAppError::new(AppError::ValidationError(format!("Invalid UUID: '{}'", id))).with_context(
        ErrorContext::new("parse_uuid")
            .with_resource("instance_id", id)
            .with_hint("Use a standard UUID string."),
    )
}

fn vm_service_http_err(e: VmServiceError) -> HttpAppError {
    match e {
        VmServiceError::ManagerUnavailable => vm_manager_unavailable(),
    }
}

async fn vm_instances_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match VmService::list_instances(&ctx).await {
        Ok(instances) => AxumJson(instances).into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_instance_resources_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::get_instance_resource_usage(&ctx, uuid).await {
        Ok(usage) => AxumJson(usage).into_response(),
        Err(VmResourceUsageError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmResourceUsageError::Query(e)) => {
            HttpAppError::new(AppError::ApiNotFound(e.to_string()))
                .with_context(
                    ErrorContext::new("vm_resource_usage").with_resource("instance_id", &id),
                )
                .into_response()
        }
    }
}

async fn vm_resource_limits_supported_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match VmService::is_resource_limits_supported(&ctx) {
        Ok(supported) => AxumJson(serde_json::json!({
            "supported": supported
        }))
        .into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_instance_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<VmCreateRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let isolation = payload.isolation.unwrap_or(vm::VmIsolation::ProcessSandbox);
    let instance_name = payload.name.clone();

    match VmService::create_instance(&ctx, payload.name, payload.resources, isolation).await {
        Ok(instance) => AxumJson(instance).into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to create VM instance: {} (name='{}')",
            e, instance_name
        )))
        .with_context(
            ErrorContext::new("create_instance").with_resource("instance", &instance_name),
        )
        .into_response(),
    }
}

async fn vm_instance_update_handler(
    State(ctx): State<ApiContext>,
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

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::update_instance(
        &ctx,
        uuid,
        payload.name,
        payload.resources,
        payload.isolation,
    )
    .await
    {
        Ok(instance) => AxumJson(instance).into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to update VM instance: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("update_instance").with_resource("instance_id", &id))
        .into_response(),
    }
}

async fn vm_instance_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: delete:all or write:vm
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::delete_instance(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} deleted successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to delete VM instance: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("delete_instance").with_resource("instance_id", &id))
        .into_response(),
    }
}

async fn vm_instance_start_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::start_instance(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} started successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to start VM instance: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("start_instance").with_resource("instance_id", &id))
        .into_response(),
    }
}

async fn vm_instance_stop_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::stop_instance(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} stopped successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to stop VM instance: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("stop_instance").with_resource("instance_id", &id))
        .into_response(),
    }
}

async fn vm_instance_restart_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::restart_instance(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("VM instance {} restarted successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to restart VM instance: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("restart_instance").with_resource("instance_id", &id))
        .into_response(),
    }
}

async fn vm_instance_health_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::get_instance_health(&ctx, uuid).await {
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
        Ok(None) => HttpAppError::new(AppError::ApiNotFound(
            "Health check not registered for this instance".to_string(),
        ))
        .with_context(ErrorContext::new("vm_health").with_resource("instance_id", &id))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to retrieve VM instance health: {} (id='{}')",
            e, id
        )))
        .with_context(ErrorContext::new("vm_health").with_resource("instance_id", &id))
        .into_response(),
    }
}

// ============================================================================
// VM Templates handlers
// ============================================================================

async fn vm_templates_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match VmService::list_templates(&ctx).await {
        Ok(templates) => AxumJson(templates).into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_template_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::get_template(&ctx, uuid).await {
        Ok(Some(template)) => AxumJson(template).into_response(),
        Ok(None) => HttpAppError::new(AppError::ApiNotFound(format!("Template not found: {}", id)))
            .with_context(ErrorContext::new("get_template").with_resource("template_id", &id))
            .into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_template_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(template): Json<vm::VmTemplate>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    match VmService::create_template(&ctx, template.clone()).await {
        Ok(()) => AxumJson(template).into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to create template: {}",
            e
        )))
        .with_context(ErrorContext::new("create_template"))
        .into_response(),
    }
}

async fn vm_template_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(template): Json<vm::VmTemplate>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    match VmService::update_template(&ctx, template).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("Template {} updated successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => {
            HttpAppError::new(AppError::ApiNotFound(e.to_string()))
                .with_context(
                    ErrorContext::new("update_template").with_resource("template_id", &id),
                )
                .into_response()
        }
    }
}

async fn vm_template_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::delete_template(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("Template {} deleted successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => {
            HttpAppError::new(AppError::ApiNotFound(e.to_string()))
                .with_context(
                    ErrorContext::new("delete_template").with_resource("template_id", &id),
                )
                .into_response()
        }
    }
}

// ============================================================================
// VM Networks handlers
// ============================================================================

async fn vm_networks_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match VmService::list_networks(&ctx).await {
        Ok(networks) => AxumJson(networks).into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_network_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::get_network(&ctx, uuid).await {
        Ok(Some(network)) => AxumJson(network).into_response(),
        Ok(None) => HttpAppError::new(AppError::ApiNotFound(format!("Network not found: {}", id)))
            .with_context(ErrorContext::new("get_network").with_resource("network_id", &id))
            .into_response(),
        Err(e) => vm_service_http_err(e).into_response(),
    }
}

async fn vm_network_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(network): Json<vm::VmNetwork>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    match VmService::create_network(&ctx, network.clone()).await {
        Ok(()) => AxumJson(network).into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to create network: {}",
            e
        )))
        .with_context(ErrorContext::new("create_network"))
        .into_response(),
    }
}

async fn vm_network_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(network): Json<vm::VmNetwork>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    match VmService::update_network(&ctx, network).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("Network {} updated successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => {
            HttpAppError::new(AppError::ApiNotFound(e.to_string()))
                .with_context(ErrorContext::new("update_network").with_resource("network_id", &id))
                .into_response()
        }
    }
}

async fn vm_network_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return vm_bad_request_uuid(&id).into_response();
        }
    };

    match VmService::delete_network(&ctx, uuid).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("Network {} deleted successfully", id)
        }))
        .into_response(),
        Err(VmMutationError::ManagerUnavailable) => vm_manager_unavailable().into_response(),
        Err(VmMutationError::Operation(e)) => {
            HttpAppError::new(AppError::ApiNotFound(e.to_string()))
                .with_context(ErrorContext::new("delete_network").with_resource("network_id", &id))
                .into_response()
        }
    }
}
