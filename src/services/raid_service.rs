//! RAID-facing operations used by the HTTP API and future callers.

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::raid::events::{EventRecord, Snapshot};
use crate::raid::{ArtifactRef, RaidManager, RaidNode};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

/// User-visible message when RAID was not attached to [`crate::core::state::AppState`].
pub const RAID_MANAGER_UNAVAILABLE_MESSAGE: &str =
    "RAID manager not initialized. Suggestion: complete application startup (raid::initialize).";

/// Errors returned by [`RaidService`] (transport layer maps these to HTTP).
#[derive(Debug)]
pub enum RaidServiceError {
    ManagerUnavailable,
    ArtifactNotFound {
        id: Uuid,
    },
    WorkerNotFound {
        id: Uuid,
    },
    /// Event store missing (distributed RAID / event sourcing not wired).
    EventStoreUnavailable {
        operation: &'static str,
    },
    Operation(AppError),
}

fn require_raid_manager(ctx: &ApiContext) -> Result<Arc<RaidManager>, RaidServiceError> {
    ctx.raid_manager
        .get()
        .cloned()
        .ok_or(RaidServiceError::ManagerUnavailable)
}

fn map_delete_artifact_err(id: Uuid, e: AppError) -> RaidServiceError {
    let msg = e.to_string();
    if msg.contains("not found") {
        RaidServiceError::ArtifactNotFound { id }
    } else {
        RaidServiceError::Operation(e)
    }
}

/// JSON body for `GET /raid/quota`.
#[derive(Debug, Serialize)]
pub struct RaidQuotaResponse {
    pub total_size_bytes: u64,
    pub quota_bytes: Option<u64>,
    pub usage_percent: Option<f64>,
    pub artifact_count: usize,
}

#[derive(Debug, Serialize)]
pub struct RaidStorageStatus {
    pub total_size_bytes: u64,
    pub quota_bytes: Option<u64>,
    pub usage_percent: Option<f64>,
    pub available_bytes: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct RaftStatus {
    pub role: String,
    pub term: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RaidStatusResponse {
    pub cluster_status: String,
    pub node_count: usize,
    pub artifact_count: usize,
    pub storage: RaidStorageStatus,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raft_status: Option<RaftStatus>,
}

/// Thin orchestration over [`RaidManager`] for API use.
pub struct RaidService;

impl RaidService {
    pub async fn list_nodes(ctx: &ApiContext) -> Result<Vec<RaidNode>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        Ok(manager.list_nodes().await)
    }

    pub async fn list_artifacts(ctx: &ApiContext) -> Result<Vec<ArtifactRef>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        Ok(manager.list_artifacts().await)
    }

