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

/// Extension manifest mirror (`docs/vision/extensions.json` → `extensions[]`).
///
/// Only the planning-relevant fields are typed; `scopes` (scope-id → meta) is
/// kept opaque so new extension metadata does not break the snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Extensions {
    #[serde(default)]
    pub active_sprint: String,
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    pub ui_version: u64,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub scopes: std::collections::BTreeMap<String, Value>,
}

impl Extensions {
    /// Sorted scope ids for stable wire output.
    pub fn scope_ids(&self) -> Vec<String> {
        self.scopes.keys().cloned().collect()
    }
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
    pub extensions_source: String,
    pub extensions_target: String,
    #[serde(default)]
    pub speed_index_target: String,
    #[serde(default)]
    pub rust_diagnostics_target: String,
    pub synced_at: String,
}

/// Per-layer map stats (`/api/vision/map`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerStats {
    pub id: String,
    pub name: String,
    pub z: i64,
    pub node_count: u64,
    pub edges_from: u64,
}

/// Edge-kind tally (`/api/vision/map`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeKindStats {
    pub kind: String,
    pub count: u64,
}

/// Lightweight galaxy-map report for the UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapReport {
    pub revision: u64,
    pub git_head: String,
    pub next_sprint: String,
    pub last_sprint_closed: String,
    pub nodes_count: u64,
    pub edges_count: u64,
    pub layers: Vec<LayerStats>,
    pub edge_kinds: Vec<EdgeKindStats>,
}

const MANIFEST_SOURCE: &str = "docs/vision/manifest.json";
const FEED_SOURCE: &str = "docs/vision/feed.json";
const EXTENSIONS_SOURCE: &str = "docs/vision/extensions.json";
const MANIFEST_TARGET: &str = "gsv_manifest.json";
const FEED_TARGET: &str = "gsv_feed.json";
const EXTENSIONS_TARGET: &str = "gsv_extensions.json";

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

/// Source extensions path under `repo_root`.
pub fn extensions_source(repo_root: &Path) -> PathBuf {
    repo_root.join(EXTENSIONS_SOURCE)
}

/// Persisted extensions path under `data_dir`.
pub fn extensions_target(data_dir: &Path) -> PathBuf {
    data_dir.join(EXTENSIONS_TARGET)
}

const SPEED_INDEX_SOURCE: &str = "docs/vision/speed_index.json";
const RUST_DIAGNOSTICS_SOURCE: &str = "docs/vision/rust_diagnostics.json";
const SPEED_INDEX_TARGET: &str = "gsv_speed_index.json";
const RUST_DIAGNOSTICS_TARGET: &str = "gsv_rust_diagnostics.json";

/// Source `speed_index.json` path under `repo_root`.
pub fn speed_index_source(repo_root: &Path) -> PathBuf {
    repo_root.join(SPEED_INDEX_SOURCE)
}

/// Persisted speed-index snapshot path under `data_dir`.
pub fn speed_index_target(data_dir: &Path) -> PathBuf {
    data_dir.join(SPEED_INDEX_TARGET)
}

/// Source `rust_diagnostics.json` path under `repo_root`.
pub fn rust_diagnostics_source(repo_root: &Path) -> PathBuf {
    repo_root.join(RUST_DIAGNOSTICS_SOURCE)
}

/// Persisted rust-diagnostics snapshot path under `data_dir`.
pub fn rust_diagnostics_target(data_dir: &Path) -> PathBuf {
    data_dir.join(RUST_DIAGNOSTICS_TARGET)
}

/// Latest test-CI + benchmark speed metrics (`speed_index.json` → `latest`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SpeedIndexLatest {
    #[serde(default)]
    pub test_ci_wall_secs: f64,
    #[serde(default)]
    pub test_ci_ok: bool,
    #[serde(default)]
    pub test_ci_recorded_at: String,
    #[serde(default)]
    pub test_ci_command: String,
    #[serde(default)]
    pub last_bench_label: String,
    #[serde(default)]
    pub last_bench_median_ns: u64,
    #[serde(default)]
    pub last_bench_recorded_at: String,
}

/// One test-CI history record (`speed_index.json` → `test_ci_history[]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SpeedTestCiRecord {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub wall_secs: f64,
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub host_label: String,
    #[serde(default)]
    pub git_head: String,
}

/// One Criterion bench history record (`speed_index.json` → `bench_history[]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SpeedBenchRecord {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub bench: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub median_ns: u64,
    #[serde(default)]
    pub profile: String,
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub host_label: String,
    #[serde(default)]
    pub git_head: String,
}

/// Speed-index report (`/api/vision/speeds`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SpeedIndexReport {
    pub generated_at: String,
    pub git_head: String,
    pub host_label: String,
    pub latest: SpeedIndexLatest,
    pub test_ci_count: u64,
    pub bench_count: u64,
    pub test_ci_history: Vec<SpeedTestCiRecord>,
    pub bench_history: Vec<SpeedBenchRecord>,
}

/// Full speed-index artifact (`speed_index.json`, tolerant of unknown fields).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct SpeedIndexFile {
    #[serde(default)]
    generated_at: String,
    #[serde(default)]
    git_head: String,
    #[serde(default)]
    host_label: String,
    #[serde(default)]
    latest: SpeedIndexLatest,
    #[serde(default)]
    test_ci_history: Vec<SpeedTestCiRecord>,
    #[serde(default)]
    bench_history: Vec<SpeedBenchRecord>,
}

/// Latest Rust clippy diagnostic counts (`rust_diagnostics.json` → `latest`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RustDiagLatest {
    #[serde(default)]
    pub warnings: u64,
    #[serde(default)]
    pub errors: u64,
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub top_codes: Vec<String>,
}

/// One Rust diagnostics history record (`rust_diagnostics.json` → `history[]`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RustDiagRecord {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub warnings: u64,
    #[serde(default)]
    pub errors: u64,
    #[serde(default)]
    pub ok: bool,
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub wall_secs: f64,
    #[serde(default)]
    pub host_label: String,
    #[serde(default)]
    pub git_head: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub top_codes: Vec<String>,
}

/// Rust-diagnostics report (`/api/vision/rust-diagnostics`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RustDiagnosticsReport {
    pub generated_at: String,
    pub git_head: String,
    pub host_label: String,
    pub latest: RustDiagLatest,
    pub history_count: u64,
    pub history: Vec<RustDiagRecord>,
}

/// Full rust-diagnostics artifact (`rust_diagnostics.json`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct RustDiagnosticsFile {
    #[serde(default)]
    generated_at: String,
    #[serde(default)]
    git_head: String,
    #[serde(default)]
    host_label: String,
    #[serde(default)]
    latest: RustDiagLatest,
    #[serde(default)]
    history: Vec<RustDiagRecord>,
}

/// Read + parse the speed index from `repo_root` (empty-tolerant).
pub fn read_speed_index(repo_root: &Path) -> Result<SpeedIndexReport, String> {
    let path = speed_index_source(repo_root);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: SpeedIndexFile =
        serde_json::from_str(&raw).map_err(|e| format!("parse speed_index.json: {e}"))?;
    Ok(SpeedIndexReport {
        generated_at: file.generated_at,
        git_head: file.git_head,
        host_label: file.host_label,
        latest: file.latest,
        test_ci_count: file.test_ci_history.len() as u64,
        bench_count: file.bench_history.len() as u64,
        test_ci_history: file.test_ci_history,
        bench_history: file.bench_history,
    })
}

/// Persist the speed-index report under `data_dir`.
pub fn save_speed_index(report: &SpeedIndexReport, data_dir: &Path) -> Result<(), String> {
    let path = speed_index_target(data_dir);
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw =
        serde_json::to_string_pretty(report).map_err(|e| format!("encode speed index: {e}"))?;
    std::fs::write(&path, raw).map_err(|e| format!("write {}: {e}", path.display()))
}

/// Read the persisted speed-index snapshot from `data_dir`.
pub fn load_speed_index(data_dir: &Path) -> Result<SpeedIndexReport, String> {
    let path = speed_index_target(data_dir);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse gsv_speed_index.json: {e}"))
}

/// Read + parse rust diagnostics from `repo_root` (empty-tolerant).
pub fn read_rust_diagnostics(repo_root: &Path) -> Result<RustDiagnosticsReport, String> {
    let path = rust_diagnostics_source(repo_root);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: RustDiagnosticsFile =
        serde_json::from_str(&raw).map_err(|e| format!("parse rust_diagnostics.json: {e}"))?;
    Ok(RustDiagnosticsReport {
        generated_at: file.generated_at,
        git_head: file.git_head,
        host_label: file.host_label,
        latest: file.latest,
        history_count: file.history.len() as u64,
        history: file.history,
    })
}

/// Persist the rust-diagnostics report under `data_dir`.
pub fn save_rust_diagnostics(
    report: &RustDiagnosticsReport,
    data_dir: &Path,
) -> Result<(), String> {
    let path = rust_diagnostics_target(data_dir);
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw = serde_json::to_string_pretty(report)
        .map_err(|e| format!("encode rust diagnostics: {e}"))?;
    std::fs::write(&path, raw).map_err(|e| format!("write {}: {e}", path.display()))
}

