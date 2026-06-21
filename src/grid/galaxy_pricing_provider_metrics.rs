//! Galaxy pricing provider catalog metrics (PH-S172, PH-S173, §4.2.5).
//!
//! Counters when [`GalaxyPricingProviderCatalog::matching_entries`] resolves allow-list hits
//! and when live provider HTTP fetch fails in [`fetch_live_provider_quotes`].

use std::sync::atomic::{AtomicU64, Ordering};

/// Catalog allow-list lookups since process start.
pub const METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL: &str =
    "galaxy_pricing_provider_catalog_lookups_total";

/// Matching provider rows returned across lookups (sum of hit counts).
pub const METRIC_PROVIDER_CATALOG_HITS_TOTAL: &str = "galaxy_pricing_provider_catalog_hits_total";

/// Live provider HTTP fetch failures (network, non-2xx, parse, missing unit) — PH-S173.
pub const METRIC_PROVIDER_ERRORS_TOTAL: &str = "galaxy_pricing_provider_errors_total";

/// Live provider HTTP fetch timeouts (connect or request) — PH-S900.
pub const METRIC_PROVIDER_TIMEOUTS_TOTAL: &str = "galaxy_pricing_provider_timeouts_total";

static LOOKUPS_TOTAL: AtomicU64 = AtomicU64::new(0);
static HITS_TOTAL: AtomicU64 = AtomicU64::new(0);
static ERRORS_TOTAL: AtomicU64 = AtomicU64::new(0);
static TIMEOUTS_TOTAL: AtomicU64 = AtomicU64::new(0);

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

/// Record one failed live provider fetch attempt for a catalog entry.
pub fn record_provider_fetch_error() {
    ERRORS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn provider_errors_total() -> u64 {
    ERRORS_TOTAL.load(Ordering::Relaxed)
}

/// Record one timed-out live provider fetch attempt (PH-S900).
pub fn record_provider_fetch_timeout() {
    TIMEOUTS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn provider_timeouts_total() -> u64 {
    TIMEOUTS_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_provider_catalog_metrics_for_test() {
    LOOKUPS_TOTAL.store(0, Ordering::Relaxed);
    HITS_TOTAL.store(0, Ordering::Relaxed);
    ERRORS_TOTAL.store(0, Ordering::Relaxed);
    TIMEOUTS_TOTAL.store(0, Ordering::Relaxed);
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

    #[test]
    fn record_provider_fetch_error_increments_counter_ph_s173() {
        reset_provider_catalog_metrics_for_test();
        record_provider_fetch_error();
        record_provider_fetch_error();
        assert_eq!(provider_errors_total(), 2);
        reset_provider_catalog_metrics_for_test();
    }

    #[test]
    fn record_provider_fetch_timeout_increments_counter_ph_s900() {
        reset_provider_catalog_metrics_for_test();
        record_provider_fetch_timeout();
        assert_eq!(provider_timeouts_total(), 1);
        reset_provider_catalog_metrics_for_test();
    }
}
