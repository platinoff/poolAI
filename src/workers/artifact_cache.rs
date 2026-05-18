//! Local artifact probe cache on virtual-node device (FM-016+++).
//!
//! Set `POOLAI_WORKER_CACHE_DIR` to persist bytes sent via `raid_artifact_probe`.

use std::path::{Path, PathBuf};

const PROBE_SUBDIR: &str = "artifacts";

/// Resolved cache root from `POOLAI_WORKER_CACHE_DIR`, if set.
pub fn resolve_cache_dir() -> Option<PathBuf> {
    std::env::var("POOLAI_WORKER_CACHE_DIR")
        .ok()
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}

fn sanitize_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "probe".to_string()
    } else {
        out
    }
}

fn artifacts_dir(cache_root: &Path) -> PathBuf {
    cache_root.join(PROBE_SUBDIR)
}

/// Write probe bytes under `{cache}/artifacts/{name}-{unix_ms}.bin`.
pub fn store_probe(cache_root: &Path, logical_name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    let dir = artifacts_dir(cache_root);
    std::fs::create_dir_all(&dir).map_err(|e| format!("create cache dir: {e}"))?;
    let safe = sanitize_name(logical_name);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let path = dir.join(format!("{safe}-{ts}.bin"));
    std::fs::write(&path, bytes).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

/// Count `*.bin` files in the cache artifacts subdirectory.
pub fn count_cached_probes(cache_root: &Path) -> usize {
    let dir = artifacts_dir(cache_root);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return 0;
    };
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "bin"))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn store_and_count_probe_files() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();
        let p1 = store_probe(root, "vn-probe", b"abc").expect("store");
        assert!(p1.exists());
        store_probe(root, "other", b"xyz").expect("store2");
        assert_eq!(count_cached_probes(root), 2);
    }
}