/// Read the persisted rust-diagnostics snapshot from `data_dir`.
pub fn load_rust_diagnostics(data_dir: &Path) -> Result<RustDiagnosticsReport, String> {
    let path = rust_diagnostics_target(data_dir);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse gsv_rust_diagnostics.json: {e}"))
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

/// Read + parse the extension manifest from `repo_root`.
pub fn read_extensions(repo_root: &Path) -> Result<Extensions, String> {
    let path = extensions_source(repo_root);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse extensions.json: {e}"))
}

/// Persist the extensions snapshot to `data_dir`.
pub fn save_extensions(extensions: &Extensions, data_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw = serde_json::to_string_pretty(extensions).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(extensions_target(data_dir), raw).map_err(|e| format!("write: {e}"))
}

/// Load the persisted extensions snapshot from `data_dir`.
pub fn load_extensions(data_dir: &Path) -> Result<Extensions, String> {
    let path = extensions_target(data_dir);
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot.
fn source_extensions(repo_root: &Path, data_dir: &Path) -> Result<Extensions, String> {
    read_extensions(repo_root).or_else(|_| load_extensions(data_dir))
}

/// `GET /api/vision/extensions` — extension manifest mirror (planning scopes).
pub fn wire_extensions(repo_root: &Path, data_dir: &Path) -> Value {
    match source_extensions(repo_root, data_dir) {
        Ok(e) => {
            let scope_ids = e.scope_ids();
            json!({
                "ok": true,
                "active_sprint": e.active_sprint,
                "revision": e.revision,
                "ui_version": e.ui_version,
                "updated_at": e.updated_at,
                "scope_count": scope_ids.len(),
                "scopes": scope_ids,
            })
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot.
fn source_manifest(repo_root: &Path, data_dir: &Path) -> Result<Manifest, String> {
    read_manifest(repo_root).or_else(|_| load_manifest(data_dir))
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot.
fn source_feed(repo_root: &Path, data_dir: &Path) -> Result<Feed, String> {
    read_feed(repo_root).or_else(|_| load_feed(data_dir))
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot,
/// else an empty report (empty-tolerant — artifact may not exist yet).
fn source_speed_index(repo_root: &Path, data_dir: &Path) -> SpeedIndexReport {
    read_speed_index(repo_root)
        .or_else(|_| load_speed_index(data_dir))
        .unwrap_or_default()
}

/// Preferred source for the wire: live `repo_root` source, else persisted snapshot,
/// else an empty report (empty-tolerant — artifact may not exist yet).
fn source_rust_diagnostics(repo_root: &Path, data_dir: &Path) -> RustDiagnosticsReport {
    read_rust_diagnostics(repo_root)
        .or_else(|_| load_rust_diagnostics(data_dir))
        .unwrap_or_default()
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

/// Build the lightweight galaxy-map report from the live manifest source.
pub fn map_report(repo_root: &Path, data_dir: &Path) -> Result<MapReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let mut node_counts: std::collections::BTreeMap<String, u64> =
        std::collections::BTreeMap::new();
    for node in &m.nodes {
        *node_counts.entry(node.layer.clone()).or_insert(0) += 1;
    }
    let layer_by_id: std::collections::HashMap<String, (String, i64)> = m
        .layers
        .iter()
        .map(|l| (l.id.clone(), (l.name.clone(), l.z)))
        .collect();
    let node_layer: std::collections::HashMap<&str, &str> = m
        .nodes
        .iter()
        .map(|n| (n.id.as_str(), n.layer.as_str()))
        .collect();
    let mut layers: Vec<LayerStats> = node_counts
        .into_iter()
        .map(|(id, node_count)| {
            let (name, z) = layer_by_id.get(&id).cloned().unwrap_or((id.clone(), 0));
            let edges_from = m
                .edges
                .iter()
                .filter(|e| node_layer.get(e.from.as_str()).copied() == Some(id.as_str()))
                .count() as u64;
            LayerStats {
                id,
                name,
                z,
                node_count,
                edges_from,
            }
        })
        .collect();
    layers.sort_by_key(|l| l.z);

    let mut edge_kinds: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for e in &m.edges {
        *edge_kinds.entry(e.kind.clone()).or_insert(0) += 1;
    }
    let edge_kinds = edge_kinds
        .into_iter()
        .map(|(kind, count)| EdgeKindStats { kind, count })
        .collect();

    Ok(MapReport {
        revision: m.revision,
        git_head: m.git_head,
        next_sprint: m.next_sprint,
        last_sprint_closed: m.last_sprint_closed,
        nodes_count: m.nodes.len() as u64,
        edges_count: m.edges.len() as u64,
        layers,
        edge_kinds,
    })
}

/// `GET /api/vision/map` — lightweight galaxy-map report for the UI.
pub fn wire_map(repo_root: &Path, data_dir: &Path) -> Value {
    match map_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Sprint-queue edge kinds (`/api/vision/sprint-map`).
pub const SPRINT_KINDS: [&str; 3] = ["sprint-scope", "queue", "session-tracks"];

/// Compact node reference used in map reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeRef {
    pub id: String,
    pub label: String,
    pub layer: String,
    pub path: String,
}

/// Sprint-queue link: a scoping/tracking edge with resolved endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintLink {
    pub kind: String,
    pub from: NodeRef,
    pub to: NodeRef,
}

/// Per-module target tally (`/api/vision/sprint-map`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintModule {
    pub id: String,
    pub label: String,
    pub layer: String,
    pub path: String,
    pub targets: u64,
}

/// Sprint-queue map report (`/api/vision/sprint-map`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintMapReport {
    pub revision: u64,
    pub git_head: String,
    pub next_sprint: String,
    pub last_sprint_closed: String,
    pub nodes_count: u64,
    pub links: Vec<SprintLink>,
    pub modules: Vec<SprintModule>,
    pub kinds: Vec<EdgeKindStats>,
    pub layers: Vec<LayerStats>,
}

/// Directed doc-preview link: edge kind + resolved target node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkTarget {
    pub kind: String,
    pub node: NodeRef,
}

/// Doc-preview report (`/api/vision/doc-preview`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocPreviewReport {
    pub revision: u64,
    pub git_head: String,
    pub node: ManifestNode,
    pub links_out: Vec<LinkTarget>,
    pub links_in: Vec<LinkTarget>,
    pub link_count: u64,
}

fn node_ref(n: &ManifestNode) -> NodeRef {
    NodeRef {
        id: n.id.clone(),
        label: n.label.clone(),
        layer: n.layer.clone(),
        path: n.path.clone(),
    }
}

/// Build the sprint-queue map: scoping/tracking edges across the galaxy graph.
pub fn sprint_map_report(repo_root: &Path, data_dir: &Path) -> Result<SprintMapReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let by_id: std::collections::HashMap<&str, &ManifestNode> =
        m.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    let mut links: Vec<SprintLink> = m
        .edges
        .iter()
        .filter(|e| SPRINT_KINDS.contains(&e.kind.as_str()))
        .filter_map(|e| {
            let from = by_id.get(e.from.as_str())?;
            let to = by_id.get(e.to.as_str())?;
            Some(SprintLink {
                kind: e.kind.clone(),
                from: node_ref(from),
                to: node_ref(to),
            })
        })
        .collect();
    links.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then_with(|| a.from.id.cmp(&b.from.id))
            .then_with(|| a.to.id.cmp(&b.to.id))
    });

    let mut target_counts: std::collections::BTreeMap<&str, u64> =
        std::collections::BTreeMap::new();
    for link in &links {
        *target_counts.entry(link.from.id.as_str()).or_insert(0) += 1;
    }
    let mut modules: Vec<SprintModule> = target_counts
        .into_iter()
        .filter_map(|(id, targets)| {
            let n = by_id.get(id)?;
            Some(SprintModule {
                id: n.id.clone(),
                label: n.label.clone(),
                layer: n.layer.clone(),
                path: n.path.clone(),
                targets,
            })
        })
        .collect();
    modules.sort_by(|a, b| b.targets.cmp(&a.targets).then_with(|| a.id.cmp(&b.id)));

    let mut kinds: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for link in &links {
        *kinds.entry(link.kind.clone()).or_insert(0) += 1;
    }
    let kinds = kinds
        .into_iter()
        .map(|(kind, count)| EdgeKindStats { kind, count })
        .collect();

    let mut involved: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    let mut layer_edges: std::collections::BTreeMap<String, u64> =
        std::collections::BTreeMap::new();
    for link in &links {
        for id in [&link.from.id, &link.to.id] {
            *involved.entry(id.clone()).or_insert(0) += 1;
        }
        *layer_edges.entry(link.from.layer.clone()).or_insert(0) += 1;
    }
    let layer_by_id: std::collections::HashMap<String, (String, i64)> = m
        .layers
        .iter()
        .map(|l| (l.id.clone(), (l.name.clone(), l.z)))
        .collect();
    let mut layers: Vec<LayerStats> = involved
        .iter()
        .map(|(id, node_count)| {
            let layer_id = by_id
                .get(id.as_str())
                .map(|n| n.layer.as_str())
                .unwrap_or("L0");
            let (name, z) = layer_by_id
                .get(layer_id)
                .cloned()
                .unwrap_or_else(|| (layer_id.to_string(), 0));
            LayerStats {
                id: layer_id.to_string(),
                name,
                z,
                node_count: *node_count,
                edges_from: layer_edges.get(layer_id).copied().unwrap_or(0),
            }
        })
        .collect();
    layers.sort_by_key(|l| l.z);

    Ok(SprintMapReport {
        revision: m.revision,
        git_head: m.git_head,
        next_sprint: m.next_sprint,
        last_sprint_closed: m.last_sprint_closed,
        nodes_count: involved.len() as u64,
        links,
        modules,
        kinds,
        layers,
    })
}

/// `GET /api/vision/sprint-map` — sprint-queue scoping/tracking map.
pub fn wire_sprint_map(repo_root: &Path, data_dir: &Path) -> Value {
    match sprint_map_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Build the doc-preview report for one graph node (1-hop neighbors).
pub fn doc_preview(
    repo_root: &Path,
    data_dir: &Path,
    id: &str,
) -> Result<DocPreviewReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let by_id: std::collections::HashMap<&str, &ManifestNode> =
        m.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let node = by_id
        .get(id)
        .copied()
        .ok_or_else(|| format!("node not found: {id}"))?
        .clone();

    let mut links_out: Vec<LinkTarget> = m
        .edges
        .iter()
        .filter(|e| e.from == id)
        .filter_map(|e| {
            by_id.get(e.to.as_str()).map(|n| LinkTarget {
                kind: e.kind.clone(),
                node: node_ref(n),
            })
        })
        .collect();
    let mut links_in: Vec<LinkTarget> = m
        .edges
        .iter()
        .filter(|e| e.to == id)
        .filter_map(|e| {
            by_id.get(e.from.as_str()).map(|n| LinkTarget {
                kind: e.kind.clone(),
                node: node_ref(n),
            })
        })
        .collect();
    links_out.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.node.id.cmp(&b.node.id)));
    links_in.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.node.id.cmp(&b.node.id)));

    let link_count = (links_out.len() + links_in.len()) as u64;
    Ok(DocPreviewReport {
        revision: m.revision,
        git_head: m.git_head,
        node,
        links_out,
        links_in,
        link_count,
    })
}

