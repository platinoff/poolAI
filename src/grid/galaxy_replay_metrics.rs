//! Galaxy Grid replay pending metrics stub (PH-S176, §6.3).
//!
//! Gauge for in-flight replay verifications blocking settlement; no live replay enqueue wire.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_settlement::SettlementStatus;

/// Current replay verifications pending coordinator verdict (mirrored on `GET /metrics`).
pub const METRIC_REPLAY_PENDING: &str = "galaxy_replay_pending";

static REPLAY_PENDING: AtomicU64 = AtomicU64::new(0);

/// Schedule one replay verification hold (mismatch / explicit flag).
pub fn record_replay_pending_scheduled() {
    REPLAY_PENDING.fetch_add(1, Ordering::Relaxed);
}

/// Clear one replay verification hold when verdict arrives.
pub fn record_replay_pending_resolved() {
    let _ = REPLAY_PENDING.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
        Some(n.saturating_sub(1))
    });
}

pub fn replay_pending() -> u64 {
    REPLAY_PENDING.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replay_pending_metrics_for_test() {
    REPLAY_PENDING.store(0, Ordering::Relaxed);
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
    metrics: Option<&serde_json::Value>,
    settlement_status: SettlementStatus,
) {
    if replay_verdict_resolved(metrics) {
        record_replay_pending_resolved();
        return;
    }
    if should_schedule_replay_pending(metrics, settlement_status) {
        record_replay_pending_scheduled();
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
            Some(&json!({ "verification_verdict": "mismatch" })),
            SettlementStatus::Cleared,
        );
        assert_eq!(replay_pending(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn evaluate_result_replay_pending_resolves_on_verdict_ph_s176() {
        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        record_replay_pending_scheduled();
        record_replay_pending_scheduled();
        evaluate_result_replay_pending(
            Some(&json!({ "replay_verdict": "accepted" })),
            SettlementStatus::Cleared,
        );
        assert_eq!(replay_pending(), 1);
        reset_replay_pending_metrics_for_test();
    }
}
