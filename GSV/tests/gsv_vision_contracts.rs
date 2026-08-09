//! GSV vision box API contracts — Rust integration tests.
//!
//! Scope: the poolAI vision mirror (`read_manifest`/`read_feed`/`sync`/`drift`
//! and the `/api/vision*` endpoints). Real-workspace reads run against the
//! enclosing `poolAI` repo (`docs/vision/`); API tests use a temp data dir so the
//! durable `GSV/data/gsv_*.json` snapshots are untouched.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use gsv::boxes::vision::{self, collect_drift};
use gsv::server::router;
use gsv::AppState;
use serde_json::Value;
use tokio::sync::broadcast;
use tower::ServiceExt;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("poolAI parent")
        .to_path_buf()
}

fn temp_data_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("gsv-vision-{tag}-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn app(data_dir: PathBuf) -> axum::Router {
    let (tx, _rx) = broadcast::channel(64);
    let state = AppState::new(Some(repo_root()), Some(data_dir), tx);
    router(state)
}

async fn get(app: &axum::Router, path: &str) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(Request::get(path).body(Body::empty()).expect("req"))
        .await
        .expect("resp");
    let status = res.status();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[test]
fn vision_manifest_reads_real_workspace() {
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    assert!(m.revision > 0, "revision must be set");
    assert!(!m.layers.is_empty());
    assert!(
        m.nodes.len() >= 1000,
        "expected the galaxy graph nodes, got {}",
        m.nodes.len()
    );
    assert!(
        m.edges.len() >= 500,
        "expected the galaxy graph edges, got {}",
        m.edges.len()
    );
    let first = m.nodes.first().expect("first node");
    assert!(!first.id.is_empty());
    assert!(!first.layer.is_empty());
    assert!(!m.next_sprint.is_empty());
}

#[test]
fn vision_feed_reads_real_workspace() {
    let f = vision::read_feed(&repo_root()).expect("feed");
    assert!(!f.title.is_empty());
    assert!(
        f.items.len() >= 12,
        "expected the sprint ticker, got {}",
        f.items.len()
    );
    assert!(
        f.items.iter().any(|i| i.id == "PH-S1828"),
        "feed must include the band-118 close entry"
    );
    for item in &f.items {
        assert!(!item.id.is_empty());
        assert!(!item.title.is_empty());
    }
}

#[test]
fn vision_sync_writes_snapshots_to_data_dir() {
    let dir = temp_data_dir("sync");
    let report = vision::sync(&repo_root(), &dir).expect("sync");
    assert_eq!(
        report.revision,
        vision::read_manifest(&repo_root()).unwrap().revision
    );
    assert!(dir.join("gsv_manifest.json").exists());
    assert!(dir.join("gsv_feed.json").exists());
    assert!(dir.join("gsv_extensions.json").exists());
    assert_eq!(
        vision::load_manifest(&dir).expect("load manifest").revision,
        report.revision
    );
    assert_eq!(
        vision::load_feed(&dir).expect("load feed").items.len(),
        report.feed_items as usize
    );
    assert_eq!(
        vision::load_extensions(&dir)
            .expect("load extensions")
            .revision,
        vision::read_extensions(&repo_root())
            .expect("extensions")
            .revision
    );
}

#[test]
fn vision_extensions_reads_real_workspace() {
    let e = vision::read_extensions(&repo_root()).expect("extensions");
    assert!(e.revision > 0, "extensions revision must be set");
    assert!(!e.active_sprint.is_empty());
    assert!(!e.scope_ids().is_empty(), "planning scopes must be present");
    assert_eq!(
        e.active_sprint,
        vision::read_manifest(&repo_root()).unwrap().next_sprint,
        "extensions active sprint must match the manifest next sprint"
    );
}

#[test]
fn vision_drift_green_on_real_workspace() {
    let dir = temp_data_dir("drift");
    let issues = collect_drift(&repo_root(), &dir);
    assert!(
        issues.is_empty(),
        "expected a green drift gate, got: {issues:?}"
    );
}

