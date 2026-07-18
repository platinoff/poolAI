//! Replication quorum digest gate on grid result path (PH-S545, Galaxy §6.4).
//!
//! Collects `executor_digest` from result metrics and blocks Cleared settlement when
//! strict-tier K-of-M quorum is not met.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::grid::galaxy_replication::{
    replication_quorum_met, replication_tier_from_policy, ReplicationProfile,
};

fn digests_map() -> &'static Mutex<HashMap<String, Vec<String>>> {
    static EXECUTOR_DIGESTS: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();
    EXECUTOR_DIGESTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Record one executor digest from grid result metrics (`executor_digest` field).
pub fn record_result_executor_digest(job_id: &str, metrics: Option<&serde_json::Value>) {
    let Some(digest) = metrics
        .and_then(|m| m.get("executor_digest"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return;
    };
    if let Ok(mut map) = digests_map().lock() {
        map.entry(job_id.to_string())
            .or_default()
            .push(digest.to_string());
    }
}

/// Whether Cleared settlement may proceed for replication-tier jobs (PH-S545).
pub fn replication_quorum_allows_cleared(job_id: &str, verification_policy: Option<&str>) -> bool {
    let tier = replication_tier_from_policy(verification_policy);
    if tier.profile != ReplicationProfile::Strict {
        return true;
    }
    let digests_owned = match digests_map().lock() {
        Ok(map) => map.get(job_id).cloned().unwrap_or_default(),
        Err(_) => return true,
    };
    if digests_owned.is_empty() {
        return true;
    }
    let refs: Vec<&str> = digests_owned.iter().map(String::as_str).collect();
    replication_quorum_met(&refs, tier)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_replication_quorum_gate_for_test() {
    if let Ok(mut map) = digests_map().lock() {
        map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_replication::REPLICATION_STRICT;

    #[test]
    fn quorum_gate_blocks_mismatch_ph_s545() {
        reset_replication_quorum_gate_for_test();
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "a"})));
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "a"})));
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "b"})));
        assert!(!replication_quorum_allows_cleared(
            "job-1",
            Some("replication_strict")
        ));
        reset_replication_quorum_gate_for_test();
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "a"})));
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "a"})));
        record_result_executor_digest("job-1", Some(&serde_json::json!({"executor_digest": "a"})));
        assert!(replication_quorum_allows_cleared(
            "job-1",
            Some("replication_strict")
        ));
        assert_eq!(REPLICATION_STRICT.quorum_k, 3);
        reset_replication_quorum_gate_for_test();
    }
}
