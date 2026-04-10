//! P2b harness: HTTP wire path for distributed `PutArtifact` (single-node stand-in for a peer).
//!
//! Run: `cargo test -j 1 --features test-utils --test distributed_raid_wire_integration`
//! With TQ01 size check: add `--features ml` (TQ01 test is `#[cfg(feature = "ml")]`).
//! Covers: PutArtifact, SyncArtifacts catalog diff (FM-007), LeaveCluster membership (FM-008).

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::Utc;
use poolai::core::state::{ApiContext, AppState};
use poolai::network::api::raid::create_raid_routes;
use poolai::raid::protocol::{
    ArtifactMetadata, LeaveClusterPayload, LeaveReason, ProtocolMessage, PutArtifactPayload,
    SyncArtifactsPayload, SyncArtifactsResponse, SyncDirection, SyncMode,
};
use poolai::raid::{RaidConfig, RaidManager, RaidMode};
use poolai::services::raid_service::RaidService;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;
use uuid::Uuid;

/// Extra UUID present on the peer but not stored locally (Push → missing on this node).
const REMOTE_ONLY_ID: &str = "00000000-0000-0000-0000-000000000099";

async fn build_api_context(temp: &TempDir) -> ApiContext {
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
        .attach_raid_manager_for_test(manager)
        .expect("attach raid");
    state
}

fn put_artifact_message(data_b64: Option<String>, logical_name: &str) -> ProtocolMessage {
    let data_len = data_b64
        .as_ref()
        .map(|s| B64.decode(s).map(|v| v.len()).unwrap_or(0))
        .unwrap_or(0) as u64;
    let metadata = ArtifactMetadata {
        name: logical_name.to_string(),
        version: "1.0.0".to_string(),
        size_bytes: data_len,
        checksum: "test-checksum".to_string(),
        created_at: Utc::now(),
        content_type: Some("application/octet-stream".to_string()),
        tags: None,
    };
    let payload = PutArtifactPayload {
        artifact_id: format!("artifact-{}", Uuid::new_v4()),
        source_node: "stand-node-a".to_string(),
        data: data_b64,
        metadata,
        replication_factor: 1,
        sync_mode: SyncMode::Async,
    };
    ProtocolMessage::put_artifact("stand-node-b".to_string(), payload).unwrap()
}

#[tokio::test]
async fn wire_put_artifact_round_trip_over_http_json() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx.clone());

    let payload_bytes = b"wire-replication-stand-payload".to_vec();
    let msg = put_artifact_message(Some(B64.encode(&payload_bytes)), "stand-model");
    let body = serde_json::to_vec(&msg).unwrap();
    let wire_len = body.len();

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/artifacts/replicate")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert!(
        wire_len > payload_bytes.len(),
        "JSON envelope should add overhead"
    );

    let artifacts = RaidService::list_artifacts(&ctx).await.unwrap();
    assert!(
        artifacts.iter().any(|a| a.name.contains("stand-model")),
        "artifact should be stored via distributed handler; got {:?}",
        artifacts
    );
}

#[tokio::test]
async fn wire_sync_artifacts_push_reports_peer_only_ids_as_missing() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;
    let art = RaidService::put_artifact(&ctx, "sync-lib-a", b"x")
        .await
        .unwrap();
    let local_id = art.id.to_string();

    let payload = SyncArtifactsPayload {
        last_sync_timestamp: None,
        artifact_ids: Some(vec![local_id.clone(), REMOTE_ONLY_ID.to_string()]),
        remote_versions: None,
        direction: SyncDirection::Push,
    };
    let msg = ProtocolMessage::sync_artifacts("stand-sync-node".to_string(), payload).unwrap();
    let body = serde_json::to_vec(&msg).unwrap();

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/artifacts/sync")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let resp_msg: ProtocolMessage = serde_json::from_slice(&bytes).unwrap();
    let inner: SyncArtifactsResponse = serde_json::from_value(resp_msg.payload).unwrap();
    assert_eq!(inner.synced_count, 1);
    assert_eq!(inner.missing_artifacts, vec![REMOTE_ONLY_ID.to_string()]);
    assert!(inner.conflicts.is_empty());
}

