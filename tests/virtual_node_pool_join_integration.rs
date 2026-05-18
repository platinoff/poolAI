//! FM-016+++: virtual node pool join without auth (peer must be in discovery).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::pool::{LoadBalancingStrategy, Pool, PoolConfig};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt;

async fn app_with_pool_and_discovery() -> Router {
    let pool = Arc::new(RwLock::new(Pool::new(PoolConfig {
        max_workers: 10,
        max_queue_size: 100,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        auto_scaling: false,
        scaling_threshold: 0.8,
        request_timeout: 30,
    })));

    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18081)),
        None,
    ));

    let ctx = ApiContext::default();
    ctx.attach_pool_for_test(pool).expect("pool");
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }

    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

#[tokio::test]
async fn virtual_node_pool_join_registers_worker() {
    let peer = "vn-pool-peer";
    let app = app_with_pool_and_discovery().await;

    let register = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/register-remote")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "peer_id": "{peer}",
                "address": "127.0.0.1",
                "port": 19093,
                "metadata": {{ "role": "virtual_node", "channel": "telegram" }}
            }}"#
        )))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(register).await.unwrap().status(),
        StatusCode::OK
    );

    let join = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/virtual-nodes/{peer}/pool/join"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"max_concurrent_requests":4}"#))
        .unwrap();
    let join_res = app.clone().oneshot(join).await.unwrap();
    assert_eq!(join_res.status(), StatusCode::CREATED);

    let workers = Request::builder()
        .uri("/api/v1/workers")
        .body(Body::empty())
        .unwrap();
    let workers_res = app.oneshot(workers).await.unwrap();
    assert_eq!(workers_res.status(), StatusCode::OK);
    let body = to_bytes(workers_res.into_body(), usize::MAX).await.unwrap();
    let list: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(
        list.iter().any(|w| w["id"] == peer),
        "pool should list joined virtual node, got {list:?}"
    );
}
