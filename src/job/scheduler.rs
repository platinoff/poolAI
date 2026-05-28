//! Job scheduler: `Submitted` → `Scheduled` with optional pool worker / VM binding (FM-034, PH-S38).

use std::collections::{HashMap, HashSet};

use crate::core::error::AppError;
use crate::core::state::{ApiContext, WorkerStatus};
use crate::job::store::JobStore;
use crate::job::{JobResources, JobScheduleBinding, JobSpec};
use crate::pool::worker::WorkerStatus as PoolWorkerStatus;
use crate::services::vm_service::VmService;
use crate::vm::VmStatus;

/// Eligible pool worker for least-connections placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerCandidate {
    pub id: String,
    pub active_connections: usize,
    pub is_healthy: bool,
    pub free_memory_mb: u64,
    /// Pool worker has a GPU device configured (PH-S38).
    pub has_gpu: bool,
}

/// Running VM instance eligible for job binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmCandidate {
    pub id: String,
    pub memory_mb: u64,
    /// VM instance was created with `gpu_required` (PH-S38).
    pub gpu_capable: bool,
}

/// Scheduler tick result (FM-020 + FM-034 + PH-S38).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScheduleOutcome {
    pub scheduled: usize,
    pub bound_workers: usize,
    pub bound_vms: usize,
    /// Jobs past `spec.deadline` marked `failed` instead of scheduled.
    pub expired: usize,
}

/// FM-020 compat: promote without binding when no pool/VM context is available.
pub fn schedule_pending(store: &JobStore) -> Result<usize, AppError> {
    let outcome = schedule_with_workers(store, &[], &[])?;
    Ok(outcome.scheduled)
}

/// Schedule pending jobs and bind to workers/VMs when candidates are provided.
pub fn schedule_with_workers(
    store: &JobStore,
    workers: &[WorkerCandidate],
    vms: &[VmCandidate],
) -> Result<ScheduleOutcome, AppError> {
    let mut taken_workers = HashSet::new();
    let mut taken_vms = HashSet::new();
    let (scheduled, bound_workers, bound_vms, expired) = store
        .promote_submitted_to_scheduled_with(|spec: &JobSpec| {
            pick_schedule_binding(
                workers,
                vms,
                &spec.resources,
                &mut taken_workers,
                &mut taken_vms,
            )
        })?;
    Ok(ScheduleOutcome {
        scheduled,
        bound_workers,
        bound_vms,
        expired,
    })
}

/// Grid ingress: schedule with optional originating peer as executor hint.
pub fn schedule_with_grid_peer(
    store: &JobStore,
    source_peer_id: Option<&str>,
) -> Result<ScheduleOutcome, AppError> {
    let peer_binding = source_peer_id.map(|id| JobScheduleBinding {
        worker_id: Some(id.to_string()),
        vm_id: None,
    });
    let (scheduled, bound_workers, bound_vms, expired) =
        store.promote_submitted_to_scheduled_with(|_| peer_binding.clone().unwrap_or_default())?;
    Ok(ScheduleOutcome {
        scheduled,
        bound_workers,
        bound_vms,
        expired,
    })
}

/// Gather pool/VM candidates from [`ApiContext`] and run one scheduler tick.
pub async fn schedule_from_context(
    ctx: &ApiContext,
    store: &JobStore,
) -> Result<ScheduleOutcome, AppError> {
    let workers = gather_worker_candidates(ctx).await;
    let vms = gather_vm_candidates(ctx).await;
    schedule_with_workers(store, &workers, &vms)
}

pub fn worker_candidates_from_pool_status(
    statuses: &HashMap<String, PoolWorkerStatus>,
) -> Vec<WorkerCandidate> {
    statuses
        .iter()
        .map(|(id, status)| WorkerCandidate {
            id: id.clone(),
            active_connections: status.active_connections,
            is_healthy: status.is_healthy,
            free_memory_mb: worker_free_memory_mb(status.memory_usage_mb),
            has_gpu: status.gpu_usage.is_some(),
        })
        .collect()
}

async fn gather_worker_candidates(ctx: &ApiContext) -> Vec<WorkerCandidate> {
    if let Some(pool) = ctx.pool.get() {
        let statuses = {
            let guard = pool.read().await;
            guard.get_worker_status().await
        };
        if !statuses.is_empty() {
            return worker_candidates_from_pool_status(&statuses);
        }
    }
    let guard = ctx.workers.read();
    guard
        .values()
        .filter(|w| matches!(w.status, WorkerStatus::Active))
        .map(|w| WorkerCandidate {
            id: w.id.clone(),
            active_connections: w.active_models.len(),
            is_healthy: true,
            free_memory_mb: worker_free_memory_mb(w.metrics.memory_usage_mb),
            has_gpu: w.metrics.gpu_utilization > 0.0 || w.metrics.gpu_temperature > 0.0,
        })
        .collect()
}

