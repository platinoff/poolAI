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
//!     },
//!     PipelineStep {
//!         id: "train".to_string(),
//!         step_type: StepType::Training,
//!         config: std::collections::HashMap::new(),
//!     },
//! ];
//!
//! let pipeline = manager.create_pipeline("pipeline1", steps).await?;
//! manager.execute_pipeline(pipeline.id.as_str()).await?;
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

/// Pipeline step type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StepType {
    Preprocessing,
    Training,
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
        // Simulate step execution
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let mut output = HashMap::new();
        output.insert("status".to_string(), "completed".to_string());
        output.insert("step_id".to_string(), step.id.clone());

        Ok(output)
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
}