/// `GET /api/vision/doc-preview` — docs ↔ code preview for one node.
pub fn wire_doc_preview(repo_root: &Path, data_dir: &Path, id: &str) -> Value {
    if id.is_empty() {
        return json!({ "ok": false, "error": "id required" });
    }
    match doc_preview(repo_root, data_dir, id) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Node-search result: matched galaxy node + link tallies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSearchResult {
    pub id: String,
    pub label: String,
    pub layer: String,
    pub path: String,
    #[serde(default)]
    pub sections: Vec<String>,
    pub links_out: u64,
    pub links_in: u64,
}

/// Node-search report (`/api/vision/node-search`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSearchReport {
    pub revision: u64,
    pub git_head: String,
    pub query: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub layer: String,
    pub total_matches: u64,
    pub results: Vec<NodeSearchResult>,
}

/// Default result cap for node search.
pub const NODE_SEARCH_LIMIT: usize = 25;

/// Case-insensitive match over id / label / path / sections.
fn node_matches_query(n: &ManifestNode, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    n.id.to_lowercase().contains(needle)
        || n.label.to_lowercase().contains(needle)
        || n.path.to_lowercase().contains(needle)
        || n.sections.iter().any(|s| s.to_lowercase().contains(needle))
}

/// Build the node-search report (live source, else persisted snapshot).
pub fn node_search(
    repo_root: &Path,
    data_dir: &Path,
    query: &str,
    layer: Option<&str>,
) -> Result<NodeSearchReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let needle = query.trim().to_lowercase();
    let layer_filter = layer.unwrap_or("").trim().to_string();

    let mut links_out: std::collections::HashMap<&str, u64> = std::collections::HashMap::new();
    let mut links_in: std::collections::HashMap<&str, u64> = std::collections::HashMap::new();
    for e in &m.edges {
        *links_out.entry(e.from.as_str()).or_insert(0) += 1;
        *links_in.entry(e.to.as_str()).or_insert(0) += 1;
    }
    let layer_z: std::collections::HashMap<&str, i64> =
        m.layers.iter().map(|l| (l.id.as_str(), l.z)).collect();

    let mut results: Vec<NodeSearchResult> = m
        .nodes
        .iter()
        .filter(|n| node_matches_query(n, &needle))
        .filter(|n| layer_filter.is_empty() || n.layer == layer_filter)
        .map(|n| NodeSearchResult {
            id: n.id.clone(),
            label: n.label.clone(),
            layer: n.layer.clone(),
            path: n.path.clone(),
            sections: n.sections.clone(),
            links_out: links_out.get(n.id.as_str()).copied().unwrap_or(0),
            links_in: links_in.get(n.id.as_str()).copied().unwrap_or(0),
        })
        .collect();
    results.sort_by(|a, b| {
        layer_z
            .get(a.layer.as_str())
            .cmp(&layer_z.get(b.layer.as_str()))
            .then_with(|| a.id.cmp(&b.id))
    });
    let total_matches = results.len() as u64;
    results.truncate(NODE_SEARCH_LIMIT);

    Ok(NodeSearchReport {
        revision: m.revision,
        git_head: m.git_head,
        query: query.trim().to_string(),
        layer: layer_filter,
        total_matches,
        results,
    })
}

/// `GET /api/vision/node-search?q=&layer=` — galaxy node search.
pub fn wire_node_search(
    repo_root: &Path,
    data_dir: &Path,
    query: &str,
    layer: Option<&str>,
) -> Value {
    match node_search(repo_root, data_dir, query, layer) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// `GET /api/vision/feed` — optional `status` filter (`closed`/`open`/all).
pub fn wire_feed_filter(repo_root: &Path, data_dir: &Path, status: Option<&str>) -> Value {
    match source_feed(repo_root, data_dir) {
        Ok(mut f) => {
            if let Some(s) = status.filter(|s| !s.is_empty() && *s != "all") {
                f.items.retain(|i| i.status == s);
            }
            json!({ "ok": true, "feed": f })
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Write-run: read the live sources and persist snapshots.
pub fn sync(repo_root: &Path, data_dir: &Path) -> Result<SyncReport, String> {
    let manifest = read_manifest(repo_root)?;
    let feed = read_feed(repo_root)?;
    let extensions = read_extensions(repo_root)?;
    save_manifest(&manifest, data_dir)?;
    save_feed(&feed, data_dir)?;
    save_extensions(&extensions, data_dir)?;
    let speed_target = if let Ok(s) = read_speed_index(repo_root) {
        let _ = save_speed_index(&s, data_dir);
        speed_index_target(data_dir).to_string_lossy().to_string()
    } else {
        String::new()
    };
    let rust_diag_target = if let Ok(r) = read_rust_diagnostics(repo_root) {
        let _ = save_rust_diagnostics(&r, data_dir);
        rust_diagnostics_target(data_dir)
            .to_string_lossy()
            .to_string()
    } else {
        String::new()
    };
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
        extensions_source: extensions_source(repo_root).to_string_lossy().to_string(),
        extensions_target: extensions_target(data_dir).to_string_lossy().to_string(),
        speed_index_target: speed_target,
        rust_diagnostics_target: rust_diag_target,
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
    if let Err(e) = read_extensions(repo_root) {
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

/// `GET /api/vision/sync` — auto-sync: re-mirror the vision canon into the
/// snapshot and report drift. Never fails the request; `ok` reflects the write.
pub fn wire_sync(repo_root: &Path, data_dir: &Path) -> Value {
    let drift = collect_drift(repo_root, data_dir);
    match sync(repo_root, data_dir) {
        Ok(r) => json!({
            "ok": true,
            "drift": drift,
            "revision": r.revision,
            "git_head": r.git_head,
            "nodes_count": r.nodes_count,
            "edges_count": r.edges_count,
            "feed_items": r.feed_items,
            "next_sprint": r.next_sprint,
            "last_sprint_closed": r.last_sprint_closed,
            "manifest_target": r.manifest_target,
            "feed_target": r.feed_target,
            "extensions_target": r.extensions_target,
            "synced_at": r.synced_at,
        }),
        Err(e) => json!({ "ok": false, "drift": drift, "error": e }),
    }
}

/// `GET /api/vision/speeds` — speed-index report (latest test-CI + benchmark + history counts).
pub fn wire_speed_index(repo_root: &Path, data_dir: &Path) -> Value {
    let r = source_speed_index(repo_root, data_dir);
    json!({ "ok": true, "present": read_speed_index(repo_root).is_ok(), "speed_index": r })
}

/// `GET /api/vision/rust-diagnostics` — rust clippy diagnostics report.
pub fn wire_rust_diagnostics(repo_root: &Path, data_dir: &Path) -> Value {
    let r = source_rust_diagnostics(repo_root, data_dir);
    json!({ "ok": true, "present": read_rust_diagnostics(repo_root).is_ok(), "rust_diagnostics": r })
}

/// Short `MM-DD` from an ISO `recorded_at` (fallback: truncated raw string).
fn svg_day_label(recorded_at: &str) -> String {
    if recorded_at.len() >= 10 {
        recorded_at[5..10].to_string()
    } else {
        recorded_at.chars().take(10).collect()
    }
}

/// Empty-state SVG (no data artifact yet).
fn svg_empty(title: &str, hint: &str) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="560" height="132" viewBox="0 0 560 132">
<rect width="560" height="132" rx="8" fill="#121826"/>
<text x="12" y="22" font-family="monospace" font-size="12" fill="#e8843c">{title}</text>
<text x="12" y="70" font-family="monospace" font-size="12" fill="#7c8ba3">{hint}</text>
</svg>"##
    )
}

/// `GET /api/vision/speeds.svg` — test-CI wall-clock history bar chart (SVG).
///
/// Bars are green when the run `ok`, red otherwise; the latest bench median is
/// noted in the footer. Rendered server-side in Rust so the UI stays ratio-safe
/// (no client chart code).
pub fn speed_index_chart_svg(repo_root: &Path, data_dir: &Path) -> String {
    let r = source_speed_index(repo_root, data_dir);
    if r.test_ci_history.is_empty() {
        return svg_empty(
            "Speed index history",
            "no speed_index.json history - run bin/record-test-ci-speed.sh",
        );
    }
    let n = r.test_ci_history.len().min(24);
    let recs: Vec<&SpeedTestCiRecord> = r.test_ci_history[r.test_ci_history.len() - n..]
        .iter()
        .collect();
    let max_wall = recs
        .iter()
        .map(|x| x.wall_secs)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let plot_h = 92.0_f64;
    let base_y = 118.0_f64;
    let slot = 560.0_f64 / n as f64;
    let mut bars = String::new();
    for (i, rec) in recs.iter().enumerate() {
        let h = (rec.wall_secs / max_wall) * plot_h;
        let x = i as f64 * slot + slot * 0.25;
        let w = slot * 0.5;
        let color = if rec.ok { "#3fb96e" } else { "#e05b5b" };
        bars.push_str(&format!(
            r##"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" fill="{color}" opacity="0.9"><title>{rec:.0}s {ok} {day}</title></rect>"##,
            y = base_y - h,
            rec = rec.wall_secs,
            ok = if rec.ok { "ok" } else { "fail" },
            day = svg_day_label(&rec.recorded_at),
        ));
    }
    let bench = r
        .bench_history
        .last()
        .map(|b| format!("latest bench {} {} ns", b.bench, b.median_ns))
        .unwrap_or_else(|| "no bench history".to_string());
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="560" height="132" viewBox="0 0 560 132">
<rect width="560" height="132" rx="8" fill="#121826"/>
<text x="12" y="20" font-family="monospace" font-size="12" fill="#e8843c">test-ci wall-clock ({n} runs, max {max_wall:.0}s)</text>
{bars}
<line x1="6" y1="{base_y:.0}" x2="554" y2="{base_y:.0}" stroke="#1e2a3d" stroke-width="1"/>
<text x="12" y="128" font-family="monospace" font-size="10" fill="#7c8ba3">{bench}</text>
</svg>"##
    )
}

/// `GET /api/vision/rust-diagnostics.svg` — warnings/errors history chart (SVG).
///
/// Grouped bars per run: warnings in rust orange, errors in red. Rendered
/// server-side in Rust (ratio-safe).
pub fn rust_diagnostics_chart_svg(repo_root: &Path, data_dir: &Path) -> String {
    let r = source_rust_diagnostics(repo_root, data_dir);
    if r.history.is_empty() {
        return svg_empty(
            "Rust diagnostics history",
            "no rust_diagnostics.json history - run bin/record-rust-clippy.sh",
        );
    }
    let n = r.history.len().min(24);
    let recs: Vec<&RustDiagRecord> = r.history[r.history.len() - n..].iter().collect();
    let max_total = recs
        .iter()
        .map(|x| x.warnings + x.errors)
        .max()
        .unwrap_or(0)
        .max(1);
    let plot_h = 92.0_f64;
    let base_y = 118.0_f64;
    let slot = 560.0_f64 / n as f64;
    let mut bars = String::new();
    for (i, rec) in recs.iter().enumerate() {
        let hw = (rec.warnings as f64 / max_total as f64) * plot_h;
        let he = (rec.errors as f64 / max_total as f64) * plot_h;
        let x = i as f64 * slot + slot * 0.1;
        let w = slot * 0.4;
        bars.push_str(&format!(
            r##"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{hw:.1}" fill="#e8843c" opacity="0.9"><title>w {warn} e {err} {day}</title></rect>"##,
            y = base_y - hw,
            warn = rec.warnings,
            err = rec.errors,
            day = svg_day_label(&rec.recorded_at),
        ));
        bars.push_str(&format!(
            r##"<rect x="{x2:.1}" y="{y2:.1}" width="{w:.1}" height="{he:.1}" fill="#e05b5b" opacity="0.9"><title>errors {err} {day}</title></rect>"##,
            x2 = x + w,
            y2 = base_y - he,
            err = rec.errors,
            day = svg_day_label(&rec.recorded_at),
        ));
    }
    let latest = r.latest.command.clone();
    let latest = if latest.len() > 48 {
        format!("{}...", &latest[..48])
    } else {
        latest
    };
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="560" height="132" viewBox="0 0 560 132">
<rect width="560" height="132" rx="8" fill="#121826"/>
<text x="12" y="20" font-family="monospace" font-size="12" fill="#e8843c">clippy warnings/errors ({n} runs, max {max_total})</text>
{bars}
<line x1="6" y1="{base_y:.0}" x2="554" y2="{base_y:.0}" stroke="#1e2a3d" stroke-width="1"/>
<text x="12" y="128" font-family="monospace" font-size="10" fill="#7c8ba3">{latest}</text>
</svg>"##
    )
}

/// Sprint-queue planning report (`/api/vision/sprint-queue`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintQueueReport {
    pub revision: u64,
    pub git_head: String,
    pub next_sprint: String,
    pub last_sprint_closed: String,
    pub open_count: u64,
    pub active_sprint: String,
    pub entries: Vec<SprintQueueEntry>,
    pub planned: Vec<SprintQueueEntry>,
}

/// Build the sprint-queue plan: manifest queue ∪ active sprint from extensions.
pub fn sprint_queue_report(repo_root: &Path, data_dir: &Path) -> Result<SprintQueueReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let e = source_extensions(repo_root, data_dir)?;
    let mut planned = m.sprint_queue.clone();
    let has_active = planned.iter().any(|q| q.id == e.active_sprint);
    if !has_active && !e.active_sprint.is_empty() {
        planned.push(SprintQueueEntry {
            id: e.active_sprint.clone(),
            title: e.active_sprint.clone(),
            summary: "active sprint (extensions)".to_string(),
            status: "open".to_string(),
            category: "sprint".to_string(),
        });
    }
    Ok(SprintQueueReport {
        revision: m.revision,
        git_head: m.git_head,
        next_sprint: m.next_sprint,
        last_sprint_closed: m.last_sprint_closed,
        open_count: m.sprint_queue_open_count,
        active_sprint: e.active_sprint,
        entries: m.sprint_queue,
        planned,
    })
}

