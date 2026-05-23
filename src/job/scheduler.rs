//! Job scheduler: `Submitted` → `Scheduled` with optional pool worker / VM binding (FM-034).

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
}

/// Running VM instance eligible for job binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmCandidate {
    pub id: String,
    pub memory_mb: u64,
}

/// Scheduler tick result (FM-020 + FM-034).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScheduleOutcome {
    pub scheduled: usize,
    pub bound_workers: usize,
    pub bound_vms: usize,
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
    let (scheduled, bound_workers, bound_vms) =
        store.promote_submitted_to_scheduled_with(|spec: &JobSpec| {
            let worker_id = pick_worker(workers, &spec.resources, &mut taken_workers);
            let vm_id = pick_vm(vms, &spec.resources, &mut taken_vms);
            JobScheduleBinding { worker_id, vm_id }
        })?;
    Ok(ScheduleOutcome {
        scheduled,
        bound_workers,
        bound_vms,
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
        })
        .collect()
}

fn worker_free_memory_mb(used_mb: f32) -> u64 {
    const DEFAULT_CAPACITY_MB: f32 = 8192.0;
    (DEFAULT_CAPACITY_MB - used_mb).max(0.0) as u64
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
        }
    }

    fn worker(id: &str, connections: usize) -> WorkerCandidate {
        WorkerCandidate {
            id: id.into(),
            active_connections: connections,
            is_healthy: true,
            free_memory_mb: 16_384,
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

        let workers = vec![worker("busy", 5), worker("idle", 0)];
        let outcome = schedule_with_workers(&store, &workers, &[]).expect("schedule");
        assert_eq!(outcome.scheduled, 1);
        assert_eq!(outcome.bound_workers, 1);

        let row = store.get("job-a").expect("get").expect("row");
        assert_eq!(row.worker_id.as_deref(), Some("idle"));
        assert!(row.vm_id.is_none());
    }

    #[test]
    fn binds_distinct_workers_for_two_jobs() {
        let store = JobStore::open_for_test(None);
        store.push(sample_record("job-1", 0)).expect("push");
        store.push(sample_record("job-2", 0)).expect("push");

        let workers = vec![worker("w1", 0), worker("w2", 1)];
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
            schedule_with_workers(&store, &[worker("w1", 0)], &[]).expect("schedule");
        }

        let reloaded = JobStore::open_for_test(Some(dir));
        let job = reloaded.get("job-1").expect("get").expect("row");
        assert_eq!(job.status, JobStatus::Scheduled);
        assert_eq!(job.worker_id.as_deref(), Some("w1"));
    }

    #[test]
    fn skips_non_submitted() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("done", 0);
        record.status = JobStatus::Executing;
        store.push(record).expect("push");

        let outcome = schedule_with_workers(&store, &[worker("w1", 0)], &[]).expect("schedule");
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
}
