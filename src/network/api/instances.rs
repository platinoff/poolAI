//! Model instance management API endpoints
//!
//! Provides endpoints for managing model instances:
//! - Instance preview API (placement options)
//! - Create, delete, list instances
//! - Get instance status

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::network::auth::Claims;
use crate::services::instance_service::{
    CreateInstanceRequest, CreateInstanceResponse, InstanceListResponse, InstanceService,
    InstanceServiceError,
};

// Re-exports for any external use of response shapes.
pub use crate::services::instance_service::{
    instance_placement_info, InstanceInfo, InstancePlacementInfo, InstancePreview,
    InstancePreviewResponse,
};

type InstanceJsonError = (StatusCode, Json<serde_json::Value>);

#[derive(Deserialize)]
struct CreateInstanceBody {
    instance: serde_json::Value,
}

/// Create instance routes
pub fn create_instance_routes() -> Router<ApiContext> {
    Router::new()
        .route("/instance/previews", get(instance_previews_handler))
        .route(
            "/instance",
            post(instance_create_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/instance", get(instance_list_handler))
        .route(
            "/instance/{id}",
            delete(instance_delete_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/instance/{id}", get(instance_get_handler))
        .route("/state", get(state_handler))
}

async fn instance_previews_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let model_id = match params.get("model_id") {
        Some(id) => id.as_str(),
        None => {
            let (s, j) = api_json_error(
                "VALIDATION_ERROR",
                "model_id query parameter is required",
                Some(
                    ErrorContext::new("instance_previews").with_resource("query_param", "model_id"),
                ),
                StatusCode::BAD_REQUEST,
            );
            return (s, Json(j.0)).into_response();
        }
    };

    match InstanceService::placement_previews(&ctx, model_id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(
                    ErrorContext::new("instance_previews")
                        .with_resource("instance_manager", "default"),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, Json(j.0)).into_response()
        }
        Err(InstanceServiceError::Preview(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("instance_previews").with_resource("model_id", model_id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, Json(j.0)).into_response()
        }
        Err(InstanceServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("instance_previews").with_resource("model_id", model_id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, Json(j.0)).into_response()
        }
    }
}

async fn instance_create_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Json(body): Json<CreateInstanceBody>,
) -> impl IntoResponse {
    let request = CreateInstanceRequest {
        instance: body.instance,
    };
    match InstanceService::create_instance(&ctx, &request).await {
        Ok(instance_id) => {
            let command_id = uuid::Uuid::new_v4().to_string();
            let response = CreateInstanceResponse {
                message: "Command received.".to_string(),
                command_id,
                instance_id,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(
                    ErrorContext::new("create_instance")
                        .with_resource("instance_manager", "default"),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, Json(j.0)).into_response()
        }
        Err(InstanceServiceError::Operation(e)) | Err(InstanceServiceError::Preview(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("create_instance")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, Json(j.0)).into_response()
        }
    }
}

async fn instance_list_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<InstanceListResponse>, InstanceJsonError> {
    match InstanceService::list_instances(&ctx).await {
        Ok(response) => Ok(Json(response)),
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(
                    ErrorContext::new("list_instances")
                        .with_resource("instance_manager", "default"),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            Err((s, Json(j.0)))
        }
        Err(InstanceServiceError::Preview(e)) | Err(InstanceServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("list_instances")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Err((s, Json(j.0)))
        }
    }
}

async fn instance_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<InstanceInfo>, InstanceJsonError> {
    match InstanceService::get_instance(&ctx, &id).await {
        Ok(Some(info)) => Ok(Json(info)),
        Ok(None) => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                "Instance not found",
                Some(ErrorContext::new("get_instance").with_resource("instance_id", &id)),
                StatusCode::NOT_FOUND,
            );
            Err((s, Json(j.0)))
        }
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(
                    ErrorContext::new("get_instance").with_resource("instance_manager", "default"),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            Err((s, Json(j.0)))
        }
        Err(InstanceServiceError::Preview(e)) | Err(InstanceServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("get_instance").with_resource("instance_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Err((s, Json(j.0)))
        }
    }
}

async fn instance_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match InstanceService::delete_instance(&ctx, &id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Instance deleted"})),
        )
            .into_response(),
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(
                    ErrorContext::new("delete_instance")
                        .with_resource("instance_manager", "default"),
                ),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            (s, Json(j.0)).into_response()
        }
        Err(InstanceServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("delete_instance").with_resource("instance_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, Json(j.0)).into_response()
        }
        Err(InstanceServiceError::Preview(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("delete_instance").with_resource("instance_id", &id)),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, Json(j.0)).into_response()
        }
    }
}

async fn state_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<HashMap<String, serde_json::Value>>, InstanceJsonError> {
    match InstanceService::deployment_state(&ctx).await {
        Ok(state) => Ok(Json(state)),
        Err(InstanceServiceError::ManagerUnavailable) => {
            let (s, j) = api_json_error(
                "SUBSYSTEM_UNAVAILABLE",
                "Instance manager not initialized",
                Some(ErrorContext::new("state").with_resource("instance_manager", "default")),
                StatusCode::SERVICE_UNAVAILABLE,
            );
            Err((s, Json(j.0)))
        }
        Err(InstanceServiceError::Preview(e)) | Err(InstanceServiceError::Operation(e)) => {
            let (s, j) = api_json_error(
                "INTERNAL_ERROR",
                e.to_string(),
                Some(ErrorContext::new("state")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Err((s, Json(j.0)))
        }
    }
}
