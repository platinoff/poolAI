//! Model instance management API endpoints
//!
//! Provides endpoints for managing model instances:
//! - Instance preview API (placement options)
//! - Create, delete, list instances
//! - Get instance status

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::model_interface::ModelInfo;
use crate::network::api::common::check_permission;
use crate::network::auth::Claims;
use crate::runtime::instance::{
    get_global_instance_manager, InstanceManager, InstancePlacement, InstanceStatus,
};

/// Instance preview response
#[derive(Serialize)]
struct InstancePreviewResponse {
    previews: Vec<InstancePreview>,
}

/// Instance preview
#[derive(Serialize)]
struct InstancePreview {
    model_id: String,
    sharding: String,
    instance_meta: String,
    instance: serde_json::Value, // Simplified for now
    memory_delta_by_node: HashMap<String, i64>,
    error: Option<String>,
}

/// Create instance request
#[derive(Deserialize)]
struct CreateInstanceRequest {
    instance: serde_json::Value, // Full instance placement
}

/// Create instance response
#[derive(Serialize)]
struct CreateInstanceResponse {
    message: String,
    command_id: String,
    instance_id: String,
}

/// Instance list response
#[derive(Serialize)]
struct InstanceListResponse {
    instances: Vec<InstanceInfo>,
}

/// Instance info
#[derive(Serialize)]
struct InstanceInfo {
    instance_id: String,
    model_id: String,
    status: String,
    created_at: String,
    placement: InstancePlacement,
}

/// Create instance routes
pub fn create_instance_routes() -> Router {
    Router::new()
        .route("/instance/previews", get(instance_previews_handler))
        .route(
            "/instance",
            post(instance_create_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/instance", get(instance_list_handler))
        .route(
            "/instance/:id",
            delete(instance_delete_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
        .route("/instance/:id", get(instance_get_handler))
        .route("/state", get(state_handler))
}

/// Handler for GET /api/v1/instance/previews?model_id=xxx
/// Returns placement previews for a model
async fn instance_previews_handler(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let model_id = match params.get("model_id") {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "model_id parameter required"})),
            )
                .into_response();
        }
    };

    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;

        // Get model info (simplified - in real implementation would fetch from model registry)
        let model_info = ModelInfo {
            name: model_id.clone(),
            version: "1.0".to_string(),
            description: None,
            parameters: 1000000, // Placeholder
            gpu_requirements: crate::core::model_interface::GpuRequirements {
                min_memory_mb: 1000,
                recommended_memory_mb: 2000,
                supported_architectures: vec!["CUDA".to_string()],
                requires_cuda: true,
            },
        };

        match manager.get_placement_previews(model_id, &model_info).await {
            Ok(placements) => {
                let previews: Vec<InstancePreview> = placements
                    .into_iter()
                    .map(|placement| InstancePreview {
                        model_id: model_id.clone(),
                        sharding: format!("{:?}", placement.strategy),
                        instance_meta: "MlxRing".to_string(), // Placeholder
                        instance: serde_json::json!({
                            "strategy": format!("{:?}", placement.strategy),
                            "node_ids": placement.node_ids,
                        }),
                        memory_delta_by_node: placement
                            .memory_by_node
                            .iter()
                            .map(|(k, v)| (k.clone(), *v as i64))
                            .collect(),
                        error: placement.error,
                    })
                    .collect();

                let response = InstancePreviewResponse { previews };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for POST /api/v1/instance
/// Creates a new model instance
async fn instance_create_handler(
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreateInstanceRequest>,
) -> impl IntoResponse {
    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;

        // Parse placement from request (simplified)
        let placement = InstancePlacement {
            strategy: crate::runtime::instance::PlacementStrategy::Single,
            node_ids: vec!["local".to_string()],
            memory_by_node: {
                let mut map = HashMap::new();
                map.insert("local".to_string(), 2000);
                map
            },
            memory_delta: 2000,
            error: None,
        };

        // Extract model_id from instance JSON (simplified)
        let model_id = request
            .instance
            .get("model_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        match manager
            .create_instance(model_id, placement, HashMap::new())
            .await
        {
            Ok(instance_id) => {
                let command_id = uuid::Uuid::new_v4().to_string();
                let response = CreateInstanceResponse {
                    message: "Command received.".to_string(),
                    command_id,
                    instance_id,
                };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/instance
/// Lists all instances
async fn instance_list_handler() -> impl IntoResponse {
    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;
        let instances = manager.list_instances().await;

        let instance_infos: Vec<InstanceInfo> = instances
            .into_iter()
            .map(|instance| {
                let status = instance.status.read().await.clone();
                InstanceInfo {
                    instance_id: instance.instance_id.clone(),
                    model_id: instance.model_id.clone(),
                    status: format!("{:?}", status),
                    created_at: instance.created_at.to_rfc3339(),
                    placement: instance.placement.clone(),
                }
            })
            .collect();

        let response = InstanceListResponse {
            instances: instance_infos,
        };
        (StatusCode::OK, Json(response)).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/instance/:id
/// Gets a specific instance
async fn instance_get_handler(Path(id): Path<String>) -> impl IntoResponse {
    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;

        if let Some(instance) = manager.get_instance(&id).await {
            let status = instance.status.read().await.clone();
            let info = InstanceInfo {
                instance_id: instance.instance_id,
                model_id: instance.model_id,
                status: format!("{:?}", status),
                created_at: instance.created_at.to_rfc3339(),
                placement: instance.placement,
            };
            (StatusCode::OK, Json(info)).into_response()
        } else {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Instance not found"})),
            )
                .into_response()
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for DELETE /api/v1/instance/:id
/// Deletes an instance
async fn instance_delete_handler(
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;

        match manager.delete_instance(&id).await {
            Ok(_) => (
                StatusCode::OK,
                Json(serde_json::json!({"message": "Instance deleted"})),
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}

/// Handler for GET /api/v1/state
/// Returns deployment state (instances and their status)
async fn state_handler() -> impl IntoResponse {
    if let Some(manager_arc) = get_global_instance_manager() {
        let manager = manager_arc.read().await;
        let instances = manager.list_instances().await;

        let state: HashMap<String, serde_json::Value> = instances
            .into_iter()
            .map(|instance| {
                let status = instance.status.read().await.clone();
                (
                    instance.instance_id.clone(),
                    serde_json::json!({
                        "model_id": instance.model_id,
                        "status": format!("{:?}", status),
                        "created_at": instance.created_at.to_rfc3339(),
                    }),
                )
            })
            .collect();

        (StatusCode::OK, Json(state)).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Instance manager not initialized"})),
        )
            .into_response()
    }
}
