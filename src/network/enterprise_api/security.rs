//! Enterprise API: security CRUD (OAuth2/SAML providers, policies).

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::enterprise;
use crate::network::api::check_permission;
use crate::network::auth::Claims;
use crate::services::enterprise_service::{EnterpriseSecurityError, EnterpriseService};
use axum::extract::{Extension, Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use super::{enterprise_err, enterprise_json_err};

/// GET /security/sso/store — band-62 wire snapshot over HTTP (PH-S1272).
pub(super) async fn sso_store_wire_handler() -> impl IntoResponse {
    Json(enterprise::security::sso_store_wire())
}

#[derive(Deserialize)]
pub(super) struct OAuth2ProviderRegisterRequest {
    name: String,
    config: enterprise::security::OAuth2Config,
    enabled: Option<bool>,
}

pub(super) async fn security_oauth2_providers_handler(
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
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

pub(super) async fn security_oauth2_provider_register_handler(
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

#[derive(Deserialize)]
pub(super) struct SamlProviderRegisterRequest {
    name: String,
    config: enterprise::security::SamlConfig,
    enabled: Option<bool>,
}

pub(super) async fn security_saml_providers_handler(
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
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

pub(super) async fn security_saml_provider_register_handler(
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

#[derive(Deserialize)]
pub(super) struct SecurityPolicyCreateRequest {
    #[allow(dead_code)] // Policy name is included in the policy struct itself
    name: String,
    policy: enterprise::security::SecurityPolicy,
}

pub(super) async fn security_policies_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
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

pub(super) async fn security_policy_create_handler(
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

pub(super) async fn security_oauth2_provider_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_oauth2_provider(&ctx, &name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => enterprise_json_err(
            "OAUTH2_PROVIDER_NOT_FOUND",
            format!("OAuth2 provider not found: {}", name),
            ErrorContext::new("security_oauth2_provider_get").with_resource("name", &name),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
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

#[derive(Deserialize)]
pub(super) struct OAuth2ProviderUpdateRequest {
    config: Option<enterprise::security::OAuth2Config>,
    enabled: Option<bool>,
}

pub(super) async fn security_oauth2_provider_update_handler(
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

pub(super) async fn security_oauth2_provider_delete_handler(
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

pub(super) async fn security_saml_provider_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_saml_provider(&ctx, &name).await {
        Ok(Some(provider)) => Json(provider).into_response(),
        Ok(None) => enterprise_json_err(
            "SAML_PROVIDER_NOT_FOUND",
            format!("SAML provider not found: {}", name),
            ErrorContext::new("security_saml_provider_get").with_resource("name", &name),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
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

#[derive(Deserialize)]
pub(super) struct SamlProviderUpdateRequest {
    config: Option<enterprise::security::SamlConfig>,
    enabled: Option<bool>,
}

pub(super) async fn security_saml_provider_update_handler(
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

pub(super) async fn security_saml_provider_delete_handler(
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

pub(super) async fn security_policy_get_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match EnterpriseService::get_security_policy(&ctx, &name).await {
        Ok(Some(policy)) => Json(policy).into_response(),
        Ok(None) => enterprise_json_err(
            "SECURITY_POLICY_NOT_FOUND",
            format!("Security policy not found: {}", name),
            ErrorContext::new("security_policy_get").with_resource("name", &name),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
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

pub(super) async fn security_policy_update_handler(
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
        return enterprise_json_err(
            "POLICY_NAME_MISMATCH",
            format!(
                "Policy name in body '{}' does not match path parameter '{}'",
                policy.name, name
            ),
            ErrorContext::new("security_policy_update"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
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

pub(super) async fn security_policy_delete_handler(
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
