//! Merge git-tracked repo files into `docs/vision/manifest.json` (incremental map growth).
//!
//! ```text
//! cargo run --bin poolai-vision-sync
//! cargo run --bin poolai-vision-sync -- --dry-run
//! ```

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const MANIFEST_REL: &str = "docs/vision/manifest.json";
const FEED_REL: &str = "docs/vision/feed.json";
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const FEED_CLOSED_CAP: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
struct SprintQueueEntry {
    row: u32,
    id: String,
    title: String,
    deps: String,
    acceptance: String,
    status: String,
    open: bool,
}

const SKIP_PREFIXES: &[&str] = &[
    "target/",
    "data/audit/",
    "comitmsg/",
    "docs/archive/",
    ".git/",
];

const SKIP_SUFFIXES: &[&str] = &["/mod.rs"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn git_tracked_files(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git ls-files: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output
        .stdout
        .split(|&b| b == 0)
        .filter(|chunk| !chunk.is_empty())
        .filter_map(|chunk| std::str::from_utf8(chunk).ok().map(str::to_string))
        .collect())
}

fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn should_index(path: &str) -> bool {
    let p = normalize_path(path);
    if SKIP_PREFIXES.iter().any(|pre| p.starts_with(pre)) {
        return false;
    }
    if SKIP_SUFFIXES.iter().any(|suf| p.ends_with(suf)) {
        return false;
    }
    if p.starts_with("bin/commit-")
        || p.starts_with("bin/fix-commit-")
        || p.starts_with("bin/push-")
    {
        return false;
    }

    let lower = p.to_ascii_lowercase();
    if !(lower.ends_with(".md")
        || lower.ends_with(".rs")
        || lower.ends_with(".ts")
        || lower.ends_with(".yaml")
        || lower.ends_with(".yml")
        || lower.ends_with(".toml")
        || lower.ends_with(".js"))
    {
        return false;
    }

    p.starts_with("docs/")
        || p.starts_with("src/")
        || p.starts_with("e2e/")
        || p.starts_with("crates/")
        || p.starts_with("tests/")
        || p == "Cargo.toml"
        || p.starts_with(".cargo/")
        || p.starts_with(".cursor/rules/")
        || p.starts_with(".cursor/skills/")
}

fn infer_layer(path: &str) -> &'static str {
    let p = normalize_path(path);
    if p.starts_with("docs/concept/") {
        return "L0";
    }
    if p.starts_with("docs/development/")
        || p.starts_with("docs/vision/")
        || p.starts_with(".cursor/commands/")
    {
        return "L1";
    }
    if p.starts_with("docs/") {
        return "L2";
    }
    if p == "Cargo.toml" || p.starts_with(".cargo/") {
        return "L5";
    }
    if p == "src/lib.rs" || p.starts_with("crates/") {
        return "L4";
    }
    "L3"
}

fn path_slug(path: &str) -> String {
    normalize_path(path)
        .trim_start_matches("./")
        .replace(['/', '.'], "_")
        .replace('-', "_")
}

fn label_from_path(path: &str) -> String {
    let p = normalize_path(path);
    Path::new(&p)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&p)
        .to_string()
}

fn hub_links(path: &str) -> Vec<(&'static str, &'static str)> {
    let p = normalize_path(path);
    match p.as_str() {
        p if p.starts_with("docs/catalog/") || p == "docs/openapi.yaml" => {
            vec![("fm", "catalog"), ("galaxy_grid", "implements")]
        }
        p if p.starts_with("docs/development/") => vec![("handoff", "session-tracks")],
        p if p.starts_with("docs/vision/") && p != "docs/vision/manifest.json" => {
            vec![("handoff", "session-tracks")]
        }
        p if p.starts_with("docs/concept/") => vec![("galaxy_grid", "concept-ref")],
        p if p.starts_with("src/grid/") => vec![("galaxy_grid", "implements")],
        p if p.starts_with("src/job/") => vec![("galaxy_grid", "implements")],
        "src/network/api/jobs.rs" => vec![("job_types", "implements")],
        "src/network/api/grid.rs" => vec![("grid_pricing_api", "implements")],
        "src/bin/poolai-worker.rs" => vec![("poolai_worker", "implements")],
        "src/bin/poolai_openapi_gap_audit.rs" => vec![("fm", "catalog")],
        "src/bin/poolai_vision_sync.rs" => vec![("handoff", "session-tracks")],
        p if p.starts_with("e2e/tests/jobs_lease") => vec![("job_types", "sprint-scope")],
        p if p.starts_with("e2e/tests/grid_") => vec![("grid_dispatch", "sprint-scope")],
        p if p.starts_with("crates/") => vec![("crate_solana", "implements")],
        "Cargo.toml" => vec![("fm", "catalog")],
        p if p.starts_with(".cargo/") => vec![("cargo_toml", "workspace")],
        p if p.starts_with("tests/jobs_") => vec![("job_types", "sprint-scope")],
        _ => vec![],
    }
}

