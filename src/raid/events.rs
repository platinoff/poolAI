//! Event Sourcing for Distributed RAID
//!
//! This module provides event sourcing capabilities for auditability
//! and state reconstruction in the Distributed RAID system.

use crate::core::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

/// Event types for Distributed RAID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaidEvent {
    /// Artifact was created
    ArtifactCreated {
        artifact_id: String,
        node_id: u64,
        timestamp: DateTime<Utc>,
        metadata: serde_json::Value,
    },
    /// Artifact was updated
    ArtifactUpdated {
        artifact_id: String,
        node_id: u64,
        timestamp: DateTime<Utc>,
        changes: serde_json::Value,
    },
    /// Artifact was deleted
    ArtifactDeleted {
        artifact_id: String,
        node_id: u64,
        timestamp: DateTime<Utc>,
    },
    /// Node joined the cluster
    NodeJoined {
        node_id: u64,
        address: String,
        timestamp: DateTime<Utc>,
    },
    /// Node left the cluster
    NodeLeft {
        node_id: u64,
        timestamp: DateTime<Utc>,
    },
    /// Replication started
    ReplicationStarted {
        artifact_id: String,
        source_node: u64,
        target_node: u64,
        timestamp: DateTime<Utc>,
    },
    /// Replication completed
    ReplicationCompleted {
        artifact_id: String,
        source_node: u64,
        target_node: u64,
        timestamp: DateTime<Utc>,
    },
}

/// Event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    /// Unique event ID
    pub event_id: Uuid,
    /// Event sequence number
    pub sequence: u64,
    /// Event type and data
    pub event: RaidEvent,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
}

/// Snapshot data structure for fast recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Snapshot sequence (last event sequence included)
    pub sequence: u64,
    /// Snapshot timestamp
    pub timestamp: DateTime<Utc>,
    /// Artifact manifest snapshot
    pub artifacts: serde_json::Value,
    /// Nodes snapshot
    pub nodes: serde_json::Value,
}

/// Event store for Distributed RAID
pub struct EventStore {
    /// Storage path for events
    storage_path: PathBuf,
    /// Current sequence number
    sequence: Arc<RwLock<u64>>,
    /// Event log file path
    event_log_path: PathBuf,
    /// Snapshot file path
    snapshot_path: PathBuf,
}

impl EventStore {
    /// Create a new event store
    pub fn new(storage_path: PathBuf) -> Self {
        let event_log_path = storage_path.join("events.log");
        let snapshot_path = storage_path.join("snapshot.json");

        Self {
            storage_path,
            sequence: Arc::new(RwLock::new(0)),
            event_log_path,
            snapshot_path,
        }
    }