#[tokio::test]
async fn vision_api_summary_endpoint() {
    let dir = temp_data_dir("api");
    let (status, body) = get(&app(dir), "/api/vision").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["revision"].as_u64().unwrap_or(0) > 0);
    assert!(body["nodes_count"].as_u64().unwrap_or(0) >= 1000);
    assert!(body["edges_count"].as_u64().unwrap_or(0) >= 500);
    assert!(!body["next_sprint"].as_str().unwrap_or("").is_empty());
    assert!(body["feed_items"].as_array().is_some());
}

#[tokio::test]
async fn vision_api_manifest_endpoint() {
    let dir = temp_data_dir("api-manifest");
    let (status, body) = get(&app(dir), "/api/vision/manifest").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    let nodes = body["nodes"].as_array().expect("nodes array");
    let edges = body["edges"].as_array().expect("edges array");
    assert!(nodes.len() >= 1000);
    assert!(edges.len() >= 500);
    assert_eq!(body["layers"].as_array().unwrap().len(), 6);
}

#[tokio::test]
async fn vision_api_feed_endpoint() {
    let dir = temp_data_dir("api-feed");
    let (status, body) = get(&app(dir), "/api/vision/feed").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    let items = body["feed"]["items"].as_array().expect("feed items");
    assert!(items.len() >= 12);
}

#[tokio::test]
async fn vision_api_map_endpoint() {
    let dir = temp_data_dir("api-map");
    let (status, body) = get(&app(dir), "/api/vision/map").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["nodes_count"].as_u64().unwrap_or(0) >= 1000);
    assert!(body["edges_count"].as_u64().unwrap_or(0) >= 500);
    let layers = body["layers"].as_array().expect("layers array");
    assert_eq!(layers.len(), 6, "L0..L5 map layers");
    assert_eq!(layers[0]["id"], "L0");
    assert_eq!(layers[5]["id"], "L5");
    let zs: Vec<i64> = layers.iter().filter_map(|l| l["z"].as_i64()).collect();
    let mut sorted = zs.clone();
    sorted.sort();
    assert_eq!(zs, sorted, "layers must be z-sorted");
    let layer_sum: u64 = layers.iter().filter_map(|l| l["node_count"].as_u64()).sum();
    assert_eq!(layer_sum, body["nodes_count"].as_u64().unwrap_or(0));
    assert!(body["edge_kinds"]
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false));
}

#[tokio::test]
async fn vision_api_feed_filter_closed() {
    let dir = temp_data_dir("api-feed-filter");
    let (status, body) = get(&app(dir), "/api/vision/feed?status=closed").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    let items = body["feed"]["items"].as_array().expect("feed items");
    assert!(items.len() >= 12);
    for item in items {
        assert_eq!(item["status"], "closed");
    }
}

#[tokio::test]
async fn vision_api_sprint_map_endpoint() {
    let dir = temp_data_dir("api-sprint-map");
    let (status, body) = get(&app(dir), "/api/vision/sprint-map").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["revision"].as_u64().unwrap_or(0) > 0);
    assert!(!body["next_sprint"].as_str().unwrap_or("").is_empty());
    let kinds = body["kinds"].as_array().expect("kinds array");
    assert!(
        kinds.iter().all(|k| matches!(
            k["kind"].as_str(),
            Some("sprint-scope") | Some("queue") | Some("session-tracks")
        )),
        "sprint-map kinds must be scoping/tracking edges: {kinds:?}"
    );
    let layers = body["layers"].as_array().expect("layers array");
    let zs: Vec<i64> = layers.iter().filter_map(|l| l["z"].as_i64()).collect();
    let mut sorted = zs.clone();
    sorted.sort();
    assert_eq!(zs, sorted, "layers must be z-sorted");
}

#[test]
fn vision_sprint_map_reads_real_workspace() {
    let r = vision::sprint_map_report(&repo_root(), &temp_data_dir("sprint-map")).expect("report");
    assert!(r.revision > 0);
    assert!(!r.next_sprint.is_empty());
    assert!(
        r.kinds
            .iter()
            .all(|k| matches!(k.kind.as_str(), "sprint-scope" | "queue" | "session-tracks")),
        "unexpected sprint-map kind: {r:?}"
    );
    let module_ids: Vec<&str> = r.modules.iter().map(|m| m.id.as_str()).collect();
    assert!(
        module_ids
            .iter()
            .any(|id| id.starts_with("PH-S") || id.contains("handoff")),
        "expected sprint modules, got: {module_ids:?}"
    );
    for m in &r.modules {
        assert!(m.targets >= 1);
    }
}