async fn gather_vm_candidates(ctx: &ApiContext) -> Vec<VmCandidate> {
    let Ok(instances) = VmService::list_instances(ctx).await else {
        return Vec::new();
    };
    instances
        .into_iter()
        .filter(|inst| matches!(inst.status, VmStatus::Running))
        .map(|inst| VmCandidate {
            id: inst.id.to_string(),
            memory_mb: inst.resources.memory_mb as u64,
            gpu_capable: inst.resources.gpu_required,
        })
        .collect()
}

fn worker_free_memory_mb(used_mb: f32) -> u64 {
    const DEFAULT_CAPACITY_MB: f32 = 8192.0;
    (DEFAULT_CAPACITY_MB - used_mb).max(0.0) as u64
}

fn pick_schedule_binding(
    workers: &[WorkerCandidate],
    vms: &[VmCandidate],
    resources: &JobResources,
    taken_workers: &mut HashSet<String>,
    taken_vms: &mut HashSet<String>,
) -> JobScheduleBinding {
    if resources.gpu_memory_mb.is_some() {
        if let Some(vm_id) = pick_vm(vms, resources, taken_vms) {
            return JobScheduleBinding {
                worker_id: None,
                vm_id: Some(vm_id),
            };
        }
        if let Some(worker_id) = pick_worker(workers, resources, taken_workers) {
            return JobScheduleBinding {
                worker_id: Some(worker_id),
                vm_id: None,
            };
        }
        return JobScheduleBinding::default();
    }
    let worker_id = pick_worker(workers, resources, taken_workers);
    let vm_id = pick_vm(vms, resources, taken_vms);
    JobScheduleBinding { worker_id, vm_id }
}

fn pick_worker(
    workers: &[WorkerCandidate],
    resources: &JobResources,
    taken: &mut HashSet<String>,
) -> Option<String> {
    let mut eligible: Vec<&WorkerCandidate> = workers
        .iter()
        .filter(|w| w.is_healthy && !taken.contains(&w.id))
        .filter(|w| worker_meets_resources(w, resources))
        .filter(|w| worker_meets_gpu(w, resources))
        .collect();
    eligible.sort_by(|a, b| {
        a.active_connections
            .cmp(&b.active_connections)
            .then_with(|| a.id.cmp(&b.id))
    });
    let id = eligible.first().map(|w| w.id.clone())?;
    taken.insert(id.clone());
    Some(id)
}

fn pick_vm(
    vms: &[VmCandidate],
    resources: &JobResources,
    taken: &mut HashSet<String>,
) -> Option<String> {
    let mut eligible: Vec<&VmCandidate> = vms
        .iter()
        .filter(|v| !taken.contains(&v.id))
        .filter(|v| vm_meets_resources(v, resources))
        .filter(|v| vm_meets_gpu(v, resources))
        .collect();
    eligible.sort_by(|a, b| a.memory_mb.cmp(&b.memory_mb).then_with(|| a.id.cmp(&b.id)));
    let id = eligible.first().map(|v| v.id.clone())?;
    taken.insert(id.clone());
    Some(id)
}

fn worker_meets_resources(worker: &WorkerCandidate, resources: &JobResources) -> bool {
    if let Some(ram) = resources.ram_mb {
        if worker.free_memory_mb < ram {
            return false;
        }
    }
    true
}

fn vm_meets_resources(vm: &VmCandidate, resources: &JobResources) -> bool {
    if let Some(ram) = resources.ram_mb {
        if vm.memory_mb < ram {
            return false;
        }
    }
    true
}

fn worker_meets_gpu(worker: &WorkerCandidate, resources: &JobResources) -> bool {
    if resources.gpu_memory_mb.is_some() {
        return worker.has_gpu;
    }
    true
}

