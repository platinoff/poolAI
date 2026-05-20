//! Map Memory types ↔ Grid envelope / RAID naming.

use crate::grid::{GridEnvelope, GridMemoryShardBody, GridMessage};
use crate::memory::types::{MemoryShardId, MemoryShardRef};

pub fn memory_shard_to_grid_body(shard: &MemoryShardRef) -> GridMemoryShardBody {
    GridMemoryShardBody {
        shard_id: shard.shard_id.0.clone(),
        artifact_id: shard.artifact_id.clone(),
        version: shard.version.clone(),
        raid_logical_name: shard.raid_logical_name.clone(),
        seed_hints: shard.seed_hints.clone(),
    }
}

pub fn envelope_from_memory_shard(shard: &MemoryShardRef) -> GridEnvelope {
    GridEnvelope::new(
        GridMessage::MemoryShard(memory_shard_to_grid_body(shard)),
        None,
    )
}

pub fn memory_shard_from_grid_body(body: &GridMemoryShardBody) -> MemoryShardRef {
    MemoryShardRef {
        shard_id: MemoryShardId::new(body.shard_id.clone()),
        artifact_id: body.artifact_id.clone(),
        version: body.version.clone(),
        raid_logical_name: body.raid_logical_name.clone(),
        seed_hints: body.seed_hints.clone(),
    }
}

pub fn memory_shard_from_envelope(env: &GridEnvelope) -> Option<MemoryShardRef> {
    match &env.msg {
        GridMessage::MemoryShard(body) => Some(memory_shard_from_grid_body(body)),
        _ => None,
    }
}

/// Build shard ref from RAID logical name + artifact id (FM-022 RAID map).
pub fn memory_shard_from_raid(
    raid_logical_name: impl Into<String>,
    artifact_id: impl Into<String>,
    version: impl Into<String>,
) -> MemoryShardRef {
    let raid_logical_name = raid_logical_name.into();
    let version = version.into();
    MemoryShardRef {
        shard_id: MemoryShardId::new(format!("{raid_logical_name}:{version}")),
        artifact_id: artifact_id.into(),
        version,
        raid_logical_name: Some(raid_logical_name),
        seed_hints: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_shard_from_raid_builds_id() {
        let shard = memory_shard_from_raid("weights", "art-uuid", "1.0.0");
        assert_eq!(shard.shard_id.0, "weights:1.0.0");
        assert_eq!(shard.raid_logical_name.as_deref(), Some("weights"));
    }

    #[test]
    fn memory_shard_grid_round_trip() {
        let shard = MemoryShardRef {
            shard_id: MemoryShardId::new("weights:1.0.0"),
            artifact_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            version: "1.0.0".into(),
            raid_logical_name: Some("weights".into()),
            seed_hints: Some(vec!["seed".into()]),
        };
        let env = envelope_from_memory_shard(&shard);
        let back = memory_shard_from_envelope(&env).expect("memory_shard");
        assert_eq!(back, shard);
    }
}