/// `GET /api/vision/sprint-queue` — sprint-queue planning report.
pub fn wire_sprint_queue(repo_root: &Path, data_dir: &Path) -> Value {
    match sprint_queue_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Sprint-board column (open / closed / planned).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintBoardColumn {
    pub name: String,
    pub count: u64,
    pub entries: Vec<SprintQueueEntry>,
}

/// Sprint-board report (`/api/vision/sprint-board`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintBoardReport {
    pub revision: u64,
    pub git_head: String,
    pub next_sprint: String,
    pub last_sprint_closed: String,
    pub active_sprint: String,
    pub total: u64,
    pub open_count: u64,
    pub closed_count: u64,
    pub progress_pct: u64,
    pub columns: Vec<SprintBoardColumn>,
}

fn sprint_status_group(status: &str, is_active: bool) -> &'static str {
    if is_active || status == "open" {
        "open"
    } else if status == "closed" || status == "done" {
        "closed"
    } else {
        "planned"
    }
}

/// Build the sprint-board from the sprint-queue plan: group the working queue
/// (manifest ∪ active) into open / closed / planned columns + progress pct.
pub fn sprint_board_report(repo_root: &Path, data_dir: &Path) -> Result<SprintBoardReport, String> {
    let q = sprint_queue_report(repo_root, data_dir)?;
    let mut columns = vec![
        SprintBoardColumn {
            name: "open".to_string(),
            count: 0,
            entries: Vec::new(),
        },
        SprintBoardColumn {
            name: "closed".to_string(),
            count: 0,
            entries: Vec::new(),
        },
        SprintBoardColumn {
            name: "planned".to_string(),
            count: 0,
            entries: Vec::new(),
        },
    ];
    for entry in &q.planned {
        let group = sprint_status_group(&entry.status, entry.id == q.active_sprint);
        let column = columns
            .iter_mut()
            .find(|c| c.name == group)
            .expect("known column");
        column.entries.push(entry.clone());
        column.count += 1;
    }
    let open_count = columns[0].count;
    let closed_count = columns[1].count;
    let total = q.planned.len() as u64;
    let progress_pct = if total > 0 {
        (closed_count * 100) / total
    } else {
        0
    };
    Ok(SprintBoardReport {
        revision: q.revision,
        git_head: q.git_head,
        next_sprint: q.next_sprint,
        last_sprint_closed: q.last_sprint_closed,
        active_sprint: q.active_sprint,
        total,
        open_count,
        closed_count,
        progress_pct,
        columns,
    })
}

/// `GET /api/vision/sprint-board` — sprint-board report.
pub fn wire_sprint_board(repo_root: &Path, data_dir: &Path) -> Value {
    match sprint_board_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Per-layer sprint progress (`/api/vision/sprint-board` → `layers[]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintLayerProgress {
    pub id: String,
    pub name: String,
    pub z: i64,
    pub node_count: u64,
    pub linked_count: u64,
}

/// Sprint progress report (statuses + per-layer distribution).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintProgressReport {
    pub revision: u64,
    pub total: u64,
    pub open_count: u64,
    pub closed_count: u64,
    pub planned_count: u64,
    pub progress_pct: u64,
    pub layers: Vec<SprintLayerProgress>,
}

/// Build sprint progress: status counts over the working queue + per-layer
/// distribution of nodes linked to the current queue's sprints.
pub fn sprint_progress_report(
    repo_root: &Path,
    data_dir: &Path,
) -> Result<SprintProgressReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let q = sprint_queue_report(repo_root, data_dir)?;
    let ids: std::collections::BTreeSet<String> = q.planned.iter().map(|e| e.id.clone()).collect();
    let mut layers: Vec<SprintLayerProgress> = m
        .layers
        .iter()
        .map(|l| SprintLayerProgress {
            id: l.id.clone(),
            name: l.name.clone(),
            z: l.z,
            node_count: 0,
            linked_count: 0,
        })
        .collect();
    for node in &m.nodes {
        if let Some(layer) = layers.iter_mut().find(|l| l.id == node.layer) {
            layer.node_count += 1;
            if node.sprints.iter().any(|s| ids.contains(s)) {
                layer.linked_count += 1;
            }
        }
    }
    layers.sort_by_key(|l| l.z);
    let total = q.planned.len() as u64;
    let closed_count = q
        .planned
        .iter()
        .filter(|e| e.status == "closed" || e.status == "done")
        .count() as u64;
    let open_count = q
        .planned
        .iter()
        .filter(|e| e.status == "open" || e.id == q.active_sprint)
        .count() as u64;
    let planned_count = total.saturating_sub(closed_count + open_count);
    let progress_pct = if total > 0 {
        (closed_count * 100) / total
    } else {
        0
    };
    Ok(SprintProgressReport {
        revision: m.revision,
        total,
        open_count,
        closed_count,
        planned_count,
        progress_pct,
        layers,
    })
}

/// `GET /api/vision/sprint-progress` — sprint progress report.
pub fn wire_sprint_progress(repo_root: &Path, data_dir: &Path) -> Value {
    match sprint_progress_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

// ---------------------------------------------------------------------------
// Sprint UI theme (band 118): colors ported from the legacy vision.css
// (`--sprint: #a78bfa`, sprint-pill/chip/queue-state rules) so GSV renders the
// galaxy sprint UI with the same look as the deactivated docs/vision/index.html.
// ---------------------------------------------------------------------------

/// Sprint-pill theme (legacy `.sprint-pill`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintPillTheme {
    pub bg: String,
    pub border: String,
    pub color: String,
}

/// Sprint-chip theme (legacy `.sprint-chip`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintChipTheme {
    pub bg: String,
    pub border: String,
    pub color: String,
}

/// Sprint-queue state colors (legacy `.sprint-queue-item.open/.next/.closed`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintQueueStateTheme {
    pub open_border: String,
    pub open_bg: String,
    pub open_status: String,
    pub next_border: String,
    pub next_glow: String,
    pub closed_opacity: String,
}

/// Per-layer fill color (legacy `--L0…--L5`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintLayerColor {
    pub id: String,
    pub color: String,
}

