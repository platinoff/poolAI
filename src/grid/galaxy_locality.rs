//! Galaxy Grid locality placement stub (PH-S128): `locality_score(worker, task)` pure fn
//! and scheduler ranking stub per `docs/concept/POOLAI_GALAXY_GRID.md` §5.1–5.2.
//! Stale `network_profile` penalty stub (PH-S169, §8.1). No prefetch wire (PH-S129).
//! Last observed `shard_local_hit_ratio` gauge on rank path (PH-S183).
//! Last observed `cross_region_egress_mb` gauge on rank/prefetch path (PH-S185).

use std::sync::atomic::{AtomicU64, Ordering};

/// Network profile locality subset (Galaxy §8.1; full wire adds bandwidth/egress/topology).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalityNetworkProfile {
    pub region: String,
    pub latency_ms_p50: u32,
    /// Seconds since `last_measured_at`; `None` = missing freshness probe (stale, §8.1).
    pub profile_age_secs: Option<u64>,
}

/// Hot tier cache on a worker (concept §5.4).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalityHotTier {
    pub ram_bytes_used: u64,
    pub vram_bytes_used: u64,
    pub profiles: Vec<String>,
}

/// Worker seed inventory subset (concept §5.2 wire extension).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalitySeedInventory {
    pub shard_ids: Vec<String>,
    pub hot_tier: LocalityHotTier,
    pub local_replica_regions: Vec<String>,
}

/// Worker inputs for locality scoring (off-chain coordinator stub).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalityWorker {
    pub worker_id: String,
    pub seed_inventory: LocalitySeedInventory,
    pub network_profile: LocalityNetworkProfile,
    /// Worker queue depth for scheduler tie-break (PH-S548, Galaxy §5.2).
    pub queue_depth: u32,
    /// Optional pricing quote in usd_micro for tie-break after locality score (PH-S548).
    pub pricing_usd_micro: Option<u64>,
}

/// Task / job inputs for locality scoring.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalityTask {
    pub required_shard_ids: Vec<String>,
    pub task_profile: String,
    /// Job-level WAN egress estimate in MB (concept §5.2).
    pub estimated_cross_region_egress_mb: f64,
    /// Source region for cross-region penalty; when absent, egress term is 0.
    pub source_region: Option<String>,
}

/// Tunable weights for the placement score (concept §5.2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalityWeights {
    pub w_shard: f64,
    pub w_lat: f64,
    pub w_hot: f64,
    pub w_egress: f64,
}

/// Default weights aligned with Galaxy §5.2 signal priorities.
pub const DEFAULT_LOCALITY_WEIGHTS: LocalityWeights = LocalityWeights {
    w_shard: 0.40,
    w_lat: 0.30,
    w_hot: 0.30,
    w_egress: 0.01,
};

/// Stale `network_profile` threshold (Galaxy §8.1): >24h since `last_measured_at`.
pub const DEFAULT_STALE_PROFILE_MAX_AGE_SECS: u64 = 86_400;

/// Score deduction when profile freshness is missing or older than [`DEFAULT_STALE_PROFILE_MAX_AGE_SECS`].
pub const DEFAULT_STALE_PROFILE_PENALTY: f64 = 0.05;

/// Last observed shard local hit ratio in basis points 0..=10_000 (PH-S183 `/metrics` gauge).
pub const METRIC_SHARD_LOCAL_HIT_RATIO: &str = "galaxy_shard_local_hit_ratio";

/// Last observed cross-region egress in whole MB (PH-S185 `/metrics` gauge).
pub const METRIC_CROSS_REGION_EGRESS_MB: &str = "galaxy_cross_region_egress_mb";

/// Locality rank invocations on grid job ingest (PH-S295 stub).
pub const METRIC_LOCALITY_RANK_INGEST_TOTAL: &str = "galaxy_locality_rank_ingest_total";

/// Locality rank misses on grid job ingest (PH-S305 stub).
pub const METRIC_LOCALITY_RANK_MISS_TOTAL: &str = "galaxy_locality_rank_miss_total";

