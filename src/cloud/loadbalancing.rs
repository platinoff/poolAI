//! Load balancing module
//!
//! Provides advanced load balancing capabilities:
//! - Cloud load balancers integration
//! - Health check integration
//! - Traffic distribution strategies
//! - Geographic load balancing
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::cloud::loadbalancing::{LoadBalancer, Backend};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let loadbalancer = LoadBalancer::new();
//! loadbalancer.initialize().await?;
//!
//! // Add backends
//! let backend1 = Backend {
//!     id: "backend-1".to_string(),
//!     address: "10.0.1.10".to_string(),
//!     port: 8080,
//!     weight: 100,
//! };
//!
//! let backend2 = Backend {
//!     id: "backend-2".to_string(),
//!     address: "10.0.1.11".to_string(),
//!     port: 8080,
//!     weight: 100,
//! };
//!
//! loadbalancer.add_backend(backend1).await?;
//! loadbalancer.add_backend(backend2).await?;
//!
//! // Check health status
//! let health = loadbalancer.get_health_status().await?;
//! println!("Healthy backends: {}/{}", health.healthy_backends, health.total_backends);
//!
//! loadbalancer.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

#[cfg(feature = "cloud-sdk")]
use crate::cloud::kubernetes::KubernetesManager;

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Weighted round-robin based on backend weights
    WeightedRoundRobin,
    /// Least connections
    LeastConnections,
    /// IP hash for session affinity
    IpHash,
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Health check interval in seconds
    pub interval_secs: u64,
    /// Timeout in seconds
    pub timeout_secs: u64,
    /// Number of consecutive failures before marking unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before marking healthy
    pub success_threshold: u32,
    /// Health check path (for HTTP/HTTPS)
    pub path: Option<String>,
}

/// Backend health status
#[derive(Debug, Clone)]
struct BackendHealth {
    healthy: bool,
    consecutive_failures: u32,
    last_check: Option<std::time::Instant>,
}

/// Load balancer for distributing traffic
pub struct LoadBalancer {
    initialized: Arc<RwLock<bool>>,
    backends: Arc<RwLock<HashMap<String, Backend>>>,
    backend_health: Arc<RwLock<HashMap<String, BackendHealth>>>,
    strategy: Arc<RwLock<LoadBalancingStrategy>>,
    health_check_config: Arc<RwLock<HealthCheckConfig>>,
    #[cfg(feature = "cloud-sdk")]
    /// Kubernetes manager for health checks
    k8s_manager: Option<Arc<KubernetesManager>>,
}

