//! GSV ratio box API contracts — Rust integration tests.
//!
//! Scope: the Rust 95–100% LOC-ratio audit (`audit`/`save`/`load`/`wire`) and the
//! `/api/ratio` endpoint. The real-workspace audit runs against the enclosing
//! `poolAI` git repo (paths under `GSV/`); API tests use a temp data dir so the
//! durable `GSV/data/rust_ratio.json` is untouched.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use gsv::boxes::ratio::{audit, load, save, wire};
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
    let dir = std::env::temp_dir().join(format!("gsv-ratio-{tag}-{}", std::process::id()));
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
fn ratio_audit_reports_real_workspace_within_band() {
    let report = audit(&repo_root()).expect("audit");
    assert!(
        report.product_loc_total > 0,
        "expected git-tracked GSV product files"
    );
    assert!(report.rust_loc > 0);
    assert!(
        report.rust_ratio >= report.formal_band_min,
        "Rust ratio {:.2}% below formal band {:.2}%",
        report.rust_ratio_pct,
        report.formal_band_min * 100.0
    );
    assert!(report.meets_min_ratio);
    assert!((report.formal_band_min - 0.95).abs() < f64::EPSILON);
    assert!(report.by_category.contains_key("rust_src"));
    assert!(report.by_category.contains_key("ui_html"));
    assert!(report.notes.is_empty());
}

#[test]
fn ratio_save_load_roundtrip_preserves_fields() {
    let dir = temp_data_dir("roundtrip");
    let report = audit(&repo_root()).expect("audit");
    save(&report, &dir).expect("save");
    assert!(dir.join("rust_ratio.json").exists());
    let loaded = load(&dir).expect("load");
    assert_eq!(loaded.rust_loc, report.rust_loc);
    assert_eq!(loaded.non_rust_product_loc, report.non_rust_product_loc);
    assert_eq!(loaded.product_loc_total, report.product_loc_total);
    assert!((loaded.rust_ratio - report.rust_ratio).abs() < f64::EPSILON);
    assert_eq!(loaded.meets_min_ratio, report.meets_min_ratio);
    assert_eq!(loaded.stretch_target, report.stretch_target);
    assert_eq!(loaded.meets_stretch_96, report.meets_stretch_96);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ratio_wire_reports_ok_false_when_store_missing() {
    let dir = temp_data_dir("missing");
    let v = wire(&dir);
    assert_eq!(v["ok"], false);
    assert!(v["error"].is_string());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ratio_wire_reports_ok_true_with_ratio() {
    let dir = temp_data_dir("present");
    save(&audit(&repo_root()).expect("audit"), &dir).expect("save");
    let v = wire(&dir);
    assert_eq!(v["ok"], true);
    assert!(v["rust_ratio"].is_number());
    assert!(v["rust_ratio_pct"].is_number());
    assert!(v["meets_min_ratio"].is_boolean());
    assert!(v["meets_stretch_96"].is_boolean());
    assert_eq!(v["stretch_target"].as_f64(), Some(0.96));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn api_ratio_endpoint_serves_stored_report() {
    let dir = temp_data_dir("api-ok");
    save(&audit(&repo_root()).expect("audit"), &dir).expect("save");
    let app = app(dir.clone());
    let (status, json) = get(&app, "/api/ratio").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["rust_ratio_pct"]
        .as_f64()
        .map(|p| p >= 95.0)
        .unwrap_or(false));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn api_ratio_endpoint_ok_false_without_store() {
    let dir = temp_data_dir("api-missing");
    let app = app(dir.clone());
    let (status, json) = get(&app, "/api/ratio").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], false);
    assert!(json["error"].is_string());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ratio_report_json_shape_is_stable() {
    let dir = temp_data_dir("shape");
    save(&audit(&repo_root()).expect("audit"), &dir).expect("save");
    let raw = std::fs::read_to_string(dir.join("rust_ratio.json")).expect("read");
    let v: Value = serde_json::from_str(&raw).expect("json");
    for key in [
        "generated_at",
        "rust_loc",
        "non_rust_product_loc",
        "product_loc_total",
        "rust_ratio",
        "rust_ratio_pct",
        "formal_band_min",
        "min_ratio",
        "meets_min_ratio",
        "stretch_target",
        "meets_stretch_96",
        "by_category",
        "notes",
    ] {
        assert!(v.get(key).is_some(), "missing key {key}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}
