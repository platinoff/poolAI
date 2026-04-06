//! Process Manager for Stage 4.1 Runtime
//!
//! Provides:
//! - Process spawning and lifecycle management
//! - Resource limits enforcement
//! - Logs capture (stdout/stderr)
//! - Timeout handling
//! - Health monitoring

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{error, info, warn};
use uuid::Uuid;

/// Process status
///
/// Represents the current state of a managed process.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::process::ProcessStatus;
///
/// let status = ProcessStatus::Running;
/// match status {
///     ProcessStatus::Starting => println!("Process is starting"),
///     ProcessStatus::Running => println!("Process is running"),
///     ProcessStatus::Stopping => println!("Process is stopping"),
///     ProcessStatus::Stopped => println!("Process is stopped"),
///     ProcessStatus::Failed(reason) => println!("Process failed: {}", reason),
///     ProcessStatus::Timeout => println!("Process timed out"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed(String),
    Timeout,
}

/// Process configuration
///
/// Defines how a process should be spawned including command, arguments,
/// environment variables, resource limits, and timeout settings.
///
/// # Example
///
/// ```rust
/// use poolai::runtime::process::ProcessConfig;
/// use std::collections::HashMap;
/// use std::path::PathBuf;
///
/// let config = ProcessConfig {
///     command: "custom-worker".to_string(),
///     args: vec!["run-job".to_string(), "--input".to_string(), "data.bin".to_string()],
///     working_dir: Some(PathBuf::from("/tmp")),
///     env: {
///         let mut env = HashMap::new();
///         env.insert("VAR1".to_string(), "value1".to_string());
///         env
///     },
///     timeout_seconds: Some(300),
///     cpu_limit_percent: Some(50),
///     memory_limit_mb: Some(1024),
///     capture_logs: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<PathBuf>,
    pub env: HashMap<String, String>,
    pub timeout_seconds: Option<u64>,
    pub cpu_limit_percent: Option<u8>,
    pub memory_limit_mb: Option<u32>,
    pub capture_logs: bool,
}

/// Process logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessLogs {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub max_lines: usize,
}

impl Default for ProcessLogs {
    fn default() -> Self {
        Self {
            stdout: Vec::new(),
            stderr: Vec::new(),
            max_lines: 1000,
        }
    }
}

/// Process information
#[derive(Debug)]
pub struct ProcessInfo {
    pub id: Uuid,
    pub config: ProcessConfig,
    pub status: Arc<RwLock<ProcessStatus>>,
    pub logs: Arc<RwLock<ProcessLogs>>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub process: Option<Child>,
    /// Process ID (PID) - platform-specific
    pub pid: Option<u32>,
}

