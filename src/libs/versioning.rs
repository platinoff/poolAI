//! Version Manager - Library versioning management
//!
//! Provides:
//! - Semantic versioning support
//! - Version tracking
//! - Rollback capabilities

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

/// Version Manager - Manages library versions
pub struct VersionManager {
    versions: HashMap<String, Vec<VersionInfo>>, // name -> versions
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub path: PathBuf,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
}

impl VersionManager {
    /// Create new version manager
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// Register a version
    pub async fn register_version(
        &mut self,
        name: &str,
        version: &str,
        path: &PathBuf,
    ) -> Result<(), AppError> {
        let versions = self.versions
            .entry(name.to_string())
            .or_insert_with(Vec::new);
        
        // Check if version already exists
        if versions.iter().any(|v| v.version == version) {
            return Err(AppError::ConfigError(
                format!("Version {} already registered for {}", version, name)
            ));
        }
        
        // Mark all other versions as inactive
        for v in versions.iter_mut() {
            v.is_active = false;
        }
        
        // Add new version as active
        versions.push(VersionInfo {
            version: version.to_string(),
            path: path.clone(),
            installed_at: chrono::Utc::now(),
            is_active: true,
        });
        
        // Sort versions (semantic versioning)
        versions.sort_by(|a, b| {
            compare_versions(&a.version, &b.version)
        });
        
        Ok(())
    }

    /// Unregister a version
    pub async fn unregister_version(&mut self, name: &str) -> Result<(), AppError> {
        self.versions.remove(name);
        Ok(())
    }

    /// Get active version for a library
    pub fn get_active_version(&self, name: &str) -> Option<&VersionInfo> {
        self.versions
            .get(name)?
            .iter()
            .find(|v| v.is_active)
    }

    /// Get all versions for a library
    pub fn get_versions(&self, name: &str) -> Option<&Vec<VersionInfo>> {
        self.versions.get(name)
    }

    /// Rollback to a specific version
    pub async fn rollback(&mut self, name: &str, version: &str) -> Result<(), AppError> {
        let versions = self.versions.get_mut(name)
            .ok_or_else(|| AppError::ConfigError(format!("Library {} not found", name)))?;
        
        // Find version index first
        let version_index = versions.iter()
            .position(|v| v.version == version)
            .ok_or_else(|| AppError::ConfigError(format!("Version {} not found", version)))?;
        
        // Mark all as inactive
        for v in versions.iter_mut() {
            v.is_active = false;
        }
        
        // Mark target version as active
        versions[version_index].is_active = true;
        
        info!("Rolled back {} to version {}", name, version);
        Ok(())
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Compare semantic versions
/// Supports basic semantic versioning (MAJOR.MINOR.PATCH)
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    // Parse version strings into components
    let a_parts: Vec<u32> = a
        .split('.')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    
    let b_parts: Vec<u32> = b
        .split('.')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    
    // Compare major, minor, patch
    for i in 0..3 {
        let a_val = a_parts.get(i).copied().unwrap_or(0);
        let b_val = b_parts.get(i).copied().unwrap_or(0);
        
        match a_val.cmp(&b_val) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    
    // If all components are equal, compare as strings (for pre-release, build metadata)
    a.cmp(b)
}

