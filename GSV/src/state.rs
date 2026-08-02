//! GSV shared application state.
//!
//! `AppState` is handed to every axum handler via `State<AppState>`. It carries:
//! - repo root + data dir paths (repo defaults to the PoolAI root, sibling of `GSV/`)
//! - durable Tracker store (`Arc<RwLock<TrackerStore>>`)
//! - IDE session selection (in-memory)
//! - update flag (`Arc<AtomicBool>`) + build metadata
//! - SSE event broadcast sender (`/events`)

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::{broadcast, RwLock};

use crate::tracker::TrackerStore;

/// Shared application state for the GSV server.
#[derive(Clone)]
pub struct AppState {
    /// Repo root (PoolAI root, parent of `GSV/`).
    pub repo_root: Arc<PathBuf>,
    /// Durable data directory (`GSV/data/`).
    pub data_dir: Arc<PathBuf>,
    /// Server build version (`CARGO_PKG_VERSION`).
    pub version: Arc<str>,
    /// Process start time (health/uptime).
    pub started_at: SystemTime,
    /// Tracker box durable store.
    pub tracker: Arc<RwLock<TrackerStore>>,
    /// Currently selected IDE session (in-memory selection).
    pub ide_selection: Arc<RwLock<Option<crate::boxes::ide::IdeSelection>>>,
    /// `true` once an update notification has been received.
    pub update_flag: Arc<AtomicBool>,
    /// SSE event broadcast channel (string payloads, JSON).
    pub events: broadcast::Sender<String>,
}

impl AppState {
    /// Build a new `AppState`.
    ///
    /// `repo_root` defaults to the parent of the `GSV/` manifest dir when `None`.
    /// `data_dir` defaults to `{repo_root}/GSV/data` when `None`.
    pub fn new(
        repo_root: Option<PathBuf>,
        data_dir: Option<PathBuf>,
        events: broadcast::Sender<String>,
    ) -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = repo_root.unwrap_or_else(|| {
            manifest_dir
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| manifest_dir.clone())
        });
        let data = data_dir.unwrap_or_else(|| root.join("GSV").join("data"));
        let tracker = TrackerStore::load(&root, &data).unwrap_or_default();
        Self {
            repo_root: Arc::new(root),
            data_dir: Arc::new(data),
            version: Arc::from(crate::gsv_version()),
            started_at: SystemTime::now(),
            tracker: Arc::new(RwLock::new(tracker)),
            ide_selection: Arc::new(RwLock::new(None)),
            update_flag: Arc::new(AtomicBool::new(false)),
            events,
        }
    }

    /// Reset the update flag (used after a UI "Update" handshake).
    pub fn clear_update(&self) {
        self.update_flag.store(false, Ordering::SeqCst);
    }

    /// Read the update flag.
    pub fn update_available(&self) -> bool {
        self.update_flag.load(Ordering::SeqCst)
    }

    /// Emit an SSE event to all connected `/events` clients.
    pub fn emit(&self, event: impl Into<String>) {
        let _ = self.events.send(event.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> AppState {
        let (tx, _rx) = broadcast::channel(16);
        AppState::new(None, None, tx)
    }

    #[test]
    fn default_repo_root_is_poolai_root() {
        let s = state();
        assert!(s.repo_root.ends_with("poolAI"));
        assert!(s.data_dir.ends_with("poolAI/GSV/data"));
    }

    #[test]
    fn update_flag_toggle() {
        let s = state();
        assert!(!s.update_available());
        s.update_flag.store(true, Ordering::SeqCst);
        assert!(s.update_available());
        s.clear_update();
        assert!(!s.update_available());
    }
}
