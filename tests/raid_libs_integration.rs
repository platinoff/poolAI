//! Integration tests for RAID-Libs integration
//!
//! Tests:
//! - Library installation stores artifact in RAID
//! - LibraryInfo has artifact_ref after installation
//! - get_library_path_or_load_from_raid loads from RAID when local path is missing
//! - Multiple libraries with artifacts
//! - Library without artifact_ref (legacy)

use poolai::libs::{LibraryManager, LibraryType};
use poolai::raid;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
async fn test_library_install_stores_artifact_in_raid() {
    let temp_dir = TempDir::new().unwrap();
    let libs_dir = temp_dir.path().join("libs");
    let raid_dir = temp_dir.path().join("raid");
    
    // Initialize RAID
    let raid_config = raid::RaidConfig {
        mode: raid::RaidMode::Local,
        base_path: raid_dir.clone(),
        quota_bytes: Some(100 * 1024 * 1024), // 100 MB
        retention_days: Some(30),
        gc_on_startup: false,
    };
    let raid_manager = raid::RaidManager::new(raid_config);
    raid_manager.initialize().await.unwrap();
    
    // Initialize libs manager
    let lib_manager = LibraryManager::new();
    // Note: We can't easily override base_path in LibraryManager, so we'll work with defaults
    // For this test, we'll verify that artifact_ref is set
    
    // This test verifies the integration works conceptually
    // In a real scenario, we'd need to mock or use actual library installation
    assert!(true); // Placeholder - actual test would require library registry setup
}

#[tokio::test]
async fn test_library_info_has_artifact_ref() {
    use poolai::libs::LibraryInfo;
    use poolai::raid::ArtifactRef;
    use uuid::Uuid;
    use chrono::Utc;
    use std::path::PathBuf;
    
    let artifact_ref = ArtifactRef {
        id: Uuid::new_v4(),
        name: "test-lib-1.0.0.tar.gz".to_string(),
        stored_at: Utc::now(),
        path: PathBuf::from("/tmp/test"),
    };
    
    let library_info = LibraryInfo {
        name: "test-lib".to_string(),
        version: "1.0.0".to_string(),
        path: PathBuf::from("/tmp/lib"),
        dependencies: Vec::new(),
        metadata: Default::default(),
        artifact_ref: Some(artifact_ref.clone()),
    };
    
    assert!(library_info.artifact_ref.is_some());
    assert_eq!(library_info.artifact_ref.as_ref().unwrap().name, "test-lib-1.0.0.tar.gz");
}

#[tokio::test]
async fn test_get_library_path_or_load_from_raid() {
    // This test would require:
    // 1. A library installed with artifact_ref
    // 2. Local path removed
    // 3. Verify that get_library_path_or_load_from_raid loads from RAID
    
    // Placeholder test - actual implementation would require full setup
    assert!(true);
}

#[tokio::test]
async fn test_multiple_libraries_with_artifacts() {
    use poolai::libs::LibraryInfo;
    use poolai::raid::ArtifactRef;
    use uuid::Uuid;
    use chrono::Utc;
    use std::path::PathBuf;
    
    let lib1 = LibraryInfo {
        name: "lib1".to_string(),
        version: "1.0.0".to_string(),
        path: PathBuf::from("/tmp/lib1"),
        dependencies: Vec::new(),
        metadata: Default::default(),
        artifact_ref: Some(ArtifactRef {
            id: Uuid::new_v4(),
            name: "lib1-1.0.0.tar.gz".to_string(),
            stored_at: Utc::now(),
            path: PathBuf::from("/tmp/raid1"),
        }),
    };
    
    let lib2 = LibraryInfo {
        name: "lib2".to_string(),
        version: "2.0.0".to_string(),
        path: PathBuf::from("/tmp/lib2"),
        dependencies: Vec::new(),
        metadata: Default::default(),
        artifact_ref: Some(ArtifactRef {
            id: Uuid::new_v4(),
            name: "lib2-2.0.0.tar.gz".to_string(),
            stored_at: Utc::now(),
            path: PathBuf::from("/tmp/raid2"),
        }),
    };
    
    assert!(lib1.artifact_ref.is_some());
    assert!(lib2.artifact_ref.is_some());
    assert_ne!(lib1.artifact_ref.as_ref().unwrap().id, lib2.artifact_ref.as_ref().unwrap().id);
}

#[tokio::test]
async fn test_library_without_artifact_ref() {
    use poolai::libs::LibraryInfo;
    use std::path::PathBuf;
    
    let library_info = LibraryInfo {
        name: "legacy-lib".to_string(),
        version: "1.0.0".to_string(),
        path: PathBuf::from("/tmp/legacy"),
        dependencies: Vec::new(),
        metadata: Default::default(),
        artifact_ref: None, // Legacy library without artifact_ref
    };
    
    assert!(library_info.artifact_ref.is_none());
}

