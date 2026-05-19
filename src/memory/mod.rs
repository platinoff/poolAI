//! Memory layer wire types (AGI memory / RAID shards, P6 / Horizon S38).

mod map;
mod types;

pub use map::{
    envelope_from_memory_shard, memory_shard_from_envelope, memory_shard_from_grid_body,
    memory_shard_to_grid_body,
};
pub use types::{MemoryShardId, MemoryShardRef};
