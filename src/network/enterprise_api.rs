//! Enterprise API endpoints
//!
//! Provides REST API endpoints for enterprise features:
//! - Tenant management
//! - Security (OAuth2, SAML)
//! - Audit logs
//! - Advanced monitoring

#[cfg(feature = "enterprise")]
use crate::core::error::ErrorContext;
#[cfg(feature = "enterprise")]
use crate::core::oauth2_pending::{store_oauth2_pending, verify_oauth2_pending};
#[cfg(feature = "enterprise")]
use crate::core::state::ApiContext;
#[cfg(feature = "enterprise")]
use crate::enterprise;
#[cfg(feature = "enterprise")]
use crate::network::api::check_permission;
#[cfg(feature = "enterprise")]
use crate::network::api::common::api_json_error;
#[cfg(feature = "enterprise")]
use crate::network::auth::{auth_middleware, Claims};
#[cfg(feature = "enterprise")]
use crate::services::enterprise_service::{
    AuditEventsQuery, DashboardCreateInput, EnterpriseAuditError, EnterpriseMonitoringError,
    EnterpriseOAuthStartError, EnterpriseSecurityError, EnterpriseService, MetricHistoryQueryInput,
    MonitoringAlertsQueryInput, TenantCreateError,
};
#[cfg(feature = "enterprise")]
use axum::{
    extract::{Extension, Form, Path, Query, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Redirect},
    routing::{delete, get, post, put},
    Json, Router,
};
#[cfg(feature = "enterprise")]
use serde::Deserialize;
#[cfg(feature = "enterprise")]
use uuid::Uuid;

/// Shorthand for structured enterprise API errors (same shape as [`api_json_error`]).
#[cfg(feature = "enterprise")]
fn enterprise_err(
    code: impl AsRef<str>,
    message: impl Into<String>,
    operation: impl Into<String>,
    status: StatusCode,
) -> (StatusCode, Json<serde_json::Value>) {
    api_json_error(
        code,
        message,
        Some(ErrorContext::new(operation.into())),
        status,
    )
}

