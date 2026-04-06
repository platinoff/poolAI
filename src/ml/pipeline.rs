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
use crate::ml::experiments::{ExperimentMetrics, ExperimentTracker};
use crate::ml::federated::{
    AggregationMode, ClientUpdate, FederatedConfig, FederatedLearningPipeline,
};
use crate::ml::optimization::{
    apply_iterative_pruning, apply_pruning, apply_quantization, profile_model, suggest_hyperparams,
    OptimizationProfile, PruningConfig, PruningStrategy, QuantizationLevel, TuningConfig,
};
use crate::ml::versioning::{ModelMetadata, ModelVersionManager};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Pipeline step type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StepType {
    Preprocessing,
    Training,
    /// ML.1 model profiling stub (`profile_model`).
    Profiling,
    /// ML.1 hyperparameter suggestion (`suggest_hyperparams`).
    HyperparameterTuning,
    /// ML.1 quantization (`apply_quantization`).
    Quantization,
    /// ML.1 structured / magnitude / unstructured pruning (`optimization` module).
    Pruning,
    /// ML.2 AutoML model selection (`AutoMLPipeline::train`).
    AutoMl,
    /// ML.3 one-shot federated round: synthetic clients + `aggregate_updates` (FedAvg/FedProx).
    FederatedAggregation,
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
    /// Shared ML.4 registry — updated when AutoML steps run (unless skipped in step config).
    version_manager: Arc<ModelVersionManager>,
    /// Shared ML.5 tracker — experiment opened/closed around successful AutoML training.
    experiment_tracker: Arc<ExperimentTracker>,
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
            version_manager: Arc::new(ModelVersionManager::new()),
            experiment_tracker: Arc::new(ExperimentTracker::new()),
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

    /// Execute a single step (Rust backends: optimization, AutoML, federated, and core stages).
    async fn execute_step(&self, step: &PipelineStep) -> Result<HashMap<String, String>, AppError> {
        match step.step_type {
            StepType::Preprocessing => Ok(Self::execute_preprocessing_step(step)),
            StepType::Training => Ok(Self::execute_training_step(step)),
            StepType::Evaluation => Ok(Self::execute_evaluation_step(step)),
            StepType::Deployment => Ok(Self::execute_deployment_step(step)),
            StepType::Profiling => Ok(Self::execute_profiling_step(step)),
            StepType::Pruning => Self::execute_pruning_step(step),
            StepType::Quantization => Self::execute_quantization_step(step),
            StepType::HyperparameterTuning => Self::execute_tuning_step(step),
            StepType::AutoMl => self.execute_automl_step(step).await,
            StepType::FederatedAggregation => Self::execute_federated_aggregation_step(step).await,
        }
    }

    /// Deterministic fingerprint for stable metrics derived from `step_id` (tests / logs).
    fn step_id_fingerprint(step_id: &str) -> u64 {
        let mut h: u64 = 5381;
        for b in step_id.as_bytes() {
            h = h.wrapping_mul(33).wrapping_add(u64::from(*b));
        }
        h
    }

    /// Feature scaling / layout estimate (no I/O): uses `feature_dim`, `sample_count`, `normalize`.
    fn execute_preprocessing_step(step: &PipelineStep) -> HashMap<String, String> {
        let cfg = &step.config;
        let feature_dim = cfg
            .get("feature_dim")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(784)
            .max(1);
        let sample_count = cfg
            .get("sample_count")
            .or_else(|| cfg.get("samples"))
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1024)
            .max(1);
        let normalize = cfg
            .get("normalize")
            .map(|s| {
                !matches!(
                    s.to_ascii_lowercase().as_str(),
                    "0" | "false" | "no" | "off"
                )
            })
            .unwrap_or(true);

        let fp = Self::step_id_fingerprint(&step.id);
        let estimated_bytes = feature_dim
            .saturating_mul(sample_count)
            .saturating_mul(size_of::<f32>());
        let checksum = format!("{:016x}", fp ^ (estimated_bytes as u64));

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "preprocessing".to_string());
        output.insert("feature_dim".to_string(), feature_dim.to_string());
        output.insert("sample_count".to_string(), sample_count.to_string());
        output.insert("normalize_enabled".to_string(), normalize.to_string());
        output.insert("estimated_bytes".to_string(), estimated_bytes.to_string());
        output.insert("pipeline_checksum".to_string(), checksum);
        output
    }

    /// Toy loss curve: `epochs`, `learning_rate` — pure Rust, deterministic.
    fn execute_training_step(step: &PipelineStep) -> HashMap<String, String> {
        let cfg = &step.config;
        let epochs = cfg
            .get("epochs")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(5)
            .clamp(1, 10_000);
        let lr = cfg
            .get("learning_rate")
            .or_else(|| cfg.get("lr"))
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.01)
            .clamp(1e-6, 1.0);

        let mut loss = 1.0_f64;
        for _ in 0..epochs {
            loss *= (1.0 - lr * 0.05).clamp(0.5, 1.0);
            loss = loss.max(1e-6);
        }

        let fp = Self::step_id_fingerprint(&step.id);
        let seed_scale = 1.0 + (fp % 97) as f64 / 10_000.0;

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "training".to_string());
        output.insert("epochs_run".to_string(), epochs.to_string());
        output.insert("learning_rate".to_string(), format!("{:.8}", lr));
        output.insert(
            "final_loss".to_string(),
            format!("{:.8}", loss * seed_scale),
        );
        output.insert("converged".to_string(), (loss < 0.05).to_string());
        output
    }

    /// Report-style metrics from `baseline_accuracy` and deterministic jitter from `step_id`.
    fn execute_evaluation_step(step: &PipelineStep) -> HashMap<String, String> {
        let cfg = &step.config;
        let baseline = cfg
            .get("baseline_accuracy")
            .or_else(|| cfg.get("expected_accuracy"))
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.91)
            .clamp(0.0, 1.0);
        let samples = cfg
            .get("samples_evaluated")
            .or_else(|| cfg.get("eval_samples"))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(500)
            .max(1);

        let fp = Self::step_id_fingerprint(&step.id);
        let jitter = (fp % 1000) as f64 / 100_000.0;
        let accuracy = (baseline + jitter).min(0.9999);
        let f1_proxy = (accuracy * 0.98).min(0.9999);

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "evaluation".to_string());
        output.insert("accuracy".to_string(), format!("{:.6}", accuracy));
        output.insert("f1_proxy".to_string(), format!("{:.6}", f1_proxy));
        output.insert("samples_evaluated".to_string(), samples.to_string());
        output
    }

    /// Rollout metadata (synthetic URI + revision) — no network.
    fn execute_deployment_step(step: &PipelineStep) -> HashMap<String, String> {
        let cfg = &step.config;
        let environment = cfg
            .get("environment")
            .or_else(|| cfg.get("target"))
            .cloned()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "staging".to_string());
        let rollout = cfg
            .get("rollout_percent")
            .or_else(|| cfg.get("rollout"))
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(100)
            .min(100);

        let fp = Self::step_id_fingerprint(&step.id);
        let revision = format!("{:08x}", (fp as u32) ^ 0xA5A5_5A5A);
        let artifact_uri = format!(
            "poolai://artifacts/ml/{}/{}/model.bundle",
            environment, revision
        );

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "deployment".to_string());
        output.insert("environment".to_string(), environment);
        output.insert("rollout_percent".to_string(), rollout.to_string());
        output.insert("revision".to_string(), revision);
        output.insert("artifact_uri".to_string(), artifact_uri);
        output
    }

    fn execute_profiling_step(step: &PipelineStep) -> HashMap<String, String> {
        let p = profile_model();
        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "profiling".to_string());
        output.insert("latency_ms".to_string(), format!("{:.6}", p.latency_ms));
        output.insert("memory_mb".to_string(), format!("{:.6}", p.memory_mb));
        output.insert("flops".to_string(), p.flops.to_string());
        output
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

    async fn execute_automl_step(
        &self,
        step: &PipelineStep,
    ) -> Result<HashMap<String, String>, AppError> {
        let cfg = &step.config;
        let data = Self::parse_automl_training_data(cfg)?;
        let automl_cfg = Self::automl_config_from_step(cfg)?;
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
        output.insert("model_id".to_string(), model.model_id.clone());
        output.insert("accuracy".to_string(), format!("{:.6}", model.accuracy));
        output.insert("model_type".to_string(), format!("{:?}", model.model_type));
        output.insert(
            "training_time_ms".to_string(),
            model.training_time_ms.to_string(),
        );
        output.insert("hyperparameters_json".to_string(), hp_json);

        let skip_registry = cfg
            .get("automl_skip_registry")
            .map(|s| matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);

        if !skip_registry {
            let registry_key = cfg
                .get("registry_model_id")
                .cloned()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "pipeline_automl_model".to_string());

            let metadata = ModelMetadata {
                model_type: format!("{:?}", model.model_type),
                accuracy: model.accuracy,
                training_time_ms: model.training_time_ms,
                hyperparameters: model.hyperparameters.clone(),
                description: Some(format!(
                    "AutoML pipeline step '{}'; internal model_id={}",
                    step.id, model.model_id
                )),
            };

            let registered = self
                .version_manager
                .register_model(&registry_key, metadata)
                .await?;
            output.insert("ml_registered_version".to_string(), registered.version);
            output.insert("ml_registry_model_id".to_string(), registry_key);

            let exp_name = cfg
                .get("experiment_name")
                .cloned()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| format!("automl_{}", step.id));

            let exp = self
                .experiment_tracker
                .start_experiment(&exp_name, &format!("{:?}", model.model_type))
                .await?;
            self.experiment_tracker
                .add_hyperparameters(exp.id.as_str(), model.hyperparameters.clone())
                .await?;
            let metrics = ExperimentMetrics {
                accuracy: model.accuracy,
                loss: 0.0,
                training_time_ms: model.training_time_ms,
                custom: HashMap::new(),
            };
            self.experiment_tracker
                .log_metrics(exp.id.as_str(), metrics)
                .await?;
            self.experiment_tracker
                .end_experiment(exp.id.as_str())
                .await?;
            output.insert("experiment_id".to_string(), exp.id);
        }

        Ok(output)
    }

    async fn execute_federated_aggregation_step(
        step: &PipelineStep,
    ) -> Result<HashMap<String, String>, AppError> {
        let cfg = &step.config;
        let fl_cfg = Self::federated_config_from_step(cfg)?;

        let dim: usize = cfg
            .get("federated_weight_dim")
            .map(|s| s.parse::<usize>())
            .transpose()
            .map_err(|_| {
                AppError::ModelError(
                    "Invalid federated_weight_dim. Suggestion: positive integer.".to_string(),
                )
            })?
            .unwrap_or(3)
            .max(1);

        let mut n = fl_cfg.min_clients_per_round;
        if let Some(s) = cfg.get("federated_synthetic_clients") {
            n = s.parse::<u32>().map_err(|_| {
                AppError::ModelError(format!("Invalid federated_synthetic_clients: '{}'.", s))
            })?;
        }
        if n < fl_cfg.min_clients_per_round {
            return Err(AppError::ModelError(format!(
                "federated_synthetic_clients ({}) must be >= federated_min_clients ({}).",
                n, fl_cfg.min_clients_per_round
            )));
        }
        if n > fl_cfg.max_clients_per_round {
            return Err(AppError::ModelError(format!(
                "federated_synthetic_clients ({}) must be <= federated_max_clients ({}).",
                n, fl_cfg.max_clients_per_round
            )));
        }

        let pipeline = FederatedLearningPipeline::new(fl_cfg.clone());
        let round = pipeline.get_current_round().await;

        for i in 0..n {
            let mut weights = vec![0.0; dim];
            for j in 0..dim {
                weights[j] = 0.1 * (f64::from(i) + 1.0) + 0.04 * (j as f64);
            }
            pipeline
                .add_client_update(ClientUpdate {
                    client_id: format!("synthetic_client_{}", i),
                    model_weights: weights,
                    sample_count: 100 + i as usize,
                    round,
                })
                .await?;
        }

        let agg = pipeline.aggregate_updates().await?;

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());
        output.insert("step_kind".to_string(), "federated_aggregation".to_string());
        output.insert("federated_round".to_string(), agg.round.to_string());
        output.insert("clients_count".to_string(), agg.clients_count.to_string());
        output.insert("total_samples".to_string(), agg.total_samples.to_string());
        output.insert(
            "aggregation_mode".to_string(),
            format!("{:?}", agg.aggregation_mode),
        );
        output.insert("weight_dim".to_string(), agg.weights.len().to_string());
        if !agg.weights.is_empty() {
            output.insert(
                "aggregated_weight_0".to_string(),
                format!("{:.8}", agg.weights[0]),
            );
        }
        Ok(output)
    }

    fn federated_config_from_step(
        cfg: &HashMap<String, String>,
    ) -> Result<FederatedConfig, AppError> {
        let mut c = FederatedConfig::default_config();
        if let Some(s) = cfg.get("federated_aggregation") {
            c.aggregation = match s.to_ascii_lowercase().as_str() {
                "fedprox" | "prox" => AggregationMode::FedProx,
                _ => AggregationMode::FedAvg,
            };
        }
        if let Some(s) = cfg.get("federated_min_clients") {
            c.min_clients_per_round = s.parse::<u32>().map_err(|_| {
                AppError::ModelError(format!("Invalid federated_min_clients: '{}'.", s))
            })?;
        }
        if let Some(s) = cfg.get("federated_max_clients") {
            c.max_clients_per_round = s.parse::<u32>().map_err(|_| {
                AppError::ModelError(format!("Invalid federated_max_clients: '{}'.", s))
            })?;
        }
        if c.min_clients_per_round < 1 {
            return Err(AppError::ModelError(
                "federated_min_clients must be at least 1.".to_string(),
            ));
        }
        if c.max_clients_per_round < c.min_clients_per_round {
            return Err(AppError::ModelError(
                "federated_max_clients must be >= federated_min_clients.".to_string(),
            ));
        }
        Ok(c)
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
    async fn test_execute_profiling_step() {
        let manager = MLPipelineManager::new();
        let steps = vec![PipelineStep {
            id: "prof1".to_string(),
            step_type: StepType::Profiling,
            config: HashMap::new(),
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pp", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["prof1"].output.as_ref().unwrap();
        assert_eq!(out.get("step_kind"), Some(&"profiling".to_string()));
        assert!(out.get("flops").unwrap().parse::<u64>().unwrap() > 0);
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
    async fn test_execute_federated_aggregation_step() {
        let manager = MLPipelineManager::new();
        let steps = vec![PipelineStep {
            id: "fed1".to_string(),
            step_type: StepType::FederatedAggregation,
            config: HashMap::new(),
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pf", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["fed1"].output.as_ref().unwrap();
        assert_eq!(
            out.get("step_kind"),
            Some(&"federated_aggregation".to_string())
        );
        assert_eq!(out.get("clients_count"), Some(&"2".to_string()));
        assert!(out.contains_key("aggregated_weight_0"));
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
        assert!(out.contains_key("ml_registered_version"));
        assert!(out.contains_key("experiment_id"));
    }

    #[tokio::test]
    async fn test_execute_automl_skip_registry() {
        let manager = MLPipelineManager::new();
        let mut cfg = HashMap::new();
        cfg.insert("automl_skip_registry".to_string(), "true".to_string());
        let steps = vec![PipelineStep {
            id: "a2".to_string(),
            step_type: StepType::AutoMl,
            config: cfg,
            dependencies: vec![],
        }];
        let pipeline = manager.create_pipeline("pa2", steps).await.unwrap();
        manager
            .execute_pipeline(pipeline.id.as_str())
            .await
            .unwrap();
        let got = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
        let out = got.step_results["a2"].output.as_ref().unwrap();
        assert!(!out.contains_key("experiment_id"));
        assert!(!out.contains_key("ml_registered_version"));
    }
}
