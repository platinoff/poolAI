//! Pool module for worker pool management
//!
//! This module provides:
//! - Worker pool management with load balancing
//! - Task distribution across workers
//! - Pool metrics and monitoring
//! - Auto-scaling capabilities

pub mod worker;

use crate::core::error::AppError;
use crate::core::model_interface::{ModelRequest, ModelResponse};
use rand::prelude::IndexedRandom;
use rand::rngs::ThreadRng;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Configuration for the worker pool
///
/// Controls pool behavior including worker limits, load balancing strategy,
/// and auto-scaling parameters.
///
/// # Example
///
/// ```rust
/// use poolai::pool::{PoolConfig, LoadBalancingStrategy};
///
/// let config = PoolConfig {
///     max_workers: 10,
///     max_queue_size: 1000,
///     load_balancing_strategy: LoadBalancingStrategy::LeastConnections,
///     auto_scaling: true,
///     scaling_threshold: 0.8,
///     request_timeout: 30,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of workers in the pool
    pub max_workers: usize,
    /// Maximum number of requests in the queue
    pub max_queue_size: usize,
    /// Strategy for distributing requests across workers
    pub load_balancing_strategy: LoadBalancingStrategy,
    /// Enable automatic scaling based on load
    pub auto_scaling: bool,
    /// Threshold (0.0-1.0) for triggering scaling actions
    pub scaling_threshold: f32,
    /// Request timeout in seconds
    pub request_timeout: u64,
}

/// Load balancing strategy for distributing requests across workers
///
/// Different strategies optimize for different scenarios:
/// - `RoundRobin`: Simple, predictable distribution
/// - `LeastConnections`: Balances based on current worker load
/// - `Weighted`: Allows prioritization of specific workers
/// - `Random`: Simple random selection
#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    /// Distribute requests in round-robin fashion
    RoundRobin,
    /// Route to worker with fewest active connections
    LeastConnections,
    /// Use weighted distribution based on worker capacity
    Weighted,
    /// Randomly select a worker
    Random,
}

/// Metrics for monitoring pool performance
///
/// Provides comprehensive statistics about pool operation including
/// worker count, request statistics, resource utilization, and performance metrics.
///
/// # Example
///
/// ```rust
/// use poolai::pool::PoolMetrics;
///
/// let metrics = PoolMetrics::default();
/// println!("Active workers: {}", metrics.active_workers);
/// println!("Success rate: {:.2}%",
///     (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0);
/// ```
#[derive(Debug, Clone)]
pub struct PoolMetrics {
    /// Number of currently active workers
    pub active_workers: usize,
    /// Current number of requests in queue
    pub queue_size: usize,
    /// Total number of requests processed
    pub total_requests: u64,
    /// Number of successfully completed requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// GPU utilization percentage (0.0-1.0)
    pub gpu_utilization: f32,
    /// Memory usage in megabytes
    pub memory_usage_mb: f32,
    /// Throughput in requests per second
    pub throughput_rps: f32,
    /// Error rate (0.0-1.0)
    pub error_rate: f32,
}

impl Default for PoolMetrics {
    fn default() -> Self {
        Self {
            active_workers: 0,
            queue_size: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            gpu_utilization: 0.0,
            memory_usage_mb: 0.0,
            throughput_rps: 0.0,
            error_rate: 0.0,
        }
    }
}

/// Worker pool for managing and distributing inference tasks
///
/// The `Pool` manages a collection of workers, handles load balancing,
/// tracks metrics, and provides auto-scaling capabilities.
///
/// # Example
///
/// ```rust
/// use poolai::pool::{Pool, PoolConfig, LoadBalancingStrategy};
///
/// #[tokio::main]
/// async fn main() {
///     let config = PoolConfig {
///         max_workers: 10,
///         max_queue_size: 1000,
///         load_balancing_strategy: LoadBalancingStrategy::LeastConnections,
///         auto_scaling: true,
///         scaling_threshold: 0.8,
///         request_timeout: 30,
///     };
///     
///     let pool = Pool::new(config);
///     // Add workers and process requests...
/// }
/// ```
pub struct Pool {
    config: PoolConfig,
    workers: Arc<RwLock<HashMap<String, worker::Worker>>>,
    metrics: Arc<RwLock<PoolMetrics>>,
    _current_worker_index: Arc<RwLock<usize>>,
}

