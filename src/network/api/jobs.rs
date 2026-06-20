//! Jobs API (P6 / FM-020–021) — store + scheduler; `PATCH` lifecycle status updates.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::job::{
    check_patch_lease_epoch, schedule_from_context, JobId, JobKind, JobRecord, JobResources,
    JobSpec, JobStatus, JobStore, PatchLeaseEpochError,
};
use crate::network::api::common::HttpAppError;
use crate::observability::lease_trace::{
    trace_lease_reject, LeaseOperation, LeaseOutcome, LeaseSource,
};

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
    /// Galaxy §4.3.1 optional lease wire (PH-S94).
    #[serde(default)]
    lease_owner: Option<String>,
    #[serde(default)]
    lease_epoch: Option<u64>,
    #[serde(default)]
    lease_expires_at: Option<DateTime<Utc>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    lease_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lease_epoch: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lease_expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct JobsListResponse {
    store_backend: &'static str,
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
    /// Galaxy §4.3.1 CAS stub: must match active `lease_epoch` when job has lease fields (PH-S95).
    #[serde(default)]
    lease_epoch: Option<u64>,
}

#[derive(Deserialize, Default)]
struct AcquireJobLeaseRequest {
    /// Optional override; defaults to bound `worker_id` or `vm_id` (PH-S98).
    #[serde(default)]
    lease_owner: Option<String>,
}

#[derive(Deserialize)]
struct RenewJobLeaseRequest {
    lease_epoch: u64,
}

fn store() -> &'static JobStore {
    JobStore::global()
}

pub fn create_jobs_routes() -> Router<ApiContext> {
    Router::new()
        .route("/jobs", get(list_jobs).post(create_job))
        .route("/jobs/schedule", post(schedule_jobs))
        .route("/jobs/{id}", get(get_job).patch(patch_job))
        .route("/jobs/{id}/lease", post(acquire_job_lease))
        .route("/jobs/{id}/lease/renew", post(renew_job_lease))
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
            lease_owner: r.lease_owner,
            lease_epoch: r.lease_epoch,
            lease_expires_at: r.lease_expires_at,
        })
        .collect();
    Ok(Json(JobsListResponse {
        store_backend: store().store_backend_label(),
        jobs,
    }))
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
        lease_owner: body.lease_owner,
        lease_epoch: body.lease_epoch,
        lease_expires_at: body.lease_expires_at,
        migration_count: None,
        fail_reason: None,
        leased_at: None,
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
        lease_owner: scheduled.lease_owner,
        lease_epoch: scheduled.lease_epoch,
        lease_expires_at: scheduled.lease_expires_at,
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

async fn acquire_job_lease(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
    Json(body): Json<AcquireJobLeaseRequest>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    match store().acquire_lease(&id, body.lease_owner) {
        Ok(job) => Ok(Json(JobDetailResponse { job })),
        Err(AppError::RestError {
            code: "lease_already_active",
            message,
        }) => Err(HttpAppError::new(AppError::RestError {
            code: "lease_already_active",
            message,
        })
        .with_status(StatusCode::CONFLICT)),
        Err(e) => Err(HttpAppError::new(e)),
    }
}

async fn renew_job_lease(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
    Json(body): Json<RenewJobLeaseRequest>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    match store().renew_lease(&id, body.lease_epoch) {
        Ok(job) => Ok(Json(JobDetailResponse { job })),
        Err(AppError::RestError {
            code: "lease_epoch_rejected",
            message,
        }) => Err(HttpAppError::new(AppError::RestError {
            code: "lease_epoch_rejected",
            message,
        })
        .with_status(StatusCode::CONFLICT)),
        Err(AppError::RestError {
            code: "lease_expired",
            message,
        }) => Err(HttpAppError::new(AppError::RestError {
            code: "lease_expired",
            message,
        })
        .with_status(StatusCode::CONFLICT)),
        Err(e) => Err(HttpAppError::new(e)),
    }
}

async fn patch_job(
    State(_ctx): State<ApiContext>,
    Path(id): Path<String>,
    Json(body): Json<PatchJobRequest>,
) -> Result<Json<JobDetailResponse>, HttpAppError> {
    let existing = store()
        .get(&id)?
        .ok_or_else(|| HttpAppError::new(AppError::ApiNotFound(format!("job '{id}' not found"))))?;
    let now = Utc::now();
    if let Err(err) = check_patch_lease_epoch(&existing, body.lease_epoch, now) {
        return Err(match err {
            PatchLeaseEpochError::NoLeaseOnJob => HttpAppError::new(AppError::ValidationError(
                "job has no lease fields; omit lease_epoch on PATCH".into(),
            )),
            PatchLeaseEpochError::Rejected => {
                trace_lease_reject(
                    &id,
                    LeaseOperation::PatchCas,
                    LeaseSource::Api,
                    LeaseOutcome::Rejected,
                    "lease_epoch_rejected",
                    existing.lease_epoch,
                    body.lease_epoch,
                    Some(409),
                );
                HttpAppError::new(AppError::RestError {
                    code: "lease_epoch_rejected",
                    message: format!(
                        "lease_epoch does not match active lease for job '{id}' (Galaxy §4.3.1 CAS stub)"
                    ),
                })
                .with_status(StatusCode::CONFLICT)
            }
        });
    }
    let prior_status = existing.status;
    let job = store().update_status(&id, body.status)?;
    if prior_status == JobStatus::Migrating && body.status == JobStatus::Leased {
        crate::grid::dispatch::re_migrate_prefetch_stub(Some(
            crate::memory::MemoryShardStore::global(),
        ));
    }
    Ok(Json(JobDetailResponse { job }))
}
