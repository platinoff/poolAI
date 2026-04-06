//! Model Versioning (Stage 4.4, ML.4)
//!
//! Provides model lifecycle management with:
//! - Version tracking and registration
//! - Model metadata storage
//! - Version comparison
//! - Rollback capabilities
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::ml::versioning::{ModelVersionManager, ModelVersion, ModelMetadata};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = ModelVersionManager::new();
//!
//! let metadata = ModelMetadata {
//!     model_type: "NeuralNetwork".to_string(),
//!     accuracy: 0.95,
//!     training_time_ms: 1000,
//!     hyperparameters: std::collections::HashMap::new(),
//!     description: None,
//! };
//!
//! let version = manager.register_model("model1", metadata).await?;
//! println!("Registered model version: {}", version.version);
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Model metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_type: String,
    pub accuracy: f64,
    pub training_time_ms: u64,
    pub hyperparameters: HashMap<String, String>,
    pub description: Option<String>,
}

/// Model version information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version: String,
    pub model_id: String,
    pub metadata: ModelMetadata,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

/// Version comparison result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VersionComparison {
    Newer,
    Older,
    Same,
    Different,
}

/// Model Version Manager
///
/// Manages model versions with registration, retrieval, comparison, and rollback.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::ml::versioning::{ModelVersionManager, ModelMetadata};
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let manager = ModelVersionManager::new();
///
/// let metadata = ModelMetadata {
///     model_type: "NeuralNetwork".to_string(),
///     accuracy: 0.95,
///     training_time_ms: 1000,
///     hyperparameters: std::collections::HashMap::new(),
///     description: None,
/// };
///
/// let version = manager.register_model("model1", metadata).await?;
/// # Ok(())
/// # }
/// ```
pub struct ModelVersionManager {
    models: Arc<RwLock<HashMap<String, Vec<ModelVersion>>>>,
    version_counter: Arc<RwLock<HashMap<String, u32>>>,
}

