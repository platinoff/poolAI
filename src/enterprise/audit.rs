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
use std::sync::{Arc, OnceLock};
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Env key for audit store backend (`file` default; `sqlite` horizon band 72+).
/// Band 71 scaffold — see [`docs/development/AUDIT_DEPTH.md`].
/// Band 72 store wire — see [`docs/development/AUDIT_STORE.md`].
pub const POOLAI_AUDIT_STORE: &str = "POOLAI_AUDIT_STORE";

/// Env key for durable audit data directory (band 72 store wire).
pub const POOLAI_AUDIT_DATA_DIR: &str = "POOLAI_AUDIT_DATA_DIR";

/// Canonical sqlite DB file name under the configured data dir (band 72 wire).
pub const AUDIT_STORE_SQLITE_FILE: &str = "audit.sqlite";

/// Canonical audit store modes (band 71 scaffold).
pub const AUDIT_STORE_MODES: &[&str] = &["file", "sqlite"];

/// Resolve configured audit store mode (PH-S1350 scaffold; durable wire PH-S1360).
pub fn audit_store_mode() -> &'static str {
    match std::env::var(POOLAI_AUDIT_STORE)
        .unwrap_or_else(|_| "file".into())
        .to_ascii_lowercase()
        .as_str()
    {
        "sqlite" => "sqlite",
        _ => "file",
    }
}

/// Optional durable data directory from env (PH-S1360).
pub fn audit_store_data_dir_from_env() -> Option<PathBuf> {
    std::env::var(POOLAI_AUDIT_DATA_DIR)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// Audit store wire snapshot (mode + durable path; CRUD later in phase C).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditStoreWire {
    /// `file` or `sqlite`.
    pub mode: String,
    /// Resolved sqlite file path when mode is sqlite and data dir is set.
    pub durable_path: Option<String>,
    /// True when sqlite mode has a durable path.
    pub configured: bool,
}

/// Resolve audit store wire for ops / verify / contracts (PH-S1360).
pub fn audit_store_wire() -> AuditStoreWire {
    let mode = audit_store_mode().to_string();
    if mode != "sqlite" {
        return AuditStoreWire {
            mode,
            durable_path: None,
            configured: false,
        };
    }
    let durable_path = audit_store_data_dir_from_env().map(|dir| {
        dir.join(AUDIT_STORE_SQLITE_FILE)
            .to_string_lossy()
            .replace('\\', "/")
    });
    let configured = durable_path.is_some();
    AuditStoreWire {
        mode,
        durable_path,
        configured,
    }
}

