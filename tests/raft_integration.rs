//! Integration tests for Raft Consensus Integration
//!
//! Tests:
//! - Raft node creation and initialization
//! - State machine operation application
//! - Storage structure functionality
//! - Transport node management

#[cfg(feature = "raft")]
use poolai::raid::{
    raft::{RaidRaftNode, RaidRaftStateMachine, RaidRaftStorage, RaftConfig, RaidRaftOperation},
    RaidConfig, RaidManager, RaidMode,
};
#[cfg(feature = "raft")]
use std::path::PathBuf;
#[cfg(feature = "raft")]
use std::sync::Arc;
#[cfg(feature = "raft")]
use tokio::sync::RwLock;
#[cfg(feature = "raft")]
use tempfile::TempDir;

#[cfg(feature = "raft")]
#[tokio::test]
async fn test_raft_node_creation() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: storage_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    let raft_config = RaftConfig {
        node_id: 1,
        cluster_members: vec![1],
        election_timeout: 1000,
        heartbeat_interval: 100,
    };

    let raft_node = RaidRaftNode::new(
        raft_config,
        raid_manager.clone(),
        storage_path.join("raft"),
    )
    .unwrap();

    // Test initialization
    raft_node.initialize().await.unwrap();

    // Test basic methods
    assert!(!raft_node.is_leader().await);
    assert_eq!(raft_node.current_term().await, 0);
    assert_eq!(raft_node.current_role().await, "Follower");
}

#[cfg(feature = "raft")]
#[tokio::test]
async fn test_raft_state_machine_apply_operation() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: storage_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    let state_machine = RaidRaftStateMachine::new(raid_manager.clone());

    // Test PutArtifact operation
    let operation = RaidRaftOperation::PutArtifact {
        artifact_id: "test-artifact-1".to_string(),
        data: b"test data".to_vec(),
        metadata: poolai::raid::manifest::ArtifactManifest::new(),
    };

    let response = state_machine.apply_operation(&operation).await.unwrap();
    match response {
        poolai::raid::raft::RaidRaftResponse::Success { message } => {
            assert!(message.contains("stored"));
        }
        _ => panic!("Expected Success response"),
    }
}

#[cfg(feature = "raft")]
#[tokio::test]
async fn test_raft_storage_paths() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: storage_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    let storage = RaidRaftStorage::new(1, raid_manager, storage_path.clone());

    let log_path = storage.log_path();
    let state_path = storage.state_path();

    assert!(log_path.to_string_lossy().contains("raft_log.json"));
    assert!(state_path.to_string_lossy().contains("raft_state.json"));
    assert!(log_path.parent().unwrap() == state_path.parent().unwrap());
}

#[cfg(feature = "raft")]
#[tokio::test]
async fn test_raft_node_apply_operation() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: storage_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    let raft_config = RaftConfig {
        node_id: 1,
        cluster_members: vec![1],
        election_timeout: 1000,
        heartbeat_interval: 100,
    };

    let raft_node = RaidRaftNode::new(
        raft_config,
        raid_manager.clone(),
        storage_path.join("raft"),
    )
    .unwrap();

    raft_node.initialize().await.unwrap();

    // Test apply operation (non-consensus mode)
    let operation = RaidRaftOperation::PutArtifact {
        artifact_id: "test-artifact-2".to_string(),
        data: b"test data 2".to_vec(),
        metadata: poolai::raid::manifest::ArtifactManifest::new(),
    };

    let response = raft_node.apply_operation(operation).await.unwrap();
    match response {
        poolai::raid::raft::RaidRaftResponse::Success { message } => {
            assert!(message.contains("stored"));
        }
        _ => panic!("Expected Success response"),
    }
}

#[cfg(feature = "raft")]
#[tokio::test]
async fn test_raft_transport_node_management() {
    use poolai::raid::raft_transport::HttpRaftTransport;

    let transport = HttpRaftTransport::new();

    // Test adding nodes
    transport.add_node(1, "http://127.0.0.1:8080".to_string()).await;
    transport.add_node(2, "http://127.0.0.1:8081".to_string()).await;

    // Test getting node address
    let addr1 = transport.get_node_address(1).await;
    assert_eq!(addr1, Some("http://127.0.0.1:8080".to_string()));

    let addr2 = transport.get_node_address(2).await;
    assert_eq!(addr2, Some("http://127.0.0.1:8081".to_string()));

    // Test removing node
    transport.remove_node(1).await;
    let addr1_after = transport.get_node_address(1).await;
    assert_eq!(addr1_after, None);
}

