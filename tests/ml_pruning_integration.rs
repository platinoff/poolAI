//! Integration tests for Pruning Strategies Module (ML.1)
//!
//! Tests the pruning functionality including magnitude-based, structured,
//! unstructured pruning, iterative pruning, and evaluation.

use poolai::ml::optimization::{
    apply_iterative_pruning, apply_pruning, evaluate_pruning, PruningConfig, PruningStrategy,
};

#[test]
fn test_pruning_config_default() {
    let config = PruningConfig::default_config();
    assert_eq!(config.strategy, PruningStrategy::MagnitudeBased);
    assert!((config.ratio - 0.1).abs() < 1e-6);
    assert!(!config.iterative);
}

#[test]
fn test_magnitude_based_pruning_basic() {
    let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::MagnitudeBased;
    config.ratio = 0.25; // Prune 25% (2 weights)

    let result = apply_pruning(&weights, &config);
    assert_eq!(result.strategy, PruningStrategy::MagnitudeBased);
    assert!(result.pruned_count > 0);
    // weights_after = count of retained non-zero weights (same vec length, sparse zeros)
    assert!(result.weights_after < result.weights_before);
    assert!(result.compression_ratio > 1.0);
    assert!(result.accuracy_drop >= 0.0);
}

#[test]
fn test_magnitude_based_pruning_large() {
    let weights: Vec<f64> = (0..1000).map(|i| (i % 10) as f64).collect();
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::MagnitudeBased;
    config.ratio = 0.5; // Prune 50%

    let result = apply_pruning(&weights, &config);
    assert_eq!(result.pruned_count, 500);
    assert_eq!(result.weights_before, 1000);
    assert!(result.weights_after < result.weights_before);
    assert!(result.weights_after > 0);
}

#[test]
fn test_structured_pruning() {
    let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::Structured;
    config.ratio = 0.25;

    let result = apply_pruning(&weights, &config);
    assert_eq!(result.strategy, PruningStrategy::Structured);
    assert!(result.pruned_count > 0);
}

#[test]
fn test_unstructured_pruning() {
    let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::Unstructured;
    config.ratio = 0.33; // Prune 33%

    let result = apply_pruning(&weights, &config);
    assert_eq!(result.strategy, PruningStrategy::Unstructured);
    assert!(result.pruned_count > 0);
}

#[test]
fn test_pruning_zero_ratio() {
    let weights = vec![1.0, 2.0, 3.0, 4.0];
    let mut config = PruningConfig::default_config();
    config.ratio = 0.0;

    let result = apply_pruning(&weights, &config);
    assert_eq!(result.pruned_count, 0);
    assert_eq!(result.weights_before, result.weights_after);
    assert!((result.compression_ratio - 1.0).abs() < 1e-6);
}

#[test]
fn test_pruning_full_ratio() {
    let weights = vec![1.0, 2.0, 3.0];
    let mut config = PruningConfig::default_config();
    config.ratio = 1.0; // Prune 100%

    let result = apply_pruning(&weights, &config);
    assert!(result.pruned_count > 0);
    assert_eq!(result.weights_before, 3);
    assert_eq!(result.weights_after, 0);
}

#[test]
fn test_iterative_pruning() {
    let weights: Vec<f64> = (0..100).map(|i| (i % 10) as f64).collect();
    let mut config = PruningConfig::default_config();
    config.iterative = true;
    config.iterations = 3;
    config.ratio = 0.3; // Total 30%, 10% per iteration

    let result = apply_iterative_pruning(&weights, &config);
    assert!(result.pruned_count > 0);
    // Note: weights_after may equal weights_before if we zero out weights
    assert!(result.weights_after <= result.weights_before);
    assert!(result.accuracy_drop > 0.0);
}

#[test]
fn test_iterative_pruning_not_iterative() {
    let weights = vec![1.0, 2.0, 3.0];
    let mut config = PruningConfig::default_config();
    config.iterative = false;

    let result = apply_iterative_pruning(&weights, &config);
    assert_eq!(result.strategy, PruningStrategy::MagnitudeBased);
}

#[test]
fn test_iterative_pruning_zero_iterations() {
    let weights = vec![1.0, 2.0, 3.0];
    let mut config = PruningConfig::default_config();
    config.iterative = true;
    config.iterations = 0;

    let result = apply_iterative_pruning(&weights, &config);
    assert_eq!(result.strategy, PruningStrategy::MagnitudeBased);
}

#[test]
fn test_evaluate_pruning() {
    let before = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let after = vec![1.0, 0.0, 3.0, 0.0, 5.0];

    let result = evaluate_pruning(&before, &after);
    assert_eq!(result.pruned_count, 2);
    assert_eq!(result.weights_before, 5);
    assert_eq!(result.weights_after, 5);
    assert!((result.compression_ratio - 1.0).abs() < 1e-6);
}

#[test]
fn test_evaluate_pruning_no_pruning() {
    let weights = vec![1.0, 2.0, 3.0];
    let result = evaluate_pruning(&weights, &weights);
    assert_eq!(result.pruned_count, 0);
    assert_eq!(result.weights_before, result.weights_after);
}

#[test]
fn test_evaluate_pruning_different_lengths() {
    let before = vec![1.0, 2.0, 3.0];
    let after = vec![1.0, 2.0];

    let result = evaluate_pruning(&before, &after);
    assert_eq!(result.pruned_count, 0);
    assert_ne!(result.weights_before, result.weights_after);
}

#[test]
fn test_pruning_all_strategies() {
    let weights: Vec<f64> = (0..100).map(|i| (i % 10) as f64).collect();

    for strategy in [
        PruningStrategy::MagnitudeBased,
        PruningStrategy::Structured,
        PruningStrategy::Unstructured,
    ] {
        let mut config = PruningConfig::default_config();
        config.strategy = strategy;
        config.ratio = 0.2;

        let result = apply_pruning(&weights, &config);
        assert_eq!(result.strategy, strategy);
        assert!(result.pruned_count > 0);
        assert!(result.weights_after < result.weights_before);
    }
}

#[test]
fn test_pruning_compression_ratio() {
    let weights: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    let mut config = PruningConfig::default_config();
    config.ratio = 0.5; // Prune 50%

    let result = apply_pruning(&weights, &config);
    assert!(result.weights_after < result.weights_before);
    assert!(result.compression_ratio > 1.0);
}

#[test]
fn test_pruning_accuracy_drop() {
    let weights = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut config = PruningConfig::default_config();
    config.ratio = 0.2; // 20% pruning

    let result = apply_pruning(&weights, &config);
    assert!(result.accuracy_drop > 0.0);
    assert!(result.accuracy_drop < 1.0);
}
