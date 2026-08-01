//! Experiment Tracking (Stage 4.4, ML.5)
//!
//! ML experiment management with:
//! - Experiment registration and lifecycle
//! - Metrics tracking (accuracy, loss, custom)
//! - Experiment comparison
//! - Best experiment selection
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::ml::experiments::{ExperimentTracker, Experiment, ExperimentMetrics};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let tracker = ExperimentTracker::new();
//!
//! let mut metrics = ExperimentMetrics::default();
//! metrics.accuracy = 0.95;
//! metrics.loss = 0.05;
//!
//! let exp = tracker.start_experiment("exp1", "NeuralNetwork").await?;
//! tracker.log_metrics(exp.id.as_str(), metrics).await?;
//! tracker.end_experiment(exp.id.as_str()).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Experiment metrics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExperimentMetrics {
    pub accuracy: f64,
    pub loss: f64,
    pub training_time_ms: u64,
    pub custom: HashMap<String, f64>,
}

/// Experiment state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum ExperimentStatus {
    #[default]
    Running,
    Completed,
    Failed,
}

/// ML experiment record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub status: ExperimentStatus,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub metrics: Option<ExperimentMetrics>,
    pub hyperparameters: HashMap<String, String>,
    pub tags: Vec<String>,
}

/// Experiment Tracker
///
/// Tracks ML experiments with metrics, comparison, and best-experiment selection.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
pub struct ExperimentTracker {
    experiments: Arc<RwLock<HashMap<String, Experiment>>>,
}

impl Default for ExperimentTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperimentTracker {
    /// Create a new experiment tracker
    pub fn new() -> Self {
        Self {
            experiments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a new experiment
    ///
    /// # Arguments
    ///
    /// * `name` - Experiment name
    /// * `model_type` - Model type (e.g. "NeuralNetwork", "RandomForest")
    pub async fn start_experiment(
        &self,
        name: &str,
        model_type: &str,
    ) -> Result<Experiment, AppError> {
        if name.is_empty() {
            return Err(AppError::ModelError(
                "Experiment name cannot be empty. Context: Empty name provided. \
                Suggestion: Provide a valid experiment identifier."
                    .to_string(),
            ));
        }

        let id = Uuid::new_v4().to_string();
        let short_id = id[..8].to_string();

        let experiment = Experiment {
            id: short_id.clone(),
            name: name.to_string(),
            model_type: model_type.to_string(),
            status: ExperimentStatus::Running,
            created_at: Utc::now(),
            ended_at: None,
            metrics: None,
            hyperparameters: HashMap::new(),
            tags: Vec::new(),
        };

        let mut experiments = self.experiments.write().await;
        experiments.insert(short_id.clone(), experiment.clone());

        Ok(experiment)
    }

    /// Log metrics for an experiment
    pub async fn log_metrics(
        &self,
        experiment_id: &str,
        metrics: ExperimentMetrics,
    ) -> Result<(), AppError> {
        let mut experiments = self.experiments.write().await;

        let exp = experiments.get_mut(experiment_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Experiment not found. Context: No experiment with id '{}'. \
                Suggestion: Start experiment with start_experiment() first. \
                Current: experiment_id={}",
                experiment_id, experiment_id
            ))
        })?;

