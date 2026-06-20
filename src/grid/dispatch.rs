//! Grid envelope ingress ? Job/Result/MemoryShard side effects (FM-023).
//!
//! Grid `Job` ingest schedules via [`schedule_with_grid_peer`](crate::job::schedule_with_grid_peer);
//! when a source peer binds `worker_id`, scheduler lease acquire sets `JobStatus::Leased` (PH-S108).
//! `Result` ingest validates `lease_epoch` CAS when the job row has active lease fields (PH-S110).
//! Edge `trust_score` settlement gate stub on result path (PH-S130, Galaxy ?6.5).
//! Settlement `pending_verification` status stub (PH-S170, ?6.3?6.5).
//! Verification sampling stub on result path (PH-S164, Galaxy ?6.2).
//! Seed inventory + task-driven prefetch policy stub (PH-S129, Galaxy ?5.5).

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

use crate::core::error::AppError;
use crate::grid::galaxy_capability_admission::check_telegram_edge_capability_admission;
use crate::grid::galaxy_fee_split::{
    split_gross_payment, SECONDARY_ADMIN_FEE_MAX_BPS, SECONDARY_ADMIN_FEE_MIN_BPS,
};
use crate::grid::galaxy_fee_split_metrics::evaluate_result_fee_split;
use crate::grid::galaxy_fraud_proof::{evaluate_fraud_proof_hold, record_fraud_proof_pending};
use crate::grid::galaxy_locality::{
    observe_last_cross_region_egress_mb, observe_last_hot_tier_hit_ratio,
    pick_best_worker_by_locality, pick_best_worker_by_locality_with_hot_tier_gate,
    record_locality_rank_empty_workers, record_locality_rank_ingest, record_locality_rank_miss,
    record_locality_rank_skip, LocalityHotTier, LocalityNetworkProfile, LocalitySeedInventory,
    LocalityTask, LocalityWorker, DEFAULT_PREFETCH_CROSS_REGION_EGRESS_MB_PER_SHARD,
};
use crate::grid::galaxy_network_profile::{load_parsed_peer_network_profile, GalaxyEgressPolicy};
use crate::grid::galaxy_prefetch_metrics::{
    observe_prefetch_queue_depth, observe_shard_fetch_latency_ms_p50, record_hot_evict,
    record_hot_promote, record_locality_unsatisfied, record_prefetch_backpressure,
    record_prefetch_co_access, record_prefetch_complete, record_prefetch_egress_blocked,
    record_prefetch_enqueue, record_prefetch_ingest, record_prefetch_lease_acquired,
    record_prefetch_peer_fetch, record_prefetch_peer_fetch_miss, record_prefetch_plan,
    record_prefetch_pull_bytes, record_prefetch_raid_fetch, record_prefetch_raid_fetch_miss,
    record_prefetch_re_migrate, record_prefetch_seed_fetch, record_prefetch_seed_fetch_miss,
    record_prefetch_seed_pull, record_prefetch_skip_ingest, record_prefetch_strict_mode,
    record_prefetch_timeout, record_prefetch_topology_blocked, record_prefetch_wait,
    record_shard_access, should_hot_promote, DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM,
    DEFAULT_PREFETCH_BYTES_PER_SHARD_VRAM, DEFAULT_SHARD_FETCH_LATENCY_MS_P50,
};
use crate::grid::galaxy_prefetch_peer_pull::fetch_seed_shards_from_peer_http;
use crate::grid::galaxy_replay_jobs::submit_replay_verification_job;
use crate::grid::galaxy_replay_metrics::evaluate_result_replay_pending;
use crate::grid::galaxy_replication::{
    replication_tier_from_policy, ReplicationTierConfig, REPLICATION_STANDARD, REPLICATION_STRICT,
};
use crate::grid::galaxy_replication_metrics::replication_executor_hook;
use crate::grid::galaxy_replication_quorum_gate::{
    record_result_executor_digest, replication_quorum_allows_cleared,
};
use crate::grid::galaxy_settlement::{
    resolve_payout_pubkey, resolve_settlement_status, PayoutBatchLedgerEntry, SettlementStatus,
};
use crate::grid::galaxy_settlement_metrics::{
    evaluate_result_settlement_cleared, evaluate_result_settlement_not_applicable,
    evaluate_result_settlement_pending_verification, evaluate_result_settlement_resolved,
    evaluate_semantic_hash_human_review_hold, record_payout_batch_ledger_entry,
    record_settlement_human_review,
};
use crate::grid::galaxy_settlement_onchain::emit_settlement_job_rewarded;
use crate::grid::galaxy_trust_score::{
    apply_verification_trust_delta, clamp_trust_score, evaluate_result_settlement_gate,
    SettlementGateVerdict, TrustScore, TrustScoreGateConfig, DEFAULT_TRUST_SCORE,
};
use crate::grid::galaxy_trust_score_store::{
    apply_lease_epoch_rejected_trust_delta, lookup_peer_trust_score, persist_peer_trust_score,
};
use crate::grid::galaxy_verification_checker_jobs::submit_shadow_verification_checker_job;
use crate::grid::galaxy_verification_metrics::{
    drain_verification_checker_task, enqueue_verification_checker_task,
    evaluate_result_verification_match, evaluate_result_verification_mismatch,
    evaluate_result_verification_sample, evaluate_result_verification_sample_completed,
    evaluate_semantic_hash_verification,
};
use crate::grid::galaxy_verify_sampling::{
    evaluate_checker_timeout_policy, evaluate_post_mismatch_elevated_sampling,
    evaluate_result_verify_sampling, VerifySamplingConfig, VerifySamplingVerdict,
};
use crate::grid::{GridEnvelope, GridEnvelopeError, GridMessage, GridResultBody};
use crate::job::{
    check_grid_result_lease_epoch, emit_memory_updated, emit_seed_provided, job_spec_from_grid_job,
    job_status_from_grid_result, memory_content_digest, schedule_with_grid_peer, JobRecord,
    JobStatus, JobStore, PatchLeaseEpochError,
};
use crate::memory::{memory_shard_from_grid_body, MemoryShardStore};
use crate::observability::lease_trace::{
    trace_lease_reject, LeaseOperation, LeaseOutcome, LeaseSource,
};

/// Worker seed inventory wire DTO (Galaxy ?5.2 / ?5.5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SeedInventoryEntry {
    #[serde(default)]
    pub shard_ids: Vec<String>,
    #[serde(default)]
    pub hot_tier: SeedInventoryHotTier,
    #[serde(default)]
    pub local_replica_regions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_inventory_at: Option<String>,
}

/// Hot tier subset inside [`SeedInventoryEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SeedInventoryHotTier {
    #[serde(default)]
    pub ram_bytes_used: u64,
    #[serde(default)]
    pub vram_bytes_used: u64,
    #[serde(default)]
    pub profiles: Vec<String>,
}

/// One worker peer row in coordinator seed inventory read model (PH-S195).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeedInventoryPeerSnapshot {
    pub peer_id: String,
    pub seed_inventory: SeedInventoryEntry,
}

/// Read-only coordinator stub inventory for discovery / prefetch planning (Galaxy §5.5).
pub fn coordinator_seed_inventory_snapshot() -> Vec<SeedInventoryPeerSnapshot> {
    vec![
        SeedInventoryPeerSnapshot {
            peer_id: "srv1-worker-a".into(),
            seed_inventory: SeedInventoryEntry {
                shard_ids: vec!["w:emb-1".into(), "w:ckpt-7".into()],
                hot_tier: SeedInventoryHotTier {
                    ram_bytes_used: 3_221_225_472,
                    vram_bytes_used: 0,
                    profiles: vec!["inference:text".into()],
                },
                local_replica_regions: vec!["eu-west".into()],
                last_inventory_at: Some("2026-05-27T10:00:00Z".into()),
            },
        },
        SeedInventoryPeerSnapshot {
            peer_id: "srv2-worker-b".into(),
            seed_inventory: SeedInventoryEntry {
                shard_ids: vec!["w:ckpt-7".into()],
                hot_tier: SeedInventoryHotTier {
                    ram_bytes_used: 1_073_741_824,
                    vram_bytes_used: 2_147_483_648,
                    profiles: vec!["inference:text".into(), "inference:gpu".into()],
                },
                local_replica_regions: vec!["us-east".into()],
                last_inventory_at: Some("2026-05-27T10:05:00Z".into()),
            },
        },
    ]
}

/// Merged coordinator seed inventory for prefetch planning (PH-S276 stub).
pub fn coordinator_merged_seed_inventory() -> SeedInventoryEntry {
    let snaps = coordinator_seed_inventory_snapshot();
    let mut shard_ids = Vec::new();
    let mut ram = 0u64;
    let mut vram = 0u64;
    for snap in &snaps {
        for id in &snap.seed_inventory.shard_ids {
            if !shard_ids.iter().any(|x| x == id) {
                shard_ids.push(id.clone());
            }
        }
        ram = ram.saturating_add(snap.seed_inventory.hot_tier.ram_bytes_used);
        vram = vram.saturating_add(snap.seed_inventory.hot_tier.vram_bytes_used);
    }
    SeedInventoryEntry {
        shard_ids,
        hot_tier: SeedInventoryHotTier {
            ram_bytes_used: ram,
            vram_bytes_used: vram,
            profiles: Vec::new(),
        },
        local_replica_regions: Vec::new(),
        last_inventory_at: None,
    }
}

/// Task-driven prefetch on grid job ingest (PH-S276 stub; no enqueue wire).
pub fn ingest_job_prefetch_stub(
    required_shard_ids: &[String],
    gpu_capable: bool,
    memory: Option<&MemoryShardStore>,
) -> usize {
    if required_shard_ids.is_empty() {
        record_prefetch_skip_ingest();
        return 0;
    }
    record_prefetch_ingest();
    let inventory = coordinator_merged_seed_inventory();
    let config = PrefetchPolicyConfig::from_env();
    if config.mode == PrefetchPolicyMode::StrictLocality {
        record_prefetch_strict_mode();
    }
    let plan = plan_prefetch(
        &inventory,
        required_shard_ids,
        PrefetchTrigger::JobAdmitted,
        gpu_capable,
        &config,
    );
    let n = complete_prefetch_hook(&plan, memory);
    // PH-S632: hot-tier skip may empty plan; still record seed-pull for inventory-resident shards.
    if plan.items.is_empty() {
        let target_tier = if gpu_capable {
            PrefetchTargetTier::Vram
        } else {
            PrefetchTargetTier::Ram
        };
        let fallback_plan = PrefetchPlan {
            items: order_shards_by_access_weight(required_shard_ids)
                .into_iter()
                .map(|shard_id| PrefetchPlanItem {
                    shard_id,
                    target_tier,
                })
                .collect(),
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: config.prefetch_deadline_ms,
            mode: config.mode,
        };
        seed_pull_hook(&fallback_plan);
    }
    if let Some(first) = required_shard_ids.first() {
        if let Some(co_plan) = plan_co_access_prefetch(first, gpu_capable, &config) {
            if !co_plan.items.is_empty() {
                let _ = complete_prefetch_hook(&co_plan, memory);
            }
        }
    }
    n
}

