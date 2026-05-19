//! Jobs API stub (P6 / Horizon S38) — in-process store, no scheduler yet.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus};
use crate::network::api::common::HttpAppError;

static JOB_STORE: LazyLock<Mutex<Vec<JobRecord>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Deserialize)]
struct CreateJobRequest {
    kind: JobKind,
    #[serde(default)]
    priority: u8,
    #[serde(default)]
    input_artifact_ids: Vec<String>,
    #[serde(default)]
    verification_policy: Option<String>,
}

#[derive(Serialize)]
struct JobSummary {
    id: String,
    kind: JobKind,
    status: JobStatus,
    created_at: String,
}

#[derive(Serialize)]
struct JobsListResponse {
    jobs: Vec<JobSummary>,
}

#[derive(Serialize)]
struct JobDetailResponse {
    job: JobRecord,
}

fn lock_store() -> Result<std::sync::MutexGuard<'static, Vec<JobRecord>>, HttpAppError> {
    JOB_STORE
        .lock()
        .map_err(|_| AppError::InternalError("job store lock poisoned".into()).into())
}

pub fn create_jobs_routes() -> Router<ApiContext> {
    Router::new()
        .route("/jobs", get(list_jobs).post(create_job))
        .route("/jobs/{id}", get(get_job))
}

async fn list_jobs(State(_ctx): State<ApiContext>) -> Result<Json<JobsListResponse>, HttpAppError> {
    let store = lock_store()?;
    let jobs = store
        .iter()
        .map(|r| JobSummary {
            id: r.spec.id.0.clone(),
            kind: r.spec.kind,
            status: r.status,
            created_at: r.created_at.to_rfc3339(),
        })
        .collect();
    Ok(Json(JobsListResponse { jobs }))
}

async fn create_job(
    State(_ctx): State<ApiContext>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobSummary>), HttpAppError> {
    let id = JobId::new(uuid::Uuid::new_v4().to_string());
    let spec = JobSpec {
        id: id.clone(),
        kind: body.kind,
        resources: Default::default(),
        priority: body.priority,
        max_duration_secs: None,
        input_artifact_ids: body.input_artifact_ids,
        verification_policy: body.verification_policy,
        deadline: None,
    };
    let record = JobRecord {
        spec: spec.clone(),
        status: JobStatus::Submitted,
        created_at: Utc::now(),
    };
    let summary = JobSummary {
        id: id.0.clone(),
        kind: record.spec.kind,
        status: record.status,
        created_at: record.created_at.to_rfc3339(),
    };
    lock_store()?.push(record);
    Ok((StatusCode::CREATED, Json(summary)))
}

async fn get_job(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    let store = lock_store()?;
    let job = store
        .iter()
        .find(|r| r.spec.id.0 == id)
        .cloned()
        .ok_or_else(|| HttpAppError::new(AppError::ApiNotFound(format!("job '{id}' not found"))))?;
    Ok(Json(JobDetailResponse { job }))
}
