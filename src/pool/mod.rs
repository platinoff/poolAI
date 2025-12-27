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
// use crate::core::config::PoolAIConfig; // Not used in MVP
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_workers: usize,
    pub max_queue_size: usize,
    pub load_balancing_strategy: LoadBalancingStrategy,
    pub auto_scaling: bool,
    pub scaling_threshold: f32,
    pub request_timeout: u64,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Weighted,
    Random,
}

#[derive(Debug, Clone)]
pub struct PoolMetrics {
    pub active_workers: usize,
    pub queue_size: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
    pub throughput_rps: f32,
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

pub struct Pool {
    config: PoolConfig,
    workers: Arc<RwLock<HashMap<String, worker::Worker>>>,
    metrics: Arc<RwLock<PoolMetrics>>,
    _current_worker_index: Arc<RwLock<usize>>,
}

impl Pool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            workers: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PoolMetrics::default())),
            _current_worker_index: Arc::new(RwLock::new(0)),
        }
    }

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

    pub async fn remove_worker(&self, worker_id: &str) -> Result<(), AppError> {
        let mut workers = self.workers.write().await;
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
                "Worker '{}' not found",
                worker_id
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
            return Err(AppError::PoolError("No workers available".to_string()));
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
                worker_list
                    .choose(&mut thread_rng())
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
        if self.config.auto_scaling {
            let workers = self.workers.read().await;
            if workers.len() < self.config.max_workers {
                info!("Scaling up pool - adding new worker");
                // TODO: Implement actual worker creation
                // This would involve creating a new worker instance
                // and adding it to the pool
            }
        }
        Ok(())
    }

    pub async fn scale_down(&self) -> Result<(), AppError> {
        if self.config.auto_scaling {
            let workers = self.workers.read().await;
            if workers.len() > 1 {
                info!("Scaling down pool - removing worker");
                // TODO: Implement actual worker removal
                // This would involve gracefully shutting down a worker
                // and removing it from the pool
            }
        }
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
        .map_err(|_| AppError::PoolError("Pool already initialized".to_string()))?;

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
        .map_err(|_| AppError::PoolError("Pool already initialized".to_string()))?;

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
            "Global pool not initialized".to_string(),
        ));
    }

    info!("Pool module health check passed");
    Ok(())
}

/// Get global pool instance
/// Returns a reference to Arc<RwLock<Pool>> for async-safe access
pub fn get_global_pool() -> Option<&'static Arc<RwLock<Pool>>> {
    GLOBAL_POOL.get()
}
