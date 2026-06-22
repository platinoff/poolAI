//! Per-peer `trust_score` disk persistence (PH-S552 JSON, PH-S910 SQLite, Galaxy §6.5).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::grid::galaxy_trust_score::{
    apply_trust_delta, TrustScore, DEFAULT_TRUST_SCORE, TRUST_DELTA_LEASE_EPOCH_REJECTED,
    TRUST_DELTA_WORKER_UNHEALTHY,
};
use crate::grid::galaxy_trust_score_store_sqlite::{self, migrate_legacy_json_file};

fn trust_map() -> &'static Mutex<HashMap<String, TrustScore>> {
    static TRUST_BY_PEER: OnceLock<Mutex<HashMap<String, TrustScore>>> = OnceLock::new();
    TRUST_BY_PEER.get_or_init(|| Mutex::new(HashMap::new()))
}

static DISK_LOADED: OnceLock<Mutex<bool>> = OnceLock::new();

fn disk_loaded_flag() -> &'static Mutex<bool> {
    DISK_LOADED.get_or_init(|| Mutex::new(false))
}

/// Trust score persistence backend (PH-S910).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustStoreBackend {
    Ephemeral,
    Json,
    Sqlite,
}

/// Env: optional JSON file path for trust score persistence (legacy single-file).
pub const ENV_TRUST_SCORE_STORE_PATH: &str = "POOLAI_TRUST_SCORE_STORE_PATH";

fn json_store_path() -> Option<PathBuf> {
    std::env::var(ENV_TRUST_SCORE_STORE_PATH)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// Active trust store backend from env (PH-S910).
pub fn current_trust_store_backend() -> TrustStoreBackend {
    if galaxy_trust_score_store_sqlite::sqlite_enabled()
        && galaxy_trust_score_store_sqlite::data_dir_from_env().is_some()
    {
        TrustStoreBackend::Sqlite
    } else if json_store_path().is_some() {
        TrustStoreBackend::Json
    } else {
        TrustStoreBackend::Ephemeral
    }
}

/// Wire label for admin / trust-metrics depth fields (PH-S913).
pub fn trust_store_backend_wire_label(backend: TrustStoreBackend) -> &'static str {
    match backend {
        TrustStoreBackend::Ephemeral => "ephemeral",
        TrustStoreBackend::Json => "json",
        TrustStoreBackend::Sqlite => "sqlite",
    }
}

/// Count of peers with stored trust scores in the in-process map (PH-S914).
pub fn persisted_trust_peer_count() -> u32 {
    trust_map()
        .lock()
        .ok()
        .map(|map| map.len() as u32)
        .unwrap_or(0)
}

fn flush_to_disk(map: &HashMap<String, TrustScore>) {
    match current_trust_store_backend() {
        TrustStoreBackend::Ephemeral => {}
        TrustStoreBackend::Json => {
            let Some(path) = json_store_path() else {
                return;
            };
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(map) {
                let _ = fs::write(path, json);
            }
        }
        TrustStoreBackend::Sqlite => {
            if let Some(dir) = galaxy_trust_score_store_sqlite::data_dir_from_env() {
                let _ = galaxy_trust_score_store_sqlite::persist(&dir, map);
            }
        }
    }
}

fn load_from_disk_once() {
    let mut loaded = disk_loaded_flag().lock().unwrap_or_else(|e| e.into_inner());
    if *loaded {
        return;
    }
    match current_trust_store_backend() {
        TrustStoreBackend::Ephemeral => {}
        TrustStoreBackend::Json => {
            if let Some(path) = json_store_path() {
                if Path::new(&path).exists() {
                    if let Ok(text) = fs::read_to_string(path) {
                        if let Ok(map) = serde_json::from_str::<HashMap<String, TrustScore>>(&text)
                        {
                            if let Ok(mut guard) = trust_map().lock() {
                                *guard = map;
                            }
                        }
                    }
                }
            }
        }
        TrustStoreBackend::Sqlite => {
            if let Some(dir) = galaxy_trust_score_store_sqlite::data_dir_from_env() {
                if let Some(legacy) = json_store_path() {
                    let _ = migrate_legacy_json_file(&dir, &legacy);
                }
                if let Ok(map) = galaxy_trust_score_store_sqlite::load(&dir) {
                    if let Ok(mut guard) = trust_map().lock() {
                        *guard = map;
                    }
                }
            }
        }
    }
    *loaded = true;
}

