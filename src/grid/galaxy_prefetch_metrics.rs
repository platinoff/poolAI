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

/// Seed pull hook invocations on prefetch complete path (PH-S424 stub).
pub const METRIC_PREFETCH_SEED_PULL_TOTAL: &str = "galaxy_prefetch_seed_pull_total";

/// Prefetch plans triggered by lease acquire (PH-S425 stub).
pub const METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL: &str = "galaxy_prefetch_lease_acquired_total";

/// Memory-layer seed fetch hits on prefetch path (PH-S444).
pub const METRIC_PREFETCH_SEED_FETCH_TOTAL: &str = "galaxy_prefetch_seed_fetch_total";

/// Memory-layer seed fetch misses on prefetch path (PH-S444).
pub const METRIC_PREFETCH_SEED_FETCH_MISS_TOTAL: &str = "galaxy_prefetch_seed_fetch_miss_total";

/// Co-access graph speculative prefetch plans (PH-S446).
pub const METRIC_PREFETCH_CO_ACCESS_TOTAL: &str = "galaxy_prefetch_co_access_total";

/// Strict locality ingest rejections (PH-S445).
pub const METRIC_LOCALITY_UNSATISFIED_TOTAL: &str = "galaxy_locality_unsatisfied_total";

/// Re-migrate prefetch plans on Migrating→Leased handoff (PH-S454).
pub const METRIC_PREFETCH_RE_MIGRATE_TOTAL: &str = "galaxy_prefetch_re_migrate_total";

/// Hot tier shard promotions on prefetch complete (PH-S458).
pub const METRIC_HOT_PROMOTE_TOTAL: &str = "galaxy_hot_promote_total";

/// Hot tier shard evictions when skipped in plan (PH-S458).
pub const METRIC_HOT_EVICT_TOTAL: &str = "galaxy_hot_evict_total";

/// Shard access events on prefetch path (Galaxy §5.3, PH-S459).
pub const METRIC_SHARD_ACCESS_TOTAL: &str = "galaxy_shard_access_total";

/// Prefetch queue depth gauge stub (Galaxy §5.3, PH-S459).
pub const METRIC_PREFETCH_QUEUE_DEPTH: &str = "galaxy_prefetch_queue_depth";

/// Prefetch enqueue skipped due to bandwidth backpressure (PH-S464).
pub const METRIC_PREFETCH_BACKPRESSURE_TOTAL: &str = "galaxy_prefetch_backpressure_total";

/// RAID artifact prefetch fetch hits (PH-S465).
pub const METRIC_PREFETCH_RAID_FETCH_TOTAL: &str = "galaxy_prefetch_raid_fetch_total";

