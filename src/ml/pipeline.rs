//! Pipeline Management (Stage 4.4, ML.6)
//!
//! ML pipeline orchestration with:
//! - Pipeline definition and steps
//! - Pipeline execution
//! - Dependency management
//! - Step status tracking
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::ml::pipeline::{MLPipelineManager, PipelineStep, StepType};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let manager = MLPipelineManager::new();
//!
//! let steps = vec![
//!     PipelineStep {
//!         id: "preprocess".to_string(),
//!         step_type: StepType::Preprocessing,
//!         config: std::collections::HashMap::new(),
//!         dependencies: vec![],
//!     },
//!     PipelineStep {
//!         id: "train".to_string(),
//!         step_type: StepType::Training,
//!         config: std::collections::HashMap::new(),
//!         dependencies: vec!["preprocess".to_string()],
//!     },
//! ];
//!
//! let pipeline = manager.create_pipeline("pipeline1", steps).await?;
//! manager.execute_pipeline(pipeline.id.as_str()).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use crate::ml::automl::{AutoMLPipeline, AutomlConfig, TrainingData};
use crate::ml::optimization::{
    apply_iterative_pruning, apply_pruning, apply_quantization, suggest_hyperparams,
    OptimizationProfile, PruningConfig, PruningStrategy, QuantizationLevel, TuningConfig,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Pipeline step type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StepType {
    Preprocessing,
    Training,
    /// ML.1 hyperparameter suggestion (`suggest_hyperparams`).
    HyperparameterTuning,
    /// ML.1 quantization (`apply_quantization`).
    Quantization,
    /// ML.1 structured / magnitude / unstructured pruning (`optimization` module).
    Pruning,
    /// ML.2 AutoML model selection (`AutoMLPipeline::train`).
    AutoMl,
    Evaluation,
    Deployment,
}

impl Default for StepType {
    fn default() -> Self {
        Self::Preprocessing
    }
}

/// Pipeline step
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineStep {
    pub id: String,
    pub step_type: StepType,
    pub config: HashMap<String, String>,
    pub dependencies: Vec<String>,
}

/// Step execution status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl Default for StepStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Step execution result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub status: StepStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub output: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

/// Pipeline status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PipelineStatus {
    Created,
    Running,
    Completed,
    Failed,
}

impl Default for PipelineStatus {
    fn default() -> Self {
        Self::Created
    }
}

/// ML Pipeline
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MLPipeline {
    pub id: String,
    pub name: String,
    pub steps: Vec<PipelineStep>,
    pub status: PipelineStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub step_results: HashMap<String, StepResult>,
}

/// ML Pipeline Manager
///
/// Manages ML pipelines with step orchestration, dependency resolution, and execution.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
pub struct MLPipelineManager {
    pipelines: Arc<RwLock<HashMap<String, MLPipeline>>>,
}

