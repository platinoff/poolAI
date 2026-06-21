//! Galaxy governance ops Prometheus stubs (PH-S528, Galaxy §9.8).

use std::sync::atomic::{AtomicU64, Ordering};

/// Successful `poolai-verify-release` runs (PH-S528).
pub const METRIC_RELEASE_VERIFY_TOTAL: &str = "poolai_release_verify_total";

/// Failed `poolai-verify-release` runs (PH-S528).
pub const METRIC_RELEASE_VERIFY_FAIL_TOTAL: &str = "poolai_release_verify_fail_total";

/// Pending opt-in update notifications (Galaxy §9.8 stub gauge).
pub const METRIC_UPDATE_NOTIFY_PENDING: &str = "poolai_update_notify_pending";

static RELEASE_VERIFY_TOTAL: AtomicU64 = AtomicU64::new(0);
static RELEASE_VERIFY_FAIL_TOTAL: AtomicU64 = AtomicU64::new(0);
static UPDATE_NOTIFY_PENDING: AtomicU64 = AtomicU64::new(0);

pub fn release_verify_total() -> u64 {
    RELEASE_VERIFY_TOTAL.load(Ordering::Relaxed)
}

pub fn release_verify_fail_total() -> u64 {
    RELEASE_VERIFY_FAIL_TOTAL.load(Ordering::Relaxed)
}

pub fn update_notify_pending() -> u64 {
    UPDATE_NOTIFY_PENDING.load(Ordering::Relaxed)
}

pub fn record_release_verify_success() {
    RELEASE_VERIFY_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn record_release_verify_fail() {
    RELEASE_VERIFY_FAIL_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn set_update_notify_pending(count: u64) {
    UPDATE_NOTIFY_PENDING.store(count, Ordering::Relaxed);
}

/// Read-only governance ops snapshot for `GET /api/v1/grid/governance-metrics` (PH-S791).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct GovernanceMetricsSnapshot {
    pub release_verify_total: u64,
    pub release_verify_fail_total: u64,
    pub update_notify_pending: u64,
    pub advisory_acknowledged_total: u64,
}

/// Coordinator governance metrics snapshot (PH-S791).
pub fn governance_metrics_snapshot() -> GovernanceMetricsSnapshot {
    GovernanceMetricsSnapshot {
        release_verify_total: release_verify_total(),
        release_verify_fail_total: release_verify_fail_total(),
        update_notify_pending: update_notify_pending(),
        advisory_acknowledged_total:
            crate::grid::galaxy_security_advisory::advisory_acknowledged_total(),
    }
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_governance_metrics_for_test() {
    RELEASE_VERIFY_TOTAL.store(0, Ordering::Relaxed);
    RELEASE_VERIFY_FAIL_TOTAL.store(0, Ordering::Relaxed);
    UPDATE_NOTIFY_PENDING.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governance_metrics_ph_s528() {
        reset_governance_metrics_for_test();
        record_release_verify_success();
        record_release_verify_fail();
        assert_eq!(release_verify_total(), 1);
        assert_eq!(release_verify_fail_total(), 1);
        set_update_notify_pending(2);
        assert_eq!(update_notify_pending(), 2);
        reset_governance_metrics_for_test();
    }
}
