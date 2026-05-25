//! PH-S21: Raft membership recovery from log entries (ConfigChange / SnapshotPointer).
//!
//! Run: `cargo test-raft-ci` or
//! `cargo test -j 1 --test raft_membership_log --features raft,test-utils -- --test-threads=1`

#![cfg(feature = "raft")]

use async_raft::raft::{
    Entry, EntryConfigChange, EntryPayload, EntrySnapshotPointer, MembershipConfig,
};
use async_raft::storage::RaftStorage;
use poolai::raid::raft::{
    extract_membership_from_log, log_has_membership_entry, RaidRaftOperation, RaidRaftStorage,
};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::RwLock;

fn membership_with_nodes(ids: &[u64]) -> MembershipConfig {
    MembershipConfig {
        members: ids.iter().copied().collect(),
        members_after_consensus: None,
    }
}

fn config_change_entry(index: u64, term: u64, members: &[u64]) -> Entry<RaidRaftOperation> {
    Entry {
        term,
        index,
        payload: EntryPayload::ConfigChange(EntryConfigChange {
            membership: membership_with_nodes(members),
        }),
    }
}

fn snapshot_pointer_entry(index: u64, term: u64, members: &[u64]) -> Entry<RaidRaftOperation> {
    Entry {
        term,
        index,
        payload: EntryPayload::SnapshotPointer(EntrySnapshotPointer {
            id: format!("snap-{index}"),
            membership: membership_with_nodes(members),
        }),
    }
}

async fn storage_for_node(temp: &TempDir, node_id: u64) -> (RaidRaftStorage, std::path::PathBuf) {
    let raft_path = temp.path().join(format!("raft-{node_id}"));
    tokio::fs::create_dir_all(&raft_path).await.unwrap();

    let raid_config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp.path().join(format!("raid-{node_id}")),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let raid_manager = Arc::new(RwLock::new(RaidManager::new(raid_config)));
    raid_manager.write().await.initialize().await.unwrap();

    let storage = RaidRaftStorage::new(node_id, raid_manager, raft_path.clone());
    (storage, raft_path)
}

#[test]
fn extract_membership_uses_latest_config_change() {
    let entries = vec![
        config_change_entry(1, 1, &[1]),
        config_change_entry(2, 1, &[1, 2]),
    ];
    assert!(log_has_membership_entry(&entries));
    let membership = extract_membership_from_log(&entries, 1);
    assert_eq!(membership.all_nodes(), HashSet::from([1, 2]));
}

#[test]
fn extract_membership_prefers_newer_snapshot_pointer() {
    let entries = vec![
        config_change_entry(1, 1, &[1, 2]),
        snapshot_pointer_entry(2, 1, &[1, 2, 3]),
    ];
    let membership = extract_membership_from_log(&entries, 1);
    assert_eq!(membership.all_nodes(), HashSet::from([1, 2, 3]));
}

#[test]
fn log_has_membership_entry_false_for_normal_only() {
    let entries = vec![Entry {
        term: 1,
        index: 1,
        payload: EntryPayload::Blank,
    }];
    assert!(!log_has_membership_entry(&entries));
}

#[tokio::test]
async fn get_membership_config_reads_from_log() {
    let temp = TempDir::new().unwrap();
    let (storage, _path) = storage_for_node(&temp, 1).await;

    storage
        .append_entry_to_log(&config_change_entry(1, 1, &[1, 2]))
        .await
        .unwrap();

    let membership = storage.get_membership_config().await.unwrap();
    assert_eq!(membership.all_nodes(), HashSet::from([1, 2]));
}

#[tokio::test]
async fn append_entry_persists_membership_cache() {
    let temp = TempDir::new().unwrap();
    let (storage, raft_path) = storage_for_node(&temp, 1).await;

    storage
        .append_entry_to_log(&config_change_entry(1, 1, &[1, 2, 3]))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(100)).await;

    let cache_path = raft_path.join("membership.json");
    assert!(cache_path.exists(), "membership cache should be written");

    tokio::fs::remove_file(raft_path.join("raft_log.json"))
        .await
        .unwrap();

    let membership = storage.get_membership_config().await.unwrap();
    assert_eq!(membership.all_nodes(), HashSet::from([1, 2, 3]));
}
