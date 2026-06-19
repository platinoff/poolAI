//! Security advisory acknowledge audit wire (PH-S573, Galaxy §9.6).
//! Read-only advisory list stub (PH-S586).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// Metric: operator security advisory acknowledgements (PH-S573).
pub const METRIC_ADVISORY_ACKNOWLEDGED_TOTAL: &str = "poolai_advisory_acknowledged_total";

/// Operator-facing advisory row (PH-S586 list stub).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityAdvisoryEntry {
    pub id: String,
    pub severity: String,
    pub summary: String,
    pub acknowledged: bool,
}

static ADVISORY_ACK_TOTAL: AtomicU64 = AtomicU64::new(0);

static ACKNOWLEDGED_IDS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Canonical stub advisories for ops UI (Galaxy §9.6); acknowledgement state is live.
pub fn list_security_advisories() -> Vec<SecurityAdvisoryEntry> {
    [
        (
            "CVE-2026-0001",
            "medium",
            "Signed release manifest rotation advisory (Galaxy §9.2)",
        ),
        (
            "key_transition-2026-q2",
            "low",
            "Maintainer trust-root key transition window (Galaxy §9.3)",
        ),
        (
            "protocol_sunset-1.0",
            "high",
            "Protocol 1.0 sunset — upgrade workers to 1.2.x (Galaxy §9.3)",
        ),
    ]
    .into_iter()
    .map(|(id, severity, summary)| SecurityAdvisoryEntry {
        id: id.to_string(),
        severity: severity.to_string(),
        summary: summary.to_string(),
        acknowledged: is_advisory_acknowledged(id),
    })
    .collect()
}

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
    fn advisory_list_ph_s586() {
        reset_security_advisory_for_test();
        let rows = list_security_advisories();
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().all(|r| !r.acknowledged));
        assert!(acknowledge_security_advisory("CVE-2026-0001"));
        let rows = list_security_advisories();
        assert!(rows
            .iter()
            .find(|r| r.id == "CVE-2026-0001")
            .is_some_and(|r| r.acknowledged));
        reset_security_advisory_for_test();
    }

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
