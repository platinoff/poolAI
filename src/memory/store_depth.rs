//! Memory shard persistence depth classification stub (PH-S860, Memory band 21).

/// Memory shard registry persistence depth (FM-022 / `POOLAI_MEMORY_DATA_DIR`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStoreDepth {
    None,
    Ephemeral,
    JsonFile,
    JsonRestartPersist,
}

/// Classify memory store depth from persist flag and on-disk shard count (PH-S860).
pub fn memory_store_depth_stub(
    persist_enabled: bool,
    persisted_shard_count: u32,
) -> MemoryStoreDepth {
    if !persist_enabled {
        return MemoryStoreDepth::Ephemeral;
    }
    if persisted_shard_count > 0 {
        MemoryStoreDepth::JsonRestartPersist
    } else {
        MemoryStoreDepth::JsonFile
    }
}

/// Wire label for admin / seed-inventory depth fields (PH-S861).
pub fn memory_store_depth_wire_label(depth: MemoryStoreDepth) -> &'static str {
    match depth {
        MemoryStoreDepth::None => "none",
        MemoryStoreDepth::Ephemeral => "ephemeral",
        MemoryStoreDepth::JsonFile => "json",
        MemoryStoreDepth::JsonRestartPersist => "json_restart",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_depth_stub_ph_s860() {
        assert_eq!(
            memory_store_depth_stub(false, 0),
            MemoryStoreDepth::Ephemeral
        );
        assert_eq!(memory_store_depth_stub(true, 0), MemoryStoreDepth::JsonFile);
        assert_eq!(
            memory_store_depth_stub(true, 3),
            MemoryStoreDepth::JsonRestartPersist
        );
        assert_eq!(
            memory_store_depth_wire_label(MemoryStoreDepth::JsonRestartPersist),
            "json_restart"
        );
    }
}
