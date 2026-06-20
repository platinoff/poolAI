//! Galaxy Grid replication metrics stub (PH-S179, §6.3).
//!
//! Counter for grid jobs ingested with `replication_strict` tier; executor enqueue to JobStore (PH-S536).

use std::sync::atomic::{AtomicU64, Ordering};

use chrono::Utc;

use crate::core::error::AppError;
use crate::grid::galaxy_replication::{ReplicationProfile, ReplicationTierConfig};
use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore};

/// Env: max strict-tier replication enqueues per hour (PH-S457 stub).
pub const ENV_REPLICATION_MAX_PER_HOUR: &str = "POOLAI_GALAXY_REPLICATION_MAX_PER_HOUR";

/// Default hourly cap for strict-tier replication enqueue stub.
pub const DEFAULT_REPLICATION_MAX_PER_HOUR: u64 = 1000;

/// In-process counter for strict-tier grid job ingests (mirrored on `GET /metrics`).
pub const METRIC_REPLICATION_STRICT_TOTAL: &str = "galaxy_replication_strict_total";

/// Executor enqueue stub invocations on grid job ingest (PH-S426).
pub const METRIC_REPLICATION_ENQUEUE_TOTAL: &str = "galaxy_replication_enqueue_total";

/// Replication executor queue stub invocations (PH-S435).
pub const METRIC_REPLICATION_EXECUTOR_ENQUEUE_TOTAL: &str =
    "galaxy_replication_executor_enqueue_total";

/// Strict-tier replication rate-limited rejections (PH-S457).
pub const METRIC_REPLICATION_RATE_LIMITED_TOTAL: &str = "galaxy_replication_rate_limited_total";

