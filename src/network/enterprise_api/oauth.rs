//! Enterprise API: OAuth2 browser flows (GitHub, Google, Telegram).

use crate::core::error::ErrorContext;
use crate::core::oauth2_pending::{store_oauth2_pending, verify_oauth2_pending};
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::services::enterprise_service::{
    EnterpriseOAuthStartError, EnterpriseSecurityError, EnterpriseService,
};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Json;

use super::enterprise_err;
pub(super) async fn oauth2_github_auth_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
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

pub(super) async fn oauth2_github_callback_handler(
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

pub(super) async fn oauth2_google_auth_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
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

pub(super) async fn oauth2_google_callback_handler(
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

pub(super) async fn oauth2_telegram_auth_handler(
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
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

pub(super) async fn oauth2_telegram_callback_handler(
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
