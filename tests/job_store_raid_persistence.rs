//! PH-S52 / PH-S850: RAID job store restart persistence integration (like PH-S52 stand-restart).

#![cfg(feature = "test-utils")]

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use chrono::Utc;
use poolai::core::state::ApiContext;
use poolai::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore};
use poolai::network::api::create_api_routes;
use serde_json::json;
use tempfile::TempDir;
use tower::ServiceExt;

fn sample_record(id: &str) -> JobRecord {
    JobRecord {
        spec: JobSpec {
            id: JobId::new(id),
            kind: JobKind::Inference,
            resources: Default::default(),
            priority: 0,
            max_duration_secs: None,
            input_artifact_ids: vec![],
            verification_policy: None,
            deadline: None,
        },
        status: JobStatus::Submitted,
        created_at: Utc::now(),
        worker_id: None,
        vm_id: None,
        lease_owner: None,
        lease_epoch: None,
        lease_expires_at: None,
        migration_count: None,
        fail_reason: None,
        leased_at: None,
    }
}

#[test]
fn job_store_persists_via_raid_snapshot_across_reload() {
    let tmp = TempDir::new().expect("tempdir");
    let raid_base = tmp.path().join("raid");

    std::env::set_var("POOLAI_RAID_BASE_PATH", &raid_base);
    std::env::set_var("POOLAI_JOB_STORE", "raid");
    std::env::remove_var("POOLAI_JOB_DATA_DIR");

    let record = sample_record("job-raid-1");

    let store1 = JobStore::open_for_test(None);
    assert_eq!(store1.store_backend_label(), "raid");
    store1.push(record.clone()).expect("push");

    // Reload store and verify it reads snapshot from RAID.
    let store2 = JobStore::open_for_test(None);
    let loaded = store2.get(&record.spec.id.0).expect("get").expect("row");

    assert_eq!(loaded.spec.id.0, "job-raid-1");
    assert_eq!(loaded.spec.kind, JobKind::Inference);
    assert_eq!(loaded.status, JobStatus::Submitted);

    // Status update must also persist via RAID.
    let store1 = JobStore::open_for_test(None);
    store1
        .update_status(&record.spec.id.0, JobStatus::Scheduled)
        .expect("update_status");

    let store3 = JobStore::open_for_test(None);
    let reloaded = store3.get(&record.spec.id.0).expect("get").expect("row");
    assert_eq!(reloaded.status, JobStatus::Scheduled);

    // Same RAID dir: persist path used from a blocking thread (HTTP handler shape).
    let async_record = sample_record("job-raid-async");
    let push_result = std::thread::spawn({
        let async_record = async_record.clone();
        move || -> Result<(), poolai::core::error::AppError> {
            let store = JobStore::open_for_test(None);
            store.push(async_record.clone())?;
            store.update_status(&async_record.spec.id.0, JobStatus::Scheduled)?;
            Ok(())
        }
    })
    .join()
    .expect("thread join");
    push_result.expect("push from blocking thread");

    let store4 = JobStore::open_for_test(None);
    let async_loaded = store4.get("job-raid-async").expect("get").expect("row");
    assert_eq!(async_loaded.status, JobStatus::Scheduled);
}

fn jobs_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

#[tokio::test]
async fn job_store_raid_http_persist_survives_reload_ph_s850() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let tmp = TempDir::new().expect("tempdir");
    let raid_base = tmp.path().join("raid-http");
    std::env::set_var("POOLAI_RAID_BASE_PATH", &raid_base);
    std::env::set_var("POOLAI_JOB_STORE", "raid");
    std::env::remove_var("POOLAI_JOB_DATA_DIR");

    let app = jobs_app();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/jobs")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "kind": "inference",
                        "priority": 3,
                        "input_artifact_ids": ["ph-s850-raid-restart"]
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let job_id = created.get("id").and_then(|v| v.as_str()).expect("job id");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/jobs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let list_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let list_json: serde_json::Value = serde_json::from_slice(&list_bytes).unwrap();
    assert_eq!(
        list_json.get("store_backend").and_then(|v| v.as_str()),
        Some("raid")
    );

    // Simulate coordinator restart: reload job store from RAID snapshot.
    let reloaded = JobStore::open_for_test(None);
    assert_eq!(reloaded.store_backend_label(), "raid");
    let row = reloaded.get(job_id).expect("get").expect("row");
    assert_eq!(row.spec.id.0, job_id);
    assert_eq!(row.spec.kind, JobKind::Inference);
}
