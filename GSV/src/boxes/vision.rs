//! Vision box — GSV mirror of the poolAI vision canon (`docs/vision/`).
//!
//! Ports `docs/vision/manifest.json` (galaxy graph: layers, nodes, edges) and
//! `docs/vision/feed.json` (sprint ticker) into `GSV/data/` and serves them via
//! `/api/vision/*`. The `gsv-vision-sync` bin is the write/drift gate.
//!
//! ```text
//! cargo run --bin gsv-vision-sync                  # write GSV/data/gsv_*.json
//! cargo run --bin gsv-vision-sync -- --check       # drift gate, no write
//! ```

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Galaxy graph node (`manifest.json` → `nodes[]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestNode {
    pub id: String,
    pub label: String,
    pub layer: String,
    pub path: String,
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub sprints: Vec<String>,
}

/// Galaxy layer (`manifest.json` → `layers[]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub z: i64,
}

/// Galaxy graph edge (`manifest.json` → `edges[]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestEdge {
    pub from: String,
    pub kind: String,
    pub to: String,
}

/// Sprint queue entry in the manifest (mirrors the feed ticker).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintQueueEntry {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub category: String,
}

/// Galaxy vision manifest (subset mirrored by GSV; unknown fields are ignored).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub auto_sync_at: String,
    #[serde(default)]
    pub git_head: String,
    #[serde(default)]
    pub last_sprint_closed: String,
    #[serde(default)]
    pub next_sprint: String,
    #[serde(default)]
    pub sprint_queue_open_count: u64,
    #[serde(default)]
    pub vision_ui_rev: u64,
    #[serde(default)]
    pub layers: Vec<Layer>,
    #[serde(default)]
    pub nodes: Vec<ManifestNode>,
    #[serde(default)]
    pub edges: Vec<ManifestEdge>,
    #[serde(default)]
    pub sprint_queue: Vec<SprintQueueEntry>,
}

/// Sprint ticker item (`feed.json` → `items[]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub published: String,
    #[serde(default)]
    pub link: String,
}

/// Sprint ticker feed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub items: Vec<FeedItem>,
}

/// Result of a `gsv-vision-sync` write run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncReport {
    pub revision: u64,
    pub git_head: String,
    pub nodes_count: u64,
    pub edges_count: u64,
    pub feed_items: u64,
    pub next_sprint: String,
    pub last_sprint_closed: String,
    pub manifest_source: String,
    pub feed_source: String,
    pub manifest_target: String,
    pub feed_target: String,
    pub synced_at: String,
}

const MANIFEST_SOURCE: &str = "docs/vision/manifest.json";
const FEED_SOURCE: &str = "docs/vision/feed.json";
const MANIFEST_TARGET: &str = "gsv_manifest.json";
const FEED_TARGET: &str = "gsv_feed.json";

/// Source manifest path under `repo_root`.
pub fn manifest_source(repo_root: &Path) -> PathBuf {
    repo_root.join(MANIFEST_SOURCE)
}

/// Source feed path under `repo_root`.
pub fn feed_source(repo_root: &Path) -> PathBuf {
    repo_root.join(FEED_SOURCE)
}

/// Persisted manifest path under `data_dir`.
pub fn manifest_target(data_dir: &Path) -> PathBuf {
    data_dir.join(MANIFEST_TARGET)
}

/// Persisted feed path under `data_dir`.
pub fn feed_target(data_dir: &Path) -> PathBuf {
    data_dir.join(FEED_TARGET)
}

/// Read + parse the vision manifest from `repo_root`.
pub fn read_manifest(repo_root: &Path) -> Result<Manifest, String> {
    let path = manifest_source(repo_root);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse manifest.json: {e}"))
}

/// Read + parse the vision feed from `repo_root`.
pub fn read_feed(repo_root: &Path) -> Result<Feed, String> {
    let path = feed_source(repo_root);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse feed.json: {e}"))
}

/// Persist the manifest snapshot to `data_dir`.
pub fn save_manifest(manifest: &Manifest, data_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw = serde_json::to_string_pretty(manifest).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(manifest_target(data_dir), raw).map_err(|e| format!("write: {e}"))
}

/// Persist the feed snapshot to `data_dir`.
pub fn save_feed(feed: &Feed, data_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw = serde_json::to_string_pretty(feed).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(feed_target(data_dir), raw).map_err(|e| format!("write: {e}"))
}

