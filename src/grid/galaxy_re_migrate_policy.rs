//! Galaxy Grid re-migrate policy depth stub (PH-S720, §4.3).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Re-migrate prefetch depth classification (Galaxy §4.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReMigratePolicyDepth {
    None,
    DeltaFetch,
    PrefetchComplete,
}

/// Classify re-migrate depth from optional grid job metrics stub fields (PH-S720).
pub fn re_migrate_policy_depth_stub(metrics: Option<&Value>) -> ReMigratePolicyDepth {
    let Some(m) = metrics else {
        return ReMigratePolicyDepth::None;
    };
    if m.get("re_migrate_prefetch_complete")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return ReMigratePolicyDepth::PrefetchComplete;
    }
    if m.get("re_migrate_delta_fetch")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return ReMigratePolicyDepth::DeltaFetch;
    }
    match m
        .get("re_migrate_shard_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0)
    {
        0 => ReMigratePolicyDepth::None,
        1 => ReMigratePolicyDepth::DeltaFetch,
        _ => ReMigratePolicyDepth::PrefetchComplete,
    }
}

/// Map prefetch hook shard count to re-migrate policy depth (PH-S720 dispatch hook).
#[inline]
pub fn re_migrate_policy_depth_from_shard_count(shard_count: usize) -> ReMigratePolicyDepth {
    re_migrate_policy_depth_stub(Some(&serde_json::json!({
        "re_migrate_shard_count": shard_count as u64,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn re_migrate_policy_depth_stub_ph_s720() {
        assert_eq!(
            re_migrate_policy_depth_stub(Some(&json!({"re_migrate_delta_fetch": true}))),
            ReMigratePolicyDepth::DeltaFetch
        );
        assert_eq!(
            re_migrate_policy_depth_stub(Some(&json!({"re_migrate_prefetch_complete": true}))),
            ReMigratePolicyDepth::PrefetchComplete
        );
        assert_eq!(
            re_migrate_policy_depth_stub(Some(&json!({"re_migrate_shard_count": 3}))),
            ReMigratePolicyDepth::PrefetchComplete
        );
        assert_eq!(
            re_migrate_policy_depth_stub(None),
            ReMigratePolicyDepth::None
        );
    }

    #[test]
    fn re_migrate_policy_depth_from_shard_count_ph_s720() {
        assert_eq!(
            re_migrate_policy_depth_from_shard_count(0),
            ReMigratePolicyDepth::None
        );
        assert_eq!(
            re_migrate_policy_depth_from_shard_count(1),
            ReMigratePolicyDepth::DeltaFetch
        );
        assert_eq!(
            re_migrate_policy_depth_from_shard_count(4),
            ReMigratePolicyDepth::PrefetchComplete
        );
    }
}
