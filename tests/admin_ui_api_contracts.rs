//! JSON shape checks for admin dashboard and RAID admin HTTP API.
//! Enterprise dashboard slices are tested when built with `--features enterprise`.
//! FM-013/014/015 + **FM-040**: admin UI field audit — keys expected by `src/ui/admin/*.rs`.
//! Manifest: `docs/development/ADMIN_UI_FIELD_AUDIT_2026-05-23.md`.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use serde_json::Value;
use tower::ServiceExt;

fn assert_structured_error(v: &Value) {
    assert!(
        v.get("error").is_some(),
        "expected structured JSON error: {v:?}"
    );
}

#[tokio::test]
async fn admin_overview_includes_dashboard_keys() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/overview")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("overview JSON");
    let o = v.as_object().expect("overview object");
    for key in [
        "status",
        "uptime_seconds",
        "version",
        "workers",
        "workers_total",
        "workers_registered",
        "vm_instances",
        "cpu_usage_percent",
        "memory_usage_mb",
        "subsystems",
    ] {
        assert!(o.contains_key(key), "admin overview missing `{key}`: {o:?}");
    }
}

#[tokio::test]
async fn raid_admin_status_json_shape() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/admin/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("raid admin status JSON");

    if status == StatusCode::OK {
        let st = v
            .get("status")
            .and_then(|x| x.as_object())
            .expect("`status` object when 200");
        for key in ["mode", "initialized", "active", "rebalancing_enabled"] {
            assert!(
                st.contains_key(key),
                "raid strategy status missing `{key}`: {st:?}"
            );
        }
    } else {
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_structured_error(&v);
    }
}

#[tokio::test]
async fn workers_list_json_shape_for_admin() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/workers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("workers JSON");
    let arr = v.as_array().expect("workers array");
    assert!(!arr.is_empty(), "mock workers list should be non-empty");
    if let Some(first) = arr.first() {
        let o = first.as_object().expect("worker object");
        for key in [
            "id",
            "status",
            "current_task",
            "is_healthy",
            "total_requests_processed",
            "queue_size",
            "active_connections",
            "average_response_time_ms",
        ] {
            assert!(o.contains_key(key), "worker missing `{key}`: {o:?}");
        }
    }
}

#[tokio::test]
async fn libraries_list_unavailable_has_structured_error() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/libraries")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("libraries error JSON");
    assert_structured_error(&v);
}

#[tokio::test]
async fn topology_overview_unavailable_has_structured_error() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/topology")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("topology error JSON");
    assert_structured_error(&v);
}

#[tokio::test]
async fn config_get_json_shape_for_admin() {
    use poolai::core::config::{initialize_config, PoolAIConfig};
    let _ = initialize_config(PoolAIConfig::default());

    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/config")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("config JSON");
    let o = v.as_object().expect("config object");
    for key in [
        "system",
        "gpu",
        "pool",
        "monitoring",
        "version",
        "health",
        "https",
    ] {
        assert!(o.contains_key(key), "config missing `{key}`: {o:?}");
    }
    let system = o
        .get("system")
        .and_then(|s| s.as_object())
        .expect("config.system object");
    for key in ["name", "log_level", "max_workers", "queue_size"] {
        assert!(
            system.contains_key(key),
            "config.system missing `{key}`: {system:?}"
        );
    }
}

#[tokio::test]
async fn users_list_json_shape_for_admin() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("users JSON");
    let arr = v.as_array().expect("users array");
    assert!(!arr.is_empty(), "seeded users list should be non-empty");
    if let Some(first) = arr.first() {
        let o = first.as_object().expect("user object");
        for key in ["id", "username", "role", "active", "created_at"] {
            assert!(o.contains_key(key), "user missing `{key}`: {o:?}");
        }
    }
}

#[tokio::test]
async fn instance_list_unavailable_has_structured_error() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/instance")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("instance list error JSON");
    assert_structured_error(&v);
}

#[tokio::test]
async fn raid_artifacts_list_unavailable_has_structured_error() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/artifacts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("raid artifacts error JSON");
    assert_structured_error(&v);
}

#[tokio::test]
async fn vm_instances_unavailable_has_structured_error() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/vm/instances")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("vm instances error JSON");
    assert_structured_error(&v);
}

