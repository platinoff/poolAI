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
    let pretty = serde_json::to_string_pretty(manifest).map_err(|e| format!("serialize: {e}"))?;
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
    println!(
        "vision sync: +{added_nodes} nodes, +{added_edges} edges (revision {})",
        manifest
            .get("revision")
            .and_then(Value::as_u64)
            .unwrap_or(0)
    );

    if added_nodes == 0 && added_edges == 0 {
        return ExitCode::SUCCESS;
    }

    if dry_run {
        println!("dry-run: manifest not written");
        return ExitCode::SUCCESS;
    }

    if let Err(e) = write_manifest(&manifest_path, &manifest) {
        eprintln!("error: {e}");
        return ExitCode::from(2);
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
