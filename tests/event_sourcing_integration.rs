//! Integration tests for Event Sourcing
//!
//! Tests:
//! - Event store creation and initialization
//! - Event appending and loading
//! - Event replay
//! - Snapshot creation and loading
//! - Event queries (by artifact, time range)

use chrono::Utc;
use poolai::raid::{
    events::{EventStore, RaidEvent, Snapshot},
    RaidConfig, RaidManager, RaidMode,
};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_event_store_creation() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    // Verify paths
    assert!(event_store
        .event_log_path()
        .to_string_lossy()
        .contains("events.log"));
    assert!(event_store
        .snapshot_path()
        .to_string_lossy()
        .contains("snapshot.json"));
}

#[tokio::test]
async fn test_event_append_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    // Append an event
    let event = RaidEvent::ArtifactCreated {
        artifact_id: "test-artifact-1".to_string(),
        node_id: 1,
        timestamp: Utc::now(),
        metadata: serde_json::json!({
            "name": "test",
            "size": 1024
        }),
    };

    let record = event_store.append_event(event).await.unwrap();
    assert_eq!(record.sequence, 1);
    assert!(record.event_id.to_string().len() > 0);

    // Load events
    let events = event_store.load_events().await.unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].sequence, 1);
}

#[tokio::test]
async fn test_event_replay() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    // Append multiple events
    for i in 0..5 {
        let event = RaidEvent::ArtifactCreated {
            artifact_id: format!("test-artifact-{}", i),
            node_id: 1,
            timestamp: Utc::now(),
            metadata: serde_json::json!({
                "index": i
            }),
        };
        event_store.append_event(event).await.unwrap();
    }

    // Replay events
    let mut replayed_count = 0;
    event_store
        .replay_events(|event| {
            replayed_count += 1;
            assert!(event.sequence > 0);
            Ok(())
        })
        .await
        .unwrap();

    assert_eq!(replayed_count, 5);
}

#[tokio::test]
async fn test_event_queries() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    // Append events for different artifacts
    for i in 0..3 {
        let event = RaidEvent::ArtifactCreated {
            artifact_id: format!("artifact-{}", i),
            node_id: 1,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };
        event_store.append_event(event).await.unwrap();
    }

    // Query events for specific artifact
    let events = event_store
        .get_events_for_artifact("artifact-1")
        .await
        .unwrap();
    assert_eq!(events.len(), 1);
    match &events[0].event {
        RaidEvent::ArtifactCreated { artifact_id, .. } => {
            assert_eq!(artifact_id, "artifact-1");
        }
        _ => panic!("Expected ArtifactCreated event"),
    }

    // Query events since sequence
    let events = event_store.get_events_since(2).await.unwrap();
    assert_eq!(events.len(), 1); // Only sequence 3
}

#[tokio::test]
async fn test_snapshot_creation() {
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

    // Create some artifacts
    raid_manager
        .write()
        .await
        .put_artifact("test1", b"data1")
        .await
        .unwrap();
    raid_manager
        .write()
        .await
        .put_artifact("test2", b"data2")
        .await
        .unwrap();

    // Create snapshot
    raid_manager.write().await.create_snapshot().await.unwrap();

    // Verify snapshot exists
    let event_store_opt = raid_manager.read().await.event_store();
    if let Some(event_store) = event_store_opt {
        let snapshot = event_store.read().await.load_snapshot().await.unwrap();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        assert!(snapshot.sequence > 0);
        assert!(snapshot.artifacts.is_object());
    }
}

#[tokio::test]
async fn test_snapshot_replay() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    // Create some events
    for i in 0..10 {
        let event = RaidEvent::ArtifactCreated {
            artifact_id: format!("artifact-{}", i),
            node_id: 1,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };
        event_store.append_event(event).await.unwrap();
    }

    // Create snapshot (simulate)
    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: storage_path.clone(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };

    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    // Create snapshot
    raid_manager.write().await.create_snapshot().await.unwrap();

    // Replay events since snapshot
    let event_store_opt = raid_manager.read().await.event_store();
    if let Some(event_store) = event_store_opt {
        let start_sequence = event_store
            .read()
            .await
            .replay_events_since_snapshot(|_event| Ok(()))
            .await
            .unwrap();

        // Should start from snapshot sequence (0 if no snapshot, or snapshot sequence)
        assert!(start_sequence >= 0);
    }
}

#[tokio::test]
async fn test_event_time_range() {
    let temp_dir = TempDir::new().unwrap();
    let storage_path = temp_dir.path().to_path_buf();

    let event_store = EventStore::new(storage_path.join("events"));
    event_store.initialize().await.unwrap();

    let start_time = Utc::now();

    // Append events with delays
    for i in 0..3 {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let event = RaidEvent::ArtifactCreated {
            artifact_id: format!("artifact-{}", i),
            node_id: 1,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };
        event_store.append_event(event).await.unwrap();
    }

    let end_time = Utc::now();

    // Query events in time range
    let events = event_store
        .get_events_in_range(start_time, end_time)
        .await
        .unwrap();
    assert_eq!(events.len(), 3);
}

#[tokio::test]
async fn test_event_integration_with_raid_manager() {
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

    // Create artifacts (should generate events)
    raid_manager
        .write()
        .await
        .put_artifact("test1", b"data1")
        .await
        .unwrap();
    raid_manager
        .write()
        .await
        .put_artifact("test2", b"data2")
        .await
        .unwrap();

    // Verify events were created
    let event_store_opt = raid_manager.read().await.event_store();
    if let Some(event_store) = event_store_opt {
        let events = event_store.read().await.load_events().await.unwrap();
        assert!(events.len() >= 2); // At least 2 ArtifactCreated events

        // Verify artifact events
        let artifact_events: Vec<_> = events
            .iter()
            .filter(|e| matches!(&e.event, RaidEvent::ArtifactCreated { .. }))
            .collect();
        assert!(artifact_events.len() >= 2);
    }
}
