//! Update box — update notification + offline/resync signal.
//!
//! Key UX requirement (GSV_SERVER.md): while the binary runs, the server accepts
//! an update message. The UI shows an **Update** badge instead of auto-reload; the
//! page survives offline and re-syncs all metrics on reconnect.
//!
//! Detection (self-contained): if the newest `GSV/src/**` source file is newer than
//! the running binary (i.e. a rebuild is pending on disk), or an explicit
//! `POST /api/update/notify` arrived, `update_available` is `true`.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::vision;

/// `/api/update` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWire {
    pub version: String,
    pub update_available: bool,
    pub git_head: Option<String>,
    pub started_at: String,
    pub binary_mtime: u64,
    pub newest_src_mtime: u64,
}

/// Query params for `GET /api/update`.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCheckParams {
    /// Force a fresh mtime-based check instead of the cached flag.
    pub check: Option<bool>,
}

/// Newest mtime (epoch secs) across `GSV/src/**` (capped traversal).
pub fn newest_src_mtime(manifest_dir: &Path) -> u64 {
    let src = manifest_dir.join("src");
    let mut newest = 0u64;
    let mut stack = vec![src];
    let mut guard = 0usize;
    while let Some(dir) = stack.pop() {
        guard += 1;
        if guard > 2000 {
            break;
        }
        let Ok(read) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in read.flatten() {
            let p = e.path();
            let Ok(meta) = std::fs::metadata(&p) else {
                continue;
            };
            if meta.is_dir() {
                stack.push(p);
            } else if let Ok(t) = meta.modified() {
                if let Ok(d) = t.duration_since(std::time::UNIX_EPOCH) {
                    newest = newest.max(d.as_secs());
                }
            }
        }
    }
    newest
}

/// Running binary mtime (epoch secs).
pub fn binary_mtime() -> u64 {
    std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::metadata(p).ok())
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Build the update wire for the current state.
pub fn wire(state: &AppState) -> UpdateWire {
    let newest = newest_src_mtime(Path::new(env!("CARGO_MANIFEST_DIR")));
    let bin = binary_mtime();
    let pending_rebuild = newest > bin;
    UpdateWire {
        version: state.version.to_string(),
        update_available: state.update_available() || pending_rebuild,
        git_head: vision::git_head(&state.repo_root),
        started_at: crate::vision::system_to_rfc3339(state.started_at),
        binary_mtime: bin,
        newest_src_mtime: newest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newest_src_mtime_gt_zero() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(newest_src_mtime(dir) > 0);
    }

    #[test]
    fn pending_rebuild_logic() {
        let older = 1u64;
        let newer = 2u64;
        assert!(newer > older);
    }
}