#[tokio::test]
async fn vision_api_doc_preview_endpoint() {
    let dir = temp_data_dir("api-doc-preview");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let probe = m.edges.first().map(|e| e.from.as_str()).expect("edge from");
    let (status, body) = get(
        &app(dir.clone()),
        &format!("/api/vision/doc-preview?id={probe}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert_eq!(body["node"]["id"], probe);
    assert_eq!(
        body["link_count"].as_u64().unwrap_or(0),
        (body["links_out"].as_array().unwrap().len() + body["links_in"].as_array().unwrap().len())
            as u64
    );

    let (missing_status, missing) =
        get(&app(dir.clone()), "/api/vision/doc-preview?id=no-such-node").await;
    assert_eq!(missing_status, StatusCode::OK);
    assert_eq!(missing["ok"], false);
    assert!(missing["error"]
        .as_str()
        .unwrap_or("")
        .contains("not found"));

    let (empty_status, empty) = get(&app(dir), "/api/vision/doc-preview").await;
    assert_eq!(empty_status, StatusCode::OK);
    assert_eq!(empty["ok"], false);
}

#[test]
fn vision_doc_preview_reads_real_workspace() {
    let dir = temp_data_dir("doc-preview");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let probe = m.edges.first().expect("edge").from.clone();
    let r = vision::doc_preview(&repo_root(), &dir, &probe).expect("report");
    assert_eq!(r.node.id, probe);
    assert!(r.link_count >= 1);
    assert_eq!(r.link_count as usize, r.links_out.len() + r.links_in.len());
    assert!(
        r.links_out.iter().all(|l| !l.node.id.is_empty())
            && r.links_in.iter().all(|l| !l.node.id.is_empty())
    );
}

#[tokio::test]
async fn vision_assets_svg_served() {
    let dir = temp_data_dir("api-svg");
    let res = app(dir)
        .oneshot(
            Request::get("/assets/vision.svg")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("resp");
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("image/svg+xml")
    );
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    let body = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert!(body.contains("<svg"), "must be an SVG document");
    assert!(body.contains("PoolAI Galaxy Starwalker Vision"));
}

#[tokio::test]
async fn vision_api_sync_endpoint() {
    let dir = temp_data_dir("api-sync");
    let (status, body) = get(&app(dir.clone()), "/api/vision/sync").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["revision"].as_u64().unwrap_or(0) > 0);
    assert_eq!(
        body["drift"],
        serde_json::json!([]),
        "auto-sync must report an empty drift gate"
    );
    assert!(dir.join("gsv_extensions.json").exists());
    assert!(!body["synced_at"].as_str().unwrap_or("").is_empty());
}

#[tokio::test]
async fn vision_api_extensions_endpoint() {
    let dir = temp_data_dir("api-extensions");
    let (status, body) = get(&app(dir), "/api/vision/extensions").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["revision"].as_u64().unwrap_or(0) > 0);
    assert!(!body["active_sprint"].as_str().unwrap_or("").is_empty());
    assert!(body["scope_count"].as_u64().unwrap_or(0) > 0);
    let scopes = body["scopes"].as_array().expect("scopes array");
    assert!(!scopes.is_empty());
}

#[tokio::test]
async fn vision_api_sprint_queue_endpoint() {
    let dir = temp_data_dir("api-sprint-queue");
    let (status, body) = get(&app(dir), "/api/vision/sprint-queue").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["ok"], true);
    assert!(body["revision"].as_u64().unwrap_or(0) > 0);
    assert!(!body["next_sprint"].as_str().unwrap_or("").is_empty());
    let active = body["active_sprint"].as_str().unwrap_or("").to_string();
    assert_eq!(active, body["next_sprint"], "active == next sprint");
    let planned = body["planned"].as_array().expect("planned array");
    assert!(
        planned.iter().any(|p| p["id"] == serde_json::json!(active)),
        "planned queue must include the active sprint"
    );
    assert!(body["open_count"].is_u64());
    let entries = body["entries"].as_array().expect("entries array");
    for entry in entries {
        assert!(
            planned.iter().any(|p| p == entry),
            "every manifest queue entry must appear in planned"
        );
    }
    assert_eq!(
        body["open_count"].as_u64().unwrap_or(0) as usize,
        entries.len(),
        "open_count must equal manifest queue entries"
    );
}

