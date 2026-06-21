//! Memory layer depth classification stub (PH-S864, POOLAI_MEMORY_LAYER band 21).

use crate::memory::store_depth::{memory_store_depth_stub, MemoryStoreDepth};

/// Memory layer production depth (registry + seed inventory wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryLayerDepth {
    None,
    RegistryEphemeral,
    PersistJson,
    SeedInventoryWire,
    FullDepth,
}

/// Classify memory layer depth from persist, registry count, and seed peer rows (PH-S864).
pub fn memory_layer_depth_stub(
    persist_enabled: bool,
    registered_shard_count: u32,
    seed_peer_count: u32,
) -> MemoryLayerDepth {
    let store = memory_store_depth_stub(persist_enabled, registered_shard_count);
    let has_seed_wire = seed_peer_count > 0;
    let has_registry = registered_shard_count > 0;

    match (store, has_seed_wire, has_registry) {
        (MemoryStoreDepth::JsonRestartPersist, true, true) => MemoryLayerDepth::FullDepth,
        (_, true, _) => MemoryLayerDepth::SeedInventoryWire,
        (MemoryStoreDepth::JsonFile | MemoryStoreDepth::JsonRestartPersist, _, _) => {
            MemoryLayerDepth::PersistJson
        }
        (MemoryStoreDepth::Ephemeral, _, true) => MemoryLayerDepth::RegistryEphemeral,
        _ => MemoryLayerDepth::None,
    }
}

/// Wire label for seed-inventory / admin depth strip (PH-S861).
pub fn memory_layer_depth_wire_label(depth: MemoryLayerDepth) -> &'static str {
    match depth {
        MemoryLayerDepth::None => "none",
        MemoryLayerDepth::RegistryEphemeral => "registry_ephemeral",
        MemoryLayerDepth::PersistJson => "persist_json",
        MemoryLayerDepth::SeedInventoryWire => "seed_inventory_wire",
        MemoryLayerDepth::FullDepth => "full_depth",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_layer_depth_stub_ph_s864() {
        assert_eq!(memory_layer_depth_stub(false, 0, 0), MemoryLayerDepth::None);
        assert_eq!(
            memory_layer_depth_stub(false, 2, 0),
            MemoryLayerDepth::RegistryEphemeral
        );
        assert_eq!(
            memory_layer_depth_stub(true, 0, 2),
            MemoryLayerDepth::SeedInventoryWire
        );
        assert_eq!(
            memory_layer_depth_stub(true, 1, 0),
            MemoryLayerDepth::PersistJson
        );
        assert_eq!(
            memory_layer_depth_stub(true, 2, 2),
            MemoryLayerDepth::FullDepth
        );
    }
}
