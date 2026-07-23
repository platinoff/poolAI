//! Enterprise API: audit log query + store wire + field validation fixtures.

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::enterprise;
use crate::enterprise::audit::{validate_audit_event_fields, AuditEvent, AuditLevel};
use crate::services::enterprise_service::{
    AuditEventsQuery, EnterpriseAuditError, EnterpriseService,
};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use super::enterprise_json_err;

#[derive(Deserialize)]
#[allow(dead_code)]
pub(super) struct AuditQueryParams {
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

/// GET /audit/store — band-72 wire snapshot over HTTP (PH-S1371).
pub(super) async fn audit_store_wire_handler() -> impl IntoResponse {
    Json(enterprise::audit::audit_store_wire())
}

/// Body for POST /audit/events/validate — field fixtures without durable write (PH-S1373).
#[derive(Deserialize)]
pub(super) struct AuditEventValidateRequest {
    #[serde(default)]
    action: Option<String>,
    #[serde(default)]
    resource_type: Option<String>,
    #[serde(default)]
    result: Option<String>,
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(default)]
    tenant_id: Option<String>,
    #[serde(default)]
    resource_id: Option<String>,
}

pub(super) async fn audit_events_query_handler(
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
        Err(EnterpriseAuditError::Init(e)) => enterprise_json_err(
            "AUDIT_LOGGER_UNAVAILABLE",
            format!("Audit logger not initialized: {}", e),
            ErrorContext::new("audit_events_query")
                .with_hint("Check system startup sequence and audit logger wiring."),
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseAuditError::Query(e)) => enterprise_json_err(
            "AUDIT_QUERY_FAILED",
            format!("Failed to query audit events: {}", e),
            ErrorContext::new("audit_events_query"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// POST /audit/events/validate — missing action/resource → 400 fixtures (PH-S1373).
pub(super) async fn audit_event_validate_handler(
    Json(body): Json<AuditEventValidateRequest>,
) -> impl IntoResponse {
    let level_raw = body.level.as_deref().unwrap_or("INFO").to_ascii_uppercase();
    let level = match level_raw.as_str() {
        "WARNING" => AuditLevel::Warning,
        "ERROR" => AuditLevel::Error,
        "CRITICAL" => AuditLevel::Critical,
        _ => AuditLevel::Info,
    };
    let mut event = AuditEvent::new(
        level,
        body.action.unwrap_or_default(),
        body.resource_type.unwrap_or_default(),
        body.result.unwrap_or_else(|| "success".into()),
    );
    if let Some(uid) = body.user_id {
        event = event.with_user_id(uid);
    }
    if let Some(tid) = body.tenant_id {
        event = event.with_tenant_id(tid);
    }
    if let Some(rid) = body.resource_id {
        event = event.with_resource_id(rid);
    }

    match validate_audit_event_fields(&event) {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "valid": true,
                "message": "Audit event fields ok"
            })),
        )
            .into_response(),
        Err(e) => {
            let msg = e.to_string();
            let code = if msg.contains("missing action") {
                "AUDIT_MISSING_ACTION"
            } else if msg.contains("missing resource_type") {
                "AUDIT_MISSING_RESOURCE"
            } else {
                "AUDIT_VALIDATION_FAILED"
            };
            enterprise_json_err(
                code,
                msg,
                ErrorContext::new("audit_event_validate")
                    .with_hint("Set non-empty action and resource_type."),
                StatusCode::BAD_REQUEST,
            )
            .into_response()
        }
    }
}
