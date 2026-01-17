//! RAID / Storage module
//!
//! Provides distributed artifact storage with replication, event sourcing,
//! circuit breaker pattern, and Raft consensus for consistency.
//!
//! # Features
//!
//! - **Local Storage**: File-based artifact storage with manifest management
//! - **Distributed RAID**: Multi-node replication with quorum-based consistency
//! - **Event Sourcing**: Complete audit trail of all operations
//! - **Circuit Breaker**: Fault tolerance and automatic recovery
//! - **Raft Consensus**: Distributed consistency (optional, requires `raft` feature)
//!
//! # Examples
//!
//! ## Storing an artifact
//!
//! ```no_run
//! use poolai::raid::{RaidManager, RaidConfig, RaidMode};
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let config = RaidConfig {
//!     mode: RaidMode::Local,
//!     base_path: PathBuf::from("./data/raid"),
//!     quota_bytes: Some(10 * 1024 * 1024 * 1024), // 10 GB
//!     retention_days: Some(30),
//!     gc_on_startup: true,
//! };
//!
//! let manager = RaidManager::new(config);
//!
//! // Store an artifact
//! let artifact_ref = manager.put_artifact("my-artifact", b"artifact data").await?;
//! println!("Stored artifact: {:?}", artifact_ref);
//! # Ok(())
//! # }
//! ```
//!
//! ## Retrieving an artifact
//!
//! ```no_run
//! use poolai::raid::RaidManager;
//! use uuid::Uuid;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! # let manager = poolai::raid::RaidManager::new(
//! #     poolai::raid::RaidConfig::default_for_platform()
//! # );
//! # manager.initialize().await?;
//! let artifact = manager.put_artifact("my-artifact", b"artifact data").await?;
//!
//! let data = manager.get_artifact(&artifact.path).await?;
//! println!("Retrieved {} bytes", data.len());
//! # Ok(())
//! # }
//! ```
//!
//! Concept alignment (planned in `docs/concept/poolAI_concept.txt`):
//! - BurstRAID logic (stub)
//! - SmallWorld distributed system (stub)
//! - Administrative management (basic primitives)
//! - Artifact storage for libraries/models (local implementation)

use crate::core::error::AppError;
use crate::raid::events::{EventStore, RaidEvent};
use crate::raid::manifest::ArtifactManifest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

pub mod circuit_breaker;
pub mod client;
pub mod events;
pub mod manifest;
pub mod protocol;
#[cfg(feature = "raft")]
pub mod raft;
#[cfg(feature = "raft")]
pub mod raft_transport;
pub mod replication;

/// RAID storage mode
///
/// Defines the storage strategy used for artifact storage and replication.
///
/// # Example
///
/// ```rust
/// use poolai::raid::RaidMode;
///
/// // Use local-only storage
/// let mode = RaidMode::Local;
///
/// // Use distributed storage strategies (planned)
/// let burst_mode = RaidMode::BurstRaid;
/// let small_world_mode = RaidMode::SmallWorld;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaidMode {
    /// Local-only artifact storage (current implementation)
    ///
    /// Stores artifacts locally with manifest management and event sourcing.
    /// No distributed replication is performed.
    Local,
    /// Placeholder for "BurstRAID" strategy (planned)
    ///
    /// Distributed storage strategy optimized for burst workloads.
    BurstRaid,
    /// Placeholder for "SmallWorld" distributed strategy (planned)
    ///
    /// Distributed storage strategy using SmallWorld network topology for replication.
    SmallWorld,
}

/// Configuration for RAID storage manager
///
/// Configures storage mode, paths, quotas, and retention policies for artifact storage.
///
/// # Example
///
/// ```rust
/// use poolai::raid::{RaidConfig, RaidMode};
/// use std::path::PathBuf;
///
/// // Create custom configuration
/// let config = RaidConfig {
///     mode: RaidMode::Local,
///     base_path: PathBuf::from("./data/raid"),
///     quota_bytes: Some(20 * 1024 * 1024 * 1024), // 20 GB
///     retention_days: Some(60), // 60 days
///     gc_on_startup: true,
/// };
///
/// // Or use platform defaults
/// let default_config = RaidConfig::default_for_platform();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidConfig {
    /// Storage mode (Local, BurstRaid, or SmallWorld)
    pub mode: RaidMode,
    /// Base directory path for artifact storage
    pub base_path: PathBuf,
    /// Maximum total size of artifacts in bytes (None = unlimited)
    pub quota_bytes: Option<u64>,
    /// Retention policy: artifacts older than this will be eligible for GC (None = no retention limit)
    pub retention_days: Option<u32>,
    /// Enable automatic garbage collection on startup
    pub gc_on_startup: bool,
}

