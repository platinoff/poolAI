//! Integration tests for Federated Learning Module (ML.3)
//!
//! Tests the federated learning pipeline functionality including client updates,
//! aggregation (FedAvg, FedProx), round management, and error handling.

use poolai::ml::federated::{
    AggregatedModel, AggregationMode, ClientUpdate, FederatedConfig, FederatedLearningPipeline,
};

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
async fn test_federated_round_management() {
    let pipeline = FederatedLearningPipeline::default();

    assert_eq!(pipeline.get_current_round().await, 0);

    let round1 = pipeline.start_round().await;
    assert_eq!(round1, 1);
    assert_eq!(pipeline.get_current_round().await, 1);

    let round2 = pipeline.start_round().await;
    assert_eq!(round2, 2);
    assert_eq!(pipeline.get_current_round().await, 2);
}

#[tokio::test]
async fn test_federated_add_multiple_updates() {
    let pipeline = FederatedLearningPipeline::default();

    for i in 0..5 {
        let update = ClientUpdate {
            client_id: format!("client{}", i),
            model_weights: vec![0.5, 0.3, 0.2],
            sample_count: 100 + (i * 10),
            round: 0,
        };

        let result = pipeline.add_client_update(update).await;
        assert!(result.is_ok());
    }

    let count = pipeline.get_pending_updates_count().await;
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_federated_aggregate_fedavg() {
    let mut config = FederatedConfig::default_config();
    config.aggregation = AggregationMode::FedAvg;
    config.min_clients_per_round = 2;

    let pipeline = FederatedLearningPipeline::new(config);

    // Add updates with different weights
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

    // Verify weighted average: (1.0*100 + 2.0*200)/300 ≈ 1.67
    assert!(model.weights[0] > 1.5 && model.weights[0] < 2.0);
    assert!(model.weights[1] > 2.5 && model.weights[1] < 3.0);
    assert!(model.weights[2] > 3.5 && model.weights[2] < 4.0);
}

#[tokio::test]
async fn test_federated_aggregate_fedprox() {
    let mut config = FederatedConfig::default_config();
    config.aggregation = AggregationMode::FedProx;
    config.min_clients_per_round = 2;

    let pipeline = FederatedLearningPipeline::new(config);

    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.0, 2.0, 3.0],
        sample_count: 100,
        round: 0,
    };

    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.0, 3.0, 4.0],
        sample_count: 100,
        round: 0,
    };

    pipeline.add_client_update(update1).await.unwrap();
    pipeline.add_client_update(update2).await.unwrap();

    let aggregated = pipeline.aggregate_updates().await;
    assert!(aggregated.is_ok());

    let model = aggregated.unwrap();
    assert_eq!(model.aggregation_mode, AggregationMode::FedProx);
    assert_eq!(model.clients_count, 2);
}

#[tokio::test]
async fn test_federated_multiple_rounds() {
    let mut config = FederatedConfig::default_config();
    config.min_clients_per_round = 2;

    let pipeline = FederatedLearningPipeline::new(config);

    // Round 1
    pipeline.start_round().await;
    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.0, 2.0],
        sample_count: 100,
        round: 1,
    };
    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.0, 3.0],
        sample_count: 100,
        round: 1,
    };

    pipeline.add_client_update(update1).await.unwrap();
    pipeline.add_client_update(update2).await.unwrap();

    let aggregated1: AggregatedModel = pipeline.aggregate_updates().await.unwrap();
    assert_eq!(aggregated1.round, 1);

    // Round 2
    pipeline.start_round().await;
    let update3 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.5, 2.5],
        sample_count: 150,
        round: 2,
    };
    let update4 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.5, 3.5],
        sample_count: 150,
        round: 2,
    };

    pipeline.add_client_update(update3).await.unwrap();
    pipeline.add_client_update(update4).await.unwrap();

    let aggregated2: AggregatedModel = pipeline.aggregate_updates().await.unwrap();
    assert_eq!(aggregated2.round, 2);

    // Verify both rounds are stored
    let model1 = pipeline.get_round_model(1).await;
    assert!(model1.is_some());

    let model2 = pipeline.get_round_model(2).await;
    assert!(model2.is_some());
}