fn node_id_for_path(path: &str, taken: &BTreeSet<String>) -> String {
    let base = path_slug(path);
    if !taken.contains(&base) {
        return base;
    }
    let with_dir = normalize_path(path).replace('/', "__");
    if !taken.contains(&with_dir) {
        return with_dir;
    }
    format!("{base}_auto")
}

fn load_manifest(path: &Path) -> Result<Value, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse manifest: {e}"))
}

fn write_manifest(path: &Path, manifest: &Value) -> Result<(), String> {
    write_json_file(path, manifest)
}

fn write_json_file(path: &Path, value: &Value) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(value).map_err(|e| format!("serialize: {e}"))?;
    fs::write(path, pretty + "\n").map_err(|e| format!("write {}: {e}", path.display()))
}

fn sync_manifest(manifest: &mut Value, paths: &[String]) -> (usize, usize) {
    let mut known_paths: BTreeMap<String, String> = BTreeMap::new();
    let mut taken_ids: BTreeSet<String> = BTreeSet::new();
    if let Some(nodes) = manifest.get("nodes").and_then(Value::as_array) {
        for node in nodes {
            if let (Some(p), Some(id)) = (
                node.get("path").and_then(Value::as_str),
                node.get("id").and_then(Value::as_str),
            ) {
                known_paths.insert(normalize_path(p), id.to_string());
                taken_ids.insert(id.to_string());
            }
        }
    }

    let mut edge_keys: BTreeSet<String> = BTreeSet::new();
    if let Some(edges) = manifest.get("edges").and_then(Value::as_array) {
        for edge in edges {
            if let (Some(a), Some(b)) = (
                edge.get("from").and_then(Value::as_str),
                edge.get("to").and_then(Value::as_str),
            ) {
                edge_keys.insert(format!("{a}|{b}"));
            }
        }
    }

    let mut added_nodes = 0usize;
    let mut added_edges = 0usize;
    let mut new_nodes: Vec<Value> = Vec::new();
    let mut new_edges: Vec<Value> = Vec::new();

    for path in paths {
        if !should_index(path) {
            continue;
        }
        let norm = normalize_path(path);
        let node_id = if let Some(id) = known_paths.get(&norm) {
            id.clone()
        } else {
            let id = node_id_for_path(&norm, &taken_ids);
            taken_ids.insert(id.clone());
            known_paths.insert(norm.clone(), id.clone());
            new_nodes.push(json!({
                "id": id,
                "label": label_from_path(&norm),
                "path": norm,
                "layer": infer_layer(&norm),
                "auto_synced": true
            }));
            added_nodes += 1;
            id
        };

        for (hub_id, kind) in hub_links(&norm) {
            if !taken_ids.contains(hub_id) {
                continue;
            }
            let key = format!("{hub_id}|{node_id}");
            if edge_keys.contains(&key) {
                continue;
            }
            new_edges.push(json!({
                "from": hub_id,
                "to": node_id,
                "kind": kind,
                "auto_synced": true
            }));
            edge_keys.insert(key);
            added_edges += 1;
        }
    }

    if added_nodes > 0 {
        manifest
            .get_mut("nodes")
            .and_then(Value::as_array_mut)
            .expect("manifest.nodes array")
            .extend(new_nodes);
    }
    if added_edges > 0 {
        manifest
            .get_mut("edges")
            .and_then(Value::as_array_mut)
            .expect("manifest.edges array")
            .extend(new_edges);
    }

    if added_nodes > 0 || added_edges > 0 {
        let rev = manifest
            .get("revision")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            + 1;
        manifest["revision"] = json!(rev);
        manifest["auto_sync_at"] = json!(today_iso());
    }

    (added_nodes, added_edges)
}

