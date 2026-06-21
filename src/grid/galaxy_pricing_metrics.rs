//! Galaxy pricing oracle metrics snapshot (PH-S691, §4.2).

use crate::grid::galaxy_pricing_oracle::{
    forced_fallback_total, fresh_served_total, stale_served_total,
};
use crate::grid::galaxy_pricing_provider_metrics::{
    provider_catalog_hits_total, provider_catalog_lookups_total, provider_errors_total,
    provider_timeouts_total,
};

/// Read-only pricing oracle counters snapshot for `GET /api/v1/grid/pricing-metrics` (PH-S691).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PricingMetricsSnapshot {
    pub fresh_served_total: u64,
    pub stale_served_total: u64,
    pub forced_fallback_total: u64,
    pub provider_catalog_lookups_total: u64,
    pub provider_catalog_hits_total: u64,
    pub provider_errors_total: u64,
    pub provider_timeouts_total: u64,
}

/// Coordinator pricing metrics snapshot (PH-S691).
pub fn pricing_metrics_snapshot() -> PricingMetricsSnapshot {
    PricingMetricsSnapshot {
        fresh_served_total: fresh_served_total(),
        stale_served_total: stale_served_total(),
        forced_fallback_total: forced_fallback_total(),
        provider_catalog_lookups_total: provider_catalog_lookups_total(),
        provider_catalog_hits_total: provider_catalog_hits_total(),
        provider_errors_total: provider_errors_total(),
        provider_timeouts_total: provider_timeouts_total(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_pricing_oracle::{
        bump_fresh_served_for_test, reset_forced_fallback_total_for_test,
        reset_fresh_served_total_for_test, reset_stale_served_total_for_test,
    };
    use crate::grid::galaxy_pricing_provider_metrics::reset_provider_catalog_metrics_for_test;

    #[test]
    fn pricing_metrics_snapshot_reflects_counters_ph_s691() {
        reset_fresh_served_total_for_test();
        reset_stale_served_total_for_test();
        reset_forced_fallback_total_for_test();
        reset_provider_catalog_metrics_for_test();
        bump_fresh_served_for_test();
        let snap = pricing_metrics_snapshot();
        assert_eq!(snap.fresh_served_total, 1);
        reset_fresh_served_total_for_test();
    }
}