#[cfg(feature = "enterprise")]
pub fn create_enterprise_api_routes() -> Router<ApiContext> {
    let router = Router::new()
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
            post(monitoring_alert_acknowledge_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/dashboards", get(monitoring_dashboards_handler))
        .route(
            "/monitoring/dashboards",
            post(monitoring_dashboard_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/metrics", get(monitoring_metrics_handler))
        .route(
            "/monitoring/alert-rules",
            post(monitoring_alert_rule_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/monitoring/alert-rules",
            get(monitoring_alert_rules_handler),
        )
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
            put(security_saml_provider_update_handler).layer(middleware::from_fn(auth_middleware)),
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
        // OAuth2 Authentication endpoints
        .route("/auth/github", get(oauth2_github_auth_handler))
        .route("/auth/github/callback", get(oauth2_github_callback_handler))
        .route("/auth/google", get(oauth2_google_auth_handler))
        .route("/auth/google/callback", get(oauth2_google_callback_handler))
        .route("/auth/telegram", get(oauth2_telegram_auth_handler))
        .route(
            "/auth/telegram/callback",
            get(oauth2_telegram_callback_handler),
        )
        // SAML SSO Authentication endpoints
        .route("/auth/saml/{provider}", get(saml_auth_handler))
        .route(
            "/auth/saml/{provider}/callback",
            post(saml_callback_handler),
        );
    #[cfg(all(feature = "enterprise", feature = "ml"))]
    let router = router.nest("/ai-ml", crate::network::api::ai_ml::create_ai_ml_routes());
    router
}

// ============================================================================
// Tenant Management Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
async fn tenants_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
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

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct TenantCreateRequest {
    name: String,
    config: enterprise::multi_tenancy::TenantConfig,
}

#[cfg(feature = "enterprise")]
async fn tenant_create_handler(
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

#[cfg(feature = "enterprise")]
async fn tenant_get_handler(
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

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
struct TenantUpdateRequest {
    config: Option<enterprise::multi_tenancy::TenantConfig>,
    active: Option<bool>,
}

#[cfg(feature = "enterprise")]
async fn tenant_update_handler(
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

#[cfg(feature = "enterprise")]
async fn tenant_delete_handler(
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

#[cfg(feature = "enterprise")]
async fn tenant_usage_handler(
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
async fn audit_events_query_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<AuditQueryParams>,
) -> impl IntoResponse {
    let q = AuditEventsQuery {
        user_id: params.user_id,
        tenant_id: params.tenant_id,
        action: params.action,
        resource_type: params.resource_type,
        result: params.result,
        min_level: params.min_level,
        start_time: params.start_time,
        end_time: params.end_time,
        limit: params.limit,
    };

    match EnterpriseService::query_audit_events(&ctx, q).await {
        Ok(events) => Json(events).into_response(),
        Err(EnterpriseAuditError::Init(e)) => {
            let (s, j) = api_json_error(
                "AUDIT_LOGGER_UNAVAILABLE",
                format!("Audit logger not initialized: {}", e),
                Some(
                    ErrorContext::new("audit_events_query")
                        .with_hint("Check system startup sequence and audit logger wiring."),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseAuditError::Query(e)) => {
            let (s, j) = api_json_error(
                "AUDIT_QUERY_FAILED",
                format!("Failed to query audit events: {}", e),
                Some(ErrorContext::new("audit_events_query")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
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
    State(ctx): State<ApiContext>,
    Query(params): Query<MonitoringAlertsQuery>,
) -> impl IntoResponse {
    let q = MonitoringAlertsQueryInput {
        severity: params.severity,
        tenant_id: params.tenant_id,
        acknowledged: params.acknowledged,
    };

    match EnterpriseService::list_monitoring_alerts(&ctx, q).await {
        Ok(alerts) => Json(alerts).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(
                    ErrorContext::new("monitoring_alerts")
                        .with_hint("Check system startup sequence."),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_ALERTS_FAILED",
                format!("Failed to retrieve alerts: {}", e),
                Some(ErrorContext::new("monitoring_alerts")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
    }
}

#[cfg(feature = "enterprise")]
async fn monitoring_alert_acknowledge_handler(
    State(ctx): State<ApiContext>,
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

    let alert_id = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format for alert id: {}", id),
                Some(ErrorContext::new("monitoring_alert_ack").with_resource("alert_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, j).into_response();
        }
    };

    match EnterpriseService::acknowledge_monitoring_alert(&ctx, alert_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Alert acknowledged successfully",
                "alert_id": id
            })),
        )
            .into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_alert_ack")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_ALERT_ACK_FAILED",
                format!("Failed to acknowledge alert: {}", e),
                Some(ErrorContext::new("monitoring_alert_ack").with_resource("alert_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
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
    State(ctx): State<ApiContext>,
    Query(params): Query<MonitoringDashboardsQuery>,
) -> impl IntoResponse {
    match EnterpriseService::list_monitoring_dashboards(&ctx, params.tenant_id).await {
        Ok(dashboards) => Json(dashboards).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_dashboards_list")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_DASHBOARDS_FAILED",
                format!("Failed to retrieve dashboards: {}", e),
                Some(ErrorContext::new("monitoring_dashboards_list")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
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
    State(ctx): State<ApiContext>,
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

    let input = DashboardCreateInput {
        name: req.name,
        description: req.description,
        metrics: req.metrics,
        layout: req.layout,
        is_public: req.is_public,
        tenant_id: req.tenant_id,
    };

    match EnterpriseService::create_monitoring_dashboard(&ctx, input).await {
        Ok(dashboard) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Dashboard created successfully",
                "dashboard_id": dashboard.id
            })),
        )
            .into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_dashboard_create")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_DASHBOARD_CREATE_FAILED",
                format!("Failed to create dashboard: {}", e),
                Some(ErrorContext::new("monitoring_dashboard_create")),
                StatusCode::BAD_REQUEST,
            );
            (s, j).into_response()
        }
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
    State(ctx): State<ApiContext>,
    Query(params): Query<MonitoringMetricsQuery>,
) -> impl IntoResponse {
    let q = MetricHistoryQueryInput {
        metric: params.metric,
        start_time: params.start_time,
        end_time: params.end_time,
        tenant_id: params.tenant_id,
        limit: params.limit,
    };

    match EnterpriseService::query_monitoring_metric_history(&ctx, q).await {
        Ok(metrics) => Json(metrics).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_metrics")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_METRICS_FAILED",
                format!("Failed to retrieve metrics: {}", e),
                Some(ErrorContext::new("monitoring_metrics")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
    }
}

#[cfg(feature = "enterprise")]
async fn monitoring_alert_rules_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_monitoring_alert_rules(&ctx).await {
        Ok(rules) => Json(rules).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_alert_rules_list")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_ALERT_RULES_FAILED",
                format!("Failed to retrieve alert rules: {}", e),
                Some(ErrorContext::new("monitoring_alert_rules_list")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
    }
}

#[cfg(feature = "enterprise")]
async fn monitoring_alert_rule_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(rule): Json<enterprise::monitoring::AlertRule>,
) -> impl IntoResponse {
    // Check permission: admin:all or write:monitoring
    if let Err(err) = check_permission(&claims, "admin:all") {
        // Try write:monitoring permission
        if check_permission(&claims, "write:monitoring").is_err() {
            return err.into_response();
        }
    }

    let rule_name = rule.name.clone();

    match EnterpriseService::create_monitoring_alert_rule(&ctx, rule).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Alert rule created successfully",
                "rule_name": rule_name
            })),
        )
            .into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_MANAGER_UNAVAILABLE",
                format!("Monitoring manager not initialized: {}", e),
                Some(ErrorContext::new("monitoring_alert_rule_create")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, j).into_response()
        }
        Err(EnterpriseMonitoringError::Operation(e)) => {
            let (s, j) = api_json_error(
                "MONITORING_ALERT_RULE_CREATE_FAILED",
                format!("Failed to create alert rule: {}", e),
                Some(ErrorContext::new("monitoring_alert_rule_create")),
                StatusCode::BAD_REQUEST,
            );
            (s, j).into_response()
        }
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
async fn security_oauth2_providers_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_oauth2_providers(&ctx).await {
        Ok(providers) => Json(providers).into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_oauth2_providers_list",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_OAUTH2_LIST_FAILED",
            format!("Failed to list OAuth2 providers: {}", e),
            "security_oauth2_providers_list",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_register_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<OAuth2ProviderRegisterRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::register_oauth2_provider(&ctx, req.name.clone(), req.config).await {
        Ok(()) => {
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
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_oauth2_provider_register",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_OAUTH2_REGISTER_FAILED",
            format!("Failed to register OAuth2 provider: {}", e),
            "security_oauth2_provider_register",
            StatusCode::BAD_REQUEST,
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
async fn security_saml_providers_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_saml_providers(&ctx).await {
        Ok(providers) => Json(providers).into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_saml_providers_list",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_SAML_LIST_FAILED",
            format!("Failed to list SAML providers: {}", e),
            "security_saml_providers_list",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_register_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<SamlProviderRegisterRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::register_saml_provider(&ctx, req.name.clone(), req.config).await {
        Ok(()) => {
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
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_saml_provider_register",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_SAML_REGISTER_FAILED",
            format!("Failed to register SAML provider: {}", e),
            "security_saml_provider_register",
            StatusCode::BAD_REQUEST,
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
async fn security_policies_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::list_security_policies(&ctx).await {
        Ok(policies) => Json(policies).into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_policies_list",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_POLICIES_LIST_FAILED",
            format!("Failed to list security policies: {}", e),
            "security_policies_list",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<SecurityPolicyCreateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::create_security_policy(&ctx, req.policy).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Security policy created successfully"
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_policy_create",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_POLICY_CREATE_FAILED",
            format!("Failed to create security policy: {}", e),
            "security_policy_create",
            StatusCode::BAD_REQUEST,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_oauth2_provider(&ctx, &name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "OAUTH2_PROVIDER_NOT_FOUND",
                format!("OAuth2 provider not found: {}", name),
                Some(
                    ErrorContext::new("security_oauth2_provider_get").with_resource("name", &name),
                ),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_oauth2_provider_get",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_OAUTH2_GET_FAILED",
            format!("Failed to retrieve OAuth2 provider: {}", e),
            "security_oauth2_provider_get",
            StatusCode::INTERNAL_SERVER_ERROR,
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
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
    Json(req): Json<OAuth2ProviderUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::update_oauth2_provider(&ctx, name.clone(), req.config, req.enabled)
        .await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "OAuth2 provider updated successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_oauth2_provider_update",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_OAUTH2_UPDATE_FAILED",
            format!("Failed to update OAuth2 provider: {}", e),
            "security_oauth2_provider_update",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_oauth2_provider_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::delete_oauth2_provider(&ctx, &name).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "OAuth2 provider deleted successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_oauth2_provider_delete",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_OAUTH2_DELETE_FAILED",
            format!("Failed to delete OAuth2 provider: {}", e),
            "security_oauth2_provider_delete",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_saml_provider(&ctx, &name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "SAML_PROVIDER_NOT_FOUND",
                format!("SAML provider not found: {}", name),
                Some(ErrorContext::new("security_saml_provider_get").with_resource("name", &name)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_saml_provider_get",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_SAML_GET_FAILED",
            format!("Failed to retrieve SAML provider: {}", e),
            "security_saml_provider_get",
            StatusCode::INTERNAL_SERVER_ERROR,
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
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
    Json(req): Json<SamlProviderUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::update_saml_provider(&ctx, name.clone(), req.config, req.enabled).await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "SAML provider updated successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_saml_provider_update",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_SAML_UPDATE_FAILED",
            format!("Failed to update SAML provider: {}", e),
            "security_saml_provider_update",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_saml_provider_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::delete_saml_provider(&ctx, &name).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "SAML provider deleted successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_saml_provider_delete",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_SAML_DELETE_FAILED",
            format!("Failed to delete SAML provider: {}", e),
            "security_saml_provider_delete",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_security_policy(&ctx, &name).await {
        Ok(Some(policy)) => Json(policy).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "SECURITY_POLICY_NOT_FOUND",
                format!("Security policy not found: {}", name),
                Some(ErrorContext::new("security_policy_get").with_resource("name", &name)),
                StatusCode::NOT_FOUND,
            );
            (s, j).into_response()
        }
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_policy_get",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_POLICY_GET_FAILED",
            format!("Failed to retrieve security policy: {}", e),
            "security_policy_get",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_update_handler(
    State(ctx): State<ApiContext>,
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
        let (s, j) = api_json_error(
            "POLICY_NAME_MISMATCH",
            format!(
                "Policy name in body '{}' does not match path parameter '{}'",
                policy.name, name
            ),
            Some(ErrorContext::new("security_policy_update")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    match EnterpriseService::update_security_policy(&ctx, policy).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Security policy updated successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_policy_update",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_POLICY_UPDATE_FAILED",
            format!("Failed to update security policy: {}", e),
            "security_policy_update",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn security_policy_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match EnterpriseService::delete_security_policy(&ctx, &name).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Security policy deleted successfully",
                "name": name
            })),
        )
            .into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "security_policy_delete",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SECURITY_POLICY_DELETE_FAILED",
            format!("Failed to delete security policy: {}", e),
            "security_policy_delete",
            StatusCode::BAD_REQUEST,
        )
        .into_response(),
    }
}

// ============================================================================
// OAuth2 Authentication Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
async fn oauth2_github_auth_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let state = uuid::Uuid::new_v4().to_string();
    store_oauth2_pending(&ctx.oauth2_pending_states, state.clone()).await;

    match EnterpriseService::start_oauth2_authorization(&ctx, "github", &state).await {
        Ok(auth_url) => Redirect::temporary(&auth_url).into_response(),
        Err(EnterpriseOAuthStartError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "oauth2_github_auth",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ListProviders(e)) => enterprise_err(
            "OAUTH2_LIST_PROVIDERS_FAILED",
            format!("Failed to list OAuth2 providers: {}", e),
            "oauth2_github_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ProviderNotConfigured) => enterprise_err(
            "OAUTH2_PROVIDER_NOT_CONFIGURED",
            "GitHub OAuth2 provider not configured. Register it in the admin panel.",
            "oauth2_github_auth",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::AuthUrl(e)) => enterprise_err(
            "OAUTH2_AUTH_URL_FAILED",
            format!("Failed to generate authorization URL: {}", e),
            "oauth2_github_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn oauth2_github_callback_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // GitHub OAuth2 callback flow implementation:
    // 1. ✅ Verify state parameter (CSRF protection) - implemented
    // 2. ✅ Exchange authorization code for access token - implemented
    // 3. ✅ Get user info from GitHub API - implemented
    // 4. ✅ Create or find user in PoolAI - implemented
    // 5. ✅ Generate PoolAI JWT token - implemented
    // 6. ✅ Return token to client - implemented

    let code = params.get("code").cloned().unwrap_or_default();
    let state = params.get("state").cloned().unwrap_or_default();
    let error = params.get("error").cloned();

    if let Some(error) = error {
        let (s, j) = api_json_error(
            "OAUTH2_PROVIDER_ERROR",
            format!("OAuth2 error: {}", error),
            Some(ErrorContext::new("oauth2_github_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    if code.is_empty() {
        let (s, j) = api_json_error(
            "OAUTH2_MISSING_CODE",
            "Missing authorization code",
            Some(ErrorContext::new("oauth2_github_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    // Verify state parameter (CSRF protection)
    if state.is_empty() || !verify_oauth2_pending(&ctx.oauth2_pending_states, &state).await {
        let (s, j) = api_json_error(
            "OAUTH2_INVALID_STATE",
            "Invalid or expired state parameter. Please try again.",
            Some(ErrorContext::new("oauth2_github_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    let token_response = match EnterpriseService::exchange_oauth2_code(&ctx, "github", &code).await
    {
        Ok(token) => token,
        Err(EnterpriseSecurityError::Init(e)) => {
            return enterprise_err(
                "SECURITY_MANAGER_UNAVAILABLE",
                format!("Security manager not initialized: {}", e),
                "oauth2_github_callback",
                StatusCode::SERVICE_UNAVAILABLE,
            )
            .into_response();
        }
        Err(EnterpriseSecurityError::Operation(e)) => {
            return enterprise_err(
                "OAUTH2_CODE_EXCHANGE_FAILED",
                format!("Failed to exchange authorization code: {}", e),
                "oauth2_github_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    let user_info =
        match EnterpriseService::get_oauth2_user_info(&ctx, "github", &token_response.access_token)
            .await
        {
            Ok(info) => info,
            Err(EnterpriseSecurityError::Init(e)) => {
                return enterprise_err(
                    "SECURITY_MANAGER_UNAVAILABLE",
                    format!("Security manager not initialized: {}", e),
                    "oauth2_github_callback",
                    StatusCode::SERVICE_UNAVAILABLE,
                )
                .into_response();
            }
            Err(EnterpriseSecurityError::Operation(e)) => {
                return enterprise_err(
                    "OAUTH2_USERINFO_FAILED",
                    format!("Failed to get user info from GitHub: {}", e),
                    "oauth2_github_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        };

    // Get or create user in PoolAI
    let user_manager = ctx.user_manager.clone();
    if let Err(e) = user_manager.initialize().await {
        return enterprise_err(
            "USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            "oauth2_github_callback",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    // Try to find existing user by username (GitHub login)
    let poolai_user = user_manager
        .get_user_by_username(&user_info.username)
        .await
        .unwrap_or(None);

    let (username, role) = if let Some(user) = poolai_user {
        // User exists, use existing role
        (user.username.clone(), user.role)
    } else {
        // Create new user with Viewer role by default
        // In production, you might want to map roles based on GitHub organization membership
        match user_manager
            .create_user(
                user_info.username.clone(),
                format!("oauth2_github_{}", user_info.id), // Dummy password (won't be used for OAuth2 users)
                crate::network::auth::UserRole::Viewer,
            )
            .await
        {
            Ok(new_user) => (new_user.username, crate::network::auth::UserRole::Viewer),
            Err(e) => {
                return enterprise_err(
                    "OAUTH2_USER_CREATE_FAILED",
                    format!("Failed to create user: {}", e),
                    "oauth2_github_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        }
    };

    // Generate PoolAI JWT token
    let poolai_token = match crate::network::auth::generate_token(&username, role.clone()) {
        Ok(token) => token,
        Err(e) => {
            return enterprise_err(
                "TOKEN_GENERATION_FAILED",
                format!("Failed to generate token: {}", e),
                "oauth2_github_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    // Redirect to UI with token in query parameters
    // The UI JavaScript will extract the token and store it
    let expires_in = token_response.expires_in.unwrap_or(3600);
    let redirect_url = format!(
        "/ui/auth?token={}&username={}&role={:?}&expires_in={}",
        urlencoding::encode(&poolai_token),
        urlencoding::encode(&username),
        role,
        expires_in
    );

    Redirect::temporary(&redirect_url).into_response()
}

#[cfg(feature = "enterprise")]
async fn oauth2_google_auth_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let state = uuid::Uuid::new_v4().to_string();

    match EnterpriseService::start_oauth2_authorization(&ctx, "google", &state).await {
        Ok(auth_url) => Redirect::temporary(&auth_url).into_response(),
        Err(EnterpriseOAuthStartError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "oauth2_google_auth",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ListProviders(e)) => enterprise_err(
            "OAUTH2_LIST_PROVIDERS_FAILED",
            format!("Failed to list OAuth2 providers: {}", e),
            "oauth2_google_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ProviderNotConfigured) => enterprise_err(
            "OAUTH2_PROVIDER_NOT_CONFIGURED",
            "Google OAuth2 provider not configured. Register it in the admin panel.",
            "oauth2_google_auth",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::AuthUrl(e)) => enterprise_err(
            "OAUTH2_AUTH_URL_FAILED",
            format!("Failed to generate authorization URL: {}", e),
            "oauth2_google_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
async fn oauth2_google_callback_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let code = params.get("code").cloned().unwrap_or_default();
    let _state = params.get("state").cloned().unwrap_or_default();
    let error = params.get("error").cloned();

    if let Some(error) = error {
        let (s, j) = api_json_error(
            "OAUTH2_PROVIDER_ERROR",
            format!("OAuth2 error: {}", error),
            Some(ErrorContext::new("oauth2_google_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    if code.is_empty() {
        let (s, j) = api_json_error(
            "OAUTH2_MISSING_CODE",
            "Missing authorization code",
            Some(ErrorContext::new("oauth2_google_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    let token_response = match EnterpriseService::exchange_oauth2_code(&ctx, "google", &code).await
    {
        Ok(token) => token,
        Err(EnterpriseSecurityError::Init(e)) => {
            return enterprise_err(
                "SECURITY_MANAGER_UNAVAILABLE",
                format!("Security manager not initialized: {}", e),
                "oauth2_google_callback",
                StatusCode::SERVICE_UNAVAILABLE,
            )
            .into_response();
        }
        Err(EnterpriseSecurityError::Operation(e)) => {
            return enterprise_err(
                "OAUTH2_CODE_EXCHANGE_FAILED",
                format!("Failed to exchange authorization code: {}", e),
                "oauth2_google_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    let user_info =
        match EnterpriseService::get_oauth2_user_info(&ctx, "google", &token_response.access_token)
            .await
        {
            Ok(info) => info,
            Err(EnterpriseSecurityError::Init(e)) => {
                return enterprise_err(
                    "SECURITY_MANAGER_UNAVAILABLE",
                    format!("Security manager not initialized: {}", e),
                    "oauth2_google_callback",
                    StatusCode::SERVICE_UNAVAILABLE,
                )
                .into_response();
            }
            Err(EnterpriseSecurityError::Operation(e)) => {
                return enterprise_err(
                    "OAUTH2_USERINFO_FAILED",
                    format!("Failed to get user info from Google: {}", e),
                    "oauth2_google_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        };

    // Get or create user in PoolAI
    let user_manager = ctx.user_manager.clone();
    if let Err(e) = user_manager.initialize().await {
        return enterprise_err(
            "USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            "oauth2_google_callback",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    // Try to find existing user by email or username
    let poolai_user = user_manager
        .get_user_by_username(&user_info.username)
        .await
        .unwrap_or(None);

    let (username, role) = if let Some(user) = poolai_user {
        (user.username.clone(), user.role)
    } else {
        // Create new user with Viewer role by default
        match user_manager
            .create_user(
                user_info.username.clone(),
                format!("oauth2_google_{}", user_info.id),
                crate::network::auth::UserRole::Viewer,
            )
            .await
        {
            Ok(new_user) => (new_user.username, crate::network::auth::UserRole::Viewer),
            Err(e) => {
                return enterprise_err(
                    "OAUTH2_USER_CREATE_FAILED",
                    format!("Failed to create user: {}", e),
                    "oauth2_google_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        }
    };

    // Generate PoolAI JWT token
    let poolai_token = match crate::network::auth::generate_token(&username, role.clone()) {
        Ok(token) => token,
        Err(e) => {
            return enterprise_err(
                "TOKEN_GENERATION_FAILED",
                format!("Failed to generate token: {}", e),
                "oauth2_google_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    // Redirect to UI with token
    let expires_in = token_response.expires_in.unwrap_or(3600);
    let redirect_url = format!(
        "/ui/auth?token={}&username={}&role={:?}&expires_in={}",
        urlencoding::encode(&poolai_token),
        urlencoding::encode(&username),
        role,
        expires_in
    );

    Redirect::temporary(&redirect_url).into_response()
}

#[cfg(feature = "enterprise")]
async fn oauth2_telegram_auth_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match EnterpriseService::get_telegram_oauth_widget_info(&ctx).await {
        Ok(info) => Json(serde_json::json!({
            "bot_name": info.client_id,
            "widget_url": "https://oauth.telegram.org/auth",
            "redirect_uri": info.redirect_uri,
            "message": "Use Telegram Login Widget on the client side. This endpoint provides configuration."
        }))
        .into_response(),
        Err(EnterpriseOAuthStartError::Init(e)) => {
            enterprise_err(
                "SECURITY_MANAGER_UNAVAILABLE",
                format!("Security manager not initialized: {}", e),
                "oauth2_telegram_auth",
                StatusCode::SERVICE_UNAVAILABLE,
            )
            .into_response()
        }
        Err(EnterpriseOAuthStartError::ListProviders(e)) => {
            enterprise_err(
                "OAUTH2_LIST_PROVIDERS_FAILED",
                format!("Failed to list OAuth2 providers: {}", e),
                "oauth2_telegram_auth",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
        }
        Err(EnterpriseOAuthStartError::ProviderNotConfigured) => {
            enterprise_err(
                "OAUTH2_PROVIDER_NOT_CONFIGURED",
                "Telegram OAuth2 provider not configured. Register it in the admin panel.",
                "oauth2_telegram_auth",
                StatusCode::NOT_FOUND,
            )
            .into_response()
        }
        Err(EnterpriseOAuthStartError::AuthUrl(e)) => {
            enterprise_err(
                "OAUTH2_TELEGRAM_CONFIG_FAILED",
                format!("Telegram OAuth configuration error: {}", e),
                "oauth2_telegram_auth",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response()
        }
    }
}

#[cfg(feature = "enterprise")]
async fn oauth2_telegram_callback_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // Telegram Login Widget sends auth data via hash in URL
    // This needs to be handled client-side, then sent to this endpoint
    let auth_data = params.get("auth_data").cloned().unwrap_or_default();
    let hash = params.get("hash").cloned().unwrap_or_default();
    let error = params.get("error").cloned();

    if let Some(error) = error {
        let (s, j) = api_json_error(
            "TELEGRAM_AUTH_ERROR",
            format!("Telegram authentication error: {}", error),
            Some(ErrorContext::new("oauth2_telegram_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    if auth_data.is_empty() || hash.is_empty() {
        let (s, j) = api_json_error(
            "TELEGRAM_MISSING_AUTH_DATA",
            "Missing authentication data from Telegram",
            Some(ErrorContext::new("oauth2_telegram_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    // Parse auth_data (it's typically URL-encoded JSON)
    // In a production environment, you should verify the hash using Telegram's bot token
    let user_data: Result<serde_json::Value, _> = serde_json::from_str(&auth_data);

    if let Err(_) = user_data {
        let (s, j) = api_json_error(
            "TELEGRAM_INVALID_AUTH_FORMAT",
            "Invalid authentication data format from Telegram",
            Some(ErrorContext::new("oauth2_telegram_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    let user_data = user_data.unwrap();
    let telegram_id = user_data
        .get("id")
        .and_then(|v| v.as_u64())
        .map(|v| v.to_string())
        .unwrap_or_default();
    let username = user_data
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("telegram_user")
        .to_string();
    let _first_name = user_data
        .get("first_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if telegram_id.is_empty() {
        let (s, j) = api_json_error(
            "TELEGRAM_MISSING_USER_ID",
            "Missing user ID in Telegram authentication data",
            Some(ErrorContext::new("oauth2_telegram_callback")),
            StatusCode::BAD_REQUEST,
        );
        return (s, j).into_response();
    }

    // Get or create user in PoolAI
    let user_manager = ctx.user_manager.clone();
    if let Err(e) = user_manager.initialize().await {
        return enterprise_err(
            "USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            "oauth2_telegram_callback",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    // Try to find existing user by username
    let poolai_user = user_manager
        .get_user_by_username(&username)
        .await
        .unwrap_or(None);

    let (final_username, role) = if let Some(user) = poolai_user {
        (user.username.clone(), user.role)
    } else {
        // Create new user with Viewer role by default
        match user_manager
            .create_user(
                username.clone(),
                format!("oauth2_telegram_{}", telegram_id),
                crate::network::auth::UserRole::Viewer,
            )
            .await
        {
            Ok(new_user) => (new_user.username, crate::network::auth::UserRole::Viewer),
            Err(e) => {
                return enterprise_err(
                    "OAUTH2_USER_CREATE_FAILED",
                    format!("Failed to create user: {}", e),
                    "oauth2_telegram_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        }
    };

    // Generate PoolAI JWT token
    let poolai_token = match crate::network::auth::generate_token(&final_username, role.clone()) {
        Ok(token) => token,
        Err(e) => {
            return enterprise_err(
                "TOKEN_GENERATION_FAILED",
                format!("Failed to generate token: {}", e),
                "oauth2_telegram_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    // Redirect to UI with token
    let redirect_url = format!(
        "/ui/auth?token={}&username={}&role={:?}&expires_in=3600",
        urlencoding::encode(&poolai_token),
        urlencoding::encode(&final_username),
        role
    );

    Redirect::temporary(&redirect_url).into_response()
}

// ============================================================================
// SAML SSO Authentication Handlers
// ============================================================================

#[cfg(feature = "enterprise")]
async fn saml_auth_handler(
    State(ctx): State<ApiContext>,
    Path(provider): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_saml_sso_redirect_url(&ctx, &provider).await {
        Ok(sso_url) => Redirect::temporary(&sso_url).into_response(),
        Err(EnterpriseSecurityError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "saml_auth",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseSecurityError::Operation(e)) => enterprise_err(
            "SAML_SSO_URL_FAILED",
            format!("Failed to generate SAML SSO URL: {}", e),
            "saml_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[cfg(feature = "enterprise")]
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SamlCallbackForm {
    SAMLResponse: String,
    RelayState: Option<String>,
}

#[cfg(feature = "enterprise")]
async fn saml_callback_handler(
    State(ctx): State<ApiContext>,
    Path(provider): Path<String>,
    Form(form): Form<SamlCallbackForm>,
) -> impl IntoResponse {
    let attributes = match EnterpriseService::validate_saml_assertion_response(
        &ctx,
        &provider,
        &form.SAMLResponse,
    )
    .await
    {
        Ok(attrs) => attrs,
        Err(EnterpriseSecurityError::Init(e)) => {
            return enterprise_err(
                "SECURITY_MANAGER_UNAVAILABLE",
                format!("Security manager not initialized: {}", e),
                "saml_callback",
                StatusCode::SERVICE_UNAVAILABLE,
            )
            .into_response();
        }
        Err(EnterpriseSecurityError::Operation(e)) => {
            return enterprise_err(
                "SAML_ASSERTION_INVALID",
                format!("Failed to validate SAML assertion: {}", e),
                "saml_callback",
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }
    };

    // Extract user information from SAML attributes
    // Map SAML attributes to user fields (nameid, email, etc.)
    let nameid = attributes
        .get("nameid")
        .or_else(|| attributes.get("NameID"))
        .cloned()
        .unwrap_or_else(|| "saml_user".to_string());
    let _email = attributes
        .get("email")
        .or_else(|| attributes.get("Email"))
        .or_else(|| attributes.get("mail"))
        .cloned();
    let username = attributes
        .get("username")
        .or_else(|| attributes.get("Username"))
        .or_else(|| attributes.get("uid"))
        .cloned()
        .unwrap_or_else(|| nameid.clone());

    // Get or create user in PoolAI
    let user_manager = ctx.user_manager.clone();
    if let Err(e) = user_manager.initialize().await {
        return enterprise_err(
            "USER_MANAGER_INIT_FAILED",
            format!("User manager initialization failed: {}", e),
            "saml_callback",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    // Try to find existing user by username
    let poolai_user = user_manager
        .get_user_by_username(&username)
        .await
        .unwrap_or(None);

    let (final_username, role) = if let Some(user) = poolai_user {
        (user.username.clone(), user.role)
    } else {
        // Create new user with Viewer role by default
        match user_manager
            .create_user(
                username.clone(),
                format!("saml_{}_{}", provider, nameid),
                crate::network::auth::UserRole::Viewer,
            )
            .await
        {
            Ok(new_user) => (new_user.username, crate::network::auth::UserRole::Viewer),
            Err(e) => {
                return enterprise_err(
                    "SAML_USER_CREATE_FAILED",
                    format!("Failed to create user from SAML attributes: {}", e),
                    "saml_callback",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            }
        }
    };

    // Generate PoolAI JWT token
    let poolai_token = match crate::network::auth::generate_token(&final_username, role.clone()) {
        Ok(token) => token,
        Err(e) => {
            return enterprise_err(
                "TOKEN_GENERATION_FAILED",
                format!("Failed to generate token after SAML authentication: {}", e),
                "saml_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    // Redirect to UI with token
    // Use RelayState if provided, otherwise default to /ui/auth
    let redirect_path = form.RelayState.unwrap_or_else(|| "/ui/auth".to_string());
    let redirect_url = format!(
        "{}?token={}&username={}&role={:?}",
        redirect_path,
        urlencoding::encode(&poolai_token),
        urlencoding::encode(&final_username),
        role
    );

    Redirect::temporary(&redirect_url).into_response()
}

// ============================================================================
// Non-enterprise stub
// ============================================================================

#[cfg(not(feature = "enterprise"))]
pub fn create_enterprise_api_routes() -> axum::Router<crate::core::state::ApiContext> {
    axum::Router::new()
}
