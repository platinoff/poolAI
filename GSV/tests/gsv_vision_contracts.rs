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
        f.items.iter().any(|i| i.id == "PH-S1748"),
        "feed must include the band-110 close entry"
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
    assert_eq!(
        vision::load_manifest(&dir).expect("load manifest").revision,
        report.revision
    );
    assert_eq!(
        vision::load_feed(&dir).expect("load feed").items.len(),
        report.feed_items as usize
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
