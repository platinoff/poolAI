//! Galaxy fraud-proof horizon stub (PH-S571, §6.6 TBD).
//!
//! When `POOLAI_GALAXY_FRAUD_PROOF=1`, verification mismatch increments
//! `galaxy_fraud_proof_pending_total` and holds settlement for proof review.

use std::sync::atomic::{AtomicU64, Ordering};

/// Env: enable fraud-proof pending hold on verification mismatch (PH-S571).
pub const ENV_FRAUD_PROOF: &str = "POOLAI_GALAXY_FRAUD_PROOF";

/// Metric: settlement rows held for fraud-proof review (PH-S571).
pub const METRIC_FRAUD_PROOF_PENDING_TOTAL: &str = "galaxy_fraud_proof_pending_total";

static FRAUD_PROOF_PENDING_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn fraud_proof_enabled() -> bool {
    matches!(
        std::env::var(ENV_FRAUD_PROOF)
            .ok()
            .map(|v| v.trim().to_ascii_lowercase())
            .as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}

pub fn fraud_proof_pending_total() -> u64 {
    FRAUD_PROOF_PENDING_TOTAL.load(Ordering::Relaxed)
}

pub fn record_fraud_proof_pending() {
    FRAUD_PROOF_PENDING_TOTAL.fetch_add(1, Ordering::Relaxed);
}

/// Returns true when mismatch should enter fraud-proof hold (PH-S571).
pub fn evaluate_fraud_proof_hold(is_mismatch: bool) -> bool {
    is_mismatch && fraud_proof_enabled()
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_fraud_proof_metrics_for_test() {
    FRAUD_PROOF_PENDING_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraud_proof_hold_ph_s571() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_fraud_proof_metrics_for_test();
        std::env::remove_var(ENV_FRAUD_PROOF);
        assert!(!evaluate_fraud_proof_hold(true));
        std::env::set_var(ENV_FRAUD_PROOF, "1");
        assert!(evaluate_fraud_proof_hold(true));
        record_fraud_proof_pending();
        assert_eq!(fraud_proof_pending_total(), 1);
        std::env::remove_var(ENV_FRAUD_PROOF);
        reset_fraud_proof_metrics_for_test();
    }
}
