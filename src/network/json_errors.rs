//! Structured JSON error responses shared across HTTP layers.
//!
//! Lives beside `auth` and `api::common` so [`crate::network::auth`] can use
//! [`api_json_error`] without importing `api::common` (which depends on `auth` for [`Claims`]).
//!
//! [`AppError`] and [`HttpAppError`] implement [`axum::response::IntoResponse`] for handlers that
//! return `Result<T, _>` with the same JSON shape as [`api_error_response`].

use crate::core::error::{AppError, ErrorContext};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::Value;
use std::io::ErrorKind;
use tracing::{error, warn};

/// HTTP status for [`AppError::ResourceError`]: many call sites encode missing entities, but some
/// mean conflict, capacity, or an operation failure on an existing resource.
fn http_status_for_resource_error(message: &str) -> StatusCode {
    let m = message.to_ascii_lowercase();

    if m.contains("already exists") || m.contains("conflict") || m.contains("duplicate") {
        return StatusCode::CONFLICT;
    }
    if m.contains("quota")
        || m.contains("exhausted")
        || m.contains("limit exceeded")
        || m.contains("capacity")
    {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    if m.contains("failed to kill") || m.contains("cannot terminate") {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if m.contains("not found") || m.contains("does not exist") || m.contains("no such") {
        return StatusCode::NOT_FOUND;
    }

    StatusCode::NOT_FOUND
}

/// Default HTTP status for an [`AppError`] when no override is needed.
///
/// Mapping is conservative: variants that usually indicate client mistakes use 4xx;
/// I/O and unknown failures use 500.
pub fn http_status_for_app_error(err: &AppError) -> StatusCode {
    use AppError::*;
    match err {
        ValidationError(_) | SerializationError(_) | ModelError(_) => StatusCode::BAD_REQUEST,
        Forbidden(_) => StatusCode::FORBIDDEN,
        ApiNotFound(_) => StatusCode::NOT_FOUND,
        SubsystemUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        RestError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        TimeoutError(_) => StatusCode::GATEWAY_TIMEOUT,
        NetworkError(_) => StatusCode::BAD_GATEWAY,
        PoolError(_) | MonitoringError(_) | GpuError(_) | MemoryError(_) | ShutdownError(_) => {
            StatusCode::SERVICE_UNAVAILABLE
        }
        ResourceError(msg) => http_status_for_resource_error(msg),
        ConfigError(_) | InitializationError(_) | Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        IoError(e) => match e.kind() {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
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

/// JSON error body matching [`api_error_response`], with an arbitrary machine-readable `code`.
///
/// Use when the failure is not represented as an [`AppError`] (e.g. subsystem unavailable strings).
pub fn api_json_error(
    code: impl AsRef<str>,
    message: impl Into<String>,
    ctx: Option<ErrorContext>,
    status: StatusCode,
) -> (StatusCode, Json<Value>) {
    let message = message.into();
    let code_ref = code.as_ref();
    if status.is_server_error() {
        error!(
            code = %code_ref,
            message = %message,
            ?ctx,
            "API error response (structured)"
        );
    } else {
        warn!(
            code = %code_ref,
            message = %message,
            ?ctx,
            "API client error response (structured)"
        );
    }

    let mut root = serde_json::json!({
        "error": {
            "code": code_ref,
            "message": message,
        }
    });
    if let Some(c) = ctx {
        if let Ok(v) = serde_json::to_value(&c) {
            root["context"] = v;
        }
    }
    (status, Json(root))
}

/// [`AppError`] plus optional [`ErrorContext`] and HTTP status override for Axum [`IntoResponse`].
///
/// Use when a handler returns `Result<T, HttpAppError>` and needs the same JSON shape as
/// [`api_error_response`] (including `context`), without building `(StatusCode, Json<Value>)` by hand.
#[derive(Debug)]
pub struct HttpAppError {
    pub err: AppError,
    pub context: Option<ErrorContext>,
    pub status_override: Option<StatusCode>,
}

impl HttpAppError {
    pub fn new(err: AppError) -> Self {
        Self {
            err,
            context: None,
            status_override: None,
        }
    }

    pub fn with_context(mut self, ctx: ErrorContext) -> Self {
        self.context = Some(ctx);
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_override = Some(status);
        self
    }
}

impl From<AppError> for HttpAppError {
    fn from(err: AppError) -> Self {
        Self::new(err)
    }
}

impl IntoResponse for HttpAppError {
    fn into_response(self) -> Response {
        let (status, body) = api_error_response(&self.err, self.context, self.status_override);
        (status, body).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = api_error_response(&self, None, None);
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn http_status_validation_is_bad_request() {
        let e = AppError::ValidationError("bad".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn http_status_io_is_internal() {
        use std::io;
        let e = AppError::from(io::Error::other("x"));
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn http_status_resource_is_not_found() {
        let e = AppError::ResourceError("missing".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::NOT_FOUND);
    }

    #[test]
    fn http_status_resource_not_found_phrase() {
        let e = AppError::ResourceError("Worker 'w1' not found".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::NOT_FOUND);
    }

    #[test]
    fn http_status_resource_kill_failure_is_internal() {
        let e = AppError::ResourceError("Failed to kill process. Context: Cannot terminate".into());
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn http_status_resource_conflict() {
        let e = AppError::ResourceError("Volume already exists".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::CONFLICT);
    }

    #[test]
    fn http_status_io_not_found_is_404() {
        use std::io;
        let e = AppError::from(io::Error::new(io::ErrorKind::NotFound, "artifact.bin"));
        assert_eq!(http_status_for_app_error(&e), StatusCode::NOT_FOUND);
    }

    #[test]
    fn http_status_io_permission_denied_is_403() {
        use std::io;
        let e = AppError::from(io::Error::new(io::ErrorKind::PermissionDenied, "denied"));
        assert_eq!(http_status_for_app_error(&e), StatusCode::FORBIDDEN);
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

    #[test]
    fn http_status_forbidden() {
        let e = AppError::Forbidden("no".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::FORBIDDEN);
    }

    #[test]
    fn http_status_api_not_found() {
        let e = AppError::ApiNotFound("missing".into());
        assert_eq!(http_status_for_app_error(&e), StatusCode::NOT_FOUND);
    }

    #[test]
    fn http_status_subsystem_unavailable() {
        let e = AppError::SubsystemUnavailable("down".into());
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn http_status_internal_error() {
        let e = AppError::InternalError("boom".into());
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn http_status_rest_error_defaults_to_internal() {
        let e = AppError::RestError {
            code: "internal_error",
            message: "x".into(),
        };
        assert_eq!(
            http_status_for_app_error(&e),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn api_error_response_rest_error_custom_code() {
        let e = AppError::RestError {
            code: "internal_error",
            message: "boom".into(),
        };
        let (st, Json(v)) = api_error_response(&e, None, None);
        assert_eq!(st, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(v["error"]["code"], "internal_error");
        assert_eq!(v["error"]["message"], "boom");
    }

    #[test]
    fn http_app_error_rest_error_unauthorized_override() {
        let e = AppError::RestError {
            code: "AUTH_MISSING_HEADER",
            message: "Missing".into(),
        };
        let resp = HttpAppError::new(e)
            .with_status(StatusCode::UNAUTHORIZED)
            .into_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn api_json_error_shape() {
        let (st, Json(v)) = api_json_error(
            "NOT_FOUND",
            "missing",
            Some(ErrorContext::new("op")),
            StatusCode::NOT_FOUND,
        );
        assert_eq!(st, StatusCode::NOT_FOUND);
        assert_eq!(v["error"]["code"], "NOT_FOUND");
        assert_eq!(v["error"]["message"], "missing");
        assert!(v.get("context").is_some());
    }

    #[test]
    fn app_error_into_response_status_and_json_shape() {
        let resp = AppError::Forbidden("x".into()).into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn http_app_error_into_response_includes_context() {
        let e = HttpAppError::new(AppError::ConfigError("c".into()))
            .with_context(ErrorContext::new("save").with_hint("fix"));
        let resp = e.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn result_ok_json_err_app_error_into_response() {
        use serde_json::json;
        let ok: Result<Json<Value>, AppError> = Ok(Json(json!({"k": 1})));
        assert_eq!(ok.into_response().status(), StatusCode::OK);
        let err: Result<Json<Value>, AppError> = Err(AppError::ValidationError("bad".into()));
        assert_eq!(err.into_response().status(), StatusCode::BAD_REQUEST);
    }
}
