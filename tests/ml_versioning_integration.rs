//! Integration tests for Model Versioning Module (ML.4)
//!
//! Tests the model versioning functionality including registration, retrieval,
//! comparison, tagging, and model listing.

use poolai::ml::versioning::{ModelMetadata, ModelVersion, ModelVersionManager, VersionComparison};
use std::collections::HashMap;

#[tokio::test]
async fn test_model_version_manager_creation() {
    let manager = ModelVersionManager::new();
    let models = manager.list_models().await;
    assert!(models.is_empty());
}

#[tokio::test]
async fn test_register_model_basic() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: Some("Best model".to_string()),
    };

    let version = manager.register_model("model1", metadata).await.unwrap();
    assert_eq!(version.version, "v1");
    assert_eq!(version.model_id, "model1");
    assert_eq!(version.metadata.accuracy, 0.95);
}

#[tokio::test]
async fn test_register_model_multiple_versions() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    let v1 = manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    let v2 = manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    let v3 = manager.register_model("model1", metadata).await.unwrap();

    assert_eq!(v1.version, "v1");
    assert_eq!(v2.version, "v2");
    assert_eq!(v3.version, "v3");
}

#[tokio::test]
async fn test_register_model_invalid_accuracy_high() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 1.5, // Invalid (> 1.0)
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    let result = manager.register_model("model1", metadata).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_model_invalid_accuracy_negative() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: -0.1, // Invalid (< 0.0)
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    let result = manager.register_model("model1", metadata).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_model_empty_id() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    let result = manager.register_model("", metadata).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_version() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "RandomForest".to_string(),
        accuracy: 0.88,
        training_time_ms: 500,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();

    let version: ModelVersion = manager.get_version("model1", "v1").await.unwrap();
    assert_eq!(version.version, "v1");
    assert_eq!(version.model_id, "model1");
    assert_eq!(version.metadata.accuracy, 0.88);
}

#[tokio::test]
async fn test_get_version_not_found_model() {
    let manager = ModelVersionManager::new();

    let result = manager.get_version("nonexistent", "v1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_version_not_found_version() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();

    let result = manager.get_version("model1", "v999").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_latest_version() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager.register_model("model1", metadata).await.unwrap();

    let latest: ModelVersion = manager.get_latest_version("model1").await.unwrap();
    assert_eq!(latest.version, "v3");
}

#[tokio::test]
async fn test_get_latest_version_not_found() {
    let manager = ModelVersionManager::new();

    let result = manager.get_latest_version("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_versions() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager.register_model("model1", metadata).await.unwrap();

    let versions: Vec<ModelVersion> = manager.list_versions("model1").await.unwrap();
    assert_eq!(versions.len(), 3);
    assert_eq!(versions[0].version, "v1");
    assert_eq!(versions[1].version, "v2");
    assert_eq!(versions[2].version, "v3");
}

#[tokio::test]
async fn test_list_versions_not_found() {
    let manager = ModelVersionManager::new();

    let result = manager.list_versions("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_compare_versions_same() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();

    let comparison = manager
        .compare_versions("model1", "v1", "v1")
        .await
        .unwrap();
    assert_eq!(comparison, VersionComparison::Same);
}

#[tokio::test]
async fn test_compare_versions_older_newer() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    manager.register_model("model1", metadata).await.unwrap();

    let comparison = manager
        .compare_versions("model1", "v1", "v2")
        .await
        .unwrap();
    assert_eq!(comparison, VersionComparison::Older);

    let comparison2 = manager
        .compare_versions("model1", "v2", "v1")
        .await
        .unwrap();
    assert_eq!(comparison2, VersionComparison::Newer);
}

#[tokio::test]
async fn test_add_tags() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();
    manager
        .add_tags(
            "model1",
            "v1",
            vec!["production".to_string(), "best".to_string()],
        )
        .await
        .unwrap();

    let version: ModelVersion = manager.get_version("model1", "v1").await.unwrap();
    assert!(version.tags.contains(&"production".to_string()));
    assert!(version.tags.contains(&"best".to_string()));
}

#[tokio::test]
async fn test_add_tags_duplicate() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();
    manager
        .add_tags("model1", "v1", vec!["production".to_string()])
        .await
        .unwrap();
    manager
        .add_tags("model1", "v1", vec!["production".to_string()]) // Duplicate
        .await
        .unwrap();

    let version: ModelVersion = manager.get_version("model1", "v1").await.unwrap();
    let production_count = version.tags.iter().filter(|t| *t == "production").count();
    assert_eq!(production_count, 1); // Should not duplicate
}

#[tokio::test]
async fn test_get_versions_by_tag() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager.register_model("model1", metadata).await.unwrap();

    manager
        .add_tags("model1", "v1", vec!["production".to_string()])
        .await
        .unwrap();
    manager
        .add_tags("model1", "v3", vec!["production".to_string()])
        .await
        .unwrap();

    let versions = manager
        .get_versions_by_tag("model1", "production")
        .await
        .unwrap();
    assert_eq!(versions.len(), 2);
    assert!(versions.iter().any(|v| v.version == "v1"));
    assert!(versions.iter().any(|v| v.version == "v3"));
}

#[tokio::test]
async fn test_get_versions_by_tag_not_found() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager.register_model("model1", metadata).await.unwrap();

    let versions = manager
        .get_versions_by_tag("model1", "nonexistent")
        .await
        .unwrap();
    assert!(versions.is_empty());
}

#[tokio::test]
async fn test_list_models() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    manager
        .register_model("model2", metadata.clone())
        .await
        .unwrap();
    manager.register_model("model3", metadata).await.unwrap();

    let models = manager.list_models().await;
    assert_eq!(models.len(), 3);
    assert!(models.contains(&"model1".to_string()));
    assert!(models.contains(&"model2".to_string()));
    assert!(models.contains(&"model3".to_string()));
}

#[tokio::test]
async fn test_multiple_models_independent_versions() {
    let manager = ModelVersionManager::new();

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters: HashMap::new(),
        description: None,
    };

    let v1_model1 = manager
        .register_model("model1", metadata.clone())
        .await
        .unwrap();
    let v1_model2 = manager.register_model("model2", metadata).await.unwrap();

    assert_eq!(v1_model1.version, "v1");
    assert_eq!(v1_model2.version, "v1"); // Independent versioning
}

#[tokio::test]
async fn test_metadata_with_hyperparameters() {
    let manager = ModelVersionManager::new();

    let mut hyperparameters = HashMap::new();
    hyperparameters.insert("learning_rate".to_string(), "0.001".to_string());
    hyperparameters.insert("batch_size".to_string(), "32".to_string());

    let metadata = ModelMetadata {
        model_type: "NeuralNetwork".to_string(),
        accuracy: 0.95,
        training_time_ms: 1000,
        hyperparameters,
        description: Some("Model with custom hyperparameters".to_string()),
    };

    let version = manager.register_model("model1", metadata).await.unwrap();
    assert_eq!(version.metadata.hyperparameters.len(), 2);
    assert_eq!(
        version.metadata.description,
        Some("Model with custom hyperparameters".to_string())
    );
}