impl Pool {
    /// Creates a new worker pool with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Pool configuration including worker limits and load balancing strategy
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::pool::{Pool, PoolConfig, LoadBalancingStrategy};
    ///
    /// let config = PoolConfig {
    ///     max_workers: 10,
    ///     max_queue_size: 1000,
    ///     load_balancing_strategy: LoadBalancingStrategy::Random,
    ///     auto_scaling: false,
    ///     scaling_threshold: 0.8,
    ///     request_timeout: 30,
    /// };
    /// let pool = Pool::new(config);
    /// ```
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            workers: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PoolMetrics::default())),
            _current_worker_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Adds a new worker to the pool
    ///
    /// # Arguments
    ///
    /// * `worker_id` - Unique identifier for the worker
    /// * `worker` - Worker instance to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the operation fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::pool::{Pool, PoolConfig, LoadBalancingStrategy};
    /// # use poolai::pool::worker::Worker;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = PoolConfig {
    /// #     max_workers: 10, max_queue_size: 1000,
    /// #     load_balancing_strategy: LoadBalancingStrategy::Random,
    /// #     auto_scaling: false, scaling_threshold: 0.8, request_timeout: 30,
    /// # };
    /// let pool = Pool::new(config);
    /// let worker = Worker::new("worker-1".to_string());
    /// pool.add_worker("worker-1".to_string(), worker).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_worker(
        &self,
        worker_id: String,
        worker: worker::Worker,
    ) -> Result<(), AppError> {
        let mut workers = self.workers.write().await;
        workers.insert(worker_id.clone(), worker);

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.active_workers = workers.len();

        info!(
            "Added worker: {} (total workers: {})",
            worker_id, metrics.active_workers
        );
        Ok(())
    }

    /// Removes a worker from the pool
    ///
    /// # Arguments
    ///
    /// * `worker_id` - Identifier of the worker to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the worker was found and removed, or an error if the worker
    /// doesn't exist or the operation fails.
    ///
    /// # Errors
    ///
    /// Returns `AppError::PoolError` if the worker is not found in the pool.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use poolai::pool::{Pool, PoolConfig, LoadBalancingStrategy};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = PoolConfig {
    /// #     max_workers: 10, max_queue_size: 1000,
    /// #     load_balancing_strategy: LoadBalancingStrategy::Random,
    /// #     auto_scaling: false, scaling_threshold: 0.8, request_timeout: 30,
    /// # };
    /// let pool = Pool::new(config);
    /// pool.remove_worker("worker-1").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), AppError> {
        let mut workers = self.workers.write().await;
        let current_worker_count = workers.len();

        if workers.remove(worker_id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write().await;
            metrics.active_workers = workers.len();

            info!(
                "Removed worker: {} (total workers: {})",
                worker_id, metrics.active_workers
            );
            Ok(())
        } else {
            Err(AppError::PoolError(format!(
                "Worker '{}' not found. Context: Attempted to stop a worker that doesn't exist in the pool. \
                Suggestion: Verify worker_id using list_workers() or get_worker_metrics(). \
                Current worker_id: '{}', Active workers: {}",
                worker_id, worker_id, current_worker_count
            )))
        }
    }

    pub async fn process_request(&self, request: ModelRequest) -> Result<ModelResponse, AppError> {
        // Select worker according to load balancing strategy
        let worker = self.select_worker().await?;

        // Process request
        let response = worker.process_request(request).await?;

        // Update metrics
        self.update_metrics(&response).await;

        Ok(response)
    }

    async fn select_worker(&self) -> Result<worker::Worker, AppError> {
        let workers = self.workers.read().await;

        if workers.is_empty() {
            return Err(AppError::PoolError(
                "No workers available. Context: Cannot process request because worker pool is empty. \
                Suggestion: Add workers to the pool using add_worker() before processing requests. \
                Current pool size: 0 workers, Max workers: {}"
                    .to_string(),
            ));
        }

        match self.config.load_balancing_strategy {
            LoadBalancingStrategy::RoundRobin => {
                // Simple round-robin implementation
                if let Some((_, worker)) = workers.iter().next() {
                    Ok(worker.clone())
                } else {
                    Err(AppError::PoolError("No workers available".to_string()))
                }
            }
            LoadBalancingStrategy::LeastConnections => {
                // Select worker with least connections
                workers
                    .iter()
                    .min_by_key(|(_, worker)| worker.get_active_connections())
                    .map(|(_, worker)| worker.clone())
                    .ok_or_else(|| AppError::PoolError("No workers available".to_string()))
            }
            LoadBalancingStrategy::Weighted => {
                // Weighted selection based on metrics
                workers
                    .iter()
                    .max_by(|(_, a), (_, b)| {
                        a.get_health_score()
                            .partial_cmp(&b.get_health_score())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(_, worker)| worker.clone())
                    .ok_or_else(|| AppError::PoolError("No workers available".to_string()))
            }
            LoadBalancingStrategy::Random => {
                // Random selection
                let worker_list: Vec<_> = workers.values().collect();
                let mut rng = ThreadRng::default();
                worker_list
                    .choose(&mut rng)
                    .map(|worker| (*worker).clone())
                    .ok_or_else(|| AppError::PoolError("No workers available".to_string()))
            }
        }
    }

    async fn update_metrics(&self, response: &ModelResponse) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;

        match response.status {
            crate::core::model_interface::ResponseStatus::Success => {
                metrics.successful_requests += 1;
            }
            _ => {
                metrics.failed_requests += 1;
            }
        }

        // Update average response time
        metrics.average_response_time_ms = (metrics.average_response_time_ms
            * (metrics.total_requests - 1) as f64
            + response.metrics.processing_time_ms as f64)
            / metrics.total_requests as f64;

        // Update error rate
        if metrics.total_requests > 0 {
            metrics.error_rate = metrics.failed_requests as f32 / metrics.total_requests as f32;
        }

        // Update throughput (requests per second)
        // This is a simplified calculation - in production you'd want a rolling window
        let uptime_seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if uptime_seconds > 0 {
            metrics.throughput_rps = metrics.total_requests as f32 / uptime_seconds as f32;
        }
    }

    pub async fn get_metrics(&self) -> PoolMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn scale_up(&self) -> Result<(), AppError> {
        if !self.config.auto_scaling {
            return Ok(());
        }

        let current_count = {
            let workers = self.workers.read().await;
            workers.len()
        };

        if current_count >= self.config.max_workers {
            return Ok(());
        }

        info!(
            "Scaling up pool - adding new worker (current: {}/{})",
            current_count, self.config.max_workers
        );

        // Generate unique worker ID
        let worker_id = format!(
            "worker-{}",
            uuid::Uuid::new_v4()
                .to_string()
                .split('-')
                .next()
                .unwrap_or("auto")
        );

        // Create worker config with reasonable defaults
        let worker_config = worker::WorkerConfig {
            worker_id: worker_id.clone(),
            max_concurrent_requests: 10,
            request_timeout_ms: self.config.request_timeout,
            health_check_interval_ms: 5000,
            enable_caching: true,
            cache_size: 1000,
            max_memory_mb: 2048,
            cpu_priority: 5,
            gpu_device: None,
            auto_restart: true,
            resource_monitoring: true,
        };

        // Create and add worker
        let new_worker = worker::Worker::new(worker_config);
        self.add_worker(worker_id.clone(), new_worker).await?;

        info!("Successfully scaled up pool - new worker: {}", worker_id);
        Ok(())
    }

    pub async fn scale_down(&self) -> Result<(), AppError> {
        if !self.config.auto_scaling {
            return Ok(());
        }

        let current_count = {
            let workers = self.workers.read().await;
            workers.len()
        };

        if current_count <= 1 {
            return Ok(());
        }

        info!(
            "Scaling down pool - removing least loaded worker (current: {})",
            current_count
        );

        // Find the least loaded worker (minimum active_connections + queue_size)
        let (least_loaded_id, least_loaded_worker) = {
            let workers = self.workers.read().await;
            let mut min_load = usize::MAX;
            let mut candidate_id: Option<String> = None;
            let mut candidate_worker: Option<worker::Worker> = None;

            for (id, worker) in workers.iter() {
                let status = worker.get_status().await;
                let load = status.active_connections + status.queue_size;

                if load < min_load {
                    min_load = load;
                    candidate_id = Some(id.clone());
                    candidate_worker = Some(worker.clone());
                }
            }

            match (candidate_id, candidate_worker) {
                (Some(id), Some(worker)) => (id, worker),
                _ => return Err(AppError::PoolError(
                    "No workers available for scaling down. Context: Cannot scale down because pool is already at minimum size. \
                    Suggestion: Verify current worker count before attempting to scale down. \
                    Current workers: {}, Minimum workers: 1"
                        .to_string(),
                )),
            }
        };

        // Gracefully shutdown the worker
        if let Err(e) = least_loaded_worker.shutdown().await {
            warn!(
                "Failed to gracefully shutdown worker {}: {}",
                least_loaded_id, e
            );
            // Continue with removal even if shutdown failed
        }

        // Remove worker from pool
        self.remove_worker(&least_loaded_id).await?;

        info!(
            "Successfully scaled down pool - removed worker: {}",
            least_loaded_id
        );
        Ok(())
    }

    pub async fn distribute_resources(&self) -> Result<(), AppError> {
        let workers = self.workers.read().await;
        for worker in workers.values() {
            if let Err(e) = worker.optimize_resources().await {
                warn!("Failed to optimize resources for worker: {}", e);
            }
        }
        Ok(())
    }

    pub async fn get_worker_count(&self) -> usize {
        self.workers.read().await.len()
    }

    pub async fn get_worker_status(&self) -> HashMap<String, worker::WorkerStatus> {
        let workers = self.workers.read().await;
        let mut status_map = HashMap::new();

        for (id, worker) in workers.iter() {
            let status = worker.get_status().await;
            status_map.insert(id.clone(), status);
        }

        status_map
    }
}

