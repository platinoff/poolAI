//! Job lease failover retry budget (PH-S518, Galaxy §4.3.3).
//!
//! PH-S524…S530: worker-unhealthy, queue-starvation, max-total-runtime.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::grid::galaxy_worker_health::is_peer_unhealthy;
use crate::job::{JobRecord, JobStatus};

/// Env: max lease-timeout re-migrations before job fails (default `3`).
pub const ENV_JOB_MAX_MIGRATIONS_PER_JOB: &str = "POOLAI_JOB_MAX_MIGRATIONS_PER_JOB";

/// Env: max wall-clock runtime since `created_at` before job fails (PH-S526).
pub const ENV_JOB_MAX_TOTAL_RUNTIME_SECS: &str = "POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS";

/// Env: `Leased` without `Executing` transition triggers requeue (PH-S530).
pub const ENV_JOB_QUEUE_STARVATION_SECS: &str = "POOLAI_JOB_QUEUE_STARVATION_SECS";

const DEFAULT_MAX_MIGRATIONS: u32 = 3;

/// Fail reason codes on lease failover path (Galaxy §4.3.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeaseFailReason {
    LeaseTimeout,
    BudgetExhausted,
    WorkerUnhealthy,
    QueueStarvation,
    MaxTotalRuntime,
}

impl LeaseFailReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LeaseTimeout => "lease-timeout",
            Self::BudgetExhausted => "budget-exhausted",
            Self::WorkerUnhealthy => "worker-unhealthy",
            Self::QueueStarvation => "queue-starvation",
            Self::MaxTotalRuntime => "max-total-runtime",
        }
    }
}

/// Configured max migrations per job from env.
pub fn max_migrations_per_job() -> u32 {
    std::env::var(ENV_JOB_MAX_MIGRATIONS_PER_JOB)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_MAX_MIGRATIONS)
}

/// Optional max total runtime from env (PH-S526).
pub fn max_total_runtime_secs() -> Option<u64> {
    std::env::var(ENV_JOB_MAX_TOTAL_RUNTIME_SECS)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
}

/// Optional queue starvation threshold from env (PH-S530).
pub fn queue_starvation_secs() -> Option<u64> {
    std::env::var(ENV_JOB_QUEUE_STARVATION_SECS)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
}

/// Next migration count after one lease-timeout requeue attempt.
pub fn next_migration_count(current: u32) -> u32 {
    current.saturating_add(1)
}

/// Whether migration budget is exhausted after increment.
pub fn migration_budget_exhausted(migration_count: u32) -> bool {
    migration_count >= max_migrations_per_job()
}

fn job_past_max_total_runtime(record: &JobRecord, now: DateTime<Utc>) -> bool {
    let Some(limit) = max_total_runtime_secs() else {
        return false;
    };
    if matches!(
        record.status,
        JobStatus::Completed | JobStatus::Failed | JobStatus::Rewarded
    ) {
        return false;
    }
    let elapsed = (now - record.created_at).num_seconds().max(0) as u64;
    elapsed >= limit
}

fn leased_queue_starved(record: &JobRecord, now: DateTime<Utc>) -> bool {
    let Some(threshold) = queue_starvation_secs() else {
        return false;
    };
    if record.status != JobStatus::Leased {
        return false;
    }
    let Some(leased_at) = record.leased_at else {
        return false;
    };
    let elapsed = (now - leased_at).num_seconds().max(0) as u64;
    elapsed >= threshold
}

fn lease_owner_unhealthy(record: &JobRecord) -> bool {
    record
        .lease_owner
        .as_deref()
        .filter(|s| !s.is_empty())
        .is_some_and(is_peer_unhealthy)
}

fn clear_lease_binding(record: &mut JobRecord) {
    record.worker_id = None;
    record.vm_id = None;
    record.lease_owner = None;
    record.lease_expires_at = None;
    record.leased_at = None;
}

fn fail_job(record: &mut JobRecord, reason: LeaseFailReason) {
    record.status = JobStatus::Failed;
    record.fail_reason = Some(reason.as_str().into());
    clear_lease_binding(record);
}

fn requeue_leased(record: &mut JobRecord, reason: LeaseFailReason) {
    record.fail_reason = Some(reason.as_str().into());
    record.status = JobStatus::Submitted;
    clear_lease_binding(record);
}

