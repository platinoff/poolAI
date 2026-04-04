//! RAID-facing operations used by the HTTP API and future callers.

use crate::core::state::ApiContext;
use crate::raid::{ArtifactRef, RaidManager, RaidNode};
use std::sync::Arc;

/// User-visible message when RAID was not attached to [`crate::core::state::AppState`].
pub const RAID_MANAGER_UNAVAILABLE_MESSAGE: &str =
    "RAID manager not initialized. Suggestion: complete application startup (raid::initialize).";

/// Errors returned by [`RaidService`] (transport layer maps these to HTTP).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaidServiceError {
    ManagerUnavailable,
}

fn require_raid_manager(ctx: &ApiContext) -> Result<Arc<RaidManager>, RaidServiceError> {
    ctx.raid_manager
        .get()
        .cloned()
        .ok_or(RaidServiceError::ManagerUnavailable)
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
}
