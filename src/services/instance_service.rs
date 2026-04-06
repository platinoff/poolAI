//! Model instance operations for the HTTP API (previews, CRUD, deployment state).

use crate::core::error::AppError;
use crate::core::model_interface::{GpuRequirements, ModelInfo};
use crate::core::state::ApiContext;
use crate::runtime::instance::{InstanceManager, InstancePlacement, PlacementStrategy};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

/// Instance manager is not attached to [`crate::core::state::AppState`].
#[derive(Debug, Clone, Copy)]
struct InstanceManagerUnavailable;

#[derive(Debug)]
pub enum InstanceServiceError {
    ManagerUnavailable,
    Preview(AppError),
    Operation(AppError),
}

/// Instance preview row for `GET /instance/previews`.
#[derive(Debug, Serialize)]
pub struct InstancePreview {
    pub model_id: String,
    pub sharding: String,
    pub instance_meta: String,
    pub instance: serde_json::Value,
    pub memory_delta_by_node: HashMap<String, i64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InstancePreviewResponse {
    pub previews: Vec<InstancePreview>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInstanceRequest {
    pub instance: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CreateInstanceResponse {
    pub message: String,
    pub command_id: String,
    pub instance_id: String,
}

#[derive(Debug, Serialize)]
pub struct InstanceListResponse {
    pub instances: Vec<InstanceInfo>,
}

#[derive(Debug, Serialize)]
pub struct InstanceInfo {
    pub instance_id: String,
    pub model_id: String,
    pub status: String,
    pub created_at: String,
    pub placement: InstancePlacementInfo,
}

/// JSON-friendly placement (admin UI expects `strategy` as a string).
#[derive(Debug, Serialize)]
pub struct InstancePlacementInfo {
    pub strategy: String,
    pub node_ids: Vec<String>,
    pub memory_by_node: HashMap<String, u64>,
    pub memory_delta: i64,
    pub error: Option<String>,
}

pub fn instance_placement_info(p: &InstancePlacement) -> InstancePlacementInfo {
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

pub struct InstanceService;

impl InstanceService {
    fn manager(
        ctx: &ApiContext,
    ) -> Result<Arc<TokioRwLock<InstanceManager>>, InstanceManagerUnavailable> {
        ctx.instance_manager
            .get()
            .cloned()
            .ok_or(InstanceManagerUnavailable)
    }

    pub async fn get_model_info(model_id: &str, ctx: &ApiContext) -> ModelInfo {
        if let Some(lib_manager_arc) = ctx.library_manager.get() {
            let lib_manager = lib_manager_arc.read().await;
            if let Some(library) = lib_manager.get_library(model_id).await {
                let size_mb = library.metadata.size_bytes.unwrap_or(0) / (1024 * 1024);
                return ModelInfo {
                    name: library.name.clone(),
                    version: library.version.clone(),
                    capabilities: vec!["text-generation".to_string()],
                    max_tokens: 2048,
                    supported_parameters: vec!["temperature".to_string(), "max_tokens".to_string()],
                    model_size_mb: size_mb.max(1),
                    supported_languages: vec!["en".to_string()],
                    gpu_requirements: GpuRequirements {
                        min_memory_mb: (size_mb / 2).max(512),
                        recommended_memory_mb: size_mb.max(1000),
                        supported_architectures: vec!["CUDA".to_string(), "CPU".to_string()],
                        requires_cuda: size_mb > 1000,
                    },
                };
            }
        }

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

    pub async fn placement_previews(
        ctx: &ApiContext,
        model_id: &str,
    ) -> Result<InstancePreviewResponse, InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        let model_info = Self::get_model_info(model_id, ctx).await;
        let placements = manager
            .get_placement_previews(model_id, &model_info)
            .await
            .map_err(InstanceServiceError::Preview)?;
        let previews = placements
            .into_iter()
            .map(|placement| InstancePreview {
                model_id: model_id.to_string(),
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
        Ok(InstancePreviewResponse { previews })
    }

    pub async fn create_instance(
        ctx: &ApiContext,
        request: &CreateInstanceRequest,
    ) -> Result<String, InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        let placement = InstancePlacement {
            strategy: PlacementStrategy::Single,
            node_ids: vec!["local".to_string()],
            memory_by_node: {
                let mut map = HashMap::new();
                map.insert("local".to_string(), 2000);
                map
            },
            memory_delta: 2000,
            error: None,
        };
        let model_id = request
            .instance
            .get("model_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        manager
            .create_instance(model_id, placement, HashMap::new())
            .await
            .map_err(InstanceServiceError::Operation)
    }

    pub async fn list_instances(
        ctx: &ApiContext,
    ) -> Result<InstanceListResponse, InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        let instances = manager.list_instances().await;
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
        Ok(InstanceListResponse {
            instances: instance_infos,
        })
    }

    pub async fn get_instance(
        ctx: &ApiContext,
        id: &str,
    ) -> Result<Option<InstanceInfo>, InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        let Some(instance) = manager.get_instance(id).await else {
            return Ok(None);
        };
        let status = instance.status.read().await.clone();
        Ok(Some(InstanceInfo {
            instance_id: instance.instance_id,
            model_id: instance.model_id,
            status: format!("{:?}", status),
            created_at: instance.created_at.to_rfc3339(),
            placement: instance_placement_info(&instance.placement),
        }))
    }

    pub async fn delete_instance(ctx: &ApiContext, id: &str) -> Result<(), InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        manager
            .delete_instance(id)
            .await
            .map_err(InstanceServiceError::Operation)
    }

    pub async fn deployment_state(
        ctx: &ApiContext,
    ) -> Result<HashMap<String, serde_json::Value>, InstanceServiceError> {
        let m = Self::manager(ctx).map_err(|_| InstanceServiceError::ManagerUnavailable)?;
        let manager = m.read().await;
        let instances = manager.list_instances().await;
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
        Ok(state)
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
