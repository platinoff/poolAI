//! Merge git-tracked repo files into `GSV/docs/vision/manifest.json` (incremental map growth).
//!
//! ```text
//! cargo run --bin poolai-vision-sync
//! cargo run --bin poolai-vision-sync -- --dry-run
//! cargo run --bin poolai-vision-sync -- --check
//! ```
//!
//! `--check` validates FM ↔ manifest ↔ extensions ↔ VDT `.mdc` cross-links **and**
//! README / INDEX / development README / NEXT_SESSION / `vision.svg` canon fields.

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const MANIFEST_REL: &str = "GSV/docs/vision/manifest.json";
const FEED_REL: &str = "GSV/docs/vision/feed.json";
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const EXTENSIONS_REL: &str = "GSV/docs/vision/extensions.json";
const FEED_CLOSED_CAP: usize = 12;
/// Closed `PH-S*` with serial ≤ this are omitted from `manifest.sprint_queue` (vision UI prune).
/// Meta (`last_sprint_closed`, feed) still derives from the full FM parse.
const SPRINT_QUEUE_CLOSED_PRUNE_MAX: u32 = 2000;

const DOCS_VISION_MDC: &str = ".cursor/rules/docs-vision.mdc";

const DOCS_VISION_CANON_PATHS: &[&str] = &[
    "GSV/docs/vision/extensions.json",
    "GSV/docs/vision/vision.svg",
    "GSV/docs/vision/index.html",
    "GSV/docs/vision/README.md",
];

// KIVI: 3-box status for any IDE/provider/model (populated as JSON literal in sync)

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

