//! Grid envelope ingress — Job/Result/MemoryShard side effects (FM-023).
//!
//! Grid `Job` ingest schedules via [`schedule_with_grid_peer`](crate::job::schedule_with_grid_peer);
//! when a source peer binds `worker_id`, scheduler lease acquire sets `JobStatus::Leased` (PH-S108).
//! `Result` ingest validates `lease_epoch` CAS when the job row has active lease fields (PH-S110).

use chrono::Utc;

use crate::core::error::AppError;
use crate::grid::{GridEnvelope, GridEnvelopeError, GridMessage, GridResultBody};
use crate::job::{
    check_grid_result_lease_epoch, emit_memory_updated, emit_seed_provided, job_spec_from_grid_job,
    job_status_from_grid_result, memory_content_digest, schedule_with_grid_peer, JobRecord,
    JobStatus, JobStore, PatchLeaseEpochError,
};
use crate::memory::{memory_shard_from_grid_body, MemoryShardStore};
use crate::observability::lease_trace::{
    trace_lease_reject, LeaseOperation, LeaseOutcome, LeaseSource,
};

/// Outcome of processing one grid envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridIngestKind {
    Job { job_id: String, status: JobStatus },
    Result { job_id: String, status: JobStatus },
    MemoryShard { shard_id: String },
    PeerStatus { peer_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridIngestOutcome {
    pub kind: GridIngestKind,
}

pub fn ingest_envelope(
    env: GridEnvelope,
    jobs: &JobStore,
    memory: &MemoryShardStore,
) -> Result<GridIngestOutcome, AppError> {
    env.validate()
        .map_err(|e: GridEnvelopeError| AppError::ValidationError(e.to_string()))?;
    match env.msg {
        GridMessage::Job(body) => ingest_job(body, env.source_peer_id.as_deref(), jobs),
        GridMessage::Result(body) => ingest_result(body, jobs),
        GridMessage::MemoryShard(body) => {
            let shard = memory_shard_from_grid_body(&body);
            let shard_id = shard.shard_id.0.clone();
            memory.upsert(shard)?;
            let provider = env.source_peer_id.as_deref().unwrap_or("coordinator");
            let digest = memory_content_digest(&body.artifact_id, &body.version);
            emit_memory_updated(
                &body.artifact_id,
                &body.version,
                &digest,
                body.raid_logical_name.as_deref(),
                format!("memory:{}:{}", shard_id, body.version),
            );
            if body.seed_hints.as_ref().is_some_and(|h| !h.is_empty()) {
                emit_seed_provided(
                    &shard_id,
                    provider,
                    &body.artifact_id,
                    format!("seed:{}:{}", shard_id, provider),
                );
            }
            Ok(GridIngestOutcome {
                kind: GridIngestKind::MemoryShard { shard_id },
            })
        }
        GridMessage::PeerStatus(body) => Ok(GridIngestOutcome {
            kind: GridIngestKind::PeerStatus {
                peer_id: body.peer_id,
            },
        }),
    }
}

fn ingest_job(
    body: crate::grid::GridJobBody,
    source_peer_id: Option<&str>,
    jobs: &JobStore,
) -> Result<GridIngestOutcome, AppError> {
    let spec = job_spec_from_grid_job(&body);
    let job_id = spec.id.0.clone();
    let record = JobRecord {
        spec,
        status: JobStatus::Submitted,
        created_at: Utc::now(),
        worker_id: None,
        vm_id: None,
        lease_owner: None,
        lease_epoch: None,
        lease_expires_at: None,
    };
    jobs.push(record)?;
    schedule_with_grid_peer(jobs, source_peer_id)?;
    let row = jobs
        .get(&job_id)?
        .ok_or_else(|| AppError::InternalError("job missing after grid ingest".into()))?;
    Ok(GridIngestOutcome {
        kind: GridIngestKind::Job {
            job_id,
            status: row.status,
        },
    })
}

fn ingest_result(body: GridResultBody, jobs: &JobStore) -> Result<GridIngestOutcome, AppError> {
    let job_id = body.job_id.clone();
    let existing = jobs
        .get(&job_id)?
        .ok_or_else(|| AppError::ApiNotFound(format!("job '{job_id}' not found")))?;
    let now = Utc::now();
    if let Err(PatchLeaseEpochError::Rejected) =
        check_grid_result_lease_epoch(&existing, body.lease_epoch, now)
    {
        trace_lease_reject(
            &job_id,
            LeaseOperation::GridResultCas,
            LeaseSource::GridIngest,
            LeaseOutcome::Rejected,
            "lease_epoch_rejected",
            existing.lease_epoch,
            body.lease_epoch,
            Some(409),
        );
        return Err(AppError::RestError {
            code: "lease_epoch_rejected",
            message: format!(
                "lease_epoch does not match active lease for job '{job_id}' (Galaxy §4.3.1 grid result CAS)"
            ),
        });
    }
    let status = job_status_from_grid_result(body.status);
    jobs.force_status(&job_id, status)?;
    Ok(GridIngestOutcome {
        kind: GridIngestKind::Result { job_id, status },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{GridEnvelope, GridJobBody, GridMessage, GridResultStatus};
    use crate::job::{JobId, JobKind, JobSpec, JobStatus};
    use chrono::Utc;

    #[test]
    fn ingest_job_with_peer_sets_leased_and_lease_fields() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-1".into(),
                task_kind: "inference".into(),
                verification_policy: None,
                input_artifact_ids: vec![],
                deadline: None,
            }),
            Some("peer-a".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Job {
                job_id: "grid-job-1".into(),
                status: JobStatus::Leased,
            }
        );
        let row = jobs.get("grid-job-1").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
        assert_eq!(row.worker_id.as_deref(), Some("peer-a"));
        assert_eq!(row.lease_owner.as_deref(), Some("peer-a"));
        assert_eq!(row.lease_epoch, Some(1));
        assert!(row.lease_expires_at.is_some());
    }

    #[test]
    fn ingest_job_without_peer_stays_scheduled_without_lease() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-no-peer".into(),
                task_kind: "inference".into(),
                verification_policy: None,
                input_artifact_ids: vec![],
                deadline: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Job {
                job_id: "grid-job-no-peer".into(),
                status: JobStatus::Scheduled,
            }
        );
        let row = jobs.get("grid-job-no-peer").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Scheduled);
        assert!(row.worker_id.is_none());
        assert!(row.lease_owner.is_none());
        assert!(row.lease_epoch.is_none());
        assert!(row.lease_expires_at.is_none());
    }

    #[test]
    fn ingest_result_updates_job() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-job-2"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-job-2".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec!["out-1".into()],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-job-2".into(),
                status: JobStatus::Completed,
            }
        );
    }

    #[test]
    fn ingest_result_accepts_matching_lease_epoch() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-ok"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(3),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-ok".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec!["out-ok".into()],
                proof: None,
                metrics: None,
                lease_epoch: Some(3),
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-result-ok".into(),
                status: JobStatus::Completed,
            }
        );
    }

    #[test]
    fn ingest_result_rejects_lease_epoch_mismatch() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-bad"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(5),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-bad".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: Some(4),
            }),
            None,
        );
        let err = ingest_envelope(env, &jobs, &memory).expect_err("reject");
        match err {
            AppError::RestError { code, .. } => assert_eq!(code, "lease_epoch_rejected"),
            other => panic!("expected RestError, got {other:?}"),
        }
        let row = jobs.get("grid-result-bad").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
    }

    #[test]
    fn ingest_result_rejects_missing_lease_epoch_on_leased_job() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-no-epoch"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(1),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-no-epoch".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            None,
        );
        let err = ingest_envelope(env, &jobs, &memory).expect_err("reject");
        match err {
            AppError::RestError { code, .. } => assert_eq!(code, "lease_epoch_rejected"),
            other => panic!("expected RestError, got {other:?}"),
        }
    }

    #[test]
    fn ingest_memory_shard_upserts() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::MemoryShard(crate::grid::GridMemoryShardBody {
                shard_id: "w:1".into(),
                artifact_id: "art-1".into(),
                version: "1".into(),
                raid_logical_name: Some("weights".into()),
                seed_hints: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::MemoryShard {
                shard_id: "w:1".into()
            }
        );
        let shard = memory.get("w:1").expect("get").expect("row");
        assert_eq!(shard.artifact_id, "art-1");
    }
}
