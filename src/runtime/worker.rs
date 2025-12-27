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
            // TODO: Implement Windows process priority setting
            // Requires windows crate dependency
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
                        // TODO: Implement restart logic
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
    pub async fn update_metrics(&self, new_metrics: WorkerMetrics) {
        let mut metrics = self.metrics.write().await;
        *metrics = new_metrics;
    }

    /// Get worker status
    pub async fn get_status(&self) -> WorkerStatus {
        let status = self.status.read().await;
        status.clone()
    }

    /// Get worker metrics
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
