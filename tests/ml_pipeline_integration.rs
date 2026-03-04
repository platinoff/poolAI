//! Integration tests for Pipeline Management Module (ML.6)
//!
//! Tests the pipeline management functionality including creation,
//! execution, dependency resolution, and status tracking.

use poolai::ml::pipeline::{
    MLPipeline, MLPipelineManager, PipelineStatus, PipelineStep, StepStatus, StepType,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_manager_creation() {
    let manager = MLPipelineManager::new();
    let list = manager.list_pipelines().await;
    assert!(list.is_empty());
}

#[tokio::test]
async fn test_create_pipeline_single_step() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    }];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    assert_eq!(pipeline.name, "pipeline1");
    assert_eq!(pipeline.steps.len(), 1);
    assert_eq!(pipeline.status, PipelineStatus::Created);
}

#[tokio::test]
async fn test_create_pipeline_multiple_steps() {
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
            dependencies: vec![],
        },
        PipelineStep {
            id: "evaluate".to_string(),
            step_type: StepType::Evaluation,
            config: HashMap::new(),
            dependencies: vec![],
        },
    ];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    assert_eq!(pipeline.steps.len(), 3);
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
async fn test_create_pipeline_invalid_dependency() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Training,
        config: HashMap::new(),
        dependencies: vec!["nonexistent".to_string()], // Invalid
    }];

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

    let got: MLPipeline = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
    assert_eq!(got.status, PipelineStatus::Completed);
    assert!(got.started_at.is_some());
    assert!(got.completed_at.is_some());
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
        PipelineStep {
            id: "evaluate".to_string(),
            step_type: StepType::Evaluation,
            config: HashMap::new(),
            dependencies: vec!["train".to_string()],
        },
    ];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .unwrap();

    let got: MLPipeline = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
    assert_eq!(got.status, PipelineStatus::Completed);
    assert_eq!(got.step_results.len(), 3);

    // Verify execution order (preprocess -> train -> evaluate)
    let preprocess_result = got.step_results.get("preprocess").unwrap();
    let train_result = got.step_results.get("train").unwrap();
    let evaluate_result = got.step_results.get("evaluate").unwrap();

    assert_eq!(preprocess_result.status, StepStatus::Completed);
    assert_eq!(train_result.status, StepStatus::Completed);
    assert_eq!(evaluate_result.status, StepStatus::Completed);

    // Verify dependencies were respected (preprocess completed before train)
    assert!(preprocess_result.completed_at.unwrap() <= train_result.started_at.unwrap());
    assert!(train_result.completed_at.unwrap() <= evaluate_result.started_at.unwrap());
}

#[tokio::test]
async fn test_execute_pipeline_status_tracking() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    }];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();

    // Check initial status
    let status = manager
        .get_pipeline_status(pipeline.id.as_str())
        .await
        .unwrap();
    assert_eq!(status, PipelineStatus::Created);

    // Execute and check final status
    manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .unwrap();
    let status = manager
        .get_pipeline_status(pipeline.id.as_str())
        .await
        .unwrap();
    assert_eq!(status, PipelineStatus::Completed);
}

#[tokio::test]
async fn test_get_pipeline() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    }];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    let got: MLPipeline = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
    assert_eq!(got.id, pipeline.id);
    assert_eq!(got.name, "pipeline1");
}

#[tokio::test]
async fn test_get_pipeline_not_found() {
    let manager = MLPipelineManager::new();
    let result = manager.get_pipeline("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pipelines() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    }];

    manager
        .create_pipeline("pipeline1", steps.clone())
        .await
        .unwrap();
    manager.create_pipeline("pipeline2", steps).await.unwrap();

    let pipelines = manager.list_pipelines().await;
    assert_eq!(pipelines.len(), 2);
}

#[tokio::test]
async fn test_get_pipeline_status() {
    let manager = MLPipelineManager::new();

    let steps = vec![PipelineStep {
        id: "step1".to_string(),
        step_type: StepType::Preprocessing,
        config: HashMap::new(),
        dependencies: vec![],
    }];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    let status = manager
        .get_pipeline_status(pipeline.id.as_str())
        .await
        .unwrap();
    assert_eq!(status, PipelineStatus::Created);

    manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .unwrap();
    let status = manager
        .get_pipeline_status(pipeline.id.as_str())
        .await
        .unwrap();
    assert_eq!(status, PipelineStatus::Completed);
}

#[tokio::test]
async fn test_pipeline_with_all_step_types() {
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
        PipelineStep {
            id: "evaluate".to_string(),
            step_type: StepType::Evaluation,
            config: HashMap::new(),
            dependencies: vec!["train".to_string()],
        },
        PipelineStep {
            id: "deploy".to_string(),
            step_type: StepType::Deployment,
            config: HashMap::new(),
            dependencies: vec!["evaluate".to_string()],
        },
    ];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    manager
        .execute_pipeline(pipeline.id.as_str())
        .await
        .unwrap();

    let got: MLPipeline = manager.get_pipeline(pipeline.id.as_str()).await.unwrap();
    assert_eq!(got.status, PipelineStatus::Completed);
    assert_eq!(got.step_results.len(), 4);
}

#[tokio::test]
async fn test_pipeline_step_config() {
    let manager = MLPipelineManager::new();

    let mut config = HashMap::new();
    config.insert("batch_size".to_string(), "32".to_string());
    config.insert("learning_rate".to_string(), "0.001".to_string());

    let steps = vec![PipelineStep {
        id: "train".to_string(),
        step_type: StepType::Training,
        config,
        dependencies: vec![],
    }];

    let pipeline = manager.create_pipeline("pipeline1", steps).await.unwrap();
    assert_eq!(pipeline.steps[0].config.len(), 2);
    assert_eq!(
        pipeline.steps[0].config.get("batch_size"),
        Some(&"32".to_string())
    );
}
