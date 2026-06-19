//! Shadow verification checker job submit (PH-S534, Galaxy §6.2).
//!
//! On `SampleScheduled`, enqueue a trusted `local_srv` shadow job in [`JobStore`].

use chrono::Utc;

use crate::core::error::AppError;
use crate::grid::galaxy_verification_metrics::{
    enqueue_verification_checker_task, record_verification_checker_enqueue,
};
use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore};

/// Trusted local worker id for shadow verification checks (Galaxy §6.2).
pub const SHADOW_CHECKER_WORKER_ID: &str = "local_srv";

/// Verification policy marker on shadow checker jobs.
pub const SHADOW_CHECKER_POLICY: &str = "shadow_checker";

/// Metric: shadow checker jobs submitted to JobStore (PH-S534).
pub const METRIC_VERIFICATION_CHECKER_JOB_SUBMIT_TOTAL: &str =
    "galaxy_verification_checker_job_submit_total";

static CHECKER_JOB_SUBMIT_TOTAL: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

pub fn verification_checker_job_submit_total() -> u64 {
    CHECKER_JOB_SUBMIT_TOTAL.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_checker_job_submit_for_test() {
    CHECKER_JOB_SUBMIT_TOTAL.store(0, std::sync::atomic::Ordering::Relaxed);
}

/// Shadow checker job id derived from the primary edge job.
pub fn shadow_checker_job_id(primary_job_id: &str) -> String {
    format!("{primary_job_id}-shadow-check")
}

/// Submit shadow verification checker job to JobStore when sample is scheduled.
pub fn submit_shadow_verification_checker_job(
    jobs: &JobStore,
    primary_job_id: &str,
) -> Result<String, AppError> {
    let shadow_id = shadow_checker_job_id(primary_job_id);
    if jobs.get(&shadow_id)?.is_some() {
        return Ok(shadow_id);
    }
    let record = JobRecord {
        spec: JobSpec {
            id: JobId::new(shadow_id.clone()),
            kind: JobKind::System,
            resources: Default::default(),
            priority: 0,
            max_duration_secs: None,
            input_artifact_ids: vec![],
            verification_policy: Some(SHADOW_CHECKER_POLICY.into()),
            deadline: None,
        },
        status: JobStatus::Scheduled,
        created_at: Utc::now(),
        worker_id: Some(SHADOW_CHECKER_WORKER_ID.into()),
        vm_id: None,
        lease_owner: None,
        lease_epoch: None,
        lease_expires_at: None,
        migration_count: None,
        fail_reason: None,
        leased_at: None,
    };
    jobs.push(record)?;
    enqueue_verification_checker_task(primary_job_id);
    record_verification_checker_enqueue();
    CHECKER_JOB_SUBMIT_TOTAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(shadow_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::JobStore;

    #[test]
    fn submit_shadow_checker_job_ph_s534() {
        reset_verification_checker_job_submit_for_test();
        let jobs = JobStore::open_for_test(None);
        let id = submit_shadow_verification_checker_job(&jobs, "job-edge-1").expect("submit");
        assert_eq!(id, "job-edge-1-shadow-check");
        let row = jobs.get(&id).expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Scheduled);
        assert_eq!(row.worker_id.as_deref(), Some(SHADOW_CHECKER_WORKER_ID));
        assert_eq!(
            row.spec.verification_policy.as_deref(),
            Some(SHADOW_CHECKER_POLICY)
        );
        assert_eq!(verification_checker_job_submit_total(), 1);
        reset_verification_checker_job_submit_for_test();
    }
}
