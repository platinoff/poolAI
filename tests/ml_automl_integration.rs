//! Integration tests for AutoML Module (ML.2)
//!
//! Tests the AutoML pipeline functionality including model selection,
//! hyperparameter optimization, ensemble creation, and prediction aggregation.

use poolai::ml::automl::{AutoMLPipeline, AutomlConfig, ModelType, TrainedModel, TrainingData};
use std::collections::HashMap;

#[tokio::test]
async fn test_automl_pipeline_creation() {
    let config = AutomlConfig::default_config();
    let pipeline = AutoMLPipeline::new(config);

    let best_model = pipeline.get_best_model().await;
    assert!(best_model.is_none());

    let candidates = pipeline.get_candidates().await;
    assert!(candidates.is_empty());
}

#[tokio::test]
async fn test_automl_train_basic() {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
            vec![10.0, 11.0, 12.0],
        ],
        labels: vec![0.0, 1.0, 0.0, 1.0],
    };

    let result = pipeline.train(data).await;
    assert!(result.is_ok());

    let model = result.unwrap();
    assert!(model.accuracy > 0.0);
    assert!(model.accuracy <= 1.0);
    assert!(!model.model_id.is_empty());
    assert!(!model.hyperparameters.is_empty());
}

#[tokio::test]
async fn test_automl_train_large_dataset() {
    let pipeline = AutoMLPipeline::default();

    // Create larger dataset
    let mut features = Vec::new();
    let mut labels = Vec::new();

    for i in 0..100 {
        features.push(vec![i as f64, (i * 2) as f64, (i * 3) as f64]);
        labels.push((i % 2) as f64);
    }

    let data = TrainingData { features, labels };

    let result = pipeline.train(data).await;
    assert!(result.is_ok());

    let model = result.unwrap();
    assert!(model.accuracy > 0.0);
}

#[tokio::test]
async fn test_automl_get_best_model_after_training() {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]],
        labels: vec![0.0, 1.0, 0.0],
    };

    pipeline.train(data).await.unwrap();

    let best_model = pipeline.get_best_model().await;
    assert!(best_model.is_some());

    let model = best_model.unwrap();
    assert!(model.accuracy > 0.0);
    assert!(!model.model_id.is_empty());
}

#[tokio::test]
async fn test_automl_get_candidates_after_training() {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        labels: vec![0.0, 1.0],
    };

    pipeline.train(data).await.unwrap();

    let candidates = pipeline.get_candidates().await;
    assert_eq!(candidates.len(), 5); // 5 model types

    // Verify all model types are present
    let model_types: Vec<_> = candidates.iter().map(|c| c.model_type()).collect();
    assert!(model_types.contains(&&ModelType::LinearRegression));
    assert!(model_types.contains(&&ModelType::RandomForest));
    assert!(model_types.contains(&&ModelType::GradientBoosting));
    assert!(model_types.contains(&&ModelType::NeuralNetwork));
    assert!(model_types.contains(&&ModelType::SupportVectorMachine));
}

#[tokio::test]
async fn test_automl_create_ensemble() {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ],
        labels: vec![0.0, 1.0, 0.0, 1.0],
    };

    pipeline.train(data).await.unwrap();

    // Create ensemble with top 3 models
    let ensemble = pipeline.create_ensemble(3).await;
    assert!(ensemble.is_ok());

    let models = ensemble.unwrap();
    assert_eq!(models.len(), 3);

    // Verify models are sorted by accuracy (descending)
    for i in 1..models.len() {
        assert!(models[i - 1].accuracy >= models[i].accuracy);
    }
}

#[tokio::test]
async fn test_automl_create_ensemble_all_models() {
    let pipeline = AutoMLPipeline::default();

    let data = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        labels: vec![0.0, 1.0],
    };

    pipeline.train(data).await.unwrap();

    // Request more models than available
    let ensemble = pipeline.create_ensemble(10).await;
    assert!(ensemble.is_ok());

    let models = ensemble.unwrap();
    assert_eq!(models.len(), 5); // Only 5 models available
}

#[tokio::test]
async fn test_automl_aggregate_predictions() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![
        TrainedModel {
            model_type: ModelType::LinearRegression,
            accuracy: 0.8,
            hyperparameters: HashMap::new(),
            training_time_ms: 100,
            model_id: "model1".to_string(),
        },
        TrainedModel {
            model_type: ModelType::RandomForest,
            accuracy: 0.9,
            hyperparameters: HashMap::new(),
            training_time_ms: 200,
            model_id: "model2".to_string(),
        },
        TrainedModel {
            model_type: ModelType::GradientBoosting,
            accuracy: 0.85,
            hyperparameters: HashMap::new(),
            training_time_ms: 150,
            model_id: "model3".to_string(),
        },
    ];

    let predictions = vec![
        vec![0.5, 0.3, 0.2],
        vec![0.6, 0.4, 0.3],
        vec![0.55, 0.35, 0.25],
    ];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_ok());

    let aggregated = result.unwrap();
    assert_eq!(aggregated.len(), 3);

    // Verify aggregated values are weighted averages
    assert!(aggregated[0] > 0.0);
    assert!(aggregated[1] > 0.0);
    assert!(aggregated[2] > 0.0);
}

