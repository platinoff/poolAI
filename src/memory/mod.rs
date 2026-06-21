//! Memory layer wire types (AGI memory / RAID shards, P6 / Horizon S38).

mod layer_depth;
mod map;
mod store;
mod store_depth;
mod types;

pub use layer_depth::{memory_layer_depth_stub, memory_layer_depth_wire_label, MemoryLayerDepth};
pub use map::{
    envelope_from_memory_shard, memory_shard_from_envelope, memory_shard_from_grid_body,
    memory_shard_from_raid, memory_shard_to_grid_body,
};
pub use store::{data_dir_from_env, MemoryShardStore};
pub use store_depth::{memory_store_depth_stub, memory_store_depth_wire_label, MemoryStoreDepth};
pub use types::{MemoryShardId, MemoryShardRef};
