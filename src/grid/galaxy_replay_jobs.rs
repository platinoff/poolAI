//! Replay verification job enqueue (PH-S535, Galaxy §6.3).
//!
//! On mismatch / replay_pending, create a replay job record blocking settlement until verdict.

use chrono::Utc;
use serde_json::Value;

use crate::core::error::AppError;
use crate::grid::galaxy_replay_metrics::record_replay_verification_enqueue;
use crate::grid::galaxy_settlement::SettlementStatus;
use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore};

/// Verification policy marker on replay verification jobs.
pub const REPLAY_VERIFICATION_POLICY: &str = "replay_verification";

/// Metric: replay verification jobs submitted to JobStore (PH-S535).
pub const METRIC_REPLAY_JOB_SUBMIT_TOTAL: &str = "galaxy_replay_job_submit_total";

static REPLAY_JOB_SUBMIT_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn replay_job_submit_total() -> u64 {
    REPLAY_JOB_SUBMIT_TOTAL.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replay_job_submit_for_test() {
    REPLAY_JOB_SUBMIT_TOTAL.store(0, std::sync::atomic::Ordering::Relaxed);
}

/// Default replay job id when metrics omit explicit `replay_job_id`.
pub fn default_replay_job_id(primary_job_id: &str) -> String {
    format!("{primary_job_id}-replay")
}

fn replay_job_id_from_metrics(primary_job_id: &str, metrics: Option<&Value>) -> String {
    metrics
        .and_then(|m| m.get("replay_job_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| default_replay_job_id(primary_job_id))
}

fn should_enqueue_replay_job(metrics: Option<&Value>, settlement_status: SettlementStatus) -> bool {
    if metrics
        .and_then(|m| m.get("replay_pending"))
        .and_then(|v| v.as_bool())
        == Some(true)
    {
        return true;
    }
    if metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("mismatch"))
    {
        return true;
    }
    settlement_status == SettlementStatus::PendingVerification
        && metrics
            .and_then(|m| m.get("replay_dispute"))
            .and_then(|v| v.as_bool())
            == Some(true)
}

/// Submit replay verification job to JobStore when mismatch/replay flags schedule hold.
pub fn submit_replay_verification_job(
    jobs: &JobStore,
    primary_job_id: &str,
    metrics: Option<&Value>,
    settlement_status: SettlementStatus,
) -> Result<Option<String>, AppError> {
    if !should_enqueue_replay_job(metrics, settlement_status) {
        return Ok(None);
    }
    let replay_id = replay_job_id_from_metrics(primary_job_id, metrics);
    if jobs.get(&replay_id)?.is_some() {
        return Ok(Some(replay_id));
    }
    let record = JobRecord {
        spec: JobSpec {
            id: JobId::new(replay_id.clone()),
            kind: JobKind::System,
            resources: Default::default(),
            priority: 0,
            max_duration_secs: None,
            input_artifact_ids: vec![],
            verification_policy: Some(REPLAY_VERIFICATION_POLICY.into()),
            deadline: None,
        },
        status: JobStatus::Verifying,
        created_at: Utc::now(),
        worker_id: None,
        vm_id: None,
        lease_owner: None,
        lease_epoch: None,
        lease_expires_at: None,
        migration_count: None,
        fail_reason: None,
        leased_at: None,
    };
    jobs.push(record)?;
    record_replay_verification_enqueue();
    REPLAY_JOB_SUBMIT_TOTAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(Some(replay_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement::SettlementStatus;
    use crate::job::JobStore;
    use serde_json::json;

    #[test]
    fn submit_replay_job_on_mismatch_ph_s535() {
        reset_replay_job_submit_for_test();
        let jobs = JobStore::open_for_test(None);
        let id = submit_replay_verification_job(
            &jobs,
            "job-1",
            Some(&json!({ "verification_verdict": "mismatch" })),
            SettlementStatus::Cleared,
        )
        .expect("submit")
        .expect("id");
        assert_eq!(id, "job-1-replay");
        let row = jobs.get(&id).expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Verifying);
        assert_eq!(replay_job_submit_total(), 1);
        reset_replay_job_submit_for_test();
    }
}
