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
//! let manager = RaidManager::new(config).await?;
//!
//! // Store an artifact
//! let artifact_id = manager.store_artifact("my-artifact", b"artifact data").await?;
//! println!("Stored artifact: {:?}", artifact_id);
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
//! # ).await?;
//! let artifact_id = Uuid::new_v4();
//!
//! let data = manager.get_artifact(artifact_id).await?;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaidMode {
    /// Local-only artifact storage (current implementation).
    Local,
    /// Placeholder for "BurstRAID" strategy.
    BurstRaid,
    /// Placeholder for "SmallWorld" distributed strategy.
    SmallWorld,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidConfig {
    pub mode: RaidMode,
    pub base_path: PathBuf,
    /// Maximum total size of artifacts in bytes (None = unlimited)
    pub quota_bytes: Option<u64>,
    /// Retention policy: artifacts older than this will be eligible for GC
    pub retention_days: Option<u32>,
    /// Enable automatic GC on startup
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidNode {
    pub id: Uuid,
    pub address: String,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub id: Uuid,
    pub name: String,
    pub stored_at: DateTime<Utc>,
    pub path: PathBuf,
}

pub struct RaidManager {
    config: Arc<RwLock<RaidConfig>>,
    nodes: Arc<RwLock<Vec<RaidNode>>>,
    artifacts: Arc<RwLock<ArtifactManifest>>,
    /// Event store for auditability and state reconstruction
    event_store: Option<Arc<RwLock<EventStore>>>,
}

impl RaidManager {
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

    /// Get event store reference (for API access)
    pub fn event_store(&self) -> Option<Arc<RwLock<EventStore>>> {
        self.event_store.clone()
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
    pub async fn put_artifact(&self, name: &str, bytes: &[u8]) -> Result<ArtifactRef, AppError> {
        let cfg = self.config.read().await;
        let artifacts_dir = cfg.base_path.join("artifacts");
        tokio::fs::create_dir_all(&artifacts_dir)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to create artifacts directory: {}", e))
            })?;

        let id = Uuid::new_v4();
        let safe_name = sanitize_filename(name);
        let path = artifacts_dir.join(format!("{}_{}", id, safe_name));

        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to create artifact file: {}", e)))?;
        file.write_all(bytes)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write artifact file: {}", e)))?;
        file.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to sync artifact file: {}", e)))?;

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
                    node_id: 0, // TODO: Get actual node ID from Raft
                    timestamp: Utc::now(),
                    metadata,
                })
                .await;
        }
        self.persist_manifest().await?;

        Ok(artifact)
    }

    /// Read an artifact from local storage.
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

    pub async fn list_artifacts(&self) -> Vec<ArtifactRef> {
        self.artifacts
            .read()
            .await
            .artifacts
            .values()
            .cloned()
            .collect()
    }

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

pub fn get_global_manager() -> Arc<RaidManager> {
    RAID_MANAGER
        .get_or_init(|| Arc::new(RaidManager::new(RaidConfig::default_for_platform())))
        .clone()
}

pub async fn initialize() -> Result<(), AppError> {
    get_global_manager().initialize().await
}

pub async fn shutdown() -> Result<(), AppError> {
    get_global_manager().shutdown().await
}
