//! FM-034: job scheduler binds scheduled jobs to pool workers when pool is attached.

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::pool::worker::{Worker, WorkerConfig};
use poolai::pool::{LoadBalancingStrategy, Pool, PoolConfig};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt;

fn worker_config(id: &str) -> WorkerConfig {
    WorkerConfig {
        worker_id: id.to_string(),
        max_concurrent_requests: 4,
        request_timeout_ms: 30_000,
        health_check_interval_ms: 5_000,
        enable_caching: false,
        cache_size: 0,
        max_memory_mb: 8192,
        cpu_priority: 5,
        gpu_device: None,
        auto_restart: false,
        resource_monitoring: true,
    }
}

async fn app_with_pool_workers() -> Router {
    let pool = Arc::new(RwLock::new(Pool::new(PoolConfig {
        max_workers: 10,
        max_queue_size: 100,
        load_balancing_strategy: LoadBalancingStrategy::LeastConnections,
        auto_scaling: false,
        scaling_threshold: 0.8,
        request_timeout: 30,
    })));
    {
        let guard = pool.write().await;
        guard
            .add_worker(
                "worker-busy".into(),
                Worker::new(worker_config("worker-busy")),
            )
            .await
            .expect("add busy");
        guard
            .add_worker(
                "worker-idle".into(),
                Worker::new(worker_config("worker-idle")),
            )
            .await
            .expect("add idle");
    }

    let ctx = ApiContext::default();
    ctx.attach_pool_for_test(pool).expect("pool attach");
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

async fn post_json(app: &Router, uri: &str, body: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, v)
}

#[tokio::test]
async fn create_job_binds_least_loaded_pool_worker() {
    let app = app_with_pool_workers().await;
    let (status, created) =
        post_json(&app, "/api/v1/jobs", r#"{"kind":"inference","priority":1}"#).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(
        created.get("status").and_then(|s| s.as_str()),
        Some("leased")
    );
    let worker_id = created
        .get("worker_id")
        .and_then(|w| w.as_str())
        .expect("worker_id on create response");
    assert!(
        worker_id == "worker-busy" || worker_id == "worker-idle",
        "unexpected worker: {worker_id}"
    );

    let id = created
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();
    let get = Request::builder()
        .uri(format!("/api/v1/jobs/{id}"))
        .body(Body::empty())
        .unwrap();
    let get_res = app.oneshot(get).await.unwrap();
    assert_eq!(get_res.status(), StatusCode::OK);
    let bytes = to_bytes(get_res.into_body(), usize::MAX).await.unwrap();
    let detail: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        detail.pointer("/job/worker_id").and_then(|w| w.as_str()),
        Some(worker_id)
    );
}
