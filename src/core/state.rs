//! Application state management module
//!
//! Provides centralized state management for PoolAI, including worker tracking,
//! system state, model states, and configuration management.
//!
//! # Features
//!
//! - **Worker Management**: Add, remove, update workers and their metrics
//! - **System State**: Track system status, metrics, and health
//! - **Model State**: Manage model states and lifecycle
//! - **Thread Safety**: All operations are thread-safe using `Arc<RwLock<>>`
//!
//! # Example
//!
//! ```no_run
//! use poolai::core::state::AppState;
//! use poolai::core::state::{Worker, WorkerStatus, WorkerMetrics};
//! use chrono::Utc;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let state = AppState::new();
//! state.initialize().await?;
//!
//! // Add a worker
//! let worker = Worker {
//!     id: "worker-1".to_string(),
//!     address: "127.0.0.1:8080".to_string(),
//!     mining_power: 100.0,
//!     status: WorkerStatus::Active,
//!     last_seen: Utc::now(),
//!     metrics: WorkerMetrics::default(),
//!     active_models: vec![],
//! };
//! state.add_worker(worker)?;
//!
//! // Get worker
//! if let Some(worker) = state.get_worker("worker-1") {
//!     println!("Worker status: {:?}", worker.status);
//! }
//!
//! // Get system state
//! let system_state = state.get_system_state();
//! println!("Active workers: {}", system_state.active_workers);
//!
//! state.cleanup().await?;
//! # Ok(())
//! # }
//! ```

use crate::core::config::PoolAIConfig;
use crate::core::error::AppError;
use crate::core::model_interface::{ModelState, ModelStatus};
use chrono::{DateTime, Utc};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Shared application context for API and service layers.
///
/// This is the primary handle that HTTP handlers and services should depend on,
/// rather than constructing their own state. It is a thin alias around
/// `Arc<AppState>` to make intent explicit at call sites.
pub type ApiContext = Arc<AppState>;

/// Worker state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    /// Unique worker ID
    pub id: String,
    /// Worker address
    pub address: String,
    /// Computational power
    pub mining_power: f64,
    /// Worker status
    pub status: WorkerStatus,
    /// Last activity time
    pub last_seen: DateTime<Utc>,
    /// Performance metrics
    pub metrics: WorkerMetrics,
    /// Active models being processed
    pub active_models: Vec<String>,
}

/// Worker metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    /// CPU utilization (%)
    pub cpu_utilization: f32,
    /// Memory usage (MB)
    pub memory_usage_mb: f32,
    /// GPU utilization (%)
    pub gpu_utilization: f32,
    /// GPU temperature (°C)
    pub gpu_temperature: f32,
    /// Number of processed requests
    pub requests_processed: u64,
    /// Average processing time (ms)
    pub avg_processing_time_ms: f32,
    /// Error count
    pub error_count: u64,
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_usage_mb: 0.0,
            gpu_utilization: 0.0,
            gpu_temperature: 0.0,
            requests_processed: 0,
            avg_processing_time_ms: 0.0,
            error_count: 0,
        }
    }
}

/// Worker status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Active,
    Inactive,
    Error,
    Maintenance,
    Shutdown,
}

/// Node status for distributed systems
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Degraded,
    Failed,
    Maintenance,
}

/// System state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// System status
    pub status: SystemStatus,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Number of active workers
    pub active_workers: usize,
    /// Total number of workers
    pub total_workers: usize,
    /// Number of active models
    pub active_models: usize,
    /// System metrics
    pub system_metrics: SystemMetrics,
}

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Initializing,
    Running,
    Degraded,
    Error,
    Shutdown,
    Maintenance,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total CPU utilization (%)
    pub total_cpu_utilization: f32,
    /// Total memory usage (MB)
    pub total_memory_usage_mb: f32,
    /// Total GPU utilization (%)
    pub total_gpu_utilization: f32,
    /// Total number of requests
    pub total_requests: u64,
    /// Average latency (ms)
    pub avg_latency_ms: f32,
    /// Throughput (requests/sec)
    pub throughput_rps: f32,
    /// Error count
    pub error_count: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            total_cpu_utilization: 0.0,
            total_memory_usage_mb: 0.0,
            total_gpu_utilization: 0.0,
            total_requests: 0,
            avg_latency_ms: 0.0,
            throughput_rps: 0.0,
            error_count: 0,
        }
    }
}

