//! JSON shape checks for admin dashboard and RAID admin HTTP API.
//! Enterprise dashboard slices are tested when built with `--features enterprise`.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use serde_json::Value;
use tower::ServiceExt;

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
        assert!(
            v.get("error").is_some(),
            "expected structured JSON error for RAID admin 503: {v:?}"
        );
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
