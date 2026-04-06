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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::error::ErrorContext;
use crate::core::model_interface::{GpuRequirements, ModelInfo};
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::network::auth::Claims;
use crate::runtime::instance::{InstancePlacement, PlacementStrategy};

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
    placement: InstancePlacementInfo,
}

/// JSON-friendly placement (admin UI expects `strategy` as a string, not an enum object).
#[derive(Serialize)]
struct InstancePlacementInfo {
    strategy: String,
    node_ids: Vec<String>,
    memory_by_node: HashMap<String, u64>,
    memory_delta: i64,
    error: Option<String>,
}

fn instance_placement_info(p: &InstancePlacement) -> InstancePlacementInfo {
    let strategy = match p.strategy {
        PlacementStrategy::Single => "single",
        PlacementStrategy::Pipeline => "pipeline",
        PlacementStrategy::Tensor => "tensor",
    };
    InstancePlacementInfo {
        strategy: strategy.to_string(),
        node_ids: p.node_ids.clone(),
        memory_by_node: p.memory_by_node.clone(),
        memory_delta: p.memory_delta,
        error: p.error.clone(),
    }
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

/// Handler for GET /api/v1/instance/previews?model_id=xxx
/// Returns placement previews for a model
async fn instance_previews_handler(
    State(ctx): State<ApiContext>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let model_id = match params.get("model_id") {
        Some(id) => id,
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

    if let Some(manager_arc) = ctx.instance_manager.get() {
        let manager = manager_arc.read().await;

        // Get model info - try to fetch from library manager, fallback to default
        let model_info = get_model_info(model_id, &ctx).await;

        match manager.get_placement_previews(model_id, &model_info).await {
            Ok(placements) => {
                let previews: Vec<InstancePreview> = placements
                    .into_iter()
                    .map(|placement| InstancePreview {
                        model_id: model_id.clone(),
                        sharding: format!("{:?}", placement.strategy),
                        instance_meta: format!(
                            "Placement: {:?}, Nodes: {:?}",
                            placement.strategy, placement.node_ids
                        ),
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
            Err(e) => {
                let (s, j) = api_json_error(
                    "INTERNAL_ERROR",
                    e.to_string(),
                    Some(
                        ErrorContext::new("instance_previews").with_resource("model_id", model_id),
                    ),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                (s, Json(j.0)).into_response()
            }
        }
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(
                ErrorContext::new("instance_previews").with_resource("instance_manager", "default"),
            ),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Handler for POST /api/v1/instance
/// Creates a new model instance
async fn instance_create_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreateInstanceRequest>,
) -> impl IntoResponse {
    if let Some(manager_arc) = ctx.instance_manager.get() {
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
            Err(e) => {
                let (s, j) = api_json_error(
                    "INTERNAL_ERROR",
                    e.to_string(),
                    Some(ErrorContext::new("create_instance")),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                (s, Json(j.0)).into_response()
            }
        }
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(ErrorContext::new("create_instance").with_resource("instance_manager", "default")),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Handler for GET /api/v1/instance
/// Lists all instances
async fn instance_list_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    if let Some(manager_arc) = ctx.instance_manager.get() {
        let manager = manager_arc.read().await;
        let instances = manager.list_instances().await;

        // Collect instance infos with async status reading
        let mut instance_infos = Vec::new();
        for instance in instances {
            let status = instance.status.read().await.clone();
            instance_infos.push(InstanceInfo {
                instance_id: instance.instance_id.clone(),
                model_id: instance.model_id.clone(),
                status: format!("{:?}", status),
                created_at: instance.created_at.to_rfc3339(),
                placement: instance_placement_info(&instance.placement),
            });
        }

        let response = InstanceListResponse {
            instances: instance_infos,
        };
        (StatusCode::OK, Json(response)).into_response()
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(ErrorContext::new("list_instances").with_resource("instance_manager", "default")),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Handler for GET /api/v1/instance/:id
/// Gets a specific instance
async fn instance_get_handler(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Some(manager_arc) = ctx.instance_manager.get() {
        let manager = manager_arc.read().await;

        if let Some(instance) = manager.get_instance(&id).await {
            let status = instance.status.read().await.clone();
            let info = InstanceInfo {
                instance_id: instance.instance_id,
                model_id: instance.model_id,
                status: format!("{:?}", status),
                created_at: instance.created_at.to_rfc3339(),
                placement: instance_placement_info(&instance.placement),
            };
            (StatusCode::OK, Json(info)).into_response()
        } else {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                "Instance not found",
                Some(ErrorContext::new("get_instance").with_resource("instance_id", &id)),
                StatusCode::NOT_FOUND,
            );
            (s, Json(j.0)).into_response()
        }
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(ErrorContext::new("get_instance").with_resource("instance_manager", "default")),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Handler for DELETE /api/v1/instance/:id
/// Deletes an instance
async fn instance_delete_handler(
    State(ctx): State<ApiContext>,
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Some(manager_arc) = ctx.instance_manager.get() {
        let manager = manager_arc.read().await;

        match manager.delete_instance(&id).await {
            Ok(_) => (
                StatusCode::OK,
                Json(serde_json::json!({"message": "Instance deleted"})),
            )
                .into_response(),
            Err(e) => {
                let (s, j) = api_json_error(
                    "INTERNAL_ERROR",
                    e.to_string(),
                    Some(ErrorContext::new("delete_instance").with_resource("instance_id", &id)),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                (s, Json(j.0)).into_response()
            }
        }
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(ErrorContext::new("delete_instance").with_resource("instance_manager", "default")),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Handler for GET /api/v1/state
/// Returns deployment state (instances and their status)
async fn state_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    if let Some(manager_arc) = ctx.instance_manager.get() {
        let manager = manager_arc.read().await;
        let instances = manager.list_instances().await;

        // Collect state with async status reading
        let mut state = HashMap::new();
        for instance in instances {
            let status = instance.status.read().await.clone();
            state.insert(
                instance.instance_id.clone(),
                serde_json::json!({
                    "model_id": instance.model_id,
                    "status": format!("{:?}", status),
                    "created_at": instance.created_at.to_rfc3339(),
                }),
            );
        }

        (StatusCode::OK, Json(state)).into_response()
    } else {
        let (s, j) = api_json_error(
            "SUBSYSTEM_UNAVAILABLE",
            "Instance manager not initialized",
            Some(ErrorContext::new("state").with_resource("instance_manager", "default")),
            StatusCode::SERVICE_UNAVAILABLE,
        );
        (s, Json(j.0)).into_response()
    }
}

/// Get model information from library manager or return default
async fn get_model_info(model_id: &str, ctx: &ApiContext) -> ModelInfo {
    // Try to get model info from library manager
    if let Some(lib_manager_arc) = ctx.library_manager.get() {
        let lib_manager = lib_manager_arc.read().await;
        if let Some(library) = lib_manager.get_library(model_id).await {
            // Convert LibraryInfo to ModelInfo
            let size_mb = library.metadata.size_bytes.unwrap_or(0) / (1024 * 1024);
            return ModelInfo {
                name: library.name.clone(),
                version: library.version.clone(),
                capabilities: vec!["text-generation".to_string()], // Default capabilities
                max_tokens: 2048,                                  // Default
                supported_parameters: vec!["temperature".to_string(), "max_tokens".to_string()],
                model_size_mb: size_mb.max(1),
                supported_languages: vec!["en".to_string()],
                gpu_requirements: GpuRequirements {
                    min_memory_mb: (size_mb / 2).max(512), // Estimate: half of model size
                    recommended_memory_mb: size_mb.max(1000),
                    supported_architectures: vec!["CUDA".to_string(), "CPU".to_string()],
                    requires_cuda: size_mb > 1000, // Large models typically need CUDA
                },
            };
        }
    }

    // Fallback to default model info
    ModelInfo {
        name: model_id.to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string(), "max_tokens".to_string()],
        model_size_mb: 2000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 1000,
            recommended_memory_mb: 2000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: true,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn instance_placement_info_strategy_is_json_string() {
        let p = InstancePlacement {
            strategy: PlacementStrategy::Pipeline,
            node_ids: vec!["a".into()],
            memory_by_node: HashMap::new(),
            memory_delta: 0,
            error: None,
        };
        let v = serde_json::to_value(instance_placement_info(&p)).unwrap();
        assert_eq!(v["strategy"], json!("pipeline"));
    }
}
