//! GSV server API contracts — Rust integration tests (HTTP/4xx/JSON).
//!
//! Uses `tower::ServiceExt::oneshot` against the axum router (no port binding).
//! Scope: all box endpoints return expected status codes + JSON shapes, and
//! error paths (404 / path-traversal) behave per canon.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use gsv::server::router;
use gsv::AppState;
use serde_json::Value;
use tokio::sync::broadcast;
use tower::ServiceExt;

/// Build a fresh app state + router for one test.
fn app() -> (axum::Router, AppState) {
    let (tx, _rx) = broadcast::channel(64);
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("GSV parent")
        .to_path_buf();
    let state = AppState::new(Some(repo_root), None, tx);
    (router(state.clone()), state)
}

async fn get(app: &axum::Router, path: &str) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .method(Method::GET)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    let status = res.status();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

async fn post(app: &axum::Router, path: &str, body: Value) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .method(Method::POST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    let status = res.status();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

async fn get_text(app: &axum::Router, path: &str) -> (StatusCode, String) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(path)
                .method(Method::GET)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    let status = res.status();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn index_serves_gsv_ui() {
    let (app, _state) = app();
    let res = app
        .oneshot(Request::get("/").body(Body::empty()).expect("req"))
        .await
        .expect("resp");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    let html = String::from_utf8_lossy(&bytes);
    assert!(html.contains("Galaxy StarWalker Vision"));
    assert!(html.contains("api/health"));
}

#[tokio::test]
async fn health_returns_ok() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["name"]
        .as_str()
        .unwrap_or_default()
        .contains("StarWalker"));
    assert!(json["version"].as_str().is_some());
}

#[tokio::test]
async fn tracker_returns_sprints_and_records() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/tracker").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["sprints"]["total"].is_number() || json["sprints"]["total"].is_null());
    assert!(json["records"].is_array());
}

#[tokio::test]
async fn sli_catalog_scans_repo() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/sli").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["catalog"]["roots"].is_array());
    // src/bin/ is scanned → at least one rs entry on a normal repo.
    assert!(json["catalog"]["entries"]
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false));
}

#[tokio::test]
async fn toolchain_inventory_present() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/toolchain").await;
    assert_eq!(status, StatusCode::OK);
    let entries = json["entries"].as_array().expect("entries");
    let tools: Vec<&str> = entries.iter().filter_map(|e| e["tool"].as_str()).collect();
    assert!(tools.iter().any(|t| *t == "rustc" || *t == "repo-head"));
}

#[tokio::test]
async fn ide_sessions_read_only() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/ide/sessions").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["sessions"].is_array());
    assert!(json["selection"].is_null() || json["selection"].is_object());
}

#[tokio::test]
async fn ide_select_sets_selection() {
    let (app, state) = app();
    let body = serde_json::json!({ "tool": "opencode", "session": "opencode/test.jsonl" });
    let (status, json) = post(&app, "/api/ide/select", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    let sel = state.ide_selection.try_read().expect("read");
    assert_eq!(sel.as_ref().map(|s| s.tool.as_str()), Some("opencode"));
}

#[tokio::test]
async fn update_wire_reports_version() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/update").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["version"].as_str().is_some());
    assert!(json["update_available"].is_boolean());
}

#[tokio::test]
async fn update_notify_flags_available() {
    let (app, state) = app();
    let (status, json) = post(&app, "/api/update/notify", serde_json::json!({})).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["update_available"], true);
    assert!(state.update_available());
    // reset for subsequent tests
    state.clear_update();
}

#[tokio::test]
async fn preview_renders_highlighted_html() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/preview?file=GSV/Cargo.toml").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["html"].as_str().unwrap_or_default().contains("g-pre"));
    assert_eq!(json["extension"], "toml");
}

#[tokio::test]
async fn preview_rejects_traversal() {
    let (app, _state) = app();
    let (status, _json) = get(&app, "/api/preview?file=../../secret").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn preview_missing_file_404() {
    let (app, _state) = app();
    let (status, _json) = get(&app, "/api/preview?file=nope/does-not-exist.rs").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn terminal_runs_whitelisted_echo() {
    let (app, _state) = app();
    let body = serde_json::json!({ "command": "echo gsv-contract" });
    let (status, json) = post(&app, "/api/terminal", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["allowed"], true);
    assert_eq!(json["exit_code"], 0);
}

#[tokio::test]
async fn terminal_blocks_injection() {
    let (app, _state) = app();
    let body = serde_json::json!({ "command": "echo safe; rm -rf /" });
    let (status, json) = post(&app, "/api/terminal", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["allowed"], false);
    assert!(json["stderr"]
        .as_str()
        .unwrap_or_default()
        .contains("forbidden"));
}

#[tokio::test]
async fn hooks_tests_read_only() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/hooks/tests").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["status"].as_str().is_some());
    assert!(json["test_bins"].is_array());
}

#[tokio::test]
async fn hooks_bench_read_only() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/hooks/bench").await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["status"].as_str().is_some());
}

