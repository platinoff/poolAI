//! Circuit Breaker Pattern for Distributed RAID
//!
//! This module provides circuit breaker functionality to prevent cascading failures
//! and improve resilience in distributed RAID operations.
//!
//! The circuit breaker has three states:
//! - **Closed**: Normal operation, requests pass through
//! - **Open**: Node is failing, requests are rejected immediately
//! - **Half-Open**: Testing if node has recovered, limited requests allowed

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation - requests pass through
    Closed,
    /// Node is failing - requests are rejected immediately
    Open,
    /// Testing if node has recovered - limited requests allowed
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of consecutive failures before opening circuit
    pub failure_threshold: u32,
    /// Timeout in seconds before attempting recovery (half-open state)
    pub timeout_seconds: u64,
    /// Number of successful requests needed to close circuit from half-open
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_seconds: 60,
            success_threshold: 2,
        }
    }
}

/// Circuit breaker for a single node
pub struct CircuitBreaker {
    /// Node ID this circuit breaker monitors
    node_id: u64,
    /// Current state
    state: Arc<RwLock<CircuitState>>,
    /// Configuration
    config: CircuitBreakerConfig,
    /// Consecutive failure count
    failure_count: Arc<RwLock<u32>>,
    /// Success count (for half-open state)
    success_count: Arc<RwLock<u32>>,
    /// Time when circuit was opened
    opened_at: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker for a node
    pub fn new(node_id: u64, config: CircuitBreakerConfig) -> Self {
        Self {
            node_id,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            config,
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            opened_at: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a circuit breaker with default configuration
    pub fn with_defaults(node_id: u64) -> Self {
        Self::new(node_id, CircuitBreakerConfig::default())
    }

    /// Get current state
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }

    /// Check if request should be allowed
    pub async fn allow_request(&self) -> Result<(), AppError> {
        let state = *self.state.read().await;

        match state {
            CircuitState::Closed => {
                // Normal operation - allow request
                Ok(())
            }
            CircuitState::Open => {
                // Check if timeout has passed
                let opened_at = *self.opened_at.read().await;
                if let Some(opened_time) = opened_at {
                    let elapsed = Utc::now().signed_duration_since(opened_time);
                    if elapsed.num_seconds() >= self.config.timeout_seconds as i64 {
                        // Transition to half-open
                        *self.state.write().await = CircuitState::HalfOpen;
                        *self.success_count.write().await = 0;
                        info!(
                            "Circuit breaker for node {} transitioning to HalfOpen",
                            self.node_id
                        );
                        Ok(())
                    } else {
                        // Still in timeout period
                        warn!(
                            "Circuit breaker for node {} is Open - rejecting request",
                            self.node_id
                        );
                        Err(AppError::NetworkError(format!(
                            "Circuit breaker is open for node {}",
                            self.node_id
                        )))
                    }
                } else {
                    // Should not happen, but handle gracefully
                    warn!(
                        "Circuit breaker for node {} is Open but opened_at is None",
                        self.node_id
                    );
                    Err(AppError::NetworkError(format!(
                        "Circuit breaker is open for node {}",
                        self.node_id
                    )))
                }
            }
            CircuitState::HalfOpen => {
                // Allow limited requests to test recovery
                Ok(())
            }
        }
    }

    /// Record a successful request
    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        let mut failure_count = self.failure_count.write().await;
        let mut success_count = self.success_count.write().await;

        match *state {
            CircuitState::Closed => {
                // Reset failure count on success
                *failure_count = 0;
                debug!(
                    "Circuit breaker for node {}: success in Closed state",
                    self.node_id
                );
            }
            CircuitState::Open => {
                // Should not happen, but handle gracefully
                warn!(
                    "Circuit breaker for node {}: success recorded in Open state",
                    self.node_id
                );
            }
            CircuitState::HalfOpen => {
                // Increment success count
                *success_count += 1;
                debug!(
                    "Circuit breaker for node {}: success in HalfOpen state (count: {})",
                    self.node_id, *success_count
                );

                // If we've reached the success threshold, close the circuit
                if *success_count >= self.config.success_threshold {
                    *state = CircuitState::Closed;
                    *failure_count = 0;
                    *success_count = 0;
                    *self.opened_at.write().await = None;
                    info!(
                        "Circuit breaker for node {} transitioning to Closed",
                        self.node_id
                    );
                }
            }
        }
    }

