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

#[cfg(test)]
mod tests {
    use super::*;

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
