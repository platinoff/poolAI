//! Galaxy Grid fee split metrics stub (PH-S194, §4.1).
//!
//! Counter when `split_gross_payment` runs on grid result ingest (metrics wire stub).

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_fee_split::{
    split_gross_payment, PRIMARY_DEV_FEE_BPS, SECONDARY_ADMIN_FEE_MAX_BPS,
    SECONDARY_ADMIN_FEE_MIN_BPS,
};

/// In-process counter for fee splits applied on grid result path (mirrored on `GET /metrics`).
pub const METRIC_FEE_SPLIT_APPLIED_TOTAL: &str = "galaxy_fee_split_applied_total";

static FEE_SPLIT_APPLIED_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn record_fee_split_applied() {
    FEE_SPLIT_APPLIED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn fee_split_applied_total() -> u64 {
    FEE_SPLIT_APPLIED_TOTAL.load(Ordering::Relaxed)
}

/// Read-only fee split counters snapshot for `GET /api/v1/grid/fee-split-metrics` (PH-S780).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct FeeSplitMetricsSnapshot {
    pub fee_split_applied_total: u64,
    pub primary_dev_fee_bps: u16,
    pub secondary_admin_fee_min_bps: u16,
    pub secondary_admin_fee_max_bps: u16,
}

/// Coordinator fee split metrics snapshot (PH-S780).
pub fn fee_split_metrics_snapshot() -> FeeSplitMetricsSnapshot {
    FeeSplitMetricsSnapshot {
        fee_split_applied_total: fee_split_applied_total(),
        primary_dev_fee_bps: PRIMARY_DEV_FEE_BPS,
        secondary_admin_fee_min_bps: SECONDARY_ADMIN_FEE_MIN_BPS,
        secondary_admin_fee_max_bps: SECONDARY_ADMIN_FEE_MAX_BPS,
    }
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_fee_split_metrics_for_test() {
    FEE_SPLIT_APPLIED_TOTAL.store(0, Ordering::Relaxed);
}

fn fee_split_wire_from_metrics(metrics: Option<&serde_json::Value>) -> Option<(u64, u16)> {
    let m = metrics?;
    let gross = m.get("gross_lamports")?.as_u64()?;
    let bps = m.get("secondary_admin_bps")?.as_u64()?;
    if bps > u64::from(u16::MAX) {
        return None;
    }
    let bps = bps as u16;
    if !(SECONDARY_ADMIN_FEE_MIN_BPS..=SECONDARY_ADMIN_FEE_MAX_BPS).contains(&bps) {
        return None;
    }
    Some((gross, bps))
}

/// Grid result path stub: apply fee split when gross + secondary bps are present (PH-S194).
pub fn evaluate_result_fee_split(metrics: Option<&serde_json::Value>) {
    let Some((gross, bps)) = fee_split_wire_from_metrics(metrics) else {
        return;
    };
    if split_gross_payment(gross, bps).is_ok() {
        record_fee_split_applied();
    }
}

#[cfg(test)]
static FEE_SPLIT_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn fee_split_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    FEE_SPLIT_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn evaluate_result_fee_split_increments_on_valid_wire_ph_s194() {
        let _lock = fee_split_metrics_test_lock();
        reset_fee_split_metrics_for_test();
        evaluate_result_fee_split(None);
        assert_eq!(fee_split_applied_total(), 0);

        evaluate_result_fee_split(Some(
            &json!({ "gross_lamports": 1_000_000, "secondary_admin_bps": 99 }),
        ));
        assert_eq!(fee_split_applied_total(), 0);

        evaluate_result_fee_split(Some(
            &json!({ "gross_lamports": 1_000_000, "secondary_admin_bps": 200 }),
        ));
        assert_eq!(fee_split_applied_total(), 1);

        evaluate_result_fee_split(Some(
            &json!({ "gross_lamports": 0, "secondary_admin_bps": 100 }),
        ));
        assert_eq!(fee_split_applied_total(), 2);

        reset_fee_split_metrics_for_test();
    }

    #[test]
    fn fee_split_metrics_snapshot_ph_s780() {
        let _lock = fee_split_metrics_test_lock();
        reset_fee_split_metrics_for_test();
        record_fee_split_applied();
        let snap = fee_split_metrics_snapshot();
        assert_eq!(snap.fee_split_applied_total, 1);
        assert_eq!(snap.primary_dev_fee_bps, PRIMARY_DEV_FEE_BPS);
        reset_fee_split_metrics_for_test();
    }
}
