//! Enterprise API: tenants.

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::enterprise;
use crate::network::api::check_permission;
use crate::network::api::common::HttpAppError;
use crate::network::auth::Claims;
use crate::services::enterprise_service::{EnterpriseService, TenantCreateError};
use axum::extract::{Extension, Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use super::enterprise_json_err;

pub(super) async fn tenants_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_tenants(&ctx).await {
        Ok(tenants) => Json(tenants).into_response(),
        Err(e) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to list tenants. Context: Cannot retrieve tenant list. Suggestion: Check system logs and tenant manager initialization status. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("list_tenants"))
        .into_response(),
    }
}

/// GET /tenants/store — band-52 wire snapshot over HTTP (PH-S1173).
pub(super) async fn tenant_store_wire_handler() -> impl IntoResponse {
    Json(enterprise::multi_tenancy::tenant_store_wire())
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
        Err(TenantCreateError::Init(e)) => HttpAppError::new(AppError::SubsystemUnavailable(format!(
            "Tenant manager not initialized. Context: Cannot create tenant - tenant manager initialization failed. Suggestion: Check system startup sequence and tenant manager initialization status. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("create_tenant").with_resource("tenant_manager", "default"))
        .into_response(),
        Err(TenantCreateError::Create(e)) => HttpAppError::new(AppError::ValidationError(format!(
            "Failed to create tenant. Context: Cannot create new tenant with specified configuration. Suggestion: Verify tenant name and configuration parameters. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("create_tenant"))
        .into_response(),
    }
}

pub(super) async fn tenant_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return HttpAppError::new(AppError::ValidationError(format!(
                "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'",
                id
            )))
            .with_context(ErrorContext::new("get_tenant").with_resource("tenant_id", &id))
            .into_response();
        }
    };

    match EnterpriseService::get_tenant(&ctx, tenant_id).await {
        Ok(Some(tenant)) => Json(tenant).into_response(),
        Ok(None) => HttpAppError::new(AppError::ApiNotFound(format!(
            "Tenant not found. Context: Cannot find tenant with specified ID. Suggestion: Verify tenant ID and ensure tenant exists. Tenant ID: '{}'",
            id
        )))
        .with_context(ErrorContext::new("get_tenant").with_resource("tenant_id", &id))
        .into_response(),
        Err(e) => HttpAppError::new(AppError::InternalError(format!(
            "Failed to retrieve tenant. Context: Cannot retrieve tenant information. Suggestion: Check system logs and tenant manager status. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("get_tenant").with_resource("tenant_id", &id))
        .into_response(),
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
            return HttpAppError::new(AppError::ValidationError(format!(
                "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'",
                id
            )))
            .with_context(ErrorContext::new("update_tenant").with_resource("tenant_id", &id))
            .into_response();
        }
    };

    match EnterpriseService::update_tenant(&ctx, tenant_id, req.config, req.active).await {
        Ok(tenant) => Json(tenant).into_response(),
        Err(e) => HttpAppError::new(AppError::ApiNotFound(format!(
            "Failed to update tenant. Context: Cannot update tenant. Suggestion: Verify tenant ID and ensure tenant exists. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("update_tenant").with_resource("tenant_id", &id))
        .into_response(),
    }
}

pub(super) async fn tenant_delete_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return HttpAppError::new(AppError::ValidationError(format!(
                "Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'",
                id
            )))
            .with_context(ErrorContext::new("delete_tenant").with_resource("tenant_id", &id))
            .into_response();
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
        Err(e) => HttpAppError::new(AppError::ValidationError(format!(
            "Failed to delete tenant. Context: Cannot delete tenant. Suggestion: Ensure tenant has no active resources. Error: {}",
            e
        )))
        .with_context(ErrorContext::new("delete_tenant").with_resource("tenant_id", &id))
        .into_response(),
    }
}

pub(super) async fn tenant_usage_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return enterprise_json_err(
                "INVALID_UUID",
                format!("Invalid UUID format for tenant id: {}", id),
                ErrorContext::new("tenant_usage").with_resource("tenant_id", &id),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }
    };

    match EnterpriseService::get_tenant_usage(&ctx, tenant_id).await {
        Ok(usage) => Json(usage).into_response(),
        Err(e) => enterprise_json_err(
            "TENANT_USAGE_FAILED",
            format!("Failed to retrieve tenant usage: {}", e),
            ErrorContext::new("tenant_usage").with_resource("tenant_id", &id),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
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
            return enterprise_json_err(
                "INVALID_UUID",
                format!("Invalid UUID format for tenant id: {}", id),
                ErrorContext::new("tenant_quota_check").with_resource("tenant_id", &id),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
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
        Err(e) => enterprise_json_err(
            "TENANT_QUOTA_CHECK_FAILED",
            format!("Failed to check tenant quota: {}", e),
            ErrorContext::new("tenant_quota_check").with_resource("tenant_id", &id),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}
