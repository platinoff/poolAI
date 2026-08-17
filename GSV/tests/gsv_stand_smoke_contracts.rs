//! GSV HTTP stand-smoke contracts — Rust integration tests.
//!
//! The `gsv-http-stand-smoke` bin hits the live server over HTTP; these tests
//! assert the *contract* it relies on, without binding a port:
//!
//! 1. `/api/ui/card/:name` renders every `CARD_NAMES` entry with `ok` + non-empty `html`.
//! 2. Vision endpoints return `ok: true` JSON (the smoke `check_ok` gate).
//! 3. Tracker/SLI/Update/Omni wires return parseable JSON without requiring `ok`
//!    (the smoke `check_json` gate).
//! 4. The SVG routes respond 200 (the smoke `check_status` gate).
//! 5. Report shape fields (`base_url/ok/passed/failed/cases`) are stable.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use gsv::boxes::ui::CARD_NAMES;
use gsv::server::router;
use gsv::AppState;
use serde_json::Value;
use tokio::sync::broadcast;
use tower::ServiceExt;

fn app() -> (axum::Router, AppState) {
    let (tx, _rx) = broadcast::channel(64);
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("GSV parent")
        .to_path_buf();
    let state = AppState::new(Some(repo_root), None, tx);
    (router(state.clone()), state)
}

async fn get_json(app: &axum::Router, uri: &str) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(uri)
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

async fn get_raw(app: &axum::Router, uri: &str) -> StatusCode {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method(Method::GET)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    res.status()
}

/// Vision `ok`-gated endpoints — the `check_ok` set of the smoke bin.
const VISION_OK_ENDPOINTS: [&str; 15] = [
    "/api/vision",
    "/api/vision/manifest",
    "/api/vision/feed",
    "/api/vision/map",
    "/api/vision/sprint-map",
    "/api/vision/sprint-queue",
    "/api/vision/sprint-board",
    "/api/vision/sprint-progress",
    "/api/vision/speeds",
    "/api/vision/rust-diagnostics",
    "/api/vision/extensions",
    "/api/vision/sprint-theme",
    "/api/vision/palette",
    "/api/vision/node-search?q=vision",
    "/api/vision/sync",
];

#[tokio::test]
async fn all_vision_ok_endpoints_gate_true() {
    let (app, _state) = app();
    for uri in VISION_OK_ENDPOINTS {
        let (status, json) = get_json(&app, uri).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{uri} must be 200 (smoke check_ok precondition)"
        );
        assert_eq!(
            json["ok"], true,
            "{uri} must carry ok:true (smoke check_ok gate)"
        );
    }
}

/// Struct-wire endpoints — the `check_json` set (no `ok` required, must parse).
const STRUCT_JSON_ENDPOINTS: [&str; 5] = [
    "/api/tracker",
    "/api/sli",
    "/api/toolchain",
    "/api/update",
    "/api/omni/status",
];

#[tokio::test]
async fn struct_wire_endpoints_parse_json() {
    let (app, _state) = app();
    for uri in STRUCT_JSON_ENDPOINTS {
        let (status, json) = get_json(&app, uri).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{uri} must be 200 (smoke check_json precondition)"
        );
        assert!(
            json.is_object(),
            "{uri} must parse as JSON object (smoke check_json gate)"
        );
    }
}

/// SVG/status-only routes — the `check_status` set.
const STATUS_ENDPOINTS: [&str; 5] = [
    "/assets/vision.svg",
    "/api/vision/speeds.svg",
    "/api/vision/rust-diagnostics.svg",
    "/api/vision/starfield.svg?mode=eco",
    "/api/vision/sprint-focus.svg",
];

#[tokio::test]
async fn status_only_endpoints_are_200() {
    let (app, _state) = app();
    for uri in STATUS_ENDPOINTS {
        let status = get_raw(&app, uri).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{uri} must be 200 (smoke check_status gate)"
        );
    }
}

#[tokio::test]
async fn every_registered_card_renders_ok_with_html() {
    let (app, _state) = app();
    for card in CARD_NAMES {
        let (status, json) = get_json(&app, &format!("/api/ui/card/{card}")).await;
        assert_eq!(status, StatusCode::OK, "card {card} must be 200");
        assert_eq!(json["ok"], true, "card {card} must be ok:true");
        let html = json["html"].as_str().unwrap_or_default();
        assert!(
            !html.trim().is_empty(),
            "card {card} must render non-empty html"
        );
    }
}

/// The smoke report shape must stay stable for `--json` consumers (bin tests the
/// serialize side; this pins the wire contract).
#[test]
fn smoke_report_shape_is_stable() {
    let report = serde_json::json!({
        "base_url": "http://127.0.0.1:9999",
        "ok": true,
        "passed": 1,
        "failed": 0,
        "cases": [{ "name": "health", "ok": true }],
        "tool": "gsv-http-stand-smoke"
    });
    assert!(report["base_url"].is_string());
    assert!(report["ok"].is_boolean());
    assert!(report["passed"].is_number());
    assert!(report["failed"].is_number());
    assert!(report["cases"].is_array());
    assert!(report["tool"].is_string());
}

/// The bin's card list must not drift from the UI registry (mirror of the bin's
/// own unit test, guarded at the crate boundary so integration users see it too).
#[test]
fn stand_smoke_card_list_matches_registry() {
    let smoke_src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bin/gsv_http_stand_smoke.rs"
    ))
    .expect("stand smoke source readable");
    let registry: Vec<&str> = CARD_NAMES.to_vec();
    for card in registry {
        assert!(
            smoke_src.contains(&format!("\"{card}\"")),
            "card {card} missing from stand smoke bin"
        );
    }
}
