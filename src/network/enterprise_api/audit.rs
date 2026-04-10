//! Enterprise API: audit log query.

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::services::enterprise_service::{
    AuditEventsQuery, EnterpriseAuditError, EnterpriseService,
};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::enterprise_json_err;
use serde::Deserialize;
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