static REPLICATION_STRICT_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLICATION_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLICATION_EXECUTOR_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLICATION_HOURLY_COUNT: AtomicU64 = AtomicU64::new(0);
static REPLICATION_RATE_LIMITED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one grid job ingest with `replication_strict` tier.
pub fn record_replication_strict_ingest() {
    REPLICATION_STRICT_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replication_strict_total() -> u64 {
    REPLICATION_STRICT_TOTAL.load(Ordering::Relaxed)
}

/// Record one replication executor enqueue stub (PH-S426).
pub fn record_replication_enqueue() {
    REPLICATION_ENQUEUE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replication_enqueue_total() -> u64 {
    REPLICATION_ENQUEUE_TOTAL.load(Ordering::Relaxed)
}

/// Record one replication executor queue stub (PH-S435).
pub fn record_replication_executor_enqueue() {
    REPLICATION_EXECUTOR_ENQUEUE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replication_executor_enqueue_total() -> u64 {
    REPLICATION_EXECUTOR_ENQUEUE_TOTAL.load(Ordering::Relaxed)
}

/// Parse hourly replication cap from env (PH-S457).
pub fn replication_max_per_hour_from_env() -> u64 {
    std::env::var(ENV_REPLICATION_MAX_PER_HOUR)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_REPLICATION_MAX_PER_HOUR)
}

/// Returns true when strict-tier replication enqueue should proceed (PH-S457 stub).
pub fn replication_enqueue_allowed(replication_tier: ReplicationTierConfig) -> bool {
    if replication_tier.profile != ReplicationProfile::Strict {
        return true;
    }
    let cap = replication_max_per_hour_from_env();
    let current = REPLICATION_HOURLY_COUNT.load(Ordering::Relaxed);
    if current >= cap {
        record_replication_rate_limited();
        false
    } else {
        REPLICATION_HOURLY_COUNT.fetch_add(1, Ordering::Relaxed);
        true
    }
}

pub fn record_replication_rate_limited() {
    REPLICATION_RATE_LIMITED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replication_rate_limited_total() -> u64 {
    REPLICATION_RATE_LIMITED_TOTAL.load(Ordering::Relaxed)
}

/// Read-only replication counters snapshot for `GET /api/v1/grid/replication-metrics` (PH-S690).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ReplicationMetricsSnapshot {
    pub strict_total: u64,
    pub enqueue_total: u64,
    pub executor_enqueue_total: u64,
    pub rate_limited_total: u64,
}

/// Coordinator replication metrics snapshot (PH-S690).
pub fn replication_metrics_snapshot() -> ReplicationMetricsSnapshot {
    ReplicationMetricsSnapshot {
        strict_total: replication_strict_total(),
        enqueue_total: replication_enqueue_total(),
        executor_enqueue_total: replication_executor_enqueue_total(),
        rate_limited_total: replication_rate_limited_total(),
    }
}

/// Grid job ingest executor queue stub (PH-S435); JobStore parallel fan-out (PH-S536).
pub fn replication_executor_hook(
    replication_tier: ReplicationTierConfig,
    jobs: Option<&JobStore>,
    primary_job_id: Option<&str>,
) {
    if !replication_enqueue_allowed(replication_tier) {
        return;
    }
    record_replication_executor_enqueue();
    evaluate_job_replication_strict(replication_tier);
    if replication_tier.profile == ReplicationProfile::Strict {
        if let (Some(store), Some(job_id)) = (jobs, primary_job_id) {
            let _ = enqueue_replication_executor_jobs(store, job_id, replication_tier);
        }
    }
}

/// Enqueue M parallel replication executor jobs for strict tier (PH-S536).
pub fn enqueue_replication_executor_jobs(
    jobs: &JobStore,
    primary_job_id: &str,
    tier: ReplicationTierConfig,
) -> Result<usize, AppError> {
    let m = tier.executors_m as usize;
    let mut enqueued = 0usize;
    for i in 0..m {
        let rep_id = format!("{primary_job_id}-rep-{i}");
        if jobs.get(&rep_id)?.is_some() {
            continue;
        }
        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new(rep_id),
                kind: JobKind::System,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: Some("replication_executor".into()),
                deadline: None,
            },
            status: JobStatus::Submitted,
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
        enqueued += 1;
    }
    Ok(enqueued)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replication_strict_metrics_for_test() {
    REPLICATION_STRICT_TOTAL.store(0, Ordering::Relaxed);
    REPLICATION_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    REPLICATION_EXECUTOR_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    REPLICATION_HOURLY_COUNT.store(0, Ordering::Relaxed);
    REPLICATION_RATE_LIMITED_TOTAL.store(0, Ordering::Relaxed);
}

/// Grid job ingest path stub: increment when tier profile is strict (PH-S179).
pub fn evaluate_job_replication_strict(replication_tier: ReplicationTierConfig) {
    record_replication_enqueue();
    if replication_tier.profile == ReplicationProfile::Strict {
        record_replication_strict_ingest();
    }
}

#[cfg(test)]
static REPLICATION_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn replication_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    REPLICATION_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_replication::{
        replication_tier_from_policy, REPLICATION_STANDARD, REPLICATION_STRICT,
    };

    #[test]
    fn evaluate_job_replication_strict_increments_on_strict_ph_s179() {
        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();
        evaluate_job_replication_strict(REPLICATION_STANDARD);
        assert_eq!(replication_strict_total(), 0);
        assert_eq!(replication_enqueue_total(), 1);

        evaluate_job_replication_strict(REPLICATION_STRICT);
        assert_eq!(replication_strict_total(), 1);
        assert_eq!(replication_enqueue_total(), 2);

        evaluate_job_replication_strict(REPLICATION_STANDARD);
        assert_eq!(replication_strict_total(), 1);
        assert_eq!(replication_enqueue_total(), 3);

        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn evaluate_job_replication_strict_matches_policy_resolver_ph_s179() {
        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();

        let tier = replication_tier_from_policy(Some("replication_strict"));
        evaluate_job_replication_strict(tier);
        assert_eq!(replication_strict_total(), 1);

        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn replication_executor_hook_ph_s435() {
        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();
        replication_executor_hook(REPLICATION_STRICT, None, None);
        assert_eq!(replication_executor_enqueue_total(), 1);
        assert_eq!(replication_enqueue_total(), 1);
        assert_eq!(replication_strict_total(), 1);
        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn replication_rate_limit_ph_s457() {
        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();
        let prior = std::env::var(ENV_REPLICATION_MAX_PER_HOUR).ok();
        std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, "1");
        assert!(replication_enqueue_allowed(REPLICATION_STRICT));
        assert!(!replication_enqueue_allowed(REPLICATION_STRICT));
        assert_eq!(replication_rate_limited_total(), 1);
        match prior {
            Some(v) => std::env::set_var(ENV_REPLICATION_MAX_PER_HOUR, v),
            None => std::env::remove_var(ENV_REPLICATION_MAX_PER_HOUR),
        }
        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn enqueue_replication_executor_jobs_ph_s536() {
        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let n = enqueue_replication_executor_jobs(&jobs, "grid-job-1", REPLICATION_STRICT)
            .expect("enqueue");
        assert_eq!(n, 3);
        assert!(jobs.get("grid-job-1-rep-0").expect("get").is_some());
        reset_replication_strict_metrics_for_test();
    }
}
