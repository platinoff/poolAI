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
    routing::{delete, get, post, put},
    Json, Router,
};
#[cfg(feature = "enterprise")]
use chrono::Utc;
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
            delete(tenant_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/tenants/{id}/usage", get(tenant_usage_handler))
        .route("/tenants/{id}/quota", post(tenant_quota_check_handler))
        // Audit logs
        .route("/audit/events", get(audit_events_query_handler))
        // Monitoring
        .route("/monitoring/alerts", get(monitoring_alerts_handler))
        .route(
            "/monitoring/alerts/{id}/acknowledge",
            post(monitoring_alert_acknowledge_handler)
                .layer(middleware::from_fn(auth_middleware)),
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
            "/security/oauth2/providers/{name}",
            get(security_oauth2_provider_get_handler),
        )
        .route(
            "/security/oauth2/providers/{name}",
            put(security_oauth2_provider_update_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/oauth2/providers/{name}",
            delete(security_oauth2_provider_delete_handler)
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
        .route(
            "/security/saml/providers/{name}",
            get(security_saml_provider_get_handler),
        )
        .route(
            "/security/saml/providers/{name}",
            put(security_saml_provider_update_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/saml/providers/{name}",
            delete(security_saml_provider_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route("/security/policies", get(security_policies_handler))
        .route(
            "/security/policies",
            post(security_policy_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/policies/{name}",
            get(security_policy_get_handler),
        )
        .route(
            "/security/policies/{name}",
            put(security_policy_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/policies/{name}",
            delete(security_policy_delete_handler).layer(middleware::from_fn(auth_middleware)),
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
#[derive(Deserialize)]
struct TenantUpdateRequest {
    config: Option<enterprise::multi_tenancy::TenantConfig>,
    active: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn tenant_update_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(req): Json<TenantUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

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
        .update_tenant(tenant_id, req.config, req.active)
        .await
    {
        Ok(tenant) => Json(tenant).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to update tenant. Context: Cannot update tenant. Suggestion: Verify tenant ID and ensure tenant exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
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
async fn audit_events_query_handler(Query(params): Query<AuditQueryParams>) -> impl IntoResponse {
    let logger = enterprise::audit::get_global_audit_logger();

    // Ensure logger is initialized
    if let Err(e) = logger.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Audit logger not initialized. Context: Audit logger is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    // Build query filters from parameters
    let filters = enterprise::audit::AuditQueryFilters {
        user_id: params.user_id,
        tenant_id: params.tenant_id,
        action: params.action,
        resource_type: params.resource_type,
        result: params.result,
        min_level: params.min_level.as_ref().and_then(|level| {
            match level.to_uppercase().as_str() {
                "INFO" => Some(enterprise::audit::AuditLevel::Info),
                "WARNING" => Some(enterprise::audit::AuditLevel::Warning),
                "ERROR" => Some(enterprise::audit::AuditLevel::Error),
                "CRITICAL" => Some(enterprise::audit::AuditLevel::Critical),
                _ => None,
            }
        }),
        start_time: params.start_time.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        end_time: params.end_time.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }),
        limit: params.limit,
    };

    match logger.query_events(&filters).await {
        Ok(events) => Json(events).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to query audit events. Context: Cannot retrieve audit events. Suggestion: Check audit log directory and permissions. Error: {}", e)
            })),
        )
            .into_response(),
    }
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
    Query(params): Query<MonitoringAlertsQuery>,
) -> impl IntoResponse {
    let manager = enterprise::monitoring::get_global_monitoring_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Monitoring manager not initialized. Context: Monitoring manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    // Parse severity filter
    let severity = params.severity.as_ref().and_then(|s| {
        match s.to_uppercase().as_str() {
            "INFO" => Some(enterprise::monitoring::AlertSeverity::Info),
            "WARNING" => Some(enterprise::monitoring::AlertSeverity::Warning),
            "ERROR" => Some(enterprise::monitoring::AlertSeverity::Error),
            "CRITICAL" => Some(enterprise::monitoring::AlertSeverity::Critical),
            _ => None,
        }
    });

    // Parse tenant ID filter
    let tenant_id = params.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());

    match manager.get_active_alerts(severity, tenant_id, params.acknowledged).await {
        Ok(alerts) => Json(alerts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve alerts. Context: Cannot retrieve monitoring alerts. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn monitoring_alert_acknowledge_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all or write:monitoring
    if let Err(err) = check_permission(&claims, "admin:all") {
        // Try write:monitoring permission
        if check_permission(&claims, "write:monitoring").is_err() {
            return err.into_response();
        }
    }

    let manager = enterprise::monitoring::get_global_monitoring_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Monitoring manager not initialized. Context: Cannot acknowledge alert - monitoring manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    let alert_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": format!("Invalid UUID format. Context: Cannot parse UUID from provided alert ID. Suggestion: Ensure UUID follows standard format. Provided ID: '{}'", id)
                })),
            )
                .into_response();
        }
    };

    match manager.acknowledge_alert(alert_id).await {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Alert acknowledged successfully",
                    "alert_id": id
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to acknowledge alert. Context: Cannot acknowledge alert. Suggestion: Verify alert ID and ensure alert exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
#[allow(dead_code)]
struct MonitoringDashboardsQuery {
    tenant_id: Option<String>,
}

#[cfg(feature = "enterprise")]
async fn monitoring_dashboards_handler(
    Query(params): Query<MonitoringDashboardsQuery>,
) -> impl IntoResponse {
    let manager = enterprise::monitoring::get_global_monitoring_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Monitoring manager not initialized. Context: Monitoring manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    // Parse tenant ID filter
    let tenant_id = params.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());

    match manager.list_dashboards(tenant_id).await {
        Ok(dashboards) => Json(dashboards).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve dashboards. Context: Cannot retrieve monitoring dashboards. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct DashboardCreateRequest {
    name: String,
    description: Option<String>,
    metrics: Vec<String>,
    layout: Option<String>,
    is_public: Option<bool>,
    tenant_id: Option<String>,
}

#[cfg(feature = "enterprise")]
async fn monitoring_dashboard_create_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<DashboardCreateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all or write:monitoring
    if let Err(err) = check_permission(&claims, "admin:all") {
        // Try write:monitoring permission
        if check_permission(&claims, "write:monitoring").is_err() {
            return err.into_response();
        }
    }

    let manager = enterprise::monitoring::get_global_monitoring_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Monitoring manager not initialized. Context: Cannot create dashboard - monitoring manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    // Parse tenant ID if provided
    let tenant_id = req.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());

    let dashboard = enterprise::monitoring::Dashboard {
        id: Uuid::new_v4(),
        name: req.name,
        description: req.description.unwrap_or_default(),
        metrics: req.metrics,
        layout: req.layout.unwrap_or_else(|| "{}".to_string()),
        is_public: req.is_public.unwrap_or(false),
        tenant_id,
        created_at: Utc::now(),
    };

    match manager.create_dashboard(dashboard.clone()).await {
        Ok(()) => {
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "message": "Dashboard created successfully",
                    "dashboard_id": dashboard.id
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to create dashboard. Context: Cannot create monitoring dashboard. Suggestion: Verify dashboard configuration and parameters. Error: {}", e)
            })),
        )
            .into_response(),
    }
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
    Query(params): Query<MonitoringMetricsQuery>,
) -> impl IntoResponse {
    let manager = enterprise::monitoring::get_global_monitoring_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Monitoring manager not initialized. Context: Monitoring manager is not available. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    // Parse time range filters
    let start_time = params.start_time.and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });
    let end_time = params.end_time.and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    // Parse tenant ID filter
    let tenant_id = params.tenant_id.and_then(|id| Uuid::parse_str(&id).ok());

    match manager
        .get_metric_history(
            params.metric.as_deref(),
            start_time,
            end_time,
            tenant_id,
            params.limit,
        )
        .await
    {
        Ok(metrics) => Json(metrics).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve metrics. Context: Cannot retrieve monitoring metrics. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
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

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_get_handler(Path(name): Path<String>) -> impl IntoResponse {
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

    match manager.get_oauth2_provider(&name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("OAuth2 provider not found: {}. Context: Cannot find OAuth2 provider with specified name. Suggestion: Verify provider name and ensure provider exists.", name)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve OAuth2 provider. Context: Cannot retrieve OAuth2 provider information. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct OAuth2ProviderUpdateRequest {
    config: Option<enterprise::security::OAuth2Config>,
    enabled: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_update_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
    Json(req): Json<OAuth2ProviderUpdateRequest>,
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
                "error": format!("Security manager not initialized. Context: Cannot update OAuth2 provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager
        .update_oauth2_provider(name.clone(), req.config, req.enabled)
        .await
    {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "OAuth2 provider updated successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to update OAuth2 provider. Context: Cannot update OAuth2 provider. Suggestion: Verify provider name and ensure provider exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
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
                "error": format!("Security manager not initialized. Context: Cannot delete OAuth2 provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.delete_oauth2_provider(&name).await {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "OAuth2 provider deleted successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to delete OAuth2 provider. Context: Cannot delete OAuth2 provider. Suggestion: Verify provider name and ensure provider exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_get_handler(Path(name): Path<String>) -> impl IntoResponse {
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

    match manager.get_saml_provider(&name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("SAML provider not found: {}. Context: Cannot find SAML provider with specified name. Suggestion: Verify provider name and ensure provider exists.", name)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve SAML provider. Context: Cannot retrieve SAML provider information. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct SamlProviderUpdateRequest {
    config: Option<enterprise::security::SamlConfig>,
    enabled: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_update_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
    Json(req): Json<SamlProviderUpdateRequest>,
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
                "error": format!("Security manager not initialized. Context: Cannot update SAML provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager
        .update_saml_provider(name.clone(), req.config, req.enabled)
        .await
    {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "SAML provider updated successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to update SAML provider. Context: Cannot update SAML provider. Suggestion: Verify provider name and ensure provider exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
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
                "error": format!("Security manager not initialized. Context: Cannot delete SAML provider - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.delete_saml_provider(&name).await {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "SAML provider deleted successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to delete SAML provider. Context: Cannot delete SAML provider. Suggestion: Verify provider name and ensure provider exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_get_handler(Path(name): Path<String>) -> impl IntoResponse {
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

    match manager.get_security_policy(&name).await {
        Ok(Some(policy)) => Json(policy).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Security policy not found: {}. Context: Cannot find security policy with specified name. Suggestion: Verify policy name and ensure policy exists.", name)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to retrieve security policy. Context: Cannot retrieve security policy information. Suggestion: Check system logs. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_update_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
    Json(policy): Json<enterprise::security::SecurityPolicy>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    // Ensure policy name matches path
    if policy.name != name {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Policy name mismatch. Context: Policy name in body '{}' does not match path parameter '{}'. Suggestion: Ensure policy name matches the path parameter.", policy.name, name)
            })),
        )
            .into_response();
    }

    let manager = enterprise::security::get_global_security_manager();

    // Ensure manager is initialized
    if let Err(e) = manager.initialize().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Security manager not initialized. Context: Cannot update security policy - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.update_security_policy(policy.clone()).await {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Security policy updated successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Failed to update security policy. Context: Cannot update security policy. Suggestion: Verify policy name and ensure policy exists. Error: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
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
                "error": format!("Security manager not initialized. Context: Cannot delete security policy - security manager initialization failed. Suggestion: Check system startup sequence. Error: {}", e)
            })),
        )
            .into_response();
    }

    match manager.delete_security_policy(&name).await {
        Ok(()) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Security policy deleted successfully",
                    "name": name
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to delete security policy. Context: Cannot delete security policy. Suggestion: Verify policy name and ensure policy exists. Error: {}", e)
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
