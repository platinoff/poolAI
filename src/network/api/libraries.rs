//! Library management API endpoints
//!
//! Provides endpoints for managing libraries:
//! - List installed libraries
//! - Get library information
//! - Install, uninstall, update libraries

use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};

use crate::libs::{get_global_manager, LibraryType};
use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};

/// Create library management routes
pub fn create_libraries_routes() -> Router {
    Router::new()
        .route("/libraries", get(libraries_list_handler))
        .route("/libraries/{name}", get(library_info_handler))
        .route(
            "/libraries/{name}/install",
            post(library_install_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/libraries/{name}/uninstall",
            post(library_uninstall_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/libraries/{name}/update",
            post(library_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/libraries/upload",
            post(library_upload_handler).layer(middleware::from_fn(auth_middleware)),
        )
}

async fn libraries_list_handler() -> impl IntoResponse {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        let libraries = manager.list_libraries().await;
        AxumJson(libraries).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}

async fn library_info_handler(Path(name): Path<String>) -> impl IntoResponse {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.get_library(&name).await {
            Some(lib) => AxumJson(lib).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                AxumJson(serde_json::json!({
                    "error": format!("Library {} not found", name)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}

async fn library_install_handler(
    Path(name): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let version = payload
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("latest");

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager
            .install_library(&name, version, LibraryType::ModelLibrary)
            .await
        {
            Ok(lib) => AxumJson(lib).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to install library. Context: Cannot install library from registry. Suggestion: Verify library name and version exist, check network connectivity, ensure sufficient disk space, and verify library manager is initialized. Library: '{}', Version: '{}', Error: {}", name, version, e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}

async fn library_uninstall_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.uninstall_library(&name).await {
            Ok(_) => AxumJson(serde_json::json!({
                "message": format!("Library {} uninstalled successfully", name)
            }))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to uninstall library. Context: Cannot remove library from system. Suggestion: Verify library is installed, check for active dependencies, ensure library is not in use, and verify library manager is initialized. Library: '{}', Error: {}", name, e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}

async fn library_update_handler(
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.update_library(&name).await {
            Ok(lib) => AxumJson(lib).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to update library. Context: Cannot update library to newer version. Suggestion: Verify library is installed, check for available updates, ensure sufficient disk space, and verify library manager is initialized. Library: '{}', Error: {}", name, e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}

#[derive(serde::Deserialize)]
struct LibraryUploadRequest {
    name: String,
    version: String,
    data: String, // Base64-encoded archive data
}

async fn library_upload_handler(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<LibraryUploadRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager
            .upload_library(
                &payload.name,
                &payload.version,
                &payload.data,
                LibraryType::ModelLibrary,
            )
            .await
        {
            Ok(lib) => AxumJson(lib).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(serde_json::json!({
                    "error": format!("Failed to upload library. Context: Cannot upload and install library from base64 data. Suggestion: Verify base64 data is valid, check disk space, ensure sufficient permissions, and verify library manager is initialized. Library: '{}', Version: '{}', Error: {}", payload.name, payload.version, e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            AxumJson(serde_json::json!({
                "error": "Library manager not initialized. Context: Library manager is not available. Suggestion: Ensure library manager is initialized before managing libraries. Check system startup sequence and library manager initialization status."
            })),
        )
            .into_response()
    }
}
