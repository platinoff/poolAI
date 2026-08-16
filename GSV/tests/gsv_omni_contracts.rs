//! GSV OmniRouter API contracts — Rust integration tests.
//!
//! Scope: `/api/omni*` box endpoints (catalog, redacted config, OpenAI-compatible
//! models + chat completions dry-run, connectivity test). Chat proxying is tested
//! with `X-Omni-Dry-Run: 1` so no network is ever hit; config is written to a
//! temp data dir so the durable store is untouched.

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{header, HeaderValue, Request, StatusCode};
use gsv::server::router;
use gsv::AppState;
use serde_json::{json, Value};
use tokio::sync::broadcast;
use tower::ServiceExt;

fn temp_data_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("gsv-omni-{tag}-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn app(data_dir: PathBuf) -> axum::Router {
    let (tx, _rx) = broadcast::channel(64);
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("GSV parent")
        .to_path_buf();
    let state = AppState::new(Some(repo_root), Some(data_dir), tx);
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

async fn post(app: &axum::Router, path: &str, body: Value) -> (StatusCode, Value) {
    let res = app
        .clone()
        .oneshot(
            Request::post(path)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .expect("req"),
        )
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

async fn post_headers(
    app: &axum::Router,
    path: &str,
    body: Value,
    hdrs: &[(&str, &str)],
) -> (StatusCode, Value) {
    let mut builder = Request::post(path).header(header::CONTENT_TYPE, "application/json");
    for (k, v) in hdrs {
        builder = builder.header(*k, HeaderValue::from_str(v).expect("hdr"));
    }
    let res = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).expect("req"))
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

#[tokio::test]
async fn omni_overview_returns_catalog_and_recommended() {
    let (app, dir) = {
        let d = temp_data_dir("overview");
        (app(d.clone()), d)
    };
    let (status, json) = get(&app, "/api/omni").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["name"], "OmniRouter");
    assert!(json["providers"]
        .as_array()
        .map(|a| a.len() >= 10)
        .unwrap_or(false));
    assert!(json["models"]
        .as_array()
        .map(|a| a.len() >= 20)
        .unwrap_or(false));
    let rec = json["recommended"].as_array().expect("recommended");
    assert_eq!(rec.len(), 6);
    assert!(json["routing"]["auto"] == true);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_config_read_is_redacted() {
    let (app, dir) = {
        let d = temp_data_dir("config-read");
        (app(d.clone()), d)
    };
    let (status, json) = get(&app, "/api/omni/config").await;
    assert_eq!(status, StatusCode::OK);
    let openai = &json["provider"]["openai"];
    assert!(openai["key_set"].is_boolean());
    assert!(openai["enabled"].is_boolean());
    assert!(openai["base_url"]
        .as_str()
        .unwrap_or_default()
        .contains("api.openai.com"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_config_post_tunes_provider_and_persists() {
    let dir = temp_data_dir("config-post");
    let app = app(dir.clone());
    let (status, json) = post(
        &app,
        "/api/omni/config",
        json!({
            "provider": {
                "openai": { "base_url": "http://127.0.0.1:20128/v1", "api_key": "sk-test-xyz" }
            }
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["ok"], true);
    // Redacted config now reflects the tuning, without the raw key.
    assert_eq!(json["config"]["provider"]["openai"]["key_set"], true);
    assert_eq!(
        json["config"]["provider"]["openai"]["base_url"],
        "http://127.0.0.1:20128/v1"
    );
    let raw = json!(&json);
    assert!(!raw.to_string().contains("sk-test-xyz"));
    // Durable file was written.
    assert!(dir.join("omni.toml").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_v1_models_is_openai_shaped() {
    let (app, dir) = {
        let d = temp_data_dir("v1-models");
        (app(d.clone()), d)
    };
    let (status, json) = get(&app, "/api/omni/v1/models").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["object"], "list");
    let data = json["data"].as_array().expect("data");
    let ids: Vec<&str> = data.iter().filter_map(|m| m["id"].as_str()).collect();
    assert!(ids.contains(&"gpt-5.2"));
    assert!(ids.contains(&"gemini-3-pro"));
    let gpt = data.iter().find(|m| m["id"] == "gpt-5.2").expect("gpt");
    assert_eq!(gpt["owned_by"], "openai");
    assert_eq!(gpt["context_window"], 400_000);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_chat_dry_run_resolves_route_without_network() {
    let dir = temp_data_dir("chat-dry");
    let app = app(dir.clone());
    let _ = post(
        &app,
        "/api/omni/config",
        json!({ "provider": { "openai": { "base_url": "http://127.0.0.1:20128/v1", "api_key": "k" } } }),
    )
    .await;
    let (status, json) = post_headers(
        &app,
        "/api/omni/v1/chat/completions",
        json!({
            "model": "gpt-5.2",
            "messages": [{ "role": "user", "content": "hi" }]
        }),
        &[("x-omni-dry-run", "1"), ("x-omni-provider", "openai")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["dry_run"], true);
    assert_eq!(json["provider"], "openai");
    assert_eq!(json["model"], "gpt-5.2");
    assert_eq!(
        json["upstream"],
        "http://127.0.0.1:20128/v1/chat/completions"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_chat_rejects_unknown_provider() {
    let (app, dir) = {
        let d = temp_data_dir("chat-bad");
        (app(d.clone()), d)
    };
    let (status, json) = post_headers(
        &app,
        "/api/omni/v1/chat/completions",
        json!({ "model": "gpt-5.2", "messages": [] }),
        &[("x-omni-provider", "nope")],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["ok"], false);
    assert!(json["error"]
        .as_str()
        .unwrap_or_default()
        .contains("unknown provider"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_chat_auto_routes_model_owner() {
    let dir = temp_data_dir("chat-auto");
    let app = app(dir.clone());
    let (status, json) = post_headers(
        &app,
        "/api/omni/v1/chat/completions",
        json!({
            "model": "claude-sonnet-4.5",
            "messages": [{ "role": "user", "content": "hi" }]
        }),
        &[("x-omni-dry-run", "1")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["provider"], "anthropic");
    assert_eq!(json["model"], "claude-sonnet-4.5");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn omni_test_requires_provider() {
    let (app, dir) = {
        let d = temp_data_dir("test-empty");
        (app(d.clone()), d)
    };
    let (status, json) = post(&app, "/api/omni/test", json!({})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["ok"], false);
    assert!(json["error"]
        .as_str()
        .unwrap_or_default()
        .contains("provider required"));
    let _ = std::fs::remove_dir_all(&dir);
}