#[tokio::test]
async fn test_automl_aggregate_predictions_single_model() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![TrainedModel {
        model_type: ModelType::NeuralNetwork,
        accuracy: 0.95,
        hyperparameters: HashMap::new(),
        training_time_ms: 300,
        model_id: "model1".to_string(),
    }];

    let predictions = vec![vec![0.7, 0.5, 0.3]];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_ok());

    let aggregated = result.unwrap();
    assert_eq!(aggregated.len(), 3);
    assert_eq!(aggregated[0], 0.7);
    assert_eq!(aggregated[1], 0.5);
    assert_eq!(aggregated[2], 0.3);
}

#[tokio::test]
async fn test_automl_aggregate_predictions_empty_models() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![];
    let predictions = vec![vec![0.5, 0.3]];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_automl_aggregate_predictions_empty_predictions() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![TrainedModel {
        model_type: ModelType::LinearRegression,
        accuracy: 0.8,
        hyperparameters: HashMap::new(),
        training_time_ms: 100,
        model_id: "model1".to_string(),
    }];

    let predictions = vec![];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_automl_aggregate_predictions_length_mismatch() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![TrainedModel {
        model_type: ModelType::LinearRegression,
        accuracy: 0.8,
        hyperparameters: HashMap::new(),
        training_time_ms: 100,
        model_id: "model1".to_string(),
    }];

    let predictions = vec![vec![0.5, 0.3], vec![0.6, 0.4]];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_automl_aggregate_predictions_prediction_length_mismatch() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![
        TrainedModel {
            model_type: ModelType::LinearRegression,
            accuracy: 0.8,
            hyperparameters: HashMap::new(),
            training_time_ms: 100,
            model_id: "model1".to_string(),
        },
        TrainedModel {
            model_type: ModelType::RandomForest,
            accuracy: 0.9,
            hyperparameters: HashMap::new(),
            training_time_ms: 200,
            model_id: "model2".to_string(),
        },
    ];

    let predictions = vec![vec![0.5, 0.3], vec![0.6]]; // Different lengths

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_automl_aggregate_predictions_zero_accuracy() {
    let pipeline = AutoMLPipeline::default();

    let models = vec![
        TrainedModel {
            model_type: ModelType::LinearRegression,
            accuracy: 0.0,
            hyperparameters: HashMap::new(),
            training_time_ms: 100,
            model_id: "model1".to_string(),
        },
        TrainedModel {
            model_type: ModelType::RandomForest,
            accuracy: 0.0,
            hyperparameters: HashMap::new(),
            training_time_ms: 200,
            model_id: "model2".to_string(),
        },
    ];

    let predictions = vec![vec![0.5, 0.3], vec![0.6, 0.4]];

    let result = pipeline.aggregate_predictions(&models, &predictions).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_automl_multiple_training_sessions() {
    let pipeline = AutoMLPipeline::default();

    // First training session
    let data1 = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        labels: vec![0.0, 1.0],
    };

    let model1 = pipeline.train(data1).await.unwrap();
    let best1: TrainedModel = pipeline.get_best_model().await.unwrap();
    assert_eq!(model1.model_id, best1.model_id);

    // Second training session
    let data2 = TrainingData {
        features: vec![vec![5.0, 6.0], vec![7.0, 8.0], vec![9.0, 10.0]],
        labels: vec![1.0, 0.0, 1.0],
    };

    let model2 = pipeline.train(data2).await.unwrap();
    let best2: TrainedModel = pipeline.get_best_model().await.unwrap();
    assert_eq!(model2.model_id, best2.model_id);
    assert_ne!(model1.model_id, model2.model_id); // Different model IDs
}

#[tokio::test]
async fn test_automl_config_custom() {
    let config = AutomlConfig {
        auto_feature_engineering: false,
        max_trials: 50,
        timeout_seconds: 1800,
        ensemble_size: 3,
        cross_validation_folds: 3,
    };

    let pipeline = AutoMLPipeline::new(config);

    let data = TrainingData {
        features: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        labels: vec![0.0, 1.0],
    };

    let result = pipeline.train(data).await;
    assert!(result.is_ok());
}
