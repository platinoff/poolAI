//! UI Management API endpoints
//!
//! Provides endpoints for managing UI components:
//! - Dashboard management (custom dashboards)
//! - Component registry (UI components configuration)
//! - Theme management (custom themes)

use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json as AxumJson, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
#[cfg(not(feature = "enterprise"))]
use crate::network::api::common::api_json_error;
#[cfg(feature = "enterprise")]
use crate::network::api::common::{api_json_error, check_permission};
use crate::network::auth::{auth_middleware, Claims};
#[cfg(feature = "enterprise")]
use crate::services::enterprise_service::{
    DashboardCreateInput, DashboardUpdateInput, EnterpriseMonitoringError,
};
use crate::services::ui_service::UiService;
#[cfg(feature = "enterprise")]
use axum::extract::State;

#[cfg(feature = "enterprise")]
type UiHttpErr = (StatusCode, AxumJson<serde_json::Value>);

#[cfg(feature = "enterprise")]
fn ui_monitoring_http_err(
    e: EnterpriseMonitoringError,
    api_code: &'static str,
    operation: &'static str,
    message_for_operation: impl Into<String>,
) -> UiHttpErr {
    match e {
        EnterpriseMonitoringError::Init(err) => api_json_error(
            "MONITORING_MANAGER_UNAVAILABLE",
            format!("Monitoring manager not initialized: {}", err),
            Some(ErrorContext::new(operation).with_hint("Check system startup sequence.")),
            StatusCode::SERVICE_UNAVAILABLE,
        ),
        EnterpriseMonitoringError::Operation(err) => api_json_error(
            api_code,
            format!("{}: {}", message_for_operation.into(), err),
            Some(ErrorContext::new(operation)),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

/// Create UI management routes
pub fn create_ui_routes() -> Router<ApiContext> {
    Router::new()
        // Dashboard management endpoints
        .route("/ui/dashboards", get(ui_dashboards_handler))
        .route(
            "/ui/dashboards",
            post(ui_dashboard_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/ui/dashboards/{id}", get(ui_dashboard_get_handler))
        .route(
            "/ui/dashboards/{id}",
            put(ui_dashboard_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/ui/dashboards/{id}",
            delete(ui_dashboard_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
        // Theme management endpoints
        .route("/ui/themes", get(ui_themes_handler))
        .route("/ui/themes/{name}", get(ui_theme_get_handler))
        // Component registry endpoints
        .route("/ui/components", get(ui_components_handler))
        .route("/ui/components/{name}", get(ui_component_get_handler))
}

// ============================================================================
// Dashboard management handlers
// ============================================================================

/// Request structure for creating a dashboard
///
/// Note: Fields are used via move semantics in handlers, which may not be detected by the compiler.
/// This is part of the API contract and fields are actively used in dashboard creation.
#[derive(Deserialize)]
#[allow(dead_code)] // Fields are used via move in handlers
struct CreateDashboardRequest {
    name: String,
    description: String,
    metrics: Vec<String>,
    layout: String,
    is_public: Option<bool>,
    tenant_id: Option<Uuid>,
}

#[cfg(feature = "enterprise")]
async fn ui_dashboards_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match UiService::list_dashboards(&ctx).await {
        Ok(dashboards) => AxumJson(dashboards).into_response(),
        Err(e) => {
            let (s, j) = ui_monitoring_http_err(
                e,
                "LIST_DASHBOARDS_FAILED",
                "ui_dashboards_list",
                "Failed to list dashboards",
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboards_handler() -> impl IntoResponse {
    let (s, j) = api_json_error(
        "FEATURE_DISABLED",
        "Dashboards API requires enterprise feature. Enable with --features enterprise",
        Some(
            ErrorContext::new("ui_dashboards_list")
                .with_hint("Build or run with --features enterprise."),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format: {}", id),
                Some(ErrorContext::new("ui_dashboard_get").with_resource("dashboard_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    match UiService::get_dashboard(&ctx, uuid).await {
        Ok(Some(dashboard)) => AxumJson(dashboard).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "DASHBOARD_NOT_FOUND",
                format!("Dashboard not found: {}", id),
                Some(ErrorContext::new("ui_dashboard_get").with_resource("dashboard_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => {
            let (s, j) = ui_monitoring_http_err(
                e,
                "GET_DASHBOARD_FAILED",
                "ui_dashboard_get",
                "Failed to get dashboard",
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_get_handler(Path(_id): Path<String>) -> impl IntoResponse {
    let (s, j) = api_json_error(
        "FEATURE_DISABLED",
        "Dashboards API requires enterprise feature. Enable with --features enterprise",
        Some(
            ErrorContext::new("ui_dashboard_get")
                .with_hint("Build or run with --features enterprise."),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_create_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "write:all")
        .or_else(|_| check_permission(&claims, "write:monitoring"))
    {
        return err.into_response();
    }

    if payload.name.is_empty() {
        let (s, j) = api_json_error(
            "VALIDATION_ERROR",
            "Dashboard name cannot be empty",
            Some(ErrorContext::new("ui_dashboard_create").with_resource("field", "name")),
            StatusCode::BAD_REQUEST,
        );
        return (s, AxumJson(j.0)).into_response();
    }

    let input = DashboardCreateInput {
        name: payload.name,
        description: Some(payload.description),
        metrics: payload.metrics,
        layout: Some(payload.layout),
        is_public: payload.is_public,
        tenant_id: payload.tenant_id.map(|u| u.to_string()),
    };

    match UiService::create_dashboard(&ctx, input).await {
        Ok(dashboard) => (StatusCode::CREATED, AxumJson(dashboard)).into_response(),
        Err(e) => {
            let (s, j) = ui_monitoring_http_err(
                e,
                "CREATE_DASHBOARD_FAILED",
                "ui_dashboard_create",
                "Failed to create dashboard",
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_create_handler(
    Extension(_claims): Extension<Claims>,
    Json(_payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    let (s, j) = api_json_error(
        "FEATURE_DISABLED",
        "Dashboards API requires enterprise feature. Enable with --features enterprise",
        Some(
            ErrorContext::new("ui_dashboard_create")
                .with_hint("Build or run with --features enterprise."),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "write:all")
        .or_else(|_| check_permission(&claims, "write:monitoring"))
    {
        return err.into_response();
    }

    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            let (s, j) = api_json_error(
                "INVALID_UUID",
                format!("Invalid UUID format: {}", id),
                Some(ErrorContext::new("ui_dashboard_update").with_resource("dashboard_id", &id)),
                StatusCode::BAD_REQUEST,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    let input = DashboardUpdateInput {
        name: payload.name,
        description: payload.description,
        metrics: payload.metrics,
        layout: payload.layout,
        is_public: payload.is_public,
        tenant_id: payload.tenant_id,
    };

    match UiService::update_dashboard(&ctx, uuid, input).await {
        Ok(Some(updated)) => AxumJson(updated).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "DASHBOARD_NOT_FOUND",
                format!("Dashboard not found: {}", id),
                Some(ErrorContext::new("ui_dashboard_update").with_resource("dashboard_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => {
            let (s, j) = ui_monitoring_http_err(
                e,
                "UPDATE_DASHBOARD_FAILED",
                "ui_dashboard_update",
                "Failed to update dashboard",
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_update_handler(
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
    Json(_payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    let (s, j) = api_json_error(
        "FEATURE_DISABLED",
        "Dashboards API requires enterprise feature. Enable with --features enterprise",
        Some(
            ErrorContext::new("ui_dashboard_update")
                .with_hint("Build or run with --features enterprise."),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_delete_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "delete:all")
        .or_else(|_| check_permission(&claims, "write:monitoring"))
    {
        return err.into_response();
    }

    // Note: MonitoringManager doesn't have delete_dashboard() yet
    let (s, j) = api_json_error(
        "NOT_IMPLEMENTED",
        "Dashboard deletion not yet implemented; MonitoringManager.delete_dashboard() is not available.",
        Some(
            ErrorContext::new("ui_dashboard_delete")
                .with_resource("dashboard_id", &id)
                .with_hint(
                    "Add delete_dashboard() to MonitoringManager or mark deleted via dashboard update.",
                ),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_delete_handler(
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    let (s, j) = api_json_error(
        "FEATURE_DISABLED",
        "Dashboards API requires enterprise feature. Enable with --features enterprise",
        Some(
            ErrorContext::new("ui_dashboard_delete")
                .with_hint("Build or run with --features enterprise."),
        ),
        StatusCode::NOT_IMPLEMENTED,
    );
    (s, AxumJson(j.0)).into_response()
}

// ============================================================================
// Theme management handlers
// ============================================================================

async fn ui_themes_handler() -> impl IntoResponse {
    AxumJson(UiService::list_themes()).into_response()
}

async fn ui_theme_get_handler(Path(name): Path<String>) -> impl IntoResponse {
    AxumJson(UiService::theme_by_name(&name)).into_response()
}

// ============================================================================
// Component registry handlers
// ============================================================================

async fn ui_components_handler() -> impl IntoResponse {
    AxumJson(UiService::list_components()).into_response()
}

async fn ui_component_get_handler(Path(name): Path<String>) -> impl IntoResponse {
    match UiService::get_component(&name) {
        Some(component) => AxumJson(component).into_response(),
        None => {
            let (s, j) = api_json_error(
                "COMPONENT_NOT_FOUND",
                format!(
                    "Component not found: {}. Available components: button, card, form",
                    name
                ),
                Some(ErrorContext::new("ui_component_get").with_resource("component", &name)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}
