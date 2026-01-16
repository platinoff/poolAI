//! Worker management for Stage 4.1 Runtime
//!
//! Provides lifecycle management for AI mining workers with:
//! - Process spawning and monitoring
//! - Resource allocation
//! - Health monitoring
//! - Auto-scaling capabilities

use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};

/// Worker status enumeration
///
/// Represents the current state of a worker instance in the runtime system.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::worker::WorkerStatus;
///
/// let status = WorkerStatus::Ready;
/// match status {
///     WorkerStatus::Initializing => println!("Worker is initializing"),
///     WorkerStatus::Ready => println!("Worker is ready"),
///     WorkerStatus::Busy => println!("Worker is busy"),
///     WorkerStatus::Idle => println!("Worker is idle"),
///     WorkerStatus::Error => println!("Worker has error"),
///     WorkerStatus::Shutdown => println!("Worker is shutdown"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    Initializing,
    Ready,
    Busy,
    Idle,
    Error,
    Shutdown,
}

/// Worker configuration
///
/// Defines configuration parameters for a worker instance including resource
/// limits, GPU assignment, and health monitoring settings.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::worker::WorkerConfig;
///
/// let config = WorkerConfig {
///     id: "worker-1".to_string(),
///     max_memory_mb: 4096,
///     cpu_priority: 7,
///     gpu_device: Some(0),
///     auto_restart: true,
///     health_check_interval: 30,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// Worker ID
    pub id: String,
    /// Maximum memory usage (MB)
    pub max_memory_mb: usize,
    /// CPU priority (1-10)
    pub cpu_priority: u8,
    /// GPU device ID (if applicable)
    pub gpu_device: Option<usize>,
    /// Auto-restart on failure
    pub auto_restart: bool,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
}

/// Worker metrics
///
/// Tracks performance and resource usage metrics for a worker instance.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::worker::WorkerMetrics;
/// use chrono::Utc;
///
/// let metrics = WorkerMetrics {
///     cpu_usage: 45.5,
///     memory_usage_mb: 2048.0,
///     gpu_usage: Some(78.0),
///     tasks_completed: 100,
///     tasks_failed: 2,
///     avg_task_duration_ms: 250.0,
///     last_activity: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage (MB)
    pub memory_usage_mb: f32,
    /// GPU usage percentage (if applicable)
    pub gpu_usage: Option<f32>,
    /// Tasks completed
    pub tasks_completed: u64,
    /// Tasks failed
    pub tasks_failed: u64,
    /// Average task duration (ms)
    pub avg_task_duration_ms: f32,
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Worker instance
///
/// Manages the lifecycle of an AI mining worker process including spawning,
/// monitoring, health checks, and graceful shutdown.
///
/// # Example
///
/// ```no_run
/// use poolai::runtime::worker::Worker;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut worker = Worker::new(1);
/// worker.initialize().await?;
/// worker.start().await?;
///
/// // Get worker status
/// let status = worker.get_status().await;
/// println!("Worker status: {:?}", status);
///
/// // Get worker metrics
/// let metrics = worker.get_metrics().await;
/// println!("CPU usage: {:.1}%", metrics.cpu_usage);
///
/// worker.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct Worker {
    config: WorkerConfig,
    status: Arc<RwLock<WorkerStatus>>,
    metrics: Arc<RwLock<WorkerMetrics>>,
    process: Option<Child>,
    #[allow(dead_code)] // Will be used for task distribution in future
    task_channel: mpsc::Sender<WorkerTask>,
    health_monitor: tokio::task::JoinHandle<()>,
}

/// Worker task definition
#[derive(Debug, Clone)]
pub struct WorkerTask {
    pub id: String,
    pub task_type: String,
    pub priority: u8,
    pub payload: Vec<u8>,
    pub timeout_seconds: u64,
}

impl Worker {
    /// Create new worker instance
    ///
    /// Initializes a new worker with default configuration. The worker must be
    /// initialized and started before use.
    ///
    /// # Arguments
    ///
    /// * `_max_workers` - Maximum number of workers (reserved for future use)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let worker = Worker::new(1);
    /// # }
    /// ```
    pub fn new(_max_workers: usize) -> Self {
        let (tx, _rx) = mpsc::channel(100);

        Self {
            config: WorkerConfig {
                id: format!("worker-{}", uuid::Uuid::new_v4()),
                max_memory_mb: 2048,
                cpu_priority: 5,
                gpu_device: None,
                auto_restart: true,
                health_check_interval: 30,
            },
            status: Arc::new(RwLock::new(WorkerStatus::Initializing)),
            metrics: Arc::new(RwLock::new(WorkerMetrics::default())),
            process: None,
            task_channel: tx,
            health_monitor: tokio::task::spawn(async {}),
        }
    }