#[tokio::test]
async fn wire_sync_artifacts_reports_conflict_when_remote_version_differs() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;
    let art = RaidService::put_artifact(&ctx, "sync-conflict-a", b"x")
        .await
        .unwrap();
    let local_id = art.id.to_string();

    let mut remote_versions = HashMap::new();
    remote_versions.insert(
        local_id.clone(),
        art.stored_at - chrono::Duration::seconds(5),
    );
    let payload = SyncArtifactsPayload {
        last_sync_timestamp: None,
        artifact_ids: Some(vec![local_id.clone()]),
        remote_versions: Some(remote_versions),
        direction: SyncDirection::Bidirectional,
    };
    let msg = ProtocolMessage::sync_artifacts("stand-sync-node".to_string(), payload).unwrap();
    let body = serde_json::to_vec(&msg).unwrap();

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/artifacts/sync")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let resp_msg: ProtocolMessage = serde_json::from_slice(&bytes).unwrap();
    let inner: SyncArtifactsResponse = serde_json::from_value(resp_msg.payload).unwrap();
    assert_eq!(inner.synced_count, 1);
    assert!(inner.missing_artifacts.is_empty());
    assert_eq!(inner.conflicts.len(), 1);
    assert_eq!(inner.conflicts[0].artifact_id, local_id);
    assert_eq!(inner.conflicts[0].reason, "local_newer_than_remote");
}

#[tokio::test]
async fn wire_leave_cluster_rejects_unknown_node_when_membership_non_empty() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;
    let mgr = ctx.raid_manager.get().expect("raid attached");
    let _registered = mgr.register_node("10.0.0.1:9000".to_string()).await;
    let stranger = Uuid::new_v4();

    let msg = ProtocolMessage::leave_cluster(
        stranger.to_string(),
        LeaveClusterPayload {
            reason: LeaveReason::Shutdown,
            graceful: false,
        },
    )
    .unwrap();
    let body = serde_json::to_vec(&msg).unwrap();

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/cluster/leave")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn wire_leave_cluster_ok_for_registered_node() {
    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;
    let mgr = ctx.raid_manager.get().expect("raid attached");
    let registered = mgr.register_node("10.0.0.2:9001".to_string()).await;

    let msg = ProtocolMessage::leave_cluster(
        registered.id.to_string(),
        LeaveClusterPayload {
            reason: LeaveReason::Shutdown,
            graceful: false,
        },
    )
    .unwrap();
    let body = serde_json::to_vec(&msg).unwrap();

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx);
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/cluster/leave")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[cfg(feature = "ml")]
async fn wire_put_artifact_tq01_base64_smaller_than_raw_f32_base64() {
    use poolai::ml::turboquant::pack_uniform_rows;

    let temp = TempDir::new().unwrap();
    let ctx = build_api_context(&temp).await;

    let app = Router::new()
        .nest("/api/v1", create_raid_routes())
        .with_state(ctx.clone());

    let rows: Vec<Vec<f32>> = (0..64)
        .map(|r| (0..256).map(|c| ((r * 256 + c) as f32) * 0.001).collect())
        .collect();
    let packed = pack_uniform_rows(&rows).expect("pack TQ01");
    assert!(
        packed.bytes_out < packed.bytes_in,
        "TQ01 should shrink uniform float matrix (in {} out {})",
        packed.bytes_in,
        packed.bytes_out
    );

    let mut raw_le = Vec::new();
    for r in &rows {
        for f in r {
            raw_le.extend_from_slice(&f.to_le_bytes());
        }
    }
    assert!(raw_le.len() as u64 > packed.bytes_out);

    let msg_tq = put_artifact_message(Some(B64.encode(&packed.bytes)), "tq01-weights");
    let json_tq = serde_json::to_vec(&msg_tq).unwrap();

    let msg_raw = put_artifact_message(Some(B64.encode(&raw_le)), "raw-weights");
    let json_raw = serde_json::to_vec(&msg_raw).unwrap();

    assert!(
        json_tq.len() < json_raw.len(),
        "stand-in wire JSON with TQ01 payload should be smaller than raw f32 payload (tq {} vs raw {})",
        json_tq.len(),
        json_raw.len()
    );

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/artifacts/replicate")
        .header("content-type", "application/json")
        .body(Body::from(json_tq))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
