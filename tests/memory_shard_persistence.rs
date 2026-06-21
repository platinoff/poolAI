//! PH-S860: Memory shard JSON persistence reload integration.

#![cfg(feature = "test-utils")]

use poolai::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};
use tempfile::TempDir;

fn sample_shard(id: &str) -> MemoryShardRef {
    MemoryShardRef {
        shard_id: MemoryShardId::new(id),
        artifact_id: "art-mem-1".into(),
        version: "1.0.0".into(),
        raid_logical_name: Some("weights".into()),
        seed_hints: None,
    }
}

#[test]
fn memory_shard_persists_across_reload_ph_s860() {
    let tmp = TempDir::new().expect("tempdir");
    let data_dir = tmp.path().to_path_buf();
    std::env::set_var("POOLAI_MEMORY_DATA_DIR", &data_dir);

    {
        let store = MemoryShardStore::open_for_test(Some(data_dir.clone()));
        assert!(store.persist_enabled());
        store.upsert(sample_shard("w:emb-s860")).expect("upsert");
    }

    let reloaded = MemoryShardStore::open_for_test(Some(data_dir));
    assert!(reloaded.persist_enabled());
    assert_eq!(reloaded.registered_shard_count().expect("count"), 1);
    let shard = reloaded.get("w:emb-s860").expect("get").expect("row");
    assert_eq!(shard.artifact_id, "art-mem-1");

    std::env::remove_var("POOLAI_MEMORY_DATA_DIR");
}
