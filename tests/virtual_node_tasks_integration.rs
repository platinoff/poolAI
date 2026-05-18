//! FM-016 phase 3: virtual-node task poll/complete + RAID health from worker path.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use chrono::Utc;
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::raid::protocol::{ArtifactMetadata, ProtocolMessage, PutArtifactPayload, SyncMode};
use poolai::services::virtual_node_task_service::VirtualNodeTaskService;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;
use uuid::Uuid;

async fn app_with_raid_and_discovery() -> (Router, TempDir) {
    let temp = TempDir::new().unwrap();
    let raid_config = poolai::raid::RaidConfig {
        mode: poolai::raid::RaidMode::Local,
        base_path: temp.path().to_path_buf(),
        quota_bytes: None,
        retention_days: None,
        gc_on_startup: false,
    };
    let manager = Arc::new(poolai::raid::RaidManager::new(raid_config));
    manager.initialize().await.unwrap();

    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18080)),
        None,
    ));

    let ctx = ApiContext::default();
    ctx.attach_raid_manager_for_test(manager).expect("raid");
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }

    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx);
    (app, temp)
}

#[tokio::test]
async fn virtual_node_task_poll_complete_cycle() {
    let peer = "vn-task-peer";
    VirtualNodeTaskService::clear_peer(peer);

    let app = app_with_raid_and_discovery().await.0;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "peer_id": "{peer}",
                "address": "127.0.0.1",
                "port": 19091,
                "metadata": {{ "channel": "telegram", "role": "virtual_node" }}
            }}"#
        )))
        .unwrap();
    let reg = app.clone().oneshot(register).await.unwrap();
    assert_eq!(reg.status(), StatusCode::OK);

    let poll = Request::builder()
        .uri(format!("/api/v1/virtual-nodes/{peer}/tasks/poll"))
        .body(Body::empty())
        .unwrap();
    let poll_res = app.clone().oneshot(poll).await.unwrap();
    assert_eq!(poll_res.status(), StatusCode::OK);
    let poll_body = to_bytes(poll_res.into_body(), usize::MAX).await.unwrap();
    let poll_v: Value = serde_json::from_slice(&poll_body).unwrap();
    let task = poll_v["task"].as_object().expect("bootstrap ping task");
    assert_eq!(task["task_type"], "ping");

    let task_id = task["id"].as_str().unwrap();
    let complete = Request::builder()
        .method("POST")
        .uri(format!(
            "/api/v1/virtual-nodes/{peer}/tasks/{task_id}/complete"
        ))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"status":"ok"}"#))
        .unwrap();
    let done = app.clone().oneshot(complete).await.unwrap();
    assert_eq!(done.status(), StatusCode::OK);

    let status_req = Request::builder()
        .uri(format!("/api/v1/virtual-nodes/{peer}/tasks/status"))
        .body(Body::empty())
        .unwrap();
    let status_res = app.oneshot(status_req).await.unwrap();
    let status_body = to_bytes(status_res.into_body(), usize::MAX).await.unwrap();
    let status_v: Value = serde_json::from_slice(&status_body).unwrap();
    assert_eq!(status_v["completed"], 1);

    VirtualNodeTaskService::clear_peer(peer);
}

#[tokio::test]
async fn raid_health_check_wire_from_virtual_node_task() {
    let peer = "vn-raid-peer";
    VirtualNodeTaskService::clear_peer(peer);
    VirtualNodeTaskService::enqueue(peer, "raid_health_check", Value::Null);

    let (app, _temp) = app_with_raid_and_discovery().await;

    let msg = ProtocolMessage::health_check(peer.to_string());
    let body = serde_json::to_vec(&msg).unwrap();
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/health")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn raid_artifact_probe_wire_from_virtual_node_task() {
    let peer = "vn-artifact-peer";
    VirtualNodeTaskService::clear_peer(peer);
    VirtualNodeTaskService::enqueue(
        peer,
        "raid_artifact_probe",
        serde_json::json!({ "name": "integration-probe" }),
    );

    let (app, _temp) = app_with_raid_and_discovery().await;

    let payload = PutArtifactPayload {
        artifact_id: format!("artifact-{}", Uuid::new_v4()),
        source_node: peer.to_string(),
        data: Some(B64.encode(b"vn-wire-probe")),
        metadata: ArtifactMetadata {
            name: "integration-probe".to_string(),
            version: "probe-1".to_string(),
            size_bytes: 13,
            checksum: "probe".to_string(),
            created_at: Utc::now(),
            content_type: None,
            tags: None,
        },
        replication_factor: 1,
        sync_mode: SyncMode::Async,
    };
    let msg = ProtocolMessage::put_artifact(peer.to_string(), payload).unwrap();
    let body = serde_json::to_vec(&msg).unwrap();
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/raid/distributed/artifacts/replicate")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