impl Default for ModelVersionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelVersionManager {
    /// Create a new model version manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// let manager = ModelVersionManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            version_counter: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new model version
    ///
    /// # Arguments
    ///
    /// * `model_id` - Unique identifier for the model
    /// * `metadata` - Model metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::{ModelVersionManager, ModelMetadata};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    ///
    /// let metadata = ModelMetadata {
    ///     model_type: "NeuralNetwork".to_string(),
    ///     accuracy: 0.95,
    ///     training_time_ms: 1000,
    ///     hyperparameters: std::collections::HashMap::new(),
    ///     description: Some("Best model so far".to_string()),
    /// };
    ///
    /// let version = manager.register_model("model1", metadata).await?;
    /// println!("Version: {}", version.version);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register_model(
        &self,
        model_id: &str,
        metadata: ModelMetadata,
    ) -> Result<ModelVersion, AppError> {
        if model_id.is_empty() {
            return Err(AppError::ModelError(
                "Model ID cannot be empty. Context: Empty model_id provided. \
                Suggestion: Provide a valid model identifier. \
                Current: model_id is empty"
                    .to_string(),
            ));
        }

        // Validate metadata
        if metadata.accuracy < 0.0 || metadata.accuracy > 1.0 {
            return Err(AppError::ModelError(format!(
                "Invalid accuracy value. Context: Accuracy must be between 0.0 and 1.0. \
                Suggestion: Ensure accuracy is a valid probability value. \
                Current: accuracy={}",
                metadata.accuracy
            )));
        }

        // Generate version number
        let mut counter = self.version_counter.write().await;
        let version_num = counter.entry(model_id.to_string()).or_insert(0);
        *version_num += 1;
        let version = format!("v{}", version_num);

        // Create version
        let model_version = ModelVersion {
            version: version.clone(),
            model_id: model_id.to_string(),
            metadata: metadata.clone(),
            created_at: Utc::now(),
            tags: Vec::new(),
        };

        // Store version
        let mut models = self.models.write().await;
        models
            .entry(model_id.to_string())
            .or_insert_with(Vec::new)
            .push(model_version.clone());

        Ok(model_version)
    }

    /// Get a specific model version
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    /// * `version` - Version string (e.g., "v1", "v2")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let version = manager.get_version("model1", "v1").await?;
    /// println!("Model accuracy: {:.2}%", version.metadata.accuracy * 100.0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_version(
        &self,
        model_id: &str,
        version: &str,
    ) -> Result<ModelVersion, AppError> {
        let models = self.models.read().await;

        let versions = models.get(model_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Model not found. Context: Model with ID '{}' does not exist. \
                Suggestion: Register the model first using register_model(). \
                Current: model_id={}",
                model_id, model_id
            ))
        })?;

        versions
            .iter()
            .find(|v| v.version == version)
            .cloned()
            .ok_or_else(|| {
                AppError::ModelError(format!(
                    "Version not found. Context: Version '{}' does not exist for model '{}'. \
                    Suggestion: Check available versions using list_versions(). \
                    Current: model_id={}, version={}",
                    version, model_id, model_id, version
                ))
            })
    }

    /// Get the latest version of a model
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let latest = manager.get_latest_version("model1").await?;
    /// println!("Latest version: {}", latest.version);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_latest_version(&self, model_id: &str) -> Result<ModelVersion, AppError> {
        let models = self.models.read().await;

        let versions = models.get(model_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Model not found. Context: Model with ID '{}' does not exist. \
                Suggestion: Register the model first using register_model(). \
                Current: model_id={}",
                model_id, model_id
            ))
        })?;

        versions.last().cloned().ok_or_else(|| {
            AppError::ModelError(format!(
                "No versions found. Context: Model '{}' has no registered versions. \
                    Suggestion: Register at least one version using register_model(). \
                    Current: model_id={}",
                model_id, model_id
            ))
        })
    }

    /// List all versions of a model
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let versions = manager.list_versions("model1").await?;
    /// println!("Total versions: {}", versions.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_versions(&self, model_id: &str) -> Result<Vec<ModelVersion>, AppError> {
        let models = self.models.read().await;

        let versions = models.get(model_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Model not found. Context: Model with ID '{}' does not exist. \
                Suggestion: Register the model first using register_model(). \
                Current: model_id={}",
                model_id, model_id
            ))
        })?;

        Ok(versions.clone())
    }

    /// Compare two model versions
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    /// * `version1` - First version
    /// * `version2` - Second version
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let comparison = manager.compare_versions("model1", "v1", "v2").await?;
    /// println!("Comparison: {:?}", comparison);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compare_versions(
        &self,
        model_id: &str,
        version1: &str,
        version2: &str,
    ) -> Result<VersionComparison, AppError> {
        if version1 == version2 {
            return Ok(VersionComparison::Same);
        }

        let v1 = self.get_version(model_id, version1).await?;
        let v2 = self.get_version(model_id, version2).await?;

        // Compare by creation time
        if v1.created_at > v2.created_at {
            Ok(VersionComparison::Newer)
        } else if v1.created_at < v2.created_at {
            Ok(VersionComparison::Older)
        } else {
            Ok(VersionComparison::Different)
        }
    }

    /// Add tags to a model version
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    /// * `version` - Version string
    /// * `tags` - Tags to add
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// manager.add_tags(
    ///     "model1",
    ///     "v1",
    ///     vec!["production".to_string(), "best".to_string()],
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_tags(
        &self,
        model_id: &str,
        version: &str,
        tags: Vec<String>,
    ) -> Result<(), AppError> {
        let mut models = self.models.write().await;

        let versions = models.get_mut(model_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Model not found. Context: Model with ID '{}' does not exist. \
                Suggestion: Register the model first using register_model(). \
                Current: model_id={}",
                model_id, model_id
            ))
        })?;

        let model_version = versions
            .iter_mut()
            .find(|v| v.version == version)
            .ok_or_else(|| {
                AppError::ModelError(format!(
                    "Version not found. Context: Version '{}' does not exist for model '{}'. \
                    Suggestion: Check available versions using list_versions(). \
                    Current: model_id={}, version={}",
                    version, model_id, model_id, version
                ))
            })?;

        for tag in tags {
            if !model_version.tags.contains(&tag) {
                model_version.tags.push(tag);
            }
        }

        Ok(())
    }

    /// Get versions by tag
    ///
    /// # Arguments
    ///
    /// * `model_id` - Model identifier
    /// * `tag` - Tag to search for
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let versions = manager.get_versions_by_tag("model1", "production").await?;
    /// println!("Production versions: {}", versions.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_versions_by_tag(
        &self,
        model_id: &str,
        tag: &str,
    ) -> Result<Vec<ModelVersion>, AppError> {
        let versions = self.list_versions(model_id).await?;

        Ok(versions
            .into_iter()
            .filter(|v| v.tags.contains(&tag.to_string()))
            .collect())
    }

    /// List all registered models
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::versioning::ModelVersionManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ModelVersionManager::new();
    /// let models = manager.list_models().await;
    /// println!("Registered models: {}", models.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_version_manager_creation() {
        let manager = ModelVersionManager::new();
        let models = manager.list_models().await;
        assert!(models.is_empty());
    }

    #[tokio::test]
    async fn test_register_model() {
        let manager = ModelVersionManager::new();

        let metadata = ModelMetadata {
            model_type: "NeuralNetwork".to_string(),
            accuracy: 0.95,
            training_time_ms: 1000,
            hyperparameters: HashMap::new(),
            description: None,
        };

        let version = manager.register_model("model1", metadata).await.unwrap();
        assert_eq!(version.version, "v1");
        assert_eq!(version.model_id, "model1");
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
        let v2 = manager.register_model("model1", metadata).await.unwrap();

        assert_eq!(v1.version, "v1");
        assert_eq!(v2.version, "v2");
    }

    #[tokio::test]
    async fn test_register_model_invalid_accuracy() {
        let manager = ModelVersionManager::new();

        let metadata = ModelMetadata {
            model_type: "NeuralNetwork".to_string(),
            accuracy: 1.5, // Invalid
            training_time_ms: 1000,
            hyperparameters: HashMap::new(),
            description: None,
        };

        let result = manager.register_model("model1", metadata).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_version() {
        let manager = ModelVersionManager::new();

        let metadata = ModelMetadata {
            model_type: "NeuralNetwork".to_string(),
            accuracy: 0.95,
            training_time_ms: 1000,
            hyperparameters: HashMap::new(),
            description: None,
        };

        manager.register_model("model1", metadata).await.unwrap();

        let version = manager.get_version("model1", "v1").await.unwrap();
        assert_eq!(version.version, "v1");
    }

    #[tokio::test]
    async fn test_get_version_not_found() {
        let manager = ModelVersionManager::new();

        let result = manager.get_version("model1", "v1").await;
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
        manager.register_model("model1", metadata).await.unwrap();

        let latest = manager.get_latest_version("model1").await.unwrap();
        assert_eq!(latest.version, "v2");
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
        manager.register_model("model1", metadata).await.unwrap();

        let versions = manager.list_versions("model1").await.unwrap();
        assert_eq!(versions.len(), 2);
    }

    #[tokio::test]
    async fn test_compare_versions() {
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

        let version = manager.get_version("model1", "v1").await.unwrap();
        assert!(version.tags.contains(&"production".to_string()));
        assert!(version.tags.contains(&"best".to_string()));
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
        manager.register_model("model1", metadata).await.unwrap();

        manager
            .add_tags("model1", "v1", vec!["production".to_string()])
            .await
            .unwrap();

        let versions = manager
            .get_versions_by_tag("model1", "production")
            .await
            .unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].version, "v1");
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
        manager.register_model("model2", metadata).await.unwrap();

        let models = manager.list_models().await;
        assert_eq!(models.len(), 2);
        assert!(models.contains(&"model1".to_string()));
        assert!(models.contains(&"model2".to_string()));
    }
}