impl RaidConfig {
    pub fn default_for_platform() -> Self {
        #[cfg(target_os = "windows")]
        let base_path = PathBuf::from("C:\\poolai\\raid");
        #[cfg(target_os = "linux")]
        let base_path = PathBuf::from("/var/lib/poolai/raid");
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        let base_path = PathBuf::from("./data/raid");

        Self {
            mode: RaidMode::Local,
            base_path,
            quota_bytes: Some(10 * 1024 * 1024 * 1024), // 10 GB default
            retention_days: Some(30),                   // 30 days default
            gc_on_startup: true,
        }
    }
}

/// A RAID cluster node
///
/// Represents a single node in the distributed RAID cluster.
/// Used for tracking cluster membership and health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidNode {
    /// Unique identifier for the node
    pub id: Uuid,
    /// Network address (e.g., "192.168.1.100:8080")
    pub address: String,
    /// Last time this node was seen/heard from
    pub last_seen: DateTime<Utc>,
}

/// Reference to a stored artifact
///
/// Contains metadata about a stored artifact including its ID, name,
/// storage timestamp, and file system path.
///
/// # Example
///
/// ```rust
/// use poolai::raid::ArtifactRef;
/// use uuid::Uuid;
/// use chrono::Utc;
/// use std::path::PathBuf;
///
/// let artifact = ArtifactRef {
///     id: Uuid::new_v4(),
///     name: "my-model-v1.0.0".to_string(),
///     stored_at: Utc::now(),
///     path: PathBuf::from("./data/raid/artifacts/12345"),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// Unique identifier for the artifact
    pub id: Uuid,
    /// Human-readable name for the artifact
    pub name: String,
    /// Timestamp when the artifact was stored
    pub stored_at: DateTime<Utc>,
    /// File system path to the artifact data
    pub path: PathBuf,
}

/// RAID storage manager
///
/// Central orchestrator for artifact storage, replication, and management.
/// Supports local storage with manifest management, event sourcing, and
/// distributed replication (planned).
///
/// # Example
///
/// ```rust,no_run
/// use poolai::raid::{RaidManager, RaidConfig, RaidMode};
/// use std::path::PathBuf;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let config = RaidConfig::default_for_platform();
/// let manager = RaidManager::new(config);
/// manager.initialize().await?;
///
/// // Store an artifact
/// let artifact = manager.put_artifact("my-artifact", b"artifact data").await?;
/// println!("Stored artifact: {} at {:?}", artifact.name, artifact.path);
///
/// // Retrieve an artifact
/// let data = manager.get_artifact(&artifact.path).await?;
/// println!("Retrieved {} bytes", data.len());
/// # Ok(())
/// # }
/// ```
pub struct RaidManager {
    config: Arc<RwLock<RaidConfig>>,
    nodes: Arc<RwLock<Vec<RaidNode>>>,
    artifacts: Arc<RwLock<ArtifactManifest>>,
    /// Event store for auditability and state reconstruction
    event_store: Option<Arc<RwLock<EventStore>>>,
}

impl RaidManager {
    /// Creates a new RAID manager instance
    ///
    /// Initializes the manager with the provided configuration.
    /// The manager must be initialized with `initialize()` before use.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for storage mode, paths, quotas, and retention
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    ///
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// ```
    pub fn new(config: RaidConfig) -> Self {
        // Initialize event store if enabled (for now, always create it)
        let event_store = Some(Arc::new(RwLock::new(EventStore::new(
            config.base_path.join("events"),
        ))));

        Self {
            config: Arc::new(RwLock::new(config)),
            nodes: Arc::new(RwLock::new(Vec::new())),
            artifacts: Arc::new(RwLock::new(ArtifactManifest::new())),
            event_store,
        }
    }

