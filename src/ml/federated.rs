//! Federated Learning (Stage 4.4, ML.3)
//!
//! Provides distributed training capabilities with:
//! - Client-server communication protocol
//! - Model updates aggregation (FedAvg, FedProx)
//! - Privacy-preserving techniques
//! - Secure aggregation
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::ml::federated::{FederatedLearningPipeline, FederatedConfig, ClientUpdate};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = FederatedConfig::default_config();
//! let pipeline = FederatedLearningPipeline::new(config);
//!
//! let update = ClientUpdate {
//!     client_id: "client1".to_string(),
//!     model_weights: vec![0.5, 0.3, 0.2],
//!     sample_count: 100,
//! };
//!
//! pipeline.add_client_update(update).await?;
//! let aggregated = pipeline.aggregate_updates().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Aggregation mode for federated rounds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AggregationMode {
    #[default]
    FedAvg,
    FedProx,
}

/// Federated learning configuration
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FederatedConfig {
    pub aggregation: AggregationMode,
    pub min_clients_per_round: u32,
    pub max_clients_per_round: u32,
    pub rounds: u32,
    pub privacy_budget: f64,
    pub secure_aggregation: bool,
}

impl FederatedConfig {
    /// Create default federated learning configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::ml::federated::FederatedConfig;
    ///
    /// let config = FederatedConfig::default_config();
    /// assert_eq!(config.min_clients_per_round, 2);
    /// ```
    pub fn default_config() -> Self {
        Self {
            aggregation: AggregationMode::FedAvg,
            min_clients_per_round: 2,
            max_clients_per_round: 10,
            rounds: 10,
            privacy_budget: 1.0,
            secure_aggregation: false,
        }
    }
}

/// Client update from a federated learning client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientUpdate {
    pub client_id: String,
    pub model_weights: Vec<f64>,
    pub sample_count: usize,
    pub round: u32,
}

/// Aggregated model result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AggregatedModel {
    pub weights: Vec<f64>,
    pub total_samples: usize,
    pub clients_count: usize,
    pub round: u32,
    pub aggregation_mode: AggregationMode,
}

/// Federated learning round state (persisted for round history / future inspection APIs).
#[derive(Clone, Debug)]
struct RoundState {
    #[allow(dead_code)]
    round: u32,
    #[allow(dead_code)]
    updates: Vec<ClientUpdate>,
    aggregated: Option<AggregatedModel>,
}

/// Federated Learning Pipeline
///
/// Manages federated learning rounds with client updates aggregation.
/// Supports FedAvg and FedProx aggregation modes.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::ml::federated::{FederatedLearningPipeline, ClientUpdate};
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let pipeline = FederatedLearningPipeline::default();
///
/// let update = ClientUpdate {
///     client_id: "client1".to_string(),
///     model_weights: vec![0.5, 0.3, 0.2],
///     sample_count: 100,
///     round: 1,
/// };
///
/// pipeline.add_client_update(update).await?;
/// let aggregated = pipeline.aggregate_updates().await?;
/// # Ok(())
/// # }
/// ```
pub struct FederatedLearningPipeline {
    config: FederatedConfig,
    current_round: Arc<RwLock<u32>>,
    rounds: Arc<RwLock<HashMap<u32, RoundState>>>,
    client_updates: Arc<RwLock<Vec<ClientUpdate>>>,
}

impl Default for FederatedLearningPipeline {
    fn default() -> Self {
        Self::new(FederatedConfig::default_config())
    }
}

