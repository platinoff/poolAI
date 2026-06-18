//! Galaxy `network_profile` disk persistence stub (PH-S489, §8.1).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;

const PROFILES_FILE: &str = "network_profiles.json";

/// Env: directory for persisted peer `network_profile` JSON (Galaxy §8.1).
pub const ENV_NETWORK_PROFILE_DATA_DIR: &str = "POOLAI_GALAXY_NETWORK_PROFILE_DATA_DIR";

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct ProfilesFile {
    profiles: HashMap<String, String>,
}

fn data_dir_from_env() -> Option<PathBuf> {
    std::env::var(ENV_NETWORK_PROFILE_DATA_DIR)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

fn load_profiles_file(path: &Path) -> Result<HashMap<String, String>, AppError> {
    let raw = fs::read_to_string(path)
        .map_err(|e| AppError::InternalError(format!("network_profile store read: {e}")))?;
    let file: ProfilesFile = serde_json::from_str(&raw)
        .map_err(|e| AppError::InternalError(format!("network_profile store parse: {e}")))?;
    Ok(file.profiles)
}

fn persist_profiles(data_dir: &Path, profiles: &HashMap<String, String>) -> Result<(), AppError> {
    fs::create_dir_all(data_dir)
        .map_err(|e| AppError::InternalError(format!("network_profile store mkdir: {e}")))?;
    let path = data_dir.join(PROFILES_FILE);
    let file = ProfilesFile {
        profiles: profiles.clone(),
    };
    let raw = serde_json::to_string_pretty(&file)
        .map_err(|e| AppError::InternalError(format!("network_profile store serialize: {e}")))?;
    fs::write(&path, raw)
        .map_err(|e| AppError::InternalError(format!("network_profile store write: {e}")))?;
    Ok(())
}

struct NetworkProfileStoreInner {
    profiles: HashMap<String, String>,
    data_dir: Option<PathBuf>,
}

impl NetworkProfileStoreInner {
    fn open(data_dir: Option<PathBuf>) -> Self {
        let profiles = data_dir
            .as_ref()
            .map(|d| d.join(PROFILES_FILE))
            .and_then(|p| load_profiles_file(&p).ok())
            .unwrap_or_default();
        Self { profiles, data_dir }
    }

    fn upsert(&mut self, peer_id: &str, canonical_json: &str) -> Result<(), AppError> {
        self.profiles
            .insert(peer_id.to_string(), canonical_json.to_string());
        if let Some(dir) = self.data_dir.as_ref() {
            persist_profiles(dir, &self.profiles)?;
        }
        Ok(())
    }

    fn get(&self, peer_id: &str) -> Option<String> {
        self.profiles.get(peer_id).cloned()
    }
}

static STORE: LazyLock<Mutex<NetworkProfileStoreInner>> =
    LazyLock::new(|| Mutex::new(NetworkProfileStoreInner::open(data_dir_from_env())));

/// Persist canonical `network_profile` JSON for a peer (PH-S489).
pub fn persist_peer_network_profile(peer_id: &str, canonical_json: &str) -> Result<(), AppError> {
    let mut guard = STORE
        .lock()
        .map_err(|_| AppError::InternalError("network_profile store lock poisoned".into()))?;
    guard.upsert(peer_id, canonical_json)
}

/// Load persisted `network_profile` JSON for a peer.
pub fn load_peer_network_profile(peer_id: &str) -> Option<String> {
    STORE.lock().ok().and_then(|g| g.get(peer_id))
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_network_profile_store_for_test() {
    if let Ok(mut guard) = STORE.lock() {
        guard.profiles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn network_profile_persist_roundtrip_ph_s489() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("poolai-np-{stamp}"));
        std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
        reset_network_profile_store_for_test();
        let inner = NetworkProfileStoreInner::open(data_dir_from_env());
        {
            let mut guard = STORE.lock().unwrap();
            *guard = inner;
        }
        let json = r#"{"region":"eu-west","latency_ms_p50":12}"#;
        persist_peer_network_profile("peer-a", json).expect("persist");
        assert_eq!(load_peer_network_profile("peer-a").as_deref(), Some(json));
        let inner2 = NetworkProfileStoreInner::open(data_dir_from_env());
        assert_eq!(inner2.get("peer-a").as_deref(), Some(json));
        let _ = fs::remove_dir_all(&dir);
        std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
        reset_network_profile_store_for_test();
    }
}
