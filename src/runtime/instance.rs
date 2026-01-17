//! Model instance management
//!
//! This module provides:
//! - Model instance lifecycle management
//! - Instance placement strategies
//! - Instance state tracking
//! - Resource allocation and validation

use crate::core::error::AppError;
use crate::core::model_interface::{ModelInfo, ModelInterface};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// Model instance state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstanceStatus {
    /// Instance is being created
    Creating,
    /// Instance is ready to process requests
    Ready,
    /// Instance is processing requests
    Active,
    /// Instance is stopped
    Stopped,
    /// Instance encountered an error
    Error(String),
    /// Instance is being deleted
    Deleting,
}

/// Instance placement strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlacementStrategy {
    /// Single node placement
    Single,
    /// Pipeline parallelism across nodes
    Pipeline,
    /// Tensor parallelism (sharding)
    Tensor,
}

/// Instance placement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePlacement {
    /// Placement strategy
    pub strategy: PlacementStrategy,
    /// Node IDs where instance is placed
    pub node_ids: Vec<String>,
    /// Memory allocation per node (MB)
    pub memory_by_node: HashMap<String, u64>,
    /// Estimated memory delta
    pub memory_delta: i64,
    /// Error if placement is invalid
    pub error: Option<String>,
}

/// Model instance
#[derive(Clone)]
pub struct ModelInstance {
    /// Unique instance ID
    pub instance_id: String,
    /// Model ID/name
    pub model_id: String,
    /// Instance status
    pub status: Arc<RwLock<InstanceStatus>>,
    /// Placement information
    pub placement: InstancePlacement,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: Arc<RwLock<DateTime<Utc>>>,
    /// Model interface (if loaded) - not included in Debug/Clone
    pub model: Option<Arc<dyn ModelInterface + Send + Sync>>,
    /// Instance metadata
    pub metadata: HashMap<String, String>,
}

impl std::fmt::Debug for ModelInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelInstance")
            .field("instance_id", &self.instance_id)
            .field("model_id", &self.model_id)
            .field("status", &"<Arc<RwLock<InstanceStatus>>>")
            .field("placement", &self.placement)
            .field("created_at", &self.created_at)
            .field("last_activity", &"<Arc<RwLock<DateTime<Utc>>>>")
            .field("model", &self.model.as_ref().map(|_| "<ModelInterface>"))
            .field("metadata", &self.metadata)
            .finish()
    }
}

/// Instance manager
pub struct InstanceManager {
    /// Active instances
    instances: Arc<RwLock<HashMap<String, ModelInstance>>>,
    /// Placement strategy calculator
    placement_calculator: Arc<dyn PlacementCalculator + Send + Sync>,
}

/// Trait for placement calculation
#[async_trait::async_trait]
pub trait PlacementCalculator: Send + Sync {
    /// Calculate placement options for a model
    async fn calculate_placements(
        &self,
        model_id: &str,
        model_info: &ModelInfo,
    ) -> Result<Vec<InstancePlacement>, AppError>;
}

/// Default placement calculator
pub struct DefaultPlacementCalculator;

#[async_trait::async_trait]
impl PlacementCalculator for DefaultPlacementCalculator {
    async fn calculate_placements(
        &self,
        _model_id: &str,
        model_info: &ModelInfo,
    ) -> Result<Vec<InstancePlacement>, AppError> {
        // Simple single-node placement for now
        let memory_required = model_info.gpu_requirements.recommended_memory_mb;

        let placement = InstancePlacement {
            strategy: PlacementStrategy::Single,
            node_ids: vec!["local".to_string()],
            memory_by_node: {
                let mut map = HashMap::new();
                map.insert("local".to_string(), memory_required);
                map
            },
            memory_delta: memory_required as i64,
            error: None,
        };

        Ok(vec![placement])
    }
}

