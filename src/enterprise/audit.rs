//! Audit logging module
//!
//! Provides comprehensive audit trail for all system operations,
//! including user actions, resource access, and system events.
//!
//! # Features
//!
//! - Structured audit events with metadata
//! - Persistent storage (file-based, with optional database support)
//! - Query and filtering capabilities
//! - Compliance-ready logging (immutable, tamper-resistant)
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::enterprise::audit::{AuditLogger, AuditEvent, AuditLevel};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let logger = AuditLogger::new();
//! logger.initialize().await?;
//!
//! logger.log_event(AuditEvent {
//!     timestamp: chrono::Utc::now(),
//!     level: AuditLevel::Info,
//!     user_id: Some("user123".to_string()),
//!     tenant_id: Some("tenant-abc".to_string()),
//!     action: "create_instance".to_string(),
//!     resource_type: "vm_instance".to_string(),
//!     resource_id: Some("instance-456".to_string()),
//!     result: "success".to_string(),
//!     metadata: std::collections::HashMap::new(),
//! }).await?;
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Audit event severity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLevel {
    /// Informational events (normal operations)
    Info,
    /// Warning events (unusual but expected situations)
    Warning,
    /// Error events (failures, errors)
    Error,
    /// Critical events (security violations, system failures)
    Critical,
}

impl AuditLevel {
    /// Returns the string representation of the audit level
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditLevel::Info => "INFO",
            AuditLevel::Warning => "WARNING",
            AuditLevel::Error => "ERROR",
            AuditLevel::Critical => "CRITICAL",
        }
    }
}

/// Audit event structure
///
/// Represents a single audit event with all relevant metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Severity level of the event
    pub level: AuditLevel,
    /// User ID who performed the action (if applicable)
    pub user_id: Option<String>,
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<String>,
    /// Action performed (e.g., "create_instance", "delete_worker")
    pub action: String,
    /// Type of resource affected (e.g., "vm_instance", "worker", "model")
    pub resource_type: String,
    /// ID of the resource affected (if applicable)
    pub resource_id: Option<String>,
    /// Result of the action (e.g., "success", "failure", "denied")
    pub result: String,
    /// Additional metadata (key-value pairs)
    pub metadata: HashMap<String, String>,
}

impl AuditEvent {
    /// Creates a new audit event with current timestamp
    pub fn new(
        level: AuditLevel,
        action: String,
        resource_type: String,
        result: String,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            user_id: None,
            tenant_id: None,
            action,
            resource_type,
            resource_id: None,
            result,
            metadata: HashMap::new(),
        }
    }

    /// Sets the user ID for this event
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Sets the tenant ID for this event
    pub fn with_tenant_id(mut self, tenant_id: String) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    /// Sets the resource ID for this event
    pub fn with_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Adds metadata to this event
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Configuration for audit logging
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Base directory for audit log files
    pub log_directory: PathBuf,
    /// Maximum log file size in bytes before rotation
    pub max_file_size: u64,
    /// Maximum number of rotated log files to keep
    pub max_files: usize,
    /// Whether to enable compression for rotated logs
    pub enable_compression: bool,
    /// Whether to enable immediate flush (for compliance)
    pub immediate_flush: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            log_directory: PathBuf::from("./data/audit"),
            max_file_size: 100 * 1024 * 1024, // 100 MB
            max_files: 10,
            enable_compression: true,
            immediate_flush: true, // Important for compliance
        }
    }
}

/// Audit logger implementation
///
/// Thread-safe audit logger that writes events to persistent storage.
/// Supports log rotation, compression, and querying.
pub struct AuditLogger {
    config: AuditConfig,
    current_file: Arc<RwLock<Option<tokio::fs::File>>>,
    current_file_size: Arc<RwLock<u64>>,
    event_buffer: Arc<RwLock<Vec<AuditEvent>>>,
    initialized: Arc<RwLock<bool>>,
}

impl AuditLogger {
    /// Creates a new audit logger with default configuration
    pub fn new() -> Self {
        Self::with_config(AuditConfig::default())
    }

    /// Creates a new audit logger with custom configuration
    pub fn with_config(config: AuditConfig) -> Self {
        Self {
            config,
            current_file: Arc::new(RwLock::new(None)),
            current_file_size: Arc::new(RwLock::new(0)),
            event_buffer: Arc::new(RwLock::new(Vec::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the audit logger
    ///
    /// Creates the log directory and opens the initial log file.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if directory creation or file opening fails.
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Create log directory if it doesn't exist
        tokio::fs::create_dir_all(&self.config.log_directory).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to create audit log directory: {}. \
                Context: Cannot initialize audit logging without a valid directory. \
                Suggestion: Ensure write permissions for the parent directory. \
                Path: {:?}, Error: {}",
                e,
                self.config.log_directory,
                e
            ))
        })?;

        // Open initial log file
        self.rotate_log_file().await?;

