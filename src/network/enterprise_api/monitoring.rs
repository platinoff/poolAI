//! Enterprise API: monitoring (alerts, dashboards, metrics, alert rules).

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::enterprise;
use crate::enterprise::monitoring::{validate_monitoring_alert_fields, AlertRule, AlertSeverity};
use crate::network::api::check_permission;
use crate::network::auth::Claims;
use crate::services::enterprise_service::{
    DashboardCreateInput, EnterpriseMonitoringError, EnterpriseService, MetricHistoryQueryInput,
    MonitoringAlertsQueryInput,
};
use axum::extract::{Extension, Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use super::enterprise_json_err;

/// GET /monitoring/store — band-92 wire snapshot over HTTP (PH-S1571).
pub(super) async fn monitoring_store_wire_handler() -> impl IntoResponse {
    Json(enterprise::monitoring::monitoring_store_wire())
}

#[derive(Deserialize)]
pub(super) struct MonitoringAlertsQuery {
    severity: Option<String>,
    tenant_id: Option<String>,
    acknowledged: Option<bool>,
    /// Soft limit for contract pagination stub (PH-S1570).
    limit: Option<usize>,
}

/// Body for POST /monitoring/alert-rules/validate — field fixtures without durable write (PH-S1573).
#[derive(Deserialize)]
pub(super) struct MonitoringAlertRuleValidateRequest {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    metric: Option<String>,
    #[serde(default)]
    threshold: Option<f64>,
    #[serde(default)]
    operator: Option<String>,
    #[serde(default)]
    severity: Option<String>,
    #[serde(default)]
    enabled: Option<bool>,
}

pub(super) async fn monitoring_alerts_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<MonitoringAlertsQuery>,
) -> impl IntoResponse {
    let q = MonitoringAlertsQueryInput {
        severity: params.severity,
        tenant_id: params.tenant_id,
        acknowledged: params.acknowledged,
    };

    match EnterpriseService::list_monitoring_alerts(&ctx, q).await {
        Ok(mut alerts) => {
            if let Some(limit) = params.limit {
                if alerts.len() > limit {
                    alerts.truncate(limit);
                }
            }
            Json(alerts).into_response()
        }
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_alerts").with_hint("Check system startup sequence."),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_ALERTS_FAILED",
            format!("Failed to retrieve alerts: {}", e),
            ErrorContext::new("monitoring_alerts"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

pub(super) async fn monitoring_alert_acknowledge_handler(
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
            return enterprise_json_err(
                "INVALID_UUID",
                format!("Invalid UUID format for alert id: {}", id),
                ErrorContext::new("monitoring_alert_ack").with_resource("alert_id", &id),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
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
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_alert_ack"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_ALERT_ACK_FAILED",
            format!("Failed to acknowledge alert: {}", e),
            ErrorContext::new("monitoring_alert_ack").with_resource("alert_id", &id),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(super) struct MonitoringDashboardsQuery {
    tenant_id: Option<String>,
}

pub(super) async fn monitoring_dashboards_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<MonitoringDashboardsQuery>,
) -> impl IntoResponse {
    match EnterpriseService::list_monitoring_dashboards(&ctx, params.tenant_id).await {
        Ok(dashboards) => Json(dashboards).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_dashboards_list"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_DASHBOARDS_FAILED",
            format!("Failed to retrieve dashboards: {}", e),
            ErrorContext::new("monitoring_dashboards_list"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

#[derive(Deserialize)]
pub(super) struct DashboardCreateRequest {
    name: String,
    description: Option<String>,
    metrics: Vec<String>,
    layout: Option<String>,
    is_public: Option<bool>,
    tenant_id: Option<String>,
}

pub(super) async fn monitoring_dashboard_create_handler(
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
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_dashboard_create"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_DASHBOARD_CREATE_FAILED",
            format!("Failed to create dashboard: {}", e),
            ErrorContext::new("monitoring_dashboard_create"),
            StatusCode::BAD_REQUEST,
        )
        .into_response(),
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(super) struct MonitoringMetricsQuery {
    metric: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    tenant_id: Option<String>,
    limit: Option<usize>,
}

pub(super) async fn monitoring_metrics_handler(
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
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_metrics"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_METRICS_FAILED",
            format!("Failed to retrieve metrics: {}", e),
            ErrorContext::new("monitoring_metrics"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

pub(super) async fn monitoring_alert_rules_handler(
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
    match EnterpriseService::list_monitoring_alert_rules(&ctx).await {
        Ok(rules) => Json(rules).into_response(),
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_alert_rules_list"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_ALERT_RULES_FAILED",
            format!("Failed to retrieve alert rules: {}", e),
            ErrorContext::new("monitoring_alert_rules_list"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// POST /monitoring/alert-rules/validate — empty name/metric/operator → 400 fixtures (PH-S1573).
pub(super) async fn monitoring_alert_rule_validate_handler(
    Json(body): Json<MonitoringAlertRuleValidateRequest>,
) -> impl IntoResponse {
    let severity = match body.severity.as_deref() {
        Some("INFO") | Some("info") => AlertSeverity::Info,
        Some("ERROR") | Some("error") => AlertSeverity::Error,
        Some("CRITICAL") | Some("critical") => AlertSeverity::Critical,
        _ => AlertSeverity::Warning,
    };
    let rule = AlertRule {
        name: body.name.unwrap_or_default(),
        metric: body.metric.unwrap_or_default(),
        threshold: body.threshold.unwrap_or(0.0),
        operator: body.operator.unwrap_or_else(|| ">".to_string()),
        severity,
        enabled: body.enabled.unwrap_or(true),
        tenant_id: None,
    };

    match validate_monitoring_alert_fields(&rule) {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "valid": true,
                "message": "Monitoring alert rule fields ok"
            })),
        )
            .into_response(),
        Err(e) => {
            let msg = e.to_string();
            let code = if msg.contains("name") {
                "MONITORING_MISSING_NAME"
            } else if msg.contains("metric") {
                "MONITORING_MISSING_METRIC"
            } else if msg.contains("operator") {
                "MONITORING_INVALID_OPERATOR"
            } else {
                "MONITORING_VALIDATION_FAILED"
            };
            enterprise_json_err(
                code,
                msg,
                ErrorContext::new("monitoring_alert_rule_validate")
                    .with_hint("Set non-empty name/metric and a valid comparison operator."),
                StatusCode::BAD_REQUEST,
            )
            .into_response()
        }
    }
}

pub(super) async fn monitoring_alert_rule_create_handler(
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
        Err(EnterpriseMonitoringError::Init(e)) => enterprise_json_err(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", e),
            ErrorContext::new("monitoring_alert_rule_create"),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseMonitoringError::Operation(e)) => enterprise_json_err(
            "MONITORING_ALERT_RULE_CREATE_FAILED",
            format!("Failed to create alert rule: {}", e),
            ErrorContext::new("monitoring_alert_rule_create"),
            StatusCode::BAD_REQUEST,
        )
        .into_response(),
    }
}