impl InstanceManager {
    /// Create a new instance manager
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            placement_calculator: Arc::new(DefaultPlacementCalculator),
        }
    }

    /// Create a new instance manager with custom placement calculator
    pub fn with_placement_calculator(
        calculator: Arc<dyn PlacementCalculator + Send + Sync>,
    ) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            placement_calculator: calculator,
        }
    }

    /// Get placement previews for a model
    pub async fn get_placement_previews(
        &self,
        model_id: &str,
        model_info: &ModelInfo,
    ) -> Result<Vec<InstancePlacement>, AppError> {
        self.placement_calculator
            .calculate_placements(model_id, model_info)
            .await
    }

    /// Create a new model instance
    pub async fn create_instance(
        &self,
        model_id: String,
        placement: InstancePlacement,
        metadata: HashMap<String, String>,
    ) -> Result<String, AppError> {
        let instance_id = format!("inst-{}", Uuid::new_v4().to_string()[..8].to_string());

        let instance = ModelInstance {
            instance_id: instance_id.clone(),
            model_id: model_id.clone(),
            status: Arc::new(RwLock::new(InstanceStatus::Creating)),
            placement,
            created_at: Utc::now(),
            last_activity: Arc::new(RwLock::new(Utc::now())),
            model: None, // Will be loaded later via load_model_for_instance
            metadata,
        };

        let mut instances = self.instances.write().await;
        instances.insert(instance_id.clone(), instance);

        info!(
            "Created model instance: {} for model: {}",
            instance_id, model_id
        );

        // Try to load model if available (non-blocking - won't fail if model not found)
        let _ = self.load_model_for_instance(&instance_id, &model_id).await;

        // Update status to Ready
        {
            let mut status = instances.get(&instance_id).unwrap().status.write().await;
            *status = InstanceStatus::Ready;
        }

        Ok(instance_id)
    }

    /// Load model for an instance (internal method - tries to find model from ModelManager or LibraryManager)
    async fn load_model_for_instance(
        &self,
        instance_id: &str,
        model_id: &str,
    ) -> Result<(), AppError> {
        // Try to find model in ModelManager first
        if let Some(model_manager) = crate::core::model_interface::get_global_model_manager() {
            let manager = model_manager.read().await;
            if manager.get_model(model_id).is_some() {
                // Model found in ModelManager - we can't clone Box<dyn ModelInterface>
                // So we store a reference that the instance can use
                // Note: In production, we'd need a different approach (e.g., model registry with Arc)
                info!(
                    "Model {} found in ModelManager for instance {}",
                    model_id, instance_id
                );

                // Update instance to mark model as available (though we can't store it directly)
                // The instance will need to query ModelManager when processing requests
                // This is a limitation of the current architecture - models are owned by ModelManager
                return Ok(());
            }
        }

        // Try to find model library in LibraryManager
        if let Some(lib_manager) = crate::libs::get_global_manager() {
            let manager = lib_manager.read().await;
            if let Some(_library) = manager.get_library(model_id).await {
                info!("Model library {} found for instance {} (model loading from library not yet implemented)", model_id, instance_id);
                // TODO: Load model from library (requires ModelInterface implementation for library models)
                // For now, we just note that the library exists
                return Ok(());
            }
        }

        // Model not found - provide detailed error information
        warn!(
            "Model '{}' not found in ModelManager or LibraryManager for instance '{}'. \
            Instance will be created, but model processing requests will fail until model is registered. \
            To resolve: 1) Register model in ModelManager via register_model(), or 2) Install model library via LibraryManager.",
            model_id, instance_id
        );
        Ok(())
    }

    /// Get instance by ID
    pub async fn get_instance(&self, instance_id: &str) -> Option<ModelInstance> {
        let instances = self.instances.read().await;
        instances.get(instance_id).cloned()
    }

    /// List all instances
    pub async fn list_instances(&self) -> Vec<ModelInstance> {
        let instances = self.instances.read().await;
        instances.values().cloned().collect()
    }

    /// Delete an instance
    pub async fn delete_instance(&self, instance_id: &str) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;

        if let Some(instance) = instances.get(instance_id) {
            // Update status to Deleting
            {
                let mut status = instance.status.write().await;
                *status = InstanceStatus::Deleting;
            }

            // Shutdown model if loaded
            if let Some(model) = &instance.model {
                if let Err(e) = model.shutdown().await {
                    warn!(
                        "Error shutting down model for instance {}: {}",
                        instance_id, e
                    );
                }
            }

            instances.remove(instance_id);
            info!("Deleted model instance: {}", instance_id);
            Ok(())
        } else {
            Err(AppError::ResourceError(format!(
                "Instance '{}' not found",
                instance_id
            )))
        }
    }

    /// Get instance status
    pub async fn get_instance_status(&self, instance_id: &str) -> Option<InstanceStatus> {
        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(instance_id) {
            // Read the status from RwLock
            let status = instance.status.read().await;
            Some(status.clone())
        } else {
            None
        }
    }

    /// Get model instance by model ID (for routing completions requests)
    pub async fn get_instance_by_model_id(&self, model_id: &str) -> Option<ModelInstance> {
        let instances = self.instances.read().await;
        instances
            .values()
            .find(|instance| instance.model_id == model_id)
            .cloned()
    }

    /// Process request using an instance's model
    pub async fn process_request_via_instance(
        &self,
        instance_id: &str,
        request: crate::core::model_interface::ModelRequest,
    ) -> Result<crate::core::model_interface::ModelResponse, AppError> {
        let instances = self.instances.read().await;
        let instance = instances.get(instance_id).ok_or_else(|| {
            AppError::ResourceError(format!("Instance '{}' not found", instance_id))
        })?;

        // Try to use instance's model first (if directly stored)
        if let Some(model) = &instance.model {
            // Update last activity
            {
                let mut last_activity = instance.last_activity.write().await;
                *last_activity = Utc::now();
            }

            // Update status to Active
            {
                let mut status = instance.status.write().await;
                if *status != InstanceStatus::Error(String::new()) {
                    *status = InstanceStatus::Active;
                }
            }

            // Process request
            let response = model.process_request(request).await?;

            // Update status back to Ready
            {
                let mut status = instance.status.write().await;
                if *status == InstanceStatus::Active {
                    *status = InstanceStatus::Ready;
                }
            }

            return Ok(response);
        }

        // If model not stored in instance, try to get from ModelManager
        if let Some(model_manager) = crate::core::model_interface::get_global_model_manager() {
            let manager = model_manager.read().await;
            if manager.get_model(&instance.model_id).is_some() {
                // Update last activity
                {
                    let mut last_activity = instance.last_activity.write().await;
                    *last_activity = Utc::now();
                }

                // Update status to Active
                {
                    let mut status = instance.status.write().await;
                    if *status != InstanceStatus::Error(String::new()) {
                        *status = InstanceStatus::Active;
                    }
                }

                // Process request via ModelManager
                let response = manager.process_request(&instance.model_id, request).await?;

                // Update status back to Ready
                {
                    let mut status = instance.status.write().await;
                    if *status == InstanceStatus::Active {
                        *status = InstanceStatus::Ready;
                    }
                }

                return Ok(response);
            }
        }

        Err(AppError::ModelError(format!(
            "Model '{}' not found in instance or ModelManager for instance '{}'",
            instance.model_id, instance_id
        )))
    }
}

