//! Galaxy Grid verification metrics stubs (PH-S175 / PH-S177, §6.2).
//!
//! Counters for grid result verification sample + mismatch; no live checker wire.

use std::sync::atomic::{AtomicU64, Ordering};

/// In-process counter for verification samples scheduled (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_SAMPLE_TOTAL: &str = "galaxy_verification_sample_total";

/// In-process counter for verification digest mismatches (mirrored on `GET /metrics`).
pub const METRIC_VERIFICATION_MISMATCH_TOTAL: &str = "galaxy_verification_mismatch_total";

static SAMPLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static MISMATCH_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one verification sample on the grid result path.
pub fn record_verification_sample() {
    SAMPLE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn verification_sample_total() -> u64 {
    SAMPLE_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_sample_metrics_for_test() {
    SAMPLE_TOTAL.store(0, Ordering::Relaxed);
}

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

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verification_metrics_for_test() {
    reset_verification_sample_metrics_for_test();
    reset_verification_mismatch_metrics_for_test();
}

/// Grid result path stub: increment sample counter when stub selects edge sample or explicit flag.
pub fn evaluate_result_verification_sample(
    metrics: Option<&serde_json::Value>,
    sample_scheduled: bool,
) -> bool {
    let explicit = metrics
        .and_then(|m| m.get("verification_sample"))
        .and_then(|v| v.as_bool())
        == Some(true);
    let scheduled = sample_scheduled || explicit;
    if scheduled {
        record_verification_sample();
    }
    scheduled
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
static VERIFICATION_METRICS_TEST_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
    std::sync::OnceLock::new();

#[cfg(test)]
pub(crate) fn verification_metrics_test_lock() -> std::sync::MutexGuard<'static, ()> {
    VERIFICATION_METRICS_TEST_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn evaluate_result_verification_sample_increments_on_scheduled_ph_s177() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(!evaluate_result_verification_sample(None, false));
        assert_eq!(verification_sample_total(), 0);

        assert!(evaluate_result_verification_sample(None, true));
        assert_eq!(verification_sample_total(), 1);

        assert!(!evaluate_result_verification_sample(None, false));
        assert_eq!(verification_sample_total(), 1);

        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_sample_increments_on_explicit_flag_ph_s177() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
        assert!(evaluate_result_verification_sample(
            Some(&json!({ "verification_sample": true })),
            false,
        ));
        assert_eq!(verification_sample_total(), 1);
        reset_verification_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verification_mismatch_increments_counter_ph_s175() {
        let _lock = verification_metrics_test_lock();
        reset_verification_metrics_for_test();
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

        reset_verification_metrics_for_test();
    }
}
