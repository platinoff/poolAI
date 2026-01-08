//! Enterprise API endpoints
//!
//! Provides REST API endpoints for enterprise features:
//! - Tenant management
//! - Security (OAuth2, SAML)
//! - Audit logs
//! - Advanced monitoring

#[cfg(feature = "enterprise")]
use crate::enterprise;
#[cfg(feature = "enterprise")]
use crate::network::api::check_permission;
#[cfg(feature = "enterprise")]
use crate::network::auth::{auth_middleware, Claims};
#[cfg(feature = "enterprise")]
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
#[cfg(feature = "enterprise")]
use serde::Deserialize;
#[cfg(feature = "enterprise")]
use uuid::Uuid;

#[cfg(feature = "enterprise")]
pub fn create_enterprise_api_routes() -> Router {
    Router::new()
        // Tenant management
        .route("/tenants", get(tenants_list_handler))
        .route(
            "/tenants",
            post(tenant_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/tenants/{id}", get(tenant_get_handler))
        .route(
            "/tenants/{id}",
            post(tenant_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/tenants/{id}",
            axum::routing::delete(tenant_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route("/tenants/{id}/usage", get(tenant_usage_handler))
        .route("/tenants/{id}/quota", post(tenant_quota_check_handler))
        // Audit logs
        .route("/audit/events", get(audit_events_query_handler))
        // Monitoring
        .route("/monitoring/alerts", get(monitoring_alerts_handler))
        .route(
            "/monitoring/alerts/{id}/acknowledge",
            post(monitoring_alert_acknowledge_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/dashboards", get(monitoring_dashboards_handler))
        .route(
            "/monitoring/dashboards",
            post(monitoring_dashboard_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/metrics", get(monitoring_metrics_handler))
        // Security
        .route(
            "/security/oauth2/providers",
            get(security_oauth2_providers_handler),
        )
        .route(
            "/security/oauth2/providers",
            post(security_oauth2_provider_register_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/saml/providers",
            get(security_saml_providers_handler),
        )
        .route(
            "/security/saml/providers",
            post(security_saml_provider_register_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route("/security/policies", get(security_policies_handler))
        .route(
            "/security/policies",
            post(security_policy_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
}

// ============================================================================
// Tenant Management Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
async fn tenants_list_handler() -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();
    match manager.list_tenants().await {
        Ok(tenants) => Json(tenants).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to list tenants. Context: Cannot retrieve tenant list. Suggestion: Check system logs and tenant manager initialization status. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct TenantCreateRequest {
    name: String,
    config: enterprise::multi_tenancy::TenantConfig,
}

#[cfg(feature = "enterprise")]
async fn tenant_create_handler(Json(req): Json<TenantCreateRequest>) -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Tenant manager not initialized. Context: Cannot create tenant - tenant manager initialization failed. Suggestion: Check system startup sequence and tenant manager initialization status. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.create_tenant(req.name, req.config).await {
        Ok(tenant) => Json(tenant).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to create tenant. Context: Cannot create new tenant with specified configuration. Suggestion: Verify tenant name and configuration parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn tenant_get_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();

    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format (e.g., '550e8400-e29b-41d4-a716-446655440000'). Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_tenant(tenant_id).await {
        Ok(Some(tenant)) => Json(tenant).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Tenant not found. Context: Cannot find tenant with specified ID. Suggestion: Verify tenant ID and ensure tenant exists. Tenant ID: '{}'", id)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve tenant. Context: Cannot retrieve tenant information. Suggestion: Check system logs and tenant manager status. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn tenant_update_handler(
    Path(_id): Path<String>,
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Tenant update not yet implemented"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
async fn tenant_delete_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();

    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.delete_tenant(tenant_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Tenant deleted successfully"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to delete tenant. Context: Cannot delete tenant. Suggestion: Ensure tenant has no active resources. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn tenant_usage_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();

    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_usage(tenant_id).await {
        Ok(usage) => Json(usage).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve tenant usage. Context: Cannot retrieve usage information for tenant. Suggestion: Verify tenant ID and ensure tenant exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct QuotaCheckRequest {
    workers: usize,
    memory_mb: u64,
    cpu_cores: usize,
    storage_mb: Option<u64>,
    vm_instances: Option<usize>,
}

#[cfg(feature = "enterprise")]
async fn tenant_quota_check_handler(
    Path(id): Path<String>,
    Json(req): Json<QuotaCheckRequest>,
) -> impl IntoResponse {
    let manager = enterprise::multi_tenancy::get_global_tenant_manager();

    let tenant_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided identifier. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager
        .check_quota(
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
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to check quota. Context: Cannot check quota for tenant. Suggestion: Verify tenant ID and ensure tenant exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

// ============================================================================
// Audit Logs Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
#[allow(dead_code)]
struct AuditQueryParams {
    user_id: Option<String>,
    tenant_id: Option<String>,
    action: Option<String>,
    resource_type: Option<String>,
    result: Option<String>,
    min_level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    limit: Option<usize>,
}

#[cfg(feature = "enterprise")]
async fn audit_events_query_handler(Query(_params): Query<AuditQueryParams>) -> impl IntoResponse {
    // TODO: Get global audit logger and query events
    // For now, return empty list
    Json::<Vec<enterprise::audit::AuditEvent>>(Vec::new())
}

// ============================================================================
// Monitoring Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
#[allow(dead_code)]
struct MonitoringAlertsQuery {
    severity: Option<String>,
    tenant_id: Option<String>,
    acknowledged: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn monitoring_alerts_handler(
    Query(_params): Query<MonitoringAlertsQuery>,
) -> impl IntoResponse {
    // TODO: Get global monitoring manager and retrieve alerts
    Json::<Vec<enterprise::monitoring::Alert>>(Vec::new())
}

#[cfg(feature = "enterprise")]
async fn monitoring_alert_acknowledge_handler(Path(_id): Path<String>) -> impl IntoResponse {
    // TODO: Get global monitoring manager and acknowledge alert
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Alert acknowledgment not yet implemented"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
async fn monitoring_dashboards_handler() -> impl IntoResponse {
    // TODO: Get global monitoring manager and retrieve dashboards
    Json::<Vec<enterprise::monitoring::Dashboard>>(Vec::new())
}

#[cfg(feature = "enterprise")]
async fn monitoring_dashboard_create_handler(
    Json(_req): Json<serde_json::Value>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Dashboard creation not yet implemented"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
#[allow(dead_code)]
struct MonitoringMetricsQuery {
    metric: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    tenant_id: Option<String>,
    limit: Option<usize>,
}

#[cfg(feature = "enterprise")]
async fn monitoring_metrics_handler(
    Query(_params): Query<MonitoringMetricsQuery>,
) -> impl IntoResponse {
    // TODO: Get global monitoring manager and retrieve metrics
    Json::<Vec<enterprise::monitoring::MetricDataPoint>>(Vec::new())
}

// ============================================================================
// Security Management Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct OAuth2ProviderRegisterRequest {
    name: String,
    config: enterprise::security::OAuth2Config,
    enabled: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_providers_handler() -> impl IntoResponse {
    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Security manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.list_oauth2_providers().await {
        Ok(providers) => Json(providers).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to list OAuth2 providers. Context: Cannot retrieve OAuth2 provider list. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_register_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<OAuth2ProviderRegisterRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Cannot register OAuth2 provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.register_oauth2_provider(req.name.clone(), req.config).await {
        Ok(()) => {
            // Update enabled status if provided
            if let Some(enabled) = req.enabled {
                if enabled {
                    // Provider is enabled by default when registered
                }
            }

            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "message": "OAuth2 provider registered successfully",
                    "name": req.name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to register OAuth2 provider. Context: Cannot register OAuth2 provider with specified configuration. Suggestion: Verify provider name and configuration parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct SamlProviderRegisterRequest {
    name: String,
    config: enterprise::security::SamlConfig,
    enabled: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn security_saml_providers_handler() -> impl IntoResponse {
    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Security manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.list_saml_providers().await {
        Ok(providers) => Json(providers).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to list SAML providers. Context: Cannot retrieve SAML provider list. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_register_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<SamlProviderRegisterRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Cannot register SAML provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.register_saml_provider(req.name.clone(), req.config).await {
        Ok(()) => {
            // Update enabled status if provided
            if let Some(enabled) = req.enabled {
                if enabled {
                    // Provider is enabled by default when registered
                }
            }

            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "message": "SAML provider registered successfully",
                    "name": req.name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to register SAML provider. Context: Cannot register SAML provider with specified configuration. Suggestion: Verify provider name and configuration parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct SecurityPolicyCreateRequest {
    #[allow(dead_code)] // Policy name is included in the policy struct itself
    name: String,
    policy: enterprise::security::SecurityPolicy,
}

#[cfg(feature = "enterprise")]
async fn security_policies_handler() -> impl IntoResponse {
    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Security manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.list_security_policies().await {
        Ok(policies) => Json(policies).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to list security policies. Context: Cannot retrieve security policy list. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_create_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<SecurityPolicyCreateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Cannot create security policy - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.create_security_policy(req.policy).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Security policy created successfully"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to create security policy. Context: Cannot create security policy with specified parameters. Suggestion: Verify policy name and configuration. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

// ============================================================================
// Non-enterprise stub
// ============================================================================

#[cfg(not(feature = "enterprise"))]
pub fn create_enterprise_api_routes() -> Router {
    Router::new()
}
