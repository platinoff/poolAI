//! Jobs API (P6 / FM-020–021) — store + scheduler; `PATCH` lifecycle status updates.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::job::{
    schedule_from_context, JobId, JobKind, JobRecord, JobResources, JobSpec, JobStatus, JobStore,
};
use crate::network::api::common::HttpAppError;

#[derive(Deserialize)]
struct CreateJobRequest {
    kind: JobKind,
    #[serde(default)]
    priority: u8,
    #[serde(default)]
    resources: JobResources,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_id: Option<String>,
}

#[derive(Serialize)]
struct JobsListResponse {
    jobs: Vec<JobSummary>,
}

#[derive(Serialize)]
struct JobDetailResponse {
    job: JobRecord,
}

#[derive(Serialize)]
struct ScheduleJobsResponse {
    scheduled: usize,
    bound_workers: usize,
    bound_vms: usize,
    expired: usize,
}

#[derive(Deserialize)]
struct PatchJobRequest {
    status: JobStatus,
}

fn store() -> &'static JobStore {
    JobStore::global()
}

pub fn create_jobs_routes() -> Router<ApiContext> {
    Router::new()
        .route("/jobs", get(list_jobs).post(create_job))
        .route("/jobs/schedule", post(schedule_jobs))
        .route("/jobs/{id}", get(get_job).patch(patch_job))
}

async fn list_jobs(State(_ctx): State<ApiContext>) -> Result<Json<JobsListResponse>, HttpAppError> {
    let jobs = store()
        .list()?
        .into_iter()
        .map(|r| JobSummary {
            id: r.spec.id.0.clone(),
            kind: r.spec.kind,
            status: r.status,
            created_at: r.created_at.to_rfc3339(),
            worker_id: r.worker_id,
            vm_id: r.vm_id,
        })
        .collect();
    Ok(Json(JobsListResponse { jobs }))
}

async fn create_job(
    State(ctx): State<ApiContext>,
    Json(body): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<JobSummary>), HttpAppError> {
    let id = JobId::new(uuid::Uuid::new_v4().to_string());
    let spec = JobSpec {
        id: id.clone(),
        kind: body.kind,
        resources: body.resources,
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
        worker_id: None,
        vm_id: None,
    };
    store().push(record)?;
    schedule_from_context(&ctx, store()).await?;
    let scheduled = store()
        .get(&id.0)?
        .ok_or_else(|| AppError::InternalError("job missing after create".into()))?;
    let summary = JobSummary {
        id: id.0.clone(),
        kind: scheduled.spec.kind,
        status: scheduled.status,
        created_at: scheduled.created_at.to_rfc3339(),
        worker_id: scheduled.worker_id,
        vm_id: scheduled.vm_id,
    };
    Ok((StatusCode::CREATED, Json(summary)))
}

async fn schedule_jobs(
    State(ctx): State<ApiContext>,
) -> Result<Json<ScheduleJobsResponse>, HttpAppError> {
    let outcome = schedule_from_context(&ctx, store()).await?;
    Ok(Json(ScheduleJobsResponse {
        scheduled: outcome.scheduled,
        bound_workers: outcome.bound_workers,
        bound_vms: outcome.bound_vms,
        expired: outcome.expired,
    }))
}

async fn get_job(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    let job = store()
        .get(&id)?
        .ok_or_else(|| HttpAppError::new(AppError::ApiNotFound(format!("job '{id}' not found"))))?;
    Ok(Json(JobDetailResponse { job }))
}

async fn patch_job(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
    Json(body): Json<PatchJobRequest>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    let job = store().update_status(&id, body.status)?;
    Ok(Json(JobDetailResponse { job }))
}
