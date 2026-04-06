//! Enterprise API endpoints (`/api/enterprise/*`).
//!
//! Handlers split by area: [`tenants`], [`audit`], [`monitoring`], [`security`], [`oauth`], [`saml`].

mod audit;
mod monitoring;
mod oauth;
mod saml;
mod security;
mod tenants;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::network::auth::auth_middleware;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Json, Router};

use audit::*;
use monitoring::*;
use oauth::*;
use saml::*;
use security::*;
use tenants::*;

/// Shorthand for structured enterprise API errors (same shape as [`api_json_error`]).
pub(super) fn enterprise_err(
    code: impl AsRef<str>,
    message: impl Into<String>,
    operation: impl Into<String>,
    status: StatusCode,
) -> (StatusCode, Json<serde_json::Value>) {
    api_json_error(
        code,
        message,
        Some(ErrorContext::new(operation.into())),
        status,
    )
}

/// Build the enterprise REST router (requires Cargo feature `enterprise`).
pub fn create_enterprise_api_routes() -> Router<ApiContext> {
    let router = Router::new()
        // Tenant management
        .route("/tenants", get(tenants_list_handler))
        .route(
            "/tenants",
            post(tenant_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/tenants/{id}", get(tenant_get_handler))
        .route(
            "/tenants/{id}",
            post(tenant_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/tenants/{id}",
            delete(tenant_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/tenants/{id}/usage", get(tenant_usage_handler))
        .route("/tenants/{id}/quota", post(tenant_quota_check_handler))
        // Audit logs
        .route("/audit/events", get(audit_events_query_handler))
        // Monitoring
        .route("/monitoring/alerts", get(monitoring_alerts_handler))
        .route(
            "/monitoring/alerts/{id}/acknowledge",
            post(monitoring_alert_acknowledge_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/dashboards", get(monitoring_dashboards_handler))
        .route(
            "/monitoring/dashboards",
            post(monitoring_dashboard_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/monitoring/metrics", get(monitoring_metrics_handler))
        .route(
            "/monitoring/alert-rules",
            post(monitoring_alert_rule_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/monitoring/alert-rules",
            get(monitoring_alert_rules_handler),
        )
        // Security
        .route(
            "/security/oauth2/providers",
            get(security_oauth2_providers_handler),
        )
        .route(
            "/security/oauth2/providers",
            post(security_oauth2_provider_register_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/oauth2/providers/{name}",
            get(security_oauth2_provider_get_handler),
        )
        .route(
            "/security/oauth2/providers/{name}",
            put(security_oauth2_provider_update_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/oauth2/providers/{name}",
            delete(security_oauth2_provider_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/saml/providers",
            get(security_saml_providers_handler),
        )
        .route(
            "/security/saml/providers",
            post(security_saml_provider_register_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/saml/providers/{name}",
            get(security_saml_provider_get_handler),
        )
        .route(
            "/security/saml/providers/{name}",
            put(security_saml_provider_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/saml/providers/{name}",
            delete(security_saml_provider_delete_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route("/security/policies", get(security_policies_handler))
        .route(
            "/security/policies",
            post(security_policy_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/policies/{name}",
            get(security_policy_get_handler),
        )
        .route(
            "/security/policies/{name}",
            put(security_policy_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/security/policies/{name}",
            delete(security_policy_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
        // OAuth2 Authentication endpoints
        .route("/auth/github", get(oauth2_github_auth_handler))
        .route("/auth/github/callback", get(oauth2_github_callback_handler))
        .route("/auth/google", get(oauth2_google_auth_handler))
        .route("/auth/google/callback", get(oauth2_google_callback_handler))
        .route("/auth/telegram", get(oauth2_telegram_auth_handler))
        .route(
            "/auth/telegram/callback",
            get(oauth2_telegram_callback_handler),
        )
        // SAML SSO Authentication endpoints
        .route("/auth/saml/{provider}", get(saml_auth_handler))
        .route(
            "/auth/saml/{provider}/callback",
            post(saml_callback_handler),
        );
    #[cfg(feature = "ml")]
    let router = router.nest("/ai-ml", crate::network::api::ai_ml::create_ai_ml_routes());
    router
}
