//! Galaxy Grid verification mismatch metrics stub (PH-S175, §6.2).
//!
//! Counter when grid result `metrics.verification_verdict` is `mismatch`; no live checker wire.

use std::sync::atomic::{AtomicU64, Ordering};

/// In-process counter for verification digest mismatches (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_MISMATCH_TOTAL: &str = "galaxy_verification_mismatch_total";

static MISMATCH_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one verification mismatch on the grid result path.
pub fn record_verification_mismatch() {
    MISMATCH_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_mismatch_total() -> u64 {
    MISMATCH_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_mismatch_metrics_for_test() {
    MISMATCH_TOTAL.store(0, Ordering::Relaxed);
}

/// Grid result path stub: read optional `metrics.verification_verdict`; increment on `mismatch`.
pub fn evaluate_result_verification_mismatch(metrics: Option<&serde_json::Value>) -> bool {
    let is_mismatch = metrics
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("mismatch"));
    if is_mismatch {
        record_verification_mismatch();
    }
    is_mismatch
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn evaluate_result_verification_mismatch_increments_counter_ph_s175() {
        reset_verification_mismatch_metrics_for_test();
        assert!(!evaluate_result_verification_mismatch(None));
        assert_eq!(verification_mismatch_total(), 0);

        assert!(evaluate_result_verification_mismatch(Some(&json!({
            "verification_verdict": "mismatch"
        }))));
        assert_eq!(verification_mismatch_total(), 1);

        assert!(!evaluate_result_verification_mismatch(Some(&json!({
            "verification_verdict": "match"
        }))));
        assert_eq!(verification_mismatch_total(), 1);

        reset_verification_mismatch_metrics_for_test();
    }
}