impl Default for MLPipelineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MLPipelineManager {
    /// Create a new pipeline manager
    pub fn new() -> Self {
        Self {
            pipelines: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new pipeline
    ///
    /// # Arguments
    ///
    /// * `name` - Pipeline name
    /// * `steps` - Pipeline steps
    pub async fn create_pipeline(
        &self,
        name: &str,
        steps: Vec<PipelineStep>,
    ) -> Result<MLPipeline, AppError> {
        if name.is_empty() {
            return Err(AppError::ModelError(
                "Pipeline name cannot be empty. Context: Empty name provided. \
                Suggestion: Provide a valid pipeline identifier."
                    .to_string(),
            ));
        }

        if steps.is_empty() {
            return Err(AppError::ModelError(
                "Pipeline must have at least one step. Context: Empty steps vector. \
                Suggestion: Add at least one step to the pipeline. \
                Current: steps=0"
                    .to_string(),
            ));
        }

        // Validate step IDs are unique
        let mut step_ids = std::collections::HashSet::new();
        for step in &steps {
            if step_ids.contains(&step.id) {
                return Err(AppError::ModelError(format!(
                    "Duplicate step ID. Context: Step ID '{}' appears multiple times. \
                    Suggestion: Ensure all step IDs are unique. \
                    Current: duplicate_id={}",
                    step.id, step.id
                )));
            }
            step_ids.insert(step.id.clone());
        }

        // Validate dependencies
        for step in &steps {
            for dep in &step.dependencies {
                if !step_ids.contains(dep) {
                    return Err(AppError::ModelError(format!(
                        "Invalid dependency. Context: Step '{}' depends on '{}' which does not exist. \
                        Suggestion: Ensure all dependencies reference existing step IDs. \
                        Current: step_id={}, dependency={}",
                        step.id, dep, step.id, dep
                    )));
                }
            }
        }

        let id = Uuid::new_v4().to_string();
        let short_id = id[..8].to_string();

        let pipeline = MLPipeline {
            id: short_id.clone(),
            name: name.to_string(),
            steps: steps.clone(),
            status: PipelineStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            step_results: HashMap::new(),
        };

        let mut pipelines = self.pipelines.write().await;
        pipelines.insert(short_id.clone(), pipeline.clone());

        Ok(pipeline)
    }

    /// Execute a pipeline
    ///
    /// # Arguments
    ///
    /// * `pipeline_id` - Pipeline ID
    pub async fn execute_pipeline(&self, pipeline_id: &str) -> Result<(), AppError> {
        let mut pipelines = self.pipelines.write().await;

        let pipeline = pipelines.get_mut(pipeline_id).ok_or_else(|| {
            AppError::ModelError(format!(
                "Pipeline not found. Context: No pipeline with id '{}'. \
                Suggestion: Create pipeline with create_pipeline() first. \
                Current: pipeline_id={}",
                pipeline_id, pipeline_id
            ))
        })?;

        if pipeline.status == PipelineStatus::Running {
            return Err(AppError::ModelError(
                "Pipeline is already running. Context: Pipeline execution already in progress. \
                Suggestion: Wait for current execution to complete. \
                Current: status=Running"
                    .to_string(),
            ));
        }

        pipeline.status = PipelineStatus::Running;
        pipeline.started_at = Some(Utc::now());

        // Execute steps in dependency order
        let execution_order = self.resolve_execution_order(&pipeline.steps)?;

        for step_id in execution_order {
            let step = pipeline.steps.iter().find(|s| s.id == step_id).unwrap();

            // Create step result
            let mut step_result = StepResult {
                step_id: step.id.clone(),
                status: StepStatus::Running,
                started_at: Some(Utc::now()),
                completed_at: None,
                output: None,
                error: None,
            };

            // Simulate step execution
            match self.execute_step(step).await {
                Ok(output) => {
                    step_result.status = StepStatus::Completed;
                    step_result.completed_at = Some(Utc::now());
                    step_result.output = Some(output);
                }
                Err(e) => {
                    step_result.status = StepStatus::Failed;
                    step_result.completed_at = Some(Utc::now());
                    step_result.error = Some(e.to_string());
                    pipeline.status = PipelineStatus::Failed;
                    pipeline.completed_at = Some(Utc::now());
                    pipeline.step_results.insert(step.id.clone(), step_result);
                    return Err(AppError::ModelError(format!(
                        "Pipeline execution failed at step '{}'. Context: Step execution failed. \
                        Suggestion: Check step configuration and dependencies. \
                        Current: step_id={}, error={}",
                        step.id, step.id, e
                    )));
                }
            }

            pipeline.step_results.insert(step.id.clone(), step_result);
        }

        pipeline.status = PipelineStatus::Completed;
        pipeline.completed_at = Some(Utc::now());

        Ok(())
    }

    /// Resolve execution order based on dependencies
    fn resolve_execution_order(&self, steps: &[PipelineStep]) -> Result<Vec<String>, AppError> {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        fn visit(
            step_id: &str,
            steps: &[PipelineStep],
            visited: &mut std::collections::HashSet<String>,
            visiting: &mut std::collections::HashSet<String>,
            order: &mut Vec<String>,
        ) -> Result<(), AppError> {
            if visited.contains(step_id) {
                return Ok(());
            }

            if visiting.contains(step_id) {
                return Err(AppError::ModelError(format!(
                    "Circular dependency detected. Context: Step '{}' has circular dependency. \
                    Suggestion: Remove circular dependencies from pipeline. \
                    Current: step_id={}",
                    step_id, step_id
                )));
            }

            visiting.insert(step_id.to_string());

            let step = steps.iter().find(|s| s.id == step_id).ok_or_else(|| {
                AppError::ModelError(format!(
                    "Step not found. Context: Step '{}' not found in pipeline. \
                    Current: step_id={}",
                    step_id, step_id
                ))
            })?;

            for dep in &step.dependencies {
                visit(dep, steps, visited, visiting, order)?;
            }

            visiting.remove(step_id);
            visited.insert(step_id.to_string());
            order.push(step_id.to_string());

            Ok(())
        }

        for step in steps {
            if !visited.contains(&step.id) {
                visit(&step.id, steps, &mut visited, &mut visiting, &mut order)?;
            }
        }

        Ok(order)
    }

    /// Execute a single step (simulated)
    async fn execute_step(&self, step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        match step.step_type {
            StepType::Pruning => Self::execute_pruning_step(step),
            StepType::Quantization => Self::execute_quantization_step(step),
            StepType::HyperparameterTuning => Self::execute_tuning_step(step),
            StepType::AutoMl => Self::execute_automl_step(step).await,
            _ => {
                let mut output = HashMap::new();
                output.insert("status".to_string(), "completed".to_string());
                output.insert("step_id".to_string(), step.id.clone());
                Ok(output)
            }
        }
    }

    fn execute_pruning_step(step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        let (weights, pconf) = Self::pruning_inputs_from_step_config(step)?;

        let result = if pconf.iterative && pconf.iterations > 1 {
            apply_iterative_pruning(&weights, &pconf)
        } else {
            apply_pruning(&weights, &pconf)
        };

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "pruning".to_string());
        output.insert(
            "pruning_strategy".to_string(),
            format!("{:?}", result.strategy),
        );
        output.insert(
            "weights_before".to_string(),
            result.weights_before.to_string(),
        );
        output.insert(
            "weights_after".to_string(),
            result.weights_after.to_string(),
        );
        output.insert("pruned_count".to_string(), result.pruned_count.to_string());
        output.insert(
            "compression_ratio".to_string(),
            format!("{:.6}", result.compression_ratio),
        );
        output.insert(
            "accuracy_drop_est".to_string(),
            format!("{:.6}", result.accuracy_drop),
        );
        Ok(output)
    }

    fn execute_quantization_step(step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        let cfg = &step.config;
        let level = Self::parse_quantization_level(
            cfg.get("quantization")
                .or_else(|| cfg.get("quantization_level"))
                .or_else(|| cfg.get("level"))
                .map(|s| s.as_str()),
        )?;
        let pruning_ratio = cfg
            .get("profile_pruning_ratio")
            .or_else(|| cfg.get("pruning_ratio"))
            .map(|s| s.parse::<f32>())
            .transpose()
            .map_err(|_| {
                AppError::ModelError(
                    "Invalid profile_pruning_ratio for quantization step. Suggestion: float or omit."
                        .to_string(),
                )
            })?
            .unwrap_or(0.0);
        if !(0.0..=1.0).contains(&pruning_ratio) {
            return Err(AppError::ModelError(
                "profile_pruning_ratio out of range [0,1].".to_string(),
            ));
        }

        let profile = OptimizationProfile {
            quantization: level,
            pruning_ratio,
        };
        let q = apply_quantization(&profile);

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "quantization".to_string());
        output.insert("quantization_level".to_string(), format!("{:?}", q.level));
        output.insert(
            "size_mb_before".to_string(),
            format!("{:.6}", q.size_mb_before),
        );
        output.insert(
            "size_mb_after".to_string(),
            format!("{:.6}", q.size_mb_after),
        );
        output.insert(
            "compression_ratio".to_string(),
            format!("{:.6}", q.compression_ratio),
        );
        Ok(output)
    }

