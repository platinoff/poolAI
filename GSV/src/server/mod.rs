//! GSV server — axum router, static UI, REST API boxes, SSE events.

use std::convert::Infallible;
use std::time::Duration;

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio_stream::wrappers::BroadcastStream;

use crate::boxes::ide::{IdeSelection, IdeWire};
use crate::boxes::preview::{resolve as preview_resolve, PreviewParams, PreviewWire};
use crate::boxes::terminal::{run as terminal_run, TerminalRequest, TerminalResponse};
use crate::boxes::update::UpdateCheckParams;
use crate::boxes::{hooks, sli, toolchain};
use crate::state::AppState;
use crate::tracker::{TrackerRecord, TrackerStore};
use crate::vision;

/// Embedded single-page UI (canon file: `GSV/ui/index.html`).
pub const INDEX_HTML: &str = include_str!("../../ui/index.html");

/// Ported vision diagram (`GSV/ui/vision.svg`), ratio-safe: `.svg` is audit-ignored.
pub const VISION_SVG: &str = include_str!("../../ui/vision.svg");

/// `/api/health` payload.
fn health(state: &AppState) -> Value {
    json!({
        "name": crate::GSV_SERVER_NAME,
        "version": *state.version,
        "ok": true,
        "uptime_secs": state.started_at.elapsed().map(|d| d.as_secs()).unwrap_or(0),
        "update_available": state.update_available(),
    })
}

/// Tracker wire.
fn tracker_wire(state: &AppState) -> Value {
    let tracker = state.tracker.clone();
    // Snapshot under read lock (no async needed — RwLock from tokio is Sync).
    json!({
        "sprints": tracker.try_read().map(|t| t.sprints().clone()).ok(),
        "records": tracker.try_read().map(|t| t.records().to_vec()).unwrap_or_default(),
        "generated_at": vision::rfc3339_now(),
    })
}

/// Build the full axum router with `AppState`.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/health", get(api_health))
        .route("/api/tracker", get(api_tracker))
        .route("/api/sli", get(api_sli))
        .route("/api/toolchain", get(api_toolchain))
        .route("/api/ide/sessions", get(api_ide_sessions))
        .route("/api/ide/select", post(api_ide_select))
        .route("/api/update", get(api_update))
        .route("/api/update/notify", post(api_update_notify))
        .route("/api/preview", get(api_preview))
        .route("/api/terminal", post(api_terminal))
        .route("/api/hooks/tests", get(api_hooks_tests))
        .route("/api/hooks/bench", get(api_hooks_bench))
        .route("/api/ratio", get(api_ratio))
        .route("/api/vision", get(api_vision))
        .route("/api/vision/manifest", get(api_vision_manifest))
        .route("/api/vision/map", get(api_vision_map))
        .route("/api/vision/feed", get(api_vision_feed))
        .route("/api/vision/sprint-map", get(api_vision_sprint_map))
        .route("/api/vision/doc-preview", get(api_vision_doc_preview))
        .route("/api/vision/node-search", get(api_vision_node_search))
        .route("/api/vision/sync", get(api_vision_sync))
        .route("/api/vision/extensions", get(api_vision_extensions))
        .route("/api/vision/sprint-queue", get(api_vision_sprint_queue))
        .route("/api/vision/sprint-board", get(api_vision_sprint_board))
        .route(
            "/api/vision/sprint-progress",
            get(api_vision_sprint_progress),
        )
        .route("/api/vision/speeds", get(api_vision_speeds))
        .route(
            "/api/vision/rust-diagnostics",
            get(api_vision_rust_diagnostics),
        )
        .route("/api/omni", get(api_omni))
        .route(
            "/api/omni/config",
            get(api_omni_config).post(api_omni_config_post),
        )
        .route("/api/omni/v1/models", get(api_omni_v1_models))
        .route("/api/omni/v1/chat/completions", post(api_omni_chat))
        .route("/api/omni/test", post(api_omni_test))
        .route("/events", get(events))
        .route("/assets/vision.svg", get(api_vision_svg))
        .with_state(state)
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn api_vision_svg() -> Response {
    (
        StatusCode::OK,
        [
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "no-cache"),
        ],
        VISION_SVG,
    )
        .into_response()
}