    /// Initialize the event store
    pub async fn initialize(&self) -> Result<(), AppError> {
        // Create storage directory if it doesn't exist
        if let Some(parent) = self.storage_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                AppError::ConfigError(format!("Failed to create event store directory: {}", e))
            })?;
        }

        // Load existing events to determine current sequence
        let events = self.load_events().await?;
        let max_sequence = events.iter().map(|e| e.sequence).max().unwrap_or(0);

        *self.sequence.write().await = max_sequence;

        info!("Event store initialized with sequence: {}", max_sequence);
        Ok(())
    }

    /// Append an event to the store
    pub async fn append_event(&self, event: RaidEvent) -> Result<EventRecord, AppError> {
        let mut sequence = self.sequence.write().await;
        *sequence += 1;

        let event_record = EventRecord {
            event_id: Uuid::new_v4(),
            sequence: *sequence,
            event: event.clone(),
            timestamp: Utc::now(),
        };

        // Append to event log file
        self.append_to_log(&event_record).await?;

        info!(
            "Event appended: sequence={}, type={:?}",
            event_record.sequence,
            std::mem::discriminant(&event)
        );

        Ok(event_record)
    }

    /// Load all events from storage
    pub async fn load_events(&self) -> Result<Vec<EventRecord>, AppError> {
        if !self.event_log_path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&self.event_log_path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to open event log: {}", e)))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to read event log: {}", e)))?;

        // Parse events (one per line, JSON)
        let mut events = Vec::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let event: EventRecord = serde_json::from_str(line)
                .map_err(|e| AppError::ConfigError(format!("Failed to parse event: {}", e)))?;
            events.push(event);
        }

        Ok(events)
    }

    /// Get events since a specific sequence number
    pub async fn get_events_since(
        &self,
        since_sequence: u64,
    ) -> Result<Vec<EventRecord>, AppError> {
        let all_events = self.load_events().await?;
        Ok(all_events
            .into_iter()
            .filter(|e| e.sequence > since_sequence)
            .collect())
    }

    /// Get events for a specific artifact
    pub async fn get_events_for_artifact(
        &self,
        artifact_id: &str,
    ) -> Result<Vec<EventRecord>, AppError> {
        let all_events = self.load_events().await?;
        Ok(all_events
            .into_iter()
            .filter(|e| match &e.event {
                RaidEvent::ArtifactCreated {
                    artifact_id: id, ..
                } => id == artifact_id,
                RaidEvent::ArtifactUpdated {
                    artifact_id: id, ..
                } => id == artifact_id,
                RaidEvent::ArtifactDeleted {
                    artifact_id: id, ..
                } => id == artifact_id,
                RaidEvent::ReplicationStarted {
                    artifact_id: id, ..
                } => id == artifact_id,
                RaidEvent::ReplicationCompleted {
                    artifact_id: id, ..
                } => id == artifact_id,
                _ => false,
            })
            .collect())
    }

    /// Get current sequence number
    pub async fn get_current_sequence(&self) -> u64 {
        *self.sequence.read().await
    }

    /// Append event to log file
    async fn append_to_log(&self, event: &EventRecord) -> Result<(), AppError> {
        let json = serde_json::to_string(event)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize event: {}", e)))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.event_log_path)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to open event log for writing: {}", e))
            })?;

        file.write_all(json.as_bytes())
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write event: {}", e)))?;

        file.write_all(b"\n")
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write newline: {}", e)))?;

        file.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to sync event log: {}", e)))?;

        Ok(())
    }

    /// Get event log path
    pub fn event_log_path(&self) -> &PathBuf {
        &self.event_log_path
    }

    /// Get snapshot path
    pub fn snapshot_path(&self) -> &PathBuf {
        &self.snapshot_path
    }

    /// Replay events to reconstruct state
    ///
    /// This method replays all events from the store, allowing
    /// state reconstruction from the event log.
    pub async fn replay_events<F>(&self, mut handler: F) -> Result<(), AppError>
    where
        F: FnMut(&EventRecord) -> Result<(), AppError>,
    {
        let events = self.load_events().await?;

        for event in events {
            handler(&event)?;
        }

        Ok(())
    }

    /// Get events in a time range
    pub async fn get_events_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<EventRecord>, AppError> {
        let all_events = self.load_events().await?;
        Ok(all_events
            .into_iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect())
    }

    /// Create a snapshot of current state
    ///
    /// This creates a snapshot that can be used for fast recovery
    /// without replaying all events from the beginning.
    pub async fn create_snapshot(
        &self,
        artifacts: &crate::raid::manifest::ArtifactManifest,
        nodes: &[crate::raid::RaidNode],
    ) -> Result<Snapshot, AppError> {
        let sequence = self.get_current_sequence().await;
        let timestamp = Utc::now();

        let artifacts_json = serde_json::to_value(artifacts)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize artifacts: {}", e)))?;

        let nodes_json = serde_json::to_value(nodes)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize nodes: {}", e)))?;

        let snapshot = Snapshot {
            sequence,
            timestamp,
            artifacts: artifacts_json,
            nodes: nodes_json,
        };

        // Save snapshot to file
        let snapshot_json = serde_json::to_string_pretty(&snapshot)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize snapshot: {}", e)))?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.snapshot_path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to open snapshot file: {}", e)))?;

        file.write_all(snapshot_json.as_bytes())
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to write snapshot: {}", e)))?;

        file.sync_all()
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to sync snapshot: {}", e)))?;

        info!(
            "Snapshot created: sequence={}, timestamp={}",
            sequence, timestamp
        );
        Ok(snapshot)
    }

    /// Load snapshot from storage
    pub async fn load_snapshot(&self) -> Result<Option<Snapshot>, AppError> {
        if !self.snapshot_path.exists() {
            return Ok(None);
        }

        let mut file = File::open(&self.snapshot_path)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to open snapshot file: {}", e)))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| AppError::ConfigError(format!("Failed to read snapshot: {}", e)))?;

        let snapshot: Snapshot = serde_json::from_str(&contents)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse snapshot: {}", e)))?;

        Ok(Some(snapshot))
    }

    /// Replay events since snapshot
    ///
    /// This method loads a snapshot and replays only events that occurred
    /// after the snapshot, allowing for fast state reconstruction.
    pub async fn replay_events_since_snapshot<F>(&self, mut handler: F) -> Result<u64, AppError>
    where
        F: FnMut(&EventRecord) -> Result<(), AppError>,
    {
        // Try to load snapshot
        let snapshot = self.load_snapshot().await?;
        let start_sequence = snapshot.as_ref().map(|s| s.sequence).unwrap_or(0);

        // Replay events since snapshot
        let events = self.get_events_since(start_sequence).await?;
        for event in events {
            handler(&event)?;
        }

        Ok(start_sequence)
    }
}