fn try_requeue_with_budget(record: &mut JobRecord, reason: LeaseFailReason) -> bool {
    let next_count = next_migration_count(record.migration_count.unwrap_or(0));
    record.migration_count = Some(next_count);
    if migration_budget_exhausted(next_count) {
        fail_job(record, LeaseFailReason::BudgetExhausted);
        return true;
    }
    requeue_leased(record, reason);
    true
}

/// Apply max-total-runtime cap (PH-S526). Returns true when status changed.
pub fn apply_max_total_runtime_failover(record: &mut JobRecord, now: DateTime<Utc>) -> bool {
    if !job_past_max_total_runtime(record, now) {
        return false;
    }
    fail_job(record, LeaseFailReason::MaxTotalRuntime);
    true
}

/// Apply lease failover triggers for one row (PH-S518, S524, S530). Returns true when changed.
pub fn apply_lease_failover(record: &mut JobRecord, now: DateTime<Utc>) -> bool {
    if record.status != JobStatus::Leased {
        return false;
    }
    if lease_owner_unhealthy(record) {
        return try_requeue_with_budget(record, LeaseFailReason::WorkerUnhealthy);
    }
    if leased_queue_starved(record, now) {
        return try_requeue_with_budget(record, LeaseFailReason::QueueStarvation);
    }
    let Some(expires_at) = record.lease_expires_at else {
        return false;
    };
    if now < expires_at {
        return false;
    }
    try_requeue_with_budget(record, LeaseFailReason::LeaseTimeout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobRecord, JobSpec};
    use chrono::Duration;

    fn leased_record(id: &str, owner: &str) -> JobRecord {
        let now = Utc::now();
        JobRecord {
            spec: JobSpec {
                id: JobId::new(id),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 1,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now - Duration::seconds(10),
            worker_id: Some(owner.into()),
            vm_id: None,
            lease_owner: Some(owner.into()),
            lease_epoch: Some(1),
            lease_expires_at: Some(now + Duration::seconds(90)),
            migration_count: None,
            fail_reason: None,
            leased_at: Some(now - Duration::seconds(5)),
        }
    }

    #[test]
    fn max_migrations_default_ph_s518() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
        assert_eq!(max_migrations_per_job(), 3);
    }

    #[test]
    fn migration_budget_exhausted_ph_s518() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB, "2");
        assert!(!migration_budget_exhausted(1));
        assert!(migration_budget_exhausted(2));
        std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
    }

    #[test]
    fn fail_reason_wire_labels() {
        assert_eq!(LeaseFailReason::LeaseTimeout.as_str(), "lease-timeout");
        assert_eq!(
            LeaseFailReason::BudgetExhausted.as_str(),
            "budget-exhausted"
        );
        assert_eq!(
            LeaseFailReason::WorkerUnhealthy.as_str(),
            "worker-unhealthy"
        );
        assert_eq!(
            LeaseFailReason::QueueStarvation.as_str(),
            "queue-starvation"
        );
        assert_eq!(
            LeaseFailReason::MaxTotalRuntime.as_str(),
            "max-total-runtime"
        );
    }

    #[test]
    fn max_total_runtime_fails_job_ph_s526() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var(ENV_JOB_MAX_TOTAL_RUNTIME_SECS, "30");
        let mut record = leased_record("rt-1", "w1");
        record.created_at = Utc::now() - Duration::seconds(60);
        assert!(apply_max_total_runtime_failover(&mut record, Utc::now()));
        assert_eq!(record.status, JobStatus::Failed);
        assert_eq!(record.fail_reason.as_deref(), Some("max-total-runtime"));
        std::env::remove_var(ENV_JOB_MAX_TOTAL_RUNTIME_SECS);
    }

    #[test]
    fn queue_starvation_requeues_ph_s530() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var(ENV_JOB_QUEUE_STARVATION_SECS, "10");
        std::env::set_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB, "3");
        let mut record = leased_record("qs-1", "w1");
        record.leased_at = Some(Utc::now() - Duration::seconds(20));
        assert!(apply_lease_failover(&mut record, Utc::now()));
        assert_eq!(record.status, JobStatus::Submitted);
        assert_eq!(record.fail_reason.as_deref(), Some("queue-starvation"));
        std::env::remove_var(ENV_JOB_QUEUE_STARVATION_SECS);
        std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
    }
}