    /// Record a failed request
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        let mut failure_count = self.failure_count.write().await;
        let mut success_count = self.success_count.write().await;

        match *state {
            CircuitState::Closed => {
                // Increment failure count
                *failure_count += 1;
                debug!(
                    "Circuit breaker for node {}: failure in Closed state (count: {})",
                    self.node_id, *failure_count
                );

                // If we've reached the failure threshold, open the circuit
                if *failure_count >= self.config.failure_threshold {
                    *state = CircuitState::Open;
                    *self.opened_at.write().await = Some(Utc::now());
                    info!(
                        "Circuit breaker for node {} transitioning to Open ({} failures)",
                        self.node_id, *failure_count
                    );
                }
            }
            CircuitState::Open => {
                // Already open, just update timestamp
                *self.opened_at.write().await = Some(Utc::now());
                debug!(
                    "Circuit breaker for node {}: failure in Open state",
                    self.node_id
                );
            }
            CircuitState::HalfOpen => {
                // Failure in half-open means node is still failing - open circuit
                *state = CircuitState::Open;
                *failure_count = self.config.failure_threshold;
                *success_count = 0;
                *self.opened_at.write().await = Some(Utc::now());
                warn!(
                    "Circuit breaker for node {} transitioning back to Open from HalfOpen",
                    self.node_id
                );
            }
        }
    }

    /// Get failure count
    pub async fn failure_count(&self) -> u32 {
        *self.failure_count.read().await
    }

    /// Get success count (for half-open state)
    pub async fn success_count(&self) -> u32 {
        *self.success_count.read().await
    }

    /// Get time when circuit was opened
    pub async fn opened_at(&self) -> Option<DateTime<Utc>> {
        *self.opened_at.read().await
    }

    /// Manually reset circuit breaker to Closed state
    pub async fn reset(&self) {
        *self.state.write().await = CircuitState::Closed;
        *self.failure_count.write().await = 0;
        *self.success_count.write().await = 0;
        *self.opened_at.write().await = None;
        info!("Circuit breaker for node {} manually reset to Closed", self.node_id);
    }

    /// Get node ID
    pub fn node_id(&self) -> u64 {
        self.node_id
    }
}

/// Circuit breaker manager for multiple nodes
pub struct CircuitBreakerManager {
    /// Circuit breakers by node ID
    breakers: Arc<RwLock<std::collections::HashMap<u64, Arc<CircuitBreaker>>>>,
    /// Default configuration
    default_config: CircuitBreakerConfig,
}

impl CircuitBreakerManager {
    /// Create a new circuit breaker manager
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            breakers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            default_config: config,
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }

    /// Get or create circuit breaker for a node
    pub async fn get_or_create(&self, node_id: u64) -> Arc<CircuitBreaker> {
        let mut breakers = self.breakers.write().await;
        
        if let Some(breaker) = breakers.get(&node_id) {
            return breaker.clone();
        }

        let breaker = Arc::new(CircuitBreaker::new(node_id, self.default_config.clone()));
        breakers.insert(node_id, breaker.clone());
        breaker
    }

    /// Get circuit breaker for a node (returns None if not exists)
    pub async fn get(&self, node_id: u64) -> Option<Arc<CircuitBreaker>> {
        let breakers = self.breakers.read().await;
        breakers.get(&node_id).cloned()
    }

    /// Remove circuit breaker for a node
    pub async fn remove(&self, node_id: u64) {
        let mut breakers = self.breakers.write().await;
        breakers.remove(&node_id);
        info!("Removed circuit breaker for node {}", node_id);
    }

    /// Get all circuit breakers
    pub async fn all_breakers(&self) -> Vec<Arc<CircuitBreaker>> {
        let breakers = self.breakers.read().await;
        breakers.values().cloned().collect()
    }

    /// Get circuit breaker states for all nodes
    pub async fn get_states(&self) -> std::collections::HashMap<u64, CircuitState> {
        let breakers = self.breakers.read().await;
        let mut states = std::collections::HashMap::new();
        
        for (node_id, breaker) in breakers.iter() {
            states.insert(*node_id, breaker.state().await);
        }
        
        states
    }
}