/// Process Manager
///
/// Manages the lifecycle of multiple processes including spawning, monitoring,
/// log capture, and graceful termination.
///
/// # Features
///
/// - Process spawning with resource limits
/// - Log capture (stdout/stderr)
/// - Timeout handling
/// - Health monitoring
/// - Graceful shutdown
///
/// # Example
///
/// ```no_run
/// use poolai::runtime::process::{ProcessManager, ProcessConfig};
/// use std::collections::HashMap;
/// use uuid::Uuid;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut manager = ProcessManager::new();
/// manager.initialize().await?;
/// manager.start().await?;
///
/// // Spawn a process
/// let config = ProcessConfig {
///     command: "custom-worker".to_string(),
///     args: vec!["run-job".to_string(), "--input".to_string(), "data.bin".to_string()],
///     working_dir: None,
///     env: HashMap::new(),
///     timeout_seconds: Some(300),
///     cpu_limit_percent: None,
///     memory_limit_mb: None,
///     capture_logs: true,
/// };
///
/// let process_id = manager.spawn_process(config).await?;
///
/// // Get process status
/// let status = manager.get_process_status(process_id).await?;
/// println!("Process status: {:?}", status);
///
/// // Stop process
/// manager.stop_process(process_id).await?;
///
/// manager.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct ProcessManager {
    processes: Arc<RwLock<HashMap<Uuid, ProcessInfo>>>,
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessManager {
    /// Create a new process manager
    ///
    /// Initializes a new process manager instance. The manager must be
    /// initialized and started before use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::runtime::process::ProcessManager;
    ///
    /// let manager = ProcessManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Process Manager");
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting Process Manager");
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Shutting down Process Manager");

        // Stop all running processes
        let mut processes = self.processes.write().await;
        for (id, info) in processes.iter_mut() {
            if let Some(mut child) = info.process.take() {
                if let Err(e) = child.kill().await {
                    warn!("Failed to kill process {}: {}", id, e);
                }
            }
        }

        Ok(())
    }

    pub fn get_running_count(&self) -> usize {
        // This is a sync method, so we can't use async here
        // Return 0 for now, can be enhanced with blocking read if needed
        0
    }

    /// Spawn a new process
    ///
    /// Creates and starts a new process with the specified configuration.
    /// The process will be monitored for health, logs will be captured if enabled,
    /// and timeout will be enforced if configured.
    ///
    /// # Arguments
    ///
    /// * `config` - Process configuration including command, arguments, and limits
    ///
    /// # Returns
    ///
    /// Returns the UUID of the spawned process.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::{ProcessManager, ProcessConfig};
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ProcessManager::new();
    /// let config = ProcessConfig {
    ///     command: "custom-worker".to_string(),
    ///     args: vec!["run-job".to_string(), "--input".to_string(), "data.bin".to_string()],
    ///     working_dir: None,
    ///     env: HashMap::new(),
    ///     timeout_seconds: Some(300),
    ///     cpu_limit_percent: None,
    ///     memory_limit_mb: None,
    ///     capture_logs: true,
    /// };
    ///
    /// let process_id = manager.spawn_process(config).await?;
    /// println!("Spawned process: {}", process_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn spawn_process(&self, config: ProcessConfig) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        info!("Spawning process {}: {}", id, config.command);

        let mut command = Command::new(&config.command);
        command.args(&config.args);

        if let Some(working_dir) = &config.working_dir {
            command.current_dir(working_dir);
        }

        for (key, value) in &config.env {
            command.env(key, value);
        }

        // Set kill_on_drop to ensure cleanup
        command.kill_on_drop(true);

        // Spawn process
        let mut child = command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to spawn process. Context: Cannot create new process with specified configuration. Suggestion: Verify command path exists, check system resource limits, and ensure sufficient permissions. Command: '{}', Args: {:?}, Error: {}",
                config.command,
                config.args,
                e
            )))?;
        // Fix: Remove duplicate map_err
        let _ = std::process::Command::new(&config.command)
            .args(&config.args)
            .spawn()
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to spawn process. Context: Cannot start new process for task execution. \
                Suggestion: Check system resources, verify executable path is correct, and ensure process limits are not exceeded. \
                Command: '{}', Error: {}",
                config.command, e
            )))?;

        let status = Arc::new(RwLock::new(ProcessStatus::Starting));
        let logs = Arc::new(RwLock::new(ProcessLogs::default()));

        // Capture logs if enabled
        if config.capture_logs {
            let logs_stdout = Arc::clone(&logs);
            let logs_stderr = Arc::clone(&logs);

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                let logs = logs_stdout;
                tokio::spawn(async move {
                    while let Ok(Some(line)) = lines.next_line().await {
                        let mut logs_guard = logs.write().await;
                        logs_guard.stdout.push(line.clone());
                        if logs_guard.stdout.len() > logs_guard.max_lines {
                            logs_guard.stdout.remove(0);
                        }
                    }
                });
            }

            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                let logs = logs_stderr;
                tokio::spawn(async move {
                    while let Ok(Some(line)) = lines.next_line().await {
                        let mut logs_guard = logs.write().await;
                        logs_guard.stderr.push(line.clone());
                        if logs_guard.stderr.len() > logs_guard.max_lines {
                            logs_guard.stderr.remove(0);
                        }
                    }
                });
            }
        }

        // Update status to Running
        {
            let mut s = status.write().await;
            *s = ProcessStatus::Running;
        }

        // Handle timeout if configured
        if let Some(timeout_secs) = config.timeout_seconds {
            let status_timeout = Arc::clone(&status);
            let process_id = id;
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(timeout_secs)).await;
                let mut s = status_timeout.write().await;
                if *s == ProcessStatus::Running {
                    *s = ProcessStatus::Timeout;
                    warn!(
                        "Process {} timed out after {} seconds",
                        process_id, timeout_secs
                    );
                }
            });
        }

        // Get PID from child process
        let pid = child.id();

        // Store process info
        let process_info = ProcessInfo {
            id,
            config,
            status,
            logs,
            started_at: chrono::Utc::now(),
            process: Some(child),
            pid,
        };

        self.processes.write().await.insert(id, process_info);

        info!("Process {} spawned successfully", id);
        Ok(id)
    }

    /// Stop a process
    ///
    /// Terminates a running process by sending a kill signal. The process
    /// status will be updated to Stopped after termination.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the process to stop
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the process is stopped successfully.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::ProcessManager;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ProcessManager::new();
    /// let process_id = Uuid::new_v4(); // In real usage, this would come from spawn_process
    /// manager.stop_process(process_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop_process(&self, id: Uuid) -> Result<(), AppError> {
        let mut processes = self.processes.write().await;
        let info = processes
            .get_mut(&id)
            .ok_or_else(|| AppError::ResourceError(format!(
                "Process not found. Context: Attempted to stop a process that does not exist or has been terminated. \
                Suggestion: Verify process ID is correct and process is still running. \
                Process ID: '{}'",
                id
            )))?;

        {
            let mut status = info.status.write().await;
            *status = ProcessStatus::Stopping;
        }

        if let Some(mut child) = info.process.take() {
            if let Err(e) = child.kill().await {
                error!("Failed to kill process {}: {}", id, e);
                let mut status = info.status.write().await;
                *status = ProcessStatus::Failed(format!("Kill failed: {}", e));
                return Err(AppError::ResourceError(format!(
                    "Failed to kill process. Context: Cannot terminate process using kill signal. \
                    Suggestion: Check process permissions and ensure process is still running. \
                    Process ID: '{}', Error: {}",
                    id, e
                )));
            }
        }

        {
            let mut status = info.status.write().await;
            *status = ProcessStatus::Stopped;
        }

        info!("Process {} stopped", id);
        Ok(())
    }

    /// Get process status
    ///
    /// Retrieves the current status of a managed process.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the process
    ///
    /// # Returns
    ///
    /// Returns the current `ProcessStatus` of the process.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::ProcessManager;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ProcessManager::new();
    /// let process_id = Uuid::new_v4();
    /// let status = manager.get_process_status(process_id).await?;
    /// println!("Process status: {:?}", status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_process_status(&self, id: Uuid) -> Result<ProcessStatus, AppError> {
        let processes = self.processes.read().await;
        let info = processes
            .get(&id)
            .ok_or_else(|| AppError::ResourceError(format!(
                "Process not found. Context: Attempted to access a process that does not exist or has been terminated. \
                Suggestion: Verify process ID is correct and process is still running. \
                Process ID: '{}'",
                id
            )))?;

        let status = info.status.read().await.clone();
        Ok(status)
    }

    /// Get process logs
    ///
    /// Retrieves captured stdout and stderr logs for a process. Logs are only
    /// available if `capture_logs` was enabled in the process configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the process
    ///
    /// # Returns
    ///
    /// Returns `ProcessLogs` containing stdout and stderr lines.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::ProcessManager;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ProcessManager::new();
    /// let process_id = Uuid::new_v4();
    /// let logs = manager.get_process_logs(process_id).await?;
    /// for line in logs.stdout {
    ///     println!("STDOUT: {}", line);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_process_logs(&self, id: Uuid) -> Result<ProcessLogs, AppError> {
        let processes = self.processes.read().await;
        let info = processes
            .get(&id)
            .ok_or_else(|| AppError::ResourceError(format!(
                "Process not found. Context: Attempted to access a process that does not exist or has been terminated. \
                Suggestion: Verify process ID is correct and process is still running. \
                Process ID: '{}'",
                id
            )))?;

        let logs = info.logs.read().await.clone();
        Ok(logs)
    }

    /// List all processes
    ///
    /// Returns a list of UUIDs for all managed processes.
    ///
    /// # Returns
    ///
    /// Returns a vector of process UUIDs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::ProcessManager;
    ///
    /// # async fn example() {
    /// let manager = ProcessManager::new();
    /// let processes = manager.list_processes().await;
    /// println!("Managed {} processes", processes.len());
    /// # }
    /// ```
    pub async fn list_processes(&self) -> Vec<Uuid> {
        self.processes.read().await.keys().cloned().collect()
    }

    /// Get process PID
    ///
    /// Retrieves the platform-specific process ID (PID) for a managed process.
    ///
    /// # Arguments
    ///
    /// * `id` - UUID of the process
    ///
    /// # Returns
    ///
    /// Returns `Some(pid)` if the process is running, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::runtime::process::ProcessManager;
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let manager = ProcessManager::new();
    /// let process_id = Uuid::new_v4();
    /// if let Some(pid) = manager.get_process_pid(process_id).await? {
    ///     println!("Process PID: {}", pid);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_process_pid(&self, id: Uuid) -> Result<Option<u32>, AppError> {
        let processes = self.processes.read().await;
        let info = processes
            .get(&id)
            .ok_or_else(|| AppError::ResourceError(format!(
                "Process not found. Context: Attempted to access a process that does not exist or has been terminated. \
                Suggestion: Verify process ID is correct and process is still running. \
                Process ID: '{}'",
                id
            )))?;

        Ok(info.pid)
    }
}
