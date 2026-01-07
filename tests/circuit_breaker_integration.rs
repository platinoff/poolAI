//! Integration tests for Circuit Breaker Pattern

use poolai::raid::circuit_breaker::{
    CircuitBreaker, CircuitBreakerConfig, CircuitBreakerManager, CircuitState,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_circuit_breaker_creation() {
    let breaker = CircuitBreaker::with_defaults(1);
    assert_eq!(breaker.state().await, CircuitState::Closed);
    assert_eq!(breaker.failure_count().await, 0);
}

#[tokio::test]
async fn test_circuit_breaker_closed_to_open() {
    let breaker = CircuitBreaker::with_defaults(1);
    let config = CircuitBreakerConfig::default();

    // Record failures up to threshold
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }

    // Circuit should be open now
    assert_eq!(breaker.state().await, CircuitState::Open);
    assert_eq!(breaker.failure_count().await, config.failure_threshold);

    // Requests should be rejected
    let result = breaker.allow_request().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_circuit_breaker_open_to_half_open() {
    let mut config = CircuitBreakerConfig::default();
    config.timeout_seconds = 1; // Short timeout for testing
    let breaker = CircuitBreaker::new(1, config.clone());

    // Open the circuit
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Wait for timeout
    sleep(Duration::from_secs(2)).await;

    // Request should transition to half-open
    let result = breaker.allow_request().await;
    assert!(result.is_ok());
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_to_closed() {
    let mut config = CircuitBreakerConfig::default();
    config.timeout_seconds = 1;
    config.success_threshold = 2;
    let breaker = CircuitBreaker::new(1, config.clone());

    // Open the circuit
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Wait for timeout and transition to half-open
    sleep(Duration::from_secs(2)).await;
    breaker.allow_request().await.unwrap();
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    // Record successes up to threshold
    for _ in 0..config.success_threshold {
        breaker.record_success().await;
    }

    // Circuit should be closed now
    assert_eq!(breaker.state().await, CircuitState::Closed);
    assert_eq!(breaker.failure_count().await, 0);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_to_open() {
    let mut config = CircuitBreakerConfig::default();
    config.timeout_seconds = 1;
    let breaker = CircuitBreaker::new(1, config.clone());

    // Open the circuit
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }

    // Wait for timeout and transition to half-open
    sleep(Duration::from_secs(2)).await;
    breaker.allow_request().await.unwrap();
    assert_eq!(breaker.state().await, CircuitState::HalfOpen);

    // Record a failure in half-open state
    breaker.record_failure().await;

    // Circuit should be open again
    assert_eq!(breaker.state().await, CircuitState::Open);
}

#[tokio::test]
async fn test_circuit_breaker_reset() {
    let breaker = CircuitBreaker::with_defaults(1);
    let config = CircuitBreakerConfig::default();

    // Open the circuit
    for _ in 0..config.failure_threshold {
        breaker.record_failure().await;
    }
    assert_eq!(breaker.state().await, CircuitState::Open);

    // Reset
    breaker.reset().await;
    assert_eq!(breaker.state().await, CircuitState::Closed);
    assert_eq!(breaker.failure_count().await, 0);
}

#[tokio::test]
async fn test_circuit_breaker_manager() {
    let manager = CircuitBreakerManager::with_defaults();

    // Get or create breakers for different nodes
    let breaker1 = manager.get_or_create(1).await;
    let breaker2 = manager.get_or_create(2).await;
    let breaker1_again = manager.get_or_create(1).await;

    // Should return the same instance for node 1
    assert_eq!(breaker1.node_id(), breaker1_again.node_id());
    assert_ne!(breaker1.node_id(), breaker2.node_id());

    // Get states
    let states = manager.get_states().await;
    assert_eq!(states.len(), 2);
    assert_eq!(states.get(&1), Some(&CircuitState::Closed));
    assert_eq!(states.get(&2), Some(&CircuitState::Closed));
}

#[tokio::test]
async fn test_circuit_breaker_success_resets_failure_count() {
    let breaker = CircuitBreaker::with_defaults(1);

    // Record some failures
    breaker.record_failure().await;
    breaker.record_failure().await;
    assert_eq!(breaker.failure_count().await, 2);

    // Record success - should reset failure count
    breaker.record_success().await;
    assert_eq!(breaker.failure_count().await, 0);
    assert_eq!(breaker.state().await, CircuitState::Closed);
}