impl Default for InstanceManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global instance manager
use std::sync::OnceLock;

static GLOBAL_INSTANCE_MANAGER: OnceLock<Arc<RwLock<InstanceManager>>> = OnceLock::new();

/// Initialize global instance manager
pub fn initialize_global_instance_manager() -> Result<(), AppError> {
    // Try to use topology-aware placement if topology manager is available
    let manager =
        if let Some(_topology_manager) = crate::pool::topology::get_global_topology_manager() {
            // Use topology-aware placement calculator
            let calculator = Arc::new(crate::pool::placement::TopologyAwarePlacementCalculator);
            InstanceManager::with_placement_calculator(calculator)
        } else {
            // Fallback to default placement calculator
            InstanceManager::new()
        };

    GLOBAL_INSTANCE_MANAGER
        .set(Arc::new(RwLock::new(manager)))
        .map_err(|_| AppError::ConfigError("Instance manager already initialized".to_string()))?;
    Ok(())
}

/// Get global instance manager
pub fn get_global_instance_manager() -> Option<&'static Arc<RwLock<InstanceManager>>> {
    GLOBAL_INSTANCE_MANAGER.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model_interface::{GpuRequirements, ModelInfo};

    #[tokio::test]
    async fn test_instance_manager_creation() {
        let manager = InstanceManager::new();
        let instances = manager.list_instances().await;
        assert_eq!(instances.len(), 0);
    }

    #[tokio::test]
    async fn test_get_placement_previews() {
        let manager = InstanceManager::new();
        let model_info = ModelInfo {
            name: "test-model".to_string(),
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
        };

        let placements = manager
            .get_placement_previews("test-model", &model_info)
            .await
            .unwrap();

        assert!(!placements.is_empty());
        assert_eq!(placements[0].strategy, PlacementStrategy::Single);
    }

    #[tokio::test]
    async fn test_create_and_delete_instance() {
        let manager = InstanceManager::new();
        let model_info = ModelInfo {
            name: "test-model".to_string(),
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
        };

        let placements = manager
            .get_placement_previews("test-model", &model_info)
            .await
            .unwrap();

        let instance_id = manager
            .create_instance(
                "test-model".to_string(),
                placements[0].clone(),
                HashMap::new(),
            )
            .await
            .unwrap();

        assert!(!instance_id.is_empty());

        let instance = manager.get_instance(&instance_id).await;
        assert!(instance.is_some());

        manager.delete_instance(&instance_id).await.unwrap();

        let instance = manager.get_instance(&instance_id).await;
        assert!(instance.is_none());
    }
}
