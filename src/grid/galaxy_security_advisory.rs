//! Security advisory acknowledge audit wire (PH-S573, Galaxy §9.6).

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// Metric: operator security advisory acknowledgements (PH-S573).
pub const METRIC_ADVISORY_ACKNOWLEDGED_TOTAL: &str = "poolai_advisory_acknowledged_total";

static ADVISORY_ACK_TOTAL: AtomicU64 = AtomicU64::new(0);

static ACKNOWLEDGED_IDS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub fn advisory_acknowledged_total() -> u64 {
    ADVISORY_ACK_TOTAL.load(Ordering::Relaxed)
}

/// Record advisory acknowledgement; returns false when already acknowledged.
pub fn acknowledge_security_advisory(advisory_id: &str) -> bool {
    let id = advisory_id.trim();
    if id.is_empty() {
        return false;
    }
    let mut guard = ACKNOWLEDGED_IDS.lock().unwrap_or_else(|e| e.into_inner());
    if !guard.insert(id.to_string()) {
        return false;
    }
    ADVISORY_ACK_TOTAL.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn is_advisory_acknowledged(advisory_id: &str) -> bool {
    ACKNOWLEDGED_IDS
        .lock()
        .ok()
        .is_some_and(|g| g.contains(advisory_id.trim()))
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_security_advisory_for_test() {
    ADVISORY_ACK_TOTAL.store(0, Ordering::Relaxed);
    if let Ok(mut g) = ACKNOWLEDGED_IDS.lock() {
        g.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advisory_ack_ph_s573() {
        reset_security_advisory_for_test();
        assert!(acknowledge_security_advisory("CVE-2026-0001"));
        assert_eq!(advisory_acknowledged_total(), 1);
        assert!(!acknowledge_security_advisory("CVE-2026-0001"));
        assert!(is_advisory_acknowledged("CVE-2026-0001"));
        reset_security_advisory_for_test();
    }
}
