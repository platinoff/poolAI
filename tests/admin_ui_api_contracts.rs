//! JSON shape checks for admin dashboard and RAID admin HTTP API.
//! Enterprise dashboard slices are tested when built with `--features enterprise`.
//! FM-013/014/015: admin UI JSON contracts — keys expected by `src/ui/admin/*.rs`.

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
    use poolai::runtime::instance::InstanceManager;
    use poolai::vm::{VmIsolation, VmManager, VmResources};
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
        }
    }

    #[tokio::test]
    async fn topology_nodes_json_shape_when_manager_attached() {
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
        assert!(
            o.contains_key("nodes"),
            "topology nodes missing `nodes`: {o:?}"
        );
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
            for key in ["strategy", "node_ids"] {
                assert!(
                    placement.contains_key(key),
                    "placement missing `{key}`: {placement:?}"
                );
            }
        }
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
mod enterprise_dashboard_slices {
    use super::*;
    use poolai::network::enterprise_api::create_enterprise_api_routes;

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
                "action",
                "resource_type",
                "result",
                "metadata",
            ] {
                assert!(o.contains_key(key), "audit event missing `{key}`: {o:?}");
            }
        }
    }
}
