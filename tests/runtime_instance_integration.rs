//! Integration tests for runtime::instance module
//!
//! Tests model instance management, placement strategies, and instance lifecycle.

use poolai::core::model_interface::{GpuRequirements, ModelInfo, ModelRequest};
use poolai::runtime::instance::{
    get_global_instance_manager, initialize_global_instance_manager, InstanceManager,
    InstanceStatus, PlacementStrategy,
};
use std::collections::HashMap;

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
    assert!(matches!(placements[0].strategy, PlacementStrategy::Single));
    assert!(placements[0].error.is_none());
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

    let status = manager.get_instance_status(&instance_id).await;
    assert_eq!(status, Some(InstanceStatus::Ready));

    manager.delete_instance(&instance_id).await.unwrap();

    let instance = manager.get_instance(&instance_id).await;
    assert!(instance.is_none());
}

#[tokio::test]
async fn test_list_instances() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "test-model-1".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 1000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 500,
            recommended_memory_mb: 1000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: false,
        },
    };

    let placements = manager
        .get_placement_previews("test-model-1", &model_info)
        .await
        .unwrap();

    let instance_id1 = manager
        .create_instance("test-model-1".to_string(), placements[0].clone(), HashMap::new())
        .await
        .unwrap();

    let instance_id2 = manager
        .create_instance("test-model-1".to_string(), placements[0].clone(), HashMap::new())
        .await
        .unwrap();

    let instances = manager.list_instances().await;
    assert!(instances.len() >= 2);

    // Cleanup
    manager.delete_instance(&instance_id1).await.unwrap();
    manager.delete_instance(&instance_id2).await.unwrap();
}

#[tokio::test]
async fn test_get_instance_by_model_id() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "unique-model".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 1000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 500,
            recommended_memory_mb: 1000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: false,
        },
    };

    let placements = manager
        .get_placement_previews("unique-model", &model_info)
        .await
        .unwrap();

    let instance_id = manager
        .create_instance("unique-model".to_string(), placements[0].clone(), HashMap::new())
        .await
        .unwrap();

    let instance = manager.get_instance_by_model_id("unique-model").await;
    assert!(instance.is_some());
    assert_eq!(instance.unwrap().model_id, "unique-model");

    // Cleanup
    manager.delete_instance(&instance_id).await.unwrap();
}

#[tokio::test]
async fn test_get_instance_by_model_id_not_found() {
    let manager = InstanceManager::new();
    let instance = manager.get_instance_by_model_id("non-existent-model").await;
    assert!(instance.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_instance() {
    let manager = InstanceManager::new();
    let result = manager.delete_instance("non-existent-instance-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_instance_status_transitions() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "status-test-model".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 1000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 500,
            recommended_memory_mb: 1000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: false,
        },
    };

    let placements = manager
        .get_placement_previews("status-test-model", &model_info)
        .await
        .unwrap();

    let instance_id = manager
        .create_instance("status-test-model".to_string(), placements[0].clone(), HashMap::new())
        .await
        .unwrap();

    // Instance should be Ready after creation
    let status = manager.get_instance_status(&instance_id).await;
    assert_eq!(status, Some(InstanceStatus::Ready));

    // Cleanup
    manager.delete_instance(&instance_id).await.unwrap();
}

#[tokio::test]
async fn test_placement_strategies() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "placement-test-model".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 5000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 2000,
            recommended_memory_mb: 4000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: true,
        },
    };

    let placements = manager
        .get_placement_previews("placement-test-model", &model_info)
        .await
        .unwrap();

    assert!(!placements.is_empty());

    // Verify placement has correct memory requirements
    let placement = &placements[0];
    assert_eq!(placement.memory_delta, 4000); // recommended_memory_mb
    assert!(!placement.node_ids.is_empty());
}

#[tokio::test]
async fn test_global_instance_manager() {
    // Initialize global instance manager
    initialize_global_instance_manager().unwrap();

    let manager_arc = get_global_instance_manager();
    assert!(manager_arc.is_some());

    let manager = manager_arc.unwrap().read().await;
    let instances = manager.list_instances().await;
    assert_eq!(instances.len(), 0);
}

#[tokio::test]
async fn test_instance_metadata() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "metadata-test-model".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 1000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 500,
            recommended_memory_mb: 1000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: false,
        },
    };

    let placements = manager
        .get_placement_previews("metadata-test-model", &model_info)
        .await
        .unwrap();

    let mut metadata = HashMap::new();
    metadata.insert("env".to_string(), "test".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());

    let instance_id = manager
        .create_instance(
            "metadata-test-model".to_string(),
            placements[0].clone(),
            metadata.clone(),
        )
        .await
        .unwrap();

    let instance = manager.get_instance(&instance_id).await.unwrap();
    assert_eq!(instance.metadata.get("env"), Some(&"test".to_string()));
    assert_eq!(instance.metadata.get("version"), Some(&"1.0".to_string()));

    // Cleanup
    manager.delete_instance(&instance_id).await.unwrap();
}

#[tokio::test]
async fn test_process_request_via_instance_without_model() {
    let manager = InstanceManager::new();
    let model_info = ModelInfo {
        name: "no-model-test".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["text-generation".to_string()],
        max_tokens: 2048,
        supported_parameters: vec!["temperature".to_string()],
        model_size_mb: 1000,
        supported_languages: vec!["en".to_string()],
        gpu_requirements: GpuRequirements {
            min_memory_mb: 500,
            recommended_memory_mb: 1000,
            supported_architectures: vec!["CUDA".to_string()],
            requires_cuda: false,
        },
    };

    let placements = manager
        .get_placement_previews("no-model-test", &model_info)
        .await
        .unwrap();

    let instance_id = manager
        .create_instance("no-model-test".to_string(), placements[0].clone(), HashMap::new())
        .await
        .unwrap();

    // Attempt to process request without loaded model should fail
    let request = ModelRequest {
        input: "test input".to_string(),
        parameters: Default::default(),
        session_id: None,
        priority: 5,
        timeout: Some(30),
    };

    let result = manager.process_request_via_instance(&instance_id, request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Model not loaded"));

    // Cleanup
    manager.delete_instance(&instance_id).await.unwrap();
}
