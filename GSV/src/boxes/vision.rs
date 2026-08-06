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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// Write-run: read the live sources and persist both snapshots.
pub fn sync(repo_root: &Path, data_dir: &Path) -> Result<SyncReport, String> {
    let manifest = read_manifest(repo_root)?;
    let feed = read_feed(repo_root)?;
    let extensions = read_extensions(repo_root)?;
    save_manifest(&manifest, data_dir)?;
    save_feed(&feed, data_dir)?;
    save_extensions(&extensions, data_dir)?;
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
            link: "docs/vision/index.html#sprint-queue".to_string(),
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
}