#[cfg(feature = "test-utils")]
mod attached_managers {
    use super::*;
    use poolai::core::state::AppState;
    use poolai::libs::LibraryManager;
    use poolai::pool::topology::TopologyManager;
    use poolai::raid::{RaidConfig, RaidManager, RaidMode};
    use poolai::runtime::instance::{InstanceManager, InstancePlacement, PlacementStrategy};
    use poolai::vm::{VmIsolation, VmManager, VmResources};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::sync::RwLock as TokioRwLock;

    #[tokio::test]
    async fn libraries_list_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        state
            .attach_library_manager_for_test(Arc::new(TokioRwLock::new(LibraryManager::new())))
            .expect("attach library manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/libraries")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("libraries JSON");
        let arr = v.as_array().expect("libraries array");
        if let Some(first) = arr.first() {
            let o = first.as_object().expect("library object");
            for key in ["name", "version", "metadata"] {
                assert!(o.contains_key(key), "library missing `{key}`: {o:?}");
            }
            let meta = o
                .get("metadata")
                .expect("metadata present")
                .as_object()
                .expect("metadata object for admin libs UI");
            if meta.contains_key("installed_at") {
                assert!(
                    meta.get("installed_at").and_then(|x| x.as_str()).is_some(),
                    "metadata.installed_at should be string when present: {meta:?}"
                );
            }
        }
    }

    #[tokio::test]
    async fn topology_nodes_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        let topology = Arc::new(TokioRwLock::new(TopologyManager::new(None)));
        {
            let guard = topology.read().await;
            guard.test_add_node("audit-node-a", "127.0.0.1:1").await;
            guard.test_add_node("audit-node-b", "127.0.0.1:2").await;
        }
        state
            .attach_topology_manager_for_test(topology)
            .expect("attach topology manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/topology/nodes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("topology nodes JSON");
        let o = v.as_object().expect("topology nodes object");
        let nodes = o
            .get("nodes")
            .and_then(|x| x.as_object())
            .expect("topology nodes map");
        assert!(!nodes.is_empty(), "expected seeded topology nodes");
        let node = nodes
            .get("audit-node-a")
            .and_then(|x| x.as_object())
            .expect("audit-node-a entry");
        for key in [
            "available_gpu_memory_mb",
            "total_gpu_memory_mb",
            "available_cpu_cores",
            "total_cpu_cores",
            "current_load",
        ] {
            assert!(
                node.contains_key(key),
                "topology node resource missing `{key}`: {node:?}"
            );
        }
    }

    #[tokio::test]
    async fn topology_overview_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        state
            .attach_topology_manager_for_test(Arc::new(TokioRwLock::new(TopologyManager::new(
                None,
            ))))
            .expect("attach topology manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/topology")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("topology JSON");
        let o = v.as_object().expect("topology object");
        for key in [
            "node_count",
            "latency_measurements",
            "last_updated",
            "node_ids",
        ] {
            assert!(o.contains_key(key), "topology missing `{key}`: {o:?}");
        }
    }

    #[tokio::test]
    async fn instance_list_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        state
            .attach_instance_manager_for_test(Arc::new(TokioRwLock::new(InstanceManager::new())))
            .expect("attach instance manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/instance")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("instance list JSON");
        let o = v.as_object().expect("instance list object");
        assert!(
            o.get("instances").and_then(|x| x.as_array()).is_some(),
            "instance list missing `instances` array: {o:?}"
        );
        if let Some(first) = o
            .get("instances")
            .and_then(|a| a.as_array())
            .and_then(|a| a.first())
        {
            let inst = first.as_object().expect("instance object");
            for key in [
                "instance_id",
                "model_id",
                "status",
                "created_at",
                "placement",
            ] {
                assert!(inst.contains_key(key), "instance missing `{key}`: {inst:?}");
            }
            let placement = inst
                .get("placement")
                .and_then(|p| p.as_object())
                .expect("placement object");
            for key in ["strategy", "node_ids", "memory_by_node", "memory_delta"] {
                assert!(
                    placement.contains_key(key),
                    "placement missing `{key}`: {placement:?}"
                );
            }
        }
    }

    #[tokio::test]
    async fn instance_previews_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        state
            .attach_instance_manager_for_test(Arc::new(TokioRwLock::new(InstanceManager::new())))
            .expect("attach instance manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/instance/previews?model_id=audit-preview-model")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("instance previews JSON");
        let previews = v
            .get("previews")
            .and_then(|x| x.as_array())
            .expect("previews array");
        assert!(
            !previews.is_empty(),
            "expected at least one placement preview"
        );
        let row = previews[0].as_object().expect("preview object");
        for key in ["model_id", "sharding", "memory_delta_by_node"] {
            assert!(row.contains_key(key), "preview missing `{key}`: {row:?}");
        }
    }

    #[tokio::test]
    async fn instance_get_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        let manager = Arc::new(TokioRwLock::new(InstanceManager::new()));
        let placement = InstancePlacement {
            strategy: PlacementStrategy::Single,
            node_ids: vec!["local".into()],
            memory_by_node: HashMap::from([("local".into(), 1024)]),
            memory_delta: 1024,
            error: None,
        };
        let instance_id = manager
            .write()
            .await
            .create_instance("audit-instance-model".into(), placement, HashMap::new())
            .await
            .expect("create instance");
        state
            .attach_instance_manager_for_test(manager)
            .expect("attach instance manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/instance/{instance_id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let inst = serde_json::from_slice::<Value>(&body).expect("instance JSON");
        let o = inst.as_object().expect("instance object");
        for key in [
            "instance_id",
            "model_id",
            "status",
            "created_at",
            "placement",
        ] {
            assert!(o.contains_key(key), "instance GET missing `{key}`: {o:?}");
        }
        let placement = o
            .get("placement")
            .and_then(|p| p.as_object())
            .expect("placement object");
        assert!(
            placement.get("strategy").and_then(|s| s.as_str()).is_some(),
            "placement.strategy string for admin modal: {placement:?}"
        );
        assert!(
            placement
                .get("node_ids")
                .and_then(|n| n.as_array())
                .is_some(),
            "placement.node_ids for admin modal"
        );
    }

    #[tokio::test]
    async fn topology_latency_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        let topology = Arc::new(TokioRwLock::new(TopologyManager::new(None)));
        {
            let guard = topology.read().await;
            guard.test_add_node("lat-a", "127.0.0.1:1").await;
            guard.test_add_node("lat-b", "127.0.0.1:2").await;
            guard.test_update_latency("lat-a", "lat-b", 15.0).await;
        }
        state
            .attach_topology_manager_for_test(topology)
            .expect("attach topology manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/topology/latency")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("topology latency JSON");
        let matrix = v
            .get("latency_matrix")
            .and_then(|x| x.as_object())
            .expect("latency_matrix object for admin topology UI");
        assert!(
            !matrix.is_empty(),
            "expected seeded latency_matrix entries: {matrix:?}"
        );
    }

    #[tokio::test]
    async fn topology_node_detail_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        let topology = Arc::new(TokioRwLock::new(TopologyManager::new(None)));
        {
            let guard = topology.read().await;
            guard.test_add_node("detail-node", "127.0.0.1:9").await;
        }
        state
            .attach_topology_manager_for_test(topology)
            .expect("attach topology manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/topology/nodes/detail-node")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let node = serde_json::from_slice::<Value>(&body).expect("node detail JSON");
        let o = node.as_object().expect("node object");
        for key in [
            "node_id",
            "available_gpu_memory_mb",
            "total_gpu_memory_mb",
            "available_cpu_cores",
            "total_cpu_cores",
            "current_load",
        ] {
            assert!(o.contains_key(key), "node detail missing `{key}`: {o:?}");
        }
    }

    #[tokio::test]
    async fn raid_cluster_status_json_shape_when_manager_attached() {
        let temp = TempDir::new().expect("tempdir");
        let config = RaidConfig {
            mode: RaidMode::Local,
            base_path: temp.path().to_path_buf(),
            quota_bytes: None,
            retention_days: None,
            gc_on_startup: false,
        };
        let manager = Arc::new(RaidManager::new(config));
        manager.initialize().await.expect("raid init");

        let state = Arc::new(AppState::default());
        state
            .attach_raid_manager_for_test(manager)
            .expect("attach raid manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/raid/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("raid cluster status JSON");
        let o = v.as_object().expect("raid status object");
        for key in [
            "cluster_status",
            "node_count",
            "artifact_count",
            "mode",
            "storage",
        ] {
            assert!(o.contains_key(key), "raid status missing `{key}`: {o:?}");
        }
        let storage = o
            .get("storage")
            .and_then(|x| x.as_object())
            .expect("storage");
        assert!(storage.contains_key("total_size_bytes"));
    }

    #[tokio::test]
    async fn raid_artifacts_list_json_shape_when_manager_attached() {
        let temp = TempDir::new().expect("tempdir");
        let config = RaidConfig {
            mode: RaidMode::Local,
            base_path: temp.path().to_path_buf(),
            quota_bytes: None,
            retention_days: None,
            gc_on_startup: false,
        };
        let manager = Arc::new(RaidManager::new(config));
        manager.initialize().await.expect("raid init");

        let state = Arc::new(AppState::default());
        state
            .attach_raid_manager_for_test(manager)
            .expect("attach raid manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/raid/artifacts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("raid artifacts JSON");
        let arr = v.as_array().expect("raid artifacts array");
        if let Some(first) = arr.first() {
            let o = first.as_object().expect("artifact object");
            for key in ["id", "name", "stored_at", "path"] {
                assert!(o.contains_key(key), "artifact missing `{key}`: {o:?}");
            }
        }
    }

    #[tokio::test]
    async fn vm_instances_json_shape_when_manager_attached() {
        let state = Arc::new(AppState::default());
        let manager = Arc::new(VmManager::new());
        manager.initialize().await.expect("vm init");
        manager
            .create_instance(
                "admin-contract-vm".to_string(),
                VmResources {
                    cpu_cores: 1,
                    memory_mb: 512,
                    gpu_required: false,
                    gpu_scheduling_policy: None,
                },
                VmIsolation::ProcessSandbox,
            )
            .await
            .expect("create vm");
        state
            .attach_vm_manager_for_test(manager)
            .expect("attach vm manager");
        let app = Router::new()
            .nest("/api/v1", create_api_routes())
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/vm/instances")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("vm instances JSON");
        let arr = v.as_array().expect("vm instances array");
        assert!(!arr.is_empty(), "expected at least one VM instance");
        let o = arr[0].as_object().expect("vm instance object");
        for key in ["id", "name", "status", "resources"] {
            assert!(o.contains_key(key), "vm instance missing `{key}`: {o:?}");
        }
        let resources = o
            .get("resources")
            .and_then(|r| r.as_object())
            .expect("resources object");
        for key in ["cpu_cores", "memory_mb"] {
            assert!(
                resources.contains_key(key),
                "vm resources missing `{key}`: {resources:?}"
            );
        }
    }
}

#[tokio::test]
async fn raid_admin_burst_metrics_json_shape() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/admin/metrics/burst")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("burst metrics JSON");

    if status == StatusCode::OK {
        let m = v
            .get("metrics")
            .and_then(|x| x.as_object())
            .expect("`metrics` object when 200");
        for key in [
            "total_artifacts",
            "artifacts_in_burst",
            "total_requests",
            "burst_threshold_rps",
            "base_replication_factor",
            "max_replication_factor",
        ] {
            assert!(m.contains_key(key), "burst metrics missing `{key}`: {m:?}");
        }
    } else {
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn raid_admin_smallworld_metrics_json_shape() {
    let app = Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/raid/admin/metrics/smallworld")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).expect("smallworld metrics JSON");

    if status == StatusCode::OK {
        let m = v
            .get("metrics")
            .and_then(|x| x.as_object())
            .expect("`metrics` object when 200");
        for key in [
            "total_artifacts",
            "total_nodes",
            "avg_clustering_coefficient",
            "target_clustering_coefficient",
            "base_replication_factor",
        ] {
            assert!(
                m.contains_key(key),
                "smallworld metrics missing `{key}`: {m:?}"
            );
        }
    } else {
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_structured_error(&v);
    }
}

#[cfg(feature = "enterprise")]
mod enterprise_admin_contract_slices {
    use super::*;
    use chrono::Utc;
    use poolai::enterprise::monitoring::{AlertRule, AlertSeverity, Dashboard, MetricDataPoint};
    use poolai::enterprise::multi_tenancy::TenantConfig;
    use poolai::enterprise::security::{OAuth2Config, SamlConfig, SecurityPolicy};
    use poolai::network::enterprise_api::create_enterprise_api_routes;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn enterprise_monitoring_alerts_json_shape() {
        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ApiContext::default());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/monitoring/alerts?limit=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("alerts JSON");
        let arr = v.as_array().expect("alerts response is array");
        if let Some(first) = arr.first() {
            let o = first.as_object().expect("alert object");
            for key in [
                "id",
                "rule_name",
                "metric",
                "current_value",
                "threshold",
                "severity",
                "triggered_at",
                "acknowledged",
            ] {
                assert!(o.contains_key(key), "alert missing `{key}`: {o:?}");
            }
        }
    }

    #[tokio::test]
    async fn enterprise_tenants_list_json_shape() {
        let ctx = ApiContext::default();
        ctx.tenant_manager.initialize().await.unwrap();
        ctx.tenant_manager
            .create_tenant("admin-contract-tenant".to_string(), TenantConfig::default())
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/tenants")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("tenants JSON");
        let arr = v.as_array().expect("tenants response is array");
        assert!(!arr.is_empty(), "seeded tenant list should be non-empty");
        let o = arr[0].as_object().expect("tenant object");
        for key in ["id", "name", "config", "usage", "created_at", "updated_at"] {
            assert!(o.contains_key(key), "tenant missing `{key}`: {o:?}");
        }
        let config = o.get("config").and_then(|x| x.as_object()).expect("config");
        for key in ["active", "max_workers", "max_memory_mb"] {
            assert!(
                config.contains_key(key),
                "tenant.config missing `{key}`: {config:?}"
            );
        }
        let usage = o.get("usage").and_then(|x| x.as_object()).expect("usage");
        for key in ["workers", "memory_mb", "cpu_cores"] {
            assert!(
                usage.contains_key(key),
                "tenant.usage missing `{key}`: {usage:?}"
            );
        }
    }

    #[tokio::test]
    async fn enterprise_oauth2_providers_json_shape() {
        let ctx = ApiContext::default();
        ctx.security_manager.initialize().await.unwrap();
        ctx.security_manager
            .register_oauth2_provider(
                "admin-contract-oauth2".to_string(),
                OAuth2Config {
                    client_id: "contract-client".to_string(),
                    client_secret: "secret".to_string(),
                    authorization_url: "https://oauth.example.com/authorize".to_string(),
                    token_url: "https://oauth.example.com/token".to_string(),
                    redirect_uri: "https://poolai.example.com/callback".to_string(),
                    scopes: vec!["openid".to_string()],
                    telegram_allow_user_ids: vec![],
                },
            )
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/security/oauth2/providers")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("oauth2 providers JSON");
        let arr = v.as_array().expect("oauth2 providers response is array");
        assert!(!arr.is_empty(), "seeded oauth2 list should be non-empty");
        let o = arr[0].as_object().expect("oauth2 provider object");
        for key in ["name", "config", "enabled"] {
            assert!(
                o.contains_key(key),
                "oauth2 provider missing `{key}`: {o:?}"
            );
        }
        let config = o.get("config").and_then(|x| x.as_object()).expect("config");
        for key in [
            "client_id",
            "authorization_url",
            "token_url",
            "redirect_uri",
        ] {
            assert!(
                config.contains_key(key),
                "oauth2 config missing `{key}`: {config:?}"
            );
        }
    }

    #[tokio::test]
    async fn enterprise_monitoring_dashboards_json_shape() {
        let ctx = ApiContext::default();
        ctx.enterprise_monitoring_manager
            .initialize()
            .await
            .unwrap();
        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name: "admin-contract-dashboard".to_string(),
            description: "contract test".to_string(),
            metrics: vec!["cpu_usage".to_string()],
            layout: "{}".to_string(),
            is_public: false,
            tenant_id: None,
            created_at: Utc::now(),
        };
        ctx.enterprise_monitoring_manager
            .create_dashboard(dashboard)
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/monitoring/dashboards")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("dashboards JSON");
        let arr = v.as_array().expect("dashboards response is array");
        assert!(
            !arr.is_empty(),
            "seeded dashboards list should be non-empty"
        );
        let o = arr[0].as_object().expect("dashboard object");
        for key in [
            "id",
            "name",
            "description",
            "metrics",
            "is_public",
            "created_at",
        ] {
            assert!(o.contains_key(key), "dashboard missing `{key}`: {o:?}");
        }
    }

    #[tokio::test]
    async fn enterprise_monitoring_metrics_json_shape() {
        let ctx = ApiContext::default();
        ctx.enterprise_monitoring_manager
            .initialize()
            .await
            .unwrap();
        ctx.enterprise_monitoring_manager
            .record_metric(MetricDataPoint {
                timestamp: Utc::now(),
                metric: "cpu_usage".to_string(),
                value: 42.5,
                tags: HashMap::new(),
                tenant_id: None,
            })
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/monitoring/metrics?metric=cpu_usage&limit=10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("metrics JSON");
        let arr = v.as_array().expect("metrics response is array");
        assert!(!arr.is_empty(), "seeded metrics list should be non-empty");
        let o = arr[0].as_object().expect("metric data point object");
        for key in ["timestamp", "metric", "value"] {
            assert!(o.contains_key(key), "metric point missing `{key}`: {o:?}");
        }
        assert_eq!(o.get("metric").and_then(|x| x.as_str()), Some("cpu_usage"));
    }

    #[tokio::test]
    async fn enterprise_monitoring_alert_rules_json_shape() {
        let ctx = ApiContext::default();
        ctx.enterprise_monitoring_manager
            .initialize()
            .await
            .unwrap();
        ctx.enterprise_monitoring_manager
            .create_alert_rule(AlertRule {
                name: "admin-contract-rule".to_string(),
                metric: "memory_usage".to_string(),
                threshold: 80.0,
                operator: ">".to_string(),
                severity: AlertSeverity::Warning,
                enabled: true,
                tenant_id: None,
            })
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/monitoring/alert-rules")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("alert rules JSON");
        let arr = v.as_array().expect("alert rules response is array");
        assert!(
            !arr.is_empty(),
            "seeded alert rules list should be non-empty"
        );
        let o = arr[0].as_object().expect("alert rule object");
        for key in [
            "name",
            "metric",
            "threshold",
            "operator",
            "severity",
            "enabled",
        ] {
            assert!(o.contains_key(key), "alert rule missing `{key}`: {o:?}");
        }
    }

    #[tokio::test]
    async fn enterprise_saml_providers_json_shape() {
        let ctx = ApiContext::default();
        ctx.security_manager.initialize().await.unwrap();
        ctx.security_manager
            .register_saml_provider(
                "admin-contract-saml".to_string(),
                SamlConfig {
                    entity_id: "https://idp.example.com/entity".to_string(),
                    sso_url: "https://idp.example.com/sso".to_string(),
                    acs_url: None,
                    slo_url: None,
                    certificate: "-----BEGIN CERTIFICATE-----\nTEST\n-----END CERTIFICATE-----"
                        .to_string(),
                    attribute_mapping: HashMap::new(),
                },
            )
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/security/saml/providers")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("saml providers JSON");
        let arr = v.as_array().expect("saml providers response is array");
        assert!(!arr.is_empty(), "seeded saml list should be non-empty");
        let o = arr[0].as_object().expect("saml provider object");
        for key in ["name", "config", "enabled"] {
            assert!(o.contains_key(key), "saml provider missing `{key}`: {o:?}");
        }
        let config = o.get("config").and_then(|x| x.as_object()).expect("config");
        for key in ["entity_id", "sso_url", "certificate"] {
            assert!(
                config.contains_key(key),
                "saml config missing `{key}`: {config:?}"
            );
        }
    }

    #[tokio::test]
    async fn enterprise_security_policies_json_shape() {
        let ctx = ApiContext::default();
        ctx.security_manager.initialize().await.unwrap();
        ctx.security_manager
            .create_security_policy(SecurityPolicy {
                name: "admin-contract-policy".to_string(),
                description: "contract test policy".to_string(),
                allowed_ip_ranges: vec![],
                require_mfa: true,
                session_timeout: 3600,
                max_failed_attempts: 5,
            })
            .await
            .unwrap();

        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/security/policies")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("security policies JSON");
        let arr = v.as_array().expect("security policies response is array");
        assert!(
            !arr.is_empty(),
            "seeded security policies list should be non-empty"
        );
        let o = arr[0].as_object().expect("security policy object");
        for key in [
            "name",
            "description",
            "require_mfa",
            "session_timeout",
            "max_failed_attempts",
        ] {
            assert!(
                o.contains_key(key),
                "security policy missing `{key}`: {o:?}"
            );
        }
    }

    #[tokio::test]
    async fn enterprise_audit_events_json_shape() {
        let app = Router::new()
            .nest("/api/enterprise", create_enterprise_api_routes())
            .with_state(ApiContext::default());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/enterprise/audit/events?limit=10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let v: Value = serde_json::from_slice(&body).expect("audit JSON");
        let arr = v.as_array().expect("audit events response is array");
        if let Some(first) = arr.first() {
            let o = first.as_object().expect("audit event object");
            for key in [
                "timestamp",
                "level",
                "user_id",
                "action",
                "resource_type",
                "resource_id",
                "result",
                "metadata",
            ] {
                assert!(o.contains_key(key), "audit event missing `{key}`: {o:?}");
            }
        }
    }
}
