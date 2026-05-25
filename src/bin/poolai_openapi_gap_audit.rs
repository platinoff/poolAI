//! Compare Axum `.route("…")` registrations under `src/network` with `docs/openapi.yaml` paths.
//!
//! ```text
//! cargo run --bin poolai-openapi-gap-audit
//! ```
//!
//! Exit code: `0` if every route (except ignores) has an OpenAPI path; `1` if any missing.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const NETWORK_DIR: &str = "src/network";
const OPENAPI_FILE: &str = "docs/openapi.yaml";

/// Routes registered under `Router::nest(prefix, …)` — OpenAPI paths include the prefix.
fn nest_prefix_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("/", "/ai-ml/"),
        ("/status", "/ai-ml/"),
        ("/optimization", "/ai-ml/"),
        ("/optimization/profile", "/ai-ml/"),
        ("/optimization/tuning", "/ai-ml/"),
        ("/optimization/quantization-result", "/ai-ml/"),
        ("/automl", "/ai-ml/"),
        ("/federated", "/ai-ml/"),
        ("/pipeline", "/ai-ml/"),
        ("/pipeline/demo", "/ai-ml/"),
        ("/pipeline/{id}", "/ai-ml/"),
        ("/pipeline/{id}/execute", "/ai-ml/"),
    ])
}

/// Exact paths excluded from the public OpenAPI contract.
const IGNORE_ROUTES_EXACT: &[&str] = &["/api/workers"];

/// Prefixes excluded (HTML UI, inter-node Raft wire — not customer REST).
const IGNORE_ROUTE_PREFIXES: &[&str] = &["/ui/", "/raft/"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn collect_routes(network_dir: &Path) -> BTreeSet<String> {
    let mut routes = BTreeSet::new();
    walk_rs(network_dir, &mut |path| {
        let Ok(text) = fs::read_to_string(path) else {
            return;
        };
        for route in extract_route_literals(&text) {
            routes.insert(route);
        }
    });
    routes
}

fn walk_rs(dir: &Path, file_fn: &mut dyn FnMut(&Path)) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rs(&path, file_fn);
        } else if path.extension().is_some_and(|e| e == "rs") {
            file_fn(&path);
        }
    }
}

/// Match `.route("…")` / `.route( "…" )` string literals (same as legacy audit script).
fn extract_route_literals(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = source;
    while let Some(idx) = rest.find(".route(") {
        let after_paren = &rest[idx + ".route(".len()..];
        let Some(route) = parse_route_string_literal(after_paren) else {
            rest = &after_paren[1..];
            continue;
        };
        out.push(route);
        rest = &after_paren[1..];
    }
    out
}

fn parse_route_string_literal(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'"' {
        return None;
    }
    i += 1;
    let start = i;
    while i < bytes.len() && bytes[i] != b'"' {
        i += 1;
    }
    if i >= bytes.len() {
        return None;
    }
    std::str::from_utf8(&bytes[start..i])
        .ok()
        .map(|s| s.to_string())
}

fn collect_openapi_paths(openapi: &Path) -> BTreeSet<String> {
    let Ok(text) = fs::read_to_string(openapi) else {
        return BTreeSet::new();
    };
    let mut paths = BTreeSet::new();
    for line in text.lines() {
        if let Some(path) = parse_openapi_path_line(line) {
            paths.insert(path);
        }
    }
    paths
}

/// Lines like `  /api/v1/health:` in `openapi.yaml`.
fn parse_openapi_path_line(line: &str) -> Option<String> {
    let trimmed = line.strip_prefix("  ")?;
    if !trimmed.starts_with('/') {
        return None;
    }
    let path = trimmed.split(':').next()?;
    if path.is_empty() || path.contains(' ') {
        return None;
    }
    Some(path.to_string())
}

fn openapi_path_for_route(route: &str, nest: &HashMap<&str, &str>) -> String {
    if let Some(prefix) = nest.get(route) {
        return format!("{}{}", prefix, route.trim_start_matches('/'));
    }
    route.to_string()
}

fn is_ignored(route: &str) -> bool {
    IGNORE_ROUTES_EXACT.iter().any(|ig| route == *ig)
        || IGNORE_ROUTE_PREFIXES
            .iter()
            .any(|pfx| route.starts_with(pfx))
}

fn main() -> ExitCode {
    let root = repo_root();
    let network = root.join(NETWORK_DIR);
    let openapi = root.join(OPENAPI_FILE);

    if !network.is_dir() {
        eprintln!("error: missing {NETWORK_DIR} under {}", root.display());
        return ExitCode::from(2);
    }
    if !openapi.is_file() {
        eprintln!("error: missing {OPENAPI_FILE} under {}", root.display());
        return ExitCode::from(2);
    }

    let nest = nest_prefix_map();
    let routes = collect_routes(&network);
    let paths = collect_openapi_paths(&openapi);

    let missing: Vec<_> = routes
        .iter()
        .filter(|r| !is_ignored(r))
        .filter(|r| !paths.contains(&openapi_path_for_route(r, &nest)))
        .cloned()
        .collect();

    println!("routes in src/network: {}", routes.len());
    println!("paths in openapi.yaml: {}", paths.len());
    println!("\n=== In code, missing from openapi.yaml ===");
    for m in &missing {
        println!("{m}");
    }
    println!("\nTotal missing: {}", missing.len());

    if missing.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_route_literal() {
        let src = r#".route("/api/v1/health", get(handler))"#;
        assert_eq!(
            extract_route_literals(src),
            vec!["/api/v1/health".to_string()]
        );
    }

    #[test]
    fn nest_prefix_ai_ml() {
        let nest = nest_prefix_map();
        assert_eq!(
            openapi_path_for_route("/pipeline/{id}", &nest),
            "/ai-ml/pipeline/{id}"
        );
    }

    #[test]
    fn parses_openapi_path_line() {
        assert_eq!(
            parse_openapi_path_line("  /vm/templates:"),
            Some("/vm/templates".into())
        );
    }

    #[test]
    fn ignores_raft_wire_routes() {
        assert!(is_ignored("/raft/vote"));
        assert!(is_ignored("/raft/append-entries"));
        assert!(!is_ignored("/api/v1/health"));
    }
}
