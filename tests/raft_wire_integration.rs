//! PH-S04: HTTP wire path for Raft cluster status (`GET /api/v1/raid/status`).
//!
//! Run: `cargo test -j 1 --features raft,test-utils --test raft_wire_integration -- --test-threads=1`

#![cfg(feature = "raft")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::{ApiContext, AppState};
use poolai::network::api::raid::create_raid_routes;
use poolai::raid::raft::{RaftConfig, RaidRaftNode};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use serde_json::Value;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;
use tower::ServiceExt;

async fn build_api_context(temp: &TempDir, with_raft: bool) -> ApiContext {
    let config = RaidConfig {
        mode: RaidMode::Local,
        base_path: temp.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = Arc::new(RaidManager::new(config));
    manager.initialize().await.unwrap();

    let state: ApiContext = Arc::new(AppState::new());
    state
        .attach_raid_manager_for_test(manager.clone())
        .expect("attach raid");

    if with_raft {
        let raid_manager = Arc::new(RwLock::new(RaidManager::new(RaidConfig {
            mode: RaidMode::Local,
            base_path: temp.path().join("raft-data"),
            quota_bytes: None,
            retention_days: None,
            gc_on_startup: false,
        })));
        raid_manager.write().await.initialize().await.unwrap();

        let raft_config = RaftConfig {
            node_id: 1,
            cluster_members: vec![1],
            election_timeout: 1000,
            heartbeat_interval: 100,
        };
        let raft_node =
            RaidRaftNode::new(raft_config, raid_manager, temp.path().join("raft-node")).unwrap();
        raft_node.initialize().await.unwrap();
        let _ = raft_node.wait_for_leader(3000).await;

        state
            .attach_raft_node_for_test(Arc::new(raft_node))
            .expect("attach raft");
    }

    state
}

async fn get_raid_status(app: &Router) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/raid/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    (status, body)
}

#[tokio::test]
async fn wire_raid_status_omits_raft_when_node_not_attached() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp, false).await;
    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);

    let (status, body) = get_raid_status(&app).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.get("raft_status").is_none() || body["raft_status"].is_null());
    assert!(body["cluster_status"].is_string());
    assert!(body["mode"].is_string());
}

#[tokio::test]
async fn wire_raft_status_includes_role_and_term_when_node_attached() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp, true).await;
    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);

    let (status, body) = get_raid_status(&app).await;
    assert_eq!(status, StatusCode::OK);

    let raft = body
        .get("raft_status")
        .expect("raft_status should be present when node attached");
    assert!(raft["role"].is_string());
    assert!(raft["term"].is_u64() || raft["term"].is_number());
    let role = raft["role"].as_str().unwrap();
    assert!(
        role == "Leader" || role == "Follower" || role == "Candidate",
        "unexpected raft role: {role}"
    );
}
