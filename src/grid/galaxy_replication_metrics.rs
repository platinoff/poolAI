//! Galaxy Grid replication metrics stub (PH-S179, §6.3).
//!
//! Counter for grid jobs ingested with `replication_strict` tier; no live executor enqueue wire.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_replication::{ReplicationProfile, ReplicationTierConfig};

/// In-process counter for strict-tier grid job ingests (mirrored on `GET /metrics`).
pub const METRIC_REPLICATION_STRICT_TOTAL: &str = "galaxy_replication_strict_total";

/// Executor enqueue stub invocations on grid job ingest (PH-S426).
pub const METRIC_REPLICATION_ENQUEUE_TOTAL: &str = "galaxy_replication_enqueue_total";

/// Replication executor queue stub invocations (PH-S435).
pub const METRIC_REPLICATION_EXECUTOR_ENQUEUE_TOTAL: &str =
    "galaxy_replication_executor_enqueue_total";

static REPLICATION_STRICT_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLICATION_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLICATION_EXECUTOR_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);

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

/// Grid job ingest executor queue stub (PH-S435).
pub fn replication_executor_hook(replication_tier: ReplicationTierConfig) {
    record_replication_executor_enqueue();
    evaluate_job_replication_strict(replication_tier);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replication_strict_metrics_for_test() {
    REPLICATION_STRICT_TOTAL.store(0, Ordering::Relaxed);
    REPLICATION_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    REPLICATION_EXECUTOR_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
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
        replication_executor_hook(REPLICATION_STRICT);
        assert_eq!(replication_executor_enqueue_total(), 1);
        assert_eq!(replication_enqueue_total(), 1);
        assert_eq!(replication_strict_total(), 1);
        reset_replication_strict_metrics_for_test();
    }
}
