//! Galaxy pricing oracle production depth classification (PH-S904, §4.2).

use crate::grid::galaxy_pricing_metrics::PricingMetricsSnapshot;
use crate::grid::galaxy_pricing_oracle::provider_http_timeout_ms_from_env;

/// Pricing oracle wire depth (Galaxy §4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PricingDepth {
    None,
    L2Fallback,
    L1Cache,
    LiveFetch,
    FullProduction,
}

/// Classify pricing depth from metrics snapshot + configured timeout (PH-S904).
pub fn pricing_depth_stub(
    snapshot: Option<&PricingMetricsSnapshot>,
    provider_timeout_ms: u64,
) -> PricingDepth {
    let Some(s) = snapshot else {
        return PricingDepth::None;
    };
    let has_l2 = s.forced_fallback_total > 0;
    let has_l1 = s.fresh_served_total > 0 || s.stale_served_total > 0;
    let has_live = s.provider_catalog_lookups_total > 0 || s.provider_errors_total > 0;
    let timeout_configured =
        provider_timeout_ms >= crate::grid::galaxy_pricing_oracle::MIN_PROVIDER_HTTP_TIMEOUT_MS;

    if has_l2 && has_l1 && has_live && timeout_configured {
        PricingDepth::FullProduction
    } else if has_live {
        PricingDepth::LiveFetch
    } else if has_l1 {
        PricingDepth::L1Cache
    } else if has_l2 {
        PricingDepth::L2Fallback
    } else {
        PricingDepth::None
    }
}

/// Wire label for pricing-metrics / stand smoke (PH-S904).
pub fn pricing_depth_wire_label(depth: PricingDepth) -> &'static str {
    match depth {
        PricingDepth::None => "none",
        PricingDepth::L2Fallback => "l2_fallback",
        PricingDepth::L1Cache => "l1_cache",
        PricingDepth::LiveFetch => "live_fetch",
        PricingDepth::FullProduction => "full_production",
    }
}

/// Runtime pricing depth from in-process counters.
pub fn current_pricing_depth() -> PricingDepth {
    pricing_depth_stub(
        Some(&crate::grid::galaxy_pricing_metrics::pricing_metrics_snapshot()),
        provider_http_timeout_ms_from_env(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_pricing_metrics::pricing_metrics_snapshot;
    use crate::grid::galaxy_pricing_oracle::{
        bump_forced_fallback_for_test, bump_fresh_served_for_test,
        reset_forced_fallback_total_for_test, reset_fresh_served_total_for_test,
    };
    use crate::grid::galaxy_pricing_provider_metrics::{
        record_provider_catalog_lookup, reset_provider_catalog_metrics_for_test,
    };

    #[test]
    fn pricing_depth_stub_ph_s904() {
        reset_fresh_served_total_for_test();
        reset_forced_fallback_total_for_test();
        reset_provider_catalog_metrics_for_test();

        let empty = pricing_metrics_snapshot();
        assert_eq!(pricing_depth_stub(None, 1500), PricingDepth::None);
        assert_eq!(pricing_depth_stub(Some(&empty), 1500), PricingDepth::None);

        bump_forced_fallback_for_test();
        let l2 = pricing_metrics_snapshot();
        assert_eq!(
            pricing_depth_stub(Some(&l2), 1500),
            PricingDepth::L2Fallback
        );
        reset_forced_fallback_total_for_test();

        bump_fresh_served_for_test();
        let l1 = pricing_metrics_snapshot();
        assert_eq!(pricing_depth_stub(Some(&l1), 1500), PricingDepth::L1Cache);
        reset_fresh_served_total_for_test();

        record_provider_catalog_lookup(1);
        let live = pricing_metrics_snapshot();
        assert_eq!(
            pricing_depth_stub(Some(&live), 1500),
            PricingDepth::LiveFetch
        );
        reset_provider_catalog_metrics_for_test();

        bump_forced_fallback_for_test();
        bump_fresh_served_for_test();
        record_provider_catalog_lookup(1);
        let full = pricing_metrics_snapshot();
        assert_eq!(
            pricing_depth_stub(Some(&full), 1500),
            PricingDepth::FullProduction
        );
        assert_eq!(
            pricing_depth_wire_label(PricingDepth::FullProduction),
            "full_production"
        );

        reset_fresh_served_total_for_test();
        reset_forced_fallback_total_for_test();
        reset_provider_catalog_metrics_for_test();
    }
}
