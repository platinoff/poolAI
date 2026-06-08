//! Grid envelope ingress — Job/Result/MemoryShard side effects (FM-023).
//!
//! Grid `Job` ingest schedules via [`schedule_with_grid_peer`](crate::job::schedule_with_grid_peer);
//! when a source peer binds `worker_id`, scheduler lease acquire sets `JobStatus::Leased` (PH-S108).
//! `Result` ingest validates `lease_epoch` CAS when the job row has active lease fields (PH-S110).
//! Seed inventory + task-driven prefetch policy stub (PH-S129, Galaxy §5.5).

use chrono::Utc;
use serde::{Deserialize, Serialize};

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

/// Worker seed inventory wire DTO (Galaxy §5.2 / §5.5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SeedInventoryEntry {
    #[serde(default)]
    pub shard_ids: Vec<String>,
    #[serde(default)]
    pub hot_tier: SeedInventoryHotTier,
    #[serde(default)]
    pub local_replica_regions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_inventory_at: Option<String>,
}

/// Hot tier subset inside [`SeedInventoryEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SeedInventoryHotTier {
    #[serde(default)]
    pub ram_bytes_used: u64,
    #[serde(default)]
    pub vram_bytes_used: u64,
    #[serde(default)]
    pub profiles: Vec<String>,
}

/// Prefetch destination tier (concept §5.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrefetchTargetTier {
    Ram,
    Vram,
}

/// Prefetch trigger (concept §5.5 table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchTrigger {
    JobAdmitted,
    LeaseAcquired,
    ReMigrate,
}

/// Locality / prefetch strictness (concept §5.5, §5.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrefetchPolicyMode {
    #[default]
    BestEffort,
    StrictLocality,
}

/// Default max wait before Running (`POOLAI_GALAXY_PREFETCH_DEADLINE_MS`, §5.6).
pub const DEFAULT_PREFETCH_DEADLINE_MS: u64 = 15_000;

/// Env: prefetch wait deadline milliseconds (§5.6).
pub const ENV_PREFETCH_DEADLINE_MS: &str = "POOLAI_GALAXY_PREFETCH_DEADLINE_MS";

/// Env: `strict_locality` | `best_effort` (§5.6).
pub const ENV_LOCALITY_MODE: &str = "POOLAI_GALAXY_LOCALITY_MODE";

/// One shard scheduled for prefetch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefetchPlanItem {
    pub shard_id: String,
    pub target_tier: PrefetchTargetTier,
}

/// Planned prefetch work (no wire enqueue in PH-S129).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefetchPlan {
    pub items: Vec<PrefetchPlanItem>,
    pub trigger: PrefetchTrigger,
    pub deadline_ms: u64,
    pub mode: PrefetchPolicyMode,
}

/// Whether `shard_id` is present and hot tier has active bytes (stub §5.5).
#[inline]
pub fn hot_hit(inventory: &SeedInventoryEntry, shard_id: &str) -> bool {
    inventory.shard_ids.iter().any(|id| id == shard_id)
        && (inventory.hot_tier.ram_bytes_used > 0 || inventory.hot_tier.vram_bytes_used > 0)
}

/// Task-driven prefetch plan: skip shards already hot; pick RAM/VRAM tier from capabilities.
pub fn plan_prefetch(
    inventory: &SeedInventoryEntry,
    required_shard_ids: &[String],
    trigger: PrefetchTrigger,
    gpu_capable: bool,
    mode: PrefetchPolicyMode,
) -> PrefetchPlan {
    let target_tier = if gpu_capable {
        PrefetchTargetTier::Vram
    } else {
        PrefetchTargetTier::Ram
    };
    let items = required_shard_ids
        .iter()
        .filter(|shard_id| !hot_hit(inventory, shard_id))
        .map(|shard_id| PrefetchPlanItem {
            shard_id: shard_id.clone(),
            target_tier,
        })
        .collect();
    PrefetchPlan {
        items,
        trigger,
        deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
        mode,
    }
}

/// No-op prefetch hook (PH-S129): returns planned item count; no enqueue/wait wire.
#[inline]
pub fn noop_prefetch_hook(plan: &PrefetchPlan) -> usize {
    plan.items.len()
}

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

    #[test]
    fn seed_inventory_entry_roundtrip_json() {
        let entry = SeedInventoryEntry {
            shard_ids: vec!["w:emb-1".into()],
            hot_tier: SeedInventoryHotTier {
                ram_bytes_used: 1024,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            local_replica_regions: vec!["eu-west".into()],
            last_inventory_at: Some("2026-05-27T10:00:00Z".into()),
        };
        let json = serde_json::to_string(&entry).expect("serialize");
        let back: SeedInventoryEntry = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(entry, back);
    }

    #[test]
    fn plan_prefetch_skips_hot_shards() {
        let inventory = SeedInventoryEntry {
            shard_ids: vec!["w:emb-1".into(), "w:ckpt-7".into()],
            hot_tier: SeedInventoryHotTier {
                ram_bytes_used: 4096,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            ..Default::default()
        };
        let required = vec!["w:emb-1".into(), "w:missing".into()];
        let plan = plan_prefetch(
            &inventory,
            &required,
            PrefetchTrigger::JobAdmitted,
            false,
            PrefetchPolicyMode::BestEffort,
        );
        assert_eq!(plan.items.len(), 1);
        assert_eq!(plan.items[0].shard_id, "w:missing");
        assert_eq!(plan.items[0].target_tier, PrefetchTargetTier::Ram);
        assert_eq!(plan.trigger, PrefetchTrigger::JobAdmitted);
        assert_eq!(plan.deadline_ms, DEFAULT_PREFETCH_DEADLINE_MS);
    }

    #[test]
    fn plan_prefetch_gpu_uses_vram_tier() {
        let inventory = SeedInventoryEntry::default();
        let required = vec!["w:gpu-1".into()];
        let plan = plan_prefetch(
            &inventory,
            &required,
            PrefetchTrigger::LeaseAcquired,
            true,
            PrefetchPolicyMode::StrictLocality,
        );
        assert_eq!(plan.items[0].target_tier, PrefetchTargetTier::Vram);
        assert_eq!(plan.mode, PrefetchPolicyMode::StrictLocality);
    }

    #[test]
    fn noop_prefetch_hook_returns_planned_count() {
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "a".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "b".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::ReMigrate,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(noop_prefetch_hook(&plan), 2);
    }
}