/// Main application state
///
/// Centralized state management for PoolAI application. Provides thread-safe
/// access to workers, system state, model states, and configuration.
///
/// # Thread Safety
///
/// All operations are thread-safe using `Arc<RwLock<>>` for shared state
/// and `Arc<Mutex<>>` for synchronization.
///
/// # Example
///
/// ```no_run
/// use poolai::core::state::AppState;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let state = AppState::new();
/// state.initialize().await?;
///
/// // Use state for worker and model management
/// // ...
///
/// state.cleanup().await?;
/// # Ok(())
/// # }
/// ```
pub struct AppState {
    /// Workers
    pub workers: Arc<RwLock<HashMap<String, Worker>>>,
    /// Configuration
    pub config: Arc<RwLock<PoolAIConfig>>,
    /// System state
    pub system_state: Arc<RwLock<SystemState>>,
    /// Model states
    pub model_states: Arc<RwLock<HashMap<String, ModelState>>>,
    /// Initialization flag
    pub is_initialized: Arc<RwLock<bool>>,
    /// Mutex for synchronization
    pub state_mutex: Arc<Mutex<()>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create new application state
    ///
    /// Initializes a new `AppState` with empty collections and default values.
    /// Remember to call `initialize()` before using the state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::core::state::AppState;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let state = AppState::new();
    /// state.initialize().await?;
    /// // Now state is ready to use
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(PoolAIConfig::default())),
            system_state: Arc::new(RwLock::new(SystemState {
                status: SystemStatus::Initializing,
                start_time: Utc::now(),
                last_activity: Utc::now(),
                active_workers: 0,
                total_workers: 0,
                active_models: 0,
                system_metrics: SystemMetrics::default(),
            })),
            model_states: Arc::new(RwLock::new(HashMap::new())),
            is_initialized: Arc::new(RwLock::new(false)),
            state_mutex: Arc::new(Mutex::new(())),
        }
    }

    /// Initialize state
    pub async fn initialize(&self) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut initialized = self.is_initialized.write();

        if *initialized {
            return Ok(());
        }

        info!("Initializing application state...");

        // Load configuration
        let config = PoolAIConfig::default();
        *self.config.write() = config;

        // Update system state
        let mut system_state = self.system_state.write();
        system_state.status = SystemStatus::Running;
        system_state.last_activity = Utc::now();

        *initialized = true;
        info!("Application state initialized successfully");
        Ok(())
    }

    /// Cleanup state
    pub async fn cleanup(&self) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        info!("Cleaning up application state...");

        // Clear workers
        self.workers.write().clear();

        // Clear model states
        self.model_states.write().clear();

        // Update system state
        let mut system_state = self.system_state.write();
        system_state.status = SystemStatus::Shutdown;
        system_state.active_workers = 0;
        system_state.total_workers = 0;
        system_state.active_models = 0;

        // Reset initialization flag
        *self.is_initialized.write() = false;

        info!("Application state cleanup complete");
        Ok(())
    }

    /// Add worker
    pub fn add_worker(&self, worker: Worker) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();

        workers.insert(worker.id.clone(), worker.clone());
        system_state.total_workers += 1;

        if matches!(worker.status, WorkerStatus::Active) {
            system_state.active_workers += 1;
        }

        system_state.last_activity = Utc::now();

        info!("Added worker: {} (status: {:?})", worker.id, worker.status);
        Ok(())
    }

    /// Remove worker
    pub fn remove_worker(&self, worker_id: &str) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();

        if let Some(worker) = workers.remove(worker_id) {
            system_state.total_workers -= 1;

            if matches!(worker.status, WorkerStatus::Active) {
                system_state.active_workers = system_state.active_workers.saturating_sub(1);
            }

            system_state.last_activity = Utc::now();

            info!("Removed worker: {}", worker_id);
            Ok(())
        } else {
            Err(AppError::ResourceError(format!(
                "Worker '{}' not found",
                worker_id
            )))
        }
    }

    /// Get worker
    pub fn get_worker(&self, worker_id: &str) -> Option<Worker> {
        self.workers.read().get(worker_id).cloned()
    }

    /// Get all workers
    pub fn get_all_workers(&self) -> Vec<Worker> {
        self.workers.read().values().cloned().collect()
    }

    /// Update worker status
    pub fn update_worker_status(
        &self,
        worker_id: &str,
        status: WorkerStatus,
    ) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();

        if let Some(worker) = workers.get_mut(worker_id) {
            let was_active = matches!(worker.status, WorkerStatus::Active);
            let is_active = matches!(status, WorkerStatus::Active);

            worker.status = status.clone();
            worker.last_seen = Utc::now();

            // Update active worker counters
            if was_active && !is_active {
                system_state.active_workers = system_state.active_workers.saturating_sub(1);
            } else if !was_active && is_active {
                system_state.active_workers += 1;
            }

            system_state.last_activity = Utc::now();

            info!("Updated worker {} status to {:?}", worker_id, status);
            Ok(())
        } else {
            Err(AppError::ResourceError(format!(
                "Worker '{}' not found",
                worker_id
            )))
        }
    }

    /// Update worker metrics
    pub fn update_worker_metrics(
        &self,
        worker_id: &str,
        metrics: WorkerMetrics,
    ) -> Result<(), AppError> {
        let mut workers = self.workers.write();

        if let Some(worker) = workers.get_mut(worker_id) {
            worker.metrics = metrics;
            worker.last_seen = Utc::now();
            Ok(())
        } else {
            Err(AppError::ResourceError(format!(
                "Worker '{}' not found",
                worker_id
            )))
        }
    }

    /// Add model state
    pub fn add_model_state(&self, model_name: String, state: ModelState) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut model_states = self.model_states.write();
        let mut system_state = self.system_state.write();

        model_states.insert(model_name.clone(), state.clone());

        if matches!(state.status, ModelStatus::Ready) {
            system_state.active_models += 1;
        }

        system_state.last_activity = Utc::now();

        info!(
            "Added model state: {} (status: {:?})",
            model_name, state.status
        );
        Ok(())
    }

    /// Update model state
    pub fn update_model_state(&self, model_name: &str, state: ModelState) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut model_states = self.model_states.write();
        let mut system_state = self.system_state.write();

        if let Some(existing_state) = model_states.get(model_name) {
            let was_ready = matches!(existing_state.status, ModelStatus::Ready);
            let is_ready = matches!(state.status, ModelStatus::Ready);

            // Update active model counters
            if was_ready && !is_ready {
                system_state.active_models = system_state.active_models.saturating_sub(1);
            } else if !was_ready && is_ready {
                system_state.active_models += 1;
            }
        }

        model_states.insert(model_name.to_string(), state.clone());
        system_state.last_activity = Utc::now();

        info!(
            "Updated model state: {} (status: {:?})",
            model_name, state.status
        );
        Ok(())
    }

    /// Get model state
    pub fn get_model_state(&self, model_name: &str) -> Option<ModelState> {
        self.model_states.read().get(model_name).cloned()
    }

    /// Get all model states
    pub fn get_all_model_states(&self) -> HashMap<String, ModelState> {
        self.model_states.read().clone()
    }

    /// Get uptime
    pub fn get_uptime(&self) -> std::time::Duration {
        let system_state = self.system_state.read();
        let now = Utc::now();
        (now - system_state.start_time).to_std().unwrap_or_default()
    }

    /// Check if system is ready
    pub fn is_ready(&self) -> bool {
        *self.is_initialized.read()
    }

    /// Get system state
    pub fn get_system_state(&self) -> SystemState {
        self.system_state.read().clone()
    }

    /// Update system metrics
    pub fn update_system_metrics(&self, metrics: SystemMetrics) -> Result<(), AppError> {
        let mut system_state = self.system_state.write();
        system_state.system_metrics = metrics;
        system_state.last_activity = Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn app_state_initialize_sets_running_status() {
        let state = AppState::new();
        assert!(!state.is_ready());

        state.initialize().await.expect("initialize should succeed");

        assert!(state.is_ready());
        let system_state = state.get_system_state();
        assert!(matches!(system_state.status, SystemStatus::Running));
    }

    #[tokio::test]
    async fn app_state_cleanup_resets_workers_and_models() {
        let state = AppState::new();
        state.initialize().await.expect("initialize should succeed");

        // Add a worker and a model, then cleanup and verify they are cleared.
        let worker = Worker {
            id: "w-1".to_string(),
            address: "127.0.0.1:1234".to_string(),
            mining_power: 1.0,
            status: WorkerStatus::Active,
            last_seen: Utc::now(),
            metrics: WorkerMetrics::default(),
            active_models: vec!["m-1".to_string()],
        };
        state.add_worker(worker).expect("add_worker should succeed");

        let model_state = ModelState::default();
        state
            .add_model_state("m-1".to_string(), model_state)
            .expect("add_model_state should succeed");

        assert_eq!(state.get_all_workers().len(), 1);
        assert_eq!(state.get_all_model_states().len(), 1);

        state.cleanup().await.expect("cleanup should succeed");

        assert_eq!(state.get_all_workers().len(), 0);
        assert_eq!(state.get_all_model_states().len(), 0);
        assert!(!state.is_ready());
    }
}