fn vm_meets_gpu(vm: &VmCandidate, resources: &JobResources) -> bool {
    if resources.gpu_memory_mb.is_some() {
        return vm.gpu_capable;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus};
    use chrono::Utc;
    use tempfile::TempDir;

    fn sample_record(id: &str, priority: u8) -> JobRecord {
        JobRecord {
            spec: JobSpec {
                id: JobId::new(id),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority,
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
        }
    }

    fn worker(id: &str, connections: usize, has_gpu: bool) -> WorkerCandidate {
        WorkerCandidate {
            id: id.into(),
            active_connections: connections,
            is_healthy: true,
            free_memory_mb: 16_384,
            has_gpu,
        }
    }

    #[test]
    fn promotes_submitted_by_priority() {
        let tmp = TempDir::new().expect("tempdir");
        let store = JobStore::open_for_test(Some(tmp.path().to_path_buf()));

        store.push(sample_record("low", 1)).expect("push");
        store.push(sample_record("high", 10)).expect("push");

        let outcome = schedule_with_workers(&store, &[], &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 2);
        assert_eq!(outcome.bound_workers, 0);

        let high = store.get("high").expect("get").expect("row");
        let low = store.get("low").expect("get").expect("row");
        assert_eq!(high.status, JobStatus::Scheduled);
        assert_eq!(low.status, JobStatus::Scheduled);
    }

    #[test]
    fn binds_least_loaded_worker() {
        let store = JobStore::open_for_test(None);
        store.push(sample_record("job-a", 0)).expect("push");

        let workers = vec![worker("busy", 5, false), worker("idle", 0, false)];
        let outcome = schedule_with_workers(&store, &workers, &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 1);
        assert_eq!(outcome.bound_workers, 1);

        let row = store.get("job-a").expect("get").expect("row");
        assert_eq!(row.worker_id.as_deref(), Some("idle"));
        assert!(row.vm_id.is_none());
        assert_eq!(row.status, JobStatus::Leased, "PH-S100: lease → Leased");
        assert!(row.has_lease_fields(), "PH-S98: schedule acquires lease");
        assert_eq!(row.lease_owner.as_deref(), Some("idle"));
        assert_eq!(row.lease_epoch, Some(1));
    }

    #[test]
    fn binds_distinct_workers_for_two_jobs() {
        let store = JobStore::open_for_test(None);
        store.push(sample_record("job-1", 0)).expect("push");
        store.push(sample_record("job-2", 0)).expect("push");

        let workers = vec![worker("w1", 0, false), worker("w2", 1, false)];
        schedule_with_workers(&store, &workers, &[]).expect("schedule");

        let j1 = store.get("job-1").expect("get").expect("row");
        let j2 = store.get("job-2").expect("get").expect("row");
        assert_ne!(j1.worker_id, j2.worker_id);
    }

    #[test]
    fn schedule_persists_binding_across_reload() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();

        {
            let store = JobStore::open_for_test(Some(dir.clone()));
            store.push(sample_record("job-1", 0)).expect("push");
            schedule_with_workers(&store, &[worker("w1", 0, false)], &[]).expect("schedule");
        }

        let reloaded = JobStore::open_for_test(Some(dir));
        let job = reloaded.get("job-1").expect("get").expect("row");
        assert_eq!(job.status, JobStatus::Leased);
        assert_eq!(job.worker_id.as_deref(), Some("w1"));
    }

    #[test]
    fn skips_non_submitted() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("done", 0);
        record.status = JobStatus::Executing;
        store.push(record).expect("push");

        let outcome =
            schedule_with_workers(&store, &[worker("w1", 0, false)], &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 0);
        assert_eq!(
            store.get("done").expect("get").expect("row").status,
            JobStatus::Executing
        );
    }

    #[test]
    fn respects_ram_requirement() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("heavy", 0);
        record.spec.resources.ram_mb = Some(32_768);
        store.push(record).expect("push");

        let workers = vec![WorkerCandidate {
            id: "small".into(),
            active_connections: 0,
            is_healthy: true,
            free_memory_mb: 4096,
            has_gpu: false,
        }];
        let outcome = schedule_with_workers(&store, &workers, &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 1);
        assert_eq!(outcome.bound_workers, 0);
        assert!(store
            .get("heavy")
            .expect("get")
            .expect("row")
            .worker_id
            .is_none());
    }

    #[test]
    fn expired_deadline_in_promote() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("expired", 0);
        record.spec.deadline = Some(Utc::now() - chrono::Duration::minutes(5));
        store.push(record).expect("push");

        let outcome = schedule_with_workers(&store, &[], &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 0);
        assert_eq!(outcome.expired, 1);
    }

    #[test]
    fn gpu_job_prefers_gpu_vm_over_worker() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("gpu-job", 0);
        record.spec.resources.gpu_memory_mb = Some(4096);
        store.push(record).expect("push");

        let workers = vec![worker("cpu-worker", 0, false)];
        let vms = vec![VmCandidate {
            id: "gpu-vm".into(),
            memory_mb: 8192,
            gpu_capable: true,
        }];
        let outcome = schedule_with_workers(&store, &workers, &vms).expect("schedule");
        assert_eq!(outcome.bound_vms, 1);
        assert_eq!(outcome.bound_workers, 0);
        let row = store.get("gpu-job").expect("get").expect("row");
        assert_eq!(row.vm_id.as_deref(), Some("gpu-vm"));
    }

    #[test]
    fn grid_peer_binds_worker_on_schedule() {
        let store = JobStore::open_for_test(None);
        store.push(sample_record("grid-1", 0)).expect("push");
        let outcome = schedule_with_grid_peer(&store, Some("peer-grid-a")).expect("schedule");
        assert_eq!(outcome.bound_workers, 1);
        let row = store.get("grid-1").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
        assert_eq!(row.worker_id.as_deref(), Some("peer-grid-a"));
    }

    #[test]
    fn expired_leased_job_requeues_and_rebinds() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("leased-expired", 1);
        record.status = JobStatus::Leased;
        record.worker_id = Some("old-worker".into());
        record.lease_owner = Some("old-worker".into());
        record.lease_epoch = Some(7);
        record.lease_expires_at = Some(Utc::now() - chrono::Duration::seconds(30));
        store.push(record).expect("push");

        let workers = vec![
            worker("old-worker", 9, false),
            worker("new-worker", 0, false),
        ];
        let outcome = schedule_with_workers(&store, &workers, &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 1);

        let row = store.get("leased-expired").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
        assert_eq!(row.worker_id.as_deref(), Some("new-worker"));
        assert_eq!(row.lease_owner.as_deref(), Some("new-worker"));
        assert_eq!(row.lease_epoch, Some(8));
        assert!(row.lease_expires_at.is_some());
    }
}
