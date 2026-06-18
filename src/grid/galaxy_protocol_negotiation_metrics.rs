//! Protocol negotiation rejection counter (PH-S449, Galaxy §9.8).

use std::sync::atomic::{AtomicU64, Ordering};

/// Unsupported protocol rejections on guarded wire routes (PH-S449).
pub const METRIC_PROTOCOL_NEGOTIATION_REJECTED_TOTAL: &str =
    "poolai_protocol_negotiation_rejected_total";

static PROTOCOL_NEGOTIATION_REJECTED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one unsupported protocol negotiation rejection.
pub fn record_protocol_negotiation_rejected() {
    PROTOCOL_NEGOTIATION_REJECTED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn protocol_negotiation_rejected_total() -> u64 {
    PROTOCOL_NEGOTIATION_REJECTED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_protocol_negotiation_metrics_for_test() {
    PROTOCOL_NEGOTIATION_REJECTED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_protocol_negotiation_rejected_ph_s449() {
        reset_protocol_negotiation_metrics_for_test();
        record_protocol_negotiation_rejected();
        assert_eq!(protocol_negotiation_rejected_total(), 1);
        reset_protocol_negotiation_metrics_for_test();
    }
}
