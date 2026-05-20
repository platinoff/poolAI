//! File-backed memory shard registry (FM-022). Set `POOLAI_MEMORY_DATA_DIR` (e.g. `data/memory`) to persist.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;
use crate::memory::MemoryShardRef;

const SHARDS_FILE: &str = "shards.json";

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct ShardsFile {
    shards: Vec<MemoryShardRef>,
}

/// In-process shard registry with optional JSON persistence.
pub struct MemoryShardStore {
    shards: Mutex<Vec<MemoryShardRef>>,
    data_dir: Option<PathBuf>,
}

impl MemoryShardStore {
    pub fn global() -> &'static MemoryShardStore {
        static STORE: LazyLock<MemoryShardStore> =
            LazyLock::new(|| MemoryShardStore::open(data_dir_from_env()));
        &STORE
    }

    #[cfg(test)]
    pub fn open_for_test(data_dir: Option<PathBuf>) -> Self {
        Self::open(data_dir)
    }

    fn open(data_dir: Option<PathBuf>) -> Self {
        let shards = data_dir
            .as_ref()
            .map(|d| d.join(SHARDS_FILE))
            .and_then(|p| load_shards_file(&p).ok())
            .unwrap_or_default();
        Self {
            shards: Mutex::new(shards),
            data_dir,
        }
    }

    pub fn list(&self) -> Result<Vec<MemoryShardRef>, AppError> {
        let guard = self
            .shards
            .lock()
            .map_err(|_| AppError::InternalError("memory store lock poisoned".into()))?;
        Ok(guard.clone())
    }

    pub fn list_by_raid_logical_name(
        &self,
        logical_name: &str,
    ) -> Result<Vec<MemoryShardRef>, AppError> {
        let guard = self
            .shards
            .lock()
            .map_err(|_| AppError::InternalError("memory store lock poisoned".into()))?;
        Ok(guard
            .iter()
            .filter(|s| {
                s.raid_logical_name
                    .as_deref()
                    .is_some_and(|n| n == logical_name)
            })
            .cloned()
            .collect())
    }

    pub fn get(&self, shard_id: &str) -> Result<Option<MemoryShardRef>, AppError> {
        let guard = self
            .shards
            .lock()
            .map_err(|_| AppError::InternalError("memory store lock poisoned".into()))?;
        Ok(guard.iter().find(|s| s.shard_id.0 == shard_id).cloned())
    }

    /// Insert or replace shard ref by `shard_id`.
    pub fn upsert(&self, shard: MemoryShardRef) -> Result<MemoryShardRef, AppError> {
        {
            let mut guard = self
                .shards
                .lock()
                .map_err(|_| AppError::InternalError("memory store lock poisoned".into()))?;
            if let Some(existing) = guard.iter_mut().find(|s| s.shard_id.0 == shard.shard_id.0) {
                *existing = shard.clone();
            } else {
                guard.push(shard.clone());
            }
        }
        self.persist()?;
        Ok(shard)
    }

    fn persist(&self) -> Result<(), AppError> {
        let Some(dir) = self.data_dir.as_ref() else {
            return Ok(());
        };
        let guard = self
            .shards
            .lock()
            .map_err(|_| AppError::InternalError("memory store lock poisoned".into()))?;
        let path = dir.join(SHARDS_FILE);
        let snapshot = ShardsFile {
            shards: guard.clone(),
        };
        write_json_atomic(&path, &snapshot)
            .map_err(|e| AppError::InternalError(format!("persist memory shards: {e}")))
    }
}

pub fn data_dir_from_env() -> Option<PathBuf> {
    std::env::var("POOLAI_MEMORY_DATA_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

fn load_shards_file(path: &Path) -> Result<Vec<MemoryShardRef>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: ShardsFile =
        serde_json::from_slice(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))?;
    Ok(file.shards)
}

fn write_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| format!("serialize {}: {e}", path.display()))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, &data).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{MemoryShardId, MemoryShardRef};
    use tempfile::TempDir;

    fn sample_shard(id: &str) -> MemoryShardRef {
        MemoryShardRef {
            shard_id: MemoryShardId::new(id),
            artifact_id: "art-1".into(),
            version: "1.0.0".into(),
            raid_logical_name: Some("weights".into()),
            seed_hints: None,
        }
    }

    #[test]
    fn upsert_and_reload() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();

        {
            let store = MemoryShardStore::open_for_test(Some(dir.clone()));
            store.upsert(sample_shard("w:1")).expect("upsert");
        }

        let reloaded = MemoryShardStore::open_for_test(Some(dir));
        let shard = reloaded.get("w:1").expect("get").expect("row");
        assert_eq!(shard.artifact_id, "art-1");
    }

    #[test]
    fn filter_by_raid_logical_name() {
        let store = MemoryShardStore::open_for_test(None);
        store.upsert(sample_shard("a:1")).expect("upsert");
        let mut other = sample_shard("b:1");
        other.raid_logical_name = Some("embeddings".into());
        store.upsert(other).expect("upsert");

        let weights = store.list_by_raid_logical_name("weights").expect("list");
        assert_eq!(weights.len(), 1);
        assert_eq!(weights[0].shard_id.0, "a:1");
    }
}