fn strip_md_cell(raw: &str) -> String {
    raw.trim()
        .trim_matches('`')
        .replace("**", "")
        .trim()
        .to_string()
}

fn sprint_status_from_cell(cell: &str) -> (String, bool) {
    let plain = strip_md_cell(cell).to_ascii_lowercase();
    if plain.contains('✅') || plain.contains("closed") {
        return ("closed".to_string(), false);
    }
    if plain.contains("blocked") {
        return ("blocked".to_string(), false);
    }
    if plain.contains("deferred") {
        return ("deferred".to_string(), false);
    }
    if plain.contains("відкрито") || plain == "open" {
        return ("open".to_string(), true);
    }
    ("open".to_string(), true)
}

/// Parse FM §5.12 research backlog table rows (`| N | **PH-Snnn** | … |`).
fn parse_fm_sprint_queue_section(section: &str) -> Vec<SprintQueueEntry> {
    let mut out = Vec::new();
    for line in section.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') || !trimmed.contains("PH-S") {
            continue;
        }
        let cells: Vec<&str> = trimmed.split('|').map(str::trim).collect();
        // leading/trailing empty from split
        if cells.len() < 7 {
            continue;
        }
        let row = match cells[1].parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let id_cell = cells[2];
        let Some(id) = id_cell
            .strip_prefix("**")
            .and_then(|s| s.strip_suffix("**"))
            .filter(|s| s.starts_with("PH-S"))
        else {
            continue;
        };
        let title = strip_md_cell(cells[3]);
        let deps = strip_md_cell(cells[4]);
        let acceptance = strip_md_cell(cells[5]);
        let (status, open) = sprint_status_from_cell(cells[6]);
        out.push(SprintQueueEntry {
            row,
            id: id.to_string(),
            title,
            deps,
            acceptance,
            status,
            open,
        });
    }
    out
}

fn extract_fm_section_512(content: &str) -> Option<&str> {
    let start = content.find("### 5.12")?;
    let rest = &content[start..];
    let end = rest[10..]
        .find("\n### 5.")
        .map(|i| 10 + i)
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn parse_fm_sprint_queue(content: &str) -> Vec<SprintQueueEntry> {
    let Some(section) = extract_fm_section_512(content) else {
        return Vec::new();
    };
    parse_fm_sprint_queue_section(section)
}

fn sprint_queue_json(entries: &[SprintQueueEntry]) -> Value {
    Value::Array(
        entries
            .iter()
            .map(|e| {
                json!({
                    "row": e.row,
                    "id": e.id,
                    "title": e.title,
                    "deps": e.deps,
                    "acceptance": e.acceptance,
                    "status": e.status,
                    "open": e.open,
                })
            })
            .collect(),
    )
}

fn derive_sprint_meta(entries: &[SprintQueueEntry]) -> (Option<String>, Option<String>, u32) {
    let open_count = entries.iter().filter(|e| e.open).count() as u32;
    let next_sprint = entries.iter().find(|e| e.open).map(|e| e.id.clone());
    let last_closed = entries
        .iter()
        .filter(|e| !e.open && e.status == "closed")
        .map(|e| e.id.clone())
        .last();
    (next_sprint, last_closed, open_count)
}

fn bump_manifest_revision(manifest: &mut Value) {
    let rev = manifest
        .get("revision")
        .and_then(Value::as_u64)
        .unwrap_or(0)
        + 1;
    manifest["revision"] = json!(rev);
    manifest["auto_sync_at"] = json!(today_iso());
}

fn read_fm_sprint_entries(root: &Path) -> Vec<SprintQueueEntry> {
    let fm_path = root.join(FM_REL);
    let content = match fs::read_to_string(&fm_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("warn: read {}: {e}", fm_path.display());
            return Vec::new();
        }
    };
    parse_fm_sprint_queue(&content)
}