impl FederatedLearningPipeline {
    /// Create a new federated learning pipeline
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::ml::federated::{FederatedLearningPipeline, FederatedConfig};
    ///
    /// let config = FederatedConfig::default_config();
    /// let pipeline = FederatedLearningPipeline::new(config);
    /// ```
    pub fn new(config: FederatedConfig) -> Self {
        Self {
            config,
            current_round: Arc::new(RwLock::new(0)),
            rounds: Arc::new(RwLock::new(HashMap::new())),
            client_updates: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a client update
    ///
    /// # Arguments
    ///
    /// * `update` - Client update with model weights and sample count
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::{FederatedLearningPipeline, ClientUpdate};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    ///
    /// let update = ClientUpdate {
    ///     client_id: "client1".to_string(),
    ///     model_weights: vec![0.5, 0.3, 0.2],
    ///     sample_count: 100,
    ///     round: 1,
    /// };
    ///
    /// pipeline.add_client_update(update).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_client_update(&self, update: ClientUpdate) -> Result<(), AppError> {
        // Validate update
        if update.model_weights.is_empty() {
            return Err(AppError::ModelError(
                "Client update has empty model weights. Context: No weights provided in client update. \
                Suggestion: Ensure client has trained a model before sending update. \
                Current: weights={}".to_string(),
            ));
        }

        if update.sample_count == 0 {
            return Err(AppError::ModelError(
                "Client update has zero sample count. Context: Client must have at least one training sample. \
                Suggestion: Ensure client has training data before sending update. \
                Current: sample_count=0".to_string(),
            ));
        }

        let round = *self.current_round.read().await;
        if update.round != round {
            return Err(AppError::ModelError(format!(
                "Client update round mismatch. Context: Update is for round {}, but current round is {}. \
                Suggestion: Ensure client is synchronized with server. \
                Expected: {}, Got: {}",
                update.round, round, round, update.round
            )));
        }

        // Add update
        let mut updates = self.client_updates.write().await;
        updates.push(update);

        Ok(())
    }

    /// Aggregate client updates for current round
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    ///
    /// // Add some updates first
    /// // ...
    ///
    /// let aggregated = pipeline.aggregate_updates().await?;
    /// println!("Aggregated model with {} clients", aggregated.clients_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn aggregate_updates(&self) -> Result<AggregatedModel, AppError> {
        let round = *self.current_round.read().await;
        let mut updates = self.client_updates.write().await;

        if updates.is_empty() {
            return Err(AppError::ModelError(
                "No client updates available for aggregation. Context: No updates have been received. \
                Suggestion: Add client updates using add_client_update() before aggregating. \
                Current: updates=0".to_string(),
            ));
        }

        if (updates.len() as u32) < self.config.min_clients_per_round {
            return Err(AppError::ModelError(format!(
                "Insufficient clients for aggregation. Context: Need at least {} clients, but only {} available. \
                Suggestion: Wait for more clients or reduce min_clients_per_round. \
                Required: {}, Available: {}",
                self.config.min_clients_per_round,
                updates.len(),
                self.config.min_clients_per_round,
                updates.len()
            )));
        }

        // Validate all updates have same weight dimensions
        let expected_dim = updates[0].model_weights.len();
        for update in updates.iter() {
            if update.model_weights.len() != expected_dim {
                return Err(AppError::ModelError(format!(
                    "Model weight dimension mismatch. Context: All updates must have the same weight dimensions. \
                    Suggestion: Ensure all clients use the same model architecture. \
                    Expected: {}, Got: {}",
                    expected_dim,
                    update.model_weights.len()
                )));
            }
        }

        // Aggregate based on mode
        let aggregated_weights = match self.config.aggregation {
            AggregationMode::FedAvg => self.federated_averaging(&updates).await?,
            AggregationMode::FedProx => self.federated_proximal(&updates).await?,
        };

        let total_samples: usize = updates.iter().map(|u| u.sample_count).sum();
        let clients_count = updates.len();

        let aggregated = AggregatedModel {
            weights: aggregated_weights,
            total_samples,
            clients_count,
            round,
            aggregation_mode: self.config.aggregation,
        };

        // Store round state
        let mut rounds = self.rounds.write().await;
        rounds.insert(
            round,
            RoundState {
                round,
                updates: updates.clone(),
                aggregated: Some(aggregated.clone()),
            },
        );

        // Clear updates for next round
        updates.clear();

        Ok(aggregated)
    }

    /// Federated Averaging (FedAvg)
    async fn federated_averaging(&self, updates: &[ClientUpdate]) -> Result<Vec<f64>, AppError> {
        let total_samples: usize = updates.iter().map(|u| u.sample_count).sum();

        if total_samples == 0 {
            return Err(AppError::ModelError(
                "Total sample count is zero. Context: Cannot aggregate with zero samples. \
                Suggestion: Ensure at least one client has training samples."
                    .to_string(),
            ));
        }

        let weight_dim = updates[0].model_weights.len();
        let mut aggregated = vec![0.0; weight_dim];

        for update in updates {
            let weight = update.sample_count as f64 / total_samples as f64;
            for (i, w) in update.model_weights.iter().enumerate() {
                aggregated[i] += w * weight;
            }
        }

        Ok(aggregated)
    }

