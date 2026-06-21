//! Signed capability admission counters (PH-S740, Galaxy §6.6).

use std::sync::atomic::{AtomicU64, Ordering};

/// Unsigned or invalid signed capability rejections on `telegram_edge` register-remote (PH-S740).
pub const METRIC_CAPABILITY_UNSIGNED_REJECTED_TOTAL: &str =
    "galaxy_capability_unsigned_rejected_total";

/// Successful signed capability admissions on `telegram_edge` register-remote (PH-S741).
pub const METRIC_CAPABILITY_SIGNED_ACCEPTED_TOTAL: &str = "galaxy_capability_signed_accepted_total";

static CAPABILITY_UNSIGNED_REJECTED_TOTAL: AtomicU64 = AtomicU64::new(0);
static CAPABILITY_SIGNED_ACCEPTED_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn record_capability_unsigned_rejected() {
    CAPABILITY_UNSIGNED_REJECTED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn capability_unsigned_rejected_total() -> u64 {
    CAPABILITY_UNSIGNED_REJECTED_TOTAL.load(Ordering::Relaxed)
}

pub fn record_capability_signed_accepted() {
    CAPABILITY_SIGNED_ACCEPTED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn capability_signed_accepted_total() -> u64 {
    CAPABILITY_SIGNED_ACCEPTED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_capability_admission_metrics_for_test() {
    CAPABILITY_UNSIGNED_REJECTED_TOTAL.store(0, Ordering::Relaxed);
    CAPABILITY_SIGNED_ACCEPTED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_admission_metrics_ph_s740() {
        reset_capability_admission_metrics_for_test();
        record_capability_unsigned_rejected();
        record_capability_signed_accepted();
        assert_eq!(capability_unsigned_rejected_total(), 1);
        assert_eq!(capability_signed_accepted_total(), 1);
        reset_capability_admission_metrics_for_test();
    }
}