/// Strict locality prefetch deadline fail policy (PH-S546, Galaxy §5.6).
pub fn evaluate_strict_prefetch_timeout(
    required_shard_ids: &[String],
    memory: &MemoryShardStore,
) -> Result<(), AppError> {
    let config = PrefetchPolicyConfig::from_env();
    if config.mode != PrefetchPolicyMode::StrictLocality || required_shard_ids.is_empty() {
        return Ok(());
    }
    let hits = required_shard_ids
        .iter()
        .filter(|id| memory.get(id).ok().flatten().is_some())
        .count();
    if hits >= required_shard_ids.len() {
        return Ok(());
    }
    let inventory = coordinator_merged_seed_inventory();
    let plan = plan_prefetch(
        &inventory,
        required_shard_ids,
        PrefetchTrigger::JobAdmitted,
        false,
        &config,
    );
    let waited = wait_prefetch_hook(&plan);
    if waited >= config.prefetch_deadline_ms && hits < required_shard_ids.len() {
        record_prefetch_timeout();
        return Err(AppError::RestError {
            code: "prefetch-timeout",
            message: format!(
                "prefetch deadline exceeded under strict_locality ({}/{} shards in memory)",
                hits,
                required_shard_ids.len()
            ),
        });
    }
    Ok(())
}

/// Lease-acquired prefetch stub (PH-S425; no enqueue wire).
pub fn lease_acquire_prefetch_stub() -> usize {
    let inventory = coordinator_merged_seed_inventory();
    let config = PrefetchPolicyConfig::from_env();
    let shard_ids: Vec<String> = inventory.shard_ids.iter().take(2).cloned().collect();
    if shard_ids.is_empty() {
        return 0;
    }
    let plan = plan_prefetch(
        &inventory,
        &shard_ids,
        PrefetchTrigger::LeaseAcquired,
        false,
        &config,
    );
    complete_prefetch_hook(&plan, None)
}

/// Re-migrate prefetch stub on Migrating→Leased handoff (PH-S454; PH-S613 delta-fetch missing shards).
pub fn re_migrate_prefetch_stub(memory: Option<&MemoryShardStore>) -> usize {
    let inventory = coordinator_merged_seed_inventory();
    let config = PrefetchPolicyConfig::from_env();
    let shard_ids: Vec<String> = inventory
        .shard_ids
        .iter()
        .filter(|shard_id| {
            memory
                .and_then(|store| store.get(shard_id).ok().flatten())
                .is_none()
        })
        .take(4)
        .cloned()
        .collect();
    if shard_ids.is_empty() {
        return 0;
    }
    let plan = plan_prefetch(
        &inventory,
        &shard_ids,
        PrefetchTrigger::ReMigrate,
        false,
        &config,
    );
    complete_prefetch_hook(&plan, memory)
}

/// Coordinator worker rows for locality rank stub (PH-S285).
pub fn locality_workers_from_seed_snapshots() -> Vec<LocalityWorker> {
    coordinator_seed_inventory_snapshot()
        .into_iter()
        .map(|snap| {
            let region = snap
                .seed_inventory
                .local_replica_regions
                .first()
                .cloned()
                .unwrap_or_else(|| "unknown".into());
            let network_profile = load_parsed_peer_network_profile(&snap.peer_id)
                .map(|p| p.locality_subset())
                .unwrap_or(LocalityNetworkProfile {
                    region: region.clone(),
                    latency_ms_p50: 12,
                    latency_ms_p95: None,
                    profile_age_secs: None,
                });
            LocalityWorker {
                worker_id: snap.peer_id,
                queue_depth: 0,
                pricing_usd_micro: None,
                seed_inventory: LocalitySeedInventory {
                    shard_ids: snap.seed_inventory.shard_ids,
                    hot_tier: LocalityHotTier {
                        ram_bytes_used: snap.seed_inventory.hot_tier.ram_bytes_used,
                        vram_bytes_used: snap.seed_inventory.hot_tier.vram_bytes_used,
                        profiles: snap.seed_inventory.hot_tier.profiles,
                    },
                    local_replica_regions: snap.seed_inventory.local_replica_regions,
                },
                network_profile,
            }
        })
        .collect()
}

/// Rank workers by locality on grid job ingest (PH-S285 stub; no bind wire).
pub fn ingest_job_locality_rank_stub(
    required_shard_ids: &[String],
    task_kind: &str,
) -> Option<String> {
    if required_shard_ids.is_empty() {
        record_locality_rank_skip();
        return None;
    }
    let workers = locality_workers_from_seed_snapshots();
    if workers.is_empty() {
        record_locality_rank_empty_workers();
        return None;
    }
    let task = LocalityTask {
        required_shard_ids: required_shard_ids.to_vec(),
        task_profile: task_kind.to_string(),
        estimated_cross_region_egress_mb: 0.0,
        source_region: None,
    };
    pick_best_worker_by_locality_with_hot_tier_gate(&workers, &task)
        .map(|w| {
            record_locality_rank_ingest();
            w.worker_id.clone()
        })
        .or_else(|| {
            record_locality_rank_miss();
            None
        })
}

fn grid_job_gpu_capable(task_kind: &str) -> bool {
    let k = task_kind.to_ascii_lowercase();
    k.contains("train") || k.contains("gpu")
}

/// Prefetch destination tier (concept ?5.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrefetchTargetTier {
    Ram,
    Vram,
}

/// Prefetch trigger (concept ?5.5 table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchTrigger {
    JobAdmitted,
    LeaseAcquired,
    ReMigrate,
    CoAccessGraph,
}

/// Locality / prefetch strictness (concept ?5.5, ?5.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrefetchPolicyMode {
    #[default]
    BestEffort,
    StrictLocality,
}

/// Default max wait before Running (`POOLAI_GALAXY_PREFETCH_DEADLINE_MS`, ?5.6).
pub const DEFAULT_PREFETCH_DEADLINE_MS: u64 = 15_000;

/// Env: prefetch wait deadline milliseconds (?5.6).
pub const ENV_PREFETCH_DEADLINE_MS: &str = "POOLAI_GALAXY_PREFETCH_DEADLINE_MS";

/// Env: `strict_locality` | `best_effort` (?5.6).
pub const ENV_LOCALITY_MODE: &str = "POOLAI_GALAXY_LOCALITY_MODE";

/// Env: minimum peer bandwidth (Mbps) before prefetch enqueue is skipped (PH-S464).
pub const ENV_PREFETCH_MIN_BANDWIDTH_MBPS: &str = "POOLAI_GALAXY_PREFETCH_MIN_BANDWIDTH_MBPS";

/// Env: stub peer bandwidth (Mbps) for backpressure gate (PH-S464).
pub const ENV_PREFETCH_PEER_BANDWIDTH_MBPS: &str = "POOLAI_GALAXY_PREFETCH_PEER_BANDWIDTH_MBPS";

/// Env: JSON map of admitted shard → speculative neighbors (PH-S469).
pub const ENV_CO_ACCESS_GRAPH_JSON: &str = "POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON";

/// Env: coordinator region for cross-region egress guardrail (PH-S474).
pub const ENV_PREFETCH_COORDINATOR_REGION: &str = "POOLAI_GALAXY_COORDINATOR_REGION";

/// Env: stub peer region for prefetch egress gate (PH-S474).
pub const ENV_PREFETCH_PEER_REGION: &str = "POOLAI_GALAXY_PREFETCH_PEER_REGION";

/// Env: stub peer egress policy (`lan_only` | `direct`, PH-S474).
pub const ENV_PREFETCH_PEER_EGRESS_POLICY: &str = "POOLAI_GALAXY_PREFETCH_PEER_EGRESS_POLICY";

/// Coordinator topology ring for SmallWorld admission (PH-S604, Galaxy §8.1).
pub const ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING: &str = "POOLAI_GALAXY_COORDINATOR_TOPOLOGY_RING";

