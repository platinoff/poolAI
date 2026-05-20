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

#[cfg(feature = "cloud")]
use crate::cloud::CloudManager;
use crate::core::config::PoolAIConfig;
use crate::core::discovery_handle::SharedDiscoverySlot;
use crate::core::error::AppError;
use crate::core::model_interface::{ModelState, ModelStatus};
#[cfg(feature = "enterprise")]
use crate::core::oauth2_pending::OAuth2PendingEntry;
use crate::core::user_manager::UserManager;
use crate::core::ws_manager::WebSocketManager;
#[cfg(feature = "enterprise")]
use crate::enterprise::audit::AuditLogger;
#[cfg(feature = "enterprise")]
use crate::enterprise::monitoring::MonitoringManager;
#[cfg(feature = "enterprise")]
use crate::enterprise::multi_tenancy::TenantManager;
#[cfg(feature = "enterprise")]
use crate::enterprise::security::SecurityManager;
use crate::libs::LibraryManager;
#[cfg(feature = "ml")]
use crate::ml::pipeline::MLPipelineManager;
use crate::pool::topology::TopologyManager;
use crate::pool::Pool;
use crate::raid::RaidManager;
use crate::rewards::RewardSystem;
use crate::runtime::instance::InstanceManager;
use crate::vm::VmManager;
use chrono::{DateTime, Utc};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock as TokioRwLock;
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
    /// User accounts for HTTP auth (`/login`, `/users`, enterprise OAuth/SAML).
    pub user_manager: Arc<UserManager>,
    /// WebSocket hub for `/ws/metrics` and broadcasts.
    pub ws_manager: Arc<WebSocketManager>,
    /// Discovery service registration (see `network::start_server` when `DiscoveryConfig::enabled`).
    pub discovery: SharedDiscoverySlot,
    /// Worker pool (`pool::initialize` + `attach_core_http_singletons` before serving HTTP).
    pub pool: OnceLock<Arc<TokioRwLock<Pool>>>,
    /// RAID manager (`raid::initialize` + attach).
    pub raid_manager: OnceLock<Arc<RaidManager>>,
    /// VM manager (`vm::initialize` + attach).
    pub vm_manager: OnceLock<Arc<VmManager>>,
    /// Library manager (`libs::initialize` + attach).
    pub library_manager: OnceLock<Arc<TokioRwLock<LibraryManager>>>,
    /// Model/instance manager (`initialize_global_instance_manager` + attach).
    pub instance_manager: OnceLock<Arc<TokioRwLock<InstanceManager>>>,
    /// Topology manager (`initialize_global_topology_manager` + attach).
    pub topology_manager: OnceLock<Arc<TokioRwLock<TopologyManager>>>,
    /// Reward system (`shared_reward_engine` + [`Self::attach_rewards_engine`] from bootstrap).
    pub rewards_engine: OnceLock<Arc<RewardSystem>>,
    /// Cloud integration (`CloudManager::new` + `initialize` in `main`, then attach).
    #[cfg(feature = "cloud")]
    pub cloud_manager: OnceLock<Arc<CloudManager>>,
    /// OAuth2 CSRF state tokens (enterprise GitHub flow).
    #[cfg(feature = "enterprise")]
    pub oauth2_pending_states: Arc<tokio::sync::RwLock<HashMap<String, OAuth2PendingEntry>>>,
    /// Enterprise multi-tenant manager (`/api/enterprise/tenants/*`).
    #[cfg(feature = "enterprise")]
    pub tenant_manager: Arc<TenantManager>,
    /// Enterprise audit log query (`/api/enterprise/audit/*`).
    #[cfg(feature = "enterprise")]
    pub audit_logger: Arc<AuditLogger>,
    /// Enterprise monitoring (alerts, dashboards, metrics).
    #[cfg(feature = "enterprise")]
    pub enterprise_monitoring_manager: Arc<MonitoringManager>,
    /// Enterprise security (OAuth2/SAML providers, policies).
    #[cfg(feature = "enterprise")]
    pub security_manager: Arc<SecurityManager>,
    /// ML.6 orchestration (feature `ml`); shared across `/api/enterprise/ai-ml/pipeline/*`.
    #[cfg(feature = "ml")]
    pub ml_pipeline_manager: Arc<MLPipelineManager>,
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
            user_manager: Arc::new(UserManager::new()),
            ws_manager: Arc::new(WebSocketManager::new()),
            discovery: Arc::new(tokio::sync::RwLock::new(None)),
            pool: OnceLock::new(),
            raid_manager: OnceLock::new(),
            vm_manager: OnceLock::new(),
            library_manager: OnceLock::new(),
            instance_manager: OnceLock::new(),
            topology_manager: OnceLock::new(),
            rewards_engine: OnceLock::new(),
            #[cfg(feature = "cloud")]
            cloud_manager: OnceLock::new(),
            #[cfg(feature = "enterprise")]
            oauth2_pending_states: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            #[cfg(feature = "enterprise")]
            tenant_manager: Arc::new(TenantManager::new()),
            #[cfg(feature = "enterprise")]
            audit_logger: Arc::new(AuditLogger::new()),
            #[cfg(feature = "enterprise")]
            enterprise_monitoring_manager: Arc::new(MonitoringManager::new_from_env()),
            #[cfg(feature = "enterprise")]
            security_manager: Arc::new(SecurityManager::new()),
            #[cfg(feature = "ml")]
            ml_pipeline_manager: Arc::new(MLPipelineManager::new()),
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

    /// Attach pool/RAID/VM/library/instance/topology handles for HTTP handlers (after module `initialize`).
    pub fn attach_core_http_singletons(&self) -> Result<(), String> {
        self.pool
            .set(
                crate::pool::get_global_pool()
                    .ok_or_else(|| "pool global not initialized".to_string())?
                    .clone(),
            )
            .map_err(|_| "pool handle already attached".to_string())?;
        self.raid_manager
            .set(crate::raid::get_global_manager())
            .map_err(|_| "raid_manager handle already attached".to_string())?;
        self.vm_manager
            .set(crate::vm::get_global_manager())
            .map_err(|_| "vm_manager handle already attached".to_string())?;
        self.library_manager
            .set(
                crate::libs::get_global_manager()
                    .ok_or_else(|| "library manager global not initialized".to_string())?
                    .clone(),
            )
            .map_err(|_| "library_manager handle already attached".to_string())?;
        self.instance_manager
            .set(
                crate::runtime::instance::get_global_instance_manager()
                    .ok_or_else(|| "instance manager global not initialized".to_string())?
                    .clone(),
            )
            .map_err(|_| "instance_manager handle already attached".to_string())?;
        self.topology_manager
            .set(
                crate::pool::topology::get_global_topology_manager()
                    .ok_or_else(|| "topology manager global not initialized".to_string())?
                    .clone(),
            )
            .map_err(|_| "topology_manager handle already attached".to_string())?;
        Ok(())
    }

    /// Attach the process-wide reward engine for HTTP (same `Arc` as [`crate::rewards::shared_reward_engine`]).
    pub fn attach_rewards_engine(&self) -> Result<(), String> {
        self.rewards_engine
            .set(crate::rewards::shared_reward_engine())
            .map_err(|_| "rewards_engine handle already attached".to_string())
    }

    /// Attach a pre-initialized cloud manager (set from `main` after `initialize().await`).
    #[cfg(feature = "cloud")]
    pub fn attach_cloud_manager(&self, manager: Arc<CloudManager>) -> Result<(), String> {
        self.cloud_manager
            .set(manager)
            .map_err(|_| "cloud_manager handle already attached".to_string())
    }

    /// Attach core runtime handles for integration tests (no `get_global_*` / full `main` init).
    ///
    /// Enable crate feature **`test-utils`**. Each `OnceLock` may only be set once.
    #[cfg(feature = "test-utils")]
    pub fn attach_pool_for_test(&self, pool: Arc<TokioRwLock<Pool>>) -> Result<(), String> {
        self.pool
            .set(pool)
            .map_err(|_| "pool handle already attached".to_string())
    }

    #[cfg(feature = "test-utils")]
    pub fn attach_raid_manager_for_test(&self, manager: Arc<RaidManager>) -> Result<(), String> {
        self.raid_manager
            .set(manager)
            .map_err(|_| "raid_manager handle already attached".to_string())
    }

    #[cfg(feature = "test-utils")]
    pub fn attach_vm_manager_for_test(&self, manager: Arc<VmManager>) -> Result<(), String> {
        self.vm_manager
            .set(manager)
            .map_err(|_| "vm_manager handle already attached".to_string())
    }

    #[cfg(feature = "test-utils")]
    pub fn attach_library_manager_for_test(
        &self,
        manager: Arc<TokioRwLock<LibraryManager>>,
    ) -> Result<(), String> {
        self.library_manager
            .set(manager)
            .map_err(|_| "library_manager handle already attached".to_string())
    }

    #[cfg(feature = "test-utils")]
    pub fn attach_instance_manager_for_test(
        &self,
        manager: Arc<TokioRwLock<InstanceManager>>,
    ) -> Result<(), String> {
        self.instance_manager
            .set(manager)
            .map_err(|_| "instance_manager handle already attached".to_string())
    }

    #[cfg(feature = "test-utils")]
    pub fn attach_topology_manager_for_test(
        &self,
        manager: Arc<TokioRwLock<TopologyManager>>,
    ) -> Result<(), String> {
        self.topology_manager
            .set(manager)
            .map_err(|_| "topology_manager handle already attached".to_string())
    }

    #[cfg(all(feature = "cloud", feature = "test-utils"))]
    pub fn attach_cloud_manager_for_test(&self, manager: Arc<CloudManager>) -> Result<(), String> {
        self.cloud_manager
            .set(manager)
            .map_err(|_| "cloud_manager handle already attached".to_string())
    }

    /// Attach a custom reward engine for integration tests.
    #[cfg(feature = "test-utils")]
    pub fn attach_rewards_engine_for_test(&self, engine: Arc<RewardSystem>) -> Result<(), String> {
        self.rewards_engine
            .set(engine)
            .map_err(|_| "rewards_engine handle already attached".to_string())
    }

    /// Align legacy `get_global_*` enterprise singletons with this `AppState` (no-op if already set).
    #[cfg(feature = "enterprise")]
    pub fn sync_enterprise_globals(&self) {
        crate::enterprise::multi_tenancy::try_install_global_tenant_manager(
            self.tenant_manager.clone(),
        );
        crate::enterprise::audit::try_install_global_audit_logger(self.audit_logger.clone());
        crate::enterprise::monitoring::try_install_global_monitoring_manager(
            self.enterprise_monitoring_manager.clone(),
        );
        crate::enterprise::security::try_install_global_security_manager(
            self.security_manager.clone(),
        );
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