    /// Initializes the RAID manager
    ///
    /// Creates necessary directories, loads manifests, and performs garbage
    /// collection if configured. Must be called before using the manager.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if initialization succeeds.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if directory creation fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// manager.initialize().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(&self) -> Result<(), AppError> {
        let cfg = self.config.read().await;
        info!("Initializing RAID manager (mode: {:?})", cfg.mode);
        tokio::fs::create_dir_all(&cfg.base_path)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to create RAID base directory: {}", e))
            })?;

        // Load manifest (if exists), prune missing files and persist.
        let manifest_path = self.manifest_path_inner(&cfg.base_path);
        if let Some(mut m) = ArtifactManifest::load(&manifest_path).await? {
            m.prune_missing_files();
            *self.artifacts.write().await = m;
            self.persist_manifest().await?;
        }

        // Run GC on startup if enabled
        if cfg.gc_on_startup {
            info!("Running GC on startup");
            self.gc_old_artifacts().await?;
        }

        // Enforce quota if configured
        if cfg.quota_bytes.is_some() {
            self.enforce_quota().await?;
        }

        // Initialize event store
        if let Some(ref event_store) = self.event_store {
            event_store.write().await.initialize().await?;
        }

        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down RAID manager");

        // Create snapshot before shutdown
        if let Some(ref event_store) = self.event_store {
            let artifacts = self.artifacts.read().await.clone();
            let nodes = self.nodes.read().await.clone();
            let _ = event_store
                .write()
                .await
                .create_snapshot(&artifacts, &nodes)
                .await;
        }

        Ok(())
    }

    /// Create a snapshot of current state
    pub async fn create_snapshot(&self) -> Result<(), AppError> {
        if let Some(ref event_store) = self.event_store {
            let artifacts = self.artifacts.read().await.clone();
            let nodes = self.nodes.read().await.clone();
            event_store
                .write()
                .await
                .create_snapshot(&artifacts, &nodes)
                .await?;
            info!("Snapshot created successfully");
        }
        Ok(())
    }

    /// Restore RAID state from a snapshot
    ///
    /// Loads a snapshot from the event store and restores the artifacts
    /// and nodes state. This will overwrite the current state.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if restore succeeds.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if snapshot loading or state restoration fails.
    pub async fn restore_from_snapshot(&self) -> Result<(), AppError> {
        if let Some(ref event_store) = self.event_store {
            let snapshot = event_store
                .read()
                .await
                .load_snapshot()
                .await?
                .ok_or_else(|| {
                    AppError::ConfigError("No snapshot available for restore".to_string())
                })?;

            info!(
                "Restoring RAID state from snapshot (sequence: {}, timestamp: {})",
                snapshot.sequence, snapshot.timestamp
            );

            // Restore artifacts from snapshot
            // snapshot.artifacts is already the JSON representation of ArtifactManifest
            match serde_json::from_value::<ArtifactManifest>(snapshot.artifacts.clone()) {
                Ok(artifacts) => {
                    *self.artifacts.write().await = artifacts;
                    self.persist_manifest().await?;
                    info!("Artifacts restored from snapshot");
                }
                Err(e) => {
                    warn!("Failed to restore artifacts from snapshot: {}. Attempting fallback parsing.", e);
                    // Fallback: try to parse as an object with "artifacts" key
                    if let Some(artifacts_obj) = snapshot.artifacts.as_object() {
                        if let Some(artifacts_array) = artifacts_obj.get("artifacts").and_then(|v| v.as_array()) {
                            let mut artifacts = ArtifactManifest::new();
                            if let Ok(artifacts_vec) = serde_json::from_value::<Vec<ArtifactRef>>(
                                serde_json::Value::Array(artifacts_array.clone())
                            ) {
                                for artifact in artifacts_vec {
                                    artifacts.artifacts.insert(artifact.id, artifact);
                                }
                                *self.artifacts.write().await = artifacts;
                                self.persist_manifest().await?;
                                info!("Artifacts restored from snapshot (fallback parsing)");
                            }
                        }
                    }
                }
            }

            // Restore nodes from snapshot
            // snapshot.nodes is already the JSON representation of Vec<RaidNode>
            match serde_json::from_value::<Vec<RaidNode>>(snapshot.nodes.clone()) {
                Ok(nodes) => {
                    *self.nodes.write().await = nodes;
                    info!("Nodes restored from snapshot");
                }
                Err(e) => {
                    warn!("Failed to restore nodes from snapshot: {}. Attempting fallback parsing.", e);
                    // Fallback: try to parse as an object with "nodes" key
                    if let Some(nodes_obj) = snapshot.nodes.as_object() {
                        if let Some(nodes_array) = nodes_obj.get("nodes").and_then(|v| v.as_array()) {
                            if let Ok(nodes_vec) = serde_json::from_value::<Vec<RaidNode>>(
                                serde_json::Value::Array(nodes_array.clone())
                            ) {
                                *self.nodes.write().await = nodes_vec;
                                info!("Nodes restored from snapshot (fallback parsing)");
                            }
                        }
                    }
                }
            }

            info!("RAID state restored from snapshot successfully");
            Ok(())
        } else {
            Err(AppError::ConfigError(
                "Event store not available for snapshot restore".to_string(),
            ))
        }
    }

    /// Get event store reference (for API access)
    ///
    /// Returns a clone of the event store Arc if event sourcing is enabled,
    /// or `None` if event sourcing is disabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use poolai::raid::RaidManager;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// # let manager = RaidManager::new(poolai::raid::RaidConfig::default_for_platform());
    /// if let Some(event_store) = manager.event_store() {
    ///     // Access event store for querying events
    ///     let events = event_store.read().await.load_events().await?;
    ///     println!("Found {} events", events.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn event_store(&self) -> Option<Arc<RwLock<EventStore>>> {
        self.event_store.clone()
    }

    /// Get current node ID from Raft (if available) or return default
    async fn get_node_id(&self) -> u64 {
        #[cfg(feature = "raft")]
        {
            // Try to get node ID from Raft storage if available
            // In a full implementation with Raft initialized, this would query the Raft node
            // For now, return default node ID (1) when Raft is not available
            // This can be extended to access global Raft instance when it's available
        }
        // Default node ID when Raft is not available or not initialized
        // In production, this would be configured per node
        1
    }

    pub async fn list_nodes(&self) -> Vec<RaidNode> {
        self.nodes.read().await.clone()
    }

    pub async fn register_node(&self, address: String) -> RaidNode {
        let node = RaidNode {
            id: Uuid::new_v4(),
            address,
            last_seen: Utc::now(),
        };
        self.nodes.write().await.push(node.clone());
        node
    }

    /// Store an artifact (library, model weights, etc.).
    ///
    /// Current implementation: local file write into `base_path/artifacts/<id>_<name>`.
    /// Stores an artifact in RAID storage
    ///
    /// Writes the artifact data to disk, updates the manifest, and records
    /// an event in the event store. Returns a reference to the stored artifact.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable name for the artifact
    /// * `bytes` - Artifact data to store
    ///
    /// # Returns
    ///
    /// Returns `ArtifactRef` containing the artifact ID, name, storage timestamp, and path.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if file creation, writing, or manifest persistence fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// manager.initialize().await?;
    ///
    /// let artifact = manager.put_artifact("my-model", b"model data").await?;
    /// println!("Stored artifact: {} at {:?}", artifact.name, artifact.path);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put_artifact(&self, name: &str, bytes: &[u8]) -> Result<ArtifactRef, AppError> {
        let cfg = self.config.read().await;
        let artifacts_dir = cfg.base_path.join("artifacts");
        tokio::fs::create_dir_all(&artifacts_dir)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create artifacts directory: {:?}. \
                    Context: The artifacts directory is required for storing RAID artifacts. \
                    Suggestion: Check filesystem permissions and ensure the base path exists and is writable. \
                    Base path: {:?}, Error: {}",
                    artifacts_dir, cfg.base_path, e
                ))
            })?;

        let id = Uuid::new_v4();
        let safe_name = sanitize_filename(name);
        let path = artifacts_dir.join(format!("{}_{}", id, safe_name));

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to create artifact file: {:?}. \
                Context: Unable to create a new file for storing the artifact. \
                Suggestion: Check filesystem permissions and available disk space. Ensure the artifacts directory exists and is writable. \
                Path: {:?}, Error: {}",
                path, path, e
            )))?;
        file.write_all(bytes)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to write artifact file: {:?}. \
                Context: Unable to write artifact data to the file. \
                Suggestion: Check available disk space and filesystem permissions. Ensure the file is not locked by another process. \
                Path: {:?}, Size: {} bytes, Error: {}",
                path, path, bytes.len(), e
            )))?;
        file.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to sync artifact file: {:?}. \
                Context: Unable to sync file data to disk. This may cause data loss if the system crashes. \
                Suggestion: Check filesystem status and available disk space. Ensure the filesystem supports sync operations. \
                Path: {:?}, Error: {}",
                path, path, e
            )))?;

        let artifact = ArtifactRef {
            id,
            name: name.to_string(),
            stored_at: Utc::now(),
            path,
        };

        // Update manifest
        {
            let mut m = self.artifacts.write().await;
            m.artifacts.insert(id, artifact.clone());
            m.updated_at = Utc::now();
        }

        // Record event
        if let Some(ref event_store) = self.event_store {
            let metadata = serde_json::json!({
                "name": name,
                "size": bytes.len(),
                "path": artifact.path.to_string_lossy(),
            });
            let _ = event_store
                .write()
                .await
                .append_event(RaidEvent::ArtifactCreated {
                    artifact_id: id.to_string(),
                    node_id: self.get_node_id().await,
                    timestamp: Utc::now(),
                    metadata,
                })
                .await;
        }
        self.persist_manifest().await?;

        Ok(artifact)
    }

    /// Reads an artifact from local storage
    ///
    /// Reads the artifact data from the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - File system path to the artifact file
    ///
    /// # Returns
    ///
    /// Returns the artifact data as a byte vector.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if the file cannot be opened or read.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    /// use std::path::Path;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// manager.initialize().await?;
    ///
    /// let artifact = manager.put_artifact("my-model", b"model data").await?;
    /// let data = manager.get_artifact(&artifact.path).await?;
    /// println!("Retrieved {} bytes", data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_artifact(&self, path: &Path) -> Result<Vec<u8>, AppError> {
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to open artifact: {}", e)))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to read artifact: {}", e)))?;
        Ok(buf)
    }

    /// Lists all stored artifacts
    ///
    /// Returns a list of all artifacts currently stored in RAID storage,
    /// including their IDs, names, storage timestamps, and paths.
    ///
    /// # Returns
    ///
    /// Returns a vector of `ArtifactRef` for all stored artifacts.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// manager.initialize().await?;
    ///
    /// let artifacts = manager.list_artifacts().await;
    /// for artifact in artifacts {
    ///     println!("Artifact: {} ({})", artifact.name, artifact.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_artifacts(&self) -> Vec<ArtifactRef> {
        self.artifacts
            .read()
            .await
            .artifacts
            .values()
            .cloned()
            .collect()
    }

    /// Deletes an artifact from storage
    ///
    /// Removes the artifact from the manifest, deletes the file from disk,
    /// and records a deletion event. The artifact is permanently removed.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the artifact to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the artifact was successfully deleted.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ConfigError` if the artifact doesn't exist or file deletion fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::raid::{RaidManager, RaidConfig};
    /// use uuid::Uuid;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let config = RaidConfig::default_for_platform();
    /// let manager = RaidManager::new(config);
    /// manager.initialize().await?;
    ///
    /// let artifact = manager.put_artifact("my-model", b"model data").await?;
    /// manager.delete_artifact(artifact.id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_artifact(&self, id: Uuid) -> Result<(), AppError> {
        let artifact = {
            let mut m = self.artifacts.write().await;
            let a = m
                .artifacts
                .remove(&id)
                .ok_or_else(|| AppError::ConfigError(format!("Artifact {} not found", id)))?;
            m.updated_at = Utc::now();
            a
        };

        if artifact.path.exists() {
            tokio::fs::remove_file(&artifact.path).await.map_err(|e| {
                AppError::ConfigError(format!("Failed to remove artifact file: {}", e))
            })?;
        }

        // Record event
        if let Some(ref event_store) = self.event_store {
            let _ = event_store
                .write()
                .await
                .append_event(RaidEvent::ArtifactDeleted {
                    artifact_id: id.to_string(),
                    node_id: self.get_node_id().await,
                    timestamp: Utc::now(),
                })
                .await;
        }

        self.persist_manifest().await?;
        Ok(())
    }

    /// Garbage collection: remove old artifacts based on retention policy
    pub async fn gc_old_artifacts(&self) -> Result<usize, AppError> {
        let cfg = self.config.read().await;
        let retention_days = match cfg.retention_days {
            Some(days) => days,
            None => {
                info!("GC skipped: no retention policy configured");
                return Ok(0);
            }
        };

        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        let mut removed = 0;

        let artifacts_to_remove: Vec<Uuid> = {
            let artifacts = self.artifacts.read().await;
            artifacts
                .artifacts
                .values()
                .filter(|a| a.stored_at < cutoff)
                .map(|a| a.id)
                .collect()
        };

        for id in artifacts_to_remove {
            if let Err(e) = self.delete_artifact(id).await {
                warn!("Failed to delete artifact {} during GC: {}", id, e);
            } else {
                removed += 1;
            }
        }

        if removed > 0 {
            info!(
                "GC removed {} old artifacts (retention: {} days)",
                removed, retention_days
            );
        }
        Ok(removed)
    }

    /// Enforce quota: remove oldest artifacts if quota is exceeded
    pub async fn enforce_quota(&self) -> Result<usize, AppError> {
        let quota_bytes = {
            let cfg = self.config.read().await;
            match cfg.quota_bytes {
                Some(quota) => quota,
                None => {
                    info!("Quota enforcement skipped: no quota configured");
                    return Ok(0);
                }
            }
        };

        let total_size = self.get_total_size().await?;
        if total_size <= quota_bytes {
            info!("Quota OK: {} / {} bytes", total_size, quota_bytes);
            return Ok(0);
        }

        let excess = total_size - quota_bytes;
        info!(
            "Quota exceeded: {} / {} bytes (excess: {} bytes)",
            total_size, quota_bytes, excess
        );

        // Sort artifacts by age (oldest first) and remove until quota is met
        let mut artifacts_by_age: Vec<(Uuid, DateTime<Utc>, u64)> = {
            let artifacts = self.artifacts.read().await;
            artifacts
                .artifacts
                .values()
                .filter_map(|a| {
                    if a.path.exists() {
                        if let Ok(metadata) = std::fs::metadata(&a.path) {
                            Some((a.id, a.stored_at, metadata.len()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        };

        artifacts_by_age.sort_by_key(|(_, stored_at, _)| *stored_at);

        let mut removed = 0;
        let mut freed_bytes = 0u64;

        for (id, _, size) in artifacts_by_age {
            if freed_bytes >= excess {
                break;
            }

            if let Err(e) = self.delete_artifact(id).await {
                warn!(
                    "Failed to delete artifact {} during quota enforcement: {}",
                    id, e
                );
            } else {
                removed += 1;
                freed_bytes += size;
            }
        }

        if removed > 0 {
            info!(
                "Quota enforcement removed {} artifacts, freed {} bytes",
                removed, freed_bytes
            );
        }
        Ok(removed)
    }

    /// Get total size of all artifacts in bytes
    pub async fn get_total_size(&self) -> Result<u64, AppError> {
        let artifacts = self.artifacts.read().await;
        let mut total = 0u64;

        for artifact in artifacts.artifacts.values() {
            if artifact.path.exists() {
                match std::fs::metadata(&artifact.path) {
                    Ok(metadata) => total += metadata.len(),
                    Err(e) => {
                        warn!("Failed to get metadata for artifact {}: {}", artifact.id, e);
                    }
                }
            }
        }

        Ok(total)
    }

    /// Get quota bytes from configuration
    pub async fn get_quota_bytes(&self) -> Option<u64> {
        let config = self.config.read().await;
        config.quota_bytes
    }

    /// Placeholder: rebalancing would run for distributed modes.
    #[allow(dead_code)]
    pub async fn rebalance(&self) -> Result<(), AppError> {
        Ok(())
    }

    fn manifest_path_inner(&self, base: &Path) -> PathBuf {
        base.join("artifacts").join("manifest.json")
    }

    async fn persist_manifest(&self) -> Result<(), AppError> {
        let base = self.config.read().await.base_path.clone();
        let path = self.manifest_path_inner(&base);
        let m = self.artifacts.read().await.clone();
        m.save_atomic(&path).await
    }
}

fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

static RAID_MANAGER: OnceLock<Arc<RaidManager>> = OnceLock::new();

/// Get global RAID manager instance.
///
/// This function returns a singleton instance of `RaidManager` that can be used
/// throughout the application. The instance is created on first access with
/// default platform-specific configuration and reused for subsequent calls.
///
/// # Examples
///
/// ```no_run
/// use poolai::raid::get_global_manager;
/// use uuid::Uuid;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let manager = get_global_manager();
///
/// // List all artifacts
/// let artifacts = manager.list_artifacts().await;
/// for artifact in artifacts {
///     println!("Artifact: {} ({})", artifact.name, artifact.id);
/// }
/// # Ok(())
/// # }
/// ```
pub fn get_global_manager() -> Arc<RaidManager> {
    RAID_MANAGER
        .get_or_init(|| Arc::new(RaidManager::new(RaidConfig::default_for_platform())))
        .clone()
}

/// Initialize the RAID module.
///
/// This function initializes the global RAID manager instance, including:
/// - Creating base directories
/// - Loading artifact manifest
/// - Running garbage collection (if enabled)
/// - Enforcing quota (if configured)
/// - Initializing event store
///
/// # Examples
///
/// ```no_run
/// use poolai::raid::initialize;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// // Initialize RAID module at application startup
/// initialize().await?;
/// println!("RAID module initialized");
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns `AppError::ConfigError` if initialization fails (e.g., directory creation, manifest loading).
pub async fn initialize() -> Result<(), AppError> {
    get_global_manager().initialize().await
}

/// Shutdown the RAID module.
///
/// This function gracefully shuts down the global RAID manager instance, including:
/// - Creating a final snapshot of current state
/// - Persisting manifest changes
/// - Cleaning up resources
///
/// # Examples
///
/// ```no_run
/// use poolai::raid::shutdown;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// // Shutdown RAID module at application exit
/// shutdown().await?;
/// println!("RAID module shut down");
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns `AppError::ConfigError` if shutdown fails (e.g., snapshot creation).
pub async fn shutdown() -> Result<(), AppError> {
    get_global_manager().shutdown().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raid_mode_variants() {
        let modes = vec![RaidMode::Local, RaidMode::BurstRaid, RaidMode::SmallWorld];
        for mode in modes {
            let cloned = mode.clone();
            assert!(matches!(
                cloned,
                RaidMode::Local | RaidMode::BurstRaid | RaidMode::SmallWorld
            ));
        }
    }

    #[test]
    fn test_raid_config_default_for_platform() {
        let config = RaidConfig::default_for_platform();
        assert!(matches!(config.mode, RaidMode::Local));
        assert!(config.quota_bytes.is_some());
        assert!(config.retention_days.is_some());
        assert!(config.gc_on_startup);
    }

    #[test]
    fn test_raid_config_clone() {
        let config = RaidConfig {
            mode: RaidMode::Local,
            base_path: PathBuf::from("./test"),
            quota_bytes: Some(1000),
            retention_days: Some(7),
            gc_on_startup: false,
        };
        let cloned = config.clone();
        assert!(matches!(config.mode, RaidMode::Local));
        assert!(matches!(cloned.mode, RaidMode::Local));
        assert_eq!(config.quota_bytes, cloned.quota_bytes);
        assert_eq!(config.retention_days, cloned.retention_days);
    }

    #[test]
    fn test_artifact_ref_fields() {
        let artifact = ArtifactRef {
            id: Uuid::new_v4(),
            name: "test-artifact".to_string(),
            stored_at: Utc::now(),
            path: PathBuf::from("./test/path"),
        };
        assert_eq!(artifact.name, "test-artifact");
        assert!(!artifact.path.as_os_str().is_empty());
    }

    #[test]
    fn test_raid_node_fields() {
        let node = RaidNode {
            id: Uuid::new_v4(),
            address: "127.0.0.1:8080".to_string(),
            last_seen: Utc::now(),
        };
        assert_eq!(node.address, "127.0.0.1:8080");
        assert!(!node.id.to_string().is_empty());
    }

    #[tokio::test]
    async fn test_raid_manager_new() {
        let config = RaidConfig::default_for_platform();
        let manager = RaidManager::new(config);
        // Manager should be created successfully
        assert!(manager.list_artifacts().await.is_empty());
    }

    #[tokio::test]
    async fn test_raid_manager_list_artifacts_empty() {
        let config = RaidConfig {
            mode: RaidMode::Local,
            base_path: PathBuf::from("./test_raid_empty"),
            quota_bytes: Some(1000),
            retention_days: Some(7),
            gc_on_startup: false,
        };
        let manager = RaidManager::new(config);
        let artifacts = manager.list_artifacts().await;
        assert_eq!(artifacts.len(), 0);
    }
}