/// Locality rank with empty worker inventory on ingest (PH-S315 stub).
pub const METRIC_LOCALITY_RANK_EMPTY_WORKERS_TOTAL: &str =
    "galaxy_locality_rank_empty_workers_total";

/// Locality rank skipped when job has no required shards (PH-S325 stub).
pub const METRIC_LOCALITY_RANK_SKIP_TOTAL: &str = "galaxy_locality_rank_skip_total";

/// Stub MB per cold shard on prefetch plan path when no task egress wire (PH-S185).
pub const DEFAULT_PREFETCH_CROSS_REGION_EGRESS_MB_PER_SHARD: f64 = 50.0;

static LAST_SHARD_LOCAL_HIT_RATIO_BPS: AtomicU64 = AtomicU64::new(0);
static LAST_CROSS_REGION_EGRESS_MB: AtomicU64 = AtomicU64::new(0);
static LOCALITY_RANK_INGEST_TOTAL: AtomicU64 = AtomicU64::new(0);
static LOCALITY_RANK_MISS_TOTAL: AtomicU64 = AtomicU64::new(0);
static LOCALITY_RANK_EMPTY_WORKERS_TOTAL: AtomicU64 = AtomicU64::new(0);
static LOCALITY_RANK_SKIP_TOTAL: AtomicU64 = AtomicU64::new(0);

fn worker_queue_depth(workers: &[LocalityWorker], worker_id: &str) -> u32 {
    workers
        .iter()
        .find(|w| w.worker_id == worker_id)
        .map(|w| w.queue_depth)
        .unwrap_or(0)
}

fn worker_pricing_usd_micro(workers: &[LocalityWorker], worker_id: &str) -> u64 {
    workers
        .iter()
        .find(|w| w.worker_id == worker_id)
        .and_then(|w| w.pricing_usd_micro)
        .unwrap_or(u64::MAX)
}

/// Ranked worker for scheduler stub (locality first; pricing/queue_depth tie-break PH-S548).
#[derive(Debug, Clone, PartialEq)]
pub struct LocalityRankedWorker {
    pub worker_id: String,
    pub score: f64,
}

/// Fraction of `required_shard_ids` present in worker inventory (0..=1).
#[inline]
pub fn shard_local_hit(inventory_shard_ids: &[String], required_shard_ids: &[String]) -> f64 {
    if required_shard_ids.is_empty() {
        return 1.0;
    }
    let hits = required_shard_ids
        .iter()
        .filter(|id| inventory_shard_ids.contains(id))
        .count();
    hits as f64 / required_shard_ids.len() as f64
}

/// `1 / (1 + latency_ms_p50/100)` from `network_profile` (concept §5.2).
#[inline]
pub fn latency_factor(latency_ms_p50: u32) -> f64 {
    1.0 / (1.0 + f64::from(latency_ms_p50) / 100.0)
}

/// Hot tier effectiveness: blend shard presence and task profile match (0..=1).
pub fn hot_tier_hit_ratio(inventory: &LocalitySeedInventory, task: &LocalityTask) -> f64 {
    let shard = shard_local_hit(&inventory.shard_ids, &task.required_shard_ids);
    let profile_ok = inventory.hot_tier.profiles.is_empty()
        || inventory.hot_tier.profiles.contains(&task.task_profile);
    let profile = if profile_ok { 1.0 } else { 0.0 };
    (shard + profile) / 2.0
}

/// Cross-region egress MB applied to the score (0 when local replica or same region).
pub fn effective_cross_region_egress_mb(worker: &LocalityWorker, task: &LocalityTask) -> f64 {
    if shard_local_hit(&worker.seed_inventory.shard_ids, &task.required_shard_ids) >= 1.0 {
        return 0.0;
    }
    let Some(source) = task.source_region.as_deref() else {
        return 0.0;
    };
    if worker.network_profile.region == source {
        return 0.0;
    }
    if worker
        .seed_inventory
        .local_replica_regions
        .iter()
        .any(|r| r == source)
    {
        return 0.0;
    }
    task.estimated_cross_region_egress_mb.max(0.0)
}

