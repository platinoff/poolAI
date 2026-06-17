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

/// Prefetch enqueue hook invocations (shard items enqueued, PH-S283 stub).
pub const METRIC_PREFETCH_ENQUEUE_TOTAL: &str = "galaxy_prefetch_enqueue_total";

/// Prefetch wait stub milliseconds (deadline × planned shards, PH-S293).
pub const METRIC_PREFETCH_WAIT_MS_TOTAL: &str = "galaxy_prefetch_wait_ms_total";

/// Prefetch plans under strict locality mode (PH-S303 stub).
pub const METRIC_PREFETCH_STRICT_MODE_TOTAL: &str = "galaxy_prefetch_strict_mode_total";

/// Prefetch complete hook invocations (PH-S307 stub).
pub const METRIC_PREFETCH_COMPLETE_TOTAL: &str = "galaxy_prefetch_complete_total";

/// Grid job ingest prefetch stub invocations (PH-S313 stub).
pub const METRIC_PREFETCH_INGEST_TOTAL: &str = "galaxy_prefetch_ingest_total";

/// Prefetch ingest skipped when job has no required shards (PH-S323 stub).
pub const METRIC_PREFETCH_SKIP_INGEST_TOTAL: &str = "galaxy_prefetch_skip_ingest_total";

/// Default stub bytes per RAM-tier planned shard (4 MiB, Galaxy §5.5).
pub const DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM: u64 = 4_194_304;

/// Default stub bytes per VRAM-tier planned shard (8 MiB, Galaxy §5.5 GPU path).
pub const DEFAULT_PREFETCH_BYTES_PER_SHARD_VRAM: u64 = 8_388_608;

static PLAN_TOTAL: AtomicU64 = AtomicU64::new(0);
static PLANNED_SHARDS_TOTAL: AtomicU64 = AtomicU64::new(0);
static HOT_SKIP_TOTAL: AtomicU64 = AtomicU64::new(0);
static PREFETCH_BYTES_TOTAL: AtomicU64 = AtomicU64::new(0);
static ENQUEUE_TOTAL: AtomicU64 = AtomicU64::new(0);
static WAIT_MS_TOTAL: AtomicU64 = AtomicU64::new(0);
static STRICT_MODE_TOTAL: AtomicU64 = AtomicU64::new(0);
static COMPLETE_TOTAL: AtomicU64 = AtomicU64::new(0);
static INGEST_TOTAL: AtomicU64 = AtomicU64::new(0);
static SKIP_INGEST_TOTAL: AtomicU64 = AtomicU64::new(0);

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

/// Record prefetch enqueue stub (PH-S283; no live seed pull wire).
pub fn record_prefetch_enqueue(shard_count: usize) {
    if shard_count > 0 {
        ENQUEUE_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn prefetch_enqueue_total() -> u64 {
    ENQUEUE_TOTAL.load(Ordering::Relaxed)
}

/// Record prefetch wait stub (PH-S293; no live seed pull wire).
pub fn record_prefetch_wait(planned_shards: usize, deadline_ms: u64) {
    if planned_shards > 0 && deadline_ms > 0 {
        WAIT_MS_TOTAL.fetch_add(
            deadline_ms.saturating_mul(planned_shards as u64),
            Ordering::Relaxed,
        );
    }
}

pub fn prefetch_wait_ms_total() -> u64 {
    WAIT_MS_TOTAL.load(Ordering::Relaxed)
}

/// Record strict locality prefetch plan (PH-S303).
pub fn record_prefetch_strict_mode() {
    STRICT_MODE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_strict_mode_total() -> u64 {
    STRICT_MODE_TOTAL.load(Ordering::Relaxed)
}

/// Record prefetch complete hook (enqueue+wait path, PH-S307).
pub fn record_prefetch_complete(planned_shards: usize) {
    if planned_shards > 0 {
        COMPLETE_TOTAL.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn prefetch_complete_total() -> u64 {
    COMPLETE_TOTAL.load(Ordering::Relaxed)
}

/// Record grid job ingest prefetch stub (PH-S313).
pub fn record_prefetch_ingest() {
    INGEST_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_ingest_total() -> u64 {
    INGEST_TOTAL.load(Ordering::Relaxed)
}

/// Record prefetch ingest skip on empty `required_shard_ids` (PH-S323).
pub fn record_prefetch_skip_ingest() {
    SKIP_INGEST_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_skip_ingest_total() -> u64 {
    SKIP_INGEST_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_prefetch_metrics_for_test() {
    PLAN_TOTAL.store(0, Ordering::Relaxed);
    PLANNED_SHARDS_TOTAL.store(0, Ordering::Relaxed);
    HOT_SKIP_TOTAL.store(0, Ordering::Relaxed);
    PREFETCH_BYTES_TOTAL.store(0, Ordering::Relaxed);
    ENQUEUE_TOTAL.store(0, Ordering::Relaxed);
    WAIT_MS_TOTAL.store(0, Ordering::Relaxed);
    STRICT_MODE_TOTAL.store(0, Ordering::Relaxed);
    COMPLETE_TOTAL.store(0, Ordering::Relaxed);
    INGEST_TOTAL.store(0, Ordering::Relaxed);
    SKIP_INGEST_TOTAL.store(0, Ordering::Relaxed);
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
    }

    #[test]
    fn record_prefetch_enqueue_ph_s283() {
        reset_prefetch_metrics_for_test();
        record_prefetch_enqueue(2);
        assert_eq!(prefetch_enqueue_total(), 2);
        record_prefetch_enqueue(0);
        assert_eq!(prefetch_enqueue_total(), 2);
    }

    #[test]
    fn record_prefetch_wait_ph_s293() {
        reset_prefetch_metrics_for_test();
        record_prefetch_wait(2, 15_000);
        assert_eq!(prefetch_wait_ms_total(), 30_000);
        record_prefetch_wait(0, 15_000);
        assert_eq!(prefetch_wait_ms_total(), 30_000);
    }

    #[test]
    fn record_prefetch_strict_mode_ph_s303() {
        reset_prefetch_metrics_for_test();
        record_prefetch_strict_mode();
        assert_eq!(prefetch_strict_mode_total(), 1);
    }

    #[test]
    fn record_prefetch_complete_ph_s307() {
        reset_prefetch_metrics_for_test();
        record_prefetch_complete(2);
        assert_eq!(prefetch_complete_total(), 1);
        record_prefetch_complete(0);
        assert_eq!(prefetch_complete_total(), 1);
    }

    #[test]
    fn record_prefetch_ingest_ph_s313() {
        reset_prefetch_metrics_for_test();
        record_prefetch_ingest();
        assert_eq!(prefetch_ingest_total(), 1);
    }

    #[test]
    fn record_prefetch_skip_ingest_ph_s323() {
        reset_prefetch_metrics_for_test();
        record_prefetch_skip_ingest();
        assert_eq!(prefetch_skip_ingest_total(), 1);
    }
}
