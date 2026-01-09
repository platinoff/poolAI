//! Common types and utilities for API handlers

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
