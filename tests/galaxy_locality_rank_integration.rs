//! PH-S138: multi-worker locality rank integration (Galaxy §5.2, PH-S128 wire).

use poolai::grid::galaxy_locality::{
    pick_best_worker_by_locality, rank_workers_by_locality, LocalityHotTier,
    LocalityNetworkProfile, LocalitySeedInventory, LocalityTask, LocalityWorker,
};

mod fixture {
    use super::*;

    pub fn worker(
        id: &str,
        region: &str,
        latency_ms: u32,
        shards: &[&str],
        profiles: &[&str],
        replica_regions: &[&str],
    ) -> LocalityWorker {
        LocalityWorker {
            worker_id: id.into(),
            queue_depth: 0,
            pricing_usd_micro: None,
            seed_inventory: LocalitySeedInventory {
                shard_ids: shards.iter().map(|s| (*s).to_string()).collect(),
                hot_tier: LocalityHotTier {
                    ram_bytes_used: if shards.is_empty() { 0 } else { 4096 },
                    vram_bytes_used: 0,
                    profiles: profiles.iter().map(|p| (*p).to_string()).collect(),
                },
                local_replica_regions: replica_regions.iter().map(|r| (*r).to_string()).collect(),
            },
            network_profile: LocalityNetworkProfile {
                region: region.into(),
                latency_ms_p50: latency_ms,
                latency_ms_p95: None,
                profile_age_secs: Some(0),
            },
        }
    }

    pub fn inference_task(shards: &[&str], source_region: &str, egress_mb: f64) -> LocalityTask {
        LocalityTask {
            required_shard_ids: shards.iter().map(|s| (*s).to_string()).collect(),
            task_profile: "inference:text".into(),
            estimated_cross_region_egress_mb: egress_mb,
            source_region: Some(source_region.into()),
        }
    }

    /// Three workers: local full hit, remote partial, remote empty.
    pub fn three_worker_pool() -> [LocalityWorker; 3] {
        [
            worker(
                "eu-primary",
                "eu-west",
                15,
                &["w:emb-1", "w:ckpt-7"],
                &["inference:text"],
                &["eu-west"],
            ),
            worker(
                "us-replica",
                "us-east",
                80,
                &["w:emb-1"],
                &["inference:text"],
                &["eu-west", "us-east"],
            ),
            worker("ap-empty", "ap-south", 250, &[], &[], &["ap-south"]),
        ]
    }
}

use fixture::{inference_task, three_worker_pool, worker};

#[test]
fn rank_workers_by_locality_three_worker_fixture_orders_best_first() {
    let task = inference_task(&["w:emb-1", "w:ckpt-7"], "eu-west", 120.0);
    let workers = three_worker_pool();
    let ranked = rank_workers_by_locality(&workers, &task);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0].worker_id, "eu-primary");
    assert_eq!(ranked[1].worker_id, "us-replica");
    assert_eq!(ranked[2].worker_id, "ap-empty");
    assert!(ranked[0].score > ranked[1].score);
    assert!(ranked[1].score > ranked[2].score);
}

#[test]
fn pick_best_worker_matches_top_ranked_in_fixture() {
    let task = inference_task(&["w:emb-1", "w:ckpt-7"], "eu-west", 120.0);
    let workers = three_worker_pool();
    let ranked = rank_workers_by_locality(&workers, &task);
    let picked = pick_best_worker_by_locality(&workers, &task).expect("pick");
    assert_eq!(picked.worker_id, ranked[0].worker_id);
    assert_eq!(picked.worker_id, "eu-primary");
}

#[test]
fn rank_workers_tie_breaks_by_worker_id_asc() {
    let task = inference_task(&["w:emb-1"], "eu-west", 0.0);
    let w_a = worker(
        "z-worker",
        "eu-west",
        40,
        &["w:emb-1"],
        &["inference:text"],
        &["eu-west"],
    );
    let w_b = worker(
        "a-worker",
        "eu-west",
        40,
        &["w:emb-1"],
        &["inference:text"],
        &["eu-west"],
    );
    let ranked = rank_workers_by_locality(&[w_a, w_b], &task);
    assert!((ranked[0].score - ranked[1].score).abs() < f64::EPSILON);
    assert_eq!(ranked[0].worker_id, "a-worker");
    assert_eq!(ranked[1].worker_id, "z-worker");
}

#[test]
fn rank_workers_latency_beats_empty_inventory_same_region() {
    let task = inference_task(&["w:gpu-weights"], "eu-west", 50.0);
    let low_latency_empty = worker("fast-empty", "eu-west", 10, &[], &[], &["eu-west"]);
    let high_latency_hit = worker(
        "slow-hit",
        "eu-west",
        200,
        &["w:gpu-weights"],
        &["inference:text"],
        &["eu-west"],
    );
    let ranked = rank_workers_by_locality(&[low_latency_empty, high_latency_hit], &task);
    assert_eq!(ranked[0].worker_id, "slow-hit");
    assert!(ranked[0].score > ranked[1].score);
}