    pub async fn put_artifact(
        ctx: &ApiContext,
        name: &str,
        data: &[u8],
    ) -> Result<ArtifactRef, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .put_artifact(name, data)
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn delete_artifact(ctx: &ApiContext, id: Uuid) -> Result<(), RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .delete_artifact(id)
            .await
            .map_err(|e| map_delete_artifact_err(id, e))
    }

    pub async fn quota(ctx: &ApiContext) -> Result<RaidQuotaResponse, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let total_size = manager.get_total_size().await.unwrap_or(0);
        let artifacts = manager.list_artifacts().await;
        let artifact_count = artifacts.len();
        let quota_bytes = manager.get_quota_bytes().await;
        let usage_percent = quota_bytes.map(|quota| {
            if quota > 0 {
                (total_size as f64 / quota as f64) * 100.0
            } else {
                0.0
            }
        });
        Ok(RaidQuotaResponse {
            total_size_bytes: total_size,
            quota_bytes,
            usage_percent,
            artifact_count,
        })
    }

    pub async fn cluster_status(ctx: &ApiContext) -> Result<RaidStatusResponse, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let nodes = manager.list_nodes().await;
        let artifacts = manager.list_artifacts().await;
        let node_count = nodes.len();
        let artifact_count = artifacts.len();

        let total_size = manager.get_total_size().await.unwrap_or(0);
        let quota_bytes = manager.get_quota_bytes().await;
        let usage_percent = quota_bytes.map(|quota| {
            if quota > 0 {
                (total_size as f64 / quota as f64) * 100.0
            } else {
                0.0
            }
        });
        let available_bytes = quota_bytes.map(|quota| quota.saturating_sub(total_size));

        let mode = format!("{:?}", manager.get_mode().await);

        let cluster_status = if node_count == 0 {
            "unhealthy".to_string()
        } else if let Some(usage) = usage_percent {
            if usage >= 95.0 {
                "unhealthy".to_string()
            } else if usage >= 90.0 {
                "degraded".to_string()
            } else {
                "healthy".to_string()
            }
        } else {
            "healthy".to_string()
        };

        let replication_status = if mode != "Local" {
            Some("active".to_string())
        } else {
            None
        };

        // Placeholder until Raft status is wired to the manager.
        #[cfg(feature = "raft")]
        let raft_status: Option<RaftStatus> = None;

        #[cfg(not(feature = "raft"))]
        let raft_status: Option<RaftStatus> = None;

        Ok(RaidStatusResponse {
            cluster_status,
            node_count,
            artifact_count,
            storage: RaidStorageStatus {
                total_size_bytes: total_size,
                quota_bytes,
                usage_percent,
                available_bytes,
            },
            mode,
            replication_status,
            raft_status,
        })
    }

    pub async fn load_all_events(ctx: &ApiContext) -> Result<Vec<EventRecord>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let Some(es) = manager.event_store() else {
            return Err(RaidServiceError::EventStoreUnavailable {
                operation: "raid_events",
            });
        };
        let guard = es.read().await;
        guard
            .load_events()
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn load_events_for_artifact(
        ctx: &ApiContext,
        artifact_id: &str,
    ) -> Result<Vec<EventRecord>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let Some(es) = manager.event_store() else {
            return Err(RaidServiceError::EventStoreUnavailable {
                operation: "raid_events_for_artifact",
            });
        };
        let guard = es.read().await;
        guard
            .get_events_for_artifact(artifact_id)
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn load_events_in_range(
        ctx: &ApiContext,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<EventRecord>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let Some(es) = manager.event_store() else {
            return Err(RaidServiceError::EventStoreUnavailable {
                operation: "raid_events_range",
            });
        };
        let guard = es.read().await;
        guard
            .get_events_in_range(start, end)
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn load_snapshot(ctx: &ApiContext) -> Result<Option<Snapshot>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        let Some(es) = manager.event_store() else {
            return Err(RaidServiceError::EventStoreUnavailable {
                operation: "raid_snapshot_get",
            });
        };
        let guard = es.read().await;
        guard
            .load_snapshot()
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn create_snapshot(ctx: &ApiContext) -> Result<(), RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .create_snapshot()
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn restore_from_snapshot(ctx: &ApiContext) -> Result<(), RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .restore_from_snapshot()
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn gc_old_artifacts(ctx: &ApiContext) -> Result<usize, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .gc_old_artifacts()
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn list_workers(ctx: &ApiContext) -> Result<Vec<RaidNode>, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        Ok(manager.list_nodes().await)
    }

    pub async fn get_worker(ctx: &ApiContext, id: Uuid) -> Result<RaidNode, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .get_node(id)
            .await
            .ok_or(RaidServiceError::WorkerNotFound { id })
    }

    pub async fn register_worker(
        ctx: &ApiContext,
        address: String,
    ) -> Result<RaidNode, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        Ok(manager.register_node(address).await)
    }

    pub async fn update_worker(
        ctx: &ApiContext,
        id: Uuid,
        address: Option<String>,
    ) -> Result<RaidNode, RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .update_node(id, address)
            .await
            .map_err(RaidServiceError::Operation)
    }

    pub async fn delete_worker(ctx: &ApiContext, id: Uuid) -> Result<(), RaidServiceError> {
        let manager = require_raid_manager(ctx)?;
        manager
            .delete_node(id)
            .await
            .map_err(RaidServiceError::Operation)
    }
}