fn sync_fm_sprint_queue(manifest: &mut Value, entries: &[SprintQueueEntry]) -> bool {
    if entries.is_empty() {
        return false;
    }
    let queue = sprint_queue_json(entries);
    let (next_sprint, last_closed, open_count) = derive_sprint_meta(entries);
    let prev_next = manifest.get("next_sprint").and_then(Value::as_str);
    let prev_last = manifest.get("last_sprint_closed").and_then(Value::as_str);
    let changed = manifest.get("sprint_queue") != Some(&queue)
        || manifest
            .get("sprint_queue_open_count")
            .and_then(Value::as_u64)
            != Some(u64::from(open_count))
        || prev_next != next_sprint.as_deref()
        || prev_last != last_closed.as_deref();
    manifest["sprint_queue"] = queue;
    manifest["sprint_queue_open_count"] = json!(open_count);
    if let Some(ns) = next_sprint {
        manifest["next_sprint"] = json!(ns);
    }
    if let Some(lc) = last_closed {
        manifest["last_sprint_closed"] = json!(lc);
    }
    changed
}

fn feed_item_json(entry: &SprintQueueEntry, next_sprint: Option<&str>) -> Value {
    let category = if entry.open { "open" } else { "closed" };
    let mut item = json!({
        "id": entry.id,
        "title": entry.title,
        "category": category,
        "status": entry.status,
        "published": today_iso(),
        "summary": entry.acceptance,
        "link": "docs/vision/index.html#sprint-queue"
    });
    if entry.open && next_sprint == Some(entry.id.as_str()) {
        item["next"] = json!(true);
    }
    item
}

/// RSS-style sprint feed for the vision ticker (open queue + recent closed).
fn build_sprint_feed(entries: &[SprintQueueEntry]) -> Value {
    let (next_sprint, _, _) = derive_sprint_meta(entries);
    let next_ref = next_sprint.as_deref();
    let mut items: Vec<Value> = entries
        .iter()
        .filter(|e| e.open)
        .map(|e| feed_item_json(e, next_ref))
        .collect();
    let closed_recent: Vec<&SprintQueueEntry> = entries
        .iter()
        .filter(|e| !e.open && e.status == "closed")
        .rev()
        .take(FEED_CLOSED_CAP)
        .collect();
    for entry in closed_recent {
        items.push(feed_item_json(entry, None));
    }
    json!({
        "updated_at": today_iso(),
        "title": "PoolAI Vision Sprint Feed",
        "link": "docs/vision/index.html",
        "description": "RSS-style ticker of FM §5.12 sprint queue (open + recent closed)",
        "items": items
    })
}

fn sync_sprint_feed(root: &Path, entries: &[SprintQueueEntry]) -> bool {
    if entries.is_empty() {
        return false;
    }
    let feed = build_sprint_feed(entries);
    let feed_path = root.join(FEED_REL);
    let prev = fs::read_to_string(&feed_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());
    let changed = prev.as_ref() != Some(&feed);
    if changed {
        if let Err(e) = write_json_file(&feed_path, &feed) {
            eprintln!("warn: write {}: {e}", feed_path.display());
            return false;
        }
    }
    changed
}

fn today_iso() -> String {
    std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .map(iso_from_epoch)
        .unwrap_or_else(|| "2026-05-28".to_string())
}

fn iso_from_epoch(epoch: i64) -> String {
    let days = epoch / 86_400;
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mp < 10 { y } else { y + 1 };
    format!("{y:04}-{m:02}-{d:02}")
}

