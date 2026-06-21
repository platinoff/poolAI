//! Galaxy locality / hot-tier metrics snapshot for HTTP wire (PH-S760, §5.2–5.4).

use crate::grid::galaxy_locality::{
    last_cross_region_egress_mb, last_hot_tier_hit_ratio_bps, last_shard_local_hit_ratio_bps,
};
use crate::grid::galaxy_prefetch_metrics::{hot_evict_total, hot_promote_total};

/// Read-only locality + hot-tier counters snapshot for `GET /api/v1/grid/locality-metrics` (PH-S760).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct LocalityMetricsSnapshot {
    pub shard_local_hit_ratio_bps: u64,
    pub hot_tier_hit_ratio_bps: u64,
    pub cross_region_egress_mb: u64,
    pub hot_promote_total: u64,
    pub hot_evict_total: u64,
}

/// Coordinator locality metrics snapshot (PH-S760).
pub fn locality_metrics_snapshot() -> LocalityMetricsSnapshot {
    LocalityMetricsSnapshot {
        shard_local_hit_ratio_bps: last_shard_local_hit_ratio_bps(),
        hot_tier_hit_ratio_bps: last_hot_tier_hit_ratio_bps(),
        cross_region_egress_mb: last_cross_region_egress_mb(),
        hot_promote_total: hot_promote_total(),
        hot_evict_total: hot_evict_total(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_locality::{
        observe_last_cross_region_egress_mb, observe_last_hot_tier_hit_ratio,
        observe_last_shard_local_hit_ratio, reset_last_cross_region_egress_mb_for_test,
        reset_last_hot_tier_hit_ratio_for_test, reset_last_shard_local_hit_ratio_for_test,
    };
    use crate::grid::galaxy_prefetch_metrics::{
        record_hot_evict, record_hot_promote, reset_prefetch_metrics_for_test,
    };

    #[test]
    fn locality_metrics_snapshot_ph_s760() {
        reset_last_shard_local_hit_ratio_for_test();
        reset_last_hot_tier_hit_ratio_for_test();
        reset_last_cross_region_egress_mb_for_test();
        reset_prefetch_metrics_for_test();

        observe_last_shard_local_hit_ratio(0.75);
        observe_last_hot_tier_hit_ratio(0.5);
        observe_last_cross_region_egress_mb(42.0);
        record_hot_promote(2);
        record_hot_evict(1);

        let snap = locality_metrics_snapshot();
        assert_eq!(snap.shard_local_hit_ratio_bps, 7_500);
        assert_eq!(snap.hot_tier_hit_ratio_bps, 5_000);
        assert_eq!(snap.cross_region_egress_mb, 42);
        assert_eq!(snap.hot_promote_total, 2);
        assert_eq!(snap.hot_evict_total, 1);

        reset_last_shard_local_hit_ratio_for_test();
        reset_last_hot_tier_hit_ratio_for_test();
        reset_last_cross_region_egress_mb_for_test();
        reset_prefetch_metrics_for_test();
    }
}
