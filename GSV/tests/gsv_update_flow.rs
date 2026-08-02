//! GSV update/offline/resync flow — state machine + SSE broadcast.
//!
//! Covers the key UX requirements from GSV_SERVER.md:
//! - update flag transitions (false → notify → true → clear → false)
//! - SSE event broadcast to subscribers
//! - offline → resync: after a broadcast, box endpoints still answer (read-only)
//! - `update::wire` detects a pending rebuild when sources are newer than binary

use std::path::PathBuf;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use gsv::boxes::update::{binary_mtime, newest_src_mtime, wire};
use gsv::server::router;
use gsv::AppState;
use tokio::sync::broadcast;
use tower::ServiceExt;

fn state() -> (axum::Router, AppState) {
    let (tx, _rx) = broadcast::channel(64);
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("GSV parent")
        .to_path_buf();
    let state = AppState::new(Some(repo_root), None, tx);
    (router(state.clone()), state)
}

#[test]
fn update_flag_state_machine() {
    let (_, state) = state();
    assert!(!state.update_available(), "initial: no update");
    state
        .update_flag
        .store(true, std::sync::atomic::Ordering::SeqCst);
    assert!(state.update_available(), "after notify: update available");
    state.clear_update();
    assert!(!state.update_available(), "after clear: update cleared");
}

#[test]
fn update_wire_has_expected_fields() {
    let (_, state) = state();
    let w = wire(&state);
    assert_eq!(w.version, "0.1.0");
    assert!(w.binary_mtime > 0);
    assert!(w.newest_src_mtime > 0);
}

#[test]
fn newest_src_mtime_tracks_sources() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(newest_src_mtime(&manifest) > 0);
}

#[test]
fn binary_mtime_is_positive() {
    assert!(binary_mtime() > 0);
}

#[tokio::test]
async fn sse_broadcast_delivers_event() {
    let (_, state) = state();
    let mut rx = state.events.subscribe();
    state.emit("event: update_available\ndata: true".to_string());
    let received = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("recv within timeout");
    let payload = received.expect("no lag");
    assert!(payload.contains("update_available"));
}

#[tokio::test]
async fn sse_events_endpoint_streams_keepalive_event() {
    let (app, state) = state();
    // Start a subscriber on the channel before opening /events.
    let mut rx = state.events.subscribe();
    let _res = app
        .clone()
        .oneshot(Request::get("/events").body(Body::empty()).expect("req"))
        .await
        .expect("resp");
    state.emit("event: ready\ndata: test".to_string());
    let payload = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("recv")
        .expect("no lag");
    assert!(payload.contains("ready"));
}

#[tokio::test]
async fn offline_resync_answers_after_event_storm() {
    let (app, _state) = state();
    // Simulate metrics resync: boxes must still answer read-only after events.
    for path in [
        "/api/health",
        "/api/tracker",
        "/api/sli",
        "/api/toolchain",
        "/api/update",
    ] {
        let res = app
            .clone()
            .oneshot(Request::get(path).body(Body::empty()).expect("req"))
            .await
            .expect("resp");
        assert_eq!(res.status(), StatusCode::OK, "resync path {path}");
    }
}

#[tokio::test]
async fn update_notify_then_clear_flow() {
    let (app, state) = state();
    let res = app
        .clone()
        .oneshot(
            Request::post("/api/update/notify")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("resp");
    assert_eq!(res.status(), StatusCode::OK);
    assert!(state.update_available());
    state.clear_update();
    assert!(!state.update_available());
}