thread_local! {
    static PREFETCH_PEER_CTX: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Scope prefetch backpressure/egress gates to a source peer (PH-S591/S592).
pub fn with_prefetch_peer<R>(peer_id: Option<&str>, f: impl FnOnce() -> R) -> R {
    PREFETCH_PEER_CTX.with(|cell| {
        let prev = cell.borrow_mut().clone();
        *cell.borrow_mut() = peer_id.map(str::to_string);
        let out = f();
        *cell.borrow_mut() = prev;
        out
    })
}

fn current_prefetch_peer_id() -> Option<String> {
    PREFETCH_PEER_CTX.with(|cell| cell.borrow().clone())
}

fn resolve_prefetch_peer_bandwidth_mbps() -> Option<u32> {
    if let Some(peer_id) = current_prefetch_peer_id() {
        if let Some(profile) = load_parsed_peer_network_profile(&peer_id) {
            if let Some(bw) = profile.bandwidth_mbps {
                return Some(bw);
            }
        }
    }
    env_u32(ENV_PREFETCH_PEER_BANDWIDTH_MBPS)
}

fn resolve_prefetch_peer_region() -> Option<String> {
    if let Some(peer_id) = current_prefetch_peer_id() {
        if let Some(profile) = load_parsed_peer_network_profile(&peer_id) {
            let region = profile.region.trim().to_ascii_lowercase();
            if !region.is_empty() {
                return Some(region);
            }
        }
    }
    env_prefetch_peer_region()
}

fn resolve_prefetch_peer_egress_policy() -> Option<GalaxyEgressPolicy> {
    if let Some(peer_id) = current_prefetch_peer_id() {
        if let Some(profile) = load_parsed_peer_network_profile(&peer_id) {
            if let Some(policy) = profile.egress_policy {
                return Some(policy);
            }
        }
    }
    env_prefetch_peer_egress_policy()
}

fn resolve_prefetch_peer_topology_ring() -> Option<String> {
    if let Some(peer_id) = current_prefetch_peer_id() {
        if let Some(profile) = load_parsed_peer_network_profile(&peer_id) {
            if let Some(ring) = profile.topology_ring.as_ref() {
                let trimmed = ring.trim().to_ascii_lowercase();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
    }
    None
}

fn resolve_prefetch_peer_white_ip_only() -> bool {
    if let Some(peer_id) = current_prefetch_peer_id() {
        if let Some(profile) = load_parsed_peer_network_profile(&peer_id) {
            return profile.white_ip_only.unwrap_or(false)
                || profile.egress_policy == Some(GalaxyEgressPolicy::WhiteIp);
        }
    }
    false
}

fn env_coordinator_topology_ring() -> Option<String> {
    std::env::var(ENV_PREFETCH_COORDINATOR_TOPOLOGY_RING)
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .filter(|v| !v.is_empty())
}

/// Coordinator prefetch policy from environment (PH-S136, Galaxy ?5.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefetchPolicyConfig {
    pub mode: PrefetchPolicyMode,
    pub prefetch_deadline_ms: u64,
}

impl Default for PrefetchPolicyConfig {
    fn default() -> Self {
        Self {
            mode: PrefetchPolicyMode::default(),
            prefetch_deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
        }
    }
}

impl PrefetchPolicyConfig {
    pub fn from_env() -> Self {
        Self {
            mode: prefetch_policy_mode_from_env(),
            prefetch_deadline_ms: env_u64(ENV_PREFETCH_DEADLINE_MS)
                .filter(|&v| v > 0)
                .unwrap_or(DEFAULT_PREFETCH_DEADLINE_MS),
        }
    }
}

/// Parse `POOLAI_GALAXY_LOCALITY_MODE` (`strict_locality` | `best_effort`).
pub fn parse_prefetch_policy_mode(raw: &str) -> PrefetchPolicyMode {
    match raw.trim().to_ascii_lowercase().as_str() {
        "strict_locality" | "strict" => PrefetchPolicyMode::StrictLocality,
        "best_effort" | "best-effort" => PrefetchPolicyMode::BestEffort,
        _ => PrefetchPolicyMode::BestEffort,
    }
}

fn prefetch_policy_mode_from_env() -> PrefetchPolicyMode {
    std::env::var(ENV_LOCALITY_MODE)
        .ok()
        .map(|v| parse_prefetch_policy_mode(&v))
        .unwrap_or_default()
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok())
}

fn env_u32(name: &str) -> Option<u32> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok())
}

/// One shard scheduled for prefetch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefetchPlanItem {
    pub shard_id: String,
    pub target_tier: PrefetchTargetTier,
}

/// Planned prefetch work (no wire enqueue in PH-S129).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefetchPlan {
    pub items: Vec<PrefetchPlanItem>,
    pub trigger: PrefetchTrigger,
    pub deadline_ms: u64,
    pub mode: PrefetchPolicyMode,
}

/// Whether `shard_id` is present and hot tier has active bytes (stub ?5.5).
#[inline]
pub fn hot_hit(inventory: &SeedInventoryEntry, shard_id: &str) -> bool {
    inventory.shard_ids.iter().any(|id| id == shard_id)
        && (inventory.hot_tier.ram_bytes_used > 0 || inventory.hot_tier.vram_bytes_used > 0)
}

/// Env: JSON map shard_id → access weight for prefetch ordering (Galaxy §5.5, PH-S614).
pub const ENV_SHARD_ACCESS_WEIGHTS: &str = "POOLAI_GALAXY_SHARD_ACCESS_WEIGHTS";

fn shard_access_weight(shard_id: &str) -> u64 {
    shard_access_weights_from_env()
        .get(shard_id)
        .copied()
        .unwrap_or(0)
}