#[tokio::test]
async fn unknown_route_404() {
    let (app, _state) = app();
    let (status, _json) = get(&app, "/api/does-not-exist").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn health_has_json_content_type() {
    let (app, _state) = app();
    let res = app
        .oneshot(
            Request::get("/api/health")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("resp");
    assert_eq!(res.status(), StatusCode::OK);
    let ct = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());
    assert!(ct.is_some_and(|v| v.contains("application/json")));
}

#[tokio::test]
async fn vision_node_search_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/node-search?q=galaxy").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["revision"].as_u64().unwrap_or(0) > 0);
    assert_eq!(json["query"], "galaxy");
    let results = json["results"].as_array().expect("results array");
    assert!(!results.is_empty());
    for res in results {
        assert!(res["id"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(res["links_out"].is_u64());
        assert!(res["links_in"].is_u64());
    }
    let (status, empty) = get(&app, "/api/vision/node-search").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty["ok"], true);
    assert!(empty["results"].is_array());
}

#[tokio::test]
async fn vision_sprint_board_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/sprint-board").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["revision"].as_u64().unwrap_or(0) > 0);
    assert!(json["total"].is_u64());
    assert_eq!(
        json["open_count"].as_u64().unwrap_or(0) + json["closed_count"].as_u64().unwrap_or(0),
        json["total"].as_u64().unwrap_or(0),
        "open + closed must equal the total"
    );
    assert!(json["progress_pct"].as_u64().is_some_and(|p| p <= 100));
    assert_eq!(json["next_sprint"], json["active_sprint"]);
    let columns = json["columns"].as_array().expect("columns array");
    assert_eq!(
        columns.len(),
        3,
        "board must expose open/closed/planned columns"
    );
    for column in columns {
        assert!(column["count"].is_u64());
        assert!(column["entries"].is_array());
        assert_eq!(
            column["entries"].as_array().unwrap().len() as u64,
            column["count"].as_u64().unwrap_or(0)
        );
    }
}

#[tokio::test]
async fn vision_sprint_progress_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/sprint-progress").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["revision"].as_u64().unwrap_or(0) > 0);
    assert!(json["total"].is_u64());
    assert_eq!(
        json["open_count"].as_u64().unwrap_or(0)
            + json["closed_count"].as_u64().unwrap_or(0)
            + json["planned_count"].as_u64().unwrap_or(0),
        json["total"].as_u64().unwrap_or(0)
    );
    let layers = json["layers"].as_array().expect("layers array");
    assert!(!layers.is_empty());
    for layer in layers {
        assert!(layer["id"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(layer["z"].is_i64());
        assert!(layer["node_count"].is_u64());
        assert!(layer["linked_count"].is_u64());
        assert!(
            layer["linked_count"].as_u64().unwrap_or(0)
                <= layer["node_count"].as_u64().unwrap_or(0)
        );
    }
}

#[tokio::test]
async fn vision_speeds_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/speeds").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["present"].is_boolean());
    assert!(json["speed_index"]["test_ci_count"].is_u64());
    assert!(json["speed_index"]["bench_count"].is_u64());
    assert!(json["speed_index"]["latest"]["test_ci_wall_secs"].is_f64());
    assert!(json["speed_index"]["latest"]["last_bench_median_ns"].is_u64());
    assert!(json["speed_index"]["host_label"].as_str().is_some());
}

#[tokio::test]
async fn vision_rust_diagnostics_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/rust-diagnostics").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["present"].is_boolean());
    assert!(json["rust_diagnostics"]["latest"]["warnings"].is_u64());
    assert!(json["rust_diagnostics"]["latest"]["errors"].is_u64());
    assert!(json["rust_diagnostics"]["latest"]["ok"].is_boolean());
    assert!(json["rust_diagnostics"]["history_count"].is_u64());
    assert!(json["rust_diagnostics"]["latest"]["top_codes"].is_array());
}

