//! Galaxy Grid prefetch plan metrics stub (PH-S167).
//!
//! Counters for [`crate::grid::dispatch::plan_prefetch`] (Galaxy §5.5); no enqueue wire.
//! Prefetch bytes total stub (PH-S184).

use std::sync::atomic::{AtomicU64, Ordering};

/// Prefetch plans computed since process start.
pub const METRIC_PREFETCH_PLAN_TOTAL: &str = "galaxy_prefetch_plan_total";

/// Shards scheduled for prefetch (sum of plan item counts).
pub const METRIC_PREFETCH_PLANNED_SHARDS_TOTAL: &str = "galaxy_prefetch_planned_shards_total";

/// Shards skipped because already hot in inventory.
pub const METRIC_PREFETCH_HOT_SKIP_TOTAL: &str = "galaxy_prefetch_hot_skip_total";

/// Estimated prefetch bytes scheduled in plans since process start (PH-S184 stub).
pub const METRIC_PREFETCH_BYTES_TOTAL: &str = "galaxy_prefetch_bytes_total";

/// Default stub bytes per RAM-tier planned shard (4 MiB, Galaxy §5.5).
pub const DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM: u64 = 4_194_304;

/// Default stub bytes per VRAM-tier planned shard (8 MiB, Galaxy §5.5 GPU path).
pub const DEFAULT_PREFETCH_BYTES_PER_SHARD_VRAM: u64 = 8_388_608;

static PLAN_TOTAL: AtomicU64 = AtomicU64::new(0);
static PLANNED_SHARDS_TOTAL: AtomicU64 = AtomicU64::new(0);
static HOT_SKIP_TOTAL: AtomicU64 = AtomicU64::new(0);
static PREFETCH_BYTES_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one `plan_prefetch` outcome (no wire enqueue).
pub fn record_prefetch_plan(required_shards: usize, planned_shards: usize, prefetch_bytes: u64) {
    PLAN_TOTAL.fetch_add(1, Ordering::Relaxed);
    PLANNED_SHARDS_TOTAL.fetch_add(planned_shards as u64, Ordering::Relaxed);
    HOT_SKIP_TOTAL.fetch_add(
        required_shards.saturating_sub(planned_shards) as u64,
        Ordering::Relaxed,
    );
    PREFETCH_BYTES_TOTAL.fetch_add(prefetch_bytes, Ordering::Relaxed);
}

pub fn prefetch_plan_total() -> u64 {
    PLAN_TOTAL.load(Ordering::Relaxed)
}

pub fn prefetch_planned_shards_total() -> u64 {
    PLANNED_SHARDS_TOTAL.load(Ordering::Relaxed)
}

pub fn prefetch_hot_skip_total() -> u64 {
    HOT_SKIP_TOTAL.load(Ordering::Relaxed)
}

pub fn prefetch_bytes_total() -> u64 {
    PREFETCH_BYTES_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_prefetch_metrics_for_test() {
    PLAN_TOTAL.store(0, Ordering::Relaxed);
    PLANNED_SHARDS_TOTAL.store(0, Ordering::Relaxed);
    HOT_SKIP_TOTAL.store(0, Ordering::Relaxed);
    PREFETCH_BYTES_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_prefetch_plan_increments_counters() {
        reset_prefetch_metrics_for_test();
        record_prefetch_plan(3, 2, 8_388_608);
        record_prefetch_plan(1, 1, 4_194_304);
        assert_eq!(prefetch_plan_total(), 2);
        assert_eq!(prefetch_planned_shards_total(), 3);
        assert_eq!(prefetch_hot_skip_total(), 1);
        assert_eq!(prefetch_bytes_total(), 12_582_912);
        reset_prefetch_metrics_for_test();
    }
}