/// Load the persisted manifest snapshot from `data_dir`.
pub fn load_manifest(data_dir: &Path) -> Result<Manifest, String> {
    let path = manifest_target(data_dir);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Load the persisted feed snapshot from `data_dir`.
pub fn load_feed(data_dir: &Path) -> Result<Feed, String> {
    let path = feed_target(data_dir);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot.
fn source_manifest(repo_root: &Path, data_dir: &Path) -> Result<Manifest, String> {
    read_manifest(repo_root).or_else(|_| load_manifest(data_dir))
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot.
fn source_feed(repo_root: &Path, data_dir: &Path) -> Result<Feed, String> {
    read_feed(repo_root).or_else(|_| load_feed(data_dir))
}

/// `GET /api/vision/manifest` — full galaxy graph mirror.
pub fn wire_manifest(repo_root: &Path, data_dir: &Path) -> Value {
    match source_manifest(repo_root, data_dir) {
        Ok(m) => json!({
            "ok": true,
            "revision": m.revision,
            "updated_at": m.updated_at,
            "auto_sync_at": m.auto_sync_at,
            "git_head": m.git_head,
            "last_sprint_closed": m.last_sprint_closed,
            "next_sprint": m.next_sprint,
            "sprint_queue_open_count": m.sprint_queue_open_count,
            "vision_ui_rev": m.vision_ui_rev,
            "layers": m.layers,
            "nodes_count": m.nodes.len(),
            "edges_count": m.edges.len(),
            "nodes": m.nodes,
            "edges": m.edges,
            "sprint_queue": m.sprint_queue,
        }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// `GET /api/vision/feed` — sprint ticker mirror.
pub fn wire_feed(repo_root: &Path, data_dir: &Path) -> Value {
    match source_feed(repo_root, data_dir) {
        Ok(f) => json!({ "ok": true, "feed": f }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// `GET /api/vision` — lightweight vision summary for the UI.
pub fn wire_summary(repo_root: &Path, data_dir: &Path) -> Value {
    let manifest = source_manifest(repo_root, data_dir);
    let feed = source_feed(repo_root, data_dir);
    let (revision, git_head, updated_at, last_closed, next_sprint, open_count, ui_rev) =
        match &manifest {
            Ok(m) => (
                m.revision,
                m.git_head.clone(),
                m.updated_at.clone(),
                m.last_sprint_closed.clone(),
                m.next_sprint.clone(),
                m.sprint_queue_open_count,
                m.vision_ui_rev,
            ),
            Err(_) => (
                0,
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                0,
                0,
            ),
        };
    let feed_items = match &feed {
        Ok(f) => f.items.iter().take(10).cloned().collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };
    json!({
        "ok": true,
        "revision": revision,
        "git_head": git_head,
        "updated_at": updated_at,
        "last_sprint_closed": last_closed,
        "next_sprint": next_sprint,
        "sprint_queue_open_count": open_count,
        "vision_ui_rev": ui_rev,
        "nodes_count": manifest.as_ref().map(|m| m.nodes.len()).unwrap_or(0),
        "edges_count": manifest.as_ref().map(|m| m.edges.len()).unwrap_or(0),
        "feed_items": feed_items,
        "error": manifest.err().or_else(|| feed.err()),
    })
}

/// Write-run: read the live sources and persist both snapshots.
pub fn sync(repo_root: &Path, data_dir: &Path) -> Result<SyncReport, String> {
    let manifest = read_manifest(repo_root)?;
    let feed = read_feed(repo_root)?;
    save_manifest(&manifest, data_dir)?;
    save_feed(&feed, data_dir)?;
    Ok(SyncReport {
        revision: manifest.revision,
        git_head: manifest.git_head.clone(),
        nodes_count: manifest.nodes.len() as u64,
        edges_count: manifest.edges.len() as u64,
        feed_items: feed.items.len() as u64,
        next_sprint: manifest.next_sprint.clone(),
        last_sprint_closed: manifest.last_sprint_closed.clone(),
        manifest_source: manifest_source(repo_root).to_string_lossy().to_string(),
        feed_source: feed_source(repo_root).to_string_lossy().to_string(),
        manifest_target: manifest_target(data_dir).to_string_lossy().to_string(),
        feed_target: feed_target(data_dir).to_string_lossy().to_string(),
        synced_at: crate::vision::rfc3339_now(),
    })
}

/// Drift gate issues (empty = green).
pub fn collect_drift(repo_root: &Path, data_dir: &Path) -> Vec<String> {
    let mut issues = Vec::new();
    let manifest = match read_manifest(repo_root) {
        Ok(m) => m,
        Err(e) => {
            issues.push(e);
            return issues;
        }
    };
    if let Err(e) = read_feed(repo_root) {
        issues.push(e);
        return issues;
    }
    if let Ok(persisted) = load_manifest(data_dir) {
        if persisted.revision != manifest.revision {
            issues.push(format!(
                "persisted revision {} != source revision {}",
                persisted.revision, manifest.revision
            ));
        }
    }
    if manifest.revision == 0 {
        issues.push("manifest revision is 0".to_string());
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> Manifest {
        Manifest {
            revision: 458,
            updated_at: "2026-05-28".to_string(),
            auto_sync_at: "2026-05-28T00:00:00Z".to_string(),
            git_head: "50ce232f".to_string(),
            last_sprint_closed: "PH-S1728".to_string(),
            next_sprint: "PH-S1729".to_string(),
            sprint_queue_open_count: 0,
            vision_ui_rev: 458,
            layers: vec![Layer {
                id: "L0".to_string(),
                name: "Concept".to_string(),
                z: 0,
            }],
            nodes: vec![ManifestNode {
                id: "galaxy_grid".to_string(),
                label: "POOLAI_GALAXY_GRID".to_string(),
                layer: "L0".to_string(),
                path: "docs/concept/POOLAI_GALAXY_GRID.md".to_string(),
                sections: vec!["1-3 roles".to_string()],
                sprints: vec!["PH-S55".to_string()],
            }],
            edges: vec![ManifestEdge {
                from: "galaxy_grid".to_string(),
                kind: "concept-ref".to_string(),
                to: "memory_layer".to_string(),
            }],
            sprint_queue: vec![],
        }
    }

    fn sample_feed() -> Feed {
        Feed {
            title: "PoolAI Vision Sprint Feed".to_string(),
            updated_at: "2026-05-28".to_string(),
            items: vec![FeedItem {
                id: "PH-S1728".to_string(),
                title: "Band close".to_string(),
                category: "closed".to_string(),
                summary: "ratio hold".to_string(),
                status: "closed".to_string(),
                published: "2026-05-28".to_string(),
                link: "docs/vision/index.html#sprint-queue".to_string(),
            }],
        }
    }

    #[test]
    fn manifest_roundtrip_via_disk() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_manifest");
        let _ = std::fs::remove_dir_all(&tmp);
        let m = sample_manifest();
        save_manifest(&m, &tmp).unwrap();
        assert_eq!(load_manifest(&tmp).unwrap(), m);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn feed_roundtrip_via_disk() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_feed");
        let _ = std::fs::remove_dir_all(&tmp);
        let f = sample_feed();
        save_feed(&f, &tmp).unwrap();
        assert_eq!(load_feed(&tmp).unwrap(), f);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn manifest_parse_ignores_unknown_keys() {
        let raw = r#"{
            "revision": 7,
            "galaxy_background": "dark",
            "kivi": {"on": true},
            "nodes": [{"id": "n1", "label": "N1", "layer": "L0", "path": "p"}],
            "edges": [{"from": "n1", "kind": "k", "to": "n2"}]
        }"#;
        let m: Manifest = serde_json::from_str(raw).unwrap();
        assert_eq!(m.revision, 7);
        assert_eq!(m.nodes.len(), 1);
        assert_eq!(m.edges.len(), 1);
    }

    #[test]
    fn sync_writes_snapshots_and_reports() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_sync");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&sample_manifest()).unwrap(),
        )
        .unwrap();
        std::fs::write(
            vis.join("feed.json"),
            serde_json::to_string(&sample_feed()).unwrap(),
        )
        .unwrap();

        let report = sync(&src, &data).unwrap();
        assert_eq!(report.revision, 458);
        assert_eq!(report.nodes_count, 1);
        assert_eq!(report.edges_count, 1);
        assert_eq!(report.feed_items, 1);
        assert!(manifest_target(&data).exists());
        assert!(feed_target(&data).exists());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn drift_green_when_source_matches_persisted() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_drift");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&sample_manifest()).unwrap(),
        )
        .unwrap();
        std::fs::write(
            vis.join("feed.json"),
            serde_json::to_string(&sample_feed()).unwrap(),
        )
        .unwrap();
        save_manifest(&sample_manifest(), &data).unwrap();

        assert!(collect_drift(&src, &data).is_empty());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn drift_fails_on_revision_mismatch() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_drift_mismatch");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        let mut m = sample_manifest();
        m.revision = 459;
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&m).unwrap(),
        )
        .unwrap();
        std::fs::write(
            vis.join("feed.json"),
            serde_json::to_string(&sample_feed()).unwrap(),
        )
        .unwrap();
        save_manifest(&sample_manifest(), &data).unwrap();

        let issues = collect_drift(&src, &data);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].contains("revision"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn wire_summary_counts() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_wire");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&sample_manifest()).unwrap(),
        )
        .unwrap();
        std::fs::write(
            vis.join("feed.json"),
            serde_json::to_string(&sample_feed()).unwrap(),
        )
        .unwrap();

        let v = wire_summary(&src, &data);
        assert_eq!(v["ok"], true);
        assert_eq!(v["revision"], 458);
        assert_eq!(v["nodes_count"], 1);
        assert_eq!(v["edges_count"], 1);
        assert_eq!(v["feed_items"].as_array().unwrap().len(), 1);
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
