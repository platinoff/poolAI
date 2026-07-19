//! Merge git-tracked repo files into `docs/vision/manifest.json` (incremental map growth).
//!
//! ```text
//! cargo run --bin poolai-vision-sync
//! cargo run --bin poolai-vision-sync -- --dry-run
//! cargo run --bin poolai-vision-sync -- --check
//! ```

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const MANIFEST_REL: &str = "docs/vision/manifest.json";
const FEED_REL: &str = "docs/vision/feed.json";
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const EXTENSIONS_REL: &str = "docs/vision/extensions.json";
const FEED_CLOSED_CAP: usize = 12;

const DOCS_VISION_MDC: &str = ".cursor/rules/docs-vision.mdc";

const DOCS_VISION_CANON_PATHS: &[&str] = &[
    "docs/vision/extensions.json",
    "docs/vision/vision.svg",
    "docs/vision/index.html",
    "docs/vision/README.md",
];

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
    let vision_artifact = p.starts_with("docs/vision/")
        && (lower.ends_with(".html")
            || lower.ends_with(".css")
            || lower.ends_with(".svg")
            || lower.ends_with(".json"));

    if !(lower.ends_with(".md")
        || lower.ends_with(".mdc")
        || lower.ends_with(".rs")
        || lower.ends_with(".ts")
        || lower.ends_with(".yaml")
        || lower.ends_with(".yml")
        || lower.ends_with(".toml")
        || lower.ends_with(".js")
        || vision_artifact)
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
        || p.starts_with(".cursor/rules/")
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
        DOCS_VISION_MDC => vec![("handoff", "sprint-scope")],
        p if p.starts_with(".cursor/rules/") && p.ends_with(".mdc") => {
            vec![("handoff", "session-tracks")]
        }
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

/// Numeric suffix from `PH-S1109` → `1109`.
fn parse_sprint_serial(id: &str) -> Option<u32> {
    let rest = id.strip_prefix("PH-S")?;
    rest.parse().ok()
}

/// When §5.12 has 0 open, next band start from `Master horizon: PH-S1119…S1128`.
fn parse_master_horizon_next(section: &str) -> Option<String> {
    for line in section.lines() {
        if !line.contains("Master horizon") {
            continue;
        }
        let marker = "PH-S";
        let idx = line.find(marker)?;
        let rest = &line[idx..];
        let digit_end = rest[4..]
            .find(|c: char| !c.is_ascii_digit())
            .map(|i| 4 + i)
            .unwrap_or(rest.len());
        let id = &rest[..digit_end];
        if id.len() > 4 {
            return Some(id.to_string());
        }
    }
    None
}

/// Closed sprints sorted by PH-S serial descending (deduped by id).
fn closed_entries_by_serial_desc<'a>(entries: &'a [SprintQueueEntry]) -> Vec<&'a SprintQueueEntry> {
    let mut closed: Vec<&SprintQueueEntry> = entries
        .iter()
        .filter(|e| !e.open && e.status == "closed")
        .collect();
    closed.sort_by_key(|e| std::cmp::Reverse(parse_sprint_serial(&e.id).unwrap_or(0)));
    let mut seen = BTreeSet::new();
    closed.retain(|e| seen.insert(e.id.as_str()));
    closed
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

fn derive_sprint_meta(
    entries: &[SprintQueueEntry],
    fm_section: Option<&str>,
) -> (Option<String>, Option<String>, u32) {
    let open_count = entries.iter().filter(|e| e.open).count() as u32;
    let next_sprint = match entries.iter().find(|e| e.open).map(|e| e.id.clone()) {
        Some(open) => Some(open),
        None if open_count == 0 => fm_section.and_then(parse_master_horizon_next),
        None => None,
    };
    let last_closed = closed_entries_by_serial_desc(entries)
        .first()
        .map(|e| e.id.clone());
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

fn read_fm_sprint_bundle(root: &Path) -> (Vec<SprintQueueEntry>, String) {
    let fm_path = root.join(FM_REL);
    let content = match fs::read_to_string(&fm_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("warn: read {}: {e}", fm_path.display());
            return (Vec::new(), String::new());
        }
    };
    let section = extract_fm_section_512(&content).unwrap_or("").to_string();
    let entries = parse_fm_sprint_queue_section(&section);
    (entries, section)
}