/// Persist peer trust score after grid result ingest.
pub fn persist_peer_trust_score(peer_id: &str, score: TrustScore) {
    if peer_id.trim().is_empty() {
        return;
    }
    load_from_disk_once();
    if let Ok(mut map) = trust_map().lock() {
        map.insert(peer_id.trim().to_string(), score);
        flush_to_disk(&map);
    }
}

/// Hydrate register-remote metadata with stored trust score when absent.
pub fn hydrate_register_metadata_trust_score(
    peer_id: &str,
    metadata: &mut HashMap<String, String>,
) {
    load_from_disk_once();
    if metadata.contains_key("trust_score") {
        return;
    }
    if let Ok(map) = trust_map().lock() {
        if let Some(score) = map.get(peer_id) {
            metadata.insert("trust_score".to_string(), score.to_string());
        }
    }
}

/// Apply trust delta to stored peer score (or default) and persist (PH-S610 / PH-S611).
pub fn apply_peer_trust_delta(peer_id: &str, delta: i16) -> TrustScore {
    let peer = peer_id.trim();
    if peer.is_empty() {
        return DEFAULT_TRUST_SCORE;
    }
    load_from_disk_once();
    let current = lookup_peer_trust_score(peer).unwrap_or(DEFAULT_TRUST_SCORE);
    let adjusted = apply_trust_delta(current, delta);
    persist_peer_trust_score(peer, adjusted);
    adjusted
}

/// Trust delta on stale-epoch grid result reject (PH-S610).
pub fn apply_lease_epoch_rejected_trust_delta(peer_id: &str) -> TrustScore {
    apply_peer_trust_delta(peer_id, TRUST_DELTA_LEASE_EPOCH_REJECTED)
}

/// Trust delta when worker newly marked unhealthy (PH-S611).
pub fn apply_worker_unhealthy_trust_delta(peer_id: &str) -> TrustScore {
    apply_peer_trust_delta(peer_id, TRUST_DELTA_WORKER_UNHEALTHY)
}

/// Lookup stored trust score for a peer (tests / admin / payout gate PH-S911).
pub fn lookup_peer_trust_score(peer_id: &str) -> Option<TrustScore> {
    load_from_disk_once();
    trust_map()
        .lock()
        .ok()
        .and_then(|map| map.get(peer_id).copied())
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_trust_score_store_for_test() {
    if let Ok(mut map) = trust_map().lock() {
        map.clear();
    }
    if let Ok(mut loaded) = disk_loaded_flag().lock() {
        *loaded = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn trust_score_persist_roundtrip_ph_s552() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_trust_score_store_for_test();
        std::env::remove_var(galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_STORE);
        std::env::remove_var(galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_DATA_DIR);
        let tmp = TempDir::new().expect("tempdir");
        let path = tmp.path().join("trust_scores.json");
        std::env::set_var(ENV_TRUST_SCORE_STORE_PATH, path.to_string_lossy().as_ref());
        persist_peer_trust_score("tg-edge-1", 72);
        reset_trust_score_store_for_test();
        let mut meta = HashMap::new();
        hydrate_register_metadata_trust_score("tg-edge-1", &mut meta);
        assert_eq!(meta.get("trust_score").map(String::as_str), Some("72"));
        std::env::remove_var(ENV_TRUST_SCORE_STORE_PATH);
        reset_trust_score_store_for_test();
    }

    #[test]
    fn trust_score_sqlite_persist_roundtrip_ph_s910() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_trust_score_store_for_test();
        std::env::remove_var(ENV_TRUST_SCORE_STORE_PATH);
        let tmp = TempDir::new().expect("tempdir");
        std::env::set_var(
            galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_STORE,
            "sqlite",
        );
        std::env::set_var(
            galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_DATA_DIR,
            tmp.path().to_string_lossy().as_ref(),
        );
        persist_peer_trust_score("tg-sqlite-1", 65);
        reset_trust_score_store_for_test();
        let mut meta = HashMap::new();
        hydrate_register_metadata_trust_score("tg-sqlite-1", &mut meta);
        assert_eq!(meta.get("trust_score").map(String::as_str), Some("65"));
        assert_eq!(current_trust_store_backend(), TrustStoreBackend::Sqlite);
        std::env::remove_var(galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_STORE);
        std::env::remove_var(galaxy_trust_score_store_sqlite::ENV_TRUST_SCORE_DATA_DIR);
        reset_trust_score_store_for_test();
    }
}