#[test]
fn vision_sprint_queue_reads_real_workspace() {
    let dir = temp_data_dir("sprint-queue");
    let r = vision::sprint_queue_report(&repo_root(), &dir).expect("report");
    assert!(r.revision > 0);
    assert!(!r.next_sprint.is_empty());
    assert_eq!(r.active_sprint, r.next_sprint);
    assert_eq!(
        r.open_count as usize,
        r.entries.len(),
        "open_count must equal manifest queue entries"
    );
    assert!(
        r.planned.iter().any(|p| p.id == r.active_sprint),
        "planned must include the active sprint"
    );
}

#[test]
fn vision_node_search_finds_galaxy_nodes() {
    let dir = temp_data_dir("node-search");
    let r = vision::node_search(&repo_root(), &dir, "galaxy", None).expect("search");
    assert!(r.revision > 0);
    assert_eq!(r.query, "galaxy");
    assert!(r.layer.is_empty());
    assert!(r.total_matches > 0);
    assert!(!r.results.is_empty());
    let layers = r
        .results
        .iter()
        .map(|res| res.layer.clone())
        .collect::<Vec<_>>();
    let sorted = {
        let mut v = layers.clone();
        v.sort();
        v
    };
    assert_eq!(layers, sorted, "results must be layer-z sorted then by id");
    for res in &r.results {
        assert!(!res.id.is_empty());
        assert!(!res.label.is_empty());
        assert!(!res.layer.is_empty());
        assert!(
            res.links_out > 0 || res.links_in > 0,
            "galaxy nodes are connected to at least one edge"
        );
    }
}

#[test]
fn vision_node_search_filters_by_layer() {
    let dir = temp_data_dir("node-search-layer");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let layer_id = m.layers.first().map(|l| l.id.clone()).expect("layer");
    let r = vision::node_search(&repo_root(), &dir, "", Some(&layer_id)).expect("search");
    assert_eq!(r.layer, layer_id);
    assert!(r.total_matches > 0);
    assert!(r.results.iter().all(|res| res.layer == layer_id));
}

#[test]
fn vision_node_search_no_match_empty() {
    let dir = temp_data_dir("node-search-empty");
    let r = vision::node_search(&repo_root(), &dir, "zzz-no-such-node-42", None).expect("search");
    assert_eq!(r.total_matches, 0);
    assert!(r.results.is_empty());
    let all = vision::node_search(&repo_root(), &dir, "", None).expect("search all");
    assert!(
        all.total_matches as usize > vision::NODE_SEARCH_LIMIT,
        "galaxy has more nodes than the search cap"
    );
    assert_eq!(all.results.len(), vision::NODE_SEARCH_LIMIT);
}

#[test]
fn vision_sprint_board_groups_queue() {
    let dir = temp_data_dir("sprint-board");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    assert!(r.revision > 0);
    assert_eq!(r.next_sprint, r.active_sprint);
    assert_eq!(
        r.open_count + r.closed_count,
        r.total,
        "open + closed must equal the working queue total"
    );
    let names: Vec<&str> = r.columns.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"open"));
    assert!(names.contains(&"closed"));
    assert!(names.contains(&"planned"));
    let summed: u64 = r.columns.iter().map(|c| c.count).sum();
    assert_eq!(summed, r.total, "column counts must sum to the total");
    for column in &r.columns {
        assert_eq!(column.entries.len() as u64, column.count);
    }
}

#[test]
fn vision_sprint_board_progress_pct_range() {
    let dir = temp_data_dir("sprint-board-pct");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    assert!(r.progress_pct <= 100, "progress pct must be in [0,100]");
    if r.total > 0 {
        assert_eq!(
            r.progress_pct,
            (r.closed_count * 100) / r.total,
            "progress pct must be closed/total"
        );
    } else {
        assert_eq!(r.progress_pct, 0);
    }
}