async fn api_health(State(state): State<AppState>) -> Json<Value> {
    Json(health(&state))
}

async fn api_tracker(State(state): State<AppState>) -> Json<Value> {
    Json(tracker_wire(&state))
}

async fn api_sli(State(state): State<AppState>) -> Json<Value> {
    Json(json!(sli::wire(&state.repo_root)))
}

async fn api_toolchain(State(state): State<AppState>) -> Json<Value> {
    Json(json!(toolchain::wire(&state.repo_root)))
}

async fn api_ide_sessions(State(state): State<AppState>) -> Json<Value> {
    let selection = state.ide_selection.try_read().ok().and_then(|s| s.clone());
    Json(json!(IdeWire {
        sessions: crate::boxes::ide::discover(),
        selection,
        generated_at: vision::rfc3339_now(),
    }))
}

#[derive(serde::Deserialize)]
struct IdeSelectBody {
    tool: String,
    session: String,
}

async fn api_ide_select(
    State(state): State<AppState>,
    Json(body): Json<IdeSelectBody>,
) -> Json<Value> {
    let selection = IdeSelection {
        tool: body.tool,
        session: body.session,
        selected_at: vision::rfc3339_now(),
    };
    if let Ok(mut sel) = state.ide_selection.try_write() {
        *sel = Some(selection.clone());
    }
    state.emit(format!(
        "event: ide_selected\ndata: {}",
        serde_json::to_string(&selection).unwrap_or_default()
    ));
    Json(json!({ "ok": true, "selection": selection }))
}

async fn api_update(
    State(state): State<AppState>,
    Query(_params): Query<UpdateCheckParams>,
) -> Json<Value> {
    Json(json!(crate::boxes::update::wire(&state)))
}

async fn api_update_notify(State(state): State<AppState>) -> Json<Value> {
    state
        .update_flag
        .store(true, std::sync::atomic::Ordering::SeqCst);
    state.emit("event: update_available\ndata: true".to_string());
    Json(json!({ "ok": true, "update_available": true }))
}

async fn api_preview(
    State(state): State<AppState>,
    Query(params): Query<PreviewParams>,
) -> Result<Json<PreviewWire>, (StatusCode, String)> {
    let path =
        preview_resolve(&state.repo_root, &params.file).map_err(|e| (StatusCode::NOT_FOUND, e))?;
    crate::boxes::preview::render(&path, &params.file)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn api_terminal(
    State(state): State<AppState>,
    Json(body): Json<TerminalRequest>,
) -> Json<TerminalResponse> {
    let resp = terminal_run(&body.command);
    let mut tracker = TrackerStore::load(&state.repo_root, &state.data_dir).unwrap_or_default();
    let status = if resp.allowed && resp.exit_code == Some(0) {
        "closed"
    } else if resp.allowed {
        "error"
    } else {
        "blocked"
    };
    let _ = tracker.push(
        &state.data_dir,
        TrackerRecord::new(
            "command",
            body.command,
            format!("exit={:?} ms={}", resp.exit_code, resp.duration_ms),
            status,
        ),
    );
    state.emit("event: terminal\ndata: done".to_string());
    Json(resp)
}

async fn api_hooks_tests(State(state): State<AppState>) -> Json<Value> {
    Json(json!(hooks::tests_wire(&state.repo_root)))
}

async fn api_hooks_bench(State(state): State<AppState>) -> Json<Value> {
    Json(json!(hooks::bench_wire(&state.repo_root)))
}

async fn api_ratio(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::ratio::wire(&state.data_dir))
}