fn sync_fm_sprint_queue(
    manifest: &mut Value,
    entries: &[SprintQueueEntry],
    fm_section: Option<&str>,
) -> bool {
    if entries.is_empty() {
        return false;
    }
    let queue = sprint_queue_json(entries);
    let (next_sprint, last_closed, open_count) = derive_sprint_meta(entries, fm_section);
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
    } else if manifest.get("next_sprint").is_some() {
        manifest
            .as_object_mut()
            .expect("manifest object")
            .remove("next_sprint");
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

/// RSS-style sprint feed for the vision ticker (open queue + recent closed by PH-S serial).
fn build_sprint_feed(entries: &[SprintQueueEntry], fm_section: Option<&str>) -> Value {
    let (next_sprint, _, _) = derive_sprint_meta(entries, fm_section);
    let next_ref = next_sprint.as_deref();
    let mut items: Vec<Value> = entries
        .iter()
        .filter(|e| e.open)
        .map(|e| feed_item_json(e, next_ref))
        .collect();
    for entry in closed_entries_by_serial_desc(entries)
        .into_iter()
        .take(FEED_CLOSED_CAP)
    {
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

fn sync_extensions_active_sprint(root: &Path, next_sprint: Option<&str>) -> bool {
    let path = root.join(EXTENSIONS_REL);
    let mut ext = match fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
    {
        Some(v) => v,
        None => return false,
    };
    let prev = ext.get("active_sprint").and_then(Value::as_str);
    if prev == next_sprint {
        return false;
    }
    match next_sprint {
        Some(ns) => ext["active_sprint"] = json!(ns),
        None => {
            ext.as_object_mut()
                .expect("extensions object")
                .remove("active_sprint");
        }
    }
    ext["updated_at"] = json!(today_iso());
    if let Err(e) = write_json_file(&path, &ext) {
        eprintln!("warn: write {}: {e}", path.display());
        return false;
    }
    true
}

fn sync_sprint_feed(root: &Path, entries: &[SprintQueueEntry], fm_section: Option<&str>) -> bool {
    if entries.is_empty() {
        return false;
    }
    let feed = build_sprint_feed(entries, fm_section);
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

/// Parse `Vision rev **N**` from FM §5.12 footer line.
fn parse_fm_vision_revision(section: &str) -> Option<u64> {
    for line in section.lines() {
        let marker = "Vision rev **";
        let Some(start) = line.find(marker) else {
            continue;
        };
        let rest = &line[start + marker.len()..];
        let end = rest.find("**")?;
        return rest[..end].parse().ok();
    }
    None
}

fn manifest_revision(manifest: &Value) -> u64 {
    manifest
        .get("revision")
        .and_then(Value::as_u64)
        .unwrap_or(0)
}

fn manifest_indexed_paths(manifest: &Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Some(nodes) = manifest.get("nodes").and_then(Value::as_array) else {
        return out;
    };
    for node in nodes {
        if let Some(path) = node.get("path").and_then(Value::as_str) {
            out.insert(normalize_path(path));
        }
    }
    out
}

fn tracked_vdt_mdc_files(root: &Path) -> Result<Vec<String>, String> {
    Ok(git_tracked_files(root)?
        .into_iter()
        .map(|p| normalize_path(&p))
        .filter(|p| p.starts_with(".cursor/rules/") && p.ends_with(".mdc"))
        .collect())
}

/// PH-S227: manifest nodes ↔ git-tracked VDT `.mdc` rules + `docs-vision.mdc` canon cross-links.
fn collect_mdc_manifest_drift(
    root: &Path,
    manifest: &Value,
    extensions: Option<&Value>,
) -> Vec<String> {
    let mut errors = Vec::new();
    let indexed = manifest_indexed_paths(manifest);

    let mdc_files = match tracked_vdt_mdc_files(root) {
        Ok(files) => files,
        Err(e) => {
            errors.push(e);
            return errors;
        }
    };

    for mdc in &mdc_files {
        if !indexed.contains(mdc) {
            errors.push(format!(
                "manifest missing node for VDT rule {mdc} (run poolai-vision-sync)"
            ));
        }
    }

    for canon in DOCS_VISION_CANON_PATHS {
        if !indexed.contains(*canon) {
            errors.push(format!("manifest missing docs-vision canon path {canon}"));
        }
    }

    if let Some(ext) = extensions {
        let listed = ext
            .get("extension_policy")
            .and_then(|p| p.get(".mdc"))
            .and_then(|m| m.get("vision_files"))
            .and_then(|v| v.as_array());
        match listed {
            Some(files) => {
                let has_docs_vision = files
                    .iter()
                    .filter_map(Value::as_str)
                    .any(|p| normalize_path(p) == DOCS_VISION_MDC);
                if !has_docs_vision {
                    errors.push(format!(
                        "extensions.extension_policy.mdc.vision_files missing {DOCS_VISION_MDC}"
                    ));
                }
            }
            None => errors.push("extensions.json missing extension_policy.mdc".to_string()),
        }
    }

    let mdc_path = root.join(DOCS_VISION_MDC);
    let mdc_content = match fs::read_to_string(&mdc_path) {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("read {DOCS_VISION_MDC}: {e}"));
            return errors;
        }
    };

    if !mdc_content.contains("poolai-vision-sync") {
        errors.push(format!(
            "{DOCS_VISION_MDC} should reference poolai-vision-sync (VDT autosync)"
        ));
    }
    for canon in DOCS_VISION_CANON_PATHS {
        if !mdc_content.contains(canon) {
            errors.push(format!("{DOCS_VISION_MDC} missing cross-link to {canon}"));
        }
    }

    errors
}

/// Compare on-disk manifest sprint metadata with FM §5.12 (+ optional extensions active_sprint).
fn collect_manifest_fm_drift(
    manifest: &Value,
    entries: &[SprintQueueEntry],
    fm_section: &str,
    extensions: Option<&Value>,
) -> Vec<String> {
    let mut errors = Vec::new();
    if entries.is_empty() {
        errors.push("FM §5.12 sprint queue parse returned no rows".to_string());
        return errors;
    }

    let expected_queue = sprint_queue_json(entries);
    if manifest.get("sprint_queue") != Some(&expected_queue) {
        errors.push(
            "manifest.sprint_queue differs from FM §5.12 (run poolai-vision-sync)".to_string(),
        );
    }

    let (next_sprint, last_closed, open_count) = derive_sprint_meta(entries, Some(fm_section));
    let manifest_next = manifest.get("next_sprint").and_then(Value::as_str);
    if manifest_next != next_sprint.as_deref() {
        errors.push(format!(
            "manifest.next_sprint={manifest_next:?} expected {:?}",
            next_sprint
        ));
    }
    let manifest_last = manifest.get("last_sprint_closed").and_then(Value::as_str);
    if manifest_last != last_closed.as_deref() {
        errors.push(format!(
            "manifest.last_sprint_closed={manifest_last:?} expected {:?}",
            last_closed
        ));
    }
    let manifest_open = manifest
        .get("sprint_queue_open_count")
        .and_then(Value::as_u64);
    if manifest_open != Some(u64::from(open_count)) {
        errors.push(format!(
            "manifest.sprint_queue_open_count={manifest_open:?} expected {open_count}"
        ));
    }

    if let Some(fm_rev) = parse_fm_vision_revision(fm_section) {
        let manifest_rev = manifest_revision(manifest);
        if fm_rev != manifest_rev {
            errors.push(format!(
                "FM Vision rev {fm_rev} != manifest.revision {manifest_rev}"
            ));
        }
    } else {
        errors.push("FM §5.12 missing Vision rev **N** footer".to_string());
    }

    if let Some(ext) = extensions {
        let active = ext.get("active_sprint").and_then(Value::as_str);
        if active != next_sprint.as_deref() {
            errors.push(format!(
                "extensions.active_sprint={active:?} expected {:?}",
                next_sprint
            ));
        }
    }

    errors
}

fn run_drift_check(root: &Path) -> ExitCode {
    let manifest_path = root.join(MANIFEST_REL);
    let fm_path = root.join(FM_REL);
    let extensions_path = root.join(EXTENSIONS_REL);

    let manifest = match load_manifest(&manifest_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let fm_content = match fs::read_to_string(&fm_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: read {}: {e}", fm_path.display());
            return ExitCode::from(2);
        }
    };
    let fm_section = extract_fm_section_512(&fm_content).unwrap_or("");
    let entries = parse_fm_sprint_queue(&fm_content);
    let extensions = fs::read_to_string(&extensions_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());

    let errors = collect_manifest_fm_drift(&manifest, &entries, fm_section, extensions.as_ref());
    let mut errors: Vec<String> = errors;
    errors.extend(collect_mdc_manifest_drift(
        root,
        &manifest,
        extensions.as_ref(),
    ));
    if errors.is_empty() {
        let rev = manifest_revision(&manifest);
        let next = manifest
            .get("next_sprint")
            .and_then(Value::as_str)
            .unwrap_or("?");
        println!("vision drift check: ok (revision {rev}, next {next})");
        return ExitCode::SUCCESS;
    }

    eprintln!("vision drift check: {} issue(s)", errors.len());
    for err in &errors {
        eprintln!("  - {err}");
    }
    eprintln!("hint: cargo run --bin poolai-vision-sync");
    ExitCode::from(1)
}

fn main() -> ExitCode {
    let dry_run = std::env::args().any(|a| a == "--dry-run");
    let check_only = std::env::args().any(|a| a == "--check");
    let root = repo_root();

    if check_only {
        return run_drift_check(&root);
    }

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
    let (entries, fm_section) = read_fm_sprint_bundle(&root);
    let queue_changed = sync_fm_sprint_queue(&mut manifest, &entries, Some(&fm_section));
    let feed_changed = sync_sprint_feed(&root, &entries, Some(&fm_section));
    let (next_sprint, _, _) = derive_sprint_meta(&entries, Some(&fm_section));
    let ext_changed = sync_extensions_active_sprint(&root, next_sprint.as_deref());
    if (queue_changed || feed_changed || ext_changed) && added_nodes == 0 && added_edges == 0 {
        bump_manifest_revision(&mut manifest);
    }
    println!(
        "vision sync: +{added_nodes} nodes, +{added_edges} edges, sprint_queue {}, feed {}, extensions {} (revision {})",
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
        if ext_changed {
            "updated"
        } else {
            "unchanged"
        },
        manifest
            .get("revision")
            .and_then(Value::as_u64)
            .unwrap_or(0)
    );

    if added_nodes == 0 && added_edges == 0 && !queue_changed && !feed_changed && !ext_changed {
        return ExitCode::SUCCESS;
    }

    if dry_run {
        println!("dry-run: manifest/feed not written");
        return ExitCode::SUCCESS;
    }

    if queue_changed || added_nodes > 0 || added_edges > 0 || feed_changed {
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
        let (next, last, open) = derive_sprint_meta(&entries, None);
        assert_eq!(next.as_deref(), Some("PH-S191"));
        assert_eq!(last.as_deref(), Some("PH-S190"));
        assert_eq!(open, 2);
    }

    #[test]
    fn derive_sprint_meta_uses_highest_serial_not_file_order() {
        let entries = vec![
            SprintQueueEntry {
                row: 1,
                id: "PH-S1118".into(),
                title: "Band close".into(),
                deps: String::new(),
                acceptance: "close".into(),
                status: "closed".into(),
                open: false,
            },
            SprintQueueEntry {
                row: 2,
                id: "PH-S1048".into(),
                title: "Older close".into(),
                deps: String::new(),
                acceptance: "close".into(),
                status: "closed".into(),
                open: false,
            },
        ];
        let section =
            "**Відкритих у §5.12:** **0** (band 47 ✅). **Master horizon:** PH-S1119…S1128 (band 48).";
        let (next, last, open) = derive_sprint_meta(&entries, Some(section));
        assert_eq!(last.as_deref(), Some("PH-S1118"));
        assert_eq!(next.as_deref(), Some("PH-S1119"));
        assert_eq!(open, 0);
    }

    #[test]
    fn parse_master_horizon_next_from_fm_footer() {
        let section = "**Відкритих у §5.12:** **0**. **Master horizon:** PH-S1119…S1128 (band 48). Vision rev **319**.";
        assert_eq!(
            parse_master_horizon_next(section).as_deref(),
            Some("PH-S1119")
        );
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
        let feed = build_sprint_feed(&entries, None);
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

    #[test]
    fn parse_fm_vision_revision_footer() {
        let section = "**Відкритих у §5.12:** **5** (PH-S205…S209). Vision rev **146**.\n";
        assert_eq!(parse_fm_vision_revision(section), Some(146));
    }

    #[test]
    fn drift_check_ok_when_manifest_matches_fm() {
        let sample = r###"
### 5.12 Research backlog

| 139 | **PH-S204** | Edge click | PH-S199 | edge trace | **✅** |
| 140 | **PH-S205** | Drift gate | PH-S191 | CI check | відкрито |

**Відкритих у §5.12:** **1** (PH-S205). Vision rev **42**.
"###;
        let entries = parse_fm_sprint_queue(sample);
        let queue = sprint_queue_json(&entries);
        let manifest = json!({
            "revision": 42,
            "next_sprint": "PH-S205",
            "last_sprint_closed": "PH-S204",
            "sprint_queue_open_count": 1,
            "sprint_queue": queue
        });
        let ext = json!({ "active_sprint": "PH-S205" });
        let section = extract_fm_section_512(sample).unwrap();
        let errors = collect_manifest_fm_drift(&manifest, &entries, section, Some(&ext));
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn drift_check_fails_on_revision_mismatch() {
        let sample = r###"
### 5.12 Research backlog

| 140 | **PH-S205** | Drift gate | PH-S191 | CI check | відкрито |

**Відкритих у §5.12:** **1** (PH-S205). Vision rev **99**.
"###;
        let entries = parse_fm_sprint_queue(sample);
        let queue = sprint_queue_json(&entries);
        let manifest = json!({
            "revision": 42,
            "next_sprint": "PH-S205",
            "last_sprint_closed": null,
            "sprint_queue_open_count": 1,
            "sprint_queue": queue
        });
        let section = extract_fm_section_512(sample).unwrap();
        let errors = collect_manifest_fm_drift(&manifest, &entries, section, None);
        assert!(errors.iter().any(|e| e.contains("Vision rev")));
    }

    #[test]
    fn mdc_drift_fails_when_vdt_rule_missing_from_manifest() {
        let root = repo_root();
        let manifest = json!({
            "nodes": [
                { "id": "fm", "path": "docs/catalog/FUNCTION_MANAGEMENT.md", "layer": "L2" }
            ]
        });
        let ext = json!({
            "extension_policy": {
                ".mdc": {
                    "vision_files": [DOCS_VISION_MDC]
                }
            }
        });
        let errors = collect_mdc_manifest_drift(&root, &manifest, Some(&ext));
        assert!(
            errors
                .iter()
                .any(|e| e.contains("manifest missing node for VDT rule")),
            "{errors:?}"
        );
    }

    #[test]
    fn mdc_drift_ok_when_vdt_rules_indexed() {
        let root = repo_root();
        let mdc_files = tracked_vdt_mdc_files(&root).expect("git ls-files");
        assert!(!mdc_files.is_empty(), "expected tracked .mdc rules");
        let mut nodes: Vec<Value> = mdc_files
            .iter()
            .map(|p| {
                json!({
                    "id": path_slug(p),
                    "path": p,
                    "layer": "L1"
                })
            })
            .collect();
        for canon in DOCS_VISION_CANON_PATHS {
            nodes.push(json!({
                "id": path_slug(canon),
                "path": canon,
                "layer": if canon.starts_with("docs/vision/") { "L1" } else { "L2" }
            }));
        }
        let manifest = json!({ "nodes": nodes });
        let ext = json!({
            "extension_policy": {
                ".mdc": {
                    "vision_files": [DOCS_VISION_MDC]
                }
            }
        });
        let errors = collect_mdc_manifest_drift(&root, &manifest, Some(&ext));
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn vision_js_map_sprint_chips_aria_label_ph_s233() {
        let js = std::fs::read_to_string("docs/vision/vision.js").expect("vision.js");
        assert!(
            js.contains("function bindMapLinkedSprintChip"),
            "missing bindMapLinkedSprintChip"
        );
        assert!(
            js.contains("sprintMapFocusAriaLabel"),
            "missing sprintMapFocusAriaLabel"
        );
        assert!(
            js.contains("Focus sprint ") && js.contains(" on documentation map"),
            "missing map sprint chip aria-label text"
        );
        assert!(
            js.contains("renderSprintChips") && js.contains("bindMapLinkedSprintChip(span, s)"),
            "renderSprintChips should bind map-linked chips"
        );
    }

    #[test]
    fn vision_js_map_orbit_3d_ph_s555() {
        let js = std::fs::read_to_string("docs/vision/vision.js").expect("vision.js");
        assert!(
            js.contains("function initMapOrbitControls"),
            "missing initMapOrbitControls"
        );
        assert!(
            js.contains("function applyMapOrbitTransform"),
            "missing applyMapOrbitTransform"
        );
        assert!(
            js.contains("map-orbit-pad") && js.contains("MAP_ORBIT_DEFAULT"),
            "missing orbit pad + defaults"
        );
        let html = std::fs::read_to_string("docs/vision/index.html").expect("index.html");
        assert!(
            html.contains("map-scene-3d") && html.contains("map-orbit-pad"),
            "index.html missing 3D scene / orbit pad"
        );
    }

    #[test]
    fn vision_js_map_layer_z_projection_ph_s556() {
        let js = std::fs::read_to_string("docs/vision/vision.js").expect("vision.js");
        assert!(
            js.contains("function applyMap3DProjection"),
            "missing applyMap3DProjection"
        );
        assert!(
            js.contains("function rotateProject3D") && js.contains("MAP_LAYER_Z_STEP"),
            "missing layer Z projection"
        );
        let css = std::fs::read_to_string("docs/vision/vision.css").expect("vision.css");
        assert!(
            css.contains(".map-orbit-pad") && css.contains("bottom:"),
            "orbit pad should anchor above bottom bar"
        );
    }

    #[test]
    fn vision_js_gravity_solar_layout_ph_s557() {
        let js = std::fs::read_to_string("docs/vision/vision.js").expect("vision.js");
        assert!(
            js.contains("function layoutOrphanStars")
                && js.contains("function nudgeSolarSystemsApart")
                && js.contains("function syncLayerStack3D"),
            "missing gravity solar layout + stack sync"
        );
        assert!(
            js.contains("MAP_ORBIT_STEP_DEG = 2") && js.contains("MAP_ORBIT_PAD_SENS = 0.08"),
            "orbit should be 2x slower"
        );
    }
}
