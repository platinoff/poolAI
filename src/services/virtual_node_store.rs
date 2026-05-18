//! File-backed store for virtual-node coordinator state (FM-016+).
//!
//! Set `POOLAI_VIRTUAL_NODE_DATA_DIR` (e.g. `data/virtual_nodes`) to persist bindings and
//! task queues across coordinator restarts. When unset, only in-memory state is used.

use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn data_dir() -> Option<PathBuf> {
    std::env::var("POOLAI_VIRTUAL_NODE_DATA_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

pub fn load_json(relative: &str) -> Result<Value, String> {
    let path = data_dir()
        .ok_or_else(|| "POOLAI_VIRTUAL_NODE_DATA_DIR not set".to_string())?
        .join(relative);
    if !path.exists() {
        return Ok(Value::Null);
    }
    let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))
}

pub fn save_json<T: Serialize>(relative: &str, value: &T) -> Result<(), String> {
    let dir = data_dir().ok_or_else(|| "POOLAI_VIRTUAL_NODE_DATA_DIR not set".to_string())?;
    let path = dir.join(relative);
    write_json_atomic(&path, value)
}

pub fn peer_tasks_path(peer_id: &str) -> Option<PathBuf> {
    data_dir().map(|d| d.join("tasks").join(format!("{peer_id}.json")))
}

pub fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| format!("serialize {}: {e}", path.display()))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, &data).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename {}: {e}", path.display()))
}
