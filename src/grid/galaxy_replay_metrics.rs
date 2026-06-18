//! Galaxy Grid replay pending metrics stub (PH-S176, §6.3).
//!
//! Gauge for in-flight replay verifications blocking settlement; no live replay enqueue wire.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_settlement::SettlementStatus;
use crate::grid::galaxy_verification_replay::{
    build_verification_replay_record, GalaxyVerificationReplayRecord,
};

/// Current replay verifications pending coordinator verdict (mirrored on `GET /metrics`).
pub const METRIC_REPLAY_PENDING: &str = "galaxy_replay_pending";

/// Replay holds scheduled on grid result path (PH-S333).
pub const METRIC_REPLAY_PENDING_SCHEDULED_TOTAL: &str = "galaxy_replay_pending_scheduled_total";

/// Replay holds cleared on verdict (PH-S335).
pub const METRIC_REPLAY_PENDING_RESOLVED_TOTAL: &str = "galaxy_replay_pending_resolved_total";

/// Total replay pending evaluations on grid result path (PH-S415 `/metrics` gauge).
pub const METRIC_REPLAY_EVALUATIONS_TOTAL: &str = "galaxy_replay_evaluations_total";

/// Replay verification enqueue stub invocations (PH-S438).
pub const METRIC_REPLAY_VERIFICATION_ENQUEUE_TOTAL: &str =
    "galaxy_replay_verification_enqueue_total";

static REPLAY_PENDING: AtomicU64 = AtomicU64::new(0);
static REPLAY_PENDING_SCHEDULED_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLAY_PENDING_RESOLVED_TOTAL: AtomicU64 = AtomicU64::new(0);
static REPLAY_EVALUATIONS_TOTAL: AtomicU64 = AtomicU64::new(0);
/// Replay verification structured records emitted (PH-S447).
pub const METRIC_VERIFICATION_REPLAY_RECORD_TOTAL: &str = "galaxy_verification_replay_record_total";

static REPLAY_VERIFICATION_ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFICATION_REPLAY_RECORD_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Schedule one replay verification hold (mismatch / explicit flag).
pub fn record_replay_pending_scheduled() {
    REPLAY_PENDING.fetch_add(1, Ordering::Relaxed);
    REPLAY_PENDING_SCHEDULED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

/// Clear one replay verification hold when verdict arrives.
pub fn record_replay_pending_resolved() {
    let _ = REPLAY_PENDING.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
        Some(n.saturating_sub(1))
    });
    REPLAY_PENDING_RESOLVED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replay_pending_scheduled_total() -> u64 {
    REPLAY_PENDING_SCHEDULED_TOTAL.load(Ordering::Relaxed)
}

pub fn replay_pending_resolved_total() -> u64 {
    REPLAY_PENDING_RESOLVED_TOTAL.load(Ordering::Relaxed)
}

pub fn replay_pending() -> u64 {
    REPLAY_PENDING.load(Ordering::Relaxed)
}

