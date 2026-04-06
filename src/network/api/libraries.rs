//! Library management API endpoints
//!
//! Provides endpoints for managing libraries:
//! - List installed libraries
//! - Get library information
//! - Install, uninstall, update libraries

use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::libs::LibraryType;
use crate::network::api::common::{api_json_error, check_permission};
use crate::network::auth::{auth_middleware, Claims};
use crate::services::library_service::{LibraryMutationError, LibraryService, LibraryServiceError};

type LibHttpErr = (StatusCode, AxumJson<serde_json::Value>);

fn library_manager_unavailable() -> LibHttpErr {
    api_json_error(
        "SUBSYSTEM_UNAVAILABLE",
        crate::services::library_service::LIBRARY_MANAGER_UNAVAILABLE_MESSAGE,
        Some(
            ErrorContext::new("libraries")
                .with_resource("library_manager", "default")
                .with_hint("Initialize the library manager during startup."),
        ),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}

fn library_service_err(e: LibraryServiceError) -> LibHttpErr {
    match e {
        LibraryServiceError::ManagerUnavailable => library_manager_unavailable(),
    }
}

/// Create library management routes
pub fn create_libraries_routes() -> Router<ApiContext> {
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

async fn libraries_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match LibraryService::list_libraries(&ctx).await {
        Ok(libraries) => AxumJson(libraries).into_response(),
        Err(e) => library_service_err(e).into_response(),
    }
}

async fn library_info_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match LibraryService::get_library(&ctx, &name).await {
        Ok(Some(lib)) => AxumJson(lib).into_response(),
        Ok(None) => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                format!("Library {} not found", name),
                Some(ErrorContext::new("get_library").with_resource("library", &name)),
                StatusCode::NOT_FOUND,
            );
            (s, AxumJson(j.0)).into_response()
        }
        Err(e) => library_service_err(e).into_response(),
    }
}

async fn library_install_handler(
    State(ctx): State<ApiContext>,
    Path(name): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let version = payload
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("latest");

    match LibraryService::install_library(&ctx, &name, version, LibraryType::ModelLibrary).await {
        Ok(lib) => AxumJson(lib).into_response(),
        Err(LibraryMutationError::ManagerUnavailable) => {
            library_manager_unavailable().into_response()
        }
        Err(LibraryMutationError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!(
                    "Failed to install library from registry: {} (library='{}', version='{}')",
                    e, name, version
                ),
                Some(
                    ErrorContext::new("install_library")
                        .with_resource("library", &name)
                        .with_hint(
                            "Verify name/version, connectivity, disk space, and manager state.",
                        ),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn library_uninstall_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    match LibraryService::uninstall_library(&ctx, &name).await {
        Ok(()) => AxumJson(serde_json::json!({
            "message": format!("Library {} uninstalled successfully", name)
        }))
        .into_response(),
        Err(LibraryMutationError::ManagerUnavailable) => {
            library_manager_unavailable().into_response()
        }
        Err(LibraryMutationError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!("Failed to uninstall library: {} (library='{}')", e, name),
                Some(
                    ErrorContext::new("uninstall_library")
                        .with_resource("library", &name)
                        .with_hint("Ensure the library is installed, not in use, and has no blocking dependencies."),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

async fn library_update_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    match LibraryService::update_library(&ctx, &name).await {
        Ok(lib) => AxumJson(lib).into_response(),
        Err(LibraryMutationError::ManagerUnavailable) => {
            library_manager_unavailable().into_response()
        }
        Err(LibraryMutationError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!("Failed to update library: {} (library='{}')", e, name),
                Some(
                    ErrorContext::new("update_library")
                        .with_resource("library", &name)
                        .with_hint("Verify the library is installed and updates are available."),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct LibraryUploadRequest {
    name: String,
    version: String,
    data: String, // Base64-encoded archive data
}

async fn library_upload_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<LibraryUploadRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }

    match LibraryService::upload_library(
        &ctx,
        &payload.name,
        &payload.version,
        &payload.data,
        LibraryType::ModelLibrary,
    )
    .await
    {
        Ok(lib) => AxumJson(lib).into_response(),
        Err(LibraryMutationError::ManagerUnavailable) => {
            library_manager_unavailable().into_response()
        }
        Err(LibraryMutationError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                format!(
                    "Failed to upload library: {} (library='{}', version='{}')",
                    e, payload.name, payload.version
                ),
                Some(
                    ErrorContext::new("upload_library")
                        .with_resource("library", &payload.name)
                        .with_hint("Verify base64 payload, disk space, and permissions."),
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, AxumJson(j.0)).into_response()
        }
    }
}
