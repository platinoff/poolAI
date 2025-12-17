//! RAID / Storage module
//!
//! Concept alignment (planned in `poolAI_concept.txt`):
//! - BurstRAID logic (stub)
//! - SmallWorld distributed system (stub)
//! - Administrative management (basic primitives)
//! - Artifact storage for libraries/models (local implementation)

use crate::core::error::AppError;
use crate::raid::manifest::ArtifactManifest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

pub mod manifest;

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
}

impl RaidManager {
    pub fn new(config: RaidConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            nodes: Arc::new(RwLock::new(Vec::new())),
            artifacts: Arc::new(RwLock::new(ArtifactManifest::new())),
        }
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        let cfg = self.config.read().await;
        info!("Initializing RAID manager (mode: {:?})", cfg.mode);
        tokio::fs::create_dir_all(&cfg.base_path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to create RAID base directory: {}", e)))?;

        // Load manifest (if exists), prune missing files and persist.
        let manifest_path = self.manifest_path_inner(&cfg.base_path);
        if let Some(mut m) = ArtifactManifest::load(&manifest_path).await? {
            m.prune_missing_files();
            *self.artifacts.write().await = m;
            self.persist_manifest().await?;
        }
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down RAID manager");
        Ok(())
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
            .map_err(|e| AppError::ConfigError(format!("Failed to create artifacts directory: {}", e)))?;

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
            tokio::fs::remove_file(&artifact.path)
                .await
                .map_err(|e| AppError::ConfigError(format!("Failed to remove artifact file: {}", e)))?;
        }
        self.persist_manifest().await?;
        Ok(())
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