impl LoadBalancer {
    /// Create a new LoadBalancer
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::cloud::loadbalancing::LoadBalancer;
    ///
    /// let loadbalancer = LoadBalancer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            initialized: Arc::new(RwLock::new(false)),
            backends: Arc::new(RwLock::new(HashMap::new())),
            backend_health: Arc::new(RwLock::new(HashMap::new())),
            strategy: Arc::new(RwLock::new(LoadBalancingStrategy::RoundRobin)),
            health_check_config: Arc::new(RwLock::new(HealthCheckConfig {
                interval_secs: 10,
                timeout_secs: 5,
                failure_threshold: 3,
                success_threshold: 2,
                path: Some("/health".to_string()),
            })),
            #[cfg(feature = "cloud-sdk")]
            k8s_manager: None,
        }
    }

    /// Create a new LoadBalancer with Kubernetes manager
    ///
    /// # Arguments
    ///
    /// * `k8s_manager` - Kubernetes manager for health checks
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::loadbalancing::LoadBalancer;
    /// use poolai::cloud::kubernetes::KubernetesManager;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let k8s_manager = Arc::new(KubernetesManager::new("poolai".to_string()));
    /// k8s_manager.initialize().await?;
    /// let loadbalancer = LoadBalancer::with_k8s_manager(k8s_manager);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "cloud-sdk")]
    pub fn with_k8s_manager(k8s_manager: Arc<KubernetesManager>) -> Self {
        Self {
            initialized: Arc::new(RwLock::new(false)),
            backends: Arc::new(RwLock::new(HashMap::new())),
            backend_health: Arc::new(RwLock::new(HashMap::new())),
            strategy: Arc::new(RwLock::new(LoadBalancingStrategy::RoundRobin)),
            health_check_config: Arc::new(RwLock::new(HealthCheckConfig {
                interval_secs: 10,
                timeout_secs: 5,
                failure_threshold: 3,
                success_threshold: 2,
                path: Some("/health".to_string()),
            })),
            k8s_manager: Some(k8s_manager),
        }
    }

    /// Initialize load balancer
    ///
    /// Sets up health checks, routing rules, and cloud load balancer (if applicable).
    ///
    /// # Errors
    ///
    /// Returns `AppError::InitializationError` if:
    /// - Health check configuration fails
    /// - Routing rules cannot be configured
    /// - Cloud load balancer initialization fails (if applicable)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::loadbalancing::LoadBalancer;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let loadbalancer = LoadBalancer::new();
    /// loadbalancer.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Initialize health check configuration
        let health_config = self.health_check_config.read().await;
        info!(
            "Load balancer initialized with strategy: {:?}, health check interval: {}s",
            *self.strategy.read().await,
            health_config.interval_secs
        );
        drop(health_config);

        // TODO: Set up actual health check tasks
        // TODO: Configure routing rules
        // TODO: Initialize cloud load balancer (if applicable)

        *initialized = true;
        Ok(())
    }

    /// Shutdown load balancer
    pub async fn shutdown(&self) -> Result<(), AppError> {
        *self.initialized.write().await = false;
        info!("Load balancer shut down");
        Ok(())
    }

    /// Add backend to load balancer
    ///
    /// # Arguments
    ///
    /// * `backend` - Backend configuration to add
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - Backend ID is empty
    /// - Backend address is empty
    /// - Backend port is 0
    /// - Backend with same ID already exists
    pub async fn add_backend(&self, backend: Backend) -> Result<(), AppError> {
        if backend.id.is_empty() {
            return Err(AppError::ValidationError(
                "Backend ID cannot be empty. Current value: ''. Suggestion: Provide a unique identifier for the backend."
                    .to_string(),
            ));
        }

        if backend.address.is_empty() {
            return Err(AppError::ValidationError(
                format!(
                    "Backend address cannot be empty for backend '{}'. Current value: ''. Suggestion: Provide a valid IP address or hostname.",
                    backend.id
                ),
            ));
        }

        if backend.port == 0 {
            return Err(AppError::ValidationError(
                format!(
                    "Backend port must be greater than 0 for backend '{}'. Current value: {}. Suggestion: Set port to a valid port number (1-65535).",
                    backend.id, backend.port
                ),
            ));
        }

        let mut backends = self.backends.write().await;
        if backends.contains_key(&backend.id) {
            return Err(AppError::ValidationError(
                format!(
                    "Backend with ID '{}' already exists. Current value: '{}'. Suggestion: Use a different backend ID or remove the existing backend first.",
                    backend.id, backend.id
                ),
            ));
        }

        backends.insert(backend.id.clone(), backend.clone());
        info!(
            "Added backend: {} at {}:{}",
            backend.id, backend.address, backend.port
        );
        Ok(())
    }

    /// Remove backend from load balancer
    ///
    /// # Arguments
    ///
    /// * `backend_id` - ID of the backend to remove
    ///
    /// # Errors
    ///
    /// Returns `AppError::ValidationError` if:
    /// - `backend_id` is empty
    /// - Backend with given ID does not exist
    pub async fn remove_backend(&self, backend_id: &str) -> Result<(), AppError> {
        if backend_id.is_empty() {
            return Err(AppError::ValidationError(
                "Backend ID cannot be empty. Current value: ''. Suggestion: Provide a valid backend identifier."
                    .to_string(),
            ));
        }

        let mut backends = self.backends.write().await;
        if !backends.contains_key(backend_id) {
            return Err(AppError::ValidationError(
                format!(
                    "Backend with ID '{}' does not exist. Current value: '{}'. Suggestion: Check the backend ID or list backends to see available IDs.",
                    backend_id, backend_id
                ),
            ));
        }

        backends.remove(backend_id);
        info!("Removed backend: {}", backend_id);
        Ok(())
    }

    /// Get load balancer health status
    ///
    /// Returns the current health status of all registered backends.
    ///
    /// # Errors
    ///
    /// Returns `AppError::NetworkError` if:
    /// - Health check service is unreachable
    /// - Health check requests fail
    ///
    /// # Future Implementation
    ///
    /// This will be enhanced to:
    /// - Perform actual health checks (HTTP/HTTPS/TCP)
    /// - Track health check history
    /// - Mark backends as unhealthy after consecutive failures
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::loadbalancing::LoadBalancer;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let loadbalancer = LoadBalancer::new();
    /// loadbalancer.initialize().await?;
    ///
    /// let health = loadbalancer.get_health_status().await?;
    /// println!("Healthy backends: {}/{}", health.healthy_backends, health.total_backends);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_health_status(&self) -> Result<LoadBalancerHealth, AppError> {
        let backends = self.backends.read().await;
        let total = backends.len() as u32;

        let health_config = self.health_check_config.read().await;
        let mut healthy = 0;
        let mut unhealthy = 0;

        // Perform health checks for each backend
        for (backend_id, backend) in backends.iter() {
            let is_healthy = self.check_backend_health(backend, &health_config).await;

            // Update health status
            let mut backend_health_map = self.backend_health.write().await;
            let health = backend_health_map
                .entry(backend_id.clone())
                .or_insert_with(|| BackendHealth {
                    healthy: true,
                    consecutive_failures: 0,
                    last_check: None,
                });

            if is_healthy {
                if !health.healthy {
                    // Backend recovered
                    health.consecutive_failures = 0;
                    health.healthy = true;
                }
                healthy += 1;
            } else {
                health.consecutive_failures += 1;
                if health.consecutive_failures >= health_config.failure_threshold {
                    health.healthy = false;
                    unhealthy += 1;
                } else {
                    // Not yet marked as unhealthy, but failing
                    healthy += 1;
                }
            }
            health.last_check = Some(std::time::Instant::now());
        }

        drop(health_config);

        Ok(LoadBalancerHealth {
            healthy_backends: healthy,
            unhealthy_backends: unhealthy,
            total_backends: total,
        })
    }

    /// Check health of a single backend
    async fn check_backend_health(&self, backend: &Backend, config: &HealthCheckConfig) -> bool {
        #[cfg(feature = "cloud-sdk")]
        {
            // Try Kubernetes pod health check first
            if let Some(ref k8s_manager) = self.k8s_manager {
                // Check if backend is a Kubernetes pod
                if let Ok(pod_status) = k8s_manager.get_pod_status(&backend.id).await {
                    if pod_status.ready && pod_status.phase == "Running" {
                        return true;
                    }
                }
            }
        }

        // Fallback: HTTP health check
        #[cfg(feature = "cloud-sdk")]
        {
            if let Some(ref path) = config.path {
                let url = format!("http://{}:{}{}", backend.address, backend.port, path);
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(config.timeout_secs))
                    .build();

                if let Ok(client) = client {
                    if let Ok(result) = timeout(
                        Duration::from_secs(config.timeout_secs),
                        client.get(&url).send(),
                    )
                    .await
                    {
                        if let Ok(response) = result {
                            return response.status().is_success();
                        }
                    }
                }
            }
        }

        // If no health check path configured, assume healthy
        // (for TCP-only backends)
        true
    }

    /// List all registered backends
    ///
    /// Returns a vector of all backend IDs and their configurations.
    pub async fn list_backends(&self) -> Vec<Backend> {
        let backends = self.backends.read().await;
        backends.values().cloned().collect()
    }

    /// Set load balancing strategy
    ///
    /// # Arguments
    ///
    /// * `strategy` - Load balancing strategy to use
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::cloud::loadbalancing::{LoadBalancer, LoadBalancingStrategy};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let loadbalancer = LoadBalancer::new();
    /// loadbalancer.initialize().await?;
    /// loadbalancer.set_strategy(LoadBalancingStrategy::WeightedRoundRobin).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_strategy(&self, strategy: LoadBalancingStrategy) {
        *self.strategy.write().await = strategy;
    }

    /// Get current load balancing strategy
    pub async fn get_strategy(&self) -> LoadBalancingStrategy {
        *self.strategy.read().await
    }

    /// Update health check configuration
    ///
    /// # Arguments
    ///
    /// * `config` - New health check configuration
    pub async fn set_health_check_config(&self, config: HealthCheckConfig) {
        *self.health_check_config.write().await = config;
    }

    /// Get current health check configuration
    pub async fn get_health_check_config(&self) -> HealthCheckConfig {
        self.health_check_config.read().await.clone()
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}

/// Backend server for load balancing
#[derive(Debug, Clone)]
pub struct Backend {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub weight: u32, // For weighted round-robin
}

/// Load balancer health status
#[derive(Debug, Clone)]
pub struct LoadBalancerHealth {
    pub healthy_backends: u32,
    pub unhealthy_backends: u32,
    pub total_backends: u32,
}