#[test]
fn vision_sprint_progress_layers_match_manifest() {
    let dir = temp_data_dir("sprint-progress-layers");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    assert_eq!(r.revision, m.revision);
    assert_eq!(
        r.layers.len(),
        m.layers.len(),
        "per-layer distribution must match manifest layers"
    );
    let total_nodes: u64 = r.layers.iter().map(|l| l.node_count).sum();
    assert_eq!(total_nodes, m.nodes.len() as u64);
    for (report_layer, manifest_layer) in r.layers.iter().zip(m.layers.iter()) {
        assert_eq!(report_layer.id, manifest_layer.id);
        assert!(report_layer.linked_count <= report_layer.node_count);
    }
}

#[test]
fn vision_sprint_progress_statuses_sum() {
    let dir = temp_data_dir("sprint-progress-statuses");
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    assert_eq!(
        r.open_count + r.closed_count + r.planned_count,
        r.total,
        "status counts must sum to the total"
    );
    assert!(r.progress_pct <= 100);
}

#[test]
fn vision_sprint_board_column_order_contract() {
    let dir = temp_data_dir("sprint-board-order");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    let names: Vec<&str> = r.columns.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["open", "closed", "planned"],
        "column order is fixed"
    );
}

#[test]
fn vision_sprint_board_active_sprint_in_open_column() {
    let dir = temp_data_dir("sprint-board-active");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    let open = r
        .columns
        .iter()
        .find(|c| c.name == "open")
        .expect("open column");
    assert!(
        open.entries.iter().any(|e| e.id == r.active_sprint),
        "the active sprint must sit in the open column"
    );
}

#[test]
fn vision_sprint_board_entries_unique_across_columns() {
    let dir = temp_data_dir("sprint-board-unique");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    let mut seen: Vec<String> = Vec::new();
    for column in &r.columns {
        for entry in &column.entries {
            assert!(
                !seen.contains(&entry.id),
                "id {} must appear in exactly one column",
                entry.id
            );
            seen.push(entry.id.clone());
        }
    }
    assert_eq!(
        seen.len() as u64,
        r.total,
        "columns must cover the whole working queue"
    );
}

#[test]
fn vision_sprint_board_closed_column_only_done() {
    let dir = temp_data_dir("sprint-board-closed");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    let closed = r
        .columns
        .iter()
        .find(|c| c.name == "closed")
        .expect("closed column");
    for entry in &closed.entries {
        assert!(
            entry.status == "closed" || entry.status == "done",
            "closed column must only hold finished sprints, got {}",
            entry.status
        );
    }
}

#[test]
fn vision_sprint_board_revision_matches_manifest() {
    let dir = temp_data_dir("sprint-board-revision");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let r = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    assert_eq!(r.revision, m.revision);
    assert_eq!(r.next_sprint, m.next_sprint);
    assert!(!r.last_sprint_closed.is_empty());
}

#[test]
fn vision_sprint_progress_layers_z_ordered() {
    let dir = temp_data_dir("sprint-progress-z");
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    let zs: Vec<i64> = r.layers.iter().map(|l| l.z).collect();
    let mut sorted = zs.clone();
    sorted.sort();
    assert_eq!(zs, sorted, "layers must be z-ascending");
}

#[test]
fn vision_sprint_progress_linked_counts_reflect_queue_sprints() {
    let dir = temp_data_dir("sprint-progress-linked");
    let m = vision::read_manifest(&repo_root()).expect("manifest");
    let q = vision::sprint_queue_report(&repo_root(), &dir).expect("queue");
    let ids: std::collections::BTreeSet<&str> = q.planned.iter().map(|e| e.id.as_str()).collect();
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    for layer in &r.layers {
        let expected = m
            .nodes
            .iter()
            .filter(|n| n.layer == layer.id)
            .filter(|n| n.sprints.iter().any(|s| ids.contains(s.as_str())))
            .count() as u64;
        assert_eq!(
            layer.linked_count, expected,
            "linked_count for {} must match nodes referencing queue sprints",
            layer.id
        );
    }
}

#[test]
fn vision_sprint_progress_planned_count_formula() {
    let dir = temp_data_dir("sprint-progress-planned");
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    assert_eq!(
        r.planned_count,
        r.total.saturating_sub(r.open_count + r.closed_count),
        "planned must be the residual status bucket"
    );
}