    /// Initialize worker
    ///
    /// Prepares the worker for operation by setting up health monitoring
    /// and transitioning to Ready status.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if initialization succeeds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut worker = Worker::new(1);
    /// worker.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing worker {}", self.config.id);

        // Set status to ready
        {
            let mut status = self.status.write().await;
            *status = WorkerStatus::Ready;
        }

        // Start health monitoring
        self.start_health_monitoring().await?;

        info!("Worker {} initialized successfully", self.config.id);
        Ok(())
    }

    /// Start worker
    ///
    /// Spawns the worker process and transitions the worker to Ready status.
    /// The worker process will run as a separate child process.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the worker process is spawned successfully.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut worker = Worker::new(1);
    /// worker.initialize().await?;
    /// worker.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting worker {}", self.config.id);

        // Spawn worker process
        self.spawn_process().await?;

        // Update status
        {
            let mut status = self.status.write().await;
            *status = WorkerStatus::Ready;
        }

        info!("Worker {} started successfully", self.config.id);
        Ok(())
    }

    /// Shutdown worker
    ///
    /// Gracefully shuts down the worker by terminating the process and
    /// stopping health monitoring.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if shutdown completes successfully.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut worker = Worker::new(1);
    /// worker.initialize().await?;
    /// worker.start().await?;
    /// worker.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Shutting down worker {}", self.config.id);

        // Update status
        {
            let mut status = self.status.write().await;
            *status = WorkerStatus::Shutdown;
        }

        // Terminate process if running
        if let Some(mut process) = self.process.take() {
            if let Err(e) = process.kill().await {
                warn!("Failed to kill worker process: {}", e);
            }
        }

        // Cancel health monitoring
        self.health_monitor.abort();

        info!("Worker {} shutdown completed", self.config.id);
        Ok(())
    }

    /// Get active worker count
    pub async fn get_active_count(&self) -> usize {
        let status = self.status.read().await;
        if *status == WorkerStatus::Ready || *status == WorkerStatus::Busy {
            1
        } else {
            0
        }
    }

    /// Spawn worker process
    async fn spawn_process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Prefer spawning the sibling worker binary located next to the current executable.
        // This avoids relying on PATH (common source of `ErrorKind::NotFound` on Windows/MSYS).
        let worker_exe = match std::env::current_exe() {
            Ok(exe) => {
                #[cfg(target_os = "windows")]
                let name = "poolai-worker.exe";
                #[cfg(not(target_os = "windows"))]
                let name = "poolai-worker";
                exe.with_file_name(name)
            }
            Err(_) => {
                #[cfg(target_os = "windows")]
                let name = "poolai-worker.exe";
                #[cfg(not(target_os = "windows"))]
                let name = "poolai-worker";
                std::path::PathBuf::from(name)
            }
        };

        let mut command = if worker_exe.exists() {
            Command::new(worker_exe)
        } else {
            // Fallback to PATH lookup for custom deployments.
            #[cfg(target_os = "windows")]
            let name = "poolai-worker.exe";
            #[cfg(not(target_os = "windows"))]
            let name = "poolai-worker";
            Command::new(name)
        };

        // Ensure the child process is terminated when the parent drops the handle (graceful shutdown path).
        // This prevents orphaned `poolai-worker` processes during development and normal Ctrl+C shutdown.
        command.kill_on_drop(true);

        // Set process priority (Windows-specific code commented out for now)
        #[cfg(target_os = "windows")]
        {
            // Future improvement: Implement Windows process priority setting
            // 1. Open process handle using OpenProcess() Windows API
            //    - Use PROCESS_SET_INFORMATION access right
            //    - Handle must be closed with CloseHandle() when done
            // 2. Set priority class using SetPriorityClass() Windows API
            //    - Use HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, or BELOW_NORMAL_PRIORITY_CLASS
            //    - Example: SetPriorityClass(process_handle, HIGH_PRIORITY_CLASS)
            // 3. Optionally set thread priority using SetThreadPriority() Windows API
            //    - Use THREAD_PRIORITY_* constants for fine-grained control
            //    - Requires thread handle from process
            //
            // This requires:
            // - Windows API bindings (windows-sys crate or winapi crate)
            // - Process handle management (proper cleanup)
            // - Understanding of Windows priority classes
        }

        // Spawn process
        let child = match command
            .arg("--worker-id")
            .arg(&self.config.id)
            .arg("--max-memory")
            .arg(self.config.max_memory_mb.to_string())
            .spawn()
        {
            Ok(child) => child,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                warn!(
                    "Worker binary not found; continuing without external worker process (id={}). \
                     Build/run `poolai-worker` or ensure it is next to the main executable.",
                    self.config.id
                );
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };

        self.process = Some(child);
        Ok(())
    }

    /// Start health monitoring
    async fn start_health_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let status = Arc::clone(&self.status);
        let metrics = Arc::clone(&self.metrics);
        let config = self.config.clone();

        self.health_monitor = tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
                config.health_check_interval,
            ));

            loop {
                interval.tick().await;

                // Check worker health
                if let Err(e) = Self::check_worker_health(&status, &metrics).await {
                    warn!("Health check failed for worker {}: {}", config.id, e);

                    // Auto-restart if enabled
                    if config.auto_restart {
                        warn!("Auto-restarting worker {}", config.id);
                        // Future improvement: Implement restart logic
                        // 1. Wait for process to fully terminate using wait() or kill()
                        //    - Ensure process is completely stopped before restart
                        //    - Use timeout to avoid hanging on unresponsive processes
                        // 2. Check restart attempts counter against max_restart_attempts
                        //    - Increment restart attempts counter
                        //    - If max attempts reached, mark worker as failed
                        // 3. Calculate restart delay using exponential backoff (if enabled)
                        //    - Use exponential backoff: delay = initial * 2^attempts
                        //    - Cap delay at max_restart_delay_secs
                        //    - Sleep for calculated delay using tokio::time::sleep()
                        // 4. Re-spawn worker process using the same spawn logic
                        //    - Use command.spawn() to create new process
                        //    - Update process_id in worker status
                        //    - Reset restart attempts counter on successful restart
                        // 5. Re-attach monitoring and health checks for new process
                        //    - Re-register health checks
                        //    - Re-attach process monitoring
                        //
                        // This requires:
                        // - Process state tracking (restart attempts, last restart time)
                        // - Delay calculation logic (exponential backoff)
                        // - Process spawning (reuse existing spawn logic)
                        // - Health check re-registration
                    }
                }
            }
        });

        Ok(())
    }

    /// Check worker health
    async fn check_worker_health(
        status: &Arc<RwLock<WorkerStatus>>,
        metrics: &Arc<RwLock<WorkerMetrics>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_status = status.read().await;
        let current_metrics = metrics.read().await;

        // Check if worker is responsive
        if *current_status == WorkerStatus::Error {
            return Err("Worker is in error state".into());
        }

        // Check memory usage
        if current_metrics.memory_usage_mb > 2048.0 {
            warn!(
                "Worker memory usage high: {} MB",
                current_metrics.memory_usage_mb
            );
        }

        // Check CPU usage
        if current_metrics.cpu_usage > 90.0 {
            warn!("Worker CPU usage high: {}%", current_metrics.cpu_usage);
        }

        Ok(())
    }

    /// Update worker metrics
    ///
    /// Updates the worker's performance and resource usage metrics.
    ///
    /// # Arguments
    ///
    /// * `new_metrics` - New metrics to store
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::{Worker, WorkerMetrics};
    /// use chrono::Utc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let worker = Worker::new(1);
    /// let metrics = WorkerMetrics {
    ///     cpu_usage: 50.0,
    ///     memory_usage_mb: 1024.0,
    ///     gpu_usage: Some(75.0),
    ///     tasks_completed: 50,
    ///     tasks_failed: 0,
    ///     avg_task_duration_ms: 200.0,
    ///     last_activity: Utc::now(),
    /// };
    /// worker.update_metrics(metrics).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_metrics(&self, new_metrics: WorkerMetrics) {
        let mut metrics = self.metrics.write().await;
        *metrics = new_metrics;
    }

    /// Get worker status
    ///
    /// Retrieves the current status of the worker.
    ///
    /// # Returns
    ///
    /// Returns the current `WorkerStatus`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let worker = Worker::new(1);
    /// let status = worker.get_status().await;
    /// println!("Worker status: {:?}", status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self) -> WorkerStatus {
        let status = self.status.read().await;
        status.clone()
    }

    /// Get worker metrics
    ///
    /// Retrieves the current performance and resource usage metrics.
    ///
    /// # Returns
    ///
    /// Returns a clone of the current `WorkerMetrics`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::worker::Worker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let worker = Worker::new(1);
    /// let metrics = worker.get_metrics().await;
    /// println!("CPU: {:.1}%, Memory: {:.1}MB", metrics.cpu_usage, metrics.memory_usage_mb);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_metrics(&self) -> WorkerMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage_mb: 0.0,
            gpu_usage: None,
            tasks_completed: 0,
            tasks_failed: 0,
            avg_task_duration_ms: 0.0,
            last_activity: chrono::Utc::now(),
        }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            id: "default-worker".to_string(),
            max_memory_mb: 2048,
            cpu_priority: 5,
            gpu_device: None,
            auto_restart: true,
            health_check_interval: 30,
        }
    }
}