/// Wire label for admin / metrics depth fields (PH-S1360).
pub fn audit_store_wire_label(wire: &AuditStoreWire) -> &'static str {
    if wire.mode == "sqlite" && wire.configured {
        "sqlite"
    } else if wire.mode == "sqlite" {
        "sqlite_unconfigured"
    } else {
        "file"
    }
}

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
    pub fn new(level: AuditLevel, action: String, resource_type: String, result: String) -> Self {
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

/// PH-S1350 production-verify stub: require non-empty `action` + `resource_type`.
pub fn validate_audit_event_fields(event: &AuditEvent) -> Result<(), AppError> {
    if event.action.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Audit event missing action. Context: production verify stub requires action. \
            Suggestion: set a non-empty action string (e.g. create_instance)."
                .to_string(),
        ));
    }
    if event.resource_type.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Audit event missing resource_type. Context: production verify stub requires resource_type. \
            Suggestion: set a non-empty resource_type string (e.g. vm_instance)."
                .to_string(),
        ));
    }
    Ok(())
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
        tokio::fs::create_dir_all(&self.config.log_directory)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create audit log directory: {}. \
                Context: Cannot initialize audit logging without a valid directory. \
                Suggestion: Ensure write permissions for the parent directory. \
                Path: {:?}, Error: {}",
                    e, self.config.log_directory, e
                ))
            })?;

        // Open initial log file
        self.rotate_log_file().await?;

        *initialized = true;
        info!(
            "Audit logger initialized with directory: {:?}",
            self.config.log_directory
        );
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
                AppError::ConfigError(format!("Failed to flush audit log during rotation: {}", e))
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

        // Compress old log files if enabled (before cleanup to compress recent files)
        if self.config.enable_compression {
            self.compress_old_logs().await?;
        }

        // Cleanup old log files (keep only max_files)
        self.cleanup_old_logs().await?;

        Ok(())
    }

    /// Compresses old log files using gzip compression
    ///
    /// Compresses `.log` files to `.log.gz` and removes original files.
    /// Only compresses files that are not already compressed.
    async fn compress_old_logs(&self) -> Result<(), AppError> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::fs::File;

        let mut entries = tokio::fs::read_dir(&self.config.log_directory)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to read audit log directory for compression: {}",
                    e
                ))
            })?;

        let mut log_files = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to read directory entry during compression: {}",
                e
            ))
        })? {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Only compress .log files (not .log.gz files)
                    if file_name.starts_with("audit_")
                        && file_name.ends_with(".log")
                        && !file_name.ends_with(".log.gz")
                    {
                        log_files.push(path);
                    }
                }
            }
        }

        // Compress each log file
        for log_path in log_files {
            let gz_path = log_path.with_extension("log.gz");

            // Skip if compressed file already exists
            if gz_path.exists() {
                warn!("Compressed file already exists: {:?}, skipping", gz_path);
                continue;
            }

            // Use blocking I/O for compression (flate2 is synchronous)
            let log_path_clone = log_path.clone();
            let gz_path_clone = gz_path.clone();
            let compress_result = tokio::task::spawn_blocking(move || -> Result<(), AppError> {
                // Read log file content
                let mut log_reader = File::open(&log_path_clone).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to open log file for compression (blocking): {:?}, Error: {}",
                        log_path_clone, e
                    ))
                })?;

                // Compress directly from file to gzip file (streaming)
                let gz_file = File::create(&gz_path_clone).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to create compressed file: {:?}, Error: {}",
                        gz_path_clone, e
                    ))
                })?;

                let mut gz_writer = GzEncoder::new(gz_file, Compression::default());
                std::io::copy(&mut log_reader, &mut gz_writer).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to compress log file: {:?}, Error: {}",
                        log_path_clone, e
                    ))
                })?;

                // Finish compression (flushes and closes GzEncoder)
                gz_writer.finish().map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to finish compression: {:?}, Error: {}",
                        log_path_clone, e
                    ))
                })?;

                Ok(())
            })
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Compression task failed: {:?}, Error: {}",
                    log_path, e
                ))
            })?;

            match compress_result {
                Ok(_) => {
                    // Delete original log file after successful compression
                    if let Err(e) = tokio::fs::remove_file(&log_path).await {
                        warn!(
                            "Failed to delete original log file after compression: {:?}, Error: {}",
                            log_path, e
                        );
                    } else {
                        info!("Compressed audit log file: {:?} -> {:?}", log_path, gz_path);
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to compress audit log file: {:?}, Error: {}",
                        log_path, e
                    );
                }
            }
        }

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

    /// Queries audit events from log files
    ///
    /// Reads all log files, parses JSON events, and filters based on query criteria.
    /// Returns events in reverse chronological order (newest first).
    ///
    /// # Errors
    ///
    /// Returns `AppError` if reading or parsing log files fails.
    pub async fn query_events(
        &self,
        filters: &AuditQueryFilters,
    ) -> Result<Vec<AuditEvent>, AppError> {
        let initialized = self.initialized.read().await;
        if !*initialized {
            return Err(AppError::ConfigError(
                "Audit logger not initialized. Call initialize() first.".to_string(),
            ));
        }
        drop(initialized);

        // Get all log files in the directory
        let mut entries = tokio::fs::read_dir(&self.config.log_directory)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to read audit log directory: {}. \
                Context: Cannot query audit events without access to log directory. \
                Suggestion: Check directory permissions and path. \
                Path: {:?}, Error: {}",
                    e, self.config.log_directory, e
                ))
            })?;

        let mut log_files = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to read directory entry: {}. \
                Context: Cannot enumerate log files for querying. \
                Error: {}",
                e, e
            ))
        })? {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("audit_") {
                        log_files.push(path);
                    }
                }
            }
        }

        // Sort by filename (which includes timestamp) in reverse order (newest first)
        log_files.sort_by(|a, b| {
            b.file_name()
                .and_then(|n| n.to_str())
                .cmp(&a.file_name().and_then(|n| n.to_str()))
        });

        let mut all_events = Vec::new();
        let limit = filters.limit.unwrap_or(1000);

        // Read events from each log file
        for log_file in log_files {
            if all_events.len() >= limit {
                break;
            }

            match self
                .read_events_from_file(&log_file, filters, limit - all_events.len())
                .await
            {
                Ok(mut events) => {
                    all_events.append(&mut events);
                }
                Err(e) => {
                    warn!("Failed to read events from {:?}: {}", log_file, e);
                    // Continue with other files
                }
            }
        }

        // Sort by timestamp (newest first) and apply limit
        all_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        all_events.truncate(limit);

        Ok(all_events)
    }

    /// Reads events from a single log file and applies filters
    async fn read_events_from_file(
        &self,
        file_path: &std::path::Path,
        filters: &AuditQueryFilters,
        max_events: usize,
    ) -> Result<Vec<AuditEvent>, AppError> {
        let contents = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to read audit log file: {}. \
                Context: Cannot query events from log file. \
                Suggestion: Check file permissions. \
                Path: {:?}, Error: {}",
                e, file_path, e
            ))
        })?;

        let mut events = Vec::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if events.len() >= max_events {
                break;
            }

            let event: AuditEvent = match serde_json::from_str(line) {
                Ok(e) => e,
                Err(e) => {
                    warn!(
                        "Failed to parse audit event from line: {}. Error: {}",
                        line, e
                    );
                    continue;
                }
            };

            // Apply filters
            if self.event_matches_filters(&event, filters) {
                events.push(event);
            }
        }

        Ok(events)
    }

    /// Checks if an event matches the query filters
    fn event_matches_filters(&self, event: &AuditEvent, filters: &AuditQueryFilters) -> bool {
        // Filter by user ID
        if let Some(ref user_id) = filters.user_id {
            if event.user_id.as_ref() != Some(user_id) {
                return false;
            }
        }

        // Filter by tenant ID
        if let Some(ref tenant_id) = filters.tenant_id {
            if event.tenant_id.as_ref() != Some(tenant_id) {
                return false;
            }
        }

        // Filter by action
        if let Some(ref action) = filters.action {
            if event.action != *action {
                return false;
            }
        }

        // Filter by resource type
        if let Some(ref resource_type) = filters.resource_type {
            if event.resource_type != *resource_type {
                return false;
            }
        }

        // Filter by result
        if let Some(ref result) = filters.result {
            if event.result != *result {
                return false;
            }
        }

        // Filter by minimum level
        if let Some(min_level) = filters.min_level {
            let event_level_priority = match event.level {
                AuditLevel::Info => 0,
                AuditLevel::Warning => 1,
                AuditLevel::Error => 2,
                AuditLevel::Critical => 3,
            };
            let min_level_priority = match min_level {
                AuditLevel::Info => 0,
                AuditLevel::Warning => 1,
                AuditLevel::Error => 2,
                AuditLevel::Critical => 3,
            };
            if event_level_priority < min_level_priority {
                return false;
            }
        }

        // Filter by time range
        if let Some(start_time) = filters.start_time {
            if event.timestamp < start_time {
                return false;
            }
        }

        if let Some(end_time) = filters.end_time {
            if event.timestamp > end_time {
                return false;
            }
        }

        true
    }

    /// Cleans up old log files, keeping only the most recent max_files
    async fn cleanup_old_logs(&self) -> Result<(), AppError> {
        let mut entries = tokio::fs::read_dir(&self.config.log_directory)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to read audit log directory for cleanup: {}",
                    e
                ))
            })?;

        let mut log_files = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to read directory entry during cleanup: {}",
                e
            ))
        })? {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Include both .log and .log.gz files in cleanup
                    if file_name.starts_with("audit_")
                        && (file_name.ends_with(".log") || file_name.ends_with(".log.gz"))
                    {
                        if let Ok(metadata) = entry.metadata().await {
                            if let Ok(modified) = metadata.modified() {
                                log_files.push((path, modified));
                            }
                        }
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        log_files.sort_by(|a, b| b.1.cmp(&a.1));

        // Delete old files beyond max_files
        if log_files.len() > self.config.max_files {
            for (path, _) in log_files.iter().skip(self.config.max_files) {
                if let Err(e) = tokio::fs::remove_file(path).await {
                    warn!("Failed to delete old audit log file {:?}: {}", path, e);
                } else {
                    info!("Deleted old audit log file: {:?}", path);
                }
            }
        }

        Ok(())
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

/// Global audit logger instance
static AUDIT_LOGGER: OnceLock<Arc<AuditLogger>> = OnceLock::new();

/// Align `get_global_audit_logger` with [`crate::core::state::AppState::audit_logger`] when possible.
pub fn try_install_global_audit_logger(logger: Arc<AuditLogger>) {
    let _ = AUDIT_LOGGER.set(logger);
}

/// Get global audit logger instance.
///
/// This function returns a singleton instance of `AuditLogger` that can be used
/// throughout the application. The instance is created on first access and
/// reused for subsequent calls.
///
/// # Examples
///
/// ```no_run
/// use poolai::enterprise::audit::get_global_audit_logger;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let logger = get_global_audit_logger();
/// logger.initialize().await?;
///
/// // Log an event
/// logger.log_event(poolai::enterprise::audit::AuditEvent::new(
///     poolai::enterprise::audit::AuditLevel::Info,
///     "test_action".to_string(),
///     "test_resource".to_string(),
///     "success".to_string(),
/// )).await?;
/// # Ok(())
/// # }
/// ```
pub fn get_global_audit_logger() -> Arc<AuditLogger> {
    AUDIT_LOGGER
        .get_or_init(|| Arc::new(AuditLogger::new()))
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_audit_logger_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let config = AuditConfig {
            log_directory: temp_dir.path().to_path_buf(),
            max_file_size: 1024,
            max_files: 5,
            enable_compression: false,
            immediate_flush: false,
        };

        let logger = AuditLogger::with_config(config);
        assert!(logger.initialize().await.is_ok());
        assert!(logger.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_audit_log_event() {
        let temp_dir = TempDir::new().unwrap();
        let config = AuditConfig {
            log_directory: temp_dir.path().to_path_buf(),
            max_file_size: 1024 * 1024, // 1 MB
            max_files: 5,
            enable_compression: false,
            immediate_flush: false,
        };

        let logger = AuditLogger::with_config(config);
        logger.initialize().await.unwrap();

        let event = AuditEvent::new(
            AuditLevel::Info,
            "test_action".to_string(),
            "test_resource".to_string(),
            "success".to_string(),
        )
        .with_user_id("user123".to_string())
        .with_tenant_id("tenant-abc".to_string());

        assert!(logger.log_event(event).await.is_ok());
        assert!(logger.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_audit_query_events() {
        let temp_dir = TempDir::new().unwrap();
        let config = AuditConfig {
            log_directory: temp_dir.path().to_path_buf(),
            max_file_size: 1024 * 1024,
            max_files: 5,
            enable_compression: false,
            immediate_flush: false,
        };

        let logger = AuditLogger::with_config(config);
        logger.initialize().await.unwrap();

        // Log some events
        let event1 = AuditEvent::new(
            AuditLevel::Info,
            "create_instance".to_string(),
            "vm_instance".to_string(),
            "success".to_string(),
        )
        .with_user_id("user123".to_string());

        let event2 = AuditEvent::new(
            AuditLevel::Warning,
            "delete_instance".to_string(),
            "vm_instance".to_string(),
            "success".to_string(),
        )
        .with_user_id("user456".to_string());

        logger.log_event(event1).await.unwrap();
        logger.log_event(event2).await.unwrap();

        // Query all events
        let filters = AuditQueryFilters::default();
        let events = logger.query_events(&filters).await.unwrap();
        assert!(events.len() >= 2);

        // Query by user ID
        let filters = AuditQueryFilters {
            user_id: Some("user123".to_string()),
            ..Default::default()
        };
        let events = logger.query_events(&filters).await.unwrap();
        assert!(events
            .iter()
            .all(|e| e.user_id.as_ref() == Some(&"user123".to_string())));

        // Query by action
        let filters = AuditQueryFilters {
            action: Some("create_instance".to_string()),
            ..Default::default()
        };
        let events = logger.query_events(&filters).await.unwrap();
        assert!(events.iter().all(|e| e.action == "create_instance"));

        // Query by minimum level
        let filters = AuditQueryFilters {
            min_level: Some(AuditLevel::Warning),
            ..Default::default()
        };
        let events = logger.query_events(&filters).await.unwrap();
        assert!(events.iter().all(|e| {
            matches!(
                e.level,
                AuditLevel::Warning | AuditLevel::Error | AuditLevel::Critical
            )
        }));

        assert!(logger.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_audit_event_builder() {
        let event = AuditEvent::new(
            AuditLevel::Error,
            "test_action".to_string(),
            "test_resource".to_string(),
            "failure".to_string(),
        )
        .with_user_id("user123".to_string())
        .with_tenant_id("tenant-abc".to_string())
        .with_resource_id("resource-456".to_string())
        .with_metadata("key1".to_string(), "value1".to_string());

        assert_eq!(event.level, AuditLevel::Error);
        assert_eq!(event.action, "test_action");
        assert_eq!(event.user_id, Some("user123".to_string()));
        assert_eq!(event.tenant_id, Some("tenant-abc".to_string()));
        assert_eq!(event.resource_id, Some("resource-456".to_string()));
        assert_eq!(event.metadata.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn audit_store_mode_defaults_file_ph_s1350() {
        std::env::remove_var(POOLAI_AUDIT_STORE);
        assert_eq!(audit_store_mode(), "file");
        std::env::set_var(POOLAI_AUDIT_STORE, "sqlite");
        assert_eq!(audit_store_mode(), "sqlite");
        std::env::remove_var(POOLAI_AUDIT_STORE);
        assert!(AUDIT_STORE_MODES.contains(&"file"));
        assert_eq!(POOLAI_AUDIT_DATA_DIR, "POOLAI_AUDIT_DATA_DIR");
    }

    #[test]
    fn audit_store_wire_file_default_ph_s1360() {
        std::env::remove_var(POOLAI_AUDIT_STORE);
        std::env::remove_var(POOLAI_AUDIT_DATA_DIR);
        let wire = audit_store_wire();
        assert_eq!(wire.mode, "file");
        assert!(!wire.configured);
        assert!(wire.durable_path.is_none());
        assert_eq!(audit_store_wire_label(&wire), "file");
        assert_eq!(POOLAI_AUDIT_DATA_DIR, "POOLAI_AUDIT_DATA_DIR");
        assert_eq!(AUDIT_STORE_SQLITE_FILE, "audit.sqlite");
    }

    #[test]
    fn audit_store_wire_sqlite_unconfigured_ph_s1360() {
        std::env::set_var(POOLAI_AUDIT_STORE, "sqlite");
        std::env::remove_var(POOLAI_AUDIT_DATA_DIR);
        let wire = audit_store_wire();
        assert_eq!(wire.mode, "sqlite");
        assert!(!wire.configured);
        assert_eq!(audit_store_wire_label(&wire), "sqlite_unconfigured");
        std::env::remove_var(POOLAI_AUDIT_STORE);
    }

    #[test]
    fn audit_store_wire_sqlite_configured_ph_s1360() {
        std::env::set_var(POOLAI_AUDIT_STORE, "sqlite");
        std::env::set_var(POOLAI_AUDIT_DATA_DIR, "/tmp/poolai-audit-wire");
        let wire = audit_store_wire();
        assert!(wire.configured);
        assert_eq!(audit_store_wire_label(&wire), "sqlite");
        let path = wire.durable_path.as_deref().expect("path");
        assert!(path.ends_with(AUDIT_STORE_SQLITE_FILE) || path.contains(AUDIT_STORE_SQLITE_FILE));
        std::env::remove_var(POOLAI_AUDIT_STORE);
        std::env::remove_var(POOLAI_AUDIT_DATA_DIR);
    }

    #[test]
    fn validate_audit_event_fields_ok_ph_s1350() {
        let event = AuditEvent::new(
            AuditLevel::Info,
            "create_instance".to_string(),
            "vm_instance".to_string(),
            "success".to_string(),
        );
        assert!(validate_audit_event_fields(&event).is_ok());
    }

    #[test]
    fn validate_audit_event_fields_rejects_empty_action_ph_s1350() {
        let event = AuditEvent::new(
            AuditLevel::Info,
            "  ".to_string(),
            "vm_instance".to_string(),
            "success".to_string(),
        );
        let err = validate_audit_event_fields(&event).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("missing action"), "{msg}");
    }

    #[test]
    fn validate_audit_event_fields_rejects_empty_resource_ph_s1350() {
        let event = AuditEvent::new(
            AuditLevel::Info,
            "create_instance".to_string(),
            "".to_string(),
            "success".to_string(),
        );
        let err = validate_audit_event_fields(&event).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("missing resource_type"), "{msg}");
    }
}