#[test]
fn vision_wire_sprint_board_ok_true() {
    let dir = temp_data_dir("wire-sprint-board");
    let wire = vision::wire_sprint_board(&repo_root(), &dir);
    assert_eq!(wire["ok"], true);
    assert!(wire["columns"].is_array());
    assert!(wire["progress_pct"].is_u64());
}

#[test]
fn vision_wire_sprint_progress_ok_true() {
    let dir = temp_data_dir("wire-sprint-progress");
    let wire = vision::wire_sprint_progress(&repo_root(), &dir);
    assert_eq!(wire["ok"], true);
    assert!(wire["layers"].is_array());
    assert!(wire["progress_pct"].is_u64());
}

#[test]
fn vision_sprint_progress_links_capped_by_node_count() {
    let dir = temp_data_dir("sprint-progress-cap");
    let r = vision::sprint_progress_report(&repo_root(), &dir).expect("progress");
    for layer in &r.layers {
        assert!(
            layer.linked_count <= layer.node_count,
            "layer {} linked ({}) cannot exceed nodes ({})",
            layer.id,
            layer.linked_count,
            layer.node_count
        );
    }
    let linked_total: u64 = r.layers.iter().map(|l| l.linked_count).sum();
    let node_total: u64 = r.layers.iter().map(|l| l.node_count).sum();
    assert!(
        linked_total <= node_total,
        "aggregate linked cannot exceed nodes"
    );
    assert_eq!(
        r.progress_pct,
        (r.closed_count * 100) / r.total,
        "progress pct must equal closed/total"
    );
}

#[test]
fn vision_sprint_board_wire_matches_report_function() {
    let dir = temp_data_dir("sprint-board-wire");
    let report = vision::sprint_board_report(&repo_root(), &dir).expect("board");
    let wire = vision::wire_sprint_board(&repo_root(), &dir);
    assert_eq!(wire["total"], report.total);
    assert_eq!(wire["open_count"], report.open_count);
    assert_eq!(wire["closed_count"], report.closed_count);
    assert_eq!(wire["active_sprint"], report.active_sprint);
    assert_eq!(wire["columns"].as_array().map(|a| a.len()).unwrap_or(0), 3);
}

#[test]
fn vision_speed_index_reads_real_workspace() {
    let r = vision::read_speed_index(&repo_root()).expect("speed_index.json");
    assert!(r.test_ci_count > 0, "expected test_ci_history rows");
    assert!(r.latest.test_ci_wall_secs > 0.0);
    assert!(!r.host_label.is_empty());
    let wire = vision::wire_speed_index(&repo_root(), &temp_data_dir("speed-wire"));
    assert_eq!(wire["ok"], true);
    assert_eq!(wire["present"], true);
    assert_eq!(wire["speed_index"]["test_ci_count"], r.test_ci_count);
    assert_eq!(
        wire["speed_index"]["latest"]["test_ci_wall_secs"],
        r.latest.test_ci_wall_secs
    );
}

#[test]
fn vision_rust_diagnostics_reads_real_workspace() {
    let r = vision::read_rust_diagnostics(&repo_root()).expect("rust_diagnostics.json");
    assert!(r.history_count > 0, "expected history rows");
    assert!(!r.host_label.is_empty());
    let wire = vision::wire_rust_diagnostics(&repo_root(), &temp_data_dir("rustdiag-wire"));
    assert_eq!(wire["ok"], true);
    assert_eq!(wire["present"], true);
    assert_eq!(
        wire["rust_diagnostics"]["latest"]["warnings"],
        r.latest.warnings
    );
    assert_eq!(wire["rust_diagnostics"]["history_count"], r.history_count);
}

