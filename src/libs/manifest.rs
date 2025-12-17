//! Installed libraries manifest (on-disk state)
//!
//! Production-min goal:
//! - Persist installed libraries between restarts
//! - Use atomic writes to avoid corrupt manifests on crash/power loss

use crate::core::error::AppError;
use crate::libs::LibraryInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledLibrariesManifest {
    pub schema_version: u32,
    pub updated_at: DateTime<Utc>,
    pub libraries: HashMap<String, LibraryInfo>,
}

impl InstalledLibrariesManifest {
    pub fn new(libraries: HashMap<String, LibraryInfo>) -> Self {
        Self {
            schema_version: 1,
            updated_at: Utc::now(),
            libraries,
        }
    }

    pub async fn load(path: &Path) -> Result<Option<Self>, AppError> {
        match tokio::fs::read(path).await {
            Ok(bytes) => {
                let manifest: Self = serde_json::from_slice(&bytes)
                    .map_err(|e| AppError::ConfigError(format!("Failed to parse manifest: {}", e)))?;
                Ok(Some(manifest))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AppError::ConfigError(format!("Failed to read manifest: {}", e))),
        }
    }

    pub async fn save_atomic(&self, path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::ConfigError(format!("Failed to create manifest directory: {}", e)))?;
        }

        let tmp_path = tmp_manifest_path(path);
        let data = serde_json::to_vec_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize manifest: {}", e)))?;

        let mut file = tokio::fs::File::create(&tmp_path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to create manifest tmp file: {}", e)))?;
        file.write_all(&data)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write manifest tmp file: {}", e)))?;
        file.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to sync manifest tmp file: {}", e)))?;

        tokio::fs::rename(&tmp_path, path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to move manifest into place: {}", e)))?;
        Ok(())
    }
}

fn tmp_manifest_path(path: &Path) -> PathBuf {
    let mut tmp = path.to_path_buf();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("manifest.json");
    tmp.set_file_name(format!("{}.tmp", file_name));
    tmp
}