        exp.metrics = Some(metrics);
        Ok(())
    }

    /// End an experiment (mark as completed or failed)
    pub async fn end_experiment(&self, experiment_id: &str) -> Result<(), AppError> {
        self.set_experiment_status(experiment_id, ExperimentStatus::Completed)
            .await
    }

    /// Mark experiment as failed
    pub async fn fail_experiment(&self, experiment_id: &str) -> Result<(), AppError> {
        self.set_experiment_status(experiment_id, ExperimentStatus::Failed)
            .await
    }

    async fn set_experiment_status(
        &self,
        experiment_id: &str,
        status: ExperimentStatus,
    ) -> Result<(), AppError> {
        let mut experiments = self.experiments.write().await;

        let exp = experiments.get_mut(experiment_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Experiment not found. Context: No experiment with id '{}'. \
                Suggestion: Start experiment with start_experiment() first. \
                Current: experiment_id={}",
                experiment_id, experiment_id
            ))
        })?;

        exp.status = status;
        exp.ended_at = Some(Utc::now());
        Ok(())
    }

    /// Get experiment by ID
    pub async fn get_experiment(&self, experiment_id: &str) -> Result<Experiment, AppError> {
        let experiments = self.experiments.read().await;

        experiments.get(experiment_id).cloned().ok_or_else(|| {
            AppError::ModelError(format!(
                "Experiment not found. Context: No experiment with id '{}'. \
                Suggestion: Check experiment_id or list_experiments(). \
                Current: experiment_id={}",
                experiment_id, experiment_id
            ))
        })
    }

    /// List all experiments, optionally filtered by status
    pub async fn list_experiments(
        &self,
        status_filter: Option<ExperimentStatus>,
    ) -> Vec<Experiment> {
        let experiments = self.experiments.read().await;

        let mut list: Vec<_> = experiments.values().cloned().collect();

        if let Some(status) = status_filter {
            list.retain(|e| e.status == status);
        }

        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    /// Add hyperparameters to an experiment
    pub async fn add_hyperparameters(
        &self,
        experiment_id: &str,
        params: HashMap<String, String>,
    ) -> Result<(), AppError> {
        let mut experiments = self.experiments.write().await;

        let exp = experiments.get_mut(experiment_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Experiment not found. Context: No experiment with id '{}'. \
                Current: experiment_id={}",
                experiment_id, experiment_id
            ))
        })?;

        for (k, v) in params {
            exp.hyperparameters.insert(k, v);
        }
        Ok(())
    }

    /// Add tags to an experiment
    pub async fn add_tags(&self, experiment_id: &str, tags: Vec<String>) -> Result<(), AppError> {
        let mut experiments = self.experiments.write().await;

        let exp = experiments.get_mut(experiment_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Experiment not found. Context: No experiment with id '{}'. \
                Current: experiment_id={}",
                experiment_id, experiment_id
            ))
        })?;

        for tag in tags {
            if !exp.tags.contains(&tag) {
                exp.tags.push(tag);
            }
        }
        Ok(())
    }

    /// Get best experiment by accuracy (among completed)
    pub async fn get_best_by_accuracy(&self) -> Option<Experiment> {
        let experiments = self.experiments.read().await;

        experiments
            .values()
            .filter(|e| e.status == ExperimentStatus::Completed)
            .filter(|e| e.metrics.is_some())
            .max_by(|a, b| {
                let acc_a = a.metrics.as_ref().map(|m| m.accuracy).unwrap_or(0.0);
                let acc_b = b.metrics.as_ref().map(|m| m.accuracy).unwrap_or(0.0);
                acc_a
                    .partial_cmp(&acc_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    /// Get best experiment by lowest loss (among completed)
    pub async fn get_best_by_loss(&self) -> Option<Experiment> {
        let experiments = self.experiments.read().await;

        experiments
            .values()
            .filter(|e| e.status == ExperimentStatus::Completed)
            .filter(|e| e.metrics.is_some())
            .min_by(|a, b| {
                let loss_a = a.metrics.as_ref().map(|m| m.loss).unwrap_or(f64::MAX);
                let loss_b = b.metrics.as_ref().map(|m| m.loss).unwrap_or(f64::MAX);
                loss_a
                    .partial_cmp(&loss_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    /// Compare two experiments by accuracy
    pub async fn compare_by_accuracy(
        &self,
        id_a: &str,
        id_b: &str,
    ) -> Result<std::cmp::Ordering, AppError> {
        let exp_a = self.get_experiment(id_a).await?;
        let exp_b = self.get_experiment(id_b).await?;

        let acc_a = exp_a.metrics.as_ref().map(|m| m.accuracy).unwrap_or(0.0);
        let acc_b = exp_b.metrics.as_ref().map(|m| m.accuracy).unwrap_or(0.0);

        Ok(acc_a
            .partial_cmp(&acc_b)
            .unwrap_or(std::cmp::Ordering::Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracker_creation() {
        let tracker = ExperimentTracker::new();
        let list = tracker.list_experiments(None).await;
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_start_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker
            .start_experiment("exp1", "NeuralNetwork")
            .await
            .unwrap();
        assert_eq!(exp.name, "exp1");
        assert_eq!(exp.model_type, "NeuralNetwork");
        assert_eq!(exp.status, ExperimentStatus::Running);
        assert!(exp.ended_at.is_none());
    }

    #[tokio::test]
    async fn test_start_experiment_empty_name() {
        let tracker = ExperimentTracker::new();
        let result = tracker.start_experiment("", "NeuralNetwork").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_metrics() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        let mut metrics = ExperimentMetrics::default();
        metrics.accuracy = 0.95;
        metrics.loss = 0.05;
        metrics.training_time_ms = 1000;

        tracker.log_metrics(exp.id.as_str(), metrics).await.unwrap();

        let got = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert!(got.metrics.is_some());
        let m = got.metrics.unwrap();
        assert!((m.accuracy - 0.95).abs() < 1e-9);
        assert!((m.loss - 0.05).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_end_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        tracker.end_experiment(exp.id.as_str()).await.unwrap();

        let got = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.status, ExperimentStatus::Completed);
        assert!(got.ended_at.is_some());
    }

    #[tokio::test]
    async fn test_fail_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        tracker.fail_experiment(exp.id.as_str()).await.unwrap();

        let got = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.status, ExperimentStatus::Failed);
        assert!(got.ended_at.is_some());
    }

    #[tokio::test]
    async fn test_get_best_by_accuracy() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        let mut m1 = ExperimentMetrics::default();
        m1.accuracy = 0.9;
        tracker.log_metrics(e1.id.as_str(), m1).await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        let mut m2 = ExperimentMetrics::default();
        m2.accuracy = 0.95;
        tracker.log_metrics(e2.id.as_str(), m2).await.unwrap();
        tracker.end_experiment(e2.id.as_str()).await.unwrap();

        let best = tracker.get_best_by_accuracy().await.unwrap();
        assert!((best.metrics.unwrap().accuracy - 0.95).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_compare_by_accuracy() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        let mut m1 = ExperimentMetrics::default();
        m1.accuracy = 0.9;
        tracker.log_metrics(e1.id.as_str(), m1).await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        let mut m2 = ExperimentMetrics::default();
        m2.accuracy = 0.95;
        tracker.log_metrics(e2.id.as_str(), m2).await.unwrap();
        tracker.end_experiment(e2.id.as_str()).await.unwrap();

        let ord = tracker
            .compare_by_accuracy(e1.id.as_str(), e2.id.as_str())
            .await
            .unwrap();
        assert_eq!(ord, std::cmp::Ordering::Less);
    }

    #[tokio::test]
    async fn test_add_hyperparameters_and_tags() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        let mut params = HashMap::new();
        params.insert("lr".to_string(), "0.001".to_string());
        tracker
            .add_hyperparameters(exp.id.as_str(), params)
            .await
            .unwrap();

        tracker
            .add_tags(exp.id.as_str(), vec!["best".to_string()])
            .await
            .unwrap();

        let got = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.hyperparameters.get("lr"), Some(&"0.001".to_string()));
        assert!(got.tags.contains(&"best".to_string()));
    }
}