    fn execute_tuning_step(step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        let cfg = Self::tuning_config_from_step(&step.config)?;
        let r = suggest_hyperparams(&cfg);

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "hyperparameter_tuning".to_string());
        output.insert(
            "suggested_learning_rate".to_string(),
            format!("{}", r.learning_rate),
        );
        output.insert("suggested_batch_size".to_string(), r.batch_size.to_string());
        output.insert(
            "suggested_epochs".to_string(),
            r.suggested_epochs.to_string(),
        );
        Ok(output)
    }

    async fn execute_automl_step(step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        let data = Self::parse_automl_training_data(&step.config)?;
        let automl_cfg = Self::automl_config_from_step(&step.config)?;
        let pipeline = AutoMLPipeline::new(automl_cfg);
        let model = pipeline.train(data).await?;

        let hp_json = serde_json::to_string(&model.hyperparameters).map_err(|e| {
            AppError::ModelError(format!(
                "Failed to serialize AutoML hyperparameters: {}. Context: serde_json.",
                e
            ))
        })?;

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "automl".to_string());
        output.insert("model_id".to_string(), model.model_id);
        output.insert("accuracy".to_string(), format!("{:.6}", model.accuracy));
        output.insert("model_type".to_string(), format!("{:?}", model.model_type));
        output.insert(
            "training_time_ms".to_string(),
            model.training_time_ms.to_string(),
        );
        output.insert("hyperparameters_json".to_string(), hp_json);
        Ok(output)
    }

    fn parse_quantization_level(s: Option<&str>) -> Result<QuantizationLevel, AppError> {
        let Some(key) = s else {
            return Ok(QuantizationLevel::None);
        };
        Ok(match key.to_ascii_lowercase().as_str() {
            "none" | "fp32" | "fp" => QuantizationLevel::None,
            "int8" | "q8" | "8" => QuantizationLevel::Int8,
            "int4" | "q4" | "4" => QuantizationLevel::Int4,
            other => {
                return Err(AppError::ModelError(format!(
                    "Unknown quantization level '{}'. Suggestion: none, int8, int4.",
                    other
                )));
            }
        })
    }

    fn tuning_config_from_step(cfg: &HashMap<String, String>) -> Result<TuningConfig, AppError> {
        let mut t = TuningConfig::default_config();
        if let Some(v) = cfg.get("lr_min").or_else(|| cfg.get("learning_rate_min")) {
            t.learning_rate_min = v.parse().map_err(|_| {
                AppError::ModelError(format!("Invalid learning_rate_min: '{}'.", v))
            })?;
        }
        if let Some(v) = cfg.get("lr_max").or_else(|| cfg.get("learning_rate_max")) {
            t.learning_rate_max = v.parse().map_err(|_| {
                AppError::ModelError(format!("Invalid learning_rate_max: '{}'.", v))
            })?;
        }
        if let Some(raw) = cfg
            .get("batch_sizes")
            .or_else(|| cfg.get("batch_size_candidates"))
        {
            let cand: Vec<u32> = raw
                .split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            if !cand.is_empty() {
                t.batch_size_candidates = cand;
            }
        }
        if t.learning_rate_min > t.learning_rate_max {
            return Err(AppError::ModelError(
                "Tuning config: learning_rate_min must be <= learning_rate_max.".to_string(),
            ));
        }
        Ok(t)
    }

    fn automl_config_from_step(cfg: &HashMap<String, String>) -> Result<AutomlConfig, AppError> {
        let mut c = AutomlConfig::default_config();
        if let Some(v) = cfg.get("automl_max_trials") {
            c.max_trials = v.parse().map_err(|_| {
                AppError::ModelError(format!("Invalid automl_max_trials: '{}'.", v))
            })?;
        }
        if let Some(v) = cfg.get("automl_timeout_seconds") {
            c.timeout_seconds = v.parse().map_err(|_| {
                AppError::ModelError(format!("Invalid automl_timeout_seconds: '{}'.", v))
            })?;
        }
        if let Some(v) = cfg.get("automl_ensemble_size") {
            c.ensemble_size = v.parse().map_err(|_| {
                AppError::ModelError(format!("Invalid automl_ensemble_size: '{}'.", v))
            })?;
        }
        if let Some(v) = cfg.get("automl_cv_folds") {
            c.cross_validation_folds = v
                .parse()
                .map_err(|_| AppError::ModelError(format!("Invalid automl_cv_folds: '{}'.", v)))?;
        }
        if let Some(v) = cfg.get("automl_auto_features") {
            c.auto_feature_engineering =
                matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on");
        }
        Ok(c)
    }

    /// Parse training data from step config.
    ///
    /// - `feature_rows`: rows separated by `;`, components by `,` (e.g. `"1,2;3,4;5,6"`).
    /// - `labels`: comma-separated floats, same count as rows.
    /// If omitted, uses a small built-in example (4×2 features).
    fn parse_automl_training_data(cfg: &HashMap<String, String>) -> Result<TrainingData, AppError> {
        let default_features = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ];
        let default_labels = vec![0.0, 1.0, 0.0, 1.0];

        let (features, labels) = match cfg.get("feature_rows") {
            None => (default_features, default_labels),
            Some(rows_raw) => {
                let features: Vec<Vec<f64>> = rows_raw
                    .split(';')
                    .map(|row| {
                        row.split(',')
                            .filter_map(|x| x.trim().parse::<f64>().ok())
                            .collect::<Vec<f64>>()
                    })
                    .filter(|r| !r.is_empty())
                    .collect();
                if features.is_empty() {
                    return Err(AppError::ModelError(
                        "AutoML step: feature_rows parsed to no rows. Suggestion: use `1,2;3,4` \
                         style or omit for demo data."
                            .to_string(),
                    ));
                }
                let dim0 = features[0].len();
                if features.iter().any(|r| r.len() != dim0) {
                    return Err(AppError::ModelError(
                        "AutoML step: all feature rows must have the same length.".to_string(),
                    ));
                }
                let labels_raw = cfg.get("labels").ok_or_else(|| {
                    AppError::ModelError(
                        "AutoML step: labels required when feature_rows is set. Suggestion: \
                         labels=0,1,0,1 matching row count."
                            .to_string(),
                    )
                })?;
                let labels: Vec<f64> = labels_raw
                    .split(',')
                    .filter_map(|x| x.trim().parse::<f64>().ok())
                    .collect();
                if labels.len() != features.len() {
                    return Err(AppError::ModelError(format!(
                        "AutoML step: {} labels but {} feature rows.",
                        labels.len(),
                        features.len()
                    )));
                }
                (features, labels)
            }
        };

        Ok(TrainingData { features, labels })
    }

    fn pruning_inputs_from_step_config(
        step: &PipelineStep,
    ) -> Result<(Vec<f64>, PruningConfig), AppError> {
        let cfg = &step.config;
        let weights = Self::parse_pruning_weights(cfg);

        if weights.is_empty() {
            return Err(AppError::ModelError(
                "Pruning step requires weights or default non-empty demo weights. Context: weight \
                 vector is empty after parsing. Suggestion: add config key 'weights' with \
                 comma-separated floats, or omit for built-in demo vector."
                    .to_string(),
            ));
        }

        let ratio_raw = cfg
            .get("pruning_ratio")
            .or_else(|| cfg.get("ratio"))
            .map(|s| s.as_str())
            .unwrap_or("0.1");
        let ratio: f32 = ratio_raw.parse().map_err(|_| {
            AppError::ModelError(format!(
                "Invalid pruning ratio. Context: cannot parse '{}'. Suggestion: use a float in [0,1].",
                ratio_raw
            ))
        })?;
        if !(0.0..=1.0).contains(&ratio) {
            return Err(AppError::ModelError(format!(
                "Pruning ratio out of range. Context: ratio={}. Suggestion: use 0.0–1.0.",
                ratio
            )));
        }

        let strategy = Self::parse_pruning_strategy(
            cfg.get("pruning_strategy")
                .or_else(|| cfg.get("strategy"))
                .map(|s| s.as_str()),
        );

        let iterative = cfg
            .get("iterative")
            .map(|s| matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);

        let iterations: u32 = cfg
            .get("iterations")
            .map(|s| s.parse::<u32>())
            .transpose()
            .map_err(|_| {
                AppError::ModelError(
                    "Invalid iterations. Context: expected unsigned integer. Suggestion: e.g. \
                     iterations=3"
                        .to_string(),
                )
            })?
            .unwrap_or(1)
            .clamp(1, 64);

        let mut pconf = PruningConfig {
            strategy,
            ratio,
            iterative,
            iterations,
        };

        if iterative && iterations == 1 {
            pconf.iterative = false;
        }

        Ok((weights, pconf))
    }

    fn parse_pruning_weights(cfg: &HashMap<String, String>) -> Vec<f64> {
        if let Some(raw) = cfg.get("weights") {
            let v: Vec<f64> = raw
                .split(',')
                .filter_map(|t| t.trim().parse::<f64>().ok())
                .collect();
            if !v.is_empty() {
                return v;
            }
        }
        vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2]
    }

    fn parse_pruning_strategy(s: Option<&str>) -> PruningStrategy {
        let Some(key) = s else {
            return PruningStrategy::MagnitudeBased;
        };
        match key.to_ascii_lowercase().as_str() {
            "structured" => PruningStrategy::Structured,
            "unstructured" => PruningStrategy::Unstructured,
            "magnitude" | "magnitude_based" => PruningStrategy::MagnitudeBased,
            _ => PruningStrategy::MagnitudeBased,
        }
    }

    /// Get pipeline by ID
    pub async fn get_pipeline(&self, pipeline_id: &str) -> Result<MLPipeline, AppError> {
        let pipelines = self.pipelines.read().await;

        pipelines.get(pipeline_id).cloned().ok_or_else(|| {
            AppError::ModelError(format!(
                "Pipeline not found. Context: No pipeline with id '{}'. \
                Suggestion: Check pipeline_id or list_pipelines(). \
                Current: pipeline_id={}",
                pipeline_id, pipeline_id
            ))
        })
    }

    /// List all pipelines
    pub async fn list_pipelines(&self) -> Vec<MLPipeline> {
        let pipelines = self.pipelines.read().await;
        let mut list: Vec<_> = pipelines.values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    /// Get pipeline status
    pub async fn get_pipeline_status(&self, pipeline_id: &str) -> Result<PipelineStatus, AppError> {
        let pipeline = self.get_pipeline(pipeline_id).await?;
        Ok(pipeline.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = MLPipelineManager::new();
        let list = manager.list_pipelines().await;
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_create_pipeline() {
        let manager = MLPipelineManager::new();

        let steps = vec![PipelineStep {
            id: "step1".to_string(),
            step_type: StepType::Preprocessing,
            config: HashMap::new(),
            dependencies: vec![],
        }];

        let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
        assert_eq!(pipeline.name, "pipeline1");
        assert_eq!(pipeline.status, PipelineStatus::Created);
    }

    #[tokio::test]
    async fn test_create_pipeline_empty_name() {
        let manager = MLPipelineManager::new();
        let steps = vec![PipelineStep {
            id: "step1".to_string(),
            step_type: StepType::Preprocessing,
            config: HashMap::new(),
            dependencies: vec![],
        }];

        let result = manager.create_pipeline("", steps).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_pipeline_empty_steps() {
        let manager = MLPipelineManager::new();
        let result = manager.create_pipeline("pipeline1", vec![]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_pipeline_duplicate_step_ids() {
        let manager = MLPipelineManager::new();

        let steps = vec![
            PipelineStep {
                id: "step1".to_string(),
                step_type: StepType::Preprocessing,
                config: HashMap::new(),
                dependencies: vec![],
            },
            PipelineStep {
                id: "step1".to_string(), // Duplicate
                step_type: StepType::Training,
                config: HashMap::new(),
                dependencies: vec![],
            },
        ];

        let result = manager.create_pipeline("pipeline1", steps).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_pipeline() {
        let manager = MLPipelineManager::new();

        let steps = vec![PipelineStep {
            id: "step1".to_string(),
            step_type: StepType::Preprocessing,
            config: HashMap::new(),
            dependencies: vec![],
        }];

        let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();

        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        assert_eq!(got.status, PipelineStatus::Completed);
    }

    #[tokio::test]
    async fn test_execute_pipeline_with_dependencies() {
        let manager = MLPipelineManager::new();

        let steps = vec![
            PipelineStep {
                id: "preprocess".to_string(),
                step_type: StepType::Preprocessing,
                config: HashMap::new(),
                dependencies: vec![],
            },
            PipelineStep {
                id: "train".to_string(),
                step_type: StepType::Training,
                config: HashMap::new(),
                dependencies: vec!["preprocess".to_string()],
            },
        ];

        let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();

        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        assert_eq!(got.status, PipelineStatus::Completed);
        assert_eq!(got.step_results.len(), 2);
    }

    #[tokio::test]
    async fn test_execute_pruning_step_populates_metrics() {
        let manager = MLPipelineManager::new();
        let mut cfg = HashMap::new();
        cfg.insert("pruning_ratio".to_string(), "0.2".to_string());
        cfg.insert(
            "pruning_strategy".to_string(),
            "magnitude_based".to_string(),
        );

        let steps = vec![PipelineStep {
            id: "prune1".to_string(),
            step_type: StepType::Pruning,
            config: cfg,
            dependencies: vec![],
        }];

        let pipeline = manager.create_pipeline("p", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();

        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let res = got.step_results.get("prune1").unwrap();
        assert_eq!(res.status, StepStatus::Completed);
        let out = res.output.as_ref().unwrap();
        assert_eq!(out.get("step_kind"), Some(&"pruning".to_string()));
        assert!(out.get("pruned_count").unwrap().parse::<usize>().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_execute_quantization_step() {
        let manager = MLPipelineManager::new();
        let mut cfg = HashMap::new();
        cfg.insert("quantization".to_string(), "int8".to_string());

        let steps = vec![PipelineStep {
            id: "q1".to_string(),
            step_type: StepType::Quantization,
            config: cfg,
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pq", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["q1"].output.as_ref().unwrap();
        assert_eq!(out.get("step_kind"), Some(&"quantization".to_string()));
        assert!(
            out.get("compression_ratio")
                .unwrap()
                .parse::<f64>()
                .unwrap()
                >= 1.0
        );
    }

    #[tokio::test]
    async fn test_execute_tuning_step() {
        let manager = MLPipelineManager::new();
        let steps = vec![PipelineStep {
            id: "t1".to_string(),
            step_type: StepType::HyperparameterTuning,
            config: HashMap::new(),
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pt", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["t1"].output.as_ref().unwrap();
        assert_eq!(
            out.get("step_kind"),
            Some(&"hyperparameter_tuning".to_string())
        );
        assert!(
            out.get("suggested_batch_size")
                .unwrap()
                .parse::<u32>()
                .unwrap()
                > 0
        );
    }

    #[tokio::test]
    async fn test_execute_automl_step_uses_builtin_data() {
        let manager = MLPipelineManager::new();
        let steps = vec![PipelineStep {
            id: "a1".to_string(),
            step_type: StepType::AutoMl,
            config: HashMap::new(),
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pa", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["a1"].output.as_ref().unwrap();
        assert_eq!(out.get("step_kind"), Some(&"automl".to_string()));
        assert!(out.get("accuracy").unwrap().parse::<f64>().unwrap() > 0.0);
        assert!(out.contains_key("hyperparameters_json"));
    }
}