#[test]
fn vision_sprint_theme_reads_real_workspace() {
    let dir = temp_data_dir("sprint-theme");
    let r = vision::sprint_theme_report(&repo_root(), &dir).expect("theme");
    assert!(r.revision > 0);
    assert!(!r.git_head.is_empty());
    assert!(!r.active_sprint.is_empty());
    assert!(!r.next_sprint.is_empty());
    assert_eq!(r.active_sprint, r.next_sprint);
    assert_eq!(r.sprint, "#a78bfa");
    assert_eq!(r.sprint_next, "#c4b5fd");
    assert_eq!(r.pill.color, "#d4c4ff");
    assert_eq!(r.queue.closed_opacity, "0.55");
    assert_eq!(r.layers.len(), 6, "six manifest layers L0..L5");
    assert!(r
        .layers
        .iter()
        .any(|l| l.id == "L0" && l.color == "#3d6a9e"));
    assert!(r
        .layers
        .iter()
        .any(|l| l.id == "L5" && l.color == "#4a6880"));
    assert!(
        r.edge_kinds
            .iter()
            .any(|k| k.kind == "catalog" && k.color == "#7eb8ff"),
        "unknown edge kinds must fall back to the accent blue"
    );
    assert!(r.edge_kinds.iter().all(|k| k.color.starts_with('#')));
}

#[test]
fn vision_sprint_theme_wire_shapes() {
    let dir = temp_data_dir("sprint-theme-wire");
    let wire = vision::wire_sprint_theme(&repo_root(), &dir);
    assert_eq!(wire["ok"], true);
    assert!(wire["revision"].as_u64().unwrap_or(0) > 0);
    assert_eq!(wire["active_sprint"], wire["next_sprint"]);
    assert_eq!(wire["sprint"], "#a78bfa");
    assert_eq!(wire["pill"]["border"], "rgba(167, 139, 250, 0.4)");
    assert!(wire["layers"].as_array().map(|a| a.len()).unwrap_or(0) >= 6);
    assert!(wire["edge_kinds"].as_array().map(|a| a.len()).unwrap_or(0) > 0);
}

#[test]
fn vision_sprint_focus_svg_renders_active_sprint() {
    let dir = temp_data_dir("sprint-focus");
    let svg = vision::sprint_focus_svg(&repo_root(), &dir, "PH-S146");
    assert!(svg.starts_with("<svg"), "must render a real SVG map");
    assert!(svg.contains("sprint focus: PH-S146"));
    assert!(
        svg.contains("#a78bfa"),
        "in-scope nodes must be sprint accent"
    );
    assert!(
        svg.contains("opacity=\"0.22\""),
        "out-of-scope nodes must dim"
    );

    let default = vision::sprint_focus_svg(&repo_root(), &dir, "");
    assert!(
        default.starts_with("<svg"),
        "default must fall back to a sprint"
    );
    assert!(default.contains("sprint focus:"));
}

#[test]
fn vision_sprint_focus_svg_empty_state_for_unknown_sprint() {
    let dir = temp_data_dir("sprint-focus-empty");
    let svg = vision::sprint_focus_svg(&repo_root(), &dir, "NO-SUCH-SPRINT");
    assert!(svg.contains("no galaxy nodes reference this sprint"));
}

#[test]
fn vision_palette_legacy_root_values() {
    let p = vision::GalaxyPalette::legacy();
    assert_eq!(p.bg_deep, "#06080f");
    assert_eq!(p.bg, "#0a0e18");
    assert_eq!(p.panel, "rgba(18, 26, 42, 0.72)");
    assert_eq!(p.panel_solid, "#141c2e");
    assert_eq!(p.border, "rgba(120, 160, 220, 0.18)");
    assert_eq!(p.border_bright, "rgba(138, 180, 248, 0.45)");
    assert_eq!(p.text, "#eef2ff");
    assert_eq!(p.muted, "#8b9cb8");
    assert_eq!(p.accent, "#7eb8ff");
    assert_eq!(p.accent_2, "#c4a5ff");
    assert_eq!(p.glow, "rgba(126, 184, 255, 0.35)");
    assert_eq!(p.sidebar_w, "272px");
    assert_eq!(p.edge_docs, "#90c490");
    assert_eq!(p.edge_code, "#c49ab0");
    assert_eq!(p.edge_toml, "#7eb8c4");
    assert_eq!(p.ext_md, "#58a6ff");
    assert_eq!(p.ext_rs, "#f0883e");
    assert_eq!(p.ext_json, "#3fb950");
    assert_eq!(p.ext_toml, "#7eb8c4");
    assert_eq!(p.sprint, "#a78bfa");
    assert_eq!(p.bg_tone, "0.8");
    assert_eq!(p.galaxy_bg_opacity, "0.15");
}

