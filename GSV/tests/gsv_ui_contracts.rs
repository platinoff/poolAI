//! GSV UI fragment contracts — Rust integration tests for `/api/ui/card/:name`.
//!
//! Server-rendered card bodies (band 120) must render HTML markers that match
//! the wire data, and unknown cards must 404. Uses `tower::ServiceExt::oneshot`
//! against the axum router (no port binding).

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use gsv::boxes::ui::{bar, esc, render_card, tab, CARD_NAMES};
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

/// Fetch the served single-page UI HTML (`/`) as text.
async fn get_index_html(app: &axum::Router) -> String {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("body");
    String::from_utf8(bytes.to_vec()).expect("utf8")
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

/// The 13 rustCards the thin JS glue fetches via `getText` (mirror of
/// `rustCards` in `GSV/ui/index.html`).
const RUST_CARDS: [&str; 13] = [
    "tracker",
    "sli",
    "toolchain",
    "ratio",
    "hooks-tests",
    "hooks-bench",
    "sprint-map",
    "sprint-queue",
    "sprint-progress",
    "sprint-board",
    "speed-index",
    "rust-diagnostics",
    "omni",
];

#[test]
fn card_renderers_error_state_contract() {
    // Every rustCard renderer must turn `ok:false` into the canonical error
    // marker `<span class='err'>…</span>` — never a raw panic or blank body.
    let err_wire = serde_json::json!({ "ok": false, "error": "stand-error" });
    for name in RUST_CARDS {
        let html = render_card(name, &err_wire).expect(name);
        assert!(
            html.contains("<span class='err'>stand-error</span>"),
            "{name} error marker: {html}"
        );
    }
}

#[test]
fn card_renderers_empty_state_contract() {
    // Empty-table renderers must emit the canonical empty marker
    // `<div class='dim'>… — no data</div>` instead of a blank body.
    let tracker = render_card(
        "tracker",
        &serde_json::json!({
            "sprints": { "open": [], "closed": [], "total": 0, "next": "" },
            "records": []
        }),
    )
    .expect("tracker");
    assert!(tracker.contains("tracker records — no data"), "{tracker}");

    let sli = render_card(
        "sli",
        &serde_json::json!({
            "catalog": { "used_count": 0, "unused_count": 0, "entries": [] }
        }),
    )
    .expect("sli");
    assert!(sli.contains("sli entries — no data"), "{sli}");

    let toolchain =
        render_card("toolchain", &serde_json::json!({ "entries": [] })).expect("toolchain");
    assert!(
        toolchain.contains("toolchain entries — no data"),
        "{toolchain}"
    );

    let ratio = render_card(
        "ratio",
        &serde_json::json!({
            "ok": true, "meets_min_ratio": true, "rust_ratio_pct": 96.0,
            "formal_band_min": 0.95, "rust_loc": 1, "non_rust_product_loc": 0,
            "product_loc_total": 1, "by_category": {}
        }),
    )
    .expect("ratio");
    assert!(ratio.contains("ratio by-category — no data"), "{ratio}");
}

/// A11y contract: the served UI HTML carries axe-friendly markers — lang,
/// `role="status"` live regions, `aria-live`, `aria-label`s, image `alt`.
#[tokio::test]
async fn ui_index_a11y_markers_present() {
    let (app, _state) = app();
    let html = get_index_html(&app).await;
    assert!(html.contains("lang=\"uk\""), "html lang");
    assert!(html.contains("role=\"status\""), "live status regions");
    assert!(html.contains("aria-live=\"polite\""), "aria-live polite");
    assert!(html.contains("aria-haspopup=\"true\""), "power menu popup");
    assert!(html.contains("role=\"menu\""), "power menu role");
    assert!(
        html.contains("aria-label=\"Vision controls\""),
        "controls label"
    );
    assert!(html.contains("aria-label=\"GPU mode"), "gpu aria-label");
    // JS-created card action buttons carry aria-labels in the script source.
    assert!(
        html.contains("setAttribute(\"aria-label\", \"Minimize / Restore card\")"),
        "min btn label"
    );
    assert!(
        html.contains("setAttribute(\"aria-label\", \"Fullscreen toggle card\")"),
        "max btn label"
    );
    // Every decorative/visual `<img>` has an `alt` attribute.
    assert!(
        html.contains("alt=\"\"") || html.contains("alt=''"),
        "empty alt for decorative images"
    );
    assert!(html.contains("alt=\"speed history\""), "speed chart alt");
    assert!(
        html.contains("alt=\"rust diagnostics history\""),
        "diag chart alt"
    );
    assert!(html.contains("alt=\"sprint focus map\""), "focus map alt");
}

/// A11y contract: cards expose `aria-live` region so updates announce.
#[tokio::test]
async fn ui_index_vision_sync_status_is_live() {
    let (app, _state) = app();
    let html = get_index_html(&app).await;
    assert!(html.contains(
        "id=\"b-vision-sync-status\" class=\"dim\" role=\"status\" aria-live=\"polite\""
    ));
}

/// Offline-stability contract: every rustCard has a `data-card` hook and the
/// `getText` glue keeps the last-good render on fetch failure (badge, no wipe).
#[tokio::test]
async fn ui_index_cards_are_offline_stable() {
    let (app, _state) = app();
    let html = get_index_html(&app).await;
    for name in RUST_CARDS {
        assert!(
            html.contains(&format!("data-card=\"{name}\"")),
            "{name} data-card hook"
        );
    }
    // Badge element + keep-last-good path exist in the served script.
    assert!(html.contains(".card-status"), "card-status css");
    assert!(html.contains("markStatus"), "status badge helper");
    assert!(
        html.contains("keep the last-good render"),
        "offline-stable path in getText"
    );
    assert!(html.contains("innerHTML === \"…\""), "no-wipe guard");
}