/// Total replay pending evaluations since process start (PH-S415).
pub fn replay_evaluations_total() -> u64 {
    REPLAY_EVALUATIONS_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replay_pending_metrics_for_test() {
    REPLAY_PENDING.store(0, Ordering::Relaxed);
    REPLAY_PENDING_SCHEDULED_TOTAL.store(0, Ordering::Relaxed);
    REPLAY_PENDING_RESOLVED_TOTAL.store(0, Ordering::Relaxed);
    REPLAY_EVALUATIONS_TOTAL.store(0, Ordering::Relaxed);
    REPLAY_VERIFICATION_ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    VERIFICATION_REPLAY_RECORD_TOTAL.store(0, Ordering::Relaxed);
}

/// Record one replay verification enqueue stub (PH-S438).
pub fn record_replay_verification_enqueue() {
    REPLAY_VERIFICATION_ENQUEUE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn replay_verification_enqueue_total() -> u64 {
    REPLAY_VERIFICATION_ENQUEUE_TOTAL.load(Ordering::Relaxed)
}

pub fn verification_replay_record_total() -> u64 {
    VERIFICATION_REPLAY_RECORD_TOTAL.load(Ordering::Relaxed)
}

/// Build structured replay record and bump emit counter (PH-S447).
pub fn emit_verification_replay_record(
    primary_job_id: &str,
    metrics: Option<&serde_json::Value>,
) -> GalaxyVerificationReplayRecord {
    let rec = build_verification_replay_record(primary_job_id, metrics);
    VERIFICATION_REPLAY_RECORD_TOTAL.fetch_add(1, Ordering::Relaxed);
    rec
}

/// Enqueue replay verification stub when mismatch/replay flags schedule hold (PH-S438).
pub fn enqueue_replay_verification(
    primary_job_id: &str,
    metrics: Option<&serde_json::Value>,
    settlement_status: SettlementStatus,
) {
    if should_schedule_replay_pending(metrics, settlement_status) {
        record_replay_verification_enqueue();
        let _ = emit_verification_replay_record(primary_job_id, metrics);
    }
}

#[cfg(test)]
static REPLAY_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

/// Serializes tests that mutate the global replay pending gauge.
#[cfg(test)]
pub(crate) fn replay_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    REPLAY_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn replay_verdict_resolved(metrics: Option<&serde_json::Value>) -> bool {
    metrics
        .and_then(|m| m.get("replay_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| {
            let lower = s.to_ascii_lowercase();
            matches!(lower.as_str(), "accepted" | "rejected" | "resolved")
        })
}

fn should_schedule_replay_pending(
    metrics: Option<&serde_json::Value>,
    settlement_status: SettlementStatus,
) -> bool {
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

/// Grid result path stub: track replay pending gauge from metrics + settlement (PH-S176).
pub fn evaluate_result_replay_pending(
    primary_job_id: &str,
    metrics: Option<&serde_json::Value>,
    settlement_status: SettlementStatus,
) {
    REPLAY_EVALUATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    if replay_verdict_resolved(metrics) {
        record_replay_pending_resolved();
        return;
    }
    if should_schedule_replay_pending(metrics, settlement_status) {
        record_replay_pending_scheduled();
        enqueue_replay_verification(primary_job_id, metrics, settlement_status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn evaluate_result_replay_pending_schedules_on_mismatch_ph_s176() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        evaluate_result_replay_pending(
            "job-1",
            Some(&json!({ "verification_verdict": "mismatch" })),
            SettlementStatus::Cleared,
        );
        assert_eq!(replay_pending(), 1);
        assert_eq!(replay_verification_enqueue_total(), 1);
        assert_eq!(verification_replay_record_total(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn record_replay_pending_scheduled_total_ph_s333() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        record_replay_pending_scheduled();
        assert_eq!(replay_pending_scheduled_total(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn record_replay_pending_resolved_total_ph_s335() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        record_replay_pending_scheduled();
        record_replay_pending_resolved();
        assert_eq!(replay_pending_resolved_total(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn evaluate_result_replay_pending_resolves_on_verdict_ph_s176() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        record_replay_pending_scheduled();
        record_replay_pending_scheduled();
        evaluate_result_replay_pending(
            "job-1",
            Some(&json!({ "replay_verdict": "accepted" })),
            SettlementStatus::Cleared,
        );
        assert_eq!(replay_pending(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn replay_evaluations_total_ph_s415() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        evaluate_result_replay_pending(
            "job-1",
            Some(&json!({ "verification_verdict": "mismatch" })),
            SettlementStatus::Cleared,
        );
        evaluate_result_replay_pending(
            "job-1",
            Some(&json!({ "replay_verdict": "accepted" })),
            SettlementStatus::Cleared,
        );
        assert_eq!(replay_evaluations_total(), 2);
        reset_replay_pending_metrics_for_test();
    }
}
