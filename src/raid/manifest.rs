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
                    AppError::ConfigError(format!(
                        "Failed to parse artifact manifest. Context: Cannot deserialize artifact manifest from JSON. Suggestion: Verify manifest file integrity and JSON format. Path: '{}', Error: {}",
                        path.display(),
                        e
                    ))
                })?;
                Ok(Some(m))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AppError::ConfigError(format!(
                "Failed to read artifact manifest. Context: Cannot read artifact manifest file. Suggestion: Check file permissions and disk I/O status. Path: '{}', Error: {}",
                path.display(),
                e
            ))),
        }
    }

    pub async fn save_atomic(&self, path: &Path) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create manifest directory. Context: Cannot create directory for artifact manifest. Suggestion: Check filesystem permissions and available disk space. Path: '{}', Error: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        let tmp = tmp_path(path);
        let data = serde_json::to_vec_pretty(self).map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to serialize artifact manifest. Context: Cannot serialize artifact manifest to JSON. Suggestion: Verify manifest structure and serialization logic. Artifact count: {}, Path: '{}', Error: {}",
                self.artifacts.len(),
                path.display(),
                e
            ))
        })?;

        let mut f = tokio::fs::File::create(&tmp)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to create temporary manifest file. Context: Cannot create temporary file for atomic write. Suggestion: Check filesystem permissions and available disk space. Temp path: '{}', Error: {}",
                tmp.display(),
                e
            )))?;
        f.write_all(&data)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to write temporary manifest. Context: Cannot write manifest data to temporary file. Suggestion: Check disk space and I/O status. Temp path: '{}', Data size: {} bytes, Error: {}",
                tmp.display(),
                data.len(),
                e
            )))?;
        f.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to sync temporary manifest. Context: Cannot sync temporary manifest file to disk. Suggestion: Check disk I/O status and filesystem health. Temp path: '{}', Error: {}",
                tmp.display(),
                e
            )))?;

        tokio::fs::rename(&tmp, path)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to finalize manifest. Context: Cannot rename temporary file to final manifest path (atomic write). Suggestion: Check filesystem permissions and ensure target path is not locked. Temp path: '{}', Final path: '{}', Error: {}",
                tmp.display(),
                path.display(),
                e
            )))?;
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
