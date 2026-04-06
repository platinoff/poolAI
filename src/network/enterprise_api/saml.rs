//! Enterprise API: SAML SSO browser flows.

use crate::core::state::ApiContext;
use crate::services::enterprise_service::{EnterpriseSecurityError, EnterpriseService};
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;

use super::enterprise_err;
pub(super) async fn saml_auth_handler(
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

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub(super) struct SamlCallbackForm {
    SAMLResponse: String,
    RelayState: Option<String>,
}

pub(super) async fn saml_callback_handler(
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
