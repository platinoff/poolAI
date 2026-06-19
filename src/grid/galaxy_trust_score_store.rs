//! Per-peer `trust_score` disk persistence stub (PH-S552, Galaxy §6.5).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::grid::galaxy_trust_score::TrustScore;

fn trust_map() -> &'static Mutex<HashMap<String, TrustScore>> {
    static TRUST_BY_PEER: OnceLock<Mutex<HashMap<String, TrustScore>>> = OnceLock::new();
    TRUST_BY_PEER.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Env: optional JSON file path for trust score persistence.
pub const ENV_TRUST_SCORE_STORE_PATH: &str = "POOLAI_TRUST_SCORE_STORE_PATH";

fn store_path() -> Option<PathBuf> {
    std::env::var(ENV_TRUST_SCORE_STORE_PATH)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

fn flush_to_disk(map: &HashMap<String, TrustScore>) {
    let Some(path) = store_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(map) {
        let _ = fs::write(path, json);
    }
}

fn load_from_disk() {
    let Some(path) = store_path() else {
        return;
    };
    if !Path::new(&path).exists() {
        return;
    }
    if let Ok(text) = fs::read_to_string(path) {
        if let Ok(map) = serde_json::from_str::<HashMap<String, TrustScore>>(&text) {
            if let Ok(mut guard) = trust_map().lock() {
                *guard = map;
            }
        }
    }
}

/// Persist peer trust score after grid result ingest.
pub fn persist_peer_trust_score(peer_id: &str, score: TrustScore) {
    if peer_id.trim().is_empty() {
        return;
    }
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
    load_from_disk();
    if metadata.contains_key("trust_score") {
        return;
    }
    if let Ok(map) = trust_map().lock() {
        if let Some(score) = map.get(peer_id) {
            metadata.insert("trust_score".to_string(), score.to_string());
        }
    }
}

/// Lookup stored trust score for a peer (tests / admin).
pub fn lookup_peer_trust_score(peer_id: &str) -> Option<TrustScore> {
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
}