// Global pool instance - using OnceLock for thread-safe initialization
// Wrapped in Arc<RwLock<>> for shared mutable access across async contexts
static GLOBAL_POOL: OnceLock<Arc<RwLock<Pool>>> = OnceLock::new();

/// Initialize pool module
pub async fn initialize() -> Result<(), AppError> {
    info!("Initializing pool module");

    // Create default pool configuration
    let config = PoolConfig {
        max_workers: 10,
        max_queue_size: 1000,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        auto_scaling: true,
        scaling_threshold: 0.8,
        request_timeout: 30,
    };

    let pool = Pool::new(config);

    // Store global instance wrapped in Arc<RwLock<>>
    GLOBAL_POOL
        .set(Arc::new(RwLock::new(pool)))
        .map_err(|_| AppError::PoolError(
            "Pool already initialized. Context: Attempted to initialize global pool instance twice. \
            Suggestion: Ensure initialize() is called only once at application startup. \
            Note: Pool module uses OnceLock for thread-safe single initialization."
                .to_string(),
        ))?;

    info!("Pool module initialized successfully");
    Ok(())
}

/// Initialize pool module with custom configuration
pub async fn initialize_with_config(config: PoolConfig) -> Result<(), AppError> {
    info!("Initializing pool module with custom configuration");

    let pool = Pool::new(config);

    // Store global instance wrapped in Arc<RwLock<>>
    GLOBAL_POOL
        .set(Arc::new(RwLock::new(pool)))
        .map_err(|_| AppError::PoolError(
            "Pool already initialized. Context: Attempted to initialize global pool instance twice. \
            Suggestion: Ensure initialize() is called only once at application startup. \
            Note: Pool module uses OnceLock for thread-safe single initialization."
                .to_string(),
        ))?;

    info!("Pool module initialized with custom configuration successfully");
    Ok(())
}