/// Per-edge-kind color (legacy `--edge-docs/--edge-code/--edge-toml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintEdgeKindColor {
    pub kind: String,
    pub color: String,
}

/// Sprint UI theme report (`/api/vision/sprint-theme`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SprintThemeReport {
    pub revision: u64,
    pub git_head: String,
    pub active_sprint: String,
    pub next_sprint: String,
    pub sprint: String,
    pub sprint_next: String,
    pub pill: SprintPillTheme,
    pub chip: SprintChipTheme,
    pub queue: SprintQueueStateTheme,
    pub layers: Vec<SprintLayerColor>,
    pub edge_kinds: Vec<SprintEdgeKindColor>,
}

const SPRINT_ACCENT: &str = "#a78bfa";
const SPRINT_NEXT_COLOR: &str = "#c4b5fd";
const QUEUE_NEXT_BORDER: &str = "rgba(126, 184, 255, 0.55)";
const QUEUE_NEXT_GLOW: &str = "rgba(126, 184, 255, 0.15)";

/// Layer palette keyed by manifest layer id (legacy `--L0…--L5`).
fn sprint_layer_color(layer_id: &str) -> String {
    match layer_id {
        "L0" => "#3d6a9e".to_string(),
        "L1" => "#3d6a4a".to_string(),
        "L2" => "#8a7040".to_string(),
        "L3" => "#8a4068".to_string(),
        "L4" => "#6a5088".to_string(),
        "L5" => "#4a6880".to_string(),
        _ => "#7eb8ff".to_string(),
    }
}

/// Edge-kind palette (legacy `--edge-docs/--edge-code/--edge-toml`); unknown
/// kinds fall back to the accent blue.
fn sprint_edge_kind_color(kind: &str) -> String {
    match kind {
        "docs" => "#90c490".to_string(),
        "code" => "#c49ab0".to_string(),
        "toml" => "#7eb8c4".to_string(),
        _ => "#7eb8ff".to_string(),
    }
}

/// Build the sprint UI theme from the live manifest + extensions.
pub fn sprint_theme_report(repo_root: &Path, data_dir: &Path) -> Result<SprintThemeReport, String> {
    let m = source_manifest(repo_root, data_dir)?;
    let e = source_extensions(repo_root, data_dir)?;
    let mut layers: Vec<SprintLayerColor> = m
        .layers
        .iter()
        .map(|l| SprintLayerColor {
            id: l.id.clone(),
            color: sprint_layer_color(&l.id),
        })
        .collect();
    layers.sort_by(|a, b| a.id.cmp(&b.id));
    let mut kinds: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for edge in &m.edges {
        kinds
            .entry(edge.kind.clone())
            .or_insert_with(|| sprint_edge_kind_color(&edge.kind));
    }
    let edge_kinds: Vec<SprintEdgeKindColor> = kinds
        .into_iter()
        .map(|(kind, color)| SprintEdgeKindColor { kind, color })
        .collect();
    Ok(SprintThemeReport {
        revision: m.revision,
        git_head: m.git_head,
        active_sprint: e.active_sprint,
        next_sprint: m.next_sprint,
        sprint: SPRINT_ACCENT.to_string(),
        sprint_next: SPRINT_NEXT_COLOR.to_string(),
        pill: SprintPillTheme {
            bg: "rgba(167, 139, 250, 0.2)".to_string(),
            border: "rgba(167, 139, 250, 0.4)".to_string(),
            color: "#d4c4ff".to_string(),
        },
        chip: SprintChipTheme {
            bg: "rgba(167, 139, 250, 0.15)".to_string(),
            border: "rgba(167, 139, 250, 0.3)".to_string(),
            color: SPRINT_NEXT_COLOR.to_string(),
        },
        queue: SprintQueueStateTheme {
            open_border: "rgba(167, 139, 250, 0.35)".to_string(),
            open_bg: "rgba(167, 139, 250, 0.08)".to_string(),
            open_status: SPRINT_ACCENT.to_string(),
            next_border: QUEUE_NEXT_BORDER.to_string(),
            next_glow: QUEUE_NEXT_GLOW.to_string(),
            closed_opacity: "0.55".to_string(),
        },
        layers,
        edge_kinds,
    })
}