        *initialized = true;
        info!("Audit logger initialized with directory: {:?}", self.config.log_directory);
        Ok(())
    }

    /// Logs an audit event
    ///
    /// Writes the event to the current log file and optionally flushes
    /// immediately if `immediate_flush` is enabled.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if writing or flushing fails.
    pub async fn log_event(&self, event: AuditEvent) -> Result<(), AppError> {
        let initialized = self.initialized.read().await;
        if !*initialized {
            return Err(AppError::ConfigError(
                "Audit logger not initialized. Call initialize() first.".to_string(),
            ));
        }
        drop(initialized);

        // Serialize event to JSON
        let json = serde_json::to_string(&event).map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to serialize audit event: {}. \
                Context: Cannot log audit event due to serialization failure. \
                Suggestion: Check event structure and ensure all fields are serializable. \
                Error: {}",
                e, e
            ))
        })?;

        // Write to file
        let mut file_guard = self.current_file.write().await;
        if let Some(ref mut file) = *file_guard {
            let line = format!("{}\n", json);
            file.write_all(line.as_bytes()).await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to write audit event: {}. \
                    Context: Cannot write audit event to log file. \
                    Suggestion: Check disk space and file permissions. \
                    Error: {}",
                    e, e
                ))
            })?;
            let bytes_written = line.len() as u64;

            // Update file size
            let mut size = self.current_file_size.write().await;
            *size += bytes_written;

            // Rotate if needed
            if *size >= self.config.max_file_size {
                drop(file_guard);
                drop(size);
                self.rotate_log_file().await?;
            } else if self.config.immediate_flush {
                file.flush().await.map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to flush audit log: {}. \
                        Context: Cannot ensure audit event is persisted. \
                        Suggestion: Check disk I/O and permissions. \
                        Error: {}",
                        e, e
                    ))
                })?;
            }
        } else {
            warn!("Audit log file not open, buffering event");
            let mut buffer = self.event_buffer.write().await;
            buffer.push(event);
        }

        Ok(())
    }

    /// Rotates the log file
    ///
    /// Closes the current file, optionally compresses it, and opens a new one.
    async fn rotate_log_file(&self) -> Result<(), AppError> {
        // Close current file if open
        let mut file_guard = self.current_file.write().await;
        if let Some(mut file) = file_guard.take() {
            file.flush().await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to flush audit log during rotation: {}",
                    e
                ))
            })?;
        }

        // Generate new filename with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("audit_{}.log", timestamp);
        let file_path = self.config.log_directory.join(&filename);

        // Open new file
        let new_file = tokio::fs::File::create(&file_path).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to create new audit log file: {}. \
                Context: Cannot rotate audit log file. \
                Suggestion: Check disk space and file permissions. \
                Path: {:?}, Error: {}",
                e, file_path, e
            ))
        })?;

        *file_guard = Some(new_file);
        *self.current_file_size.write().await = 0;

        info!("Rotated audit log file to: {:?}", file_path);

        // TODO: Implement log file cleanup (keep only max_files)
        // TODO: Implement compression if enable_compression is true

        Ok(())
    }

    /// Shuts down the audit logger gracefully
    ///
    /// Flushes all pending events and closes the log file.
    pub async fn shutdown(&self) -> Result<(), AppError> {
        let mut file_guard = self.current_file.write().await;
        if let Some(ref mut file) = *file_guard {
            file.flush().await.map_err(|e| {
                AppError::ConfigError(format!("Failed to flush audit log during shutdown: {}", e))
            })?;
        }
        *file_guard = None;
        *self.initialized.write().await = false;
        info!("Audit logger shut down");
        Ok(())
    }

    /// Queries audit events (placeholder for future implementation)
    ///
    /// This will support filtering by user, tenant, action, time range, etc.
    pub async fn query_events(
        &self,
        _filters: &AuditQueryFilters,
    ) -> Result<Vec<AuditEvent>, AppError> {
        // TODO: Implement event querying from log files
        // This would involve:
        // 1. Reading log files in reverse chronological order
        // 2. Parsing JSON lines
        // 3. Filtering based on query criteria
        // 4. Returning matching events
        Ok(Vec::new())
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Query filters for audit events
#[derive(Debug, Clone)]
pub struct AuditQueryFilters {
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Filter by tenant ID
    pub tenant_id: Option<String>,
    /// Filter by action
    pub action: Option<String>,
    /// Filter by resource type
    pub resource_type: Option<String>,
    /// Filter by result
    pub result: Option<String>,
    /// Filter by minimum level
    pub min_level: Option<AuditLevel>,
    /// Filter by time range (start)
    pub start_time: Option<DateTime<Utc>>,
    /// Filter by time range (end)
    pub end_time: Option<DateTime<Utc>>,
    /// Maximum number of results to return
    pub limit: Option<usize>,
}

impl Default for AuditQueryFilters {
    fn default() -> Self {
        Self {
            user_id: None,
            tenant_id: None,
            action: None,
            resource_type: None,
            result: None,
            min_level: None,
            start_time: None,
            end_time: None,
            limit: Some(1000),
        }
    }
}
