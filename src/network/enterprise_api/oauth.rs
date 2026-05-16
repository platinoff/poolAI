//! Enterprise API: OAuth2 browser flows (GitHub, Google, Telegram).

use crate::core::error::{AppError, ErrorContext};
use crate::core::oauth2_pending::{store_oauth2_pending, verify_oauth2_pending};
use crate::core::state::ApiContext;
use crate::enterprise::audit::{AuditEvent, AuditLevel};
use crate::services::enterprise_service::{
    EnterpriseOAuthStartError, EnterpriseSecurityError, EnterpriseService,
};
use axum::extract::{Query, State};
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use serde::Deserialize;
use std::collections::HashMap;

use super::{enterprise_err, enterprise_json_err};

fn escape_html_attr(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        match c {
            '&' => acc.push_str("&amp;"),
            '"' => acc.push_str("&quot;"),
            '<' => acc.push_str("&lt;"),
            _ => acc.push(c),
        }
        acc
    })
}

async fn audit_telegram_event(ctx: &ApiContext, event: AuditEvent) {
    let _ = ctx.audit_logger.initialize().await;
    let _ = ctx.audit_logger.log_event(event).await;
}

#[derive(Debug, Default, Deserialize)]
pub(super) struct TelegramAuthPageQuery {
    #[serde(default)]
    lang: Option<String>,
}

fn resolve_telegram_page_lang(headers: &HeaderMap, query_lang: Option<&str>) -> &'static str {
    if let Some(raw) = query_lang {
        let l = raw.trim().to_lowercase();
        if l == "uk" || l == "ua" || l.starts_with("uk-") {
            return "uk";
        }
        if l == "en" || l.starts_with("en-") {
            return "en";
        }
    }
    if let Some(v) = headers.get(ACCEPT_LANGUAGE).and_then(|h| h.to_str().ok()) {
        let lower = v.to_lowercase();
        if lower.contains("uk") || lower.contains("ua") {
            return "uk";
        }
    }
    "en"
}

