//! Galaxy Grid settlement metrics stub (PH-S178 / PH-S187, §6.4).
//!
//! Counters for grid results held in `pending_verification` or cleared for settlement;
//! no live payout wire.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_settlement::SettlementStatus;

/// In-process counter for settlement holds pending verification (mirrored on `GET /metrics`).
pub const METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL: &str =
    "galaxy_settlement_pending_verification_total";

/// In-process counter for settlement cleared on grid result path (mirrored on `GET /metrics`).
pub const METRIC_SETTLEMENT_CLEARED_TOTAL: &str = "galaxy_settlement_cleared_total";

/// In-process counter for settlement not applicable on grid result path (PH-S354).
pub const METRIC_SETTLEMENT_NOT_APPLICABLE_TOTAL: &str = "galaxy_settlement_not_applicable_total";

/// Total settlement status resolutions on grid result path (PH-S404 `/metrics` gauge).
pub const METRIC_SETTLEMENT_RESOLVED_TOTAL: &str = "galaxy_settlement_resolved_total";

/// Offline payout batch ledger entries on cleared settlement (PH-S427 stub).
pub const METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL: &str = "galaxy_settlement_payout_batch_total";

static PENDING_VERIFICATION_TOTAL: AtomicU64 = AtomicU64::new(0);
static CLEARED_TOTAL: AtomicU64 = AtomicU64::new(0);
static NOT_APPLICABLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static RESOLVED_TOTAL: AtomicU64 = AtomicU64::new(0);
static PAYOUT_BATCH_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one grid result held in pending verification settlement.
pub fn record_settlement_pending_verification() {
    PENDING_VERIFICATION_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn settlement_pending_verification_total() -> u64 {
    PENDING_VERIFICATION_TOTAL.load(Ordering::Relaxed)
}

/// Record one grid result with settlement cleared.
pub fn record_settlement_cleared() {
    CLEARED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn settlement_cleared_total() -> u64 {
    CLEARED_TOTAL.load(Ordering::Relaxed)
}

/// Record one grid result with settlement not applicable (local origin stub).
pub fn record_settlement_not_applicable() {
    NOT_APPLICABLE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn settlement_not_applicable_total() -> u64 {
    NOT_APPLICABLE_TOTAL.load(Ordering::Relaxed)
}

/// Record one grid result settlement resolution (any status).
pub fn record_settlement_resolved() {
    RESOLVED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn settlement_resolved_total() -> u64 {
    RESOLVED_TOTAL.load(Ordering::Relaxed)
}

/// Record one offline payout batch ledger entry (PH-S427).
pub fn record_settlement_payout_batch() {
    PAYOUT_BATCH_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn settlement_payout_batch_total() -> u64 {
    PAYOUT_BATCH_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_resolved_metrics_for_test() {
    RESOLVED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_not_applicable_metrics_for_test() {
    NOT_APPLICABLE_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_pending_verification_metrics_for_test() {
    PENDING_VERIFICATION_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_cleared_metrics_for_test() {
    CLEARED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_metrics_for_test() {
    reset_settlement_pending_verification_metrics_for_test();
    reset_settlement_cleared_metrics_for_test();
    reset_settlement_not_applicable_metrics_for_test();
    reset_settlement_resolved_metrics_for_test();
    PAYOUT_BATCH_TOTAL.store(0, Ordering::Relaxed);
}

/// Grid result path stub: increment on every settlement resolution (PH-S404).
pub fn evaluate_result_settlement_resolved(_settlement_status: SettlementStatus) {
    record_settlement_resolved();
}

/// Grid result path stub: increment when settlement resolves to pending verification (PH-S178).
pub fn evaluate_result_settlement_pending_verification(settlement_status: SettlementStatus) {
    if settlement_status == SettlementStatus::PendingVerification {
        record_settlement_pending_verification();
    }
}

/// Grid result path stub: increment when settlement resolves to cleared (PH-S187).
pub fn evaluate_result_settlement_cleared(settlement_status: SettlementStatus) {
    if settlement_status == SettlementStatus::Cleared {
        record_settlement_cleared();
        record_settlement_payout_batch();
    }
}

/// Grid result path stub: increment when settlement is not applicable (PH-S354).
pub fn evaluate_result_settlement_not_applicable(settlement_status: SettlementStatus) {
    if settlement_status == SettlementStatus::NotApplicable {
        record_settlement_not_applicable();
    }
}

#[cfg(test)]
static SETTLEMENT_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn settlement_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    SETTLEMENT_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_trust_score::SettlementGateVerdict;
    use crate::grid::galaxy_verify_sampling::VerifySamplingVerdict;

    #[test]
    fn evaluate_result_settlement_pending_verification_increments_on_hold_ph_s178() {
        let _lock = settlement_metrics_test_lock();
        reset_settlement_pending_verification_metrics_for_test();
        evaluate_result_settlement_pending_verification(SettlementStatus::Cleared);
        assert_eq!(settlement_pending_verification_total(), 0);

        evaluate_result_settlement_pending_verification(SettlementStatus::PendingVerification);
        assert_eq!(settlement_pending_verification_total(), 1);

        evaluate_result_settlement_pending_verification(SettlementStatus::NotApplicable);
        assert_eq!(settlement_pending_verification_total(), 1);

        reset_settlement_pending_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_settlement_pending_verification_matches_resolve_status_ph_s178() {
        use crate::grid::galaxy_settlement::resolve_settlement_status;

        let _lock = settlement_metrics_test_lock();
        reset_settlement_pending_verification_metrics_for_test();

        let status = resolve_settlement_status(
            SettlementGateVerdict::PayoutHeld,
            VerifySamplingVerdict::NotSelected,
        );
        evaluate_result_settlement_pending_verification(status);
        assert_eq!(settlement_pending_verification_total(), 1);

        reset_settlement_pending_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_settlement_cleared_increments_on_cleared_ph_s187() {
        let _lock = settlement_metrics_test_lock();
        reset_settlement_metrics_for_test();
        evaluate_result_settlement_cleared(SettlementStatus::PendingVerification);
        assert_eq!(settlement_cleared_total(), 0);

        evaluate_result_settlement_cleared(SettlementStatus::Cleared);
        assert_eq!(settlement_cleared_total(), 1);
        assert_eq!(settlement_payout_batch_total(), 1);

        evaluate_result_settlement_cleared(SettlementStatus::NotApplicable);
        assert_eq!(settlement_cleared_total(), 1);
        assert_eq!(settlement_payout_batch_total(), 1);

        reset_settlement_metrics_for_test();
    }

    #[test]
    fn evaluate_result_settlement_cleared_matches_resolve_status_ph_s187() {
        use crate::grid::galaxy_settlement::resolve_settlement_status;

        let _lock = settlement_metrics_test_lock();
        reset_settlement_cleared_metrics_for_test();

        let status = resolve_settlement_status(
            SettlementGateVerdict::PayoutEligible,
            VerifySamplingVerdict::NotSelected,
        );
        evaluate_result_settlement_cleared(status);
        assert_eq!(settlement_cleared_total(), 1);

        reset_settlement_cleared_metrics_for_test();
    }

    #[test]
    fn evaluate_result_settlement_resolved_increments_ph_s404() {
        let _lock = settlement_metrics_test_lock();
        reset_settlement_resolved_metrics_for_test();
        evaluate_result_settlement_resolved(SettlementStatus::Cleared);
        evaluate_result_settlement_resolved(SettlementStatus::PendingVerification);
        assert_eq!(settlement_resolved_total(), 2);
        reset_settlement_resolved_metrics_for_test();
    }

    #[test]
    fn evaluate_result_settlement_not_applicable_increments_ph_s354() {
        let _lock = settlement_metrics_test_lock();
        reset_settlement_not_applicable_metrics_for_test();
        evaluate_result_settlement_not_applicable(SettlementStatus::Cleared);
        assert_eq!(settlement_not_applicable_total(), 0);

        evaluate_result_settlement_not_applicable(SettlementStatus::NotApplicable);
        assert_eq!(settlement_not_applicable_total(), 1);

        reset_settlement_not_applicable_metrics_for_test();
    }
}