fn main() -> ExitCode {
    let dry_run = std::env::args().any(|a| a == "--dry-run");
    let root = repo_root();
    let manifest_path = root.join(MANIFEST_REL);

    let paths = match git_tracked_files(&root) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    let mut manifest = match load_manifest(&manifest_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    let (added_nodes, added_edges) = sync_manifest(&mut manifest, &paths);
    let entries = read_fm_sprint_entries(&root);
    let queue_changed = sync_fm_sprint_queue(&mut manifest, &entries);
    let feed_changed = sync_sprint_feed(&root, &entries);
    if queue_changed && added_nodes == 0 && added_edges == 0 {
        bump_manifest_revision(&mut manifest);
    }
    println!(
        "vision sync: +{added_nodes} nodes, +{added_edges} edges, sprint_queue {}, feed {} (revision {})",
        if queue_changed {
            "updated"
        } else {
            "unchanged"
        },
        if feed_changed {
            "updated"
        } else {
            "unchanged"
        },
        manifest
            .get("revision")
            .and_then(Value::as_u64)
            .unwrap_or(0)
    );

    if added_nodes == 0 && added_edges == 0 && !queue_changed && !feed_changed {
        return ExitCode::SUCCESS;
    }

    if dry_run {
        println!("dry-run: manifest/feed not written");
        return ExitCode::SUCCESS;
    }

    if queue_changed || added_nodes > 0 || added_edges > 0 {
        if let Err(e) = write_manifest(&manifest_path, &manifest) {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_index_openapi_and_skip_mod_rs() {
        assert!(should_index("docs/openapi.yaml"));
        assert!(!should_index("src/job/mod.rs"));
        assert!(!should_index("target/debug/foo.rs"));
    }

    #[test]
    fn infer_layer_openapi_is_l2() {
        assert_eq!(infer_layer("docs/openapi.yaml"), "L2");
        assert_eq!(infer_layer("docs/concept/POOLAI_GALAXY_GRID.md"), "L0");
        assert_eq!(infer_layer("src/grid/dispatch.rs"), "L3");
    }

    #[test]
    fn parse_fm_sprint_queue_open_and_closed() {
        let sample = r###"
### 5.12 Research backlog PH-S65+ (Galaxy wire / ops, 2026-05-27)

| 125 | **PH-S190** | Vision filter dropdowns | PH-S188 | dropdown menus | **✅** |
| 126 | **PH-S191** | Vision sprint queue panel | FM §5.12 | sprint_queue panel | відкрито |
| 127 | **PH-S192** | Vision overview LOD | PH-S115 | minimap | відкрито |

### 5.13 Rust ratio band
"###;
        let entries = parse_fm_sprint_queue(sample);
        assert_eq!(entries.len(), 3);
        assert!(!entries[0].open);
        assert_eq!(entries[0].id, "PH-S190");
        assert!(entries[1].open);
        assert_eq!(entries[1].id, "PH-S191");
        let (next, last, open) = derive_sprint_meta(&entries);
        assert_eq!(next.as_deref(), Some("PH-S191"));
        assert_eq!(last.as_deref(), Some("PH-S190"));
        assert_eq!(open, 2);
    }

    #[test]
    fn build_sprint_feed_open_then_recent_closed() {
        let entries = vec![
            SprintQueueEntry {
                row: 1,
                id: "PH-S198".into(),
                title: "Topology labels".into(),
                deps: String::new(),
                acceptance: "hub labels".into(),
                status: "closed".into(),
                open: false,
            },
            SprintQueueEntry {
                row: 2,
                id: "PH-S199".into(),
                title: "Map hit-test".into(),
                deps: String::new(),
                acceptance: "edge trace".into(),
                status: "closed".into(),
                open: false,
            },
            SprintQueueEntry {
                row: 3,
                id: "PH-S200".into(),
                title: "Feed ticker".into(),
                deps: String::new(),
                acceptance: "feed.json panel".into(),
                status: "open".into(),
                open: true,
            },
            SprintQueueEntry {
                row: 4,
                id: "PH-S201".into(),
                title: "Post-push hook".into(),
                deps: String::new(),
                acceptance: "cursor hooks".into(),
                status: "open".into(),
                open: true,
            },
        ];
        let feed = build_sprint_feed(&entries);
        let items = feed["items"].as_array().unwrap();
        assert_eq!(items.len(), 4);
        assert_eq!(items[0]["id"], "PH-S200");
        assert_eq!(items[0]["category"], "open");
        assert_eq!(items[0]["next"], true);
        assert_eq!(items[1]["id"], "PH-S201");
        assert_eq!(items[2]["id"], "PH-S199");
        assert_eq!(items[3]["id"], "PH-S198");
    }

    #[test]
    fn sync_adds_missing_node_and_edge() {
        let mut manifest = json!({
            "revision": 1,
            "nodes": [
                { "id": "fm", "label": "FM", "path": "docs/catalog/FUNCTION_MANAGEMENT.md", "layer": "L2" },
                { "id": "galaxy_grid", "label": "Galaxy", "path": "docs/concept/POOLAI_GALAXY_GRID.md", "layer": "L0" }
            ],
            "edges": []
        });
        let paths = vec!["docs/openapi.yaml".to_string()];
        let (n, e) = sync_manifest(&mut manifest, &paths);
        assert_eq!(n, 1);
        assert!(e >= 1);
        let nodes = manifest["nodes"].as_array().unwrap();
        assert!(nodes.iter().any(|n| n["path"] == "docs/openapi.yaml"));
    }
}