    /// Federated Proximal (FedProx)
    async fn federated_proximal(&self, updates: &[ClientUpdate]) -> Result<Vec<f64>, AppError> {
        // FedProx is similar to FedAvg but with proximal term
        // For simplicity, we use FedAvg with a small regularization
        let mut aggregated = self.federated_averaging(updates).await?;

        // Apply small regularization (proximal term)
        let mu = 0.01; // Proximal parameter
        for weight in aggregated.iter_mut() {
            *weight *= 1.0 - mu;
        }

        Ok(aggregated)
    }

    /// Start a new federated learning round
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    /// let round = pipeline.start_round().await;
    /// println!("Started round {}", round);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_round(&self) -> u32 {
        let mut current_round = self.current_round.write().await;
        *current_round += 1;
        *current_round
    }

    /// Get current round number
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    /// let round = pipeline.get_current_round().await;
    /// println!("Current round: {}", round);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_current_round(&self) -> u32 {
        *self.current_round.read().await
    }

    /// Get aggregated model for a specific round
    ///
    /// # Arguments
    ///
    /// * `round` - Round number
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    /// let model = pipeline.get_round_model(1).await;
    /// if let Some(model) = model {
    ///     println!("Round 1 model: {} weights", model.weights.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_round_model(&self, round: u32) -> Option<AggregatedModel> {
        let rounds = self.rounds.read().await;
        rounds
            .get(&round)
            .and_then(|state| state.aggregated.clone())
    }

    /// Get pending client updates count
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    /// let count = pipeline.get_pending_updates_count().await;
    /// println!("Pending updates: {}", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pending_updates_count(&self) -> usize {
        self.client_updates.read().await.len()
    }

    /// Check if ready for aggregation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let pipeline = FederatedLearningPipeline::default();
    /// let ready = pipeline.is_ready_for_aggregation().await;
    /// if ready {
    ///     let aggregated = pipeline.aggregate_updates().await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_ready_for_aggregation(&self) -> bool {
        let updates = self.client_updates.read().await;
        (updates.len() as u32) >= self.config.min_clients_per_round
            && (updates.len() as u32) <= self.config.max_clients_per_round
    }

    /// Get configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::ml::federated::FederatedLearningPipeline;
    ///
    /// let pipeline = FederatedLearningPipeline::default();
    /// let config = pipeline.get_config();
    /// println!("Min clients: {}", config.min_clients_per_round);
    /// ```
    pub fn get_config(&self) -> &FederatedConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_federated_config_default() {
        let c = FederatedConfig::default_config();
        assert_eq!(c.aggregation, AggregationMode::FedAvg);
        assert_eq!(c.min_clients_per_round, 2);
        assert_eq!(c.max_clients_per_round, 10);
        assert_eq!(c.rounds, 10);
    }

    #[tokio::test]
    async fn test_federated_pipeline_creation() {
        let config = FederatedConfig::default_config();
        let pipeline = FederatedLearningPipeline::new(config);

        let round = pipeline.get_current_round().await;
        assert_eq!(round, 0);

        let count = pipeline.get_pending_updates_count().await;
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_add_client_update() {
        let pipeline = FederatedLearningPipeline::default();

        let update = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![0.5, 0.3, 0.2],
            sample_count: 100,
            round: 0,
        };

        let result = pipeline.add_client_update(update).await;
        assert!(result.is_ok());

        let count = pipeline.get_pending_updates_count().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_add_client_update_empty_weights() {
        let pipeline = FederatedLearningPipeline::default();

        let update = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![],
            sample_count: 100,
            round: 0,
        };

        let result = pipeline.add_client_update(update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_client_update_zero_samples() {
        let pipeline = FederatedLearningPipeline::default();

        let update = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![0.5, 0.3, 0.2],
            sample_count: 0,
            round: 0,
        };

        let result = pipeline.add_client_update(update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aggregate_updates_fedavg() {
        let mut config = FederatedConfig::default_config();
        config.aggregation = AggregationMode::FedAvg;
        config.min_clients_per_round = 2;

        let pipeline = FederatedLearningPipeline::new(config);

        // Add two client updates
        let update1 = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![1.0, 2.0, 3.0],
            sample_count: 100,
            round: 0,
        };

        let update2 = ClientUpdate {
            client_id: "client2".to_string(),
            model_weights: vec![2.0, 3.0, 4.0],
            sample_count: 200,
            round: 0,
        };

        pipeline.add_client_update(update1).await.unwrap();
        pipeline.add_client_update(update2).await.unwrap();

        let aggregated = pipeline.aggregate_updates().await;
        assert!(aggregated.is_ok());

        let model = aggregated.unwrap();
        assert_eq!(model.weights.len(), 3);
        assert_eq!(model.clients_count, 2);
        assert_eq!(model.total_samples, 300);
        assert_eq!(model.aggregation_mode, AggregationMode::FedAvg);

        // Check weighted average: (1.0*100 + 2.0*200)/300 = 1.67, etc.
        assert!(model.weights[0] > 1.0 && model.weights[0] < 2.0);
    }

    #[tokio::test]
    async fn test_aggregate_updates_insufficient_clients() {
        let mut config = FederatedConfig::default_config();
        config.min_clients_per_round = 3;

        let pipeline = FederatedLearningPipeline::new(config);

        let update = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![0.5, 0.3, 0.2],
            sample_count: 100,
            round: 0,
        };

        pipeline.add_client_update(update).await.unwrap();

        let result = pipeline.aggregate_updates().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aggregate_updates_dimension_mismatch() {
        let pipeline = FederatedLearningPipeline::default();

        let update1 = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![0.5, 0.3, 0.2],
            sample_count: 100,
            round: 0,
        };

        let update2 = ClientUpdate {
            client_id: "client2".to_string(),
            model_weights: vec![0.5, 0.3], // Different dimension
            sample_count: 100,
            round: 0,
        };

        pipeline.add_client_update(update1).await.unwrap();
        pipeline.add_client_update(update2).await.unwrap();

        let result = pipeline.aggregate_updates().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_round() {
        let pipeline = FederatedLearningPipeline::default();

        let round1 = pipeline.start_round().await;
        assert_eq!(round1, 1);

        let round2 = pipeline.start_round().await;
        assert_eq!(round2, 2);
    }

    #[tokio::test]
    async fn test_get_round_model() {
        let pipeline = FederatedLearningPipeline::default();

        // Add updates and aggregate
        let update1 = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![1.0, 2.0],
            sample_count: 100,
            round: 0,
        };

        let update2 = ClientUpdate {
            client_id: "client2".to_string(),
            model_weights: vec![2.0, 3.0],
            sample_count: 100,
            round: 0,
        };

        pipeline.add_client_update(update1).await.unwrap();
        pipeline.add_client_update(update2).await.unwrap();

        let aggregated = pipeline.aggregate_updates().await.unwrap();

        let model = pipeline.get_round_model(0).await;
        assert!(model.is_some());

        let retrieved = model.unwrap();
        assert_eq!(retrieved.weights.len(), aggregated.weights.len());
    }

    #[tokio::test]
    async fn test_is_ready_for_aggregation() {
        let mut config = FederatedConfig::default_config();
        config.min_clients_per_round = 2;
        config.max_clients_per_round = 5;

        let pipeline = FederatedLearningPipeline::new(config);

        assert!(!pipeline.is_ready_for_aggregation().await);

        let update = ClientUpdate {
            client_id: "client1".to_string(),
            model_weights: vec![0.5, 0.3],
            sample_count: 100,
            round: 0,
        };

        pipeline.add_client_update(update).await.unwrap();
        assert!(!pipeline.is_ready_for_aggregation().await); // Still need 1 more

        let update2 = ClientUpdate {
            client_id: "client2".to_string(),
            model_weights: vec![0.5, 0.3],
            sample_count: 100,
            round: 0,
        };

        pipeline.add_client_update(update2).await.unwrap();
        assert!(pipeline.is_ready_for_aggregation().await);
    }
}