fn shard_access_weights_from_env() -> std::collections::HashMap<String, u64> {
    std::env::var(ENV_SHARD_ACCESS_WEIGHTS)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

/// Order required shards by descending access weight (PH-S614).
pub fn order_shards_by_access_weight(shard_ids: &[String]) -> Vec<String> {
    let mut ordered = shard_ids.to_vec();
    ordered.sort_by(|a, b| {
        shard_access_weight(b)
            .cmp(&shard_access_weight(a))
            .then_with(|| a.cmp(b))
    });
    ordered
}

/// Task-driven prefetch plan: skip shards already hot; pick RAM/VRAM tier from capabilities.
pub fn plan_prefetch(
    inventory: &SeedInventoryEntry,
    required_shard_ids: &[String],
    trigger: PrefetchTrigger,
    gpu_capable: bool,
    config: &PrefetchPolicyConfig,
) -> PrefetchPlan {
    let ordered = order_shards_by_access_weight(required_shard_ids);
    let target_tier = if gpu_capable {
        PrefetchTargetTier::Vram
    } else {
        PrefetchTargetTier::Ram
    };
    let items: Vec<PrefetchPlanItem> = ordered
        .iter()
        .filter(|shard_id| !hot_hit(inventory, shard_id))
        .map(|shard_id| PrefetchPlanItem {
            shard_id: shard_id.clone(),
            target_tier,
        })
        .collect();
    let bytes_per_shard = if gpu_capable {
        DEFAULT_PREFETCH_BYTES_PER_SHARD_VRAM
    } else {
        DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM
    };
    let prefetch_bytes = items.len() as u64 * bytes_per_shard;
    record_prefetch_plan(required_shard_ids.len(), items.len(), prefetch_bytes);
    if config.mode == PrefetchPolicyMode::StrictLocality && !items.is_empty() {
        record_prefetch_strict_mode();
    }
    if trigger == PrefetchTrigger::LeaseAcquired {
        record_prefetch_lease_acquired();
    }
    if trigger == PrefetchTrigger::CoAccessGraph {
        record_prefetch_co_access();
    }
    if trigger == PrefetchTrigger::ReMigrate {
        record_prefetch_re_migrate();
    }
    if !items.is_empty() {
        observe_last_cross_region_egress_mb(
            DEFAULT_PREFETCH_CROSS_REGION_EGRESS_MB_PER_SHARD * items.len() as f64,
        );
    }
    PrefetchPlan {
        items,
        trigger,
        deadline_ms: config.prefetch_deadline_ms,
        mode: config.mode,
    }
}

/// No-op prefetch hook (PH-S129): returns planned item count; no enqueue/wait wire.
#[inline]
pub fn noop_prefetch_hook(plan: &PrefetchPlan) -> usize {
    plan.items.len()
}

/// Prefetch enqueue stub (PH-S283): records enqueue metrics; no live seed pull wire.
#[inline]
pub fn enqueue_prefetch_hook(plan: &PrefetchPlan) -> usize {
    if prefetch_enqueue_blocked() {
        return 0;
    }
    let n = plan.items.len();
    record_prefetch_enqueue(n);
    observe_prefetch_queue_depth(n as u64);
    n
}

/// Returns true when prefetch enqueue should be skipped (backpressure or egress guardrail).
pub fn prefetch_enqueue_blocked() -> bool {
    prefetch_backpressure_skip()
        || prefetch_egress_blocked_skip()
        || prefetch_topology_admission_blocked_skip()
}

/// Bandwidth backpressure gate for prefetch enqueue (PH-S464).
pub fn prefetch_backpressure_skip() -> bool {
    let Some(min) = env_u32(ENV_PREFETCH_MIN_BANDWIDTH_MBPS) else {
        return false;
    };
    let peer_bw = resolve_prefetch_peer_bandwidth_mbps().unwrap_or(0);
    if peer_bw < min {
        record_prefetch_backpressure();
        true
    } else {
        false
    }
}

/// Cross-region egress guardrail for `lan_only` peers (PH-S474, Galaxy §8.1).
pub fn prefetch_egress_blocked_skip() -> bool {
    let Some(policy) = resolve_prefetch_peer_egress_policy() else {
        return false;
    };
    if policy != GalaxyEgressPolicy::LanOnly {
        return false;
    }
    let Some(coord) = env_prefetch_coordinator_region() else {
        return false;
    };
    let Some(peer) = resolve_prefetch_peer_region() else {
        return false;
    };
    if coord == peer {
        return false;
    }
    record_prefetch_egress_blocked();
    true
}

/// Topology / white-IP admission guard for prefetch (PH-S604, Galaxy §8.1).
pub fn prefetch_topology_admission_blocked_skip() -> bool {
    if resolve_prefetch_peer_white_ip_only() {
        let Some(coord) = env_prefetch_coordinator_region() else {
            return false;
        };
        let Some(peer) = resolve_prefetch_peer_region() else {
            return false;
        };
        if coord != peer {
            record_prefetch_topology_blocked();
            return true;
        }
    }
    if let (Some(coord_ring), Some(peer_ring)) = (
        env_coordinator_topology_ring(),
        resolve_prefetch_peer_topology_ring(),
    ) {
        if coord_ring != peer_ring {
            record_prefetch_topology_blocked();
            return true;
        }
    }
    false
}

fn env_prefetch_coordinator_region() -> Option<String> {
    std::env::var(ENV_PREFETCH_COORDINATOR_REGION)
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .filter(|v| !v.is_empty())
}

fn env_prefetch_peer_region() -> Option<String> {
    std::env::var(ENV_PREFETCH_PEER_REGION)
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .filter(|v| !v.is_empty())
}

fn env_prefetch_peer_egress_policy() -> Option<GalaxyEgressPolicy> {
    let raw = std::env::var(ENV_PREFETCH_PEER_EGRESS_POLICY).ok()?;
    match raw.trim().to_ascii_lowercase().as_str() {
        "lan_only" | "lan-only" => Some(GalaxyEgressPolicy::LanOnly),
        "direct" => Some(GalaxyEgressPolicy::Direct),
        "vpn_proxy" | "vpn-proxy" => Some(GalaxyEgressPolicy::VpnProxy),
        "white_ip" | "white-ip" => Some(GalaxyEgressPolicy::WhiteIp),
        _ => None,
    }
}

/// Prefetch wait stub (PH-S293): records wait ms metric; no live seed pull wire.
#[inline]
pub fn wait_prefetch_hook(plan: &PrefetchPlan) -> u64 {
    record_prefetch_wait(plan.items.len(), plan.deadline_ms);
    plan.deadline_ms
}

/// Prefetch complete stub (PH-S307): enqueue + wait + complete metric; optional memory fetch (PH-S444).
#[inline]
pub fn complete_prefetch_hook(plan: &PrefetchPlan, memory: Option<&MemoryShardStore>) -> usize {
    let n = plan.items.len();
    if n > 0 && !prefetch_enqueue_blocked() {
        record_prefetch_enqueue(n);
        observe_prefetch_queue_depth(n as u64);
        record_prefetch_wait(n, plan.deadline_ms);
        record_prefetch_complete(n);
        let memory_hits = if let Some(store) = memory {
            fetch_seed_shards_hook(plan, store)
        } else {
            0
        };
        if should_hot_promote(memory_hits) {
            record_hot_promote(memory_hits);
        } else if memory_hits > 0 {
            record_hot_evict(memory_hits);
        }
        record_shard_access(n);
        seed_pull_hook(plan);
        fetch_seed_shards_from_peer_hook(plan);
        if let Some(store) = memory {
            fetch_seed_shards_from_raid_hook(plan, store);
        }
    }
    n
}

/// Memory-layer seed fetch stub after inventory resolve (PH-S444); live bytes pull (PH-S484).
pub fn fetch_seed_shards_hook(plan: &PrefetchPlan, memory: &MemoryShardStore) -> usize {
    let mut hits = 0usize;
    let mut pull_bytes = 0u64;
    for item in &plan.items {
        match memory.get(&item.shard_id) {
            Ok(Some(_)) => {
                hits += 1;
                pull_bytes += match item.target_tier {
                    PrefetchTargetTier::Vram => DEFAULT_PREFETCH_BYTES_PER_SHARD_VRAM,
                    PrefetchTargetTier::Ram => DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM,
                };
            }
            Ok(None) => record_prefetch_seed_fetch_miss(),
            Err(_) => record_prefetch_seed_fetch_miss(),
        }
    }
    if hits > 0 {
        record_prefetch_seed_fetch(hits);
        record_prefetch_pull_bytes(pull_bytes);
        observe_shard_fetch_latency_ms_p50(DEFAULT_SHARD_FETCH_LATENCY_MS_P50);
    }
    hits
}

/// RAID artifact prefetch fetch stub (PH-S465): resolve shards via memory / RAID logical name.
pub fn fetch_seed_shards_from_raid_hook(plan: &PrefetchPlan, memory: &MemoryShardStore) -> usize {
    let mut hits = 0usize;
    for item in &plan.items {
        let found = memory.get(&item.shard_id).ok().flatten().is_some()
            || memory
                .list_by_raid_logical_name(&item.shard_id)
                .map(|shards| !shards.is_empty())
                .unwrap_or(false);
        if found {
            hits += 1;
            record_prefetch_raid_fetch(1);
        } else {
            record_prefetch_raid_fetch_miss();
        }
    }
    hits
}

/// Peer seed inventory prefetch fetch (PH-S479 snapshot; PH-S537 HTTP pull when env set).
pub fn fetch_seed_shards_from_peer_hook(plan: &PrefetchPlan) -> usize {
    let http_hits = fetch_seed_shards_from_peer_http(plan);
    if http_hits > 0 || peer_http_prefetch_enabled() {
        return http_hits;
    }
    let mut hits = 0usize;
    for item in &plan.items {
        let found = coordinator_seed_inventory_snapshot().iter().any(|snap| {
            snap.seed_inventory
                .shard_ids
                .iter()
                .any(|id| id == &item.shard_id)
        });
        if found {
            hits += 1;
            record_prefetch_peer_fetch(1);
        } else {
            record_prefetch_peer_fetch_miss();
        }
    }
    hits
}

fn peer_http_prefetch_enabled() -> bool {
    std::env::var(crate::grid::galaxy_prefetch_peer_pull::ENV_PREFETCH_PEER_HTTP_URL)
        .ok()
        .is_some_and(|s| !s.trim().is_empty())
}

/// Default co-access graph stub: admitted shard → speculative neighbors (PH-S446).
pub fn default_co_access_graph() -> std::collections::BTreeMap<String, Vec<String>> {
    let mut map = std::collections::BTreeMap::new();
    map.insert("w:emb-1".into(), vec!["w:ckpt-7".into()]);
    map.insert("w:ckpt-7".into(), vec!["w:emb-1".into()]);
    map
}

/// Co-access graph from env JSON or default stub (PH-S469).
pub fn co_access_graph_from_env() -> std::collections::BTreeMap<String, Vec<String>> {
    if let Ok(raw) = std::env::var(ENV_CO_ACCESS_GRAPH_JSON) {
        if let Ok(map) =
            serde_json::from_str::<std::collections::BTreeMap<String, Vec<String>>>(&raw)
        {
            return map;
        }
    }
    default_co_access_graph()
}

/// Speculative prefetch from co-access graph when shard A is admitted (PH-S446).
pub fn plan_co_access_prefetch(
    admitted_shard_id: &str,
    gpu_capable: bool,
    config: &PrefetchPolicyConfig,
) -> Option<PrefetchPlan> {
    let co_access = co_access_graph_from_env();
    let speculative = co_access.get(admitted_shard_id)?;
    if speculative.is_empty() {
        return None;
    }
    let inventory = coordinator_merged_seed_inventory();
    Some(plan_prefetch(
        &inventory,
        speculative,
        PrefetchTrigger::CoAccessGraph,
        gpu_capable,
        config,
    ))
}

/// Whether all required shards are hot in coordinator inventory (Galaxy §5.6).
pub fn all_required_shards_hot(
    inventory: &SeedInventoryEntry,
    required_shard_ids: &[String],
) -> bool {
    required_shard_ids.iter().all(|id| hot_hit(inventory, id))
}

/// Strict locality gate: reject ingest when required shards are not hot (PH-S445).
pub fn check_strict_locality_gate(required_shard_ids: &[String]) -> Result<(), AppError> {
    if required_shard_ids.is_empty() {
        return Ok(());
    }
    let config = PrefetchPolicyConfig::from_env();
    if config.mode != PrefetchPolicyMode::StrictLocality {
        return Ok(());
    }
    let inventory = coordinator_merged_seed_inventory();
    if all_required_shards_hot(&inventory, required_shard_ids) {
        return Ok(());
    }
    record_locality_unsatisfied();
    Err(AppError::RestError {
        code: "locality_unsatisfied",
        message: "required shards are not hot/local under strict_locality (Galaxy §5.6)".into(),
    })
}

/// Seed pull resolver: planned shards present in coordinator inventory (PH-S434).
pub fn resolve_seed_pull_shards(plan: &PrefetchPlan) -> usize {
    let inventory = coordinator_merged_seed_inventory();
    plan.items
        .iter()
        .filter(|item| inventory.shard_ids.iter().any(|id| id == &item.shard_id))
        .count()
}

/// Seed pull stub (PH-S424/PH-S434): resolve against inventory; no live memory/RAID fetch wire.
#[inline]
pub fn seed_pull_hook(plan: &PrefetchPlan) -> usize {
    let n = resolve_seed_pull_shards(plan);
    if n > 0 {
        record_prefetch_seed_pull(n);
    }
    n
}

/// Outcome of processing one grid envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridIngestKind {
    Job {
        job_id: String,
        status: JobStatus,
        replication_tier: ReplicationTierConfig,
    },
    Result {
        job_id: String,
        status: JobStatus,
        settlement_gate: SettlementGateVerdict,
        verification_sample: VerifySamplingVerdict,
        settlement_status: SettlementStatus,
    },
    MemoryShard {
        shard_id: String,
    },
    PeerStatus {
        peer_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridIngestOutcome {
    pub kind: GridIngestKind,
}

pub fn ingest_envelope(
    env: GridEnvelope,
    jobs: &JobStore,
    memory: &MemoryShardStore,
) -> Result<GridIngestOutcome, AppError> {
    env.validate()
        .map_err(|e: GridEnvelopeError| AppError::ValidationError(e.to_string()))?;
    match env.msg {
        GridMessage::Job(body) => ingest_job(body, env.source_peer_id.as_deref(), jobs, memory),
        GridMessage::Result(body) => ingest_result(body, env.source_peer_id.as_deref(), jobs),
        GridMessage::MemoryShard(body) => {
            let shard = memory_shard_from_grid_body(&body);
            let shard_id = shard.shard_id.0.clone();
            memory.upsert(shard)?;
            let provider = env.source_peer_id.as_deref().unwrap_or("coordinator");
            let digest = memory_content_digest(&body.artifact_id, &body.version);
            emit_memory_updated(
                &body.artifact_id,
                &body.version,
                &digest,
                body.raid_logical_name.as_deref(),
                format!("memory:{}:{}", shard_id, body.version),
            );
            if body.seed_hints.as_ref().is_some_and(|h| !h.is_empty()) {
                emit_seed_provided(
                    &shard_id,
                    provider,
                    &body.artifact_id,
                    format!("seed:{}:{}", shard_id, provider),
                );
            }
            Ok(GridIngestOutcome {
                kind: GridIngestKind::MemoryShard { shard_id },
            })
        }
        GridMessage::PeerStatus(body) => Ok(GridIngestOutcome {
            kind: GridIngestKind::PeerStatus {
                peer_id: body.peer_id,
            },
        }),
    }
}

fn ingest_job(
    body: crate::grid::GridJobBody,
    source_peer_id: Option<&str>,
    jobs: &JobStore,
    memory: &MemoryShardStore,
) -> Result<GridIngestOutcome, AppError> {
    if !body.required_shard_ids.is_empty() {
        check_strict_locality_gate(&body.required_shard_ids)?;
    }
    check_telegram_edge_capability_admission(source_peer_id, &body.task_kind)?;
    let spec = job_spec_from_grid_job(&body);
    let job_id = spec.id.0.clone();
    let record = JobRecord {
        spec,
        status: JobStatus::Submitted,
        created_at: Utc::now(),
        worker_id: None,
        vm_id: None,
        lease_owner: None,
        lease_epoch: None,
        lease_expires_at: None,
        migration_count: None,
        fail_reason: None,
        leased_at: None,
    };
    jobs.push(record)?;
    let locality_peer = if !body.required_shard_ids.is_empty() {
        ingest_job_locality_rank_stub(&body.required_shard_ids, &body.task_kind)
    } else {
        None
    };
    let schedule_peer = locality_peer.as_deref().or(source_peer_id);
    schedule_with_grid_peer(jobs, schedule_peer)?;
    if !body.required_shard_ids.is_empty() {
        with_prefetch_peer(source_peer_id, || {
            ingest_job_prefetch_stub(
                &body.required_shard_ids,
                grid_job_gpu_capable(&body.task_kind),
                Some(memory),
            );
            evaluate_strict_prefetch_timeout(&body.required_shard_ids, memory)
        })?;
    }
    let row = jobs
        .get(&job_id)?
        .ok_or_else(|| AppError::InternalError("job missing after grid ingest".into()))?;
    let replication_tier = replication_tier_from_policy(body.verification_policy.as_deref());
    replication_executor_hook(replication_tier, Some(jobs), Some(&job_id));
    Ok(GridIngestOutcome {
        kind: GridIngestKind::Job {
            job_id,
            status: row.status,
            replication_tier,
        },
    })
}

fn ingest_result(
    body: GridResultBody,
    source_peer_id: Option<&str>,
    jobs: &JobStore,
) -> Result<GridIngestOutcome, AppError> {
    let job_id = body.job_id.clone();
    let existing = jobs
        .get(&job_id)?
        .ok_or_else(|| AppError::ApiNotFound(format!("job '{job_id}' not found")))?;
    let now = Utc::now();
    if let Err(PatchLeaseEpochError::Rejected) =
        check_grid_result_lease_epoch(&existing, body.lease_epoch, now)
    {
        trace_lease_reject(
            &job_id,
            LeaseOperation::GridResultCas,
            LeaseSource::GridIngest,
            LeaseOutcome::Rejected,
            "lease_epoch_rejected",
            existing.lease_epoch,
            body.lease_epoch,
            Some(409),
        );
        if let Some(peer_id) = source_peer_id {
            apply_lease_epoch_rejected_trust_delta(peer_id);
        }
        return Err(AppError::RestError {
            code: "lease_epoch_rejected",
            message: format!(
                "lease_epoch does not match active lease for job '{job_id}' (Galaxy ?4.3.1 grid result CAS)"
            ),
        });
    }
    let status = job_status_from_grid_result(body.status);
    jobs.force_status(&job_id, status)?;
    let trust_score = trust_score_from_result_metrics(body.metrics.as_ref());
    let semantic_hash_match = evaluate_semantic_hash_verification(body.metrics.as_ref());
    let is_mismatch = evaluate_result_verification_mismatch(body.metrics.as_ref())
        || semantic_hash_match == Some(false);
    let is_match = evaluate_result_verification_match(body.metrics.as_ref())
        || semantic_hash_match == Some(true);
    if is_match || is_mismatch {
        drain_verification_checker_task(&job_id);
    }
    let verify_cfg = VerifySamplingConfig::from_env();
    if let timeout_verdict @ VerifySamplingVerdict::SampleScheduled
    | timeout_verdict @ VerifySamplingVerdict::VerificationInconclusive =
        evaluate_checker_timeout_policy(&job_id, body.metrics.as_ref(), &verify_cfg)
    {
        if timeout_verdict == VerifySamplingVerdict::SampleScheduled {
            let _ = submit_shadow_verification_checker_job(jobs, &job_id);
        }
    }
    if body
        .metrics
        .as_ref()
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("mismatch"))
    {
        let _ = evaluate_post_mismatch_elevated_sampling(&job_id, &verify_cfg);
    }
    let mut effective_trust_score = trust_score;
    if let Some(verdict) = body
        .metrics
        .as_ref()
        .and_then(|m| m.get("verification_verdict"))
        .and_then(|v| v.as_str())
    {
        if verdict.eq_ignore_ascii_case("match") || verdict.eq_ignore_ascii_case("mismatch") {
            let base = trust_score
                .or_else(|| source_peer_id.and_then(lookup_peer_trust_score))
                .unwrap_or(DEFAULT_TRUST_SCORE);
            effective_trust_score = Some(apply_verification_trust_delta(verdict, base));
        }
    }
    evaluate_result_verification_sample_completed(body.metrics.as_ref());
    let settlement_gate = evaluate_result_settlement_gate(
        source_peer_id,
        effective_trust_score,
        &TrustScoreGateConfig::from_env(),
    );
    let verification_sample =
        evaluate_result_verify_sampling(source_peer_id, &job_id, &VerifySamplingConfig::from_env());
    let sample_scheduled = verification_sample == VerifySamplingVerdict::SampleScheduled;
    evaluate_result_verification_sample(body.metrics.as_ref(), sample_scheduled);
    if sample_scheduled {
        enqueue_verification_checker_task(&job_id);
        let _ = submit_shadow_verification_checker_job(jobs, &job_id);
    }
    let mut settlement_status = resolve_settlement_status(settlement_gate, verification_sample);
    record_result_executor_digest(&job_id, body.metrics.as_ref());
    if settlement_status == SettlementStatus::Cleared
        && !replication_quorum_allows_cleared(&job_id, existing.spec.verification_policy.as_deref())
    {
        settlement_status = SettlementStatus::PendingVerification;
    }
    if settlement_status == SettlementStatus::Cleared
        && evaluate_semantic_hash_human_review_hold(body.metrics.as_ref())
    {
        settlement_status = SettlementStatus::PendingVerification;
        record_settlement_human_review();
    }
    if evaluate_fraud_proof_hold(is_mismatch) {
        record_fraud_proof_pending();
        settlement_status = SettlementStatus::PendingVerification;
    }
    if let (Some(peer_id), Some(score)) = (source_peer_id, effective_trust_score) {
        persist_peer_trust_score(peer_id, score);
    }
    evaluate_result_settlement_resolved(settlement_status);
    evaluate_result_settlement_pending_verification(settlement_status);
    evaluate_result_settlement_cleared(settlement_status);
    if settlement_status == SettlementStatus::Cleared {
        let entry = build_payout_batch_entry(
            &job_id,
            now.to_rfc3339(),
            body.metrics.as_ref(),
            source_peer_id,
        );
        record_payout_batch_ledger_entry(entry.clone());
        emit_settlement_job_rewarded(&entry, source_peer_id.unwrap_or("coordinator"));
    }
    evaluate_result_settlement_not_applicable(settlement_status);
    evaluate_result_fee_split(body.metrics.as_ref());
    evaluate_result_replay_pending(&job_id, body.metrics.as_ref(), settlement_status);
    let _ = submit_replay_verification_job(jobs, &job_id, body.metrics.as_ref(), settlement_status);
    Ok(GridIngestOutcome {
        kind: GridIngestKind::Result {
            job_id,
            status,
            settlement_gate,
            verification_sample,
            settlement_status,
        },
    })
}

/// Build payout batch ledger entry with optional fee-split fields (PH-S521).
fn build_payout_batch_entry(
    job_id: &str,
    cleared_at: String,
    metrics: Option<&serde_json::Value>,
    source_peer_id: Option<&str>,
) -> PayoutBatchLedgerEntry {
    let telegram_user_id = metrics
        .and_then(|m| m.get("telegram_user_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let mut entry = PayoutBatchLedgerEntry {
        job_id: job_id.to_string(),
        cleared_at,
        gross_usd_micro: metrics.and_then(|m| m.get("gross_usd_micro").and_then(|v| v.as_u64())),
        gross_lamports: None,
        primary_dev_lamports: None,
        secondary_admin_lamports: None,
        worker_lamports: None,
        payout_pubkey: resolve_payout_pubkey(telegram_user_id.as_deref()),
        telegram_user_id,
    };
    let Some(m) = metrics else {
        return entry;
    };
    let gross = m.get("gross_lamports").and_then(|v| v.as_u64());
    let bps = m.get("secondary_admin_bps").and_then(|v| v.as_u64());
    if let (Some(gross_lamports), Some(bps_raw)) = (gross, bps) {
        if bps_raw <= u64::from(u16::MAX) {
            let bps = bps_raw as u16;
            if (SECONDARY_ADMIN_FEE_MIN_BPS..=SECONDARY_ADMIN_FEE_MAX_BPS).contains(&bps) {
                if let Ok(split) = split_gross_payment(gross_lamports, bps) {
                    entry.gross_lamports = Some(gross_lamports);
                    entry.primary_dev_lamports = Some(split.primary_dev_lamports);
                    entry.secondary_admin_lamports = Some(split.secondary_admin_lamports);
                    entry.worker_lamports = Some(split.worker_or_operator_pool_lamports);
                }
            }
        }
    }
    entry
}

/// Optional `trust_score` on grid result metrics (PH-S130 stub wire).
fn trust_score_from_result_metrics(metrics: Option<&serde_json::Value>) -> Option<TrustScore> {
    metrics
        .and_then(|m| m.get("trust_score"))
        .and_then(|v| v.as_u64())
        .map(|v| clamp_trust_score(v as u16))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement::{resolve_settlement_status, SettlementStatus};
    use crate::grid::galaxy_trust_score::SettlementGateVerdict;
    use crate::grid::{GridEnvelope, GridJobBody, GridMessage, GridResultStatus};
    use crate::job::{JobId, JobKind, JobSpec, JobStatus};
    use chrono::Utc;

    #[test]
    fn ingest_job_with_peer_sets_leased_and_lease_fields() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-1".into(),
                task_kind: "inference".into(),
                verification_policy: None,
                input_artifact_ids: vec![],
                required_shard_ids: vec![],
                deadline: None,
            }),
            Some("peer-a".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Job {
                job_id: "grid-job-1".into(),
                status: JobStatus::Leased,
                replication_tier: REPLICATION_STANDARD,
            }
        );
        let row = jobs.get("grid-job-1").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
        assert_eq!(row.worker_id.as_deref(), Some("peer-a"));
        assert_eq!(row.lease_owner.as_deref(), Some("peer-a"));
        assert_eq!(row.lease_epoch, Some(1));
        assert!(row.lease_expires_at.is_some());
    }

    #[test]
    fn ingest_job_without_peer_stays_scheduled_without_lease() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-no-peer".into(),
                task_kind: "inference".into(),
                verification_policy: None,
                input_artifact_ids: vec![],
                required_shard_ids: vec![],
                deadline: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Job {
                job_id: "grid-job-no-peer".into(),
                status: JobStatus::Scheduled,
                replication_tier: REPLICATION_STANDARD,
            }
        );
        let row = jobs.get("grid-job-no-peer").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Scheduled);
        assert!(row.worker_id.is_none());
        assert!(row.lease_owner.is_none());
        assert!(row.lease_epoch.is_none());
        assert!(row.lease_expires_at.is_none());
    }

    #[test]
    fn ingest_job_resolves_replication_strict_tier_ph_s171() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-strict".into(),
                task_kind: "inference".into(),
                verification_policy: Some("replication_strict".into()),
                input_artifact_ids: vec![],
                required_shard_ids: vec![],
                deadline: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        match out.kind {
            GridIngestKind::Job {
                replication_tier, ..
            } => assert_eq!(replication_tier, REPLICATION_STRICT),
            other => panic!("expected Job kind, got {other:?}"),
        }
    }

    #[test]
    fn ingest_job_wire_records_replication_strict_metric_ph_s179() {
        use crate::grid::galaxy_replication_metrics::{
            replication_metrics_test_lock, replication_strict_total,
            reset_replication_strict_metrics_for_test,
        };

        let _lock = replication_metrics_test_lock();
        reset_replication_strict_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::Job(GridJobBody {
                job_id: "grid-job-strict-metric".into(),
                task_kind: "inference".into(),
                verification_policy: Some("replication_strict".into()),
                input_artifact_ids: vec![],
                required_shard_ids: vec![],
                deadline: None,
            }),
            None,
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(replication_strict_total(), 1);
        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn ingest_result_updates_job() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-job-2"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-job-2".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec!["out-1".into()],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-job-2".into(),
                status: JobStatus::Completed,
                settlement_gate: SettlementGateVerdict::NotApplicable,
                verification_sample: VerifySamplingVerdict::NotApplicable,
                settlement_status: SettlementStatus::NotApplicable,
            }
        );
    }

    #[test]
    fn ingest_result_accepts_matching_lease_epoch() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-ok"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(3),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-ok".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec!["out-ok".into()],
                proof: None,
                metrics: None,
                lease_epoch: Some(3),
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-result-ok".into(),
                status: JobStatus::Completed,
                settlement_gate: SettlementGateVerdict::NotApplicable,
                verification_sample: VerifySamplingVerdict::NotApplicable,
                settlement_status: SettlementStatus::NotApplicable,
            }
        );
    }

    #[test]
    fn ingest_result_rejects_lease_epoch_mismatch() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-bad"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(5),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-bad".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: Some(4),
            }),
            None,
        );
        let err = ingest_envelope(env, &jobs, &memory).expect_err("reject");
        match err {
            AppError::RestError { code, .. } => assert_eq!(code, "lease_epoch_rejected"),
            other => panic!("expected RestError, got {other:?}"),
        }
        let row = jobs.get("grid-result-bad").expect("get").expect("row");
        assert_eq!(row.status, JobStatus::Leased);
    }

    #[test]
    fn ingest_result_rejects_missing_lease_epoch_on_leased_job() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let now = Utc::now();
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-result-no-epoch"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: now,
            worker_id: Some("peer-r".into()),
            vm_id: None,
            lease_owner: Some("peer-r".into()),
            lease_epoch: Some(1),
            lease_expires_at: Some(now + chrono::Duration::seconds(90)),
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-result-no-epoch".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            None,
        );
        let err = ingest_envelope(env, &jobs, &memory).expect_err("reject");
        match err {
            AppError::RestError { code, .. } => assert_eq!(code, "lease_epoch_rejected"),
            other => panic!("expected RestError, got {other:?}"),
        }
    }

    #[test]
    fn ingest_memory_shard_upserts() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope::new(
            GridMessage::MemoryShard(crate::grid::GridMemoryShardBody {
                shard_id: "w:1".into(),
                artifact_id: "art-1".into(),
                version: "1".into(),
                raid_logical_name: Some("weights".into()),
                seed_hints: None,
            }),
            None,
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::MemoryShard {
                shard_id: "w:1".into()
            }
        );
        let shard = memory.get("w:1").expect("get").expect("row");
        assert_eq!(shard.artifact_id, "art-1");
    }

    #[test]
    fn coordinator_seed_inventory_snapshot_has_two_peers_ph_s195() {
        let rows = coordinator_seed_inventory_snapshot();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].peer_id, "srv1-worker-a");
        assert!(rows[0]
            .seed_inventory
            .shard_ids
            .contains(&"w:emb-1".to_string()));
    }

    #[test]
    fn seed_inventory_entry_roundtrip_json() {
        let entry = SeedInventoryEntry {
            shard_ids: vec!["w:emb-1".into()],
            hot_tier: SeedInventoryHotTier {
                ram_bytes_used: 1024,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            local_replica_regions: vec!["eu-west".into()],
            last_inventory_at: Some("2026-05-27T10:00:00Z".into()),
        };
        let json = serde_json::to_string(&entry).expect("serialize");
        let back: SeedInventoryEntry = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(entry, back);
    }

    #[test]
    fn plan_prefetch_skips_hot_shards() {
        let inventory = SeedInventoryEntry {
            shard_ids: vec!["w:emb-1".into(), "w:ckpt-7".into()],
            hot_tier: SeedInventoryHotTier {
                ram_bytes_used: 4096,
                vram_bytes_used: 0,
                profiles: vec!["inference:text".into()],
            },
            ..Default::default()
        };
        let required = vec!["w:emb-1".into(), "w:missing".into()];
        let plan = plan_prefetch(
            &inventory,
            &required,
            PrefetchTrigger::JobAdmitted,
            false,
            &PrefetchPolicyConfig::default(),
        );
        assert_eq!(plan.items.len(), 1);
        assert_eq!(plan.items[0].shard_id, "w:missing");
        assert_eq!(plan.items[0].target_tier, PrefetchTargetTier::Ram);
        assert_eq!(plan.trigger, PrefetchTrigger::JobAdmitted);
        assert_eq!(plan.deadline_ms, DEFAULT_PREFETCH_DEADLINE_MS);
    }

    #[test]
    fn plan_prefetch_records_metrics_ph_s167() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_bytes_total, prefetch_hot_skip_total, prefetch_plan_total,
            prefetch_planned_shards_total, reset_prefetch_metrics_for_test,
            DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM,
        };
        reset_prefetch_metrics_for_test();
        let inventory = SeedInventoryEntry {
            shard_ids: vec!["w:hot".into()],
            hot_tier: SeedInventoryHotTier {
                ram_bytes_used: 1024,
                vram_bytes_used: 0,
                profiles: vec![],
            },
            ..Default::default()
        };
        let _ = plan_prefetch(
            &inventory,
            &["w:hot".into(), "w:cold".into()],
            PrefetchTrigger::LeaseAcquired,
            false,
            &PrefetchPolicyConfig::default(),
        );
        assert_eq!(prefetch_plan_total(), 1);
        assert_eq!(prefetch_planned_shards_total(), 1);
        assert_eq!(prefetch_hot_skip_total(), 1);
        assert_eq!(prefetch_bytes_total(), DEFAULT_PREFETCH_BYTES_PER_SHARD_RAM);
        use crate::grid::galaxy_locality::{
            last_cross_region_egress_mb, reset_last_cross_region_egress_mb_for_test,
        };
        assert_eq!(last_cross_region_egress_mb(), 50);
        reset_prefetch_metrics_for_test();
        reset_last_cross_region_egress_mb_for_test();
    }

    #[test]
    fn plan_prefetch_gpu_uses_vram_tier() {
        let inventory = SeedInventoryEntry::default();
        let required = vec!["w:gpu-1".into()];
        let config = PrefetchPolicyConfig {
            mode: PrefetchPolicyMode::StrictLocality,
            prefetch_deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
        };
        let plan = plan_prefetch(
            &inventory,
            &required,
            PrefetchTrigger::LeaseAcquired,
            true,
            &config,
        );
        assert_eq!(plan.items[0].target_tier, PrefetchTargetTier::Vram);
        assert_eq!(plan.mode, PrefetchPolicyMode::StrictLocality);
    }

    #[test]
    fn ingest_result_telegram_edge_default_trust_eligible() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-trust-ok"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-trust-ok".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            Some("tg-edge-1".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        let cfg = VerifySamplingConfig::default_stub();
        let expected_sample = if crate::grid::galaxy_verify_sampling::deterministic_sample_selected(
            "grid-trust-ok",
            cfg.base_sample_rate,
        ) {
            VerifySamplingVerdict::SampleScheduled
        } else {
            VerifySamplingVerdict::NotSelected
        };
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-trust-ok".into(),
                status: JobStatus::Completed,
                settlement_gate: SettlementGateVerdict::PayoutEligible,
                verification_sample: expected_sample,
                settlement_status: resolve_settlement_status(
                    SettlementGateVerdict::PayoutEligible,
                    expected_sample,
                ),
            }
        );
    }

    #[test]
    fn ingest_result_telegram_edge_low_trust_holds_payout() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-trust-low"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-trust-low".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({ "trust_score": 15 })),
                lease_epoch: None,
            }),
            Some("tg-edge-low".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        let cfg = VerifySamplingConfig::default_stub();
        let expected_sample = if crate::grid::galaxy_verify_sampling::deterministic_sample_selected(
            "grid-trust-low",
            cfg.base_sample_rate,
        ) {
            VerifySamplingVerdict::SampleScheduled
        } else {
            VerifySamplingVerdict::NotSelected
        };
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-trust-low".into(),
                status: JobStatus::Completed,
                settlement_gate: SettlementGateVerdict::PayoutHeld,
                verification_sample: expected_sample,
                settlement_status: SettlementStatus::PendingVerification,
            }
        );
    }

    #[test]
    fn ingest_result_wire_applies_verify_sampling_from_env() {
        use crate::grid::galaxy_verification_metrics::{
            reset_verification_sample_metrics_for_test, verification_sample_total,
        };
        use crate::grid::galaxy_verify_sampling::{
            reset_verify_sampling_metrics_for_test, verify_sample_scheduled_total,
        };

        reset_verify_sampling_metrics_for_test();
        reset_verification_sample_metrics_for_test();
        std::env::set_var(
            crate::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE,
            "1",
        );
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-verify-wire"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-verify-wire".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: None,
                lease_epoch: None,
            }),
            Some("tg-verify".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(
            out.kind,
            GridIngestKind::Result {
                job_id: "grid-verify-wire".into(),
                status: JobStatus::Completed,
                settlement_gate: SettlementGateVerdict::PayoutEligible,
                verification_sample: VerifySamplingVerdict::SampleScheduled,
                settlement_status: SettlementStatus::PendingVerification,
            }
        );
        assert_eq!(verify_sample_scheduled_total(), 1);
        assert_eq!(verification_sample_total(), 1);
        std::env::remove_var(crate::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE);
        reset_verify_sampling_metrics_for_test();
        reset_verification_sample_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_verification_sample_metric_ph_s177() {
        use crate::grid::galaxy_verification_metrics::{
            reset_verification_sample_metrics_for_test, verification_metrics_test_lock,
            verification_sample_total,
        };

        let _lock = verification_metrics_test_lock();
        reset_verification_sample_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-verify-sample-metric"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-verify-sample-metric".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({ "verification_sample": true })),
                lease_epoch: None,
            }),
            Some("peer-local".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(verification_sample_total(), 1);
        reset_verification_sample_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_updates_trust_settlement_metrics() {
        use crate::grid::galaxy_trust_score::{
            payout_eligible_total, payout_held_total, reset_settlement_gate_metrics_for_test,
        };

        reset_settlement_gate_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);

        for (job_id, trust) in [
            ("grid-trust-m-eligible", None),
            ("grid-trust-m-held", Some(10u64)),
        ] {
            jobs.push(JobRecord {
                spec: JobSpec {
                    id: JobId::new(job_id),
                    kind: JobKind::Inference,
                    resources: Default::default(),
                    priority: 0,
                    max_duration_secs: None,
                    input_artifact_ids: vec![],
                    verification_policy: None,
                    deadline: None,
                },
                status: JobStatus::Scheduled,
                created_at: Utc::now(),
                worker_id: None,
                vm_id: None,
                lease_owner: None,
                lease_epoch: None,
                lease_expires_at: None,
                migration_count: None,
                fail_reason: None,
                leased_at: None,
            })
            .expect("push");

            let metrics = trust.map(|score| serde_json::json!({ "trust_score": score }));
            let env = GridEnvelope::new(
                GridMessage::Result(crate::grid::GridResultBody {
                    job_id: job_id.into(),
                    status: GridResultStatus::Completed,
                    output_artifact_ids: vec![],
                    proof: None,
                    metrics,
                    lease_epoch: None,
                }),
                Some(format!("tg-{job_id}")),
            );
            ingest_envelope(env, &jobs, &memory).expect("ingest");
        }

        assert_eq!(payout_eligible_total(), 1);
        assert_eq!(payout_held_total(), 1);
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_verification_mismatch_metric_ph_s175() {
        use crate::grid::galaxy_verification_metrics::{
            reset_verification_mismatch_metrics_for_test, verification_metrics_test_lock,
            verification_mismatch_total,
        };

        let _lock = verification_metrics_test_lock();
        reset_verification_mismatch_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-verify-mismatch"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-verify-mismatch".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({
                    "verification_verdict": "mismatch"
                })),
                lease_epoch: None,
            }),
            Some("tg-edge".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(verification_mismatch_total(), 1);
        reset_verification_mismatch_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_verification_match_metric_ph_s180() {
        use crate::grid::galaxy_verification_metrics::{
            reset_verification_match_metrics_for_test, verification_match_total,
            verification_metrics_test_lock,
        };

        let _lock = verification_metrics_test_lock();
        reset_verification_match_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-verify-match"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-verify-match".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({
                    "verification_verdict": "match"
                })),
                lease_epoch: None,
            }),
            Some("tg-edge".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(verification_match_total(), 1);
        reset_verification_match_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_replay_pending_metric_ph_s176() {
        use crate::grid::galaxy_replay_metrics::{
            replay_metrics_test_lock, replay_pending, reset_replay_pending_metrics_for_test,
        };

        let _lock = replay_metrics_test_lock();
        reset_replay_pending_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-replay-pending"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-replay-pending".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({
                    "verification_verdict": "mismatch"
                })),
                lease_epoch: None,
            }),
            Some("tg-edge".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(replay_pending(), 1);
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_settlement_pending_verification_metric_ph_s178() {
        use crate::grid::galaxy_settlement_metrics::{
            reset_settlement_pending_verification_metrics_for_test, settlement_metrics_test_lock,
            settlement_pending_verification_total,
        };

        let _lock = settlement_metrics_test_lock();
        reset_settlement_pending_verification_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-settle-pending-metric"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-settle-pending-metric".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({ "trust_score": 10 })),
                lease_epoch: None,
            }),
            Some("tg-settle-metric".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(settlement_pending_verification_total(), 1);
        reset_settlement_pending_verification_metrics_for_test();
    }

    #[test]
    fn ingest_result_wire_records_settlement_cleared_metric_ph_s187() {
        use crate::grid::galaxy_settlement_metrics::{
            reset_settlement_metrics_for_test, settlement_cleared_total,
            settlement_metrics_test_lock,
        };
        use crate::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE;

        let _lock = settlement_metrics_test_lock();
        reset_settlement_metrics_for_test();
        std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-settle-cleared-metric"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-settle-cleared-metric".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({ "trust_score": 80 })),
                lease_epoch: None,
            }),
            Some("tg-edge".into()),
        );
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(settlement_cleared_total(), 1);
        std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
        reset_settlement_metrics_for_test();
    }

    #[test]
    fn ingest_result_resolves_pending_verification_status_ph_s170() {
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        jobs.push(JobRecord {
            spec: JobSpec {
                id: JobId::new("grid-settle-pending"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

        let env = GridEnvelope::new(
            GridMessage::Result(crate::grid::GridResultBody {
                job_id: "grid-settle-pending".into(),
                status: GridResultStatus::Completed,
                output_artifact_ids: vec![],
                proof: None,
                metrics: Some(serde_json::json!({ "trust_score": 10 })),
                lease_epoch: None,
            }),
            Some("tg-settle".into()),
        );
        let out = ingest_envelope(env, &jobs, &memory).expect("ingest");
        match out.kind {
            GridIngestKind::Result {
                settlement_status,
                settlement_gate,
                ..
            } => {
                assert_eq!(settlement_gate, SettlementGateVerdict::PayoutHeld);
                assert_eq!(settlement_status, SettlementStatus::PendingVerification);
            }
            other => panic!("expected Result kind, got {other:?}"),
        }
    }

    #[test]
    fn noop_prefetch_hook_returns_planned_count() {
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "a".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "b".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::ReMigrate,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(noop_prefetch_hook(&plan), 2);
    }

    #[test]
    fn parse_prefetch_policy_mode_accepts_aliases() {
        assert_eq!(
            parse_prefetch_policy_mode("strict_locality"),
            PrefetchPolicyMode::StrictLocality
        );
        assert_eq!(
            parse_prefetch_policy_mode("best_effort"),
            PrefetchPolicyMode::BestEffort
        );
        assert_eq!(
            parse_prefetch_policy_mode("unknown"),
            PrefetchPolicyMode::BestEffort
        );
    }

    #[test]
    fn prefetch_policy_config_from_env_reads_locality_and_deadline() {
        use std::sync::{Mutex, MutexGuard};

        static ENV_LOCK: Mutex<()> = Mutex::new(());

        fn lock() -> MutexGuard<'static, ()> {
            ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
        }

        fn clear() {
            std::env::remove_var(ENV_LOCALITY_MODE);
            std::env::remove_var(ENV_PREFETCH_DEADLINE_MS);
        }

        let _guard = lock();
        clear();
        assert_eq!(
            PrefetchPolicyConfig::from_env().mode,
            PrefetchPolicyMode::BestEffort
        );

        std::env::set_var(ENV_LOCALITY_MODE, "strict_locality");
        std::env::set_var(ENV_PREFETCH_DEADLINE_MS, "30000");
        let cfg = PrefetchPolicyConfig::from_env();
        assert_eq!(cfg.mode, PrefetchPolicyMode::StrictLocality);
        assert_eq!(cfg.prefetch_deadline_ms, 30_000);

        std::env::set_var(ENV_PREFETCH_DEADLINE_MS, "0");
        assert_eq!(
            PrefetchPolicyConfig::from_env().prefetch_deadline_ms,
            DEFAULT_PREFETCH_DEADLINE_MS
        );

        clear();
    }

    #[test]
    fn plan_prefetch_uses_config_deadline() {
        let cfg = PrefetchPolicyConfig {
            mode: PrefetchPolicyMode::BestEffort,
            prefetch_deadline_ms: 42_000,
        };
        let plan = plan_prefetch(
            &SeedInventoryEntry::default(),
            &["w:x".into()],
            PrefetchTrigger::JobAdmitted,
            false,
            &cfg,
        );
        assert_eq!(plan.deadline_ms, 42_000);
    }

    #[test]
    fn ingest_job_prefetch_stub_ph_s276() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_plan_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let planned = ingest_job_prefetch_stub(&["w:missing-shard".into()], false, None);
        assert_eq!(planned, 1);
        assert_eq!(prefetch_plan_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn ingest_job_with_required_shards_runs_prefetch_stub_ph_s276() {
        use crate::grid::envelope::{
            GridEnvelope, GridJobBody, GridMessage, GRID_ENVELOPE_VERSION,
        };
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_plan_total, reset_prefetch_metrics_for_test,
        };
        use crate::job::JobStore;

        reset_prefetch_metrics_for_test();
        let jobs = JobStore::open_for_test(None);
        let memory = MemoryShardStore::open_for_test(None);
        let env = GridEnvelope {
            v: GRID_ENVELOPE_VERSION,
            sent_at: Utc::now(),
            source_peer_id: None,
            msg: GridMessage::Job(GridJobBody {
                job_id: "job-prefetch-1".into(),
                task_kind: "inference".into(),
                verification_policy: None,
                input_artifact_ids: vec![],
                required_shard_ids: vec!["w:missing-shard".into()],
                deadline: None,
            }),
        };
        ingest_envelope(env, &jobs, &memory).expect("ingest");
        assert_eq!(prefetch_plan_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn enqueue_prefetch_hook_ph_s283() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_enqueue_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "a".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "b".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(enqueue_prefetch_hook(&plan), 2);
        assert_eq!(prefetch_enqueue_total(), 2);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn wait_prefetch_hook_ph_s293() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_wait_ms_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let plan = PrefetchPlan {
            items: vec![PrefetchPlanItem {
                shard_id: "a".into(),
                target_tier: PrefetchTargetTier::Ram,
            }],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(wait_prefetch_hook(&plan), DEFAULT_PREFETCH_DEADLINE_MS);
        assert_eq!(prefetch_wait_ms_total(), DEFAULT_PREFETCH_DEADLINE_MS);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn ingest_job_locality_rank_stub_ph_s285() {
        use crate::grid::galaxy_locality::{
            locality_rank_ingest_total, reset_locality_rank_ingest_for_test,
        };

        reset_locality_rank_ingest_for_test();
        let picked =
            ingest_job_locality_rank_stub(&["w:emb-1".into(), "w:missing".into()], "inference");
        assert!(picked.is_some());
        assert!(picked.unwrap().starts_with("srv"));
        assert_eq!(locality_rank_ingest_total(), 1);
        reset_locality_rank_ingest_for_test();
    }

    #[test]
    fn ingest_job_prefetch_enqueue_ph_s286() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_complete_total, prefetch_enqueue_total, prefetch_ingest_total,
            prefetch_wait_ms_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        ingest_job_prefetch_stub(&["w:missing-shard".into()], false, None);
        assert_eq!(prefetch_ingest_total(), 1);
        assert_eq!(prefetch_enqueue_total(), 1);
        assert_eq!(prefetch_wait_ms_total(), DEFAULT_PREFETCH_DEADLINE_MS);
        assert_eq!(prefetch_complete_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn plan_prefetch_strict_mode_ph_s303() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_strict_mode_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let inventory = SeedInventoryEntry::default();
        let config = PrefetchPolicyConfig {
            mode: PrefetchPolicyMode::StrictLocality,
            prefetch_deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
        };
        let _plan = plan_prefetch(
            &inventory,
            &["w:cold-shard".into()],
            PrefetchTrigger::JobAdmitted,
            false,
            &config,
        );
        assert_eq!(prefetch_strict_mode_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn complete_prefetch_hook_ph_s307() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_complete_total, prefetch_enqueue_total, prefetch_wait_ms_total,
            reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let plan = PrefetchPlan {
            items: vec![PrefetchPlanItem {
                shard_id: "a".into(),
                target_tier: PrefetchTargetTier::Ram,
            }],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(complete_prefetch_hook(&plan, None), 1);
        assert_eq!(prefetch_enqueue_total(), 1);
        assert_eq!(prefetch_wait_ms_total(), DEFAULT_PREFETCH_DEADLINE_MS);
        assert_eq!(prefetch_complete_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn record_prefetch_ingest_hook_ph_s313() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_ingest_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        assert_eq!(ingest_job_prefetch_stub(&[], false, None), 0);
        assert_eq!(prefetch_ingest_total(), 0);
        ingest_job_prefetch_stub(&["w:x".into()], false, None);
        assert_eq!(prefetch_ingest_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn seed_pull_hook_resolves_inventory_ph_s434() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_seed_pull_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "w:emb-1".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "w:missing".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(resolve_seed_pull_shards(&plan), 1);
        assert_eq!(seed_pull_hook(&plan), 1);
        assert_eq!(prefetch_seed_pull_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn fetch_seed_shards_hook_ph_s444() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_seed_fetch_miss_total, prefetch_seed_fetch_total,
            reset_prefetch_metrics_for_test,
        };
        use crate::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};

        reset_prefetch_metrics_for_test();
        let memory = MemoryShardStore::open_for_test(None);
        memory
            .upsert(MemoryShardRef {
                shard_id: MemoryShardId::new("w:emb-1"),
                artifact_id: "a1".into(),
                version: "1".into(),
                raid_logical_name: None,
                seed_hints: None,
            })
            .unwrap();
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "w:emb-1".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "w:missing".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(fetch_seed_shards_hook(&plan, &memory), 1);
        assert_eq!(prefetch_seed_fetch_total(), 1);
        assert_eq!(prefetch_seed_fetch_miss_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn check_strict_locality_gate_ph_s445() {
        use std::sync::{Mutex, MutexGuard};

        static ENV_LOCK: Mutex<()> = Mutex::new(());

        fn lock() -> MutexGuard<'static, ()> {
            ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
        }

        let _guard = lock();
        std::env::remove_var(ENV_LOCALITY_MODE);
        assert!(check_strict_locality_gate(&["w:missing".into()]).is_ok());

        std::env::set_var(ENV_LOCALITY_MODE, "strict_locality");
        assert!(check_strict_locality_gate(&["w:missing-shard".into()]).is_err());
        assert!(check_strict_locality_gate(&["w:emb-1".into()]).is_ok());
        std::env::remove_var(ENV_LOCALITY_MODE);
    }

    #[test]
    fn plan_co_access_prefetch_ph_s446() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_co_access_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let config = PrefetchPolicyConfig::default();
        let plan = plan_co_access_prefetch("w:emb-1", false, &config).expect("co-access plan");
        assert_eq!(plan.trigger, PrefetchTrigger::CoAccessGraph);
        assert_eq!(prefetch_co_access_total(), 1);
        assert!(plan_co_access_prefetch("w:unknown", false, &config).is_none());
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn re_migrate_prefetch_stub_ph_s454() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_re_migrate_total, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let n = re_migrate_prefetch_stub(None);
        assert!(n > 0 || prefetch_re_migrate_total() == 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn prefetch_backpressure_skip_ph_s464() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_backpressure_total, prefetch_enqueue_total, reset_prefetch_metrics_for_test,
        };

        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
        std::env::remove_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS);
        reset_prefetch_metrics_for_test();
        assert!(!prefetch_backpressure_skip());

        std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");
        std::env::set_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS, "10");
        assert!(prefetch_backpressure_skip());
        assert_eq!(prefetch_backpressure_total(), 1);

        let plan = PrefetchPlan {
            items: vec![PrefetchPlanItem {
                shard_id: "w:emb-1".into(),
                target_tier: PrefetchTargetTier::Ram,
            }],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        reset_prefetch_metrics_for_test();
        std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");
        std::env::set_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS, "10");
        assert_eq!(enqueue_prefetch_hook(&plan), 0);
        std::env::set_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS, "200");
        assert_eq!(enqueue_prefetch_hook(&plan), 1);
        assert_eq!(prefetch_enqueue_total(), 1);
        std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
        std::env::remove_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn fetch_seed_shards_from_raid_hook_ph_s465() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_raid_fetch_miss_total, prefetch_raid_fetch_total,
            reset_prefetch_metrics_for_test,
        };
        use crate::memory::{MemoryShardId, MemoryShardRef, MemoryShardStore};

        reset_prefetch_metrics_for_test();
        let memory = MemoryShardStore::open_for_test(None);
        let shard = MemoryShardRef {
            shard_id: MemoryShardId::new("w:emb-1"),
            artifact_id: "art-raid-1".into(),
            version: "v1".into(),
            raid_logical_name: Some("weights".into()),
            seed_hints: None,
        };
        memory.upsert(shard).unwrap();
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "w:emb-1".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "w:missing".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(fetch_seed_shards_from_raid_hook(&plan, &memory), 1);
        assert_eq!(prefetch_raid_fetch_total(), 1);
        assert_eq!(prefetch_raid_fetch_miss_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn co_access_graph_from_env_ph_s469() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_CO_ACCESS_GRAPH_JSON);
        let default = co_access_graph_from_env();
        assert!(default.contains_key("w:emb-1"));

        std::env::set_var(ENV_CO_ACCESS_GRAPH_JSON, r#"{"w:custom":["w:neighbor"]}"#);
        let custom = co_access_graph_from_env();
        assert_eq!(
            custom.get("w:custom").map(|v| v.as_slice()),
            Some(["w:neighbor".to_string()].as_slice())
        );
        let config = PrefetchPolicyConfig::default();
        assert!(plan_co_access_prefetch("w:custom", false, &config).is_some());
        std::env::remove_var(ENV_CO_ACCESS_GRAPH_JSON);
    }

    #[test]
    fn prefetch_egress_blocked_skip_ph_s474() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_egress_blocked_total, reset_prefetch_metrics_for_test,
        };

        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_prefetch_metrics_for_test();
        std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
        std::env::remove_var(ENV_PREFETCH_PEER_REGION);
        std::env::remove_var(ENV_PREFETCH_PEER_EGRESS_POLICY);
        assert!(!prefetch_egress_blocked_skip());

        std::env::set_var(ENV_PREFETCH_COORDINATOR_REGION, "eu-west");
        std::env::set_var(ENV_PREFETCH_PEER_REGION, "us-east");
        std::env::set_var(ENV_PREFETCH_PEER_EGRESS_POLICY, "lan_only");
        assert!(prefetch_egress_blocked_skip());
        assert_eq!(prefetch_egress_blocked_total(), 1);

        std::env::set_var(ENV_PREFETCH_PEER_REGION, "eu-west");
        reset_prefetch_metrics_for_test();
        assert!(!prefetch_egress_blocked_skip());
        std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
        std::env::remove_var(ENV_PREFETCH_PEER_REGION);
        std::env::remove_var(ENV_PREFETCH_PEER_EGRESS_POLICY);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn fetch_seed_shards_from_peer_hook_ph_s479() {
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_peer_fetch_miss_total, prefetch_peer_fetch_total,
            reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        let plan = PrefetchPlan {
            items: vec![
                PrefetchPlanItem {
                    shard_id: "w:emb-1".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
                PrefetchPlanItem {
                    shard_id: "w:missing".into(),
                    target_tier: PrefetchTargetTier::Ram,
                },
            ],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: DEFAULT_PREFETCH_DEADLINE_MS,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(fetch_seed_shards_from_peer_hook(&plan), 1);
        assert_eq!(prefetch_peer_fetch_total(), 1);
        assert_eq!(prefetch_peer_fetch_miss_total(), 1);
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn prefetch_profile_gates_ph_s591_s592() {
        use crate::grid::galaxy_network_profile_store::{
            persist_peer_network_profile, reset_network_profile_store_for_test,
        };
        use crate::grid::galaxy_prefetch_metrics::{
            prefetch_backpressure_total, prefetch_egress_blocked_total,
            reset_prefetch_metrics_for_test,
        };

        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_network_profile_store_for_test();
        reset_prefetch_metrics_for_test();
        std::env::remove_var(ENV_PREFETCH_PEER_BANDWIDTH_MBPS);
        std::env::remove_var(ENV_PREFETCH_PEER_REGION);
        std::env::remove_var(ENV_PREFETCH_PEER_EGRESS_POLICY);
        std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
        std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);

        std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");
        persist_peer_network_profile(
            "peer-bw-low",
            r#"{"region":"eu-west","latency_ms_p50":20,"bandwidth_mbps":10}"#,
        )
        .expect("persist");
        with_prefetch_peer(Some("peer-bw-low"), || {
            assert!(prefetch_backpressure_skip());
            assert_eq!(prefetch_backpressure_total(), 1);
        });

        reset_prefetch_metrics_for_test();
        std::env::set_var(ENV_PREFETCH_COORDINATOR_REGION, "eu-west");
        persist_peer_network_profile(
            "peer-egress",
            r#"{"region":"us-east","latency_ms_p50":20,"egress_policy":"lan_only"}"#,
        )
        .expect("persist");
        with_prefetch_peer(Some("peer-egress"), || {
            assert!(prefetch_egress_blocked_skip());
            assert_eq!(prefetch_egress_blocked_total(), 1);
        });

        std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
        std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
        reset_network_profile_store_for_test();
        reset_prefetch_metrics_for_test();
    }
}
