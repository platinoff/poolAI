//! Version Manager - Library versioning management
//!
//! Provides:
//! - Semantic versioning support
//! - Version tracking
//! - Rollback capabilities

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
        path: &Path,
    ) -> Result<(), AppError> {
        let versions = self.versions.entry(name.to_string()).or_default();

        // Check if version already exists
        if versions.iter().any(|v| v.version == version) {
            return Err(AppError::ConfigError(format!(
                "Version {} already registered for {}",
                version, name
            )));
        }

        // Mark all other versions as inactive
        for v in versions.iter_mut() {
            v.is_active = false;
        }

        // Add new version as active
        versions.push(VersionInfo {
            version: version.to_string(),
            path: path.to_path_buf(),
            installed_at: chrono::Utc::now(),
            is_active: true,
        });

        // Sort versions (semantic versioning)
        versions.sort_by(|a, b| compare_versions(&a.version, &b.version));

        Ok(())
    }

    /// Unregister a version
    pub async fn unregister_version(&mut self, name: &str) -> Result<(), AppError> {
        self.versions.remove(name);
        Ok(())
    }

    /// Get active version for a library
    pub fn get_active_version(&self, name: &str) -> Option<&VersionInfo> {
        self.versions.get(name)?.iter().find(|v| v.is_active)
    }

    /// Get all versions for a library
    pub fn get_versions(&self, name: &str) -> Option<&Vec<VersionInfo>> {
        self.versions.get(name)
    }

    /// Rollback to a specific version
    pub async fn rollback(&mut self, name: &str, version: &str) -> Result<(), AppError> {
        let versions = self
            .versions
            .get_mut(name)
            .ok_or_else(|| AppError::ConfigError(format!("Library {} not found", name)))?;

        // Find version index first
        let version_index = versions
            .iter()
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
    let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse::<u32>().ok()).collect();

    let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse::<u32>().ok()).collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn register_and_get_active_version() {
        let mut vm = VersionManager::new();
        let p1 = PathBuf::from("C:\\tmp\\lib\\1.0.0");
        let p2 = PathBuf::from("C:\\tmp\\lib\\1.1.0");

        vm.register_version("lib", "1.0.0", &p1).await.unwrap();
        assert_eq!(vm.get_active_version("lib").unwrap().version, "1.0.0");

        vm.register_version("lib", "1.1.0", &p2).await.unwrap();
        assert_eq!(vm.get_active_version("lib").unwrap().version, "1.1.0");
    }

    #[test]
    fn compare_versions_semver() {
        assert_eq!(
            compare_versions("1.2.3", "1.2.3"),
            std::cmp::Ordering::Equal
        );
        assert_eq!(compare_versions("1.2.3", "1.2.4"), std::cmp::Ordering::Less);
        assert_eq!(
            compare_versions("2.0.0", "1.9.9"),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn compare_versions_edge_cases() {
        // Test with missing components (compares numerically, then falls back to string comparison)
        assert_eq!(compare_versions("1.0", "1.0.0"), std::cmp::Ordering::Less); // "1.0" < "1.0.0" lexicographically
        assert_eq!(compare_versions("1", "1.0.0"), std::cmp::Ordering::Less); // "1" < "1.0.0" lexicographically

        // Test major version differences
        assert_eq!(
            compare_versions("2.0.0", "1.9.9"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(compare_versions("1.0.0", "2.0.0"), std::cmp::Ordering::Less);

        // Test minor version differences
        assert_eq!(
            compare_versions("1.2.0", "1.1.9"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(compare_versions("1.1.0", "1.2.0"), std::cmp::Ordering::Less);

        // Test patch version differences
        assert_eq!(
            compare_versions("1.0.1", "1.0.0"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(compare_versions("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
    }

    #[tokio::test]
    async fn test_rollback() {
        let mut vm = VersionManager::new();
        let p1 = PathBuf::from("/tmp/lib/1.0.0");
        let p2 = PathBuf::from("/tmp/lib/2.0.0");

        vm.register_version("lib", "1.0.0", &p1).await.unwrap();
        vm.register_version("lib", "2.0.0", &p2).await.unwrap();

        // Active version should be 2.0.0
        assert_eq!(vm.get_active_version("lib").unwrap().version, "2.0.0");

        // Rollback to 1.0.0
        vm.rollback("lib", "1.0.0").await.unwrap();
        assert_eq!(vm.get_active_version("lib").unwrap().version, "1.0.0");
    }

    #[tokio::test]
    async fn test_register_duplicate_version() {
        let mut vm = VersionManager::new();
        let p1 = PathBuf::from("/tmp/lib/1.0.0");

        vm.register_version("lib", "1.0.0", &p1).await.unwrap();

        // Attempt to register same version again should fail
        let result = vm.register_version("lib", "1.0.0", &p1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unregister_version() {
        let mut vm = VersionManager::new();
        let p1 = PathBuf::from("/tmp/lib/1.0.0");

        vm.register_version("lib", "1.0.0", &p1).await.unwrap();
        assert!(vm.get_active_version("lib").is_some());

        vm.unregister_version("lib").await.unwrap();
        assert!(vm.get_active_version("lib").is_none());
    }

    #[test]
    fn test_get_versions() {
        let vm = VersionManager::new();

        // No versions initially
        assert!(vm.get_versions("lib").is_none());
    }
}
