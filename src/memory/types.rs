//! Memory layer wire types (P6 / Horizon S38).

use serde::{Deserialize, Serialize};

/// Shard identifier (often `{logical_name}:{version}`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MemoryShardId(pub String);

impl MemoryShardId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Reference to a memory shard backed by a RAID artifact (POOLAI_MEMORY_LAYER).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryShardRef {
    pub shard_id: MemoryShardId,
    pub artifact_id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_logical_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_hints: Option<Vec<String>>,
}
