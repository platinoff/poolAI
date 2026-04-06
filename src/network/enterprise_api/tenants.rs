//! Enterprise API: tenants.

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::enterprise;
use crate::network::api::check_permission;
use crate::network::api::common::api_json_error;
use crate::network::auth::Claims;
use crate::services::enterprise_service::{EnterpriseService, TenantCreateError};
use axum::extract::{Extension, Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

pub(super) async fn tenants_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_tenants(&ctx).await {
        Ok(tenants) => Json(tenants).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!(
                    "Failed to list tenants. Context: Cannot retrieve tenant list. Suggestion: Check system logs and tenant manager initialization status. Error: {}",
                    e
                ),
                Some(ErrorContext::new("list_tenants")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct TenantCreateRequest {
    name: String,
    config: enterprise::multi_tenancy::TenantConfig,
}

pub(super) async fn tenant_create_handler(
    State(ctx): State<ApiContext>,
    Json(req): Json<TenantCreateRequest>,
) -> impl IntoResponse {
    match EnterpriseService::create_tenant(&ctx, req.name, req.config).await {
        Ok(tenant) => Json(tenant).into_response(),
        Err(TenantCreateError::Init(e)) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                format!(
                    "Tenant manager not initialized. Context: Cannot create tenant - tenant manager initialization failed. Suggestion: Check system startup sequence and tenant manager initialization status. Error: {}",
                    e
                ),
                Some(ErrorContext::new("create_tenant").with_resource("tenant_manager", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(TenantCreateError::Create(e)) => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                format!(
                    "Failed to create tenant. Context: Cannot create new tenant with specified configuration. Suggestion: Verify tenant name and configuration parameters. Error: {}",
                    e
                ),
                Some(ErrorContext::new("create_tenant")),
                StatusCode::BAD_REQUEST,
            );
            (s, j).into_response()
        }
    }
}

pub(super) async fn tenant_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                format!(
                    "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'",
                    id
                ),
                Some(ErrorContext::new("get_tenant").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::get_tenant(&ctx, tenant_id).await {
        Ok(Some(tenant)) => Json(tenant).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                format!(
                    "Tenant not found. Context: Cannot find tenant with specified ID. Suggestion: Verify tenant ID and ensure tenant exists. Tenant ID: '{}'",
                    id
                ),
                Some(ErrorContext::new("get_tenant").with_resource("tenant_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
        Err(e) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!(
                    "Failed to retrieve tenant. Context: Cannot retrieve tenant information. Suggestion: Check system logs and tenant manager status. Error: {}",
                    e
                ),
                Some(ErrorContext::new("get_tenant").with_resource("tenant_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct TenantUpdateRequest {
    config: Option<enterprise::multi_tenancy::TenantConfig>,
    active: Option<bool>,
}

pub(super) async fn tenant_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<TenantUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                format!(
                    "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'",
                    id
                ),
                Some(ErrorContext::new("update_tenant").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::update_tenant(&ctx, tenant_id, req.config, req.active).await {
        Ok(tenant) => Json(tenant).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                format!(
                    "Failed to update tenant. Context: Cannot update tenant. Suggestion: Verify tenant ID and ensure tenant exists. Error: {}",
                    e
                ),
                Some(ErrorContext::new("update_tenant").with_resource("tenant_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
    }
}

pub(super) async fn tenant_delete_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                format!(
                    "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'",
                    id
                ),
                Some(ErrorContext::new("delete_tenant").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::delete_tenant(&ctx, tenant_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Tenant deleted successfully"
            })),
        )
            .into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                format!(
                    "Failed to delete tenant. Context: Cannot delete tenant. Suggestion: Ensure tenant has no active resources. Error: {}",
                    e
                ),
                Some(ErrorContext::new("delete_tenant").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            (s, j).into_response()
        }
    }
}

pub(super) async fn tenant_usage_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for tenant id: {}", id),
                Some(ErrorContext::new("tenant_usage").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::get_tenant_usage(&ctx, tenant_id).await {
        Ok(usage) => Json(usage).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "TENANT_USAGE_FAILED",
                format!("Failed to retrieve tenant usage: {}", e),
                Some(ErrorContext::new("tenant_usage").with_resource("tenant_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct QuotaCheckRequest {
    workers: usize,
    memory_mb: u64,
    cpu_cores: usize,
    storage_mb: Option<u64>,
    vm_instances: Option<usize>,
}

pub(super) async fn tenant_quota_check_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
    Json(req): Json<QuotaCheckRequest>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for tenant id: {}", id),
                Some(ErrorContext::new("tenant_quota_check").with_resource("tenant_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::check_tenant_quota(
        &ctx,
        tenant_id,
        req.workers,
        req.memory_mb,
        req.cpu_cores,
        req.storage_mb,
        req.vm_instances,
    )
    .await
    {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "TENANT_QUOTA_CHECK_FAILED",
                format!("Failed to check tenant quota: {}", e),
                Some(ErrorContext::new("tenant_quota_check").with_resource("tenant_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
    }
}
