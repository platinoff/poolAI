//! Dev stand last-run snapshot (`data/dev/last_run.json`, PH-S1014).

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Persisted launch parameters for `run-poolai quick` restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastRunSnapshot {
    pub preset: String,
    pub port: u16,
    pub features: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_store: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
    pub saved_at: String,
}

impl LastRunSnapshot {
    pub fn new(
        preset: impl Into<String>,
        port: u16,
        features: impl Into<String>,
        job_store: Option<String>,
        pid: Option<u32>,
    ) -> Self {
        Self {
            preset: preset.into(),
            port,
            features: features.into(),
            job_store,
            pid,
            saved_at: iso_now(),
        }
    }
}

/// Default path relative to repo / data root.
pub fn default_last_run_path(data_root: impl AsRef<Path>) -> PathBuf {
    data_root.as_ref().join("dev").join("last_run.json")
}

pub fn save_last_run(path: impl AsRef<Path>, snapshot: &LastRunSnapshot) -> std::io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(snapshot).map_err(std::io::Error::other)?;
    fs::write(path, json)
}

pub fn load_last_run(path: impl AsRef<Path>) -> std::io::Result<Option<LastRunSnapshot>> {
    let path = path.as_ref();
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)?;
    let snapshot: LastRunSnapshot = serde_json::from_str(&raw).map_err(std::io::Error::other)?;
    Ok(Some(snapshot))
}

fn iso_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn last_run_roundtrip_ph_s1014() {
        let dir = env::temp_dir().join(format!("poolai-last-run-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("last_run.json");
        let snap = LastRunSnapshot::new("quick", 8080, "enterprise,test-utils", None, Some(4242));
        save_last_run(&path, &snap).unwrap();
        let loaded = load_last_run(&path).unwrap().expect("snapshot");
        assert_eq!(loaded, snap);
        assert_eq!(loaded.preset, "quick");
        assert_eq!(loaded.port, 8080);
    }
}
