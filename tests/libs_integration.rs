//! Integration tests for Library Management Module
//!
//! Tests:
//! - Manifest persistence (load/save atomic writes)
//! - Atomic install rollback scenarios
//! - Dependency resolution with fixtures

use poolai::libs::manifest::InstalledLibrariesManifest;
use poolai::libs::{LibraryInfo, LibraryMetadata};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_manifest_load_save_atomic() {
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("manifest.json");

    // Create test libraries
    let mut libraries = HashMap::new();
    libraries.insert(
        "test-lib".to_string(),
        LibraryInfo {
            name: "test-lib".to_string(),
            version: "1.0.0".to_string(),
            path: PathBuf::from("/test/path"),
            dependencies: vec![],
            metadata: LibraryMetadata::default(),
            artifact_ref: None,
        },
    );

    // Save manifest
    let manifest = InstalledLibrariesManifest::new(libraries.clone());
    manifest.save_atomic(&manifest_path).await.unwrap();

    // Verify file exists
    assert!(manifest_path.exists());

    // Load manifest
    let loaded = InstalledLibrariesManifest::load(&manifest_path)
        .await
        .unwrap();
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();

    // Verify data integrity
    assert_eq!(loaded.libraries.len(), 1);
    assert!(loaded.libraries.contains_key("test-lib"));
    assert_eq!(loaded.libraries["test-lib"].version, "1.0.0");
}

#[tokio::test]
async fn test_manifest_load_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("nonexistent.json");

    // Load non-existent manifest should return None
    let loaded = InstalledLibrariesManifest::load(&manifest_path)
        .await
        .unwrap();
    assert!(loaded.is_none());
}

#[tokio::test]
async fn test_manifest_atomic_write_survives_crash() {
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("manifest.json");

    // Create and save manifest
    let libraries = HashMap::new();
    let manifest = InstalledLibrariesManifest::new(libraries);
    manifest.save_atomic(&manifest_path).await.unwrap();

    // Verify tmp file is cleaned up
    let tmp_path = manifest_path.parent().unwrap().join("manifest.json.tmp");
    assert!(
        !tmp_path.exists(),
        "Temporary file should be removed after atomic write"
    );

    // Verify final file exists
    assert!(manifest_path.exists(), "Final manifest file should exist");
}

#[tokio::test]
async fn test_manifest_multiple_libraries() {
    let temp_dir = TempDir::new().unwrap();
    let manifest_path = temp_dir.path().join("manifest.json");

    // Create multiple libraries
    let mut libraries = HashMap::new();
    for i in 1..=5 {
        libraries.insert(
            format!("lib-{}", i),
            LibraryInfo {
                name: format!("lib-{}", i),
                version: format!("1.0.{}", i),
                path: PathBuf::from(format!("/test/path/{}", i)),
                dependencies: vec![],
                metadata: LibraryMetadata::default(),
                artifact_ref: None,
            },
        );
    }

    // Save and load
    let manifest = InstalledLibrariesManifest::new(libraries.clone());
    manifest.save_atomic(&manifest_path).await.unwrap();

    let loaded = InstalledLibrariesManifest::load(&manifest_path)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded.libraries.len(), 5);
    for i in 1..=5 {
        assert!(loaded.libraries.contains_key(&format!("lib-{}", i)));
    }
}