/// Penalty subtracted from `locality_score` when `profile_age_secs` is missing or exceeds `max_age_secs`
/// (Galaxy §8.1 stale profile stub, PH-S169).
#[inline]
pub fn stale_network_profile_penalty(
    profile_age_secs: Option<u64>,
    max_age_secs: u64,
    max_penalty: f64,
) -> f64 {
    match profile_age_secs {
        None => max_penalty,
        Some(age) if age > max_age_secs => max_penalty,
        Some(_) => 0.0,
    }
}

/// Placement score (concept §5.2):
/// `w_shard×shard_local_hit + w_lat×latency_factor + w_hot×hot_tier_hit − w_egress×egress_mb − stale_penalty`.
pub fn locality_score(
    worker: &LocalityWorker,
    task: &LocalityTask,
    weights: &LocalityWeights,
) -> f64 {
    let shard = shard_local_hit(&worker.seed_inventory.shard_ids, &task.required_shard_ids);
    let lat = latency_factor(worker.network_profile.latency_ms_p50);
    let hot = hot_tier_hit_ratio(&worker.seed_inventory, task);
    let egress = effective_cross_region_egress_mb(worker, task);
    let stale = stale_network_profile_penalty(
        worker.network_profile.profile_age_secs,
        DEFAULT_STALE_PROFILE_MAX_AGE_SECS,
        DEFAULT_STALE_PROFILE_PENALTY,
    );

    weights.w_shard * shard + weights.w_lat * lat + weights.w_hot * hot
        - weights.w_egress * egress
        - stale
}

/// Scheduler stub: sort workers by `locality_score` desc, then `worker_id` asc.
pub fn rank_workers_by_locality(
    workers: &[LocalityWorker],
    task: &LocalityTask,
) -> Vec<LocalityRankedWorker> {
    rank_workers_by_locality_with_weights(workers, task, &DEFAULT_LOCALITY_WEIGHTS)
}

/// Same as [`rank_workers_by_locality`] with explicit weights.
pub fn rank_workers_by_locality_with_weights(
    workers: &[LocalityWorker],
    task: &LocalityTask,
    weights: &LocalityWeights,
) -> Vec<LocalityRankedWorker> {
    let mut ranked: Vec<LocalityRankedWorker> = workers
        .iter()
        .map(|w| LocalityRankedWorker {
            worker_id: w.worker_id.clone(),
            score: locality_score(w, task, weights),
        })
        .collect();
    ranked.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                worker_queue_depth(workers, &a.worker_id)
                    .cmp(&worker_queue_depth(workers, &b.worker_id))
            })
            .then_with(|| {
                worker_pricing_usd_micro(workers, &a.worker_id)
                    .cmp(&worker_pricing_usd_micro(workers, &b.worker_id))
            })
            .then_with(|| a.worker_id.cmp(&b.worker_id))
    });
    if let Some(best) = ranked.first() {
        if let Some(worker) = workers.iter().find(|w| w.worker_id == best.worker_id) {
            observe_last_shard_local_hit_ratio(shard_local_hit(
                &worker.seed_inventory.shard_ids,
                &task.required_shard_ids,
            ));
            observe_last_cross_region_egress_mb(effective_cross_region_egress_mb(worker, task));
        }
    }
    ranked
}

/// Observe last top-ranked worker shard local hit ratio for Prometheus gauge (PH-S183).
pub fn observe_last_shard_local_hit_ratio(ratio: f64) {
    let clamped = ratio.clamp(0.0, 1.0);
    let bps = (clamped * 10_000.0).round() as u64;
    LAST_SHARD_LOCAL_HIT_RATIO_BPS.store(bps.min(10_000), Ordering::Relaxed);
}