#[tokio::test]
async fn vision_sprint_theme_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/sprint-theme").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["revision"].as_u64().unwrap_or(0) > 0);
    assert!(!json["active_sprint"].as_str().unwrap_or("").is_empty());
    assert_eq!(json["next_sprint"], json["active_sprint"]);
    assert_eq!(json["sprint"], "#a78bfa");
    assert_eq!(json["sprint_next"], "#c4b5fd");
    assert_eq!(json["pill"]["bg"], "rgba(167, 139, 250, 0.2)");
    assert_eq!(json["pill"]["border"], "rgba(167, 139, 250, 0.4)");
    assert_eq!(json["pill"]["color"], "#d4c4ff");
    assert_eq!(json["chip"]["bg"], "rgba(167, 139, 250, 0.15)");
    assert_eq!(json["queue"]["open_border"], "rgba(167, 139, 250, 0.35)");
    assert_eq!(json["queue"]["open_status"], "#a78bfa");
    assert_eq!(json["queue"]["next_border"], "rgba(126, 184, 255, 0.55)");
    assert_eq!(json["queue"]["closed_opacity"], "0.55");
    let layers = json["layers"].as_array().expect("layers array");
    assert!(!layers.is_empty());
    for layer in layers {
        assert!(layer["id"].as_str().unwrap_or("").starts_with('L'));
        assert!(layer["color"].as_str().unwrap_or("").starts_with('#'));
    }
    let kinds = json["edge_kinds"].as_array().expect("edge kinds array");
    assert!(!kinds.is_empty());
    for kind in kinds {
        assert!(!kind["kind"].as_str().unwrap_or("").is_empty());
        assert!(kind["color"].as_str().unwrap_or("").starts_with('#'));
    }
}

#[tokio::test]
async fn vision_sprint_focus_svg_endpoint() {
    let (app, _state) = app();
    let (status, svg) = get_text(&app, "/api/vision/sprint-focus.svg").await;
    assert_eq!(status, StatusCode::OK);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("sprint focus:"));

    let (status, svg) = get_text(&app, "/api/vision/sprint-focus.svg?sprint=PH-S146").await;
    assert_eq!(status, StatusCode::OK);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("sprint focus: PH-S146"));
    assert!(svg.contains("#a78bfa"));
}

#[tokio::test]
async fn vision_palette_endpoint() {
    let (app, _state) = app();
    let (status, json) = get(&app, "/api/vision/palette").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert!(json["revision"].as_u64().unwrap_or(0) > 0);
    assert_eq!(json["bg_deep"], "#06080f");
    assert_eq!(json["bg"], "#0a0e18");
    assert_eq!(json["panel"], "rgba(18, 26, 42, 0.72)");
    assert_eq!(json["panel_solid"], "#141c2e");
    assert_eq!(json["border_bright"], "rgba(138, 180, 248, 0.45)");
    assert_eq!(json["accent"], "#7eb8ff");
    assert_eq!(json["accent_2"], "#c4a5ff");
    assert_eq!(json["glow"], "rgba(126, 184, 255, 0.35)");
    assert_eq!(json["sidebar_w"], "272px");
    assert_eq!(json["edge_docs"], "#90c490");
    assert_eq!(json["ext_rs"], "#f0883e");
    assert_eq!(json["sprint"], "#a78bfa");
    assert_eq!(json["bg_tone"], "0.8");
    assert_eq!(json["galaxy_bg_opacity"], "0.15");
    assert!(json["layers"].as_array().map(|a| a.len()).unwrap_or(0) >= 6);
    assert!(json["layers_dim"].as_array().map(|a| a.len()).unwrap_or(0) >= 6);
}

#[tokio::test]
async fn vision_starfield_svg_endpoint() {
    let (app, _state) = app();
    let (status, svg) = get_text(&app, "/api/vision/starfield.svg?mode=fx").await;
    assert_eq!(status, StatusCode::OK);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("starfield · FX · 160 stars"));
    assert!(svg.contains("rgba(126, 184, 255, 0.14)"));

    let (status, eco) = get_text(&app, "/api/vision/starfield.svg?mode=eco").await;
    assert_eq!(status, StatusCode::OK);
    assert!(eco.contains("starfield · Eco · 48 stars"));
    assert!(!eco.contains("rgba(126, 184, 255, 0.14)"));

    let (status, default) = get_text(&app, "/api/vision/starfield.svg").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        default.contains("starfield · FX · 160 stars"),
        "default → FX"
    );
}

#[tokio::test]
async fn vision_galaxy_svg_endpoint() {
    let (app, _state) = app();
    let (status, svg) = get_text(&app, "/api/vision/galaxy.svg").await;
    assert_eq!(status, StatusCode::OK);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("radialGradient"));
    assert!(svg.contains("galaxy backdrop"));
}

#[tokio::test]
async fn starfield_galaxy_svg_have_svg_content_type() {
    let (app, _state) = app();
    for path in [
        "/api/vision/starfield.svg?mode=fx",
        "/api/vision/galaxy.svg",
    ] {
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .method(Method::GET)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(res.status(), StatusCode::OK, "path {path}");
        let ct = res
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());
        assert!(
            ct.is_some_and(|v| v.contains("image/svg+xml")),
            "path {path} content-type: {:?}",
            ct
        );
    }
}