#[tokio::test]
async fn test_federated_round_mismatch() {
    let pipeline = FederatedLearningPipeline::default();

    let update = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![0.5, 0.3],
        sample_count: 100,
        round: 5, // Wrong round
    };

    let result = pipeline.add_client_update(update).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_federated_insufficient_clients() {
    let mut config = FederatedConfig::default_config();
    config.min_clients_per_round = 5;

    let pipeline = FederatedLearningPipeline::new(config);

    for i in 0..3 {
        let update = ClientUpdate {
            client_id: format!("client{}", i),
            model_weights: vec![0.5, 0.3],
            sample_count: 100,
            round: 0,
        };
        pipeline.add_client_update(update).await.unwrap();
    }

    let result = pipeline.aggregate_updates().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_federated_dimension_mismatch() {
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
async fn test_federated_is_ready_for_aggregation() {
    let mut config = FederatedConfig::default_config();
    config.min_clients_per_round = 3;
    config.max_clients_per_round = 5;

    let pipeline = FederatedLearningPipeline::new(config);

    assert!(!pipeline.is_ready_for_aggregation().await);

    // Add 2 clients (not enough)
    for i in 0..2 {
        let update = ClientUpdate {
            client_id: format!("client{}", i),
            model_weights: vec![0.5, 0.3],
            sample_count: 100,
            round: 0,
        };
        pipeline.add_client_update(update).await.unwrap();
    }

    assert!(!pipeline.is_ready_for_aggregation().await);

    // Add 1 more (now 3, should be ready)
    let update = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![0.5, 0.3],
        sample_count: 100,
        round: 0,
    };
    pipeline.add_client_update(update).await.unwrap();

    assert!(pipeline.is_ready_for_aggregation().await);
}

#[tokio::test]
async fn test_federated_large_model() {
    let pipeline = FederatedLearningPipeline::default();

    // Create large model weights
    let large_weights: Vec<f64> = (0..1000).map(|i| i as f64 / 1000.0).collect();

    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: large_weights.clone(),
        sample_count: 100,
        round: 0,
    };

    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: large_weights.clone(),
        sample_count: 200,
        round: 0,
    };

    pipeline.add_client_update(update1).await.unwrap();
    pipeline.add_client_update(update2).await.unwrap();

    let aggregated = pipeline.aggregate_updates().await;
    assert!(aggregated.is_ok());

    let model = aggregated.unwrap();
    assert_eq!(model.weights.len(), 1000);
}

#[tokio::test]
async fn test_federated_empty_aggregation() {
    let pipeline = FederatedLearningPipeline::default();

    let result = pipeline.aggregate_updates().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_federated_get_config() {
    let config = FederatedConfig::default_config();
    let pipeline = FederatedLearningPipeline::new(config.clone());

    let retrieved_config = pipeline.get_config();
    assert_eq!(
        retrieved_config.min_clients_per_round,
        config.min_clients_per_round
    );
    assert_eq!(retrieved_config.aggregation, config.aggregation);
}

#[tokio::test]
async fn test_federated_weighted_averaging() {
    let mut config = FederatedConfig::default_config();
    config.min_clients_per_round = 2;

    let pipeline = FederatedLearningPipeline::new(config);

    // Client 1: 100 samples, weight = 0.5
    let update1 = ClientUpdate {
        client_id: "client1".to_string(),
        model_weights: vec![1.0],
        sample_count: 100,
        round: 0,
    };

    // Client 2: 300 samples, weight = 0.75
    let update2 = ClientUpdate {
        client_id: "client2".to_string(),
        model_weights: vec![2.0],
        sample_count: 300,
        round: 0,
    };

    pipeline.add_client_update(update1).await.unwrap();
    pipeline.add_client_update(update2).await.unwrap();

    let aggregated = pipeline.aggregate_updates().await.unwrap();

    // Weighted average: (1.0*100 + 2.0*300)/400 = 700/400 = 1.75
    assert!((aggregated.weights[0] - 1.75).abs() < 0.01);
}
