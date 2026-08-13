//! GSV UI fragment contracts — Rust integration tests for `/api/ui/card/:name`.
//!
//! Server-rendered card bodies (band 120) must render HTML markers that match
//! the wire data, and unknown cards must 404. Uses `tower::ServiceExt::oneshot`
//! against the axum router (no port binding).

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use gsv::boxes::ui::{bar, esc, tab, CARD_NAMES};
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

async fn get_card(app: &axum::Router, name: &str) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/ui/card/{name}"))
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

#[tokio::test]
async fn ui_card_unknown_name_is_404() {
    let (app, _state) = app();
    let (status, json) = get_card(&app, "does-not-exist").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["ok"], false);
    assert!(json["error"].is_string());
}

#[tokio::test]
async fn ui_card_tracker_renders_table_markers() {
    let (app, _state) = app();
    let (status, json) = get_card(&app, "tracker").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert_eq!(json["card"], "tracker");
    let html = json["html"].as_str().expect("html");
    assert!(html.contains("<table><tr><th>kind</th><th>label</th><th>status</th><th>at</th></tr>"));
    assert!(html.contains("next <kbd>"));
}

#[tokio::test]
async fn ui_card_ratio_renders_band_or_missing_store() {
    let (app, _state) = app();
    let (status, json) = get_card(&app, "ratio").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    let html = json["html"].as_str().expect("html");
    // Without a stored rust_ratio.json the server renders the ok:false body
    // (missing-store message); with one it renders the band summary.
    if html.contains("missing rust_ratio.json") {
        assert!(html.contains("<span class='err'>"));
    } else {
        assert!(html.contains("Rust ratio"));
        assert!(html.contains("band min"));
    }
}

#[tokio::test]
async fn ui_card_all_registered_names_respond_ok() {
    let (app, _state) = app();
    for name in CARD_NAMES {
        let (status, json) = get_card(&app, name).await;
        assert_eq!(status, StatusCode::OK, "{name} status");
        assert_eq!(json["ok"], true, "{name} ok");
        assert!(json["html"].is_string(), "{name} html");
    }
}

#[tokio::test]
async fn ui_card_html_is_escaped_not_markup_injected() {
    let (app, _state) = app();
    let (status, json) = get_card(&app, "sli").await;
    assert_eq!(status, StatusCode::OK);
    let html = json["html"].as_str().expect("html");
    assert!(!html.contains("<script"));
}

#[test]
fn ui_helpers_match_js_semantics() {
    assert_eq!(esc("<x&y>"), "&lt;x&amp;y&gt;");
    assert!(tab(&["a"], Vec::new()).contains("<span class='dim'>—</span>"));
    assert!(bar(50.0).contains("width:50%"));
    assert!(bar(120.0).contains("width:100%"));
    assert_eq!(CARD_NAMES.len(), 20);
}

#[tokio::test]
async fn ui_card_omni_renders_summary_providers_models() {
    let (app, _state) = app();
    let (status, json) = get_card(&app, "omni").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    assert_eq!(json["card"], "omni");
    let html = json["html"].as_str().expect("html");
    assert!(html.contains("providers "), "summary: {html}");
    assert!(html.contains("default <kbd>"));
    assert!(html.contains("<summary>Providers ("));
    assert!(html.contains("<summary>Models ("));
    assert!(html.contains("<th>id</th><th>name</th><th>state</th><th>key</th><th>base_url</th>"));
}
