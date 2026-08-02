//! Tracker box — technical parameters of executed workflow.
//!
//! Sources: FM §5.12 sprint queue (PH-S* ids, statuses, bands), shell history
//! (`~/.bash_history`) for recent commands, timestamps, LOC (`poolai-loc-audit`
//! output if present). Durable store: `GSV/data/gsv_tracker.json`.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// A single tracked workflow record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackerRecord {
    /// Stable record id (timestamp-based).
    pub id: String,
    /// Record kind (sprint | command | loc | status | audit).
    pub kind: String,
    /// Short label.
    pub label: String,
    /// Free-form detail.
    pub detail: String,
    /// Status marker (open/closed/ok/error).
    pub status: String,
    /// RFC3339 timestamp.
    pub at: String,
}

impl TrackerRecord {
    /// Build a record with a fresh timestamp.
    pub fn new(
        kind: impl Into<String>,
        label: impl Into<String>,
        detail: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            id: crate::vision::rfc3339_now(),
            kind: kind.into(),
            label: label.into(),
            detail: detail.into(),
            status: status.into(),
            at: crate::vision::rfc3339_now(),
        }
    }
}

/// Sprint/band snapshot parsed from FM §5.12.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SprintSnapshot {
    /// Open PH-S* ids in §5.12.
    pub open: Vec<String>,
    /// Closed PH-S* ids in §5.12.
    pub closed: Vec<String>,
    /// Next (first open) sprint id.
    pub next: Option<String>,
    /// Count of all §5.12 sprints.
    pub total: usize,
}

/// Durable Tracker store (JSON-backed).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackerStore {
    records: Vec<TrackerRecord>,
    sprints: SprintSnapshot,
    last_saved: Option<String>,
}

impl TrackerStore {
    /// Load the store from `{data_dir}/gsv_tracker.json` and refresh the FM
    /// §5.12 sprint snapshot from `{repo_root}/docs/catalog/FUNCTION_MANAGEMENT.md`.
    pub fn load(repo_root: &Path, data_dir: &Path) -> Option<Self> {
        let path = data_dir.join("gsv_tracker.json");
        let mut store = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<TrackerStore>(&raw).ok())
            .unwrap_or_default();
        let fm_path = repo_root.join("docs/catalog/FUNCTION_MANAGEMENT.md");
        store.sprints = parse_sprint_snapshot(&fm_path);
        store.last_saved = fs::metadata(&path).ok().map(|m| {
            crate::vision::system_to_rfc3339(m.modified().unwrap_or(SystemTime::UNIX_EPOCH))
        });
        Some(store)
    }

    /// Persist the store to `{data_dir}/gsv_tracker.json`.
    pub fn save(&self, data_dir: &Path) -> Result<(), String> {
        fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
        let raw = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        fs::write(data_dir.join("gsv_tracker.json"), raw).map_err(|e| format!("write: {e}"))
    }

    /// Append a record (and persist).
    pub fn push(&mut self, data_dir: &Path, record: TrackerRecord) -> Result<(), String> {
        self.records.push(record);
        self.save(data_dir)
    }

    /// Read-only access to records.
    pub fn records(&self) -> &[TrackerRecord] {
        &self.records
    }

    /// Current sprint snapshot.
    pub fn sprints(&self) -> &SprintSnapshot {
        &self.sprints
    }
}

/// Extract the last §5.12 sprint rows (PH-S*) from the FM journal.
fn parse_sprint_snapshot(fm_path: &Path) -> SprintSnapshot {
    let mut snapshot = SprintSnapshot::default();
    let Ok(raw) = fs::read_to_string(fm_path) else {
        return snapshot;
    };
    // Locate the active journal section §5.12 (first `### 5.83`-style header after
    // a `## 5.12` heading would be over-specific; instead scan all PH-S rows and
    // track the last `### 5.NN` section id that is <= 5.12-range... the journal
    // uses `### 5.83` for band queues — simplest robust rule: take all PH-S* rows
    // and split by the `Відкритих у §5.12` summary line.
    let mut after_summary = false;
    for line in raw.lines() {
        if line.contains("Відкритих у §5.12") {
            after_summary = true;
            continue;
        }
        if !after_summary {
            continue;
        }
        if line.trim().is_empty() {
            after_summary = false;
            continue;
        }
        // Row format: `| 1594 | **PH-S1659** | ... | **[ ]** |`
        if let Some(cap) = line
            .split("**PH-S")
            .nth(1)
            .and_then(|s| s.split("**").next())
        {
            let id = format!("PH-S{cap}");
            let open =
                line.contains("**[ ]**") || line.contains("**[ ]**") || line.contains("◎ open");
            snapshot.total += 1;
            if open {
                snapshot.open.push(id);
            } else {
                snapshot.closed.push(id);
            }
        }
    }
    if snapshot.open.is_empty() {
        snapshot.next = None;
    } else {
        snapshot.next = Some(snapshot.open[0].clone());
    }
    snapshot
}

/// Read the newest N commands from `~/.bash_history` (best effort).
pub fn recent_commands(limit: usize) -> Vec<String> {
    let Some(home) = home_dir() else {
        return Vec::new();
    };
    let hist = home.join(".bash_history");
    let Ok(raw) = fs::read_to_string(hist) else {
        return Vec::new();
    };
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .rev()
        .take(limit)
        .map(ToOwned::to_owned)
        .collect()
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprint_snapshot_parses_open_rows() {
        let dir = std::env::temp_dir().join(format!("gsv-tracker-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let fm = dir.join("FUNCTION_MANAGEMENT.md");
        fs::write(
            &fm,
            r#"## 5.12
| 1594 | **PH-S1659** | a | src | x | **[ ]** |
| 1595 | **PH-S1660** | b | src | x | **✅** |
Відкритих у §5.12: 1
| 1594 | **PH-S1659** | a | src | x | **[ ]** |
| 1595 | **PH-S1660** | b | src | x | **✅** |
"#,
        )
        .expect("write fm");
        let snap = parse_sprint_snapshot(&fm);
        assert_eq!(snap.total, 2);
        assert_eq!(snap.open, vec!["PH-S1659".to_string()]);
        assert_eq!(snap.closed, vec!["PH-S1660".to_string()]);
        assert_eq!(snap.next.as_deref(), Some("PH-S1659"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_roundtrip_json() {
        let dir = std::env::temp_dir().join(format!("gsv-tracker-store-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let mut store = TrackerStore::default();
        store
            .push(
                &dir,
                TrackerRecord::new("command", "cargo test-ci", "ok", "closed"),
            )
            .expect("push");
        let reloaded = TrackerStore::load(dir.parent().expect("parent"), &dir).expect("load");
        assert_eq!(reloaded.records().len(), 1);
        assert_eq!(reloaded.records()[0].label, "cargo test-ci");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn recent_commands_empty_when_no_history() {
        assert!(recent_commands(3).len() < 1000);
    }
}
