//! System API endpoints
//!
//! Provides system-level endpoints:
//! - Status
//! - Health checks
//! - Metrics
//! - Authentication (login)
//! - Models
//! - GPU information

use axum::{
    extract::{Extension, Request, State},
    http::{header::ACCEPT, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};

use crate::core::config::PoolAIConfig;
use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::network::api::common::{check_permission, HttpAppError};
use crate::network::auth::{
    bearer_token_from_authorization_header, refresh_access_token, AuthRequest, Claims,
};
use crate::network::ws::websocket_handler;
use crate::services::system_service::{HealthResponse, MetricsResponse, ModelInfo, SystemService};

/// Create system routes
pub fn create_system_routes() -> Router<ApiContext> {
    Router::new()
        .route("/status", get(status_handler))
        .route("/health", get(health_handler))
        .route("/login", axum::routing::post(login_handler))
        .route("/refresh", axum::routing::post(refresh_handler))
        .route("/metrics", get(metrics_handler))
        .route("/models", get(models_handler))
        .route("/gpu", get(gpu_info))
        .route("/gpu-limits", get(gpu_limits_handler))
        .route("/ws/metrics", get(websocket_handler))
        .route("/config", get(config_get_handler))
        .route(
            "/config",
            put(config_update_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
}

async fn status_handler(
    State(app_state): State<ApiContext>,
    req: Request<axum::body::Body>,
) -> Response {
    // Touch system state so that future extensions can use it without changing
    // the handler signature again.
    let _ = app_state.get_system_state();
    let status = SystemService::status_snapshot();
    // Check the Accept header
    let want_html = req
        .headers()
        .get(ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/html"))
        .unwrap_or(false);
    if want_html {
        let html = super::system_status_html::status_page_html(&status);
        (
            StatusCode::OK,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response()
    } else {
        Json(status).into_response()
    }
}

async fn metrics_handler() -> Result<Json<MetricsResponse>, AppError> {
    Ok(Json(SystemService::metrics_snapshot()))
}

async fn models_handler() -> Result<Json<Vec<ModelInfo>>, AppError> {
    Ok(Json(SystemService::models_snapshot()))
}

async fn gpu_info() -> Result<Json<crate::platform::GpuInfo>, AppError> {
    Ok(Json(SystemService::gpu_snapshot()))
}

async fn gpu_limits_handler() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        poolai_ui_core::gpu_limits_store::gpu_limits_store_wire_json(),
    ))
}

async fn health_handler() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(SystemService::health_snapshot()))
}

async fn login_handler(
    State(ctx): State<ApiContext>,
    Json(auth_req): Json<AuthRequest>,
) -> impl IntoResponse {
    match SystemService::login(auth_req, ctx.user_manager.clone()).await {
        Ok(auth_response) => Json(auth_response).into_response(),
        Err(e) => e.into_response(),
    }
}

async fn refresh_handler(State(ctx): State<ApiContext>, req: Request) -> impl IntoResponse {
    let Some(token) = bearer_token_from_authorization_header(&req) else {
        return HttpAppError::new(AppError::RestError {
            code: "AUTH_MISSING_HEADER",
            message: "Missing or invalid authorization header".to_string(),
        })
        .with_context(ErrorContext::new("refresh_handler"))
        .with_status(StatusCode::UNAUTHORIZED)
        .into_response();
    };

    match refresh_access_token(&token, ctx.user_manager.clone()).await {
        Ok(auth_response) => Json(auth_response).into_response(),
        Err(e) => e.into_response(),
    }
}

/// Get system configuration
async fn config_get_handler() -> Result<Json<PoolAIConfig>, HttpAppError> {
    match SystemService::get_configuration() {
        Ok(config) => Ok(Json(config)),
        Err(e) => Err(HttpAppError::new(AppError::RestError {
            code: "CONFIG_GET_FAILED",
            message: format!("Failed to get configuration: {}", e),
        })
        .with_context(ErrorContext::new("config_get"))),
    }
}

/// Update system configuration
async fn config_update_handler(
    Extension(claims): Extension<Claims>,
    Json(config): Json<PoolAIConfig>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match SystemService::apply_configuration(config) {
        Ok(()) => Json(serde_json::json!({
            "message": "Configuration updated successfully"
        }))
        .into_response(),
        Err(e) => HttpAppError::new(AppError::RestError {
            code: "CONFIG_UPDATE_FAILED",
            message: format!("Failed to update configuration: {}", e),
        })
        .with_context(ErrorContext::new("config_update"))
        .with_status(StatusCode::BAD_REQUEST)
        .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request as HttpRequest, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn status_handler_works_with_api_context() {
        let app_state = ApiContext::default();

        let app = create_system_routes().with_state(app_state);

        let response = app
            .oneshot(
                HttpRequest::builder()
                    .uri("/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