#[test]
fn vision_palette_wire_ok_with_real_workspace() {
    let dir = temp_data_dir("palette-wire");
    let wire = vision::wire_palette(&repo_root(), &dir);
    assert_eq!(wire["ok"], true);
    assert!(wire["revision"].as_u64().unwrap_or(0) > 0);
    assert_eq!(wire["bg_deep"], "#06080f");
    assert_eq!(wire["accent"], "#7eb8ff");
    assert_eq!(wire["sprint"], "#a78bfa");
    assert!(wire["layers"].as_array().map(|a| a.len()).unwrap_or(0) >= 6);
    assert!(wire["layers_dim"].as_array().map(|a| a.len()).unwrap_or(0) >= 6);
}

#[test]
fn vision_palette_css_root_contains_all_legacy_vars() {
    let css = vision::GalaxyPalette::legacy().as_css_root();
    for name in [
        "--bg-deep",
        "--bg",
        "--panel",
        "--panel-solid",
        "--border",
        "--border-bright",
        "--text",
        "--muted",
        "--accent",
        "--accent-2",
        "--glow",
        "--sidebar-w",
        "--L0",
        "--L1",
        "--L2",
        "--L3",
        "--L4",
        "--L5",
        "--L0-dim",
        "--L1-dim",
        "--L2-dim",
        "--L3-dim",
        "--L4-dim",
        "--L5-dim",
        "--edge-docs",
        "--edge-code",
        "--edge-toml",
        "--ext-md",
        "--ext-rs",
        "--ext-json",
        "--ext-toml",
        "--sprint",
        "--bg-tone",
        "--galaxy-bg-opacity",
    ] {
        assert!(css.contains(name), "missing legacy var {name}");
    }
}

#[test]
fn vision_starfield_svg_modes_shape_and_empty_state() {
    let dir = temp_data_dir("starfield-svg");
    let eco = vision::starfield_svg_wire(&repo_root(), &dir, Some("eco"));
    let fx = vision::starfield_svg_wire(&repo_root(), &dir, Some("fx"));
    let ms = vision::starfield_svg_wire(&repo_root(), &dir, Some("ms"));
    assert!(eco.starts_with("<svg"));
    assert!(fx.starts_with("<svg"));
    assert!(ms.starts_with("<svg"));
    assert!(eco.contains("starfield · Eco · 48 stars"));
    assert!(fx.contains("starfield · FX · 160 stars"));
    assert!(ms.contains("starfield · Ms · 96 stars"));
    assert!(
        fx.contains("rgba(126, 184, 255, 0.14)"),
        "FX adds glow halo"
    );
    assert!(
        !eco.contains("rgba(126, 184, 255, 0.14)"),
        "Eco has no glow"
    );
    let no_mode = vision::starfield_svg_wire(&repo_root(), &dir, None);
    assert!(
        no_mode.contains("starfield · FX · 160 stars"),
        "default → FX"
    );
}

#[test]
fn vision_galaxy_svg_nebula_parity() {
    let svg = vision::galaxy_svg();
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("<defs>"));
    assert!(svg.contains("radialGradient"));
    assert!(svg.contains("url(#g1)"));
    assert!(svg.contains("url(#g2)"));
    assert!(svg.contains("galaxy backdrop"));
    assert!(svg.contains("#7eb8ff"));
    assert!(svg.contains("#c4a5ff"));
    assert!(svg.contains("#06080f"));
}

#[test]
fn vision_starfield_galaxy_svg_wires_render() {
    let eco = vision::starfield_svg(vision::StarfieldMode::Eco);
    let fx = vision::starfield_svg(vision::StarfieldMode::Fx);
    let ms = vision::starfield_svg(vision::StarfieldMode::Ms);
    assert!(eco.starts_with("<svg"));
    assert!(fx.starts_with("<svg"));
    assert!(ms.starts_with("<svg"));
    assert_eq!(
        eco,
        vision::starfield_svg(vision::StarfieldMode::Eco),
        "deterministic"
    );
    let galaxy = vision::galaxy_svg();
    assert!(galaxy.starts_with("<svg"));
    assert!(galaxy.contains("radialGradient"));
    assert!(galaxy.contains("url(#g1)"));
    assert!(galaxy.contains("url(#g2)"));
}
