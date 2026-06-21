//! Galaxy locality / hot-tier depth classification stub (PH-S764, §5.2–5.4).

use crate::grid::galaxy_locality_metrics::LocalityMetricsSnapshot;

/// Locality / hot-tier telemetry depth (Galaxy §5.2–5.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalityHotTierDepth {
    None,
    LocalityObserved,
    HotTierPromote,
    HotTierEvict,
    FullDepth,
}

/// Classify locality / hot-tier depth from optional metrics snapshot (PH-S764).
pub fn locality_hot_tier_depth_stub(
    snapshot: Option<&LocalityMetricsSnapshot>,
) -> LocalityHotTierDepth {
    let Some(s) = snapshot else {
        return LocalityHotTierDepth::None;
    };
    let has_ratios = s.shard_local_hit_ratio_bps > 0
        || s.hot_tier_hit_ratio_bps > 0
        || s.cross_region_egress_mb > 0;
    let has_promote = s.hot_promote_total > 0;
    let has_evict = s.hot_evict_total > 0;

    if has_ratios && (has_promote || has_evict) {
        LocalityHotTierDepth::FullDepth
    } else if has_promote {
        LocalityHotTierDepth::HotTierPromote
    } else if has_evict {
        LocalityHotTierDepth::HotTierEvict
    } else if has_ratios {
        LocalityHotTierDepth::LocalityObserved
    } else {
        LocalityHotTierDepth::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locality_hot_tier_depth_stub_ph_s764() {
        assert_eq!(
            locality_hot_tier_depth_stub(None),
            LocalityHotTierDepth::None
        );
        assert_eq!(
            locality_hot_tier_depth_stub(Some(&LocalityMetricsSnapshot {
                shard_local_hit_ratio_bps: 8_000,
                hot_tier_hit_ratio_bps: 0,
                cross_region_egress_mb: 0,
                hot_promote_total: 0,
                hot_evict_total: 0,
            })),
            LocalityHotTierDepth::LocalityObserved
        );
        assert_eq!(
            locality_hot_tier_depth_stub(Some(&LocalityMetricsSnapshot {
                shard_local_hit_ratio_bps: 0,
                hot_tier_hit_ratio_bps: 0,
                cross_region_egress_mb: 0,
                hot_promote_total: 3,
                hot_evict_total: 0,
            })),
            LocalityHotTierDepth::HotTierPromote
        );
        assert_eq!(
            locality_hot_tier_depth_stub(Some(&LocalityMetricsSnapshot {
                shard_local_hit_ratio_bps: 0,
                hot_tier_hit_ratio_bps: 0,
                cross_region_egress_mb: 0,
                hot_promote_total: 0,
                hot_evict_total: 2,
            })),
            LocalityHotTierDepth::HotTierEvict
        );
        assert_eq!(
            locality_hot_tier_depth_stub(Some(&LocalityMetricsSnapshot {
                shard_local_hit_ratio_bps: 10_000,
                hot_tier_hit_ratio_bps: 5_000,
                cross_region_egress_mb: 10,
                hot_promote_total: 1,
                hot_evict_total: 1,
            })),
            LocalityHotTierDepth::FullDepth
        );
    }
}
