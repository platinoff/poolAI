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

use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::network::api::common::HttpAppError;
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
) -> Result<Json<InstancePreviewResponse>, HttpAppError> {
    let model_id = match params.get("model_id") {
        Some(id) => id.as_str(),
        None => {
            return Err(HttpAppError::new(AppError::ValidationError(
                "model_id query parameter is required".to_string(),
            ))
            .with_context(
                ErrorContext::new("instance_previews").with_resource("query_param", "model_id"),
            ));
        }
    };

    match InstanceService::placement_previews(&ctx, model_id).await {
        Ok(response) => Ok(Json(response)),
        Err(InstanceServiceError::ManagerUnavailable) => Err(HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(
            ErrorContext::new("instance_previews").with_resource("instance_manager", "default"),
        )),
        Err(InstanceServiceError::Preview(e) | InstanceServiceError::Operation(e)) => Err(
            HttpAppError::new(AppError::InternalError(e.to_string())).with_context(
                ErrorContext::new("instance_previews").with_resource("model_id", model_id),
            ),
        ),
    }
}

async fn instance_create_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Json(body): Json<CreateInstanceBody>,
) -> Result<Json<CreateInstanceResponse>, HttpAppError> {
    let request = CreateInstanceRequest {
        instance: body.instance,
    };
    match InstanceService::create_instance(&ctx, &request).await {
        Ok(instance_id) => {
            let command_id = uuid::Uuid::new_v4().to_string();
            Ok(Json(CreateInstanceResponse {
                message: "Command received.".to_string(),
                command_id,
                instance_id,
            }))
        }
        Err(InstanceServiceError::ManagerUnavailable) => Err(HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(
            ErrorContext::new("create_instance").with_resource("instance_manager", "default"),
        )),
        Err(InstanceServiceError::Operation(e) | InstanceServiceError::Preview(e)) => {
            Err(HttpAppError::new(AppError::InternalError(e.to_string()))
                .with_context(ErrorContext::new("create_instance")))
        }
    }
}

async fn instance_list_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<InstanceListResponse>, HttpAppError> {
    match InstanceService::list_instances(&ctx).await {
        Ok(response) => Ok(Json(response)),
        Err(InstanceServiceError::ManagerUnavailable) => Err(HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(
            ErrorContext::new("list_instances").with_resource("instance_manager", "default"),
        )),
        Err(InstanceServiceError::Preview(e) | InstanceServiceError::Operation(e)) => {
            Err(HttpAppError::new(AppError::InternalError(e.to_string()))
                .with_context(ErrorContext::new("list_instances")))
        }
    }
}

async fn instance_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<InstanceInfo>, HttpAppError> {
    match InstanceService::get_instance(&ctx, &id).await {
        Ok(Some(info)) => Ok(Json(info)),
        Ok(None) => Err(
            HttpAppError::new(AppError::ApiNotFound("Instance not found".to_string()))
                .with_context(ErrorContext::new("get_instance").with_resource("instance_id", &id)),
        ),
        Err(InstanceServiceError::ManagerUnavailable) => Err(HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(
            ErrorContext::new("get_instance").with_resource("instance_manager", "default"),
        )),
        Err(InstanceServiceError::Preview(e) | InstanceServiceError::Operation(e)) => {
            Err(HttpAppError::new(AppError::InternalError(e.to_string()))
                .with_context(ErrorContext::new("get_instance").with_resource("instance_id", &id)))
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
        Err(InstanceServiceError::ManagerUnavailable) => HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(
            ErrorContext::new("delete_instance").with_resource("instance_manager", "default"),
        )
        .into_response(),
        Err(InstanceServiceError::Operation(e) | InstanceServiceError::Preview(e)) => {
            HttpAppError::new(AppError::InternalError(e.to_string()))
                .with_context(
                    ErrorContext::new("delete_instance").with_resource("instance_id", &id),
                )
                .into_response()
        }
    }
}

async fn state_handler(
    State(ctx): State<ApiContext>,
) -> Result<Json<HashMap<String, serde_json::Value>>, HttpAppError> {
    match InstanceService::deployment_state(&ctx).await {
        Ok(state) => Ok(Json(state)),
        Err(InstanceServiceError::ManagerUnavailable) => Err(HttpAppError::new(
            AppError::SubsystemUnavailable("Instance manager not initialized".to_string()),
        )
        .with_context(ErrorContext::new("state").with_resource("instance_manager", "default"))),
        Err(InstanceServiceError::Preview(e) | InstanceServiceError::Operation(e)) => {
            Err(HttpAppError::new(AppError::InternalError(e.to_string()))
                .with_context(ErrorContext::new("state")))
        }
    }
}
