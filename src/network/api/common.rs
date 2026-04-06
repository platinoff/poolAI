//! Common types and utilities for API handlers

use crate::core::error::{AppError, ErrorContext};
use crate::network::auth::Claims;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use tracing::{error, warn};

/// Default HTTP status for an [`AppError`] when no override is needed.
///
/// Mapping is conservative: variants that usually indicate client mistakes use 4xx;
/// I/O and unknown failures use 500.
pub fn http_status_for_app_error(err: &AppError) -> StatusCode {
    use AppError::*;
    match err {
        ValidationError(_) | SerializationError(_) | ModelError(_) => StatusCode::BAD_REQUEST,
        TimeoutError(_) => StatusCode::GATEWAY_TIMEOUT,
        NetworkError(_) => StatusCode::BAD_GATEWAY,
        PoolError(_) | MonitoringError(_) | GpuError(_) | MemoryError(_) | ShutdownError(_) => {
            StatusCode::SERVICE_UNAVAILABLE
        }
        ResourceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ConfigError(_) | InitializationError(_) | IoError(_) | Unknown => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Build a consistent JSON error body for REST handlers.
///
/// Shape: `{ "error": { "code", "message" }, "context"?: ... }`.
/// Logs the full error (and optional context) for operations; use `status_override` to keep
/// legacy status codes where [`AppError`] variants are overloaded (e.g. not-found as `ModelError`).
pub fn api_error_response(
    err: &AppError,
    ctx: Option<ErrorContext>,
    status_override: Option<StatusCode>,
) -> (StatusCode, Json<Value>) {
    let status = status_override.unwrap_or_else(|| http_status_for_app_error(err));
    if status.is_server_error() {
        error!(
            error = %err,
            code = err.error_code(),
            ?ctx,
            "API error response"
        );
    } else {
        warn!(
            error = %err,
            code = err.error_code(),
            ?ctx,
            "API client error response"
        );
    }

    let mut root = serde_json::json!({
        "error": {
            "code": err.error_code(),
            "message": err.to_string(),
        }
    });
    if let Some(c) = ctx {
        if let Ok(v) = serde_json::to_value(&c) {
            root["context"] = v;
        }
    }
    (status, Json(root))
}

/// Helper function to check RBAC permissions
///
/// # Errors
///
/// Returns an error tuple `(StatusCode, Json<Value>)` if permission is denied.
pub fn check_permission(
    claims: &Claims,
    required_permission: &str,
) -> Result<(), (StatusCode, Json<Value>)> {
    if !claims
        .permissions
        .contains(&required_permission.to_string())
    {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": format!("Permission denied. Context: User does not have required permission '{}'. Suggestion: Ensure user has appropriate roles and permissions.", required_permission),
                "required": required_permission,
                "user_permissions": claims.permissions
            })),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_status_validation_is_bad_request() {
        let e = AppError::ValidationError("bad".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn http_status_io_is_internal() {
        use std::io;
        let e = AppError::from(io::Error::new(io::ErrorKind::Other, "x"));
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn api_error_response_shape_without_context() {
        let e = AppError::ModelError("x".into());
        let (st, Json(v)) = api_error_response(&e, None, None);
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert_eq!(v["error"]["code"], "MODEL_ERROR");
        assert!(v["error"]["message"].is_string());
        assert!(v.get("context").is_none());
    }

    #[test]
    fn api_error_response_includes_context_and_hint() {
        let e = AppError::ConfigError("c".into());
        let ctx = ErrorContext::new("op").with_hint("fix config");
        let (_st, Json(v)) = api_error_response(&e, Some(ctx), None);
        assert!(v.get("context").is_some());
        assert_eq!(v["context"]["operation"], "op");
        assert_eq!(v["context"]["hint"], "fix config");
    }

    #[test]
    fn status_override_respected() {
        let e = AppError::ModelError("x".into());
        let (st, _) = api_error_response(&e, None, Some(StatusCode::NOT_FOUND));
        assert_eq!(st, StatusCode::NOT_FOUND);
    }
}