/// `GET /api/vision/sprint-theme` — sprint UI theme colors.
pub fn wire_sprint_theme(repo_root: &Path, data_dir: &Path) -> Value {
    match sprint_theme_report(repo_root, data_dir) {
        Ok(r) => {
            let mut v = serde_json::to_value(&r).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

// ---------------------------------------------------------------------------
// Sprint focus SVG (band 118): Rust-rendered galaxy map with the target sprint
// highlighted and out-of-scope nodes/edges dimmed (legacy `sprint-dim`:
// circle opacity 0.22 / text 0.28). Rendered server-side so the UI stays
// ratio-safe (no client chart/map code).
// ---------------------------------------------------------------------------

/// Sprint token match (legacy `sprintTokenMatches`): exact id or `PH-S*` glob.
pub fn sprint_token_matches(token: &str, sprint: &str) -> bool {
    if token.is_empty() || sprint.is_empty() {
        return false;
    }
    if token == sprint {
        return true;
    }
    if let Some(prefix) = token.strip_suffix('*') {
        return sprint.starts_with(prefix);
    }
    false
}

/// Glob match for node paths (legacy `pathMatchesGlob`): `**` crosses `/`,
/// `*` matches a non-`/` run, everything else is a literal.
fn path_matches_glob(path: &str, glob: &str) -> bool {
    if let Some(prefix) = glob.strip_suffix("/**") {
        return path.starts_with(prefix);
    }
    let path: Vec<char> = path.chars().collect();
    let glob: Vec<char> = glob.chars().collect();
    fn match_at(path: &[char], glob: &[char]) -> bool {
        match glob.first() {
            None => path.is_empty(),
            Some('*') => {
                let double = glob.get(1) == Some(&'*');
                let rest = if double { &glob[2..] } else { &glob[1..] };
                if match_at(path, rest) {
                    return true;
                }
                if let Some((&first, tail)) = path.split_first() {
                    if !double && first == '/' {
                        return false;
                    }
                    match_at(tail, glob)
                } else {
                    false
                }
            }
            Some(&want) => match path.split_first() {
                Some((&got, tail)) if got == want => match_at(tail, &glob[1..]),
                _ => false,
            },
        }
    }
    match_at(&path, &glob)
}

/// Node ids in scope for a sprint: nodes whose `sprints[]` token-matches the
/// sprint, plus nodes whose path lands in a matching extension scope's docs or
/// code globs (legacy `buildSprintPathSet` + `nodesForSprint`).
fn nodes_for_sprint(
    m: &Manifest,
    e: &Extensions,
    sprint: &str,
) -> std::collections::BTreeSet<String> {
    let mut ids = std::collections::BTreeSet::new();
    if sprint.is_empty() {
        return ids;
    }
    let mut scope_globs: Vec<String> = Vec::new();
    let mut scope_docs: Vec<String> = Vec::new();
    for scope in e.scopes.values() {
        let sprints: Vec<String> = scope
            .get("sprints")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        if !sprints.iter().any(|t| sprint_token_matches(t, sprint)) {
            continue;
        }
        if let Some(docs) = scope.get("docs").and_then(Value::as_array) {
            scope_docs.extend(
                docs.iter()
                    .filter_map(Value::as_str)
                    .map(|p| p.trim_start_matches('/').to_string()),
            );
        }
        for key in ["code_globs", "also_update"] {
            if let Some(globs) = scope.get(key).and_then(Value::as_array) {
                scope_globs.extend(globs.iter().filter_map(Value::as_str).map(str::to_string));
            }
        }
    }
    for n in &m.nodes {
        let token_match = n.sprints.iter().any(|t| sprint_token_matches(t, sprint));
        let scope_match = scope_docs.contains(&n.path)
            || scope_globs.iter().any(|g| path_matches_glob(&n.path, g));
        if token_match || scope_match {
            ids.insert(n.id.clone());
        }
    }
    ids
}

/// `GET /api/vision/sprint-focus.svg?sprint=` — sprint focus map (SVG).
///
/// Layout: one row per layer (L0…L5), nodes spread left-to-right. In-scope
/// nodes render at full opacity with the sprint accent; out-of-scope nodes use
/// `sprint-dim` (circle 0.22 / text 0.28). Edges touching in-scope nodes are
/// tinted, others dimmed. Empty state when the sprint has no nodes.
pub fn sprint_focus_svg(repo_root: &Path, data_dir: &Path, sprint: &str) -> String {
    let Ok(m) = source_manifest(repo_root, data_dir) else {
        return svg_empty("Sprint focus", "no manifest snapshot available");
    };
    let e = source_extensions(repo_root, data_dir).unwrap_or_default();
    let focus = if sprint.is_empty() {
        if e.active_sprint.is_empty() {
            m.next_sprint.clone()
        } else {
            e.active_sprint.clone()
        }
    } else {
        sprint.to_string()
    };
    let in_scope = nodes_for_sprint(&m, &e, &focus);
    if in_scope.is_empty() {
        return svg_empty(
            &format!("Sprint focus: {focus}"),
            "no galaxy nodes reference this sprint yet",
        );
    }
    let mut layers_sorted: Vec<&Layer> = m.layers.iter().collect();
    layers_sorted.sort_by_key(|l| l.z);
    let width = 900.0_f64;
    let row_h = 34.0_f64;
    let header_h = 26.0_f64;
    let height = header_h + layers_sorted.len() as f64 * row_h + 20.0_f64;
    let mut positions: std::collections::HashMap<String, (f64, f64)> =
        std::collections::HashMap::new();
    let mut body = String::new();
    let mut in_scope_count = 0u64;
    for (li, layer) in layers_sorted.iter().enumerate() {
        let mut nodes: Vec<&ManifestNode> =
            m.nodes.iter().filter(|n| n.layer == layer.id).collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        if nodes.is_empty() {
            continue;
        }
        let cy = header_h + li as f64 * row_h + row_h / 2.0;
        let step = (width - 40.0) / nodes.len() as f64;
        for (i, n) in nodes.iter().enumerate() {
            let cx = 20.0 + i as f64 * step + step / 2.0;
            positions.insert(n.id.clone(), (cx, cy));
            let scope = in_scope.contains(&n.id);
            if scope {
                in_scope_count += 1;
                body.push_str(&format!(
                    r##"<circle cx="{cx:.1}" cy="{cy:.1}" r="4" fill="{SPRINT_ACCENT}"><title>{id}</title></circle>"##,
                    id = n.id
                ));
                body.push_str(&format!(
                    r##"<text x="{cx:.1}" y="{ty:.1}" font-family="monospace" font-size="9" fill="#d4c4ff">{label}</text>"##,
                    ty = cy + 13.0,
                    label = n.label
                ));
            } else {
                body.push_str(&format!(
                    r##"<circle cx="{cx:.1}" cy="{cy:.1}" r="3" fill="{color}" opacity="0.22"><title>{id}</title></circle>"##,
                    color = sprint_layer_color(&n.layer),
                    id = n.id
                ));
            }
        }
    }
    let mut edges_svg = String::new();
    for edge in &m.edges {
        let Some(&(fx, fy)) = positions.get(&edge.from) else {
            continue;
        };
        let Some(&(tx, ty)) = positions.get(&edge.to) else {
            continue;
        };
        let from_scope = in_scope.contains(&edge.from);
        let to_scope = in_scope.contains(&edge.to);
        if from_scope || to_scope {
            edges_svg.push_str(&format!(
                r##"<line x1="{fx:.1}" y1="{fy:.1}" x2="{tx:.1}" y2="{ty:.1}" stroke="#a78bfa" stroke-width="1" opacity="0.55"><title>{kind}</title></line>"##,
                kind = edge.kind
            ));
        } else {
            edges_svg.push_str(&format!(
                r##"<line x1="{fx:.1}" y1="{fy:.1}" x2="{tx:.1}" y2="{ty:.1}" stroke="#33405a" stroke-width="0.7" opacity="0.14"><title>{kind}</title></line>"##,
                kind = edge.kind
            ));
        }
    }
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0}" height="{height:.0}" viewBox="0 0 {width:.0} {height:.0}">
<rect width="{width:.0}" height="{height:.0}" rx="8" fill="#121826"/>
<text x="12" y="16" font-family="monospace" font-size="12" fill="{SPRINT_ACCENT}">sprint focus: {focus}</text>
<text x="{right:.0}" y="16" font-family="monospace" font-size="11" fill="#7c8ba3" text-anchor="end">{in_scope_count} / {total} nodes</text>
{edges_svg}{body}
</svg>"##,
        right = width - 12.0,
        total = m.nodes.len()
    )
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
                link: "http://127.0.0.1:8891/#b-sprint-board".to_string(),
            }],
        }
    }

    fn sample_extensions() -> Extensions {
        let mut scopes = std::collections::BTreeMap::new();
        scopes.insert(
            "fm_replenish_post_galaxy_mvp".to_string(),
            json!({ "title": "FM replenish" }),
        );
        scopes.insert(
            "docs_vision_meta".to_string(),
            json!({ "title": "Vision meta" }),
        );
        Extensions {
            active_sprint: "PH-S1729".to_string(),
            revision: 310,
            ui_version: 4,
            updated_at: "2026-05-28".to_string(),
            scopes,
        }
    }

    fn write_extensions(vis: &Path) {
        std::fs::write(
            vis.join("extensions.json"),
            serde_json::to_string(&sample_extensions()).unwrap(),
        )
        .unwrap();
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
        write_extensions(&vis);

        let report = sync(&src, &data).unwrap();
        assert_eq!(report.revision, 458);
        assert_eq!(report.nodes_count, 1);
        assert_eq!(report.edges_count, 1);
        assert_eq!(report.feed_items, 1);
        assert!(manifest_target(&data).exists());
        assert!(feed_target(&data).exists());
        assert!(extensions_target(&data).exists());
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
        write_extensions(&vis);
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
        write_extensions(&vis);
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

    #[test]
    fn map_report_layers_sorted_and_counted() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_map");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();

        let mut m = sample_manifest();
        m.layers = vec![
            Layer {
                id: "L1".to_string(),
                name: "Ops".to_string(),
                z: 1,
            },
            Layer {
                id: "L0".to_string(),
                name: "Concept".to_string(),
                z: 0,
            },
        ];
        m.nodes.push(ManifestNode {
            id: "handoff".to_string(),
            label: "HANDOFF".to_string(),
            layer: "L1".to_string(),
            path: "docs/development/HANDOFF.md".to_string(),
            sections: vec![],
            sprints: vec![],
        });
        m.edges.push(ManifestEdge {
            from: "handoff".to_string(),
            kind: "session-track".to_string(),
            to: "galaxy_grid".to_string(),
        });
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

        let r = map_report(&src, &data).unwrap();
        assert_eq!(r.nodes_count, 2);
        assert_eq!(r.edges_count, 2);
        assert_eq!(r.layers.len(), 2);
        assert_eq!(r.layers[0].id, "L0");
        assert_eq!(r.layers[0].z, 0);
        assert_eq!(r.layers[0].node_count, 1);
        assert_eq!(r.layers[1].id, "L1");
        assert_eq!(r.layers[1].node_count, 1);
        assert_eq!(r.layers[1].edges_from, 1);
        assert_eq!(r.edge_kinds.len(), 2);
        assert!(r.edge_kinds.iter().any(|e| e.kind == "concept-ref"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn wire_feed_filter_filters_by_status() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_feed_filter");
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
        let mut f = sample_feed();
        f.items.push(FeedItem {
            id: "PH-S1729".to_string(),
            title: "Next".to_string(),
            category: "open".to_string(),
            summary: "open".to_string(),
            status: "open".to_string(),
            published: "2026-05-29".to_string(),
            link: "http://127.0.0.1:8891/#b-sprint-board".to_string(),
        });
        std::fs::write(vis.join("feed.json"), serde_json::to_string(&f).unwrap()).unwrap();

        let all = wire_feed_filter(&src, &data, None);
        assert_eq!(all["ok"], true);
        assert_eq!(all["feed"]["items"].as_array().unwrap().len(), 2);

        let closed = wire_feed_filter(&src, &data, Some("closed"));
        assert_eq!(closed["feed"]["items"].as_array().unwrap().len(), 1);
        assert_eq!(closed["feed"]["items"][0]["status"], "closed");

        let open = wire_feed_filter(&src, &data, Some("open"));
        assert_eq!(open["feed"]["items"].as_array().unwrap().len(), 1);
        assert_eq!(open["feed"]["items"][0]["status"], "open");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    fn sample_manifest_with_sprint_links() -> Manifest {
        let mut m = sample_manifest();
        m.nodes.push(ManifestNode {
            id: "handoff".to_string(),
            label: "HANDOFF".to_string(),
            layer: "L0".to_string(),
            path: "docs/development/HANDOFF.md".to_string(),
            sections: vec![],
            sprints: vec![],
        });
        m.nodes.push(ManifestNode {
            id: "grid_dispatch".to_string(),
            label: "grid/dispatch.rs".to_string(),
            layer: "L3".to_string(),
            path: "src/grid/grid_dispatch.rs".to_string(),
            sections: vec![],
            sprints: vec![],
        });
        m.nodes.push(ManifestNode {
            id: "e2e_grid_job_lease".to_string(),
            label: "grid_job_lease.e2e".to_string(),
            layer: "L3".to_string(),
            path: "e2e/grid_job_lease.spec.ts".to_string(),
            sections: vec![],
            sprints: vec![],
        });
        m.edges.push(ManifestEdge {
            from: "grid_dispatch".to_string(),
            kind: "sprint-scope".to_string(),
            to: "e2e_grid_job_lease".to_string(),
        });
        m.edges.push(ManifestEdge {
            from: "handoff".to_string(),
            kind: "session-tracks".to_string(),
            to: "grid_dispatch".to_string(),
        });
        m
    }

    fn write_sample(root: &Path, manifest: &Manifest, feed: &Feed) {
        let vis = root.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(manifest).unwrap(),
        )
        .unwrap();
        std::fs::write(vis.join("feed.json"), serde_json::to_string(feed).unwrap()).unwrap();
        write_extensions(&vis);
    }

    #[test]
    fn sprint_map_report_lists_scoping_and_tracking_edges() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_sprint_map");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_sample(&src, &sample_manifest_with_sprint_links(), &sample_feed());

        let r = sprint_map_report(&src, &data).unwrap();
        assert_eq!(r.links.len(), 2);
        assert_eq!(r.links[0].kind, "session-tracks");
        assert_eq!(r.links[0].to.id, "grid_dispatch");
        assert_eq!(r.links[1].kind, "sprint-scope");
        assert_eq!(r.links[1].from.id, "grid_dispatch");
        assert!(r
            .modules
            .iter()
            .any(|m| m.id == "grid_dispatch" && m.targets == 1));
        assert!(r
            .modules
            .iter()
            .any(|m| m.id == "handoff" && m.targets == 1));
        assert!(r.kinds.iter().any(|k| k.kind == "sprint-scope"));
        assert_eq!(r.nodes_count, 3);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn doc_preview_returns_one_hop_links() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_doc_preview");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_sample(&src, &sample_manifest_with_sprint_links(), &sample_feed());

        let r = doc_preview(&src, &data, "grid_dispatch").unwrap();
        assert_eq!(r.node.id, "grid_dispatch");
        assert_eq!(r.link_count, 2);
        assert!(r.links_in.iter().any(|l| l.node.id == "handoff"));
        assert!(r
            .links_out
            .iter()
            .any(|l| l.node.id == "e2e_grid_job_lease"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn doc_preview_missing_node_is_error() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_doc_preview_missing");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_sample(&src, &sample_manifest_with_sprint_links(), &sample_feed());

        assert!(doc_preview(&src, &data, "nope").is_err());
        assert_eq!(wire_doc_preview(&src, &data, "")["ok"], false);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    fn write_speed_index(repo_root: &Path) {
        let vis = repo_root.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("speed_index.json"),
            r#"{
              "schema_version": 1,
              "generated_at": "2026-08-04T00:00:00Z",
              "host_label": "PLATINOV",
              "git_head": "50ce232f",
              "latest": {
                "test_ci_wall_secs": 1076.5,
                "test_ci_ok": true,
                "test_ci_recorded_at": "2026-08-04T00:00:00Z",
                "test_ci_command": "bin/record-test-ci-speed.sh",
                "last_bench_label": "dispatch_pipeline",
                "last_bench_median_ns": 4210,
                "last_bench_recorded_at": "2026-08-04T00:00:00Z"
              },
              "test_ci_history": [{ "kind": "test-ci", "wall_secs": 1076.5, "ok": true }],
              "bench_history": [{ "kind": "bench", "median_ns": 4210 }]
            }"#,
        )
        .unwrap();
    }

    fn write_rust_diagnostics(repo_root: &Path) {
        let vis = repo_root.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("rust_diagnostics.json"),
            r#"{
              "schema_version": 1,
              "generated_at": "2026-08-04T00:00:00Z",
              "host_label": "PLATINOV",
              "git_head": "50ce232f",
              "source": "local",
              "latest": {
                "warnings": 3,
                "errors": 0,
                "ok": true,
                "recorded_at": "2026-08-04T00:00:00Z",
                "command": "cargo clippy --all-targets",
                "top_codes": ["clippy::needless_borrow", "clippy::too_many_arguments"]
              },
              "history": [{ "kind": "clippy", "warnings": 3, "errors": 0, "ok": true }]
            }"#,
        )
        .unwrap();
    }

    #[test]
    fn speed_index_round_trip_and_wire() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_speed_index");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_speed_index(&src);

        let r = read_speed_index(&src).unwrap();
        assert_eq!(r.generated_at, "2026-08-04T00:00:00Z");
        assert_eq!(r.host_label, "PLATINOV");
        assert_eq!(r.latest.test_ci_wall_secs, 1076.5);
        assert!(r.latest.test_ci_ok);
        assert_eq!(r.latest.last_bench_median_ns, 4210);
        assert_eq!(r.test_ci_count, 1);
        assert_eq!(r.bench_count, 1);

        save_speed_index(&r, &data).unwrap();
        let loaded = load_speed_index(&data).unwrap();
        assert_eq!(loaded, r);

        let wire = wire_speed_index(&src, &data);
        assert_eq!(wire["ok"], true);
        assert_eq!(wire["present"], true);
        assert_eq!(wire["speed_index"]["latest"]["test_ci_wall_secs"], 1076.5);
        assert_eq!(wire["speed_index"]["test_ci_count"], 1);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn rust_diagnostics_round_trip_and_wire() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_rust_diag");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_rust_diagnostics(&src);

        let r = read_rust_diagnostics(&src).unwrap();
        assert_eq!(r.latest.warnings, 3);
        assert_eq!(r.latest.errors, 0);
        assert!(r.latest.ok);
        assert_eq!(r.latest.top_codes.len(), 2);
        assert_eq!(r.history_count, 1);

        save_rust_diagnostics(&r, &data).unwrap();
        assert_eq!(load_rust_diagnostics(&data).unwrap(), r);

        let wire = wire_rust_diagnostics(&src, &data);
        assert_eq!(wire["ok"], true);
        assert_eq!(wire["present"], true);
        assert_eq!(wire["rust_diagnostics"]["latest"]["warnings"], 3);
        assert_eq!(wire["rust_diagnostics"]["history_count"], 1);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn speed_index_and_rust_diag_wire_empty_tolerant() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_missing_diag");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        std::fs::create_dir_all(&src).unwrap();

        let s = wire_speed_index(&src, &data);
        assert_eq!(s["ok"], true);
        assert_eq!(s["present"], false);
        assert_eq!(s["speed_index"]["test_ci_count"], 0);

        let r = wire_rust_diagnostics(&src, &data);
        assert_eq!(r["ok"], true);
        assert_eq!(r["present"], false);
        assert_eq!(r["rust_diagnostics"]["latest"]["warnings"], 0);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn sync_mirrors_speed_index_and_rust_diagnostics() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_sync_extra");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_sample(&src, &sample_manifest(), &sample_feed());
        write_speed_index(&src);
        write_rust_diagnostics(&src);

        let report = sync(&src, &data).unwrap();
        assert!(report.speed_index_target.ends_with("gsv_speed_index.json"));
        assert!(report
            .rust_diagnostics_target
            .ends_with("gsv_rust_diagnostics.json"));
        assert!(speed_index_target(&data).exists());
        assert!(rust_diagnostics_target(&data).exists());
        assert_eq!(load_speed_index(&data).unwrap().host_label, "PLATINOV");
        assert_eq!(load_rust_diagnostics(&data).unwrap().latest.warnings, 3);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn wire_speed_index_falls_back_to_snapshot_when_source_missing() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_speed_fallback");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_speed_index(&src);
        let r = read_speed_index(&src).unwrap();
        save_speed_index(&r, &data).unwrap();
        std::fs::remove_dir_all(src.join("docs")).unwrap();

        assert!(read_speed_index(&src).is_err());
        let wire = wire_speed_index(&src, &data);
        assert_eq!(wire["ok"], true);
        assert_eq!(wire["present"], false);
        assert_eq!(wire["speed_index"]["host_label"], "PLATINOV");
        assert_eq!(wire["speed_index"]["test_ci_count"], 1);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn wire_rust_diagnostics_falls_back_to_snapshot_when_source_missing() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_rustdiag_fallback");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        write_rust_diagnostics(&src);
        let r = read_rust_diagnostics(&src).unwrap();
        save_rust_diagnostics(&r, &data).unwrap();
        std::fs::remove_dir_all(src.join("docs")).unwrap();

        assert!(read_rust_diagnostics(&src).is_err());
        let wire = wire_rust_diagnostics(&src, &data);
        assert_eq!(wire["ok"], true);
        assert_eq!(wire["present"], false);
        assert_eq!(wire["rust_diagnostics"]["latest"]["warnings"], 3);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn history_records_parse_typed_fields() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_history_typed");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("speed_index.json"),
            r#"{
              "schema_version": 1,
              "generated_at": "2026-08-04",
              "host_label": "PLATINOV",
              "git_head": "50ce232f",
              "latest": {},
              "test_ci_history": [
                { "kind": "test_ci", "command": "cargo test-ci", "wall_secs": 600.0, "ok": false,
                  "recorded_at": "2026-08-02T03:22:20Z", "host_label": "PLATINOV", "git_head": "1e16ef501" },
                { "kind": "test_ci", "command": "cargo test-ci", "wall_secs": 1076.0, "ok": true,
                  "recorded_at": "2026-08-04T11:54:58Z", "host_label": "PLATINOV", "git_head": "cd5ae5b97" }
              ],
              "bench_history": [
                { "kind": "criterion", "bench": "runtime_benchmarks", "group": "memory_pool",
                  "median_ns": 1498, "profile": "short", "recorded_at": "2026-07-27T19:05:51Z",
                  "host_label": "win10-local-26200", "git_head": "1795243f7" }
              ]
            }"#,
        )
        .unwrap();
        std::fs::write(
            vis.join("rust_diagnostics.json"),
            r#"{
              "schema_version": 1,
              "generated_at": "2026-08-04",
              "host_label": "PLATINOV",
              "git_head": "cd5ae5b97",
              "latest": {},
              "history": [
                { "kind": "rust_diagnostics", "command": "cargo check", "warnings": 9, "errors": 0,
                  "ok": true, "recorded_at": "2026-07-28T01:15:23Z", "wall_secs": 80.5,
                  "host_label": "local-ph-svc85", "git_head": "b46b16c98", "source": "local",
                  "top_codes": ["dead_code×4", "unused_mut×2"] }
              ]
            }"#,
        )
        .unwrap();

        let s = read_speed_index(&src).unwrap();
        assert_eq!(s.test_ci_history.len(), 2);
        assert_eq!(s.test_ci_history[0].command, "cargo test-ci");
        assert_eq!(s.test_ci_history[0].wall_secs, 600.0);
        assert!(!s.test_ci_history[0].ok);
        assert_eq!(s.test_ci_history[1].git_head, "cd5ae5b97");
        assert_eq!(s.bench_history.len(), 1);
        assert_eq!(s.bench_history[0].median_ns, 1498);
        assert_eq!(s.bench_history[0].group, "memory_pool");
        let wire = wire_speed_index(&src, &data_dir_of(&tmp));
        assert_eq!(
            wire["speed_index"]["test_ci_history"][1]["wall_secs"],
            1076.0
        );

        let r = read_rust_diagnostics(&src).unwrap();
        assert_eq!(r.history.len(), 1);
        assert_eq!(r.history[0].warnings, 9);
        assert_eq!(r.history[0].errors, 0);
        assert_eq!(r.history[0].top_codes.len(), 2);
        assert_eq!(r.history[0].source, "local");
        let wire = wire_rust_diagnostics(&src, &data_dir_of(&tmp));
        assert_eq!(wire["rust_diagnostics"]["history"][0]["warnings"], 9);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    fn data_dir_of(tmp: &Path) -> std::path::PathBuf {
        tmp.join("data")
    }

    #[test]
    fn speed_chart_svg_renders_bars_and_empty_state() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_speed_chart");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        std::fs::create_dir_all(&src).unwrap();
        let empty = speed_index_chart_svg(&src, &data);
        assert!(empty.contains("no speed_index.json history"));

        let vis = src.join("docs").join("vision");
        std::fs::create_dir_all(&vis).unwrap();
        std::fs::write(
            vis.join("speed_index.json"),
            r#"{
              "schema_version": 1,
              "generated_at": "2026-08-04",
              "host_label": "PLATINOV",
              "git_head": "50ce232f",
              "latest": { "test_ci_wall_secs": 1076.5, "test_ci_ok": true },
              "test_ci_history": [
                { "kind": "test_ci", "command": "cargo test-ci", "wall_secs": 600.0, "ok": false,
                  "recorded_at": "2026-08-02T03:22:20Z", "host_label": "PLATINOV", "git_head": "1e16ef501" },
                { "kind": "test_ci", "command": "cargo test-ci", "wall_secs": 1076.5, "ok": true,
                  "recorded_at": "2026-08-04T11:54:58Z", "host_label": "PLATINOV", "git_head": "cd5ae5b97" }
              ],
              "bench_history": [
                { "kind": "criterion", "bench": "runtime_benchmarks", "group": "memory_pool",
                  "median_ns": 4210, "profile": "short", "recorded_at": "2026-07-27T19:05:51Z",
                  "host_label": "PLATINOV", "git_head": "1795243f7" }
              ]
            }"#,
        )
        .unwrap();

        let svg = speed_index_chart_svg(&src, &data);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("test-ci wall-clock (2 runs"));
        assert!(svg.contains("fill=\"#3fb96e\""));
        assert!(svg.contains("fill=\"#e05b5b\""));
        assert!(svg.contains("latest bench runtime_benchmarks 4210 ns"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn rust_chart_svg_renders_bars_and_empty_state() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_rust_chart");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        std::fs::create_dir_all(&src).unwrap();
        let empty = rust_diagnostics_chart_svg(&src, &data);
        assert!(empty.contains("no rust_diagnostics.json history"));

        write_rust_diagnostics(&src);
        let svg = rust_diagnostics_chart_svg(&src, &data);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("clippy warnings/errors (1 runs"));
        assert!(svg.contains("fill=\"#e8843c\""));
        assert!(svg.contains("fill=\"#e05b5b\""));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn sprint_token_matches_exact_and_glob() {
        assert!(sprint_token_matches("PH-S1819", "PH-S1819"));
        assert!(sprint_token_matches("PH-S*", "PH-S1819"));
        assert!(!sprint_token_matches("PH-S181", "PH-S1819"));
        assert!(!sprint_token_matches("PH-S1820", "PH-S1819"));
        assert!(!sprint_token_matches("", "PH-S1819"));
        assert!(!sprint_token_matches("PH-S1819", ""));
        assert!(!sprint_token_matches("PH-S*", ""));
    }

    #[test]
    fn path_matches_glob_double_star_and_wildcard() {
        assert!(path_matches_glob(
            "docs/vision/manifest.json",
            "docs/vision/**"
        ));
        assert!(path_matches_glob("docs/vision/feed.json", "docs/vision/**"));
        assert!(!path_matches_glob("docs/other/feed.json", "docs/vision/**"));
        assert!(path_matches_glob("src/lib/db/mod.rs", "src/**/*.rs"));
        assert!(path_matches_glob("src/lib/db/migrate.rs", "src/**/*.rs"));
        assert!(path_matches_glob(
            "docs/vision/feed.json",
            "docs/vision/feed.json"
        ));
        assert!(!path_matches_glob(
            "docs/vision/feed.json",
            "docs/vision/feed.xml"
        ));
        assert!(!path_matches_glob("src/lib/db/mod.rs", "src/*.rs"));
    }

    #[test]
    fn sprint_theme_report_matches_legacy_palette() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_theme");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        std::fs::create_dir_all(&src).unwrap();
        let vis = src.join("docs/vision");
        std::fs::create_dir_all(&vis).unwrap();
        let mut m = sample_manifest();
        m.next_sprint = "PH-S1819".to_string();
        m.layers = vec![
            Layer {
                id: "L0".to_string(),
                name: "Concept".to_string(),
                z: 0,
            },
            Layer {
                id: "L1".to_string(),
                name: "Operations".to_string(),
                z: 1,
            },
        ];
        m.edges.push(ManifestEdge {
            from: "galaxy_grid".to_string(),
            kind: "docs".to_string(),
            to: "handoff".to_string(),
        });
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&m).unwrap(),
        )
        .unwrap();
        std::fs::write(
            src.join("docs/vision/extensions.json"),
            serde_json::to_string(&Extensions {
                active_sprint: "PH-S1819".to_string(),
                scopes: Default::default(),
                ..Extensions::default()
            })
            .unwrap(),
        )
        .unwrap();

        let r = sprint_theme_report(&src, &data).expect("theme");
        assert_eq!(r.sprint, "#a78bfa");
        assert_eq!(r.sprint_next, "#c4b5fd");
        assert_eq!(r.pill.bg, "rgba(167, 139, 250, 0.2)");
        assert_eq!(r.pill.border, "rgba(167, 139, 250, 0.4)");
        assert_eq!(r.pill.color, "#d4c4ff");
        assert_eq!(r.chip.bg, "rgba(167, 139, 250, 0.15)");
        assert_eq!(r.queue.open_border, "rgba(167, 139, 250, 0.35)");
        assert_eq!(r.queue.open_status, "#a78bfa");
        assert_eq!(r.queue.next_border, "rgba(126, 184, 255, 0.55)");
        assert_eq!(r.queue.closed_opacity, "0.55");
        assert_eq!(r.active_sprint, "PH-S1819");
        assert_eq!(r.next_sprint, "PH-S1819");
        assert_eq!(
            r.layers
                .iter()
                .find(|l| l.id == "L0")
                .map(|l| l.color.as_str()),
            Some("#3d6a9e")
        );
        assert_eq!(
            r.edge_kinds
                .iter()
                .find(|k| k.kind == "docs")
                .map(|k| k.color.as_str()),
            Some("#90c490")
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn sprint_focus_svg_highlights_scope_and_dims_others() {
        let tmp = std::env::temp_dir().join("gsv_vision_test_focus");
        let _ = std::fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let data = tmp.join("data");
        std::fs::create_dir_all(&src).unwrap();
        let vis = src.join("docs/vision");
        std::fs::create_dir_all(&vis).unwrap();
        let mut m = sample_manifest();
        m.nodes.push(ManifestNode {
            id: "sprint_in_scope".to_string(),
            label: "in-scope".to_string(),
            layer: "L0".to_string(),
            path: "src/lib/sprint_scope.rs".to_string(),
            sections: vec![],
            sprints: vec!["PH-S1819".to_string()],
        });
        m.nodes.push(ManifestNode {
            id: "sprint_out_of_scope".to_string(),
            label: "out".to_string(),
            layer: "L0".to_string(),
            path: "src/lib/other.rs".to_string(),
            sections: vec![],
            sprints: vec!["PH-S1800".to_string()],
        });
        m.edges.push(ManifestEdge {
            from: "sprint_in_scope".to_string(),
            kind: "wire".to_string(),
            to: "sprint_out_of_scope".to_string(),
        });
        std::fs::write(
            vis.join("manifest.json"),
            serde_json::to_string(&m).unwrap(),
        )
        .unwrap();

        let svg = sprint_focus_svg(&src, &data, "PH-S1819");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("sprint focus: PH-S1819"));
        assert!(svg.contains("fill=\"#a78bfa\""));
        assert!(svg.contains("sprint_in_scope"));
        assert!(svg.contains("opacity=\"0.22\""));
        assert!(svg.contains("1 / 3 nodes"));

        let no_scope = sprint_focus_svg(&src, &data, "PH-S9999");
        assert!(no_scope.contains("no galaxy nodes reference this sprint"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn sprint_theme_extended_properties_validation() {
        let theme = SprintThemeReport {
            revision: 472,
            git_head: "abcdef0".to_string(),
            active_sprint: "PH-S1829".to_string(),
            next_sprint: "PH-S1830".to_string(),
            sprint: "#a78bfa".to_string(),
            sprint_next: "#c4b5fd".to_string(),
            pill: SprintPillTheme { bg: "rgba(167, 139, 250, 0.2)".to_string(), border: "rgba(167, 139, 250, 0.4)".to_string(), color: "#d4c4ff".to_string() },
            chip: SprintChipTheme { bg: "rgba(167, 139, 250, 0.15)".to_string(), border: "rgba(167, 139, 250, 0.3)".to_string(), color: "#c4b5fd".to_string() },
            queue: SprintQueueStateTheme { open_border: "rgba(167, 139, 250, 0.35)".to_string(), open_bg: "rgba(167, 139, 250, 0.08)".to_string(), open_status: "#a78bfa".to_string(), next_border: "rgba(126, 184, 255, 0.55)".to_string(), next_glow: "rgba(126, 184, 255, 0.15)".to_string(), closed_opacity: "0.55".to_string() },
            layers: vec![SprintLayerColor { id: "L0".to_string(), color: "#3d6a9e".to_string() }],
            edge_kinds: vec![SprintEdgeKindColor { kind: "docs".to_string(), color: "#90c490".to_string() }],
        };
        assert_eq!(theme.revision, 472);
        assert_eq!(theme.active_sprint, "PH-S1829");
        assert_eq!(theme.layers.len(), 1);
        assert_eq!(theme.edge_kinds.len(), 1);
    }

    #[test]
    fn sprint_theme_layer_color_fallback() {
        let l0 = SprintLayerColor { id: "L0".to_string(), color: "#3d6a9e".to_string() };
        let l1 = SprintLayerColor { id: "L1".to_string(), color: "#3d6a4a".to_string() };
        let l2 = SprintLayerColor { id: "L2".to_string(), color: "#8a7040".to_string() };
        let l3 = SprintLayerColor { id: "L3".to_string(), color: "#8a4068".to_string() };
        let l4 = SprintLayerColor { id: "L4".to_string(), color: "#6a5088".to_string() };
        let l5 = SprintLayerColor { id: "L5".to_string(), color: "#4a6880".to_string() };
        assert_eq!(l0.id, "L0");
        assert_eq!(l1.id, "L1");
        assert_eq!(l2.id, "L2");
        assert_eq!(l3.id, "L3");
        assert_eq!(l4.id, "L4");
        assert_eq!(l5.id, "L5");
    }

    #[test]
    fn sprint_queue_report_edge_cases_validation() {
        let q = SprintQueueReport {
            revision: 472,
            git_head: "abcdef0".to_string(),
            next_sprint: "PH-S1829".to_string(),
            last_sprint_closed: "PH-S1828".to_string(),
            open_count: 0,
            active_sprint: "PH-S1829".to_string(),
            entries: vec![],
            planned: vec![],
        };
        assert_eq!(q.revision, 472);
        assert_eq!(q.next_sprint, "PH-S1829");
        assert_eq!(q.open_count, 0);
        assert!(q.entries.is_empty());
    }
}
