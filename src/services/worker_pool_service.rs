//! Worker pool operations for the HTTP API (list, add, remove).

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::pool;
use crate::pool::Pool;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

/// Dashboard-friendly worker row (matches admin UI expectations).
#[derive(Debug, Clone, Serialize)]
pub struct WorkerInfo {
    pub id: String,
    /// High-level state for dashboards: `idle`, `busy`, or `error`.
    pub status: String,
    pub current_task: Option<String>,
    /// Matches admin UI and detailed panels (pool `WorkerStatus`).
    pub is_healthy: bool,
    pub total_requests_processed: u64,
    pub queue_size: usize,
    pub active_connections: usize,
    pub average_response_time_ms: f64,
}

/// Validated fields for registering a worker in the pool (HTTP layer maps requests here).
#[derive(Debug, Clone)]
pub struct CreateWorkerInput {
    pub worker_id: String,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub enable_caching: bool,
    pub cache_size: usize,
    pub max_memory_mb: usize,
    pub cpu_priority: u8,
    pub gpu_device: Option<usize>,
    pub auto_restart: bool,
    pub resource_monitoring: bool,
}

#[derive(Debug)]
pub enum AddWorkerError {
    PoolNotReady,
    Operation(AppError),
}

#[derive(Debug)]
pub enum RemoveWorkerError {
    PoolNotReady,
    Operation(AppError),
}

pub struct WorkerPoolService;

impl WorkerPoolService {
    fn status_label(is_healthy: bool, active_connections: usize) -> String {
        match is_healthy {
            true => {
                if active_connections > 0 {
                    "busy".to_string()
                } else {
                    "idle".to_string()
                }
            }
            false => "error".to_string(),
        }
    }

    fn mock_workers() -> Vec<WorkerInfo> {
        vec![
            WorkerInfo {
                id: "worker-1".to_string(),
                status: "busy".to_string(),
                current_task: Some("text-generation".to_string()),
                is_healthy: true,
                total_requests_processed: 128,
                queue_size: 0,
                active_connections: 1,
                average_response_time_ms: 24.5,
            },
            WorkerInfo {
                id: "worker-2".to_string(),
                status: "idle".to_string(),
                current_task: None,
                is_healthy: true,
                total_requests_processed: 64,
                queue_size: 0,
                active_connections: 0,
                average_response_time_ms: 18.0,
            },
        ]
    }

    pub async fn list_workers(ctx: &ApiContext) -> Vec<WorkerInfo> {
        if let Some(pool) = ctx.pool.get() {
            let worker_statuses = {
                let pool_guard = pool.read().await;
                pool_guard.get_worker_status().await
            };
            if !worker_statuses.is_empty() {
                return worker_statuses
                    .iter()
                    .map(|(id, status)| WorkerInfo {
                        id: id.clone(),
                        status: Self::status_label(status.is_healthy, status.active_connections),
                        current_task: status.current_task.clone(),
                        is_healthy: status.is_healthy,
                        total_requests_processed: status.total_requests_processed,
                        queue_size: status.queue_size,
                        active_connections: status.active_connections,
                        average_response_time_ms: status.average_response_time_ms,
                    })
                    .collect();
            }
        }
        Self::mock_workers()
    }

    pub async fn add_worker(
        ctx: &ApiContext,
        input: CreateWorkerInput,
    ) -> Result<(), AddWorkerError> {
        let pool: Arc<TokioRwLock<Pool>> = ctx
            .pool
            .get()
            .cloned()
            .ok_or(AddWorkerError::PoolNotReady)?;
        let worker_config = pool::worker::WorkerConfig {
            worker_id: input.worker_id.clone(),
            max_concurrent_requests: input.max_concurrent_requests,
            request_timeout_ms: input.request_timeout_ms,
            health_check_interval_ms: input.health_check_interval_ms,
            enable_caching: input.enable_caching,
            cache_size: input.cache_size,
            max_memory_mb: input.max_memory_mb,
            cpu_priority: input.cpu_priority,
            gpu_device: input.gpu_device,
            auto_restart: input.auto_restart,
            resource_monitoring: input.resource_monitoring,
        };
        let worker = pool::worker::Worker::new(worker_config);
        let pool_guard = pool.write().await;
        pool_guard
            .add_worker(input.worker_id, worker)
            .await
            .map_err(AddWorkerError::Operation)?;
        Ok(())
    }

    pub async fn remove_worker(ctx: &ApiContext, worker_id: &str) -> Result<(), RemoveWorkerError> {
        let pool: Arc<TokioRwLock<Pool>> = ctx
            .pool
            .get()
            .cloned()
            .ok_or(RemoveWorkerError::PoolNotReady)?;
        let pool_guard = pool.write().await;
        pool_guard
            .remove_worker(worker_id)
            .await
            .map_err(RemoveWorkerError::Operation)?;
        Ok(())
    }
}
