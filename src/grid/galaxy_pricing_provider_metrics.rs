//! Galaxy pricing provider catalog metrics stub (PH-S172, §4.2.5).
//!
//! Counters when [`GalaxyPricingProviderCatalog::matching_entries`] resolves allow-list hits.

use std::sync::atomic::{AtomicU64, Ordering};

/// Catalog allow-list lookups since process start.
pub const METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL: &str =
    "galaxy_pricing_provider_catalog_lookups_total";

/// Matching provider rows returned across lookups (sum of hit counts).
pub const METRIC_PROVIDER_CATALOG_HITS_TOTAL: &str = "galaxy_pricing_provider_catalog_hits_total";

static LOOKUPS_TOTAL: AtomicU64 = AtomicU64::new(0);
static HITS_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one catalog lookup and how many providers matched the allow-list filters.
pub fn record_provider_catalog_lookup(matching_count: usize) {
    LOOKUPS_TOTAL.fetch_add(1, Ordering::Relaxed);
    HITS_TOTAL.fetch_add(matching_count as u64, Ordering::Relaxed);
}

pub fn provider_catalog_lookups_total() -> u64 {
    LOOKUPS_TOTAL.load(Ordering::Relaxed)
}

pub fn provider_catalog_hits_total() -> u64 {
    HITS_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_provider_catalog_metrics_for_test() {
    LOOKUPS_TOTAL.store(0, Ordering::Relaxed);
    HITS_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_provider_catalog_lookup_increments_counters_ph_s172() {
        reset_provider_catalog_metrics_for_test();
        record_provider_catalog_lookup(2);
        record_provider_catalog_lookup(0);
        assert_eq!(provider_catalog_lookups_total(), 2);
        assert_eq!(provider_catalog_hits_total(), 2);
        reset_provider_catalog_metrics_for_test();
    }
}
