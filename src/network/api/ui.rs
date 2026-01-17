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

#[cfg(feature = "enterprise")]
use crate::enterprise::monitoring::{get_global_monitoring_manager, Dashboard};
use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};
use crate::ui::{components, get_all_themes, get_theme};

/// Create UI management routes
pub fn create_ui_routes() -> Router {
    Router::new()
        // Dashboard management endpoints
        .route("/ui/dashboards", get(ui_dashboards_handler))
        .route(
            "/ui/dashboards",
            post(ui_dashboard_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/ui/dashboards/{id}",
            get(ui_dashboard_get_handler),
        )
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
        .route(
            "/ui/themes/{name}",
            get(ui_theme_get_handler),
        )
        // Component registry endpoints
        .route("/ui/components", get(ui_components_handler))
        .route(
            "/ui/components/{name}",
            get(ui_component_get_handler),
        )
}

// ============================================================================
// Dashboard management handlers
// ============================================================================

#[derive(Deserialize)]
struct CreateDashboardRequest {
    name: String,
    description: String,
    metrics: Vec<String>,
    layout: String,
    is_public: Option<bool>,
    tenant_id: Option<Uuid>,
}

#[cfg(feature = "enterprise")]
async fn ui_dashboards_handler() -> impl IntoResponse {
    let manager = get_global_monitoring_manager();
    match manager.list_dashboards(None).await {
        Ok(dashboards) => AxumJson(dashboards).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to list dashboards: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboards_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboards API requires enterprise feature. Enable with --features enterprise"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_get_handler(Path(id): Path<String>) -> impl IntoResponse {
    let manager = get_global_monitoring_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format: {}", id)
                })),
            )
                .into_response();
        }
    };

    match manager.get_dashboard(uuid).await {
        Ok(Some(dashboard)) => AxumJson(dashboard).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            AxumJson(serde_json::json!({
                "error": format!("Dashboard not found: {}", id)
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to get dashboard: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_get_handler(Path(_id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboards API requires enterprise feature. Enable with --features enterprise"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_create_handler(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "write:all")
        .or_else(|_| check_permission(&claims, "write:monitoring"))
    {
        return err.into_response();
    }

    if payload.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            AxumJson(serde_json::json!({
                "error": "Dashboard name cannot be empty".to_string()
            })),
        )
            .into_response();
    }

    let manager = get_global_monitoring_manager();
    let dashboard = Dashboard {
        id: Uuid::new_v4(),
        name: payload.name.clone(),
        description: payload.description,
        metrics: payload.metrics,
        layout: payload.layout,
        is_public: payload.is_public.unwrap_or(false),
        tenant_id: payload.tenant_id,
        created_at: chrono::Utc::now(),
    };

    match manager.create_dashboard(dashboard.clone()).await {
        Ok(_) => (StatusCode::CREATED, AxumJson(dashboard)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to create dashboard: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_create_handler(
    Extension(_claims): Extension<Claims>,
    Json(_payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboards API requires enterprise feature. Enable with --features enterprise"
        })),
    )
        .into_response()
}

#[cfg(feature = "enterprise")]
async fn ui_dashboard_update_handler(
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "write:all")
        .or_else(|_| check_permission(&claims, "write:monitoring"))
    {
        return err.into_response();
    }

    let manager = get_global_monitoring_manager();
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                AxumJson(serde_json::json!({
                    "error": format!("Invalid UUID format: {}", id)
                })),
            )
                .into_response();
        }
    };

    // Get existing dashboard first
    let existing = match manager.get_dashboard(uuid).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                AxumJson(serde_json::json!({
                    "error": format!("Dashboard not found: {}", id)
                })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to get dashboard: {}", e)
                })),
            )
                .into_response();
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
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(serde_json::json!({
                "error": format!("Failed to update dashboard: {}", e)
            })),
        )
            .into_response(),
    }
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_update_handler(
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
    Json(_payload): Json<CreateDashboardRequest>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboards API requires enterprise feature. Enable with --features enterprise"
        })),
    )
        .into_response()
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
    // For now, return not implemented
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboard deletion not yet implemented. Context: MonitoringManager.delete_dashboard() method is not available. Suggestion: Add delete_dashboard() method to MonitoringManager or use dashboard update to mark as deleted.",
            "message": format!("Dashboard deletion requested for: {}", id)
        })),
    )
        .into_response()
}

#[cfg(not(feature = "enterprise"))]
async fn ui_dashboard_delete_handler(
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        AxumJson(serde_json::json!({
            "error": "Dashboards API requires enterprise feature. Enable with --features enterprise"
        })),
    )
        .into_response()
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
            description: Some("Button component with primary, danger, secondary variants".to_string()),
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
            description: Some("Button component with primary, danger, secondary variants".to_string()),
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
            return (
                StatusCode::NOT_FOUND,
                AxumJson(serde_json::json!({
                    "error": format!("Component not found: {}. Available components: button, card, form", name)
                })),
            )
                .into_response();
        }
    };

    AxumJson(component).into_response()
}