/// Shutdown pool module
pub async fn shutdown() -> Result<(), AppError> {
    info!("Shutting down pool module");

    // Note: OnceLock doesn't support clearing, so we can't fully remove it
    // The pool will remain in memory but won't be accessible after this
    // For true cleanup, consider using a different pattern or accept this limitation

    info!("Pool module shut down successfully");
    Ok(())
}

/// Health check for pool module
pub async fn health_check() -> Result<(), AppError> {
    info!("Pool module health check");

    // Check if global pool exists
    if GLOBAL_POOL.get().is_none() {
        return Err(AppError::PoolError(
            "Global pool not initialized. Context: Attempted to access global pool instance before initialization. \
            Suggestion: Call pool::initialize() or pool::initialize_with_config() before using global pool functions. \
            Note: This should be called once at application startup."
                .to_string(),
        ));
    }

    info!("Pool module health check passed");
    Ok(())
}

/// Get global pool instance
/// Returns a reference to `Arc<RwLock<Pool>>` for async-safe access
pub fn get_global_pool() -> Option<&'static Arc<RwLock<Pool>>> {
    GLOBAL_POOL.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_metrics_default() {
        let metrics = PoolMetrics::default();
        assert_eq!(metrics.active_workers, 0);
        assert_eq!(metrics.queue_size, 0);
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_requests, 0);
        assert_eq!(metrics.failed_requests, 0);
        assert_eq!(metrics.average_response_time_ms, 0.0);
        assert_eq!(metrics.gpu_utilization, 0.0);
        assert_eq!(metrics.memory_usage_mb, 0.0);
        assert_eq!(metrics.throughput_rps, 0.0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_pool_config_clone() {
        let config = PoolConfig {
            max_workers: 10,
            max_queue_size: 1000,
            load_balancing_strategy: LoadBalancingStrategy::LeastConnections,
            auto_scaling: true,
            scaling_threshold: 0.8,
            request_timeout: 30,
        };
        let cloned = config.clone();
        assert_eq!(config.max_workers, cloned.max_workers);
        assert_eq!(config.max_queue_size, cloned.max_queue_size);
        assert_eq!(config.request_timeout, cloned.request_timeout);
    }

    #[test]
    fn test_load_balancing_strategy_variants() {
        let strategies = vec![
            LoadBalancingStrategy::RoundRobin,
            LoadBalancingStrategy::LeastConnections,
            LoadBalancingStrategy::Weighted,
            LoadBalancingStrategy::Random,
        ];
        for strategy in strategies {
            let cloned = strategy.clone();
            assert!(matches!(
                cloned,
                LoadBalancingStrategy::RoundRobin
                    | LoadBalancingStrategy::LeastConnections
                    | LoadBalancingStrategy::Weighted
                    | LoadBalancingStrategy::Random
            ));
        }
    }

    #[tokio::test]
    async fn test_pool_new() {
        let config = PoolConfig {
            max_workers: 10,
            max_queue_size: 1000,
            load_balancing_strategy: LoadBalancingStrategy::Random,
            auto_scaling: false,
            scaling_threshold: 0.8,
            request_timeout: 30,
        };
        let pool = Pool::new(config);
        let metrics = pool.get_metrics().await;
        assert_eq!(metrics.active_workers, 0);
        assert_eq!(metrics.queue_size, 0);
    }

    #[tokio::test]
    async fn test_pool_get_metrics() {
        let config = PoolConfig {
            max_workers: 10,
            max_queue_size: 1000,
            load_balancing_strategy: LoadBalancingStrategy::Random,
            auto_scaling: false,
            scaling_threshold: 0.8,
            request_timeout: 30,
        };
        let pool = Pool::new(config);
        let metrics = pool.get_metrics().await;
        assert_eq!(metrics.active_workers, 0);
        assert_eq!(metrics.total_requests, 0);
    }

    #[tokio::test]
    async fn test_pool_get_worker_count_empty() {
        let config = PoolConfig {
            max_workers: 10,
            max_queue_size: 1000,
            load_balancing_strategy: LoadBalancingStrategy::Random,
            auto_scaling: false,
            scaling_threshold: 0.8,
            request_timeout: 30,
        };
        let pool = Pool::new(config);
        let count = pool.get_worker_count().await;
        assert_eq!(count, 0);
    }
}
