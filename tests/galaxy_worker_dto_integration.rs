//! PH-S516: unified Galaxy worker DTO capabilities + seed_inventory on virtual-nodes list.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

fn test_discovery() -> Arc<DiscoveryService> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18083));
    Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        addr,
        None,
    ))
}

async fn app_with_discovery() -> Router {
    let discovery = test_discovery();
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

#[tokio::test]
async fn virtual_nodes_galaxy_dto_capabilities_seed_inventory_ph_s516() {
    let app = app_with_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "peer_id": "dto-worker-1",
                "address": "10.0.0.5",
                "port": 9095,
                "capabilities": {
                    "cpu_cores": 8,
                    "memory_mb": 16384,
                    "gpu_devices": [0],
                    "supports_tensor_parallelism": false,
                    "supports_pipeline_parallelism": false
                },
                "metadata": {
                    "role": "virtual_node",
                    "origin": "cloud",
                    "seed_inventory": "{\"shard_ids\":[\"shard-a\"],\"hot_tier\":{\"ram_bytes_used\":1000,\"vram_bytes_used\":0,\"profiles\":[]},\"local_replica_regions\":[\"eu-west\"]}"
                }
            }"#,
        ))
        .unwrap();

    let response = app.clone().oneshot(register).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list = Request::builder()
        .uri("/api/v1/discovery/virtual-nodes")
        .body(Body::empty())
        .unwrap();
    let list_resp = app.oneshot(list).await.unwrap();
    assert_eq!(list_resp.status(), StatusCode::OK);
    let body = to_bytes(list_resp.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let node = v["nodes"]
        .as_array()
        .and_then(|a| a.iter().find(|n| n["peer"]["peer_id"] == "dto-worker-1"))
        .expect("node");
    assert_eq!(node["galaxy"]["origin"], "cloud");
    assert_eq!(node["galaxy"]["capabilities"]["memory_mb"], 16384);
    assert_eq!(node["galaxy"]["seed_inventory"]["shard_ids"][0], "shard-a");
}
