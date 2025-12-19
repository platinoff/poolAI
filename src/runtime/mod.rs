//! Runtime Module for Stage 4.1 - Advanced Runtime Management
//! 
//! This module provides:
//! - Worker lifecycle management
//! - Task scheduling with priorities
//! - Resource orchestration
//! - Process management
//! - Auto-scaling capabilities

pub mod worker;
pub mod scheduler;
pub mod queue;
pub mod cache;
pub mod storage;
pub mod process;
pub mod orchestrator;
pub mod health;

// Re-export main types for easy access
pub use worker::Worker;
pub use scheduler::TaskScheduler;
pub use queue::TaskQueue;
pub use cache::CacheManager;
pub use storage::StorageManager;
pub use process::{ProcessManager, ProcessConfig, ProcessStatus, ProcessLogs};
pub use orchestrator::ResourceOrchestrator;
pub use health::HealthMonitor;

/// Runtime configuration for Stage 4.1
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
            cache_usage: self.cache.get_usage_percentage(),
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
pub async fn initialize_runtime(config: RuntimeConfig) -> Result<RuntimeManager, Box<dyn std::error::Error>> {
    let mut runtime = RuntimeManager::new(config);
    runtime.initialize().await?;
    runtime.start().await?;
    Ok(runtime)
}