/// RAID artifact prefetch fetch misses (PH-S465).
pub const METRIC_PREFETCH_RAID_FETCH_MISS_TOTAL: &str = "galaxy_prefetch_raid_fetch_miss_total";

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
static SEED_PULL_TOTAL: AtomicU64 = AtomicU64::new(0);
static LEASE_ACQUIRED_TOTAL: AtomicU64 = AtomicU64::new(0);
static SEED_FETCH_TOTAL: AtomicU64 = AtomicU64::new(0);
static SEED_FETCH_MISS_TOTAL: AtomicU64 = AtomicU64::new(0);
static CO_ACCESS_TOTAL: AtomicU64 = AtomicU64::new(0);
static LOCALITY_UNSATISFIED_TOTAL: AtomicU64 = AtomicU64::new(0);
static RE_MIGRATE_TOTAL: AtomicU64 = AtomicU64::new(0);
static HOT_PROMOTE_TOTAL: AtomicU64 = AtomicU64::new(0);
static HOT_EVICT_TOTAL: AtomicU64 = AtomicU64::new(0);
static SHARD_ACCESS_TOTAL: AtomicU64 = AtomicU64::new(0);
static PREFETCH_QUEUE_DEPTH: AtomicU64 = AtomicU64::new(0);
static BACKPRESSURE_TOTAL: AtomicU64 = AtomicU64::new(0);
static RAID_FETCH_TOTAL: AtomicU64 = AtomicU64::new(0);
static RAID_FETCH_MISS_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Record one `plan_prefetch` outcome (no wire enqueue).
pub fn record_prefetch_plan(required_shards: usize, planned_shards: usize, prefetch_bytes: u64) {
    PLAN_TOTAL.fetch_add(1, Ordering::Relaxed);
    PLANNED_SHARDS_TOTAL.fetch_add(planned_shards as u64, Ordering::Relaxed);
    let skipped = required_shards.saturating_sub(planned_shards);
    HOT_SKIP_TOTAL.fetch_add(skipped as u64, Ordering::Relaxed);
    if skipped > 0 {
        record_hot_evict(skipped);
    }
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

/// Record seed pull stub on prefetch complete hook (PH-S424).
pub fn record_prefetch_seed_pull(shard_count: usize) {
    if shard_count > 0 {
        SEED_PULL_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn prefetch_seed_pull_total() -> u64 {
    SEED_PULL_TOTAL.load(Ordering::Relaxed)
}

/// Record lease-acquired prefetch trigger (PH-S425).
pub fn record_prefetch_lease_acquired() {
    LEASE_ACQUIRED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_lease_acquired_total() -> u64 {
    LEASE_ACQUIRED_TOTAL.load(Ordering::Relaxed)
}

/// Record memory-layer seed fetch hits (PH-S444).
pub fn record_prefetch_seed_fetch(shard_count: usize) {
    if shard_count > 0 {
        SEED_FETCH_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn prefetch_seed_fetch_total() -> u64 {
    SEED_FETCH_TOTAL.load(Ordering::Relaxed)
}

/// Record memory-layer seed fetch misses (PH-S444).
pub fn record_prefetch_seed_fetch_miss() {
    SEED_FETCH_MISS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_seed_fetch_miss_total() -> u64 {
    SEED_FETCH_MISS_TOTAL.load(Ordering::Relaxed)
}

/// Record co-access speculative prefetch plan (PH-S446).
pub fn record_prefetch_co_access() {
    CO_ACCESS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_co_access_total() -> u64 {
    CO_ACCESS_TOTAL.load(Ordering::Relaxed)
}

/// Record strict locality ingest rejection (PH-S445).
pub fn record_locality_unsatisfied() {
    LOCALITY_UNSATISFIED_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn locality_unsatisfied_total() -> u64 {
    LOCALITY_UNSATISFIED_TOTAL.load(Ordering::Relaxed)
}

/// Record re-migrate prefetch trigger (PH-S454).
pub fn record_prefetch_re_migrate() {
    RE_MIGRATE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_re_migrate_total() -> u64 {
    RE_MIGRATE_TOTAL.load(Ordering::Relaxed)
}

/// Record hot tier promotions (PH-S458).
pub fn record_hot_promote(shard_count: usize) {
    if shard_count > 0 {
        HOT_PROMOTE_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn hot_promote_total() -> u64 {
    HOT_PROMOTE_TOTAL.load(Ordering::Relaxed)
}

/// Record hot tier evictions when shards skipped (PH-S458).
pub fn record_hot_evict(shard_count: usize) {
    if shard_count > 0 {
        HOT_EVICT_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn hot_evict_total() -> u64 {
    HOT_EVICT_TOTAL.load(Ordering::Relaxed)
}

/// Record shard access on prefetch path (PH-S459).
pub fn record_shard_access(shard_count: usize) {
    if shard_count > 0 {
        SHARD_ACCESS_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn shard_access_total() -> u64 {
    SHARD_ACCESS_TOTAL.load(Ordering::Relaxed)
}

/// Observe prefetch queue depth gauge (PH-S459).
pub fn observe_prefetch_queue_depth(depth: u64) {
    PREFETCH_QUEUE_DEPTH.store(depth, Ordering::Relaxed);
}

pub fn prefetch_queue_depth() -> u64 {
    PREFETCH_QUEUE_DEPTH.load(Ordering::Relaxed)
}

/// Record prefetch enqueue skipped by bandwidth backpressure (PH-S464).
pub fn record_prefetch_backpressure() {
    BACKPRESSURE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_backpressure_total() -> u64 {
    BACKPRESSURE_TOTAL.load(Ordering::Relaxed)
}

/// Record RAID-layer seed fetch hits (PH-S465).
pub fn record_prefetch_raid_fetch(shard_count: usize) {
    if shard_count > 0 {
        RAID_FETCH_TOTAL.fetch_add(shard_count as u64, Ordering::Relaxed);
    }
}

pub fn prefetch_raid_fetch_total() -> u64 {
    RAID_FETCH_TOTAL.load(Ordering::Relaxed)
}

/// Record RAID-layer seed fetch misses (PH-S465).
pub fn record_prefetch_raid_fetch_miss() {
    RAID_FETCH_MISS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn prefetch_raid_fetch_miss_total() -> u64 {
    RAID_FETCH_MISS_TOTAL.load(Ordering::Relaxed)
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
    SEED_PULL_TOTAL.store(0, Ordering::Relaxed);
    LEASE_ACQUIRED_TOTAL.store(0, Ordering::Relaxed);
    SEED_FETCH_TOTAL.store(0, Ordering::Relaxed);
    SEED_FETCH_MISS_TOTAL.store(0, Ordering::Relaxed);
    CO_ACCESS_TOTAL.store(0, Ordering::Relaxed);
    LOCALITY_UNSATISFIED_TOTAL.store(0, Ordering::Relaxed);
    RE_MIGRATE_TOTAL.store(0, Ordering::Relaxed);
    HOT_PROMOTE_TOTAL.store(0, Ordering::Relaxed);
    HOT_EVICT_TOTAL.store(0, Ordering::Relaxed);
    SHARD_ACCESS_TOTAL.store(0, Ordering::Relaxed);
    PREFETCH_QUEUE_DEPTH.store(0, Ordering::Relaxed);
    BACKPRESSURE_TOTAL.store(0, Ordering::Relaxed);
    RAID_FETCH_TOTAL.store(0, Ordering::Relaxed);
    RAID_FETCH_MISS_TOTAL.store(0, Ordering::Relaxed);
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

    #[test]
    fn record_prefetch_seed_pull_ph_s424() {
        reset_prefetch_metrics_for_test();
        record_prefetch_seed_pull(2);
        assert_eq!(prefetch_seed_pull_total(), 2);
        record_prefetch_seed_pull(0);
        assert_eq!(prefetch_seed_pull_total(), 2);
    }

    #[test]
    fn record_prefetch_lease_acquired_ph_s425() {
        reset_prefetch_metrics_for_test();
        record_prefetch_lease_acquired();
        assert_eq!(prefetch_lease_acquired_total(), 1);
    }

    #[test]
    fn record_prefetch_re_migrate_ph_s454() {
        reset_prefetch_metrics_for_test();
        record_prefetch_re_migrate();
        assert_eq!(prefetch_re_migrate_total(), 1);
    }

    #[test]
    fn hot_promote_evict_ph_s458() {
        reset_prefetch_metrics_for_test();
        record_hot_promote(2);
        record_hot_evict(1);
        assert_eq!(hot_promote_total(), 2);
        assert_eq!(hot_evict_total(), 1);
    }

    #[test]
    fn shard_access_and_queue_depth_ph_s459() {
        reset_prefetch_metrics_for_test();
        record_shard_access(3);
        observe_prefetch_queue_depth(5);
        assert_eq!(shard_access_total(), 3);
        assert_eq!(prefetch_queue_depth(), 5);
    }
}
