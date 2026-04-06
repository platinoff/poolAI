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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
#[cfg(feature = "enterprise")]
use crate::enterprise::monitoring::Dashboard;
#[cfg(feature = "enterprise")]
use crate::network::api::check_permission;
use crate::network::api::common::api_json_error;
use crate::network::auth::{auth_middleware, Claims};
use crate::ui::{components, get_all_themes, get_theme};
#[cfg(feature = "enterprise")]
use axum::extract::State;

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
    let manager = ctx.enterprise_monitoring_manager.clone();
    match manager.list_dashboards(None).await {
        Ok(dashboards) => AxumJson(dashboards).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "LIST_DASHBOARDS_FAILED",
                format!("Failed to list dashboards: {}", e),
                Some(ErrorContext::new("ui_dashboards_list")),
                StatusCode::INTERNAL_SERVER_ERROR,
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
    let manager = ctx.enterprise_monitoring_manager.clone();
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

    match manager.get_dashboard(uuid).await {
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
            let (s, j) = api_json_error(
                "GET_DASHBOARD_FAILED",
                format!("Failed to get dashboard: {}", e),
                Some(ErrorContext::new("ui_dashboard_get").with_resource("dashboard_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
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

    let manager = ctx.enterprise_monitoring_manager.clone();
    let dashboard = Dashboard {
        id: Uuid::new_v4(),
        name: payload.name.clone(),
        description: payload.description.clone(),
        metrics: payload.metrics.clone(),
        layout: payload.layout.clone(),
        is_public: payload.is_public.unwrap_or(false),
        tenant_id: payload.tenant_id,
        created_at: chrono::Utc::now(),
    };

    match manager.create_dashboard(dashboard.clone()).await {
        Ok(_) => (StatusCode::CREATED, AxumJson(dashboard)).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "CREATE_DASHBOARD_FAILED",
                format!("Failed to create dashboard: {}", e),
                Some(ErrorContext::new("ui_dashboard_create")),
                StatusCode::INTERNAL_SERVER_ERROR,
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

    let manager = ctx.enterprise_monitoring_manager.clone();
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

    // Get existing dashboard first
    let existing = match manager.get_dashboard(uuid).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            let (s, j) = api_json_error(
                "DASHBOARD_NOT_FOUND",
                format!("Dashboard not found: {}", id),
                Some(ErrorContext::new("ui_dashboard_update").with_resource("dashboard_id", &id)),
                StatusCode::NOT_FOUND,
            );
            return (s, AxumJson(j.0)).into_response();
        }
        Err(e) => {
            let (s, j) = api_json_error(
                "GET_DASHBOARD_FAILED",
                format!("Failed to get dashboard: {}", e),
                Some(ErrorContext::new("ui_dashboard_update").with_resource("dashboard_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    // Update dashboard
    let updated = Dashboard {
        id: existing.id,
        name: payload.name,
        description: payload.description,
        metrics: payload.metrics,
        layout: payload.layout,
        is_public: payload.is_public.unwrap_or(existing.is_public),
        tenant_id: payload.tenant_id.or(existing.tenant_id),
        created_at: existing.created_at,
    };

    match manager.create_dashboard(updated.clone()).await {
        Ok(_) => AxumJson(updated).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "UPDATE_DASHBOARD_FAILED",
                format!("Failed to update dashboard: {}", e),
                Some(ErrorContext::new("ui_dashboard_update").with_resource("dashboard_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
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

#[derive(Serialize)]
struct ThemeResponse {
    name: String,
    css_variables: String,
    css: String,
}

async fn ui_themes_handler() -> impl IntoResponse {
    let themes = get_all_themes();
    let response: Vec<ThemeResponse> = themes
        .into_iter()
        .map(|theme| ThemeResponse {
            name: theme.name.to_string(),
            css_variables: theme.to_css_variables(),
            css: theme.to_css(),
        })
        .collect();
    AxumJson(response).into_response()
}

async fn ui_theme_get_handler(Path(name): Path<String>) -> impl IntoResponse {
    let theme = get_theme(&name);
    let response = ThemeResponse {
        name: theme.name.to_string(),
        css_variables: theme.to_css_variables(),
        css: theme.to_css(),
    };
    AxumJson(response).into_response()
}

// ============================================================================
// Component registry handlers
// ============================================================================

#[derive(Serialize)]
struct ComponentInfo {
    name: String,
    #[serde(rename = "type")]
    component_type: String, // "button", "card", "form", etc.
    styles: String,
    description: Option<String>,
}

async fn ui_components_handler() -> impl IntoResponse {
    // Use get_component_styles() helper if available, otherwise use constants directly
    let components = vec![
        ComponentInfo {
            name: "button".to_string(),
            component_type: "button".to_string(),
            styles: components::BUTTON_STYLES.to_string(),
            description: Some(
                "Button component with primary, danger, secondary variants".to_string(),
            ),
        },
        ComponentInfo {
            name: "card".to_string(),
            component_type: "card".to_string(),
            styles: components::CARD_STYLES.to_string(),
            description: Some("Card component for content containers".to_string()),
        },
        ComponentInfo {
            name: "form".to_string(),
            component_type: "form".to_string(),
            styles: components::FORM_STYLES.to_string(),
            description: Some("Form component for input fields and validation".to_string()),
        },
    ];

    AxumJson(components).into_response()
}

async fn ui_component_get_handler(Path(name): Path<String>) -> impl IntoResponse {
    let component: ComponentInfo = match name.as_str() {
        "button" => ComponentInfo {
            name: "button".to_string(),
            component_type: "button".to_string(),
            styles: components::BUTTON_STYLES.to_string(),
            description: Some(
                "Button component with primary, danger, secondary variants".to_string(),
            ),
        },
        "card" => ComponentInfo {
            name: "card".to_string(),
            component_type: "card".to_string(),
            styles: components::CARD_STYLES.to_string(),
            description: Some("Card component for content containers".to_string()),
        },
        "form" => ComponentInfo {
            name: "form".to_string(),
            component_type: "form".to_string(),
            styles: components::FORM_STYLES.to_string(),
            description: Some("Form component for input fields and validation".to_string()),
        },
        _ => {
            let (s, j) = api_json_error(
                "COMPONENT_NOT_FOUND",
                format!(
                    "Component not found: {}. Available components: button, card, form",
                    name
                ),
                Some(ErrorContext::new("ui_component_get").with_resource("component", &name)),
                StatusCode::NOT_FOUND,
            );
            return (s, AxumJson(j.0)).into_response();
        }
    };

    AxumJson(component).into_response()
}
