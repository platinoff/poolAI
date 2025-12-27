//! RAID artifact manifest (local persistent index)
//!
//! Goal: keep a durable list of stored artifacts for UI/API, GC and quota management.

use crate::core::error::AppError;
use crate::raid::ArtifactRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub schema_version: u32,
    pub updated_at: DateTime<Utc>,
    pub artifacts: HashMap<Uuid, ArtifactRef>,
}

impl ArtifactManifest {
    pub fn new() -> Self {
        Self {
            schema_version: 1,
            updated_at: Utc::now(),
            artifacts: HashMap::new(),
        }
    }

    pub async fn load(path: &Path) -> Result<Option<Self>, AppError> {
        match tokio::fs::read(path).await {
            Ok(bytes) => {
                let m: Self = serde_json::from_slice(&bytes).map_err(|e| {
                    AppError::ConfigError(format!("Failed to parse artifact manifest: {}", e))
                })?;
                Ok(Some(m))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AppError::ConfigError(format!(
                "Failed to read artifact manifest: {}",
                e
            ))),
        }
    }

    pub async fn save_atomic(&self, path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                AppError::ConfigError(format!("Failed to create manifest dir: {}", e))
            })?;
        }

        let tmp = tmp_path(path);
        let data = serde_json::to_vec_pretty(self).map_err(|e| {
            AppError::ConfigError(format!("Failed to serialize artifact manifest: {}", e))
        })?;

        let mut f = tokio::fs::File::create(&tmp)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to create tmp manifest: {}", e)))?;
        f.write_all(&data)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write tmp manifest: {}", e)))?;
        f.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to sync tmp manifest: {}", e)))?;

        tokio::fs::rename(&tmp, path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to finalize manifest: {}", e)))?;
        Ok(())
    }

    pub fn prune_missing_files(&mut self) {
        self.artifacts.retain(|_, a| a.path.exists());
    }
}

fn tmp_path(path: &Path) -> PathBuf {
    let mut tmp = path.to_path_buf();
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("artifacts.json");
    tmp.set_file_name(format!("{}.tmp", name));
    tmp
}