/// Last observed shard local hit ratio in basis points (10_000 = 1.0) since process start.
pub fn last_shard_local_hit_ratio_bps() -> u64 {
    LAST_SHARD_LOCAL_HIT_RATIO_BPS.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_last_shard_local_hit_ratio_for_test() {
    LAST_SHARD_LOCAL_HIT_RATIO_BPS.store(0, Ordering::Relaxed);
}

/// Observe last cross-region egress MB for Prometheus gauge (PH-S185).
pub fn observe_last_cross_region_egress_mb(egress_mb: f64) {
    let mb = egress_mb.max(0.0).round() as u64;
    LAST_CROSS_REGION_EGRESS_MB.store(mb, Ordering::Relaxed);
}

/// Last observed cross-region egress in whole MB since process start.
pub fn last_cross_region_egress_mb() -> u64 {
    LAST_CROSS_REGION_EGRESS_MB.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_last_cross_region_egress_mb_for_test() {
    LAST_CROSS_REGION_EGRESS_MB.store(0, Ordering::Relaxed);
}

/// Record one locality rank on grid job ingest (PH-S295 stub).
pub fn record_locality_rank_ingest() {
    LOCALITY_RANK_INGEST_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn locality_rank_ingest_total() -> u64 {
    LOCALITY_RANK_INGEST_TOTAL.load(Ordering::Relaxed)
}

/// Record one locality rank miss on grid job ingest (PH-S305 stub).
pub fn record_locality_rank_miss() {
    LOCALITY_RANK_MISS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn locality_rank_miss_total() -> u64 {
    LOCALITY_RANK_MISS_TOTAL.load(Ordering::Relaxed)
}

/// Record locality rank with zero workers on ingest (PH-S315).
pub fn record_locality_rank_empty_workers() {
    LOCALITY_RANK_EMPTY_WORKERS_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn locality_rank_empty_workers_total() -> u64 {
    LOCALITY_RANK_EMPTY_WORKERS_TOTAL.load(Ordering::Relaxed)
}

/// Record locality rank skip on empty `required_shard_ids` (PH-S325).
pub fn record_locality_rank_skip() {
    LOCALITY_RANK_SKIP_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn locality_rank_skip_total() -> u64 {
    LOCALITY_RANK_SKIP_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_locality_rank_ingest_for_test() {
    LOCALITY_RANK_INGEST_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_locality_rank_miss_for_test() {
    LOCALITY_RANK_MISS_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_locality_rank_empty_workers_for_test() {
    LOCALITY_RANK_EMPTY_WORKERS_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_locality_rank_skip_for_test() {
    LOCALITY_RANK_SKIP_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_locality_metrics_for_test() {
    reset_last_shard_local_hit_ratio_for_test();
    reset_last_cross_region_egress_mb_for_test();
    reset_locality_rank_ingest_for_test();
    reset_locality_rank_miss_for_test();
    reset_locality_rank_empty_workers_for_test();
    reset_locality_rank_skip_for_test();
}

/// Pick highest-scoring worker (scheduler stub only — no prefetch / lease wire).
pub fn pick_best_worker_by_locality<'a>(
    workers: &'a [LocalityWorker],
    task: &LocalityTask,
) -> Option<&'a LocalityWorker> {
    let ranked = rank_workers_by_locality(workers, task);
    let best_id = ranked.first()?.worker_id.as_str();
    workers.iter().find(|w| w.worker_id == best_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worker(id: &str, region: &str, latency: u32, shards: &[&str]) -> LocalityWorker {
        worker_with_profile_age(id, region, latency, shards, Some(0))
    }

    fn worker_with_profile_age(
        id: &str,
        region: &str,
        latency: u32,
        shards: &[&str],
        profile_age_secs: Option<u64>,
    ) -> LocalityWorker {
        LocalityWorker {
            worker_id: id.into(),
            queue_depth: 0,
            pricing_usd_micro: None,
            seed_inventory: LocalitySeedInventory {
                shard_ids: shards.iter().map(|s| (*s).to_string()).collect(),
                hot_tier: LocalityHotTier {
                    ram_bytes_used: 1,
                    vram_bytes_used: 0,
                    profiles: vec!["inference:text".into()],
                },
                local_replica_regions: vec![region.into()],
            },
            network_profile: LocalityNetworkProfile {
                region: region.into(),
                latency_ms_p50: latency,
                profile_age_secs,
            },
        }
    }

    fn task(shards: &[&str], profile: &str, egress_mb: f64, source: Option<&str>) -> LocalityTask {
        LocalityTask {
            required_shard_ids: shards.iter().map(|s| (*s).to_string()).collect(),
            task_profile: profile.into(),
            estimated_cross_region_egress_mb: egress_mb,
            source_region: source.map(str::to_string),
        }
    }

    #[test]
    fn shard_local_hit_empty_required_is_one() {
        assert_eq!(shard_local_hit(&[], &[]), 1.0);
        assert_eq!(shard_local_hit(&["a".into()], &[]), 1.0);
    }

    #[test]
    fn shard_local_hit_partial_and_full() {
        let inv = vec!["w:emb-1".into(), "w:ckpt-7".into()];
        assert!(
            (shard_local_hit(&inv, &["w:emb-1".into(), "w:missing".into()]) - 0.5).abs()
                < f64::EPSILON
        );
        assert!(
            (shard_local_hit(&inv, &["w:emb-1".into(), "w:ckpt-7".into()]) - 1.0).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn latency_factor_decreases_with_latency() {
        let low = latency_factor(10);
        let high = latency_factor(200);
        assert!(low > high);
        assert!((latency_factor(0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn locality_score_prefers_local_shards_and_low_latency() {
        let job = task(&["w:emb-1"], "inference:text", 100.0, Some("eu-west"));
        let local = worker("local", "eu-west", 20, &["w:emb-1"]);
        let remote = worker("remote", "us-east", 150, &[]);
        let local_score = locality_score(&local, &job, &DEFAULT_LOCALITY_WEIGHTS);
        let remote_score = locality_score(&remote, &job, &DEFAULT_LOCALITY_WEIGHTS);
        assert!(local_score > remote_score);
    }

    #[test]
    fn rank_workers_by_locality_orders_desc_then_id() {
        let job = task(&["w:emb-1"], "inference:text", 0.0, None);
        let w1 = worker("b-worker", "eu-west", 50, &["w:emb-1"]);
        let w2 = worker("a-worker", "eu-west", 50, &["w:emb-1"]);
        let ranked = rank_workers_by_locality(&[w1, w2], &job);
        assert_eq!(ranked.len(), 2);
        assert!(ranked[0].score >= ranked[1].score);
        assert_eq!(ranked[0].worker_id, "a-worker");
        assert_eq!(ranked[1].worker_id, "b-worker");
    }

    #[test]
    fn rank_workers_tie_break_queue_depth_then_pricing_ph_s548() {
        let job = task(&["w:emb-1"], "inference:text", 0.0, None);
        let mut low_queue = worker("w-low-q", "eu-west", 50, &["w:emb-1"]);
        low_queue.queue_depth = 1;
        low_queue.pricing_usd_micro = Some(500);
        let mut high_queue = worker("w-high-q", "eu-west", 50, &["w:emb-1"]);
        high_queue.queue_depth = 9;
        high_queue.pricing_usd_micro = Some(100);
        let ranked = rank_workers_by_locality(&[high_queue, low_queue], &job);
        assert_eq!(ranked[0].worker_id, "w-low-q");
    }

    #[test]
    fn pick_best_worker_by_locality_returns_top() {
        let job = task(&["w:emb-1"], "inference:text", 50.0, Some("eu-west"));
        let good = worker("good", "eu-west", 10, &["w:emb-1"]);
        let bad = worker("bad", "ap-south", 300, &[]);
        let workers = [bad, good];
        let picked = pick_best_worker_by_locality(&workers, &job).expect("pick");
        assert_eq!(picked.worker_id, "good");
    }

    #[test]
    fn cross_region_egress_zero_when_same_region() {
        let w = worker("w1", "eu-west", 40, &[]);
        let job = task(&["w:emb-1"], "inference:text", 200.0, Some("eu-west"));
        assert!((effective_cross_region_egress_mb(&w, &job) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn cross_region_egress_applies_when_remote() {
        let w = worker("w1", "us-east", 40, &[]);
        let job = task(&["w:emb-1"], "inference:text", 200.0, Some("eu-west"));
        assert!((effective_cross_region_egress_mb(&w, &job) - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn stale_network_profile_penalty_missing_or_old() {
        assert!(
            (stale_network_profile_penalty(None, DEFAULT_STALE_PROFILE_MAX_AGE_SECS, 0.05) - 0.05)
                .abs()
                < f64::EPSILON
        );
        assert!(
            (stale_network_profile_penalty(
                Some(DEFAULT_STALE_PROFILE_MAX_AGE_SECS + 1),
                DEFAULT_STALE_PROFILE_MAX_AGE_SECS,
                0.05,
            ) - 0.05)
                .abs()
                < f64::EPSILON
        );
        assert!(
            (stale_network_profile_penalty(
                Some(DEFAULT_STALE_PROFILE_MAX_AGE_SECS),
                DEFAULT_STALE_PROFILE_MAX_AGE_SECS,
                0.05,
            ) - 0.0)
                .abs()
                < f64::EPSILON
        );
        assert!(
            (stale_network_profile_penalty(Some(0), DEFAULT_STALE_PROFILE_MAX_AGE_SECS, 0.05)
                - 0.0)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn locality_score_applies_stale_profile_penalty_ph_s169() {
        let job = task(&["w:emb-1"], "inference:text", 0.0, None);
        let fresh = worker_with_profile_age("fresh", "eu-west", 20, &["w:emb-1"], Some(3600));
        let stale = worker_with_profile_age("stale", "eu-west", 20, &["w:emb-1"], None);
        let fresh_score = locality_score(&fresh, &job, &DEFAULT_LOCALITY_WEIGHTS);
        let stale_score = locality_score(&stale, &job, &DEFAULT_LOCALITY_WEIGHTS);
        assert!((fresh_score - stale_score - DEFAULT_STALE_PROFILE_PENALTY).abs() < f64::EPSILON);
        assert!(fresh_score > stale_score);
    }

    #[test]
    fn rank_workers_by_locality_deprioritizes_stale_profile_ph_s169() {
        let job = task(&["w:emb-1"], "inference:text", 0.0, None);
        let fresh = worker_with_profile_age("fresh", "eu-west", 20, &["w:emb-1"], Some(0));
        let stale = worker_with_profile_age("stale", "eu-west", 20, &["w:emb-1"], None);
        let ranked = rank_workers_by_locality(&[stale, fresh], &job);
        assert_eq!(ranked.first().map(|r| r.worker_id.as_str()), Some("fresh"));
    }

    #[test]
    fn rank_workers_by_locality_observes_top_shard_local_hit_ratio_ph_s183() {
        reset_last_shard_local_hit_ratio_for_test();
        let job = task(&["w:emb-1", "w:ckpt-7"], "inference:text", 0.0, None);
        let full = worker("full", "eu-west", 20, &["w:emb-1", "w:ckpt-7"]);
        let partial = worker("partial", "eu-west", 20, &["w:emb-1"]);
        let _ = rank_workers_by_locality(&[partial, full], &job);
        assert_eq!(last_shard_local_hit_ratio_bps(), 10_000);
        reset_last_shard_local_hit_ratio_for_test();
    }

    #[test]
    fn rank_workers_by_locality_observes_cross_region_egress_mb_ph_s185() {
        reset_last_cross_region_egress_mb_for_test();
        let job = task(&["w:emb-1"], "inference:text", 200.0, Some("eu-west"));
        let remote = worker("remote", "us-east", 40, &[]);
        let _ = rank_workers_by_locality(&[remote], &job);
        assert_eq!(last_cross_region_egress_mb(), 200);
        reset_last_cross_region_egress_mb_for_test();
    }

    #[test]
    fn record_locality_rank_miss_ph_s305() {
        reset_locality_rank_miss_for_test();
        record_locality_rank_miss();
        assert_eq!(locality_rank_miss_total(), 1);
        reset_locality_rank_miss_for_test();
    }

    #[test]
    fn record_locality_rank_empty_workers_ph_s315() {
        reset_locality_rank_empty_workers_for_test();
        record_locality_rank_empty_workers();
        assert_eq!(locality_rank_empty_workers_total(), 1);
        reset_locality_rank_empty_workers_for_test();
    }

    #[test]
    fn record_locality_rank_skip_ph_s325() {
        reset_locality_rank_skip_for_test();
        record_locality_rank_skip();
        assert_eq!(locality_rank_skip_total(), 1);
        reset_locality_rank_skip_for_test();
    }
}
