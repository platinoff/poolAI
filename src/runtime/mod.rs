//! Runtime Module for Stage 4.1 - Advanced Runtime Management
//!
//! This module provides comprehensive runtime management capabilities including
//! worker lifecycle, task scheduling, resource orchestration, process management,
//! and auto-scaling.
//!
//! # Features
//!
//! - **Worker Management**: Lifecycle management with auto-scaling
//! - **Task Scheduling**: Priority-based task scheduling
//! - **Resource Orchestration**: CPU, memory, and GPU resource allocation
//! - **Process Management**: Process lifecycle, logging, and monitoring
//! - **Caching**: In-memory caching for performance optimization
//! - **Storage Management**: Persistent storage for artifacts and data
//! - **Health Monitoring**: System health checks and recovery
//!
//! # Example
//!
//! ```no_run
//! use poolai::runtime::{RuntimeManager, RuntimeConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create runtime configuration
//! let config = RuntimeConfig {
//!     max_workers: 10,
//!     queue_capacity: 2000,
//!     cache_size_mb: 1024,
//!     auto_scaling: true,
//!     health_check_interval: 30,
//!     resource_monitoring: true,
//! };
//!
//! // Initialize and start runtime manager
//! let mut runtime = RuntimeManager::new(config);
//! runtime.initialize().await?;
//! runtime.start().await?;
//!
//! // Get runtime status
//! let status = runtime.get_status().await;
//! println!("Active workers: {}", status.workers_active);
//! println!("Queue length: {}", status.queue_length);
//!
//! // Shutdown gracefully
//! runtime.shutdown().await?;
//! # Ok(())
//! # }
//! ```

pub mod cache;
pub mod health;
pub mod memory_pool;
pub mod orchestrator;
pub mod process;
pub mod queue;
pub mod scheduler;
pub mod storage;
pub mod worker;

// Re-export main types for easy access
pub use cache::CacheManager;
pub use health::HealthMonitor;
pub use memory_pool::MemoryPool;
pub use orchestrator::ResourceOrchestrator;
pub use process::{ProcessConfig, ProcessLogs, ProcessManager, ProcessStatus};
pub use queue::TaskQueue;
pub use scheduler::TaskScheduler;
pub use storage::StorageManager;
pub use worker::Worker;

/// Runtime configuration for Stage 4.1
///
/// Configures the runtime manager with worker limits, queue capacity,
/// caching, and monitoring settings.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::RuntimeConfig;
///
/// let config = RuntimeConfig {
///     max_workers: 10,
///     queue_capacity: 2000,
///     cache_size_mb: 1024,
///     auto_scaling: true,
///     health_check_interval: 30,
///     resource_monitoring: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum number of concurrent workers
    pub max_workers: usize,
    /// Task queue capacity
    pub queue_capacity: usize,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Auto-scaling enabled
    pub auto_scaling: bool,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
    /// Resource monitoring enabled
    pub resource_monitoring: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_workers: 8,
            queue_capacity: 1000,
            cache_size_mb: 512,
            auto_scaling: true,
            health_check_interval: 30,
            resource_monitoring: true,
        }
    }
}

/// Main runtime manager for Stage 4.1
///
/// Orchestrates all runtime components including workers, scheduler,
/// queue, cache, storage, process manager, resource orchestrator,
/// and health monitor.
///
/// # Example
///
/// ```no_run
/// use poolai::runtime::{RuntimeManager, RuntimeConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = RuntimeConfig::default();
/// let mut runtime = RuntimeManager::new(config);
/// runtime.initialize().await?;
/// runtime.start().await?;
///
/// // Use runtime...
///
/// runtime.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct RuntimeManager {
    #[allow(dead_code)] // Configuration stored for future use (reconfiguration, etc.)
    config: RuntimeConfig,
    worker_manager: Worker,
    scheduler: TaskScheduler,
    queue: TaskQueue,
    cache: CacheManager,
    storage: StorageManager,
    process_manager: ProcessManager,
    orchestrator: ResourceOrchestrator,
    health_monitor: HealthMonitor,
}

impl RuntimeManager {
    /// Create new runtime manager with configuration
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config: config.clone(),
            worker_manager: Worker::new(config.max_workers),
            scheduler: TaskScheduler::new(),
            queue: TaskQueue::new(config.queue_capacity),
            cache: CacheManager::new(config.cache_size_mb),
            storage: StorageManager::new(),
            process_manager: ProcessManager::new(),
            orchestrator: ResourceOrchestrator::new(),
            health_monitor: HealthMonitor::new(config.health_check_interval),
        }
    }

    /// Initialize runtime system
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Initializing Runtime Manager for Stage 4.1");

        // Initialize all components
        self.worker_manager.initialize().await?;
        self.scheduler.initialize().await?;
        self.queue.initialize().await?;
        self.cache.initialize().await?;
        self.storage.initialize().await?;
        self.process_manager.initialize().await?;
        self.orchestrator.initialize().await?;
        self.health_monitor.initialize().await?;

        tracing::info!("Runtime Manager initialized successfully");
        Ok(())
    }

    /// Start runtime system
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting Runtime Manager");

        // Start all components
        self.worker_manager.start().await?;
        self.scheduler.start().await?;
        self.queue.start().await?;
        self.cache.start().await?;
        self.storage.start().await?;
        self.process_manager.start().await?;
        self.orchestrator.start().await?;
        self.health_monitor.start().await?;

        tracing::info!("Runtime Manager started successfully");
        Ok(())
    }

    /// Shutdown runtime system
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Shutting down Runtime Manager");

        // Shutdown all components gracefully
        self.health_monitor.shutdown().await?;
        self.orchestrator.shutdown().await?;
        self.process_manager.shutdown().await?;
        self.storage.shutdown().await?;
        self.cache.shutdown().await?;
        self.queue.shutdown().await?;
        self.scheduler.shutdown().await?;
        self.worker_manager.shutdown().await?;

        tracing::info!("Runtime Manager shutdown completed");
        Ok(())
    }

    /// Get runtime status
    pub async fn get_status(&self) -> RuntimeStatus {
        RuntimeStatus {
            workers_active: self.worker_manager.get_active_count().await,
            queue_length: self.queue.get_length(),
            cache_usage: self.cache.get_usage_percentage().await,
            storage_usage: self.storage.get_usage_percentage(),
            processes_running: self.process_manager.get_running_count(),
            resource_utilization: self.orchestrator.get_resource_utilization(),
            health_score: self.health_monitor.get_health_score(),
        }
    }
}

/// Runtime status information
#[derive(Debug, Clone)]
pub struct RuntimeStatus {
    pub workers_active: usize,
    pub queue_length: usize,
    pub cache_usage: f32,
    pub storage_usage: f32,
    pub processes_running: usize,
    pub resource_utilization: f32,
    pub health_score: f32,
}

/// Initialize global runtime manager
pub async fn initialize_runtime(
    config: RuntimeConfig,
) -> Result<RuntimeManager, Box<dyn std::error::Error>> {
    let mut runtime = RuntimeManager::new(config);
    runtime.initialize().await?;
    runtime.start().await?;
    Ok(runtime)
}