/// List working-tree files: git-tracked + untracked non-ignored.
///
/// Vision-close sync runs before `git add`, so drain-created files are still
/// untracked; indexing only tracked files defers the manifest bump to the
/// pre-push hook (blocking push + FM drift). Including untracked non-ignored
/// files makes the sync see them at vision-close time instead.
fn git_worktree_files(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
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

/// Git-tracked files only (used for VDT `.mdc` rule drift, which must index
/// committed rules exclusively).
fn git_ls_files_tracked_only(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "--cached"])
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
    let vision_artifact = p.starts_with("GSV/docs/vision/")
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
        || p.starts_with("GSV/docs/")
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
        || p.starts_with("GSV/docs/")
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
        p if p.starts_with("GSV/docs/vision/") && p != "GSV/docs/vision/manifest.json" => {
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

fn sync_manifest(manifest: &mut Value, paths: &[String]) -> (usize, usize, usize, usize) {
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

    let indexable: BTreeSet<String> = paths
        .iter()
        .filter(|p| should_index(p))
        .map(|p| normalize_path(p))
        .collect();

    let mut pruned_nodes = 0usize;
    let mut pruned_edges = 0usize;
    let mut removed_ids: BTreeSet<String> = BTreeSet::new();
    if let Some(nodes) = manifest.get_mut("nodes").and_then(Value::as_array_mut) {
        let kept: Vec<Value> = nodes
            .iter()
            .filter(|node| {
                let auto_synced = node
                    .get("auto_synced")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                if !auto_synced {
                    return true;
                }
                let Some(p) = node.get("path").and_then(Value::as_str) else {
                    return false;
                };
                if indexable.contains(&normalize_path(p)) {
                    return true;
                }
                if let Some(id) = node.get("id").and_then(Value::as_str) {
                    removed_ids.insert(id.to_string());
                }
                pruned_nodes += 1;
                false
            })
            .cloned()
            .collect();
        *nodes = kept;
    }
    if !removed_ids.is_empty() {
        if let Some(edges) = manifest.get_mut("edges").and_then(Value::as_array_mut) {
            let kept: Vec<Value> = edges
                .iter()
                .filter(|edge| {
                    let from = edge.get("from").and_then(Value::as_str).unwrap_or("");
                    let to = edge.get("to").and_then(Value::as_str).unwrap_or("");
                    if removed_ids.contains(from) || removed_ids.contains(to) {
                        pruned_edges += 1;
                        return false;
                    }
                    true
                })
                .cloned()
                .collect();
            *edges = kept;
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

    if added_nodes > 0 || added_edges > 0 || pruned_nodes > 0 {
        let rev = manifest
            .get("revision")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            + 1;
        manifest["revision"] = json!(rev);
        manifest["auto_sync_at"] = json!(today_iso());
    }

    (added_nodes, added_edges, pruned_nodes, pruned_edges)
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
fn closed_entries_by_serial_desc(entries: &[SprintQueueEntry]) -> Vec<&SprintQueueEntry> {
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
    if plain.contains("[x]") || plain.contains("[✓]") {
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

/// All `### …` markdown sections in FM (H3), for enterprise band queues outside §5.12.
fn iter_fm_h3_sections(content: &str) -> Vec<&str> {
    let mut starts: Vec<usize> = Vec::new();
    let mut search = 0usize;
    while let Some(rel) = content[search..].find("\n### ") {
        let abs = search + rel + 1; // point at '#'
        starts.push(abs);
        search = abs + 4;
    }
    if content.starts_with("### ") {
        starts.insert(0, 0);
    }
    let mut out = Vec::with_capacity(starts.len());
    for (i, &start) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(content.len());
        out.push(&content[start..end]);
    }
    out
}

/// Enterprise / horizon band tables live in `### 5.NN … queue — band N` (after §5.12 journal).
fn parse_fm_band_queue_sections(content: &str) -> Vec<SprintQueueEntry> {
    let mut out = Vec::new();
    for section in iter_fm_h3_sections(content) {
        let header = section.lines().next().unwrap_or("");
        if header.starts_with("### 5.12") {
            continue;
        }
        let header_l = header.to_ascii_lowercase();
        if !(header_l.contains("queue") && header_l.contains("band")) {
            continue;
        }
        out.extend(parse_fm_sprint_queue_section(section));
    }
    out
}

fn merge_sprint_queue_entries(
    primary: Vec<SprintQueueEntry>,
    extra: Vec<SprintQueueEntry>,
) -> Vec<SprintQueueEntry> {
    let mut by_id: BTreeMap<String, SprintQueueEntry> = BTreeMap::new();
    for e in primary.into_iter().chain(extra) {
        by_id.insert(e.id.clone(), e);
    }
    let mut out: Vec<SprintQueueEntry> = by_id.into_values().collect();
    out.sort_by_key(|e| (parse_sprint_serial(&e.id).unwrap_or(0), e.row));
    out
}

fn parse_fm_sprint_queue(content: &str) -> Vec<SprintQueueEntry> {
    let journal = extract_fm_section_512(content)
        .map(parse_fm_sprint_queue_section)
        .unwrap_or_default();
    let bands = parse_fm_band_queue_sections(content);
    merge_sprint_queue_entries(journal, bands)
}

/// Keep all open rows; drop closed `PH-S*` with serial ≤ [`SPRINT_QUEUE_CLOSED_PRUNE_MAX`].
/// Non-`PH-S*` ids (e.g. future service labels) stay.
fn filter_sprint_queue_for_manifest(entries: &[SprintQueueEntry]) -> Vec<SprintQueueEntry> {
    entries
        .iter()
        .filter(|e| {
            if e.open {
                return true;
            }
            match parse_sprint_serial(&e.id) {
                Some(n) => n > SPRINT_QUEUE_CLOSED_PRUNE_MAX,
                None => true,
            }
        })
        .cloned()
        .collect()
}

fn sprint_queue_json(entries: &[SprintQueueEntry]) -> Value {
    let filtered = filter_sprint_queue_for_manifest(entries);
    Value::Array(
        filtered
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
    let entries = parse_fm_sprint_queue(&content);
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

    // KIVI boxes: done (with IDE+provider+model+time sig), open (workflow), pending (queue)
    let kivi = json!([
        {
            "kind": "done",
            "count": 10,
            "signature": "opencode:xai.grok-4.3 @ 2026-08-02 20:15",
            "items": ["PH-S1699…S1708 band 106 Ratio96 loc-audit ✅"]
        },
        {
            "kind": "open",
            "count": 0,
            "signature": "workflow: AGENTS.md §100-113 drain",
            "items": []
        },
        {
            "kind": "pending",
            "count": 10,
            "signature": "queue: FM §5.12 next band 107",
            "items": ["PH-S1709…S1718 (Ratio96 horizon start)"]
        }
    ]);
    manifest["kivi"] = kivi;
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
        "link": "http://127.0.0.1:8891/#b-sprint-board"
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
        "link": "http://127.0.0.1:8891/",
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
    Ok(git_ls_files_tracked_only(root)?
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

const VISION_CANON_DOC_PATHS: &[&str] = &[
    "README.md",
    "docs/INDEX_2026-03-17.md",
    "docs/development/README.md",
    "docs/development/NEXT_SESSION_PROMPT.md",
    "GSV/docs/vision/vision.svg",
];

const RUST_RATIO_REL: &str = "docs/development/rust_ratio.json";

#[derive(Debug, Clone, PartialEq, Eq)]
struct VisionCanonSnapshot {
    revision: u64,
    closed_band: u32,
    closed_range: String,
    closed_band_title: String,
    next_band: u32,
    next_range: String,
    next_sprint: String,
    last_closed: String,
    open_count: u32,
    rust_ratio_pct: String,
}

fn sprint_serial_suffix(id: &str) -> &str {
    id.strip_prefix("PH-S").unwrap_or(id)
}

fn band_title_slug(raw: &str) -> String {
    let trimmed = raw.trim();
    let without_galaxy = trimmed
        .strip_prefix("Galaxy ")
        .or_else(|| trimmed.strip_prefix("galaxy "))
        .unwrap_or(trimmed);
    without_galaxy.to_ascii_lowercase()
}

fn parse_fm_closed_band(section: &str) -> Option<u32> {
    for line in section.lines() {
        if !line.contains("Відкритих у §5.12") {
            continue;
        }
        let marker = "(band ";
        let idx = line.find(marker)?;
        let rest = &line[idx + marker.len()..];
        let end = rest.find(|c: char| !c.is_ascii_digit())?;
        return rest[..end].parse().ok();
    }
    None
}

fn parse_fm_master_horizon(section: &str) -> Option<(String, u32)> {
    let marker = "**Master horizon:**";
    for line in section.lines() {
        let Some(after) = line.split(marker).nth(1) else {
            continue;
        };
        let trimmed = after.trim();
        let ph_idx = trimmed.find("PH-S")?;
        let range_end = trimmed[ph_idx..]
            .find('(')
            .map(|i| ph_idx + i)
            .unwrap_or(trimmed.len());
        let range = trimmed[ph_idx..range_end]
            .trim()
            .trim_end_matches('.')
            .to_string();
        let band_marker = "(band ";
        let band_idx = trimmed.find(band_marker)?;
        let band_rest = &trimmed[band_idx + band_marker.len()..];
        let band_end = band_rest.find(')')?;
        let band: u32 = band_rest[..band_end].parse().ok()?;
        return Some((range, band));
    }
    None
}

fn parse_fm_closed_range(fm_content: &str, closed_band: u32) -> Option<String> {
    for pattern in [
        format!("band {closed_band} **PH-S"),
        format!("band {closed_band} (PH-S"),
    ] {
        let Some(idx) = fm_content.find(&pattern) else {
            continue;
        };
        let start = idx + pattern.len() - 4;
        let rest = &fm_content[start..];
        // Prefer the earlier of `)` / `**` so titles like
        // `(PH-S1459…S1468) · **✅**` yield `PH-S1459…S1468`, not `…S1468) ·`.
        let close_marker = match (rest.find("**"), rest.find(')')) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => continue,
        };
        let range = rest[..close_marker].trim();
        if range.contains('…') {
            return Some(range.to_string());
        }
    }
    None
}

fn parse_fm_band_title(fm_content: &str, closed_band: u32) -> Option<String> {
    let marker = format!(" queue — band {closed_band}");
    let idx = fm_content.find(&marker)?;
    let before = &fm_content[..idx];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let header = before[line_start..].trim();
    let after_hash = header.strip_prefix("### ")?;
    let space_idx = after_hash.find(' ')?;
    Some(after_hash[space_idx + 1..].trim().to_string())
}

fn read_rust_ratio_pct(root: &Path) -> Result<String, String> {
    let path = root.join(RUST_RATIO_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let pct = value
        .get("rust_ratio_pct")
        .and_then(Value::as_f64)
        .or_else(|| {
            value
                .get("rust_ratio")
                .and_then(Value::as_f64)
                .map(|r| r * 100.0)
        })
        .ok_or_else(|| format!("{} missing rust_ratio_pct", path.display()))?;
    Ok(format!("{pct:.2}"))
}

fn closed_range_end(range: &str) -> Option<String> {
    let last_s = range.rfind('S')?;
    let serial: String = range[last_s + 1..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if serial.is_empty() {
        return None;
    }
    Some(format!("PH-S{serial}"))
}

fn build_vision_canon_snapshot(
    manifest: &Value,
    entries: &[SprintQueueEntry],
    fm_section: &str,
    fm_content: &str,
    root: &Path,
) -> Result<VisionCanonSnapshot, String> {
    let revision =
        parse_fm_vision_revision(fm_section).unwrap_or_else(|| manifest_revision(manifest));
    let (next_sprint, last_closed_meta, open_count) = derive_sprint_meta(entries, Some(fm_section));
    let next_sprint = next_sprint.ok_or("canon snapshot: missing next_sprint")?;
    let closed_band =
        parse_fm_closed_band(fm_section).ok_or("canon snapshot: FM §5.12 missing closed band")?;
    let (next_range, next_band) = parse_fm_master_horizon(fm_section)
        .ok_or("canon snapshot: FM §5.12 missing Master horizon range")?;
    let closed_range = parse_fm_closed_range(fm_content, closed_band)
        .ok_or("canon snapshot: FM missing closed band sprint range")?;
    let last_closed = if open_count == 0 {
        closed_range_end(&closed_range)
            .or(last_closed_meta)
            .ok_or("canon snapshot: missing last_closed")?
    } else {
        last_closed_meta.ok_or("canon snapshot: missing last_closed")?
    };
    let closed_title_raw = parse_fm_band_title(fm_content, closed_band)
        .ok_or("canon snapshot: FM missing band section title")?;
    let closed_band_title = band_title_slug(&closed_title_raw);
    let rust_ratio_pct = read_rust_ratio_pct(root)?;
    Ok(VisionCanonSnapshot {
        revision,
        closed_band,
        closed_range,
        closed_band_title,
        next_band,
        next_range,
        next_sprint,
        last_closed,
        open_count,
        rust_ratio_pct,
    })
}

fn replace_between_markers(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    replacement: &str,
) -> String {
    let mut out = content.to_string();
    let mut search_from = 0usize;
    while let Some(rel) = out[search_from..].find(start_marker) {
        let start = search_from + rel + start_marker.len();
        let Some(rel_end) = out[start..].find(end_marker) else {
            break;
        };
        out.replace_range(start..start + rel_end, replacement);
        search_from = start + replacement.len() + end_marker.len();
    }
    out
}

fn replace_number_after_marker(content: &str, marker: &str, new_value: u32) -> String {
    let Some(idx) = content.find(marker) else {
        return content.to_string();
    };
    let num_start = idx + marker.len();
    let rest = &content[num_start..];
    let Some(digit_rel) = rest.find(|c: char| c.is_ascii_digit()) else {
        return content.to_string();
    };
    let digit_start = num_start + digit_rel;
    let digit_end = digit_start
        + content[digit_start..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(content[digit_start..].len());
    let mut out = content.to_string();
    out.replace_range(digit_start..digit_end, &new_value.to_string());
    out
}

fn replace_galaxy_wire_end(content: &str, last_closed: &str) -> String {
    let marker = "PH-S65…S";
    let Some(idx) = content.find(marker) else {
        return content.to_string();
    };
    let serial_start = idx + marker.len();
    let serial_end = serial_start
        + content[serial_start..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(content[serial_start..].len());
    let mut out = content.to_string();
    out.replace_range(serial_start..serial_end, sprint_serial_suffix(last_closed));
    out
}

fn replace_closed_band_badges(content: &str, closed_band: u32) -> String {
    let mut out = content.to_string();
    let marker = "(band ";
    let mut search_from = 0usize;
    while let Some(rel) = out[search_from..].find(marker) {
        let idx = search_from + rel;
        let after = idx + marker.len();
        let Some(space_rel) = out[after..].find(' ') else {
            break;
        };
        if !out[after + space_rel..].starts_with("✅)") {
            search_from = after + 1;
            continue;
        }
        let digit_end = after + space_rel;
        if out[after..digit_end].chars().all(|c| c.is_ascii_digit()) {
            out.replace_range(after..digit_end, &closed_band.to_string());
        }
        search_from = digit_end + 1;
    }
    out
}

fn replace_ph_s_range_after(content: &str, prefix: &str, new_range: &str) -> String {
    let Some(idx) = content.find(prefix) else {
        return content.to_string();
    };
    let range_start = idx + prefix.len();
    let rest = &content[range_start..];
    let range_end = range_start
        + rest
            .find(|c: char| {
                !(c.is_ascii_digit() || c == '…' || c == 'S' || c == 'P' || c == 'H' || c == '-')
            })
            .unwrap_or(rest.len());
    let mut out = content.to_string();
    out.replace_range(range_start..range_end, new_range);
    out
}

fn sync_readme_canon(content: &str, snap: &VisionCanonSnapshot) -> String {
    let mut out = replace_galaxy_wire_end(content, &snap.last_closed);
    out = replace_between_markers(&out, "manifest rev **", "**", &snap.revision.to_string());
    out = replace_between_markers(&out, "vision **rev ", "**", &snap.revision.to_string());
    out = replace_between_markers(&out, "**Rust ratio:** **", "%**", &snap.rust_ratio_pct);
    out = replace_closed_band_badges(&out, snap.closed_band);
    out = replace_number_after_marker(&out, "→ band ", snap.next_band);
    out = replace_between_markers(
        &out,
        "project scan → band ",
        " **",
        &snap.next_band.to_string(),
    );
    out = replace_ph_s_range_after(
        &out,
        &format!("band {} **", snap.next_band),
        &snap.next_range,
    );
    out = replace_between_markers(&out, "last **", "**", &snap.last_closed);
    out = replace_between_markers(&out, "next **", "**", &snap.next_sprint);
    out = replace_between_markers(
        &out,
        "**§5.12:** **",
        "** відкритих",
        &snap.open_count.to_string(),
    );
    out
}

fn sync_index_canon(content: &str, snap: &VisionCanonSnapshot) -> String {
    let zriz = format!(
        "**Зріз:** {} ✅ (band {} {}) · **§5.12:** **{}** (maintenance mode) · rust_ratio **{}%** · vision **rev {}** · **HEAD:** maintenance mode — [`NEXT_SESSION_PROMPT.md`](./development/NEXT_SESSION_PROMPT.md)",
        snap.last_closed,
        snap.closed_band,
        snap.closed_band_title,
        snap.open_count,
        snap.rust_ratio_pct,
        snap.revision
    );
    replace_line_with_prefix(content, "**Зріз:**", &zriz)
}

fn sync_development_readme_canon(content: &str, snap: &VisionCanonSnapshot) -> String {
    replace_between_markers(content, "vision rev **", "**", &snap.revision.to_string())
}

fn normalize_newlines(content: &str) -> String {
    content.replace("\r\n", "\n")
}

fn canon_content_eq(left: &str, right: &str) -> bool {
    normalize_newlines(left) == normalize_newlines(right)
}

fn replace_line_with_prefix(content: &str, prefix: &str, new_line: &str) -> String {
    let trailing_newline = content.ends_with('\n');
    let mut lines: Vec<&str> = content.lines().collect();
    for line in &mut lines {
        if line.starts_with(prefix) {
            *line = new_line;
        }
    }
    let mut out = lines.join("\n");
    if trailing_newline {
        out.push('\n');
    }
    out
}

fn parse_fm_updated_date(fm_content: &str) -> Option<String> {
    let line = fm_content
        .lines()
        .find(|l| l.starts_with("**Оновлено:**"))?;
    let rest = line.strip_prefix("**Оновлено:**")?.trim();
    let end = rest.find(' ')?;
    Some(rest[..end].to_string())
}

fn sync_next_session_canon(content: &str, snap: &VisionCanonSnapshot, fm_content: &str) -> String {
    let updated_on = parse_fm_updated_date(fm_content).unwrap_or_else(today_iso);
    let header = format!(
        "**Оновлено:** {updated_on} (band {} **{}** ✅ · horizon band {})",
        snap.closed_band, snap.closed_range, snap.next_band
    );
    let mut out = replace_line_with_prefix(content, "**Оновлено:**", &header);
    out = replace_line_with_prefix(
        &out,
        "Maintenance mode",
        &format!(
            "Maintenance mode (FM §5.15) · band {} drained.",
            snap.closed_band
        ),
    );
    out = replace_line_with_prefix(
        &out,
        "| **← наступний** |",
        &format!(
            "| **← наступний** | **`абракадабра`** (project scan → band {}) |",
            snap.next_band
        ),
    );
    out = replace_line_with_prefix(
        &out,
        "| **§5.12 active** |",
        &format!(
            "| **§5.12 active** | **{}** (band {} ✅) |",
            snap.open_count, snap.closed_band
        ),
    );
    out = replace_line_with_prefix(
        &out,
        "| **Horizon** |",
        &format!(
            "| **Horizon** | band {} → **{}** |",
            snap.next_band, snap.next_range
        ),
    );
    out = replace_line_with_prefix(
        &out,
        "| **Vision** |",
        &format!("| **Vision** | rev **{}** |", snap.revision),
    );
    out = replace_line_with_prefix(
        &out,
        "## Band ",
        &format!(
            "## Band {} (очікуваний фокус — project scan)",
            snap.next_band
        ),
    );
    out = replace_line_with_prefix(
        &out,
        "| PH-S",
        &format!("| {} | horizon close + maintenance ops |", snap.next_range),
    );
    out
}

fn sync_vision_svg_canon(content: &str, snap: &VisionCanonSnapshot) -> String {
    let mut out = content.to_string();
    let desc = format!(
        "Isometric L0-L3 layers with Galaxy Grid at center. {} done, next {}.",
        snap.last_closed, snap.next_sprint
    );
    out = replace_between_markers(&out, "<desc id=\"desc\">", "</desc>", &desc);
    let footer = format!(
        "{} done {} - {} next - {} open - rev {}",
        snap.last_closed, snap.closed_band_title, snap.next_sprint, snap.open_count, snap.revision
    );
    out = replace_line_with_prefix(
        &out,
        "  <text x=\"600\" y=\"738\"",
        &format!(
            "  <text x=\"600\" y=\"738\" text-anchor=\"middle\" fill=\"#6a7a9a\" font-family=\"Segoe UI, system-ui, sans-serif\" font-size=\"12\">{footer}</text>"
        ),
    );
    out
}

fn apply_canon_snapshot(
    rel: &str,
    content: &str,
    snap: &VisionCanonSnapshot,
    fm_content: &str,
) -> String {
    match rel {
        "README.md" => sync_readme_canon(content, snap),
        "docs/INDEX_2026-03-17.md" => sync_index_canon(content, snap),
        "docs/development/README.md" => sync_development_readme_canon(content, snap),
        "docs/development/NEXT_SESSION_PROMPT.md" => {
            sync_next_session_canon(content, snap, fm_content)
        }
        "GSV/docs/vision/vision.svg" => sync_vision_svg_canon(content, snap),
        _ => content.to_string(),
    }
}

fn collect_canon_docs_drift(
    root: &Path,
    manifest: &Value,
    entries: &[SprintQueueEntry],
    fm_section: &str,
    fm_content: &str,
) -> Vec<String> {
    let snap = match build_vision_canon_snapshot(manifest, entries, fm_section, fm_content, root) {
        Ok(s) => s,
        Err(e) => return vec![e],
    };
    let mut errors = Vec::new();
    for rel in VISION_CANON_DOC_PATHS {
        let path = root.join(rel);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                errors.push(format!("read {rel}: {e}"));
                continue;
            }
        };
        let synced = apply_canon_snapshot(rel, &content, &snap, fm_content);
        if !canon_content_eq(&synced, &content) {
            errors.push(format!(
                "{rel}: stale vision canon fields (revision {}, band {}, next {})",
                snap.revision, snap.closed_band, snap.next_sprint
            ));
        }
    }
    errors
}

fn sync_vision_canon_docs(
    root: &Path,
    manifest: &Value,
    entries: &[SprintQueueEntry],
    fm_section: &str,
    fm_content: &str,
) -> Result<Vec<String>, String> {
    let snap = build_vision_canon_snapshot(manifest, entries, fm_section, fm_content, root)?;
    let mut changed = Vec::new();
    for rel in VISION_CANON_DOC_PATHS {
        let path = root.join(rel);
        let content = fs::read_to_string(&path).map_err(|e| format!("read {rel}: {e}"))?;
        let updated = apply_canon_snapshot(rel, &content, &snap, fm_content);
        if !canon_content_eq(&updated, &content) {
            fs::write(&path, &updated).map_err(|e| format!("write {rel}: {e}"))?;
            changed.push(rel.to_string());
        }
    }
    Ok(changed)
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
    errors.extend(collect_canon_docs_drift(
        root,
        &manifest,
        &entries,
        fm_section,
        &fm_content,
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

    let paths = match git_worktree_files(&root) {
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

    let (added_nodes, added_edges, pruned_nodes, pruned_edges) =
        sync_manifest(&mut manifest, &paths);
    let (entries, fm_section) = read_fm_sprint_bundle(&root);
    let queue_changed = sync_fm_sprint_queue(&mut manifest, &entries, Some(&fm_section));
    let feed_changed = sync_sprint_feed(&root, &entries, Some(&fm_section));
    let (next_sprint, _, _) = derive_sprint_meta(&entries, Some(&fm_section));
    let ext_changed = sync_extensions_active_sprint(&root, next_sprint.as_deref());
    if (queue_changed || feed_changed || ext_changed)
        && added_nodes == 0
        && added_edges == 0
        && pruned_nodes == 0
    {
        bump_manifest_revision(&mut manifest);
    }
    println!(
        "vision sync: +{added_nodes} nodes, +{added_edges} edges, -{pruned_nodes} nodes, -{pruned_edges} edges, sprint_queue {}, feed {}, extensions {} (revision {})",
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

    if dry_run {
        println!("dry-run: manifest/feed/canon docs not written");
        return ExitCode::SUCCESS;
    }

    if queue_changed || added_nodes > 0 || added_edges > 0 || pruned_nodes > 0 || feed_changed {
        if let Err(e) = write_manifest(&manifest_path, &manifest) {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    }

    let fm_path = root.join(FM_REL);
    let fm_content = fs::read_to_string(&fm_path).unwrap_or_default();
    match sync_vision_canon_docs(&root, &manifest, &entries, &fm_section, &fm_content) {
        Ok(changed) if !changed.is_empty() => {
            println!("vision canon docs: updated {}", changed.join(", "));
        }
        Ok(_) => {}
        Err(e) => eprintln!("warn: vision canon docs sync: {e}"),
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
    fn sprint_queue_json_prunes_closed_le_2000() {
        let entries = vec![
            SprintQueueEntry {
                row: 1,
                id: "PH-S1398".into(),
                title: "closed low".into(),
                deps: String::new(),
                acceptance: String::new(),
                status: "closed".into(),
                open: false,
            },
            SprintQueueEntry {
                row: 2,
                id: "PH-S1399".into(),
                title: "open".into(),
                deps: String::new(),
                acceptance: String::new(),
                status: "open".into(),
                open: true,
            },
            SprintQueueEntry {
                row: 3,
                id: "PH-S2001".into(),
                title: "closed high".into(),
                deps: String::new(),
                acceptance: String::new(),
                status: "closed".into(),
                open: false,
            },
        ];
        let q = sprint_queue_json(&entries);
        let ids: Vec<&str> = q
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap())
            .collect();
        assert_eq!(ids, vec!["PH-S1399", "PH-S2001"]);
        // Meta still sees pruned closed via full entries.
        let (_, last, open) = derive_sprint_meta(&entries, None);
        assert_eq!(open, 1);
        assert_eq!(last.as_deref(), Some("PH-S2001"));
    }

    #[test]
    fn parse_fm_sprint_queue_merges_enterprise_band_sections() {
        let sample = r###"
### 5.12 Research backlog PH-S65+ (Galaxy wire / ops, 2026-05-27)

| 953 | **PH-S1018** | Ops power band close | tests | RUN_LOCAL sync | **✅** |

**Відкритих у §5.12:** **0** (band 61 ✅). **Master horizon:** PH-S1259…S1268 (band 62).

### 5.42 SSO depth scaffold queue — band 61 (PH-S1249…S1258, 2026-07-21)

| 1192 | **PH-S1257** | Ratio hold advisory | RUST_RATIO | advisory | **✅** |
| 1193 | **PH-S1258** | SSO depth band close | Enterprise band 61 | HANDOFF → band 62 | **✅** |

### 5.16 Service band (Cursor / toolchain / docs hygiene)
"###;
        let entries = parse_fm_sprint_queue(sample);
        assert_eq!(entries.len(), 3);
        let (next, last, open) = derive_sprint_meta(&entries, extract_fm_section_512(sample));
        assert_eq!(open, 0);
        assert_eq!(last.as_deref(), Some("PH-S1258"));
        assert_eq!(next.as_deref(), Some("PH-S1259"));
        let feed = build_sprint_feed(&entries, extract_fm_section_512(sample));
        let ids: Vec<&str> = feed["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|i| i.get("id").and_then(Value::as_str))
            .collect();
        assert_eq!(ids.first().copied(), Some("PH-S1258"));
        assert!(ids.contains(&"PH-S1257"));
        assert!(ids.contains(&"PH-S1018"));
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
        let (n, e, pn, pe) = sync_manifest(&mut manifest, &paths);
        assert_eq!(n, 1);
        assert!(e >= 1);
        assert_eq!(pn, 0);
        assert_eq!(pe, 0);
        let nodes = manifest["nodes"].as_array().unwrap();
        assert!(nodes.iter().any(|n| n["path"] == "docs/openapi.yaml"));
    }

    /// Regression: nodes whose files were moved/deleted must be pruned
    /// (auto_synced only), together with their edges.
    #[test]
    fn sync_prunes_stale_auto_synced_nodes_and_edges() {
        let mut manifest = json!({
            "revision": 1,
            "nodes": [
                { "id": "fm", "label": "FM", "path": "docs/catalog/FUNCTION_MANAGEMENT.md", "layer": "L2" },
                { "id": "stale_vision_readme", "label": "README.md", "path": "docs/vision/README.md", "layer": "L1", "auto_synced": true },
                { "id": "live_doc", "label": "live.md", "path": "docs/development/live.md", "layer": "L2", "auto_synced": true }
            ],
            "edges": [
                { "from": "fm", "to": "stale_vision_readme", "kind": "catalog" },
                { "from": "fm", "to": "live_doc", "kind": "catalog" }
            ]
        });
        let paths = vec![
            "docs/catalog/FUNCTION_MANAGEMENT.md".to_string(),
            "docs/development/live.md".to_string(),
        ];
        let (n, e, pn, pe) = sync_manifest(&mut manifest, &paths);
        assert_eq!(n, 0);
        assert!(e >= 1);
        assert_eq!(pn, 1);
        assert_eq!(pe, 1);
        let nodes = manifest["nodes"].as_array().unwrap();
        assert!(!nodes.iter().any(|n| n["id"] == "stale_vision_readme"));
        assert!(nodes.iter().any(|n| n["id"] == "live_doc"));
        assert!(nodes.iter().any(|n| n["id"] == "fm"));
        let edges = manifest["edges"].as_array().unwrap();
        assert!(!edges.iter().any(|e| e["to"] == "stale_vision_readme"));
        assert!(edges.iter().any(|e| e["to"] == "live_doc"));
        assert!(manifest["revision"].as_u64().unwrap() > 1);
    }

    /// Regression: vision-close sync runs before `git add`, so drain-created
    /// files are untracked. `git_worktree_files` must include them, or the
    /// manifest bump is deferred to the pre-push hook (push blocked + FM drift).
    #[test]
    fn git_worktree_files_includes_untracked_non_ignored() {
        let dir = std::env::temp_dir().join(format!("poolai-vision-sync-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("git run");
            assert!(
                out.status.success(),
                "git {args:?}: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "test"]);
        std::fs::write(dir.join("tracked.rs"), "fn a() {}").unwrap();
        git(&["add", "tracked.rs"]);
        git(&["commit", "-qm", "init"]);
        std::fs::write(dir.join("untracked.rs"), "fn b() {}").unwrap();
        std::fs::write(dir.join("ignored.log"), "x").unwrap();
        std::fs::write(dir.join(".gitignore"), "*.log").unwrap();

        let files = git_worktree_files(&dir).expect("worktree files");
        assert!(
            files.contains(&"tracked.rs".to_string()),
            "tracked missing: {files:?}"
        );
        assert!(
            files.contains(&"untracked.rs".to_string()),
            "untracked missing: {files:?}"
        );
        assert!(
            !files.iter().any(|f| f.ends_with("ignored.log")),
            "gitignored file leaked into scan: {files:?}"
        );
        let tracked_only = git_ls_files_tracked_only(&dir).expect("tracked only");
        assert!(tracked_only.contains(&"tracked.rs".to_string()));
        assert!(!tracked_only.contains(&"untracked.rs".to_string()));
        let _ = std::fs::remove_dir_all(&dir);
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
                "layer": if canon.starts_with("GSV/docs/vision/") { "L1" } else { "L2" }
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
        let js = std::fs::read_to_string("GSV/docs/vision/vision.js").expect("vision.js");
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
        assert!(
            js.contains("DEACTIVATED — band 117"),
            "vision.js should carry the band-117 deactivation banner"
        );
    }

    #[test]
    fn vision_js_map_orbit_3d_ph_s555() {
        let js = std::fs::read_to_string("GSV/docs/vision/vision.js").expect("vision.js");
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
        let html = std::fs::read_to_string("GSV/docs/vision/index.html").expect("index.html");
        assert!(
            html.contains("superseded by GSV") && !html.contains("map-scene-3d"),
            "index.html should be a GSV pointer page (legacy 3D scene removed)"
        );
    }

    #[test]
    fn vision_js_map_layer_z_projection_ph_s556() {
        let js = std::fs::read_to_string("GSV/docs/vision/vision.js").expect("vision.js");
        assert!(
            js.contains("function applyMap3DProjection"),
            "missing applyMap3DProjection"
        );
        assert!(
            js.contains("function rotateProject3D") && js.contains("MAP_LAYER_Z_STEP"),
            "missing layer Z projection"
        );
        let css = std::fs::read_to_string("GSV/docs/vision/vision.css").expect("vision.css");
        assert!(
            css.contains(".map-orbit-pad") && css.contains("bottom:"),
            "orbit pad should anchor above bottom bar"
        );
        assert!(
            css.contains("DEACTIVATED — band 117"),
            "vision.css should carry the band-117 deactivation banner"
        );
    }

    #[test]
    fn vision_js_gravity_solar_layout_ph_s557() {
        let js = std::fs::read_to_string("GSV/docs/vision/vision.js").expect("vision.js");
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

    #[test]
    fn closed_range_end_parses_last_sprint() {
        assert_eq!(
            closed_range_end("PH-S1119…S1128").as_deref(),
            Some("PH-S1128")
        );
    }

    #[test]
    fn parse_fm_master_horizon_band_and_range() {
        let section = "**Відкритих у §5.12:** **0** (band 48 ✅). **Master horizon:** PH-S1129…S1138 (band 49). Vision rev **321**.";
        assert_eq!(parse_fm_closed_band(section), Some(48));
        assert_eq!(
            parse_fm_master_horizon(section),
            Some(("PH-S1129…S1138".to_string(), 49))
        );
    }

    #[test]
    fn canon_readme_sync_is_idempotent() {
        let root = repo_root();
        let fm_content = fs::read_to_string(root.join(FM_REL)).expect("FM");
        let fm_section = extract_fm_section_512(&fm_content).unwrap();
        let entries = parse_fm_sprint_queue(&fm_content);
        let manifest = load_manifest(&root.join(MANIFEST_REL)).expect("manifest");
        let snap = build_vision_canon_snapshot(&manifest, &entries, fm_section, &fm_content, &root)
            .expect("snapshot");
        let readme = fs::read_to_string(root.join("README.md")).expect("README");
        let once = sync_readme_canon(&readme, &snap);
        let twice = sync_readme_canon(&once, &snap);
        assert_eq!(once, twice, "README canon sync must be idempotent");
    }

    #[test]
    fn canon_docs_drift_empty_when_repo_aligned() {
        let root = repo_root();
        let fm_content = fs::read_to_string(root.join(FM_REL)).expect("FM");
        let fm_section = extract_fm_section_512(&fm_content).unwrap();
        let entries = parse_fm_sprint_queue(&fm_content);
        let manifest = load_manifest(&root.join(MANIFEST_REL)).expect("manifest");
        let errors = collect_canon_docs_drift(&root, &manifest, &entries, fm_section, &fm_content);
        assert!(errors.is_empty(), "{errors:?}");
    }
}
