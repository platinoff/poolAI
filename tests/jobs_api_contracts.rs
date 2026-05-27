//! JSON shape and lifecycle checks for `GET/POST/PATCH /api/v1/jobs` (FM-026).

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use serde_json::{json, Value};
use tower::ServiceExt;

fn jobs_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

fn assert_structured_error(v: &Value) {
    assert!(
        v.get("error").is_some(),
        "expected structured JSON error: {v:?}"
    );
}

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let req_body = if let Some(v) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&v).unwrap())
    } else {
        Body::empty()
    };
    let response = app
        .clone()
        .oneshot(builder.body(req_body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("JSON body")
    };
    (status, v)
}

#[tokio::test]
async fn jobs_list_json_shape() {
    let app = jobs_app();
    let (status, v) = request_json(&app, "GET", "/api/v1/jobs", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        v.get("store_backend").and_then(|x| x.as_str()).is_some(),
        "list response missing store_backend: {v:?}"
    );
    let jobs = v
        .get("jobs")
        .and_then(|x| x.as_array())
        .expect("`jobs` array");
    if let Some(first) = jobs.first() {
        let o = first.as_object().expect("job summary object");
        for key in ["id", "kind", "status", "created_at"] {
            assert!(o.contains_key(key), "job summary missing `{key}`: {o:?}");
        }
    }
}

#[tokio::test]
async fn jobs_create_get_and_patch_lifecycle() {
    let app = jobs_app();

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({
            "kind": "inference",
            "priority": 10,
            "input_artifact_ids": ["artifact-a"]
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let summary = created.as_object().expect("create response object");
    for key in ["id", "kind", "status", "created_at"] {
        assert!(
            summary.contains_key(key),
            "create job summary missing `{key}`: {summary:?}"
        );
    }
    assert_eq!(
        summary.get("kind").and_then(|k| k.as_str()),
        Some("inference")
    );
    assert_eq!(
        summary.get("status").and_then(|s| s.as_str()),
        Some("scheduled"),
        "POST /jobs auto-schedules submitted → scheduled (FM-020)"
    );
    let id = summary
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();

    let (get_status, detail) = request_json(&app, "GET", &format!("/api/v1/jobs/{id}"), None).await;
    assert_eq!(get_status, StatusCode::OK);
    let job = detail
        .get("job")
        .and_then(|x| x.as_object())
        .expect("`job` object in detail");
    assert_eq!(
        job.get("status").and_then(|s| s.as_str()),
        Some("scheduled")
    );
    let spec = job
        .get("spec")
        .and_then(|x| x.as_object())
        .expect("job.spec object");
    for key in ["id", "kind", "resources", "priority", "input_artifact_ids"] {
        assert!(spec.contains_key(key), "job.spec missing `{key}`: {spec:?}");
    }
    assert_eq!(spec.get("id").and_then(|x| x.as_str()), Some(id.as_str()));
    let artifacts = spec
        .get("input_artifact_ids")
        .and_then(|x| x.as_array())
        .expect("input_artifact_ids array");
    assert!(
        artifacts.iter().any(|a| a.as_str() == Some("artifact-a")),
        "expected artifact-a in spec: {artifacts:?}"
    );

    let (patch_status, patched) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{id}"),
        Some(json!({ "status": "executing" })),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(
        patched
            .get("job")
            .and_then(|j| j.get("status"))
            .and_then(|s| s.as_str()),
        Some("executing")
    );

    let (schedule_status, tick) = request_json(&app, "POST", "/api/v1/jobs/schedule", None).await;
    assert_eq!(schedule_status, StatusCode::OK);
    assert!(
        tick.get("scheduled").and_then(|n| n.as_u64()).is_some(),
        "schedule tick missing `scheduled` count: {tick:?}"
    );
}

#[tokio::test]
async fn jobs_patch_invalid_transition_returns_400() {
    let app = jobs_app();

    let (_, created) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({ "kind": "training" })),
    )
    .await;
    let id = created
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();

    let (status, err_body) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{id}"),
        Some(json!({ "status": "completed" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_structured_error(&err_body);
}

#[tokio::test]
async fn jobs_create_with_optional_lease_fields_roundtrip() {
    let app = jobs_app();
    let expires = "2026-12-31T23:59:59Z";

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({
            "kind": "inference",
            "lease_owner": "worker-lease-e2e",
            "lease_epoch": 7,
            "lease_expires_at": expires
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let summary = created.as_object().expect("create response");
    assert_eq!(
        summary.get("lease_owner").and_then(|x| x.as_str()),
        Some("worker-lease-e2e")
    );
    assert_eq!(summary.get("lease_epoch").and_then(|x| x.as_u64()), Some(7));
    assert_eq!(
        summary.get("lease_expires_at").and_then(|x| x.as_str()),
        Some(expires)
    );

    let id = summary
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();
    let (_, detail) = request_json(&app, "GET", &format!("/api/v1/jobs/{id}"), None).await;
    let job = detail
        .get("job")
        .and_then(|x| x.as_object())
        .expect("job detail");
    assert_eq!(
        job.get("lease_owner").and_then(|x| x.as_str()),
        Some("worker-lease-e2e")
    );
    assert_eq!(job.get("lease_epoch").and_then(|x| x.as_u64()), Some(7));
    assert_eq!(
        job.get("lease_expires_at").and_then(|x| x.as_str()),
        Some(expires)
    );
}

#[tokio::test]
async fn jobs_patch_lease_epoch_cas_guard() {
    let app = jobs_app();
    let expires = "2026-12-31T23:59:59Z";

    let (_, created) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({
            "kind": "inference",
            "lease_owner": "worker-cas",
            "lease_epoch": 11,
            "lease_expires_at": expires
        })),
    )
    .await;
    let id = created
        .get("id")
        .and_then(|x| x.as_str())
        .expect("job id")
        .to_string();

    let (ok_status, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{id}"),
        Some(json!({ "status": "executing", "lease_epoch": 11 })),
    )
    .await;
    assert_eq!(ok_status, StatusCode::OK);

    let (reject_status, reject_body) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{id}"),
        Some(json!({ "status": "verifying", "lease_epoch": 10 })),
    )
    .await;
    assert_eq!(reject_status, StatusCode::CONFLICT);
    assert_eq!(
        reject_body
            .get("error")
            .and_then(|e| e.get("code"))
            .and_then(|c| c.as_str()),
        Some("lease_epoch_rejected")
    );

    let (_, no_lease) = request_json(
        &app,
        "POST",
        "/api/v1/jobs",
        Some(json!({ "kind": "training" })),
    )
    .await;
    let plain_id = no_lease
        .get("id")
        .and_then(|x| x.as_str())
        .expect("plain job id")
        .to_string();
    let (bad_status, _) = request_json(
        &app,
        "PATCH",
        &format!("/api/v1/jobs/{plain_id}"),
        Some(json!({ "status": "executing", "lease_epoch": 1 })),
    )
    .await;
    assert_eq!(bad_status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn jobs_get_unknown_returns_404() {
    let app = jobs_app();
    let (status, v) = request_json(
        &app,
        "GET",
        "/api/v1/jobs/00000000-0000-0000-0000-000000000099",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_structured_error(&v);
}