async fn api_vision(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_summary(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_manifest(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_manifest(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_map(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_map(
        &state.repo_root,
        &state.data_dir,
    ))
}

#[derive(serde::Deserialize)]
struct VisionFeedParams {
    status: Option<String>,
}

async fn api_vision_feed(
    State(state): State<AppState>,
    Query(params): Query<VisionFeedParams>,
) -> Json<Value> {
    Json(crate::boxes::vision::wire_feed_filter(
        &state.repo_root,
        &state.data_dir,
        params.status.as_deref(),
    ))
}

async fn api_vision_sprint_map(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_sprint_map(
        &state.repo_root,
        &state.data_dir,
    ))
}

#[derive(serde::Deserialize)]
struct VisionDocPreviewParams {
    id: Option<String>,
}

async fn api_vision_doc_preview(
    State(state): State<AppState>,
    Query(params): Query<VisionDocPreviewParams>,
) -> Json<Value> {
    let id = params.id.unwrap_or_default();
    Json(crate::boxes::vision::wire_doc_preview(
        &state.repo_root,
        &state.data_dir,
        &id,
    ))
}

async fn api_vision_sync(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_sync(
        &state.repo_root,
        &state.data_dir,
    ))
}

#[derive(serde::Deserialize)]
struct VisionNodeSearchParams {
    q: Option<String>,
    layer: Option<String>,
}

async fn api_vision_node_search(
    State(state): State<AppState>,
    Query(params): Query<VisionNodeSearchParams>,
) -> Json<Value> {
    Json(crate::boxes::vision::wire_node_search(
        &state.repo_root,
        &state.data_dir,
        params.q.as_deref().unwrap_or_default(),
        params.layer.as_deref(),
    ))
}

async fn api_vision_extensions(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_extensions(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_sprint_queue(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_sprint_queue(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_sprint_board(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_sprint_board(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_sprint_progress(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_sprint_progress(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_speeds(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_speed_index(
        &state.repo_root,
        &state.data_dir,
    ))
}

async fn api_vision_rust_diagnostics(State(state): State<AppState>) -> Json<Value> {
    Json(crate::boxes::vision::wire_rust_diagnostics(
        &state.repo_root,
        &state.data_dir,
    ))
}

// ── OmniRouter box ─────────────────────────────────────────────────────────────

async fn api_omni(State(state): State<AppState>) -> Json<Value> {
    Json(json!(crate::boxes::omni::wire(&state.omni).await))
}

async fn api_omni_config(State(state): State<AppState>) -> Json<Value> {
    Json(state.omni.config.read().await.redacted())
}

async fn api_omni_config_post(
    State(state): State<AppState>,
    Json(patch): Json<Value>,
) -> Json<Value> {
    let applied = {
        let mut cfg = state.omni.config.write().await;
        cfg.apply(&patch)
    };
    match applied {
        Ok(()) => {
            state.omni.persist();
            state.emit("event: omni_config\ndata: changed".to_string());
            Json(json!({
                "ok": true,
                "config": state.omni.config.read().await.redacted(),
            }))
        }
        Err(msg) => Json(json!({
            "ok": false,
            "error": msg,
        })),
    }
}

async fn api_omni_v1_models(State(state): State<AppState>) -> Response {
    crate::boxes::omni::proxy::v1_models(&state.omni)
        .await
        .unwrap_or_else(api_error_response)
}

async fn api_omni_chat(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    crate::boxes::omni::proxy::chat_completions(&state.omni, &headers, &body)
        .await
        .unwrap_or_else(api_error_response)
}

async fn api_omni_test(State(state): State<AppState>, Json(body): Json<Value>) -> Json<Value> {
    let provider = body
        .get("provider")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if provider.is_empty() {
        return Json(json!({ "ok": false, "error": "provider required" }));
    }
    match crate::boxes::omni::proxy::test_provider(&state.omni, provider).await {
        Ok(res) => Json(json!({ "ok": true, "result": res })),
        Err(e) => Json(json!({ "ok": false, "error": e.message() })),
    }
}

/// Convert an `AppError` into a 400 JSON response (route/config errors).
fn api_error_response(err: crate::AppError) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": { "code": "OMNI_ERROR", "message": err.message() } })),
    )
        .into_response()
}

/// Server-Sent Events stream: broadcasts state events + periodic keepalive.
async fn events(
    State(state): State<AppState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.events.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|item| async move {
        match item {
            Ok(payload) => Some(Ok(Event::default().data(payload))),
            Err(_) => None, // lagged behind — skip, client reconnects
        }
    });
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    )
}
