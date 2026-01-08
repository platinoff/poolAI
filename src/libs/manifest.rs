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
                let manifest: Self = serde_json::from_slice(&bytes).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to parse manifest. Context: Manifest file contains invalid JSON. \
                        Suggestion: Verify manifest file integrity or delete corrupted manifest to regenerate. \
                        Path: '{}', Error: {}",
                        path.display(), e
                    ))
                })?;
                Ok(Some(manifest))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AppError::ConfigError(format!(
                "Failed to read manifest. Context: Cannot read manifest file from disk. \
                Suggestion: Check file permissions and ensure file is not locked by another process. \
                Path: '{}', Error: {}",
                path.display(), e
            ))),
        }
    }

    pub async fn save_atomic(&self, path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create manifest directory. Context: Cannot create parent directory for manifest file. \
                    Suggestion: Check filesystem permissions and ensure parent directory path is valid. \
                    Path: '{}', Error: {}",
                    parent.display(), e
                ))
            })?;
        }

        let tmp_path = tmp_manifest_path(path);
        let data = serde_json::to_vec_pretty(self)
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to serialize manifest. Context: Cannot convert manifest to JSON format. \
                Suggestion: Check manifest data structure and ensure all fields are serializable. \
                Error: {}",
                e
            )))?;

        let mut file = tokio::fs::File::create(&tmp_path).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to create manifest tmp file. Context: Cannot create temporary manifest file for atomic write. \
                Suggestion: Check filesystem permissions and disk space. \
                Path: '{}', Error: {}",
                tmp_path.display(), e
            ))
        })?;
        file.write_all(&data).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to write manifest tmp file. Context: Cannot write manifest data to temporary file. \
                Suggestion: Check disk space and filesystem permissions. \
                Path: '{}', Error: {}",
                tmp_path.display(), e
            ))
        })?;
        file.sync_all().await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to sync manifest tmp file. Context: Cannot flush manifest data to disk. \
                Suggestion: Check disk space and filesystem integrity. \
                Path: '{}', Error: {}",
                tmp_path.display(), e
            ))
        })?;

        tokio::fs::rename(&tmp_path, path).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to move manifest into place. Context: Cannot atomically replace manifest file. \
                Suggestion: Ensure both source and destination are on the same filesystem and check permissions. \
                From: '{}', To: '{}', Error: {}",
                tmp_path.display(), path.display(), e
            ))
        })?;
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
