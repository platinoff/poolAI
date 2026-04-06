//! Common types and utilities for API handlers

pub use crate::network::json_errors::{
    api_error_response, api_json_error, http_status_for_app_error, HttpAppError,
};

use crate::core::error::{AppError, ErrorContext};
use crate::network::auth::Claims;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

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
        let err = AppError::Forbidden(format!(
            "Missing required permission '{}'",
            required_permission
        ));
        let ctx = ErrorContext::new("check_permission")
            .with_resource("permission", required_permission)
            .with_details(format!("user_permissions={:?}", claims.permissions))
            .with_hint("Ensure the user has a role that grants this permission.");
        return Err(api_error_response(&err, Some(ctx), None));
    }
    Ok(())
}