fn telegram_auth_page_copy(lang: &str) -> (&'static str, &'static str, &'static str) {
    match lang {
        "uk" => (
            "Вхід через Telegram",
            "Увійдіть через Telegram",
            "Після входу цю сторінку можна закрити.",
        ),
        _ => (
            "Telegram sign-in",
            "Sign in with Telegram",
            "You can close this page after signing in.",
        ),
    }
}
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
        return enterprise_json_err(
            "OAUTH2_PROVIDER_ERROR",
            format!("OAuth2 error: {}", error),
            ErrorContext::new("oauth2_github_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    if code.is_empty() {
        return enterprise_json_err(
            "OAUTH2_MISSING_CODE",
            "Missing authorization code",
            ErrorContext::new("oauth2_github_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    // Verify state parameter (CSRF protection)
    if state.is_empty() || !verify_oauth2_pending(&ctx.oauth2_pending_states, &state).await {
        return enterprise_json_err(
            "OAUTH2_INVALID_STATE",
            "Invalid or expired state parameter. Please try again.",
            ErrorContext::new("oauth2_github_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
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
        return enterprise_json_err(
            "OAUTH2_PROVIDER_ERROR",
            format!("OAuth2 error: {}", error),
            ErrorContext::new("oauth2_google_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    if code.is_empty() {
        return enterprise_json_err(
            "OAUTH2_MISSING_CODE",
            "Missing authorization code",
            ErrorContext::new("oauth2_google_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
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
    headers: HeaderMap,
    Query(page): Query<TelegramAuthPageQuery>,
) -> impl IntoResponse {
    match EnterpriseService::get_telegram_oauth_widget_info(&ctx).await {
        Ok(info) => {
            let login = escape_html_attr(info.client_id.trim());
            let auth_url = escape_html_attr(info.redirect_uri.trim());
            let lang = resolve_telegram_page_lang(&headers, page.lang.as_deref());
            let (title, lead, footer) = telegram_auth_page_copy(lang);
            let html_lang = if lang == "uk" { "uk" } else { "en" };
            Html(format!(
                r#"<!DOCTYPE html>
<html lang="{html_lang}">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
</head>
<body style="font-family:system-ui,sans-serif;text-align:center;padding:2rem;background:#0f1216;color:#e8e8e8;">
  <p style="color:#a8b0bf;">{lead}</p>
  <script async src="https://telegram.org/js/telegram-widget.js?22"
    data-telegram-login="{login}"
    data-size="large"
    data-auth-url="{auth_url}"
    data-request-access="write"></script>
  <p style="margin-top:1.5rem;font-size:0.9em;color:#a8b0bf;">{footer}</p>
</body>
</html>"#
            ))
            .into_response()
        }
        Err(EnterpriseOAuthStartError::Init(e)) => enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "oauth2_telegram_auth",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ListProviders(e)) => enterprise_err(
            "OAUTH2_LIST_PROVIDERS_FAILED",
            format!("Failed to list OAuth2 providers: {}", e),
            "oauth2_telegram_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::ProviderNotConfigured) => enterprise_err(
            "OAUTH2_PROVIDER_NOT_CONFIGURED",
            "Telegram OAuth2 provider not configured. Register it in the admin panel.",
            "oauth2_telegram_auth",
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(EnterpriseOAuthStartError::AuthUrl(e)) => enterprise_err(
            "OAUTH2_TELEGRAM_CONFIG_FAILED",
            format!("Telegram OAuth configuration error: {}", e),
            "oauth2_telegram_auth",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

pub(super) async fn oauth2_telegram_callback_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let tg_id_hint = params.get("id").cloned().unwrap_or_default();

    if let Some(error) = params.get("error").cloned() {
        audit_telegram_event(
            &ctx,
            AuditEvent::new(
                AuditLevel::Warning,
                "oauth_telegram_login".to_string(),
                "authentication".to_string(),
                "provider_error".to_string(),
            )
            .with_metadata("telegram_id".to_string(), tg_id_hint.clone())
            .with_metadata("detail".to_string(), error.clone()),
        )
        .await;
        return enterprise_json_err(
            "TELEGRAM_AUTH_ERROR",
            format!("Telegram authentication error: {}", error),
            ErrorContext::new("oauth2_telegram_callback"),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    if let Err(e) = ctx.security_manager.initialize().await {
        return enterprise_err(
            "SECURITY_MANAGER_UNAVAILABLE",
            format!("Security manager not initialized: {}", e),
            "oauth2_telegram_callback",
            StatusCode::SERVICE_UNAVAILABLE,
        )
        .into_response();
    }

    let verify = ctx
        .security_manager
        .verify_telegram_oauth_callback(&params)
        .await;
    let (telegram_id, username) = match verify {
        Ok(pair) => pair,
        Err(AppError::Forbidden(msg)) => {
            audit_telegram_event(
                &ctx,
                AuditEvent::new(
                    AuditLevel::Warning,
                    "oauth_telegram_login".to_string(),
                    "authentication".to_string(),
                    "denied_allowlist".to_string(),
                )
                .with_metadata("telegram_id".to_string(), tg_id_hint.clone())
                .with_metadata("detail".to_string(), msg.clone()),
            )
            .await;
            return enterprise_json_err(
                "TELEGRAM_USER_NOT_ALLOWED",
                msg,
                ErrorContext::new("oauth2_telegram_callback"),
                StatusCode::FORBIDDEN,
            )
            .into_response();
        }
        Err(AppError::ValidationError(msg)) => {
            audit_telegram_event(
                &ctx,
                AuditEvent::new(
                    AuditLevel::Warning,
                    "oauth_telegram_login".to_string(),
                    "authentication".to_string(),
                    "invalid_request".to_string(),
                )
                .with_metadata("telegram_id".to_string(), tg_id_hint.clone())
                .with_metadata("detail".to_string(), msg.clone()),
            )
            .await;
            return enterprise_json_err(
                "TELEGRAM_AUTH_INVALID",
                msg,
                ErrorContext::new("oauth2_telegram_callback"),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }
        Err(e) => {
            return enterprise_err(
                "TELEGRAM_AUTH_FAILED",
                e.to_string(),
                "oauth2_telegram_callback",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    };

    let telegram_id_str = telegram_id.to_string();

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

    let poolai_user = user_manager
        .get_user_by_username(&username)
        .await
        .unwrap_or(None);

    let (final_username, role) = if let Some(user) = poolai_user {
        (user.username.clone(), user.role)
    } else {
        match user_manager
            .create_user(
                username.clone(),
                format!("oauth2_telegram_{}", telegram_id_str),
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

    audit_telegram_event(
        &ctx,
        AuditEvent::new(
            AuditLevel::Info,
            "oauth_telegram_login".to_string(),
            "authentication".to_string(),
            "success".to_string(),
        )
        .with_user_id(final_username.clone())
        .with_metadata("telegram_id".to_string(), telegram_id_str),
    )
    .await;

    let redirect_url = format!(
        "/ui/auth?token={}&username={}&role={:?}&expires_in=3600",
        urlencoding::encode(&poolai_token),
        urlencoding::encode(&final_username),
        role
    );

    Redirect::temporary(&redirect_url).into_response()
}
