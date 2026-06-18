//! PH-S07 / FM-043: Prometheus text exposition (`GET /metrics`).
//!
//! Pull-model metrics complement FM-038 OTLP tracing (no duplicate export path).
//! Gauges are refreshed on each scrape from [`ApiContext`] / enterprise monitoring.

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Router,
};
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, IntGauge, Opts, Registry, TextEncoder,
};
use std::sync::OnceLock;

use crate::core::state::ApiContext;
use crate::grid::galaxy_fee_split_metrics::{
    fee_split_applied_total, METRIC_FEE_SPLIT_APPLIED_TOTAL,
};
use crate::grid::galaxy_governance_metrics::{
    release_verify_fail_total, release_verify_total, update_notify_pending,
    METRIC_RELEASE_VERIFY_FAIL_TOTAL, METRIC_RELEASE_VERIFY_TOTAL, METRIC_UPDATE_NOTIFY_PENDING,
};
use crate::grid::galaxy_locality::{
    last_cross_region_egress_mb, last_shard_local_hit_ratio_bps, locality_rank_empty_workers_total,
    locality_rank_ingest_total, locality_rank_miss_total, locality_rank_skip_total,
    METRIC_CROSS_REGION_EGRESS_MB, METRIC_LOCALITY_RANK_EMPTY_WORKERS_TOTAL,
    METRIC_LOCALITY_RANK_INGEST_TOTAL, METRIC_LOCALITY_RANK_MISS_TOTAL,
    METRIC_LOCALITY_RANK_SKIP_TOTAL, METRIC_SHARD_LOCAL_HIT_RATIO,
};
use crate::grid::galaxy_prefetch_metrics::{
    hot_evict_total, hot_promote_total, locality_unsatisfied_total, prefetch_backpressure_total,
    prefetch_bytes_total, prefetch_co_access_total, prefetch_complete_total,
    prefetch_egress_blocked_total, prefetch_enqueue_total, prefetch_hot_skip_total,
    prefetch_ingest_total, prefetch_lease_acquired_total, prefetch_peer_fetch_miss_total,
    prefetch_peer_fetch_total, prefetch_plan_total, prefetch_planned_shards_total,
    prefetch_pull_bytes_total, prefetch_queue_depth, prefetch_raid_fetch_miss_total,
    prefetch_raid_fetch_total, prefetch_re_migrate_total, prefetch_seed_fetch_miss_total,
    prefetch_seed_fetch_total, prefetch_seed_pull_total, prefetch_skip_ingest_total,
    prefetch_strict_mode_total, prefetch_wait_ms_total, shard_access_total, METRIC_HOT_EVICT_TOTAL,
    METRIC_HOT_PROMOTE_TOTAL, METRIC_LOCALITY_UNSATISFIED_TOTAL,
    METRIC_PREFETCH_BACKPRESSURE_TOTAL, METRIC_PREFETCH_BYTES_TOTAL,
    METRIC_PREFETCH_COMPLETE_TOTAL, METRIC_PREFETCH_CO_ACCESS_TOTAL,
    METRIC_PREFETCH_EGRESS_BLOCKED_TOTAL, METRIC_PREFETCH_ENQUEUE_TOTAL,
    METRIC_PREFETCH_HOT_SKIP_TOTAL, METRIC_PREFETCH_INGEST_TOTAL,
    METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL, METRIC_PREFETCH_PEER_FETCH_MISS_TOTAL,
    METRIC_PREFETCH_PEER_FETCH_TOTAL, METRIC_PREFETCH_PLANNED_SHARDS_TOTAL,
    METRIC_PREFETCH_PLAN_TOTAL, METRIC_PREFETCH_PULL_BYTES_TOTAL, METRIC_PREFETCH_QUEUE_DEPTH,
    METRIC_PREFETCH_RAID_FETCH_MISS_TOTAL, METRIC_PREFETCH_RAID_FETCH_TOTAL,
    METRIC_PREFETCH_RE_MIGRATE_TOTAL, METRIC_PREFETCH_SEED_FETCH_MISS_TOTAL,
    METRIC_PREFETCH_SEED_FETCH_TOTAL, METRIC_PREFETCH_SEED_PULL_TOTAL,
    METRIC_PREFETCH_SKIP_INGEST_TOTAL, METRIC_PREFETCH_STRICT_MODE_TOTAL,
    METRIC_PREFETCH_WAIT_MS_TOTAL, METRIC_SHARD_ACCESS_TOTAL,
};
use crate::grid::galaxy_pricing_oracle::{
    forced_fallback_total, fresh_served_total, last_market_min_usd_micro, last_quote_usd_micro,
    pricing_cache_age_seconds, stale_served_total, METRIC_CACHE_AGE_SECONDS,
    METRIC_FORCED_FALLBACK_TOTAL, METRIC_FRESH_SERVED_TOTAL, METRIC_MARKET_MIN_USD_MICRO,
    METRIC_QUOTE_USD_MICRO, METRIC_STALE_SERVED_TOTAL,
};
use crate::grid::galaxy_pricing_provider_metrics::{
    provider_catalog_hits_total, provider_catalog_lookups_total, provider_errors_total,
    METRIC_PROVIDER_CATALOG_HITS_TOTAL, METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL,
    METRIC_PROVIDER_ERRORS_TOTAL,
};
use crate::grid::galaxy_protocol_negotiation_metrics::{
    protocol_negotiation_accepted_total, protocol_negotiation_rejected_total,
    METRIC_PROTOCOL_NEGOTIATION_ACCEPTED_TOTAL, METRIC_PROTOCOL_NEGOTIATION_REJECTED_TOTAL,
};
use crate::grid::galaxy_replay_metrics::{
    replay_evaluations_total, replay_pending, replay_pending_resolved_total,
    replay_pending_scheduled_total, replay_verification_enqueue_total,
    verification_replay_record_total, METRIC_REPLAY_EVALUATIONS_TOTAL, METRIC_REPLAY_PENDING,
    METRIC_REPLAY_PENDING_RESOLVED_TOTAL, METRIC_REPLAY_PENDING_SCHEDULED_TOTAL,
    METRIC_REPLAY_VERIFICATION_ENQUEUE_TOTAL, METRIC_VERIFICATION_REPLAY_RECORD_TOTAL,
};
use crate::grid::galaxy_replication_metrics::{
    replication_enqueue_total, replication_executor_enqueue_total, replication_rate_limited_total,
    replication_strict_total, METRIC_REPLICATION_ENQUEUE_TOTAL,
    METRIC_REPLICATION_EXECUTOR_ENQUEUE_TOTAL, METRIC_REPLICATION_RATE_LIMITED_TOTAL,
    METRIC_REPLICATION_STRICT_TOTAL,
};
use crate::grid::galaxy_settlement_metrics::{
    settlement_cleared_total, settlement_not_applicable_total, settlement_payout_batch_total,
    settlement_pending_verification_total, settlement_resolved_total,
    METRIC_SETTLEMENT_CLEARED_TOTAL, METRIC_SETTLEMENT_NOT_APPLICABLE_TOTAL,
    METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL, METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL,
    METRIC_SETTLEMENT_RESOLVED_TOTAL,
};
use crate::grid::galaxy_trust_score::{
    configured_default_trust_score, configured_min_trust_for_payout, default_score_applied_total,
    explicit_score_total, gate_evaluations_total, last_trust_score, payout_eligible_total,
    payout_held_total, payout_not_applicable_total, trust_score_delta_total,
    METRIC_DEFAULT_SCORE_APPLIED_TOTAL, METRIC_EXPLICIT_SCORE_TOTAL, METRIC_GATE_EVALUATIONS_TOTAL,
    METRIC_PAYOUT_ELIGIBLE_TOTAL, METRIC_PAYOUT_HELD_TOTAL, METRIC_PAYOUT_NOT_APPLICABLE_TOTAL,
    METRIC_TRUST_GATE_DEFAULT_SCORE, METRIC_TRUST_GATE_MIN_THRESHOLD, METRIC_TRUST_SCORE,
    METRIC_TRUST_SCORE_DELTA_TOTAL,
};
use crate::grid::galaxy_verification_metrics::{
    verification_checker_enqueue_total, verification_checker_pending_total,
    verification_match_total, verification_mismatch_total, verification_sample_completed_total,
    verification_sample_total, METRIC_VERIFICATION_CHECKER_ENQUEUE_TOTAL,
    METRIC_VERIFICATION_CHECKER_PENDING_TOTAL, METRIC_VERIFICATION_MATCH_TOTAL,
    METRIC_VERIFICATION_MISMATCH_TOTAL, METRIC_VERIFICATION_SAMPLE_COMPLETED_TOTAL,
    METRIC_VERIFICATION_SAMPLE_TOTAL,
};
use crate::grid::galaxy_verify_sampling::{
    verify_elevated_applied_total, verify_sample_not_applicable_total,
    verify_sample_scheduled_total, verify_sample_skipped_total, verify_sampling_evaluations_total,
    METRIC_VERIFY_ELEVATED_APPLIED_TOTAL, METRIC_VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL,
    METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL, METRIC_VERIFY_SAMPLE_SKIPPED_TOTAL,
    METRIC_VERIFY_SAMPLING_EVALUATIONS_TOTAL,
};
use crate::grid::galaxy_worker_health::{
    galaxy_worker_unhealthy_total, METRIC_WORKER_UNHEALTHY_TOTAL,
};

/// Lazily initialized Prometheus registry and metric handles.
pub struct PoolAiPrometheus {
    registry: Registry,
    http_requests_total: IntCounterVec,
    http_request_duration_seconds: HistogramVec,
    secret_rotations_total: IntCounterVec,
    workers_active: IntGauge,
    system_total_requests: IntGauge,
    uptime_seconds: IntGauge,
    build_info: IntGauge,
    #[cfg(feature = "enterprise")]
    monitoring_alert_rules: IntGauge,
    #[cfg(feature = "enterprise")]
    monitoring_dashboards: IntGauge,
    galaxy_pricing_fresh_served: IntGauge,
    galaxy_pricing_stale_served: IntGauge,
    galaxy_pricing_forced_fallback_total: IntGauge,
    galaxy_pricing_cache_age_seconds: IntGauge,
    galaxy_pricing_quote_usd_micro: IntGauge,
    galaxy_pricing_market_min_usd_micro: IntGauge,
    galaxy_pricing_provider_catalog_lookups_total: IntGauge,
    galaxy_pricing_provider_catalog_hits_total: IntGauge,
    galaxy_pricing_provider_errors_total: IntGauge,
    galaxy_trust_payout_eligible_total: IntGauge,
    galaxy_trust_payout_held_total: IntGauge,
    galaxy_trust_payout_not_applicable_total: IntGauge,
    galaxy_trust_score: IntGauge,
    galaxy_trust_gate_min_threshold: IntGauge,
    galaxy_trust_gate_default_score: IntGauge,
    galaxy_trust_gate_evaluations_total: IntGauge,
    galaxy_trust_default_score_applied_total: IntGauge,
    galaxy_trust_explicit_score_total: IntGauge,
    galaxy_trust_score_delta_total: IntGauge,
    galaxy_shard_local_hit_ratio: IntGauge,
    galaxy_cross_region_egress_mb: IntGauge,
    galaxy_prefetch_plan_total: IntGauge,
    galaxy_prefetch_planned_shards_total: IntGauge,
    galaxy_prefetch_hot_skip_total: IntGauge,
    galaxy_prefetch_bytes_total: IntGauge,
    galaxy_prefetch_enqueue_total: IntGauge,
    galaxy_prefetch_wait_ms_total: IntGauge,
    galaxy_prefetch_strict_mode_total: IntGauge,
    galaxy_prefetch_complete_total: IntGauge,
    galaxy_prefetch_ingest_total: IntGauge,
    galaxy_prefetch_skip_ingest_total: IntGauge,
    galaxy_prefetch_seed_pull_total: IntGauge,
    galaxy_prefetch_lease_acquired_total: IntGauge,
    galaxy_prefetch_seed_fetch_total: IntGauge,
    galaxy_prefetch_seed_fetch_miss_total: IntGauge,
    galaxy_prefetch_co_access_total: IntGauge,
    galaxy_locality_unsatisfied_total: IntGauge,
    galaxy_prefetch_re_migrate_total: IntGauge,
    galaxy_hot_promote_total: IntGauge,
    galaxy_hot_evict_total: IntGauge,
    galaxy_shard_access_total: IntGauge,
    galaxy_prefetch_queue_depth: IntGauge,
    galaxy_prefetch_backpressure_total: IntGauge,
    galaxy_prefetch_raid_fetch_total: IntGauge,
    galaxy_prefetch_raid_fetch_miss_total: IntGauge,
    galaxy_prefetch_egress_blocked_total: IntGauge,
    galaxy_prefetch_peer_fetch_total: IntGauge,
    galaxy_prefetch_peer_fetch_miss_total: IntGauge,
    galaxy_prefetch_pull_bytes_total: IntGauge,
    poolai_protocol_negotiation_rejected_total: IntGauge,
    poolai_protocol_negotiation_accepted_total: IntGauge,
    galaxy_locality_rank_ingest_total: IntGauge,
    galaxy_locality_rank_miss_total: IntGauge,
    galaxy_locality_rank_empty_workers_total: IntGauge,
    galaxy_locality_rank_skip_total: IntGauge,
    galaxy_verification_mismatch_total: IntGauge,
    galaxy_verification_match_total: IntGauge,
    galaxy_verification_sample_total: IntGauge,
    galaxy_verification_sample_scheduled_total: IntGauge,
    galaxy_verification_sample_completed_total: IntGauge,
    galaxy_verification_sample_skipped_total: IntGauge,
    galaxy_verification_sample_not_applicable_total: IntGauge,
    galaxy_verification_sampling_evaluations_total: IntGauge,
    galaxy_verification_elevated_applied_total: IntGauge,
    galaxy_verification_checker_enqueue_total: IntGauge,
    galaxy_verification_checker_pending_total: IntGauge,
    galaxy_replay_pending: IntGauge,
    galaxy_replay_pending_scheduled_total: IntGauge,
    galaxy_replay_pending_resolved_total: IntGauge,
    galaxy_replay_evaluations_total: IntGauge,
    galaxy_replay_verification_enqueue_total: IntGauge,
    galaxy_verification_replay_record_total: IntGauge,
    galaxy_settlement_pending_verification_total: IntGauge,
    galaxy_settlement_cleared_total: IntGauge,
    galaxy_settlement_not_applicable_total: IntGauge,
    galaxy_settlement_resolved_total: IntGauge,
    galaxy_settlement_payout_batch_total: IntGauge,
    galaxy_worker_unhealthy_total: IntGauge,
    poolai_release_verify_total: IntGauge,
    poolai_release_verify_fail_total: IntGauge,
    poolai_update_notify_pending: IntGauge,
    galaxy_fee_split_applied_total: IntGauge,
    galaxy_replication_strict_total: IntGauge,
    galaxy_replication_enqueue_total: IntGauge,
    galaxy_replication_executor_enqueue_total: IntGauge,
    galaxy_replication_rate_limited_total: IntGauge,
}

static PROMETHEUS: OnceLock<PoolAiPrometheus> = OnceLock::new();

/// Register metrics once; safe to call repeatedly.
pub fn init_prometheus() -> &'static PoolAiPrometheus {
    PROMETHEUS.get_or_init(build_prometheus)
}

fn build_prometheus() -> PoolAiPrometheus {
    let registry = Registry::new();

    let http_requests_total = IntCounterVec::new(
        Opts::new("poolai_http_requests_total", "Total HTTP requests served"),
        &["method", "status"],
    )
    .expect("poolai_http_requests_total");
    registry
        .register(Box::new(http_requests_total.clone()))
        .expect("register poolai_http_requests_total");

    let http_request_duration_seconds = HistogramVec::new(
        HistogramOpts::new(
            "poolai_http_request_duration_seconds",
            "HTTP request wall time in seconds",
        )
        .buckets(vec![
            0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ]),
        &["method"],
    )
    .expect("poolai_http_request_duration_seconds");
    registry
        .register(Box::new(http_request_duration_seconds.clone()))
        .expect("register poolai_http_request_duration_seconds");

    let secret_rotations_total = IntCounterVec::new(
        Opts::new(
            "poolai_secret_rotations_total",
            "Secret rotation hook runs (PH-S24 / PH-S29)",
        ),
        &["kind", "success"],
    )
    .expect("poolai_secret_rotations_total");
    registry
        .register(Box::new(secret_rotations_total.clone()))
        .expect("register poolai_secret_rotations_total");

    let workers_active = IntGauge::with_opts(Opts::new(
        "poolai_workers_active",
        "Active workers from application system state",
    ))
    .expect("poolai_workers_active");
    registry
        .register(Box::new(workers_active.clone()))
        .expect("register poolai_workers_active");

    let system_total_requests = IntGauge::with_opts(Opts::new(
        "poolai_system_total_requests",
        "Cumulative request counter from system metrics snapshot",
    ))
    .expect("poolai_system_total_requests");
    registry
        .register(Box::new(system_total_requests.clone()))
        .expect("register poolai_system_total_requests");

    let uptime_seconds = IntGauge::with_opts(Opts::new(
        "poolai_uptime_seconds",
        "Process uptime since coordinator start",
    ))
    .expect("poolai_uptime_seconds");
    registry
        .register(Box::new(uptime_seconds.clone()))
        .expect("register poolai_uptime_seconds");

    let build_info = IntGauge::with_opts(
        Opts::new(
            "poolai_build_info",
            "PoolAI build metadata (value is always 1)",
        )
        .const_label("version", env!("CARGO_PKG_VERSION")),
    )
    .expect("poolai_build_info");
    build_info.set(1);
    registry
        .register(Box::new(build_info.clone()))
        .expect("register poolai_build_info");

    #[cfg(feature = "enterprise")]
    let monitoring_alert_rules = {
        let g = IntGauge::with_opts(Opts::new(
            "poolai_monitoring_alert_rules",
            "Enterprise monitoring alert rules (enabled + disabled)",
        ))
        .expect("poolai_monitoring_alert_rules");
        registry
            .register(Box::new(g.clone()))
            .expect("register poolai_monitoring_alert_rules");
        g
    };

    #[cfg(feature = "enterprise")]
    let monitoring_dashboards = {
        let g = IntGauge::with_opts(Opts::new(
            "poolai_monitoring_dashboards",
            "Enterprise monitoring dashboards persisted",
        ))
        .expect("poolai_monitoring_dashboards");
        registry
            .register(Box::new(g.clone()))
            .expect("register poolai_monitoring_dashboards");
        g
    };

    let galaxy_pricing_fresh_served = IntGauge::with_opts(Opts::new(
        METRIC_FRESH_SERVED_TOTAL,
        "Galaxy pricing oracle L1 fresh cache serves (PH-S127)",
    ))
    .expect(METRIC_FRESH_SERVED_TOTAL);
    registry
        .register(Box::new(galaxy_pricing_fresh_served.clone()))
        .expect("register galaxy_pricing_fresh_served");

    let galaxy_pricing_stale_served = IntGauge::with_opts(Opts::new(
        METRIC_STALE_SERVED_TOTAL,
        "Galaxy pricing oracle L1 stale cache serves (PH-S127)",
    ))
    .expect(METRIC_STALE_SERVED_TOTAL);
    registry
        .register(Box::new(galaxy_pricing_stale_served.clone()))
        .expect("register galaxy_pricing_stale_served");

    let galaxy_pricing_forced_fallback_total = IntGauge::with_opts(Opts::new(
        METRIC_FORCED_FALLBACK_TOTAL,
        "Galaxy pricing oracle forced L2 fallback quotes (PH-S127)",
    ))
    .expect(METRIC_FORCED_FALLBACK_TOTAL);
    registry
        .register(Box::new(galaxy_pricing_forced_fallback_total.clone()))
        .expect("register galaxy_pricing_forced_fallback_total");

    let galaxy_pricing_cache_age_seconds = IntGauge::with_opts(Opts::new(
        METRIC_CACHE_AGE_SECONDS,
        "Galaxy pricing L1 cache age seconds last observed (PH-S168)",
    ))
    .expect(METRIC_CACHE_AGE_SECONDS);
    registry
        .register(Box::new(galaxy_pricing_cache_age_seconds.clone()))
        .expect("register galaxy_pricing_cache_age_seconds");

    let galaxy_pricing_quote_usd_micro = IntGauge::with_opts(Opts::new(
        METRIC_QUOTE_USD_MICRO,
        "Galaxy pricing last served PoolAI quote micro-USD (PH-S174)",
    ))
    .expect(METRIC_QUOTE_USD_MICRO);
    registry
        .register(Box::new(galaxy_pricing_quote_usd_micro.clone()))
        .expect("register galaxy_pricing_quote_usd_micro");

    let galaxy_pricing_market_min_usd_micro = IntGauge::with_opts(Opts::new(
        METRIC_MARKET_MIN_USD_MICRO,
        "Galaxy pricing last observed market min micro-USD (PH-S181)",
    ))
    .expect(METRIC_MARKET_MIN_USD_MICRO);
    registry
        .register(Box::new(galaxy_pricing_market_min_usd_micro.clone()))
        .expect("register galaxy_pricing_market_min_usd_micro");

    let galaxy_pricing_provider_catalog_lookups_total = IntGauge::with_opts(Opts::new(
        METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL,
        "Galaxy pricing provider catalog allow-list lookups (PH-S172)",
    ))
    .expect(METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL);
    registry
        .register(Box::new(
            galaxy_pricing_provider_catalog_lookups_total.clone(),
        ))
        .expect("register galaxy_pricing_provider_catalog_lookups_total");

    let galaxy_pricing_provider_catalog_hits_total = IntGauge::with_opts(Opts::new(
        METRIC_PROVIDER_CATALOG_HITS_TOTAL,
        "Galaxy pricing provider catalog allow-list hits (PH-S172)",
    ))
    .expect(METRIC_PROVIDER_CATALOG_HITS_TOTAL);
    registry
        .register(Box::new(galaxy_pricing_provider_catalog_hits_total.clone()))
        .expect("register galaxy_pricing_provider_catalog_hits_total");

    let galaxy_pricing_provider_errors_total = IntGauge::with_opts(Opts::new(
        METRIC_PROVIDER_ERRORS_TOTAL,
        "Galaxy pricing live provider HTTP fetch failures (PH-S173)",
    ))
    .expect(METRIC_PROVIDER_ERRORS_TOTAL);
    registry
        .register(Box::new(galaxy_pricing_provider_errors_total.clone()))
        .expect("register galaxy_pricing_provider_errors_total");

    let galaxy_trust_payout_eligible_total = IntGauge::with_opts(Opts::new(
        METRIC_PAYOUT_ELIGIBLE_TOTAL,
        "Galaxy trust gate edge results eligible for payout stub (PH-S137)",
    ))
    .expect(METRIC_PAYOUT_ELIGIBLE_TOTAL);
    registry
        .register(Box::new(galaxy_trust_payout_eligible_total.clone()))
        .expect("register galaxy_trust_payout_eligible_total");

    let galaxy_trust_payout_held_total = IntGauge::with_opts(Opts::new(
        METRIC_PAYOUT_HELD_TOTAL,
        "Galaxy trust gate edge results held pending verification (PH-S137)",
    ))
    .expect(METRIC_PAYOUT_HELD_TOTAL);
    registry
        .register(Box::new(galaxy_trust_payout_held_total.clone()))
        .expect("register galaxy_trust_payout_held_total");

    let galaxy_trust_payout_not_applicable_total = IntGauge::with_opts(Opts::new(
        METRIC_PAYOUT_NOT_APPLICABLE_TOTAL,
        "Galaxy trust gate local-origin results not applicable (PH-S364)",
    ))
    .expect(METRIC_PAYOUT_NOT_APPLICABLE_TOTAL);
    registry
        .register(Box::new(galaxy_trust_payout_not_applicable_total.clone()))
        .expect("register galaxy_trust_payout_not_applicable_total");

    let galaxy_trust_score = IntGauge::with_opts(Opts::new(
        METRIC_TRUST_SCORE,
        "Galaxy last observed grid result trust score 0..=100 (PH-S182)",
    ))
    .expect(METRIC_TRUST_SCORE);
    registry
        .register(Box::new(galaxy_trust_score.clone()))
        .expect("register galaxy_trust_score");

    let galaxy_trust_gate_min_threshold = IntGauge::with_opts(Opts::new(
        METRIC_TRUST_GATE_MIN_THRESHOLD,
        "Galaxy configured minimum trust 0..=100 for edge auto payout (PH-S374)",
    ))
    .expect(METRIC_TRUST_GATE_MIN_THRESHOLD);
    registry
        .register(Box::new(galaxy_trust_gate_min_threshold.clone()))
        .expect("register galaxy_trust_gate_min_threshold");

    let galaxy_trust_gate_default_score = IntGauge::with_opts(Opts::new(
        METRIC_TRUST_GATE_DEFAULT_SCORE,
        "Galaxy default trust score 0..=100 when grid result omits trust_score (PH-S384)",
    ))
    .expect(METRIC_TRUST_GATE_DEFAULT_SCORE);
    registry
        .register(Box::new(galaxy_trust_gate_default_score.clone()))
        .expect("register galaxy_trust_gate_default_score");

    let galaxy_trust_gate_evaluations_total = IntGauge::with_opts(Opts::new(
        METRIC_GATE_EVALUATIONS_TOTAL,
        "Galaxy trust gate evaluations on grid result path (PH-S394)",
    ))
    .expect(METRIC_GATE_EVALUATIONS_TOTAL);
    registry
        .register(Box::new(galaxy_trust_gate_evaluations_total.clone()))
        .expect("register galaxy_trust_gate_evaluations_total");

    let galaxy_trust_default_score_applied_total = IntGauge::with_opts(Opts::new(
        METRIC_DEFAULT_SCORE_APPLIED_TOTAL,
        "Galaxy grid results where default trust score was applied (PH-S395)",
    ))
    .expect(METRIC_DEFAULT_SCORE_APPLIED_TOTAL);
    registry
        .register(Box::new(galaxy_trust_default_score_applied_total.clone()))
        .expect("register galaxy_trust_default_score_applied_total");

    let galaxy_trust_explicit_score_total = IntGauge::with_opts(Opts::new(
        METRIC_EXPLICIT_SCORE_TOTAL,
        "Galaxy grid results with explicit trust_score on ingest (PH-S405)",
    ))
    .expect(METRIC_EXPLICIT_SCORE_TOTAL);
    registry
        .register(Box::new(galaxy_trust_explicit_score_total.clone()))
        .expect("register galaxy_trust_explicit_score_total");

    let galaxy_trust_score_delta_total = IntGauge::with_opts(Opts::new(
        METRIC_TRUST_SCORE_DELTA_TOTAL,
        "Galaxy trust score delta applications on verification verdict (PH-S456)",
    ))
    .expect(METRIC_TRUST_SCORE_DELTA_TOTAL);
    registry
        .register(Box::new(galaxy_trust_score_delta_total.clone()))
        .expect("register galaxy_trust_score_delta_total");

    let galaxy_shard_local_hit_ratio = IntGauge::with_opts(Opts::new(
        METRIC_SHARD_LOCAL_HIT_RATIO,
        "Galaxy last observed top-ranked shard local hit ratio basis points 0-10000 (PH-S183)",
    ))
    .expect(METRIC_SHARD_LOCAL_HIT_RATIO);
    registry
        .register(Box::new(galaxy_shard_local_hit_ratio.clone()))
        .expect("register galaxy_shard_local_hit_ratio");

    let galaxy_cross_region_egress_mb = IntGauge::with_opts(Opts::new(
        METRIC_CROSS_REGION_EGRESS_MB,
        "Galaxy last observed cross-region egress whole MB on rank/prefetch path (PH-S185)",
    ))
    .expect(METRIC_CROSS_REGION_EGRESS_MB);
    registry
        .register(Box::new(galaxy_cross_region_egress_mb.clone()))
        .expect("register galaxy_cross_region_egress_mb");

    let galaxy_prefetch_plan_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_PLAN_TOTAL,
        "Galaxy prefetch plans computed (PH-S167)",
    ))
    .expect(METRIC_PREFETCH_PLAN_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_plan_total.clone()))
        .expect("register galaxy_prefetch_plan_total");

    let galaxy_prefetch_planned_shards_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_PLANNED_SHARDS_TOTAL,
        "Galaxy prefetch shards scheduled in plans (PH-S167)",
    ))
    .expect(METRIC_PREFETCH_PLANNED_SHARDS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_planned_shards_total.clone()))
        .expect("register galaxy_prefetch_planned_shards_total");

    let galaxy_prefetch_hot_skip_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_HOT_SKIP_TOTAL,
        "Galaxy prefetch shards skipped as already hot (PH-S167)",
    ))
    .expect(METRIC_PREFETCH_HOT_SKIP_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_hot_skip_total.clone()))
        .expect("register galaxy_prefetch_hot_skip_total");

    let galaxy_prefetch_bytes_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_BYTES_TOTAL,
        "Galaxy estimated prefetch bytes scheduled in plans (PH-S184 stub)",
    ))
    .expect(METRIC_PREFETCH_BYTES_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_bytes_total.clone()))
        .expect("register galaxy_prefetch_bytes_total");

    let galaxy_prefetch_enqueue_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_ENQUEUE_TOTAL,
        "Galaxy prefetch enqueue stub shard items (PH-S283)",
    ))
    .expect(METRIC_PREFETCH_ENQUEUE_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_enqueue_total.clone()))
        .expect("register galaxy_prefetch_enqueue_total");

    let galaxy_prefetch_wait_ms_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_WAIT_MS_TOTAL,
        "Galaxy prefetch wait stub milliseconds (PH-S293)",
    ))
    .expect(METRIC_PREFETCH_WAIT_MS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_wait_ms_total.clone()))
        .expect("register galaxy_prefetch_wait_ms_total");

    let galaxy_prefetch_strict_mode_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_STRICT_MODE_TOTAL,
        "Galaxy prefetch strict locality mode plans (PH-S303)",
    ))
    .expect(METRIC_PREFETCH_STRICT_MODE_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_strict_mode_total.clone()))
        .expect("register galaxy_prefetch_strict_mode_total");

    let galaxy_prefetch_complete_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_COMPLETE_TOTAL,
        "Galaxy prefetch complete hook invocations (PH-S307)",
    ))
    .expect(METRIC_PREFETCH_COMPLETE_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_complete_total.clone()))
        .expect("register galaxy_prefetch_complete_total");

    let galaxy_prefetch_ingest_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_INGEST_TOTAL,
        "Galaxy prefetch ingest stub invocations (PH-S313)",
    ))
    .expect(METRIC_PREFETCH_INGEST_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_ingest_total.clone()))
        .expect("register galaxy_prefetch_ingest_total");

    let galaxy_prefetch_skip_ingest_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_SKIP_INGEST_TOTAL,
        "Galaxy prefetch ingest skips when job has no required shards (PH-S323)",
    ))
    .expect(METRIC_PREFETCH_SKIP_INGEST_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_skip_ingest_total.clone()))
        .expect("register galaxy_prefetch_skip_ingest_total");

    let galaxy_prefetch_seed_pull_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_SEED_PULL_TOTAL,
        "Galaxy prefetch seed pull stub invocations (PH-S424)",
    ))
    .expect(METRIC_PREFETCH_SEED_PULL_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_seed_pull_total.clone()))
        .expect("register galaxy_prefetch_seed_pull_total");

    let galaxy_prefetch_lease_acquired_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL,
        "Galaxy prefetch plans triggered by lease acquire (PH-S425)",
    ))
    .expect(METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_lease_acquired_total.clone()))
        .expect("register galaxy_prefetch_lease_acquired_total");

    let galaxy_prefetch_seed_fetch_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_SEED_FETCH_TOTAL,
        "Galaxy prefetch memory-layer seed fetch hits (PH-S444)",
    ))
    .expect(METRIC_PREFETCH_SEED_FETCH_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_seed_fetch_total.clone()))
        .expect("register galaxy_prefetch_seed_fetch_total");

    let galaxy_prefetch_seed_fetch_miss_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_SEED_FETCH_MISS_TOTAL,
        "Galaxy prefetch memory-layer seed fetch misses (PH-S444)",
    ))
    .expect(METRIC_PREFETCH_SEED_FETCH_MISS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_seed_fetch_miss_total.clone()))
        .expect("register galaxy_prefetch_seed_fetch_miss_total");

    let galaxy_prefetch_co_access_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_CO_ACCESS_TOTAL,
        "Galaxy co-access graph speculative prefetch plans (PH-S446)",
    ))
    .expect(METRIC_PREFETCH_CO_ACCESS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_co_access_total.clone()))
        .expect("register galaxy_prefetch_co_access_total");

    let galaxy_locality_unsatisfied_total = IntGauge::with_opts(Opts::new(
        METRIC_LOCALITY_UNSATISFIED_TOTAL,
        "Galaxy strict locality ingest rejections (PH-S445)",
    ))
    .expect(METRIC_LOCALITY_UNSATISFIED_TOTAL);
    registry
        .register(Box::new(galaxy_locality_unsatisfied_total.clone()))
        .expect("register galaxy_locality_unsatisfied_total");

    let galaxy_prefetch_re_migrate_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_RE_MIGRATE_TOTAL,
        "Galaxy re-migrate prefetch plans (PH-S454)",
    ))
    .expect(METRIC_PREFETCH_RE_MIGRATE_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_re_migrate_total.clone()))
        .expect("register galaxy_prefetch_re_migrate_total");

    let galaxy_hot_promote_total = IntGauge::with_opts(Opts::new(
        METRIC_HOT_PROMOTE_TOTAL,
        "Galaxy hot tier shard promotions (PH-S458)",
    ))
    .expect(METRIC_HOT_PROMOTE_TOTAL);
    registry
        .register(Box::new(galaxy_hot_promote_total.clone()))
        .expect("register galaxy_hot_promote_total");

    let galaxy_hot_evict_total = IntGauge::with_opts(Opts::new(
        METRIC_HOT_EVICT_TOTAL,
        "Galaxy hot tier shard evictions (PH-S458)",
    ))
    .expect(METRIC_HOT_EVICT_TOTAL);
    registry
        .register(Box::new(galaxy_hot_evict_total.clone()))
        .expect("register galaxy_hot_evict_total");

    let galaxy_shard_access_total = IntGauge::with_opts(Opts::new(
        METRIC_SHARD_ACCESS_TOTAL,
        "Galaxy shard access events on prefetch path (PH-S459)",
    ))
    .expect(METRIC_SHARD_ACCESS_TOTAL);
    registry
        .register(Box::new(galaxy_shard_access_total.clone()))
        .expect("register galaxy_shard_access_total");

    let galaxy_prefetch_queue_depth = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_QUEUE_DEPTH,
        "Galaxy prefetch queue depth gauge stub (PH-S459)",
    ))
    .expect(METRIC_PREFETCH_QUEUE_DEPTH);
    registry
        .register(Box::new(galaxy_prefetch_queue_depth.clone()))
        .expect("register galaxy_prefetch_queue_depth");

    let galaxy_prefetch_backpressure_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_BACKPRESSURE_TOTAL,
        "Galaxy prefetch enqueue skipped by bandwidth backpressure (PH-S464)",
    ))
    .expect(METRIC_PREFETCH_BACKPRESSURE_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_backpressure_total.clone()))
        .expect("register galaxy_prefetch_backpressure_total");

    let galaxy_prefetch_raid_fetch_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_RAID_FETCH_TOTAL,
        "Galaxy RAID artifact prefetch fetch hits (PH-S465)",
    ))
    .expect(METRIC_PREFETCH_RAID_FETCH_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_raid_fetch_total.clone()))
        .expect("register galaxy_prefetch_raid_fetch_total");

    let galaxy_prefetch_raid_fetch_miss_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_RAID_FETCH_MISS_TOTAL,
        "Galaxy RAID artifact prefetch fetch misses (PH-S465)",
    ))
    .expect(METRIC_PREFETCH_RAID_FETCH_MISS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_raid_fetch_miss_total.clone()))
        .expect("register galaxy_prefetch_raid_fetch_miss_total");

    let galaxy_prefetch_egress_blocked_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_EGRESS_BLOCKED_TOTAL,
        "Galaxy prefetch blocked by lan_only cross-region egress guardrail (PH-S474)",
    ))
    .expect(METRIC_PREFETCH_EGRESS_BLOCKED_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_egress_blocked_total.clone()))
        .expect("register galaxy_prefetch_egress_blocked_total");

    let galaxy_prefetch_peer_fetch_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_PEER_FETCH_TOTAL,
        "Galaxy peer seed inventory prefetch fetch hits (PH-S479)",
    ))
    .expect(METRIC_PREFETCH_PEER_FETCH_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_peer_fetch_total.clone()))
        .expect("register galaxy_prefetch_peer_fetch_total");

    let galaxy_prefetch_peer_fetch_miss_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_PEER_FETCH_MISS_TOTAL,
        "Galaxy peer seed inventory prefetch fetch misses (PH-S479)",
    ))
    .expect(METRIC_PREFETCH_PEER_FETCH_MISS_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_peer_fetch_miss_total.clone()))
        .expect("register galaxy_prefetch_peer_fetch_miss_total");

    let galaxy_prefetch_pull_bytes_total = IntGauge::with_opts(Opts::new(
        METRIC_PREFETCH_PULL_BYTES_TOTAL,
        "Galaxy live prefetch bytes pulled from memory (PH-S484)",
    ))
    .expect(METRIC_PREFETCH_PULL_BYTES_TOTAL);
    registry
        .register(Box::new(galaxy_prefetch_pull_bytes_total.clone()))
        .expect("register galaxy_prefetch_pull_bytes_total");

    let poolai_protocol_negotiation_rejected_total = IntGauge::with_opts(Opts::new(
        METRIC_PROTOCOL_NEGOTIATION_REJECTED_TOTAL,
        "Unsupported protocol negotiation rejections (PH-S449)",
    ))
    .expect(METRIC_PROTOCOL_NEGOTIATION_REJECTED_TOTAL);
    registry
        .register(Box::new(poolai_protocol_negotiation_rejected_total.clone()))
        .expect("register poolai_protocol_negotiation_rejected_total");

    let poolai_protocol_negotiation_accepted_total = IntGauge::with_opts(Opts::new(
        METRIC_PROTOCOL_NEGOTIATION_ACCEPTED_TOTAL,
        "Successful protocol negotiations on register-remote (PH-S468)",
    ))
    .expect(METRIC_PROTOCOL_NEGOTIATION_ACCEPTED_TOTAL);
    registry
        .register(Box::new(poolai_protocol_negotiation_accepted_total.clone()))
        .expect("register poolai_protocol_negotiation_accepted_total");

    let galaxy_locality_rank_ingest_total = IntGauge::with_opts(Opts::new(
        METRIC_LOCALITY_RANK_INGEST_TOTAL,
        "Galaxy locality rank invocations on grid job ingest (PH-S295)",
    ))
    .expect(METRIC_LOCALITY_RANK_INGEST_TOTAL);
    registry
        .register(Box::new(galaxy_locality_rank_ingest_total.clone()))
        .expect("register galaxy_locality_rank_ingest_total");

    let galaxy_locality_rank_miss_total = IntGauge::with_opts(Opts::new(
        METRIC_LOCALITY_RANK_MISS_TOTAL,
        "Galaxy locality rank misses on grid job ingest (PH-S305)",
    ))
    .expect(METRIC_LOCALITY_RANK_MISS_TOTAL);
    registry
        .register(Box::new(galaxy_locality_rank_miss_total.clone()))
        .expect("register galaxy_locality_rank_miss_total");

    let galaxy_locality_rank_empty_workers_total = IntGauge::with_opts(Opts::new(
        METRIC_LOCALITY_RANK_EMPTY_WORKERS_TOTAL,
        "Galaxy locality rank empty worker inventory on ingest (PH-S315)",
    ))
    .expect(METRIC_LOCALITY_RANK_EMPTY_WORKERS_TOTAL);
    registry
        .register(Box::new(galaxy_locality_rank_empty_workers_total.clone()))
        .expect("register galaxy_locality_rank_empty_workers_total");

    let galaxy_locality_rank_skip_total = IntGauge::with_opts(Opts::new(
        METRIC_LOCALITY_RANK_SKIP_TOTAL,
        "Galaxy locality rank skips when job has no required shards (PH-S325)",
    ))
    .expect(METRIC_LOCALITY_RANK_SKIP_TOTAL);
    registry
        .register(Box::new(galaxy_locality_rank_skip_total.clone()))
        .expect("register galaxy_locality_rank_skip_total");

    let galaxy_verification_mismatch_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_MISMATCH_TOTAL,
        "Galaxy verification digest mismatches on grid result path (PH-S175)",
    ))
    .expect(METRIC_VERIFICATION_MISMATCH_TOTAL);
    registry
        .register(Box::new(galaxy_verification_mismatch_total.clone()))
        .expect("register galaxy_verification_mismatch_total");

    let galaxy_verification_match_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_MATCH_TOTAL,
        "Galaxy verification digest matches on grid result path (PH-S180)",
    ))
    .expect(METRIC_VERIFICATION_MATCH_TOTAL);
    registry
        .register(Box::new(galaxy_verification_match_total.clone()))
        .expect("register galaxy_verification_match_total");

    let galaxy_verification_sample_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_SAMPLE_TOTAL,
        "Galaxy verification samples scheduled on grid result path (PH-S177)",
    ))
    .expect(METRIC_VERIFICATION_SAMPLE_TOTAL);
    registry
        .register(Box::new(galaxy_verification_sample_total.clone()))
        .expect("register galaxy_verification_sample_total");

    let galaxy_verification_sample_scheduled_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL,
        "Galaxy verification stub samples scheduled on grid result path (PH-S164; PH-S186 /metrics)",
    ))
    .expect(METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL);
    registry
        .register(Box::new(galaxy_verification_sample_scheduled_total.clone()))
        .expect("register galaxy_verification_sample_scheduled_total");

    let galaxy_verification_sample_completed_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_SAMPLE_COMPLETED_TOTAL,
        "Galaxy verification samples completed with verdict on grid result path (PH-S343)",
    ))
    .expect(METRIC_VERIFICATION_SAMPLE_COMPLETED_TOTAL);
    registry
        .register(Box::new(galaxy_verification_sample_completed_total.clone()))
        .expect("register galaxy_verification_sample_completed_total");

    let galaxy_verification_sample_skipped_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFY_SAMPLE_SKIPPED_TOTAL,
        "Galaxy verification edge samples skipped by deterministic stub (PH-S345)",
    ))
    .expect(METRIC_VERIFY_SAMPLE_SKIPPED_TOTAL);
    registry
        .register(Box::new(galaxy_verification_sample_skipped_total.clone()))
        .expect("register galaxy_verification_sample_skipped_total");

    let galaxy_verification_sample_not_applicable_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL,
        "Galaxy verification samples not applicable on local origin path (PH-S356)",
    ))
    .expect(METRIC_VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL);
    registry
        .register(Box::new(
            galaxy_verification_sample_not_applicable_total.clone(),
        ))
        .expect("register galaxy_verification_sample_not_applicable_total");

    let galaxy_verification_sampling_evaluations_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFY_SAMPLING_EVALUATIONS_TOTAL,
        "Galaxy verification sampling evaluations on grid result path (PH-S414)",
    ))
    .expect(METRIC_VERIFY_SAMPLING_EVALUATIONS_TOTAL);
    registry
        .register(Box::new(
            galaxy_verification_sampling_evaluations_total.clone(),
        ))
        .expect("register galaxy_verification_sampling_evaluations_total");

    let galaxy_verification_elevated_applied_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFY_ELEVATED_APPLIED_TOTAL,
        "Galaxy elevated verification sample rate applied after mismatch (PH-S455)",
    ))
    .expect(METRIC_VERIFY_ELEVATED_APPLIED_TOTAL);
    registry
        .register(Box::new(galaxy_verification_elevated_applied_total.clone()))
        .expect("register galaxy_verification_elevated_applied_total");

    let galaxy_verification_checker_enqueue_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_CHECKER_ENQUEUE_TOTAL,
        "Galaxy verification checker enqueue stub on sample verdict (PH-S437)",
    ))
    .expect(METRIC_VERIFICATION_CHECKER_ENQUEUE_TOTAL);
    registry
        .register(Box::new(galaxy_verification_checker_enqueue_total.clone()))
        .expect("register galaxy_verification_checker_enqueue_total");

    let galaxy_verification_checker_pending_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_CHECKER_PENDING_TOTAL,
        "Galaxy verification checker pending stub tasks (PH-S496)",
    ))
    .expect(METRIC_VERIFICATION_CHECKER_PENDING_TOTAL);
    registry
        .register(Box::new(galaxy_verification_checker_pending_total.clone()))
        .expect("register galaxy_verification_checker_pending_total");

    let galaxy_replay_pending = IntGauge::with_opts(Opts::new(
        METRIC_REPLAY_PENDING,
        "Galaxy replay verifications pending coordinator verdict (PH-S176)",
    ))
    .expect(METRIC_REPLAY_PENDING);
    registry
        .register(Box::new(galaxy_replay_pending.clone()))
        .expect("register galaxy_replay_pending");

    let galaxy_replay_pending_scheduled_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLAY_PENDING_SCHEDULED_TOTAL,
        "Galaxy replay holds scheduled on grid result path (PH-S333)",
    ))
    .expect(METRIC_REPLAY_PENDING_SCHEDULED_TOTAL);
    registry
        .register(Box::new(galaxy_replay_pending_scheduled_total.clone()))
        .expect("register galaxy_replay_pending_scheduled_total");

    let galaxy_replay_pending_resolved_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLAY_PENDING_RESOLVED_TOTAL,
        "Galaxy replay holds cleared on verdict (PH-S335)",
    ))
    .expect(METRIC_REPLAY_PENDING_RESOLVED_TOTAL);
    registry
        .register(Box::new(galaxy_replay_pending_resolved_total.clone()))
        .expect("register galaxy_replay_pending_resolved_total");

    let galaxy_replay_evaluations_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLAY_EVALUATIONS_TOTAL,
        "Galaxy replay pending evaluations on grid result path (PH-S415)",
    ))
    .expect(METRIC_REPLAY_EVALUATIONS_TOTAL);
    registry
        .register(Box::new(galaxy_replay_evaluations_total.clone()))
        .expect("register galaxy_replay_evaluations_total");

    let galaxy_replay_verification_enqueue_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLAY_VERIFICATION_ENQUEUE_TOTAL,
        "Galaxy replay verification enqueue stub on mismatch (PH-S438)",
    ))
    .expect(METRIC_REPLAY_VERIFICATION_ENQUEUE_TOTAL);
    registry
        .register(Box::new(galaxy_replay_verification_enqueue_total.clone()))
        .expect("register galaxy_replay_verification_enqueue_total");

    let galaxy_verification_replay_record_total = IntGauge::with_opts(Opts::new(
        METRIC_VERIFICATION_REPLAY_RECORD_TOTAL,
        "Galaxy structured verification replay records emitted (PH-S447)",
    ))
    .expect(METRIC_VERIFICATION_REPLAY_RECORD_TOTAL);
    registry
        .register(Box::new(galaxy_verification_replay_record_total.clone()))
        .expect("register galaxy_verification_replay_record_total");

    let galaxy_settlement_pending_verification_total = IntGauge::with_opts(Opts::new(
        METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL,
        "Galaxy settlement holds pending verification on grid result path (PH-S178)",
    ))
    .expect(METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL);
    registry
        .register(Box::new(
            galaxy_settlement_pending_verification_total.clone(),
        ))
        .expect("register galaxy_settlement_pending_verification_total");

    let galaxy_settlement_cleared_total = IntGauge::with_opts(Opts::new(
        METRIC_SETTLEMENT_CLEARED_TOTAL,
        "Galaxy settlement cleared on grid result path (PH-S187)",
    ))
    .expect(METRIC_SETTLEMENT_CLEARED_TOTAL);
    registry
        .register(Box::new(galaxy_settlement_cleared_total.clone()))
        .expect("register galaxy_settlement_cleared_total");

    let galaxy_settlement_not_applicable_total = IntGauge::with_opts(Opts::new(
        METRIC_SETTLEMENT_NOT_APPLICABLE_TOTAL,
        "Galaxy settlement not applicable on grid result path (PH-S354)",
    ))
    .expect(METRIC_SETTLEMENT_NOT_APPLICABLE_TOTAL);
    registry
        .register(Box::new(galaxy_settlement_not_applicable_total.clone()))
        .expect("register galaxy_settlement_not_applicable_total");

    let galaxy_settlement_resolved_total = IntGauge::with_opts(Opts::new(
        METRIC_SETTLEMENT_RESOLVED_TOTAL,
        "Galaxy settlement status resolutions on grid result path (PH-S404)",
    ))
    .expect(METRIC_SETTLEMENT_RESOLVED_TOTAL);
    registry
        .register(Box::new(galaxy_settlement_resolved_total.clone()))
        .expect("register galaxy_settlement_resolved_total");

    let galaxy_settlement_payout_batch_total = IntGauge::with_opts(Opts::new(
        METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL,
        "Galaxy offline payout batch ledger entries on cleared settlement (PH-S427)",
    ))
    .expect(METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL);
    registry
        .register(Box::new(galaxy_settlement_payout_batch_total.clone()))
        .expect("register galaxy_settlement_payout_batch_total");

    let galaxy_worker_unhealthy_total = IntGauge::with_opts(Opts::new(
        METRIC_WORKER_UNHEALTHY_TOTAL,
        "Galaxy workers marked unhealthy after consecutive heartbeat misses (PH-S522)",
    ))
    .expect(METRIC_WORKER_UNHEALTHY_TOTAL);
    registry
        .register(Box::new(galaxy_worker_unhealthy_total.clone()))
        .expect("register galaxy_worker_unhealthy_total");

    let poolai_release_verify_total = IntGauge::with_opts(Opts::new(
        METRIC_RELEASE_VERIFY_TOTAL,
        "Successful poolai-verify-release runs (PH-S528, Galaxy §9.8)",
    ))
    .expect(METRIC_RELEASE_VERIFY_TOTAL);
    registry
        .register(Box::new(poolai_release_verify_total.clone()))
        .expect("register poolai_release_verify_total");

    let poolai_release_verify_fail_total = IntGauge::with_opts(Opts::new(
        METRIC_RELEASE_VERIFY_FAIL_TOTAL,
        "Failed poolai-verify-release runs (PH-S528, Galaxy §9.8)",
    ))
    .expect(METRIC_RELEASE_VERIFY_FAIL_TOTAL);
    registry
        .register(Box::new(poolai_release_verify_fail_total.clone()))
        .expect("register poolai_release_verify_fail_total");

    let poolai_update_notify_pending = IntGauge::with_opts(Opts::new(
        METRIC_UPDATE_NOTIFY_PENDING,
        "Pending opt-in update notifications (PH-S528 stub, Galaxy §9.8)",
    ))
    .expect(METRIC_UPDATE_NOTIFY_PENDING);
    registry
        .register(Box::new(poolai_update_notify_pending.clone()))
        .expect("register poolai_update_notify_pending");

    let galaxy_fee_split_applied_total = IntGauge::with_opts(Opts::new(
        METRIC_FEE_SPLIT_APPLIED_TOTAL,
        "Galaxy fee split applied on grid result path (PH-S194)",
    ))
    .expect(METRIC_FEE_SPLIT_APPLIED_TOTAL);
    registry
        .register(Box::new(galaxy_fee_split_applied_total.clone()))
        .expect("register galaxy_fee_split_applied_total");

    let galaxy_replication_strict_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLICATION_STRICT_TOTAL,
        "Galaxy replication strict tier grid job ingests (PH-S179)",
    ))
    .expect(METRIC_REPLICATION_STRICT_TOTAL);
    registry
        .register(Box::new(galaxy_replication_strict_total.clone()))
        .expect("register galaxy_replication_strict_total");

    let galaxy_replication_enqueue_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLICATION_ENQUEUE_TOTAL,
        "Galaxy replication executor enqueue stub on grid job ingest (PH-S426)",
    ))
    .expect(METRIC_REPLICATION_ENQUEUE_TOTAL);
    registry
        .register(Box::new(galaxy_replication_enqueue_total.clone()))
        .expect("register galaxy_replication_enqueue_total");

    let galaxy_replication_executor_enqueue_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLICATION_EXECUTOR_ENQUEUE_TOTAL,
        "Galaxy replication executor queue stub on grid job ingest (PH-S435)",
    ))
    .expect(METRIC_REPLICATION_EXECUTOR_ENQUEUE_TOTAL);
    registry
        .register(Box::new(galaxy_replication_executor_enqueue_total.clone()))
        .expect("register galaxy_replication_executor_enqueue_total");

    let galaxy_replication_rate_limited_total = IntGauge::with_opts(Opts::new(
        METRIC_REPLICATION_RATE_LIMITED_TOTAL,
        "Galaxy strict-tier replication rate-limited rejections (PH-S457)",
    ))
    .expect(METRIC_REPLICATION_RATE_LIMITED_TOTAL);
    registry
        .register(Box::new(galaxy_replication_rate_limited_total.clone()))
        .expect("register galaxy_replication_rate_limited_total");

    #[cfg(target_os = "linux")]
    {
        let collector = prometheus::process_collector::ProcessCollector::for_self();
        let _ = registry.register(Box::new(collector));
    }

    PoolAiPrometheus {
        registry,
        http_requests_total,
        http_request_duration_seconds,
        secret_rotations_total,
        workers_active,
        system_total_requests,
        uptime_seconds,
        build_info,
        #[cfg(feature = "enterprise")]
        monitoring_alert_rules,
        #[cfg(feature = "enterprise")]
        monitoring_dashboards,
        galaxy_pricing_fresh_served,
        galaxy_pricing_stale_served,
        galaxy_pricing_forced_fallback_total,
        galaxy_pricing_cache_age_seconds,
        galaxy_pricing_quote_usd_micro,
        galaxy_pricing_market_min_usd_micro,
        galaxy_pricing_provider_catalog_lookups_total,
        galaxy_pricing_provider_catalog_hits_total,
        galaxy_pricing_provider_errors_total,
        galaxy_trust_payout_eligible_total,
        galaxy_trust_payout_held_total,
        galaxy_trust_payout_not_applicable_total,
        galaxy_trust_score,
        galaxy_trust_gate_min_threshold,
        galaxy_trust_gate_default_score,
        galaxy_trust_gate_evaluations_total,
        galaxy_trust_default_score_applied_total,
        galaxy_trust_explicit_score_total,
        galaxy_trust_score_delta_total,
        galaxy_shard_local_hit_ratio,
        galaxy_cross_region_egress_mb,
        galaxy_prefetch_plan_total,
        galaxy_prefetch_planned_shards_total,
        galaxy_prefetch_hot_skip_total,
        galaxy_prefetch_bytes_total,
        galaxy_prefetch_enqueue_total,
        galaxy_prefetch_wait_ms_total,
        galaxy_prefetch_strict_mode_total,
        galaxy_prefetch_complete_total,
        galaxy_prefetch_ingest_total,
        galaxy_prefetch_skip_ingest_total,
        galaxy_prefetch_seed_pull_total,
        galaxy_prefetch_lease_acquired_total,
        galaxy_prefetch_seed_fetch_total,
        galaxy_prefetch_seed_fetch_miss_total,
        galaxy_prefetch_co_access_total,
        galaxy_locality_unsatisfied_total,
        galaxy_prefetch_re_migrate_total,
        galaxy_hot_promote_total,
        galaxy_hot_evict_total,
        galaxy_shard_access_total,
        galaxy_prefetch_queue_depth,
        galaxy_prefetch_backpressure_total,
        galaxy_prefetch_raid_fetch_total,
        galaxy_prefetch_raid_fetch_miss_total,
        galaxy_prefetch_egress_blocked_total,
        galaxy_prefetch_peer_fetch_total,
        galaxy_prefetch_peer_fetch_miss_total,
        galaxy_prefetch_pull_bytes_total,
        poolai_protocol_negotiation_rejected_total,
        poolai_protocol_negotiation_accepted_total,
        galaxy_locality_rank_ingest_total,
        galaxy_locality_rank_miss_total,
        galaxy_locality_rank_empty_workers_total,
        galaxy_locality_rank_skip_total,
        galaxy_verification_mismatch_total,
        galaxy_verification_match_total,
        galaxy_verification_sample_total,
        galaxy_verification_sample_scheduled_total,
        galaxy_verification_sample_completed_total,
        galaxy_verification_sample_skipped_total,
        galaxy_verification_sample_not_applicable_total,
        galaxy_verification_sampling_evaluations_total,
        galaxy_verification_elevated_applied_total,
        galaxy_verification_checker_enqueue_total,
        galaxy_verification_checker_pending_total,
        galaxy_replay_pending,
        galaxy_replay_pending_scheduled_total,
        galaxy_replay_pending_resolved_total,
        galaxy_replay_evaluations_total,
        galaxy_replay_verification_enqueue_total,
        galaxy_verification_replay_record_total,
        galaxy_settlement_pending_verification_total,
        galaxy_settlement_cleared_total,
        galaxy_settlement_not_applicable_total,
        galaxy_settlement_resolved_total,
        galaxy_settlement_payout_batch_total,
        galaxy_worker_unhealthy_total,
        poolai_release_verify_total,
        poolai_release_verify_fail_total,
        poolai_update_notify_pending,
        galaxy_fee_split_applied_total,
        galaxy_replication_strict_total,
        galaxy_replication_enqueue_total,
        galaxy_replication_executor_enqueue_total,
        galaxy_replication_rate_limited_total,
    }
}

/// Mirror in-process oracle counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_pricing_gauges() {
    let prom = init_prometheus();
    prom.galaxy_pricing_fresh_served
        .set(fresh_served_total() as i64);
    prom.galaxy_pricing_stale_served
        .set(stale_served_total() as i64);
    prom.galaxy_pricing_forced_fallback_total
        .set(forced_fallback_total() as i64);
    prom.galaxy_pricing_cache_age_seconds
        .set(pricing_cache_age_seconds() as i64);
    prom.galaxy_pricing_quote_usd_micro
        .set(last_quote_usd_micro() as i64);
    prom.galaxy_pricing_market_min_usd_micro
        .set(last_market_min_usd_micro() as i64);
    prom.galaxy_pricing_provider_catalog_lookups_total
        .set(provider_catalog_lookups_total() as i64);
    prom.galaxy_pricing_provider_catalog_hits_total
        .set(provider_catalog_hits_total() as i64);
    prom.galaxy_pricing_provider_errors_total
        .set(provider_errors_total() as i64);
}

/// Mirror in-process trust gate counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_trust_gauges() {
    let prom = init_prometheus();
    prom.galaxy_trust_payout_eligible_total
        .set(payout_eligible_total() as i64);
    prom.galaxy_trust_payout_held_total
        .set(payout_held_total() as i64);
    prom.galaxy_trust_payout_not_applicable_total
        .set(payout_not_applicable_total() as i64);
    prom.galaxy_trust_score.set(last_trust_score() as i64);
    prom.galaxy_trust_gate_min_threshold
        .set(configured_min_trust_for_payout() as i64);
    prom.galaxy_trust_gate_default_score
        .set(configured_default_trust_score() as i64);
    prom.galaxy_trust_gate_evaluations_total
        .set(gate_evaluations_total() as i64);
    prom.galaxy_trust_default_score_applied_total
        .set(default_score_applied_total() as i64);
    prom.galaxy_trust_explicit_score_total
        .set(explicit_score_total() as i64);
    prom.galaxy_trust_score_delta_total
        .set(trust_score_delta_total() as i64);
}

/// Mirror in-process locality rank counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_locality_gauges() {
    let prom = init_prometheus();
    prom.galaxy_shard_local_hit_ratio
        .set(last_shard_local_hit_ratio_bps() as i64);
    prom.galaxy_cross_region_egress_mb
        .set(last_cross_region_egress_mb() as i64);
    prom.galaxy_locality_rank_ingest_total
        .set(locality_rank_ingest_total() as i64);
    prom.galaxy_locality_rank_miss_total
        .set(locality_rank_miss_total() as i64);
    prom.galaxy_locality_rank_empty_workers_total
        .set(locality_rank_empty_workers_total() as i64);
    prom.galaxy_locality_rank_skip_total
        .set(locality_rank_skip_total() as i64);
}

/// Mirror in-process prefetch plan counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_prefetch_gauges() {
    let prom = init_prometheus();
    prom.galaxy_prefetch_plan_total
        .set(prefetch_plan_total() as i64);
    prom.galaxy_prefetch_planned_shards_total
        .set(prefetch_planned_shards_total() as i64);
    prom.galaxy_prefetch_hot_skip_total
        .set(prefetch_hot_skip_total() as i64);
    prom.galaxy_prefetch_bytes_total
        .set(prefetch_bytes_total() as i64);
    prom.galaxy_prefetch_enqueue_total
        .set(prefetch_enqueue_total() as i64);
    prom.galaxy_prefetch_wait_ms_total
        .set(prefetch_wait_ms_total() as i64);
    prom.galaxy_prefetch_strict_mode_total
        .set(prefetch_strict_mode_total() as i64);
    prom.galaxy_prefetch_complete_total
        .set(prefetch_complete_total() as i64);
    prom.galaxy_prefetch_ingest_total
        .set(prefetch_ingest_total() as i64);
    prom.galaxy_prefetch_skip_ingest_total
        .set(prefetch_skip_ingest_total() as i64);
    prom.galaxy_prefetch_seed_pull_total
        .set(prefetch_seed_pull_total() as i64);
    prom.galaxy_prefetch_lease_acquired_total
        .set(prefetch_lease_acquired_total() as i64);
    prom.galaxy_prefetch_seed_fetch_total
        .set(prefetch_seed_fetch_total() as i64);
    prom.galaxy_prefetch_seed_fetch_miss_total
        .set(prefetch_seed_fetch_miss_total() as i64);
    prom.galaxy_prefetch_co_access_total
        .set(prefetch_co_access_total() as i64);
    prom.galaxy_locality_unsatisfied_total
        .set(locality_unsatisfied_total() as i64);
    prom.galaxy_prefetch_re_migrate_total
        .set(prefetch_re_migrate_total() as i64);
    prom.galaxy_hot_promote_total
        .set(hot_promote_total() as i64);
    prom.galaxy_hot_evict_total.set(hot_evict_total() as i64);
    prom.galaxy_shard_access_total
        .set(shard_access_total() as i64);
    prom.galaxy_prefetch_queue_depth
        .set(prefetch_queue_depth() as i64);
    prom.galaxy_prefetch_backpressure_total
        .set(prefetch_backpressure_total() as i64);
    prom.galaxy_prefetch_raid_fetch_total
        .set(prefetch_raid_fetch_total() as i64);
    prom.galaxy_prefetch_raid_fetch_miss_total
        .set(prefetch_raid_fetch_miss_total() as i64);
    prom.galaxy_prefetch_egress_blocked_total
        .set(prefetch_egress_blocked_total() as i64);
    prom.galaxy_prefetch_peer_fetch_total
        .set(prefetch_peer_fetch_total() as i64);
    prom.galaxy_prefetch_peer_fetch_miss_total
        .set(prefetch_peer_fetch_miss_total() as i64);
    prom.galaxy_prefetch_pull_bytes_total
        .set(prefetch_pull_bytes_total() as i64);
    prom.poolai_protocol_negotiation_rejected_total
        .set(protocol_negotiation_rejected_total() as i64);
    prom.poolai_protocol_negotiation_accepted_total
        .set(protocol_negotiation_accepted_total() as i64);
}

/// Mirror in-process verification counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_verification_gauges() {
    let prom = init_prometheus();
    prom.galaxy_verification_mismatch_total
        .set(verification_mismatch_total() as i64);
    prom.galaxy_verification_match_total
        .set(verification_match_total() as i64);
    prom.galaxy_verification_sample_total
        .set(verification_sample_total() as i64);
    prom.galaxy_verification_sample_scheduled_total
        .set(verify_sample_scheduled_total() as i64);
    prom.galaxy_verification_sample_completed_total
        .set(verification_sample_completed_total() as i64);
    prom.galaxy_verification_sample_skipped_total
        .set(verify_sample_skipped_total() as i64);
    prom.galaxy_verification_sample_not_applicable_total
        .set(verify_sample_not_applicable_total() as i64);
    prom.galaxy_verification_sampling_evaluations_total
        .set(verify_sampling_evaluations_total() as i64);
    prom.galaxy_verification_elevated_applied_total
        .set(verify_elevated_applied_total() as i64);
    prom.galaxy_verification_checker_enqueue_total
        .set(verification_checker_enqueue_total() as i64);
    prom.galaxy_verification_checker_pending_total
        .set(verification_checker_pending_total() as i64);
    prom.galaxy_worker_unhealthy_total
        .set(galaxy_worker_unhealthy_total() as i64);
    prom.poolai_release_verify_total
        .set(release_verify_total() as i64);
    prom.poolai_release_verify_fail_total
        .set(release_verify_fail_total() as i64);
    prom.poolai_update_notify_pending
        .set(update_notify_pending() as i64);
    prom.galaxy_replay_pending.set(replay_pending() as i64);
    prom.galaxy_replay_pending_scheduled_total
        .set(replay_pending_scheduled_total() as i64);
    prom.galaxy_replay_pending_resolved_total
        .set(replay_pending_resolved_total() as i64);
    prom.galaxy_replay_evaluations_total
        .set(replay_evaluations_total() as i64);
    prom.galaxy_replay_verification_enqueue_total
        .set(replay_verification_enqueue_total() as i64);
    prom.galaxy_verification_replay_record_total
        .set(verification_replay_record_total() as i64);
    prom.galaxy_settlement_pending_verification_total
        .set(settlement_pending_verification_total() as i64);
    prom.galaxy_settlement_cleared_total
        .set(settlement_cleared_total() as i64);
    prom.galaxy_settlement_not_applicable_total
        .set(settlement_not_applicable_total() as i64);
    prom.galaxy_settlement_resolved_total
        .set(settlement_resolved_total() as i64);
    prom.galaxy_settlement_payout_batch_total
        .set(settlement_payout_batch_total() as i64);
    prom.galaxy_fee_split_applied_total
        .set(fee_split_applied_total() as i64);
}

/// Mirror in-process replication tier counters into Prometheus gauges (scrape snapshot).
pub fn refresh_galaxy_replication_gauges() {
    let prom = init_prometheus();
    prom.galaxy_replication_strict_total
        .set(replication_strict_total() as i64);
    prom.galaxy_replication_enqueue_total
        .set(replication_enqueue_total() as i64);
    prom.galaxy_replication_executor_enqueue_total
        .set(replication_executor_enqueue_total() as i64);
    prom.galaxy_replication_rate_limited_total
        .set(replication_rate_limited_total() as i64);
}

/// Record a secret rotation attempt (called from `security::secret_rotation`).
pub fn record_secret_rotation(kind: &str, success: bool) {
    let prom = init_prometheus();
    let success_label = if success { "true" } else { "false" };
    prom.secret_rotations_total
        .with_label_values(&[kind, success_label])
        .inc();
}

/// Record one completed HTTP request (called from middleware).
pub fn record_http_request(method: &str, status: u16, duration_secs: f64) {
    let prom = init_prometheus();
    let status_label = status.to_string();
    prom.http_requests_total
        .with_label_values(&[method, &status_label])
        .inc();
    prom.http_request_duration_seconds
        .with_label_values(&[method])
        .observe(duration_secs);
}

/// Refresh gauges from live application state before encoding the registry.
pub async fn refresh_scrape_gauges(ctx: &ApiContext) {
    let prom = init_prometheus();
    refresh_galaxy_pricing_gauges();
    refresh_galaxy_trust_gauges();
    refresh_galaxy_locality_gauges();
    refresh_galaxy_prefetch_gauges();
    refresh_galaxy_verification_gauges();
    refresh_galaxy_replication_gauges();
    prom.uptime_seconds
        .set(crate::version::get_uptime_seconds() as i64);
    prom.build_info.set(1);

    let state = ctx.get_system_state();
    prom.workers_active.set(state.active_workers as i64);
    prom.system_total_requests
        .set(state.system_metrics.total_requests as i64);

    #[cfg(feature = "enterprise")]
    {
        if let Ok(rules) = ctx.enterprise_monitoring_manager.list_alert_rules().await {
            prom.monitoring_alert_rules.set(rules.len() as i64);
        }
        if let Ok(dashboards) = ctx
            .enterprise_monitoring_manager
            .list_dashboards(None)
            .await
        {
            prom.monitoring_dashboards.set(dashboards.len() as i64);
        }
    }
}

/// Encode all registered metrics as Prometheus text exposition format 0.0.4.
pub fn encode_metrics_text() -> Result<String, prometheus::Error> {
    let prom = init_prometheus();
    let families = prom.registry.gather();
    let mut buffer = Vec::new();
    TextEncoder::new().encode(&families, &mut buffer)?;
    String::from_utf8(buffer).map_err(|e| prometheus::Error::Msg(format!("utf8: {e}")))
}

/// `GET /metrics` — Prometheus scrape endpoint (not JSON `/api/v1/metrics`).
pub async fn metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    refresh_scrape_gauges(&ctx).await;
    match encode_metrics_text() {
        Ok(body) => (
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                "text/plain; version=0.0.4; charset=utf-8",
            )],
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("prometheus encode failed: {e}"),
        )
            .into_response(),
    }
}

/// Tower middleware: count requests and observe latency (skips scrape path to avoid noise).
pub async fn prometheus_http_metrics(request: Request<Body>, next: Next) -> Response<Body> {
    if request.uri().path() == "/metrics" {
        return next.run(request).await;
    }
    let method = request.method().as_str().to_string();
    let started = std::time::Instant::now();
    let response = next.run(request).await;
    let status = response.status().as_u16();
    record_http_request(&method, status, started.elapsed().as_secs_f64());
    response
}

/// Attach HTTP metrics middleware (inner relative to later layers).
pub fn apply_prometheus_http_layer<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(axum::middleware::from_fn(prometheus_http_metrics))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_encode_contains_poolai_metrics() {
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains("poolai_build_info"));
        assert!(body.contains("poolai_uptime_seconds"));
        assert!(body.contains("poolai_workers_active"));
    }

    #[test]
    fn record_http_request_increments_counter() {
        init_prometheus();
        record_http_request("GET", 200, 0.01);
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(r#"method="GET""#) || body.contains("method=\"GET\""));
        assert!(body.contains("poolai_http_requests_total"));
    }

    #[test]
    fn encode_contains_galaxy_pricing_oracle_metrics() {
        init_prometheus();
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_FRESH_SERVED_TOTAL));
        assert!(body.contains(METRIC_STALE_SERVED_TOTAL));
        assert!(body.contains(METRIC_FORCED_FALLBACK_TOTAL));
        assert!(body.contains(METRIC_CACHE_AGE_SECONDS));
        assert!(body.contains(METRIC_QUOTE_USD_MICRO));
        assert!(body.contains(METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL));
        assert!(body.contains(METRIC_PROVIDER_CATALOG_HITS_TOTAL));
        assert!(body.contains(METRIC_PROVIDER_ERRORS_TOTAL));
    }

    #[test]
    fn galaxy_pricing_quote_usd_micro_gauge_reflects_last_quote_ph_s174() {
        use crate::grid::galaxy_pricing_oracle::{
            observe_last_quote_usd_micro, reset_last_quote_usd_micro_for_test,
        };

        reset_last_quote_usd_micro_for_test();
        init_prometheus();
        observe_last_quote_usd_micro(450_000);
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_QUOTE_USD_MICRO} 450000")));
        reset_last_quote_usd_micro_for_test();
    }

    #[test]
    fn galaxy_pricing_market_min_usd_micro_gauge_reflects_last_observed_ph_s181() {
        use crate::grid::galaxy_pricing_oracle::{
            observe_last_market_min_usd_micro, reset_last_market_min_usd_micro_for_test,
        };

        reset_last_market_min_usd_micro_for_test();
        init_prometheus();
        observe_last_market_min_usd_micro(500_000);
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_MARKET_MIN_USD_MICRO));
        assert!(body.contains(&format!("{METRIC_MARKET_MIN_USD_MICRO} 500000")));
        reset_last_market_min_usd_micro_for_test();
    }

    #[test]
    fn galaxy_pricing_provider_errors_gauge_reflects_fetch_fail_ph_s173() {
        use crate::grid::galaxy_pricing_provider_metrics::{
            record_provider_fetch_error, reset_provider_catalog_metrics_for_test,
        };

        reset_provider_catalog_metrics_for_test();
        init_prometheus();
        record_provider_fetch_error();
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_PROVIDER_ERRORS_TOTAL} 1")));
        reset_provider_catalog_metrics_for_test();
    }

    #[test]
    fn galaxy_pricing_provider_catalog_gauges_reflect_hits_ph_s172() {
        use crate::grid::galaxy_pricing_oracle::bundled_pricing_provider_catalog;
        use crate::grid::galaxy_pricing_provider_metrics::reset_provider_catalog_metrics_for_test;

        reset_provider_catalog_metrics_for_test();
        init_prometheus();
        let catalog = bundled_pricing_provider_catalog();
        let _ = catalog.matching_entries("inference:text", "gpt-4o-mini");
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_PROVIDER_CATALOG_LOOKUPS_TOTAL} 1")));
        assert!(body.contains(METRIC_PROVIDER_CATALOG_HITS_TOTAL));
        reset_provider_catalog_metrics_for_test();
    }

    #[test]
    fn galaxy_pricing_cache_age_gauge_reflects_observation() {
        use crate::grid::galaxy_pricing_oracle::{
            observe_l1_cache_age_secs, reset_pricing_cache_age_for_test,
        };
        reset_pricing_cache_age_for_test();
        init_prometheus();
        observe_l1_cache_age_secs(600);
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_CACHE_AGE_SECONDS} 600")));
        reset_pricing_cache_age_for_test();
    }

    #[test]
    fn galaxy_pricing_gauges_reflect_oracle_counters() {
        use crate::grid::galaxy_pricing_oracle::{
            bump_forced_fallback_for_test, bump_fresh_served_for_test, bump_stale_served_for_test,
            reset_forced_fallback_total_for_test, reset_fresh_served_total_for_test,
            reset_stale_served_total_for_test,
        };
        reset_fresh_served_total_for_test();
        reset_stale_served_total_for_test();
        reset_forced_fallback_total_for_test();
        bump_fresh_served_for_test();
        bump_stale_served_for_test();
        bump_forced_fallback_for_test();
        bump_forced_fallback_for_test();
        refresh_galaxy_pricing_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_FRESH_SERVED_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_STALE_SERVED_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_FORCED_FALLBACK_TOTAL} 2")));
    }

    #[test]
    fn encode_contains_galaxy_trust_settlement_metrics() {
        init_prometheus();
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_PAYOUT_ELIGIBLE_TOTAL));
        assert!(body.contains(METRIC_PAYOUT_HELD_TOTAL));
        assert!(body.contains(METRIC_TRUST_SCORE));
        assert!(body.contains(METRIC_TRUST_GATE_MIN_THRESHOLD));
        assert!(body.contains(METRIC_TRUST_GATE_DEFAULT_SCORE));
        assert!(body.contains(METRIC_GATE_EVALUATIONS_TOTAL));
        assert!(body.contains(METRIC_DEFAULT_SCORE_APPLIED_TOTAL));
    }

    #[test]
    fn galaxy_trust_gate_default_score_reflects_constant_ph_s384() {
        init_prometheus();
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_TRUST_GATE_DEFAULT_SCORE} 50")));
    }

    #[test]
    fn galaxy_trust_gate_min_threshold_reflects_env_ph_s374() {
        use crate::grid::galaxy_trust_score::ENV_MIN_TRUST_PAYOUT;
        let prior = std::env::var(ENV_MIN_TRUST_PAYOUT).ok();
        std::env::set_var(ENV_MIN_TRUST_PAYOUT, "55");
        init_prometheus();
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_TRUST_GATE_MIN_THRESHOLD} 55")));
        match prior {
            Some(v) => std::env::set_var(ENV_MIN_TRUST_PAYOUT, v),
            None => std::env::remove_var(ENV_MIN_TRUST_PAYOUT),
        }
    }

    #[test]
    fn galaxy_trust_score_gauge_reflects_last_observed() {
        use crate::grid::galaxy_trust_score::{
            observe_last_trust_score, reset_last_trust_score_for_test,
        };
        reset_last_trust_score_for_test();
        observe_last_trust_score(63);
        init_prometheus();
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_TRUST_SCORE} 63")));
        reset_last_trust_score_for_test();
    }

    #[test]
    fn encode_contains_galaxy_locality_metrics() {
        init_prometheus();
        refresh_galaxy_locality_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_SHARD_LOCAL_HIT_RATIO));
        assert!(body.contains(METRIC_CROSS_REGION_EGRESS_MB));
    }

    #[test]
    fn galaxy_cross_region_egress_mb_gauge_reflects_last_observed_ph_s185() {
        use crate::grid::galaxy_locality::{
            observe_last_cross_region_egress_mb, reset_last_cross_region_egress_mb_for_test,
        };
        reset_last_cross_region_egress_mb_for_test();
        observe_last_cross_region_egress_mb(120.0);
        init_prometheus();
        refresh_galaxy_locality_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_CROSS_REGION_EGRESS_MB} 120")));
        reset_last_cross_region_egress_mb_for_test();
    }

    #[test]
    fn galaxy_shard_local_hit_ratio_gauge_reflects_last_observed() {
        use crate::grid::galaxy_locality::{
            observe_last_shard_local_hit_ratio, reset_last_shard_local_hit_ratio_for_test,
        };
        reset_last_shard_local_hit_ratio_for_test();
        observe_last_shard_local_hit_ratio(0.75);
        init_prometheus();
        refresh_galaxy_locality_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_SHARD_LOCAL_HIT_RATIO} 7500")));
        reset_last_shard_local_hit_ratio_for_test();
    }

    #[test]
    fn galaxy_trust_gauges_reflect_settlement_counters() {
        use crate::grid::galaxy_trust_score::{
            record_settlement_gate_verdict, reset_settlement_gate_metrics_for_test,
            SettlementGateVerdict,
        };
        reset_settlement_gate_metrics_for_test();
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutEligible);
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutHeld);
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutHeld);
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_PAYOUT_ELIGIBLE_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_PAYOUT_HELD_TOTAL} 2")));
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn encode_contains_galaxy_prefetch_metrics() {
        init_prometheus();
        refresh_galaxy_prefetch_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_PREFETCH_PLAN_TOTAL));
        assert!(body.contains(METRIC_PREFETCH_PLANNED_SHARDS_TOTAL));
        assert!(body.contains(METRIC_PREFETCH_HOT_SKIP_TOTAL));
        assert!(body.contains(METRIC_PREFETCH_BYTES_TOTAL));
    }

    #[test]
    fn galaxy_prefetch_bytes_gauge_reflects_counter_ph_s184() {
        use crate::grid::galaxy_prefetch_metrics::{
            record_prefetch_plan, reset_prefetch_metrics_for_test,
        };
        reset_prefetch_metrics_for_test();
        record_prefetch_plan(2, 2, 16_777_216);
        init_prometheus();
        refresh_galaxy_prefetch_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_PREFETCH_BYTES_TOTAL} 16777216")));
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn galaxy_prefetch_gauges_reflect_plan_counters() {
        use crate::grid::galaxy_prefetch_metrics::{
            record_prefetch_plan, reset_prefetch_metrics_for_test,
        };
        reset_prefetch_metrics_for_test();
        record_prefetch_plan(2, 1, 4_194_304);
        refresh_galaxy_prefetch_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(&format!("{METRIC_PREFETCH_PLAN_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_PREFETCH_PLANNED_SHARDS_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_PREFETCH_HOT_SKIP_TOTAL} 1")));
        assert!(body.contains(&format!("{METRIC_PREFETCH_BYTES_TOTAL} 4194304")));
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn galaxy_verification_mismatch_gauge_reflects_counter_ph_s175() {
        use crate::grid::galaxy_verification_metrics::{
            record_verification_mismatch, reset_verification_mismatch_metrics_for_test,
        };

        reset_verification_mismatch_metrics_for_test();
        init_prometheus();
        record_verification_mismatch();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_VERIFICATION_MISMATCH_TOTAL));
        assert!(body.contains(&format!("{METRIC_VERIFICATION_MISMATCH_TOTAL} 1")));
        reset_verification_mismatch_metrics_for_test();
    }

    #[test]
    fn galaxy_verification_match_gauge_reflects_counter_ph_s180() {
        use crate::grid::galaxy_verification_metrics::{
            record_verification_match, reset_verification_match_metrics_for_test,
        };

        reset_verification_match_metrics_for_test();
        init_prometheus();
        record_verification_match();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_VERIFICATION_MATCH_TOTAL));
        assert!(body.contains(&format!("{METRIC_VERIFICATION_MATCH_TOTAL} 1")));
        reset_verification_match_metrics_for_test();
    }

    #[test]
    fn galaxy_verification_sample_gauge_reflects_counter_ph_s177() {
        use crate::grid::galaxy_verification_metrics::{
            record_verification_sample, reset_verification_sample_metrics_for_test,
        };

        reset_verification_sample_metrics_for_test();
        init_prometheus();
        record_verification_sample();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_VERIFICATION_SAMPLE_TOTAL));
        assert!(body.contains(&format!("{METRIC_VERIFICATION_SAMPLE_TOTAL} 1")));
        reset_verification_sample_metrics_for_test();
    }

    #[test]
    fn galaxy_verification_sample_scheduled_gauge_reflects_counter_ph_s186() {
        use crate::grid::galaxy_verify_sampling::{
            record_verify_sampling_verdict, reset_verify_sampling_metrics_for_test,
            VerifySamplingVerdict,
        };

        reset_verify_sampling_metrics_for_test();
        init_prometheus();
        record_verify_sampling_verdict(VerifySamplingVerdict::SampleScheduled);
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL));
        assert!(body.contains(&format!("{METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL} 1")));
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn galaxy_replay_pending_gauge_reflects_stub_ph_s176() {
        use crate::grid::galaxy_replay_metrics::{
            record_replay_pending_scheduled, reset_replay_pending_metrics_for_test,
        };

        reset_replay_pending_metrics_for_test();
        init_prometheus();
        record_replay_pending_scheduled();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_REPLAY_PENDING));
        assert!(body.contains(&format!("{METRIC_REPLAY_PENDING} 1")));
        reset_replay_pending_metrics_for_test();
    }

    #[test]
    fn galaxy_settlement_pending_verification_gauge_reflects_counter_ph_s178() {
        use crate::grid::galaxy_settlement_metrics::{
            record_settlement_pending_verification,
            reset_settlement_pending_verification_metrics_for_test,
        };

        reset_settlement_pending_verification_metrics_for_test();
        init_prometheus();
        record_settlement_pending_verification();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL));
        assert!(body.contains(&format!("{METRIC_SETTLEMENT_PENDING_VERIFICATION_TOTAL} 1")));
        reset_settlement_pending_verification_metrics_for_test();
    }

    #[test]
    fn galaxy_settlement_cleared_gauge_reflects_counter_ph_s187() {
        use crate::grid::galaxy_settlement_metrics::{
            record_settlement_cleared, reset_settlement_cleared_metrics_for_test,
        };

        reset_settlement_cleared_metrics_for_test();
        init_prometheus();
        record_settlement_cleared();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_SETTLEMENT_CLEARED_TOTAL));
        assert!(body.contains(&format!("{METRIC_SETTLEMENT_CLEARED_TOTAL} 1")));
        reset_settlement_cleared_metrics_for_test();
    }

    #[test]
    fn galaxy_settlement_resolved_gauge_reflects_counter_ph_s404() {
        use crate::grid::galaxy_settlement_metrics::{
            record_settlement_resolved, reset_settlement_resolved_metrics_for_test,
        };

        reset_settlement_resolved_metrics_for_test();
        init_prometheus();
        record_settlement_resolved();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_SETTLEMENT_RESOLVED_TOTAL));
        assert!(body.contains(&format!("{METRIC_SETTLEMENT_RESOLVED_TOTAL} 1")));
        reset_settlement_resolved_metrics_for_test();
    }

    #[test]
    fn galaxy_trust_explicit_score_gauge_reflects_counter_ph_s405() {
        use crate::grid::galaxy_trust_score::{
            evaluate_result_settlement_gate, reset_settlement_gate_metrics_for_test,
            TrustScoreGateConfig,
        };

        reset_settlement_gate_metrics_for_test();
        init_prometheus();
        let cfg = TrustScoreGateConfig::default_stub();
        evaluate_result_settlement_gate(Some("tg-peer"), Some(72), &cfg);
        refresh_galaxy_trust_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_EXPLICIT_SCORE_TOTAL));
        assert!(body.contains(&format!("{METRIC_EXPLICIT_SCORE_TOTAL} 1")));
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn galaxy_fee_split_applied_gauge_reflects_counter_ph_s194() {
        use crate::grid::galaxy_fee_split_metrics::{
            record_fee_split_applied, reset_fee_split_metrics_for_test,
        };

        reset_fee_split_metrics_for_test();
        init_prometheus();
        record_fee_split_applied();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_FEE_SPLIT_APPLIED_TOTAL));
        assert!(body.contains(&format!("{METRIC_FEE_SPLIT_APPLIED_TOTAL} 1")));
        reset_fee_split_metrics_for_test();
    }

    #[test]
    fn galaxy_replication_strict_gauge_reflects_counter_ph_s179() {
        use crate::grid::galaxy_replication_metrics::{
            record_replication_strict_ingest, reset_replication_strict_metrics_for_test,
        };

        reset_replication_strict_metrics_for_test();
        init_prometheus();
        record_replication_strict_ingest();
        refresh_galaxy_replication_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_REPLICATION_STRICT_TOTAL));
        assert!(body.contains(&format!("{METRIC_REPLICATION_STRICT_TOTAL} 1")));
        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn galaxy_prefetch_seed_pull_gauge_reflects_counter_ph_s424() {
        use crate::grid::galaxy_prefetch_metrics::{
            record_prefetch_seed_pull, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        init_prometheus();
        record_prefetch_seed_pull(2);
        refresh_galaxy_prefetch_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_PREFETCH_SEED_PULL_TOTAL));
        assert!(body.contains(&format!("{METRIC_PREFETCH_SEED_PULL_TOTAL} 2")));
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn galaxy_prefetch_lease_acquired_gauge_reflects_counter_ph_s425() {
        use crate::grid::galaxy_prefetch_metrics::{
            record_prefetch_lease_acquired, reset_prefetch_metrics_for_test,
        };

        reset_prefetch_metrics_for_test();
        init_prometheus();
        record_prefetch_lease_acquired();
        refresh_galaxy_prefetch_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL));
        assert!(body.contains(&format!("{METRIC_PREFETCH_LEASE_ACQUIRED_TOTAL} 1")));
        reset_prefetch_metrics_for_test();
    }

    #[test]
    fn galaxy_replication_enqueue_gauge_reflects_counter_ph_s426() {
        use crate::grid::galaxy_replication_metrics::{
            record_replication_enqueue, reset_replication_strict_metrics_for_test,
        };

        reset_replication_strict_metrics_for_test();
        init_prometheus();
        record_replication_enqueue();
        refresh_galaxy_replication_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_REPLICATION_ENQUEUE_TOTAL));
        assert!(body.contains(&format!("{METRIC_REPLICATION_ENQUEUE_TOTAL} 1")));
        reset_replication_strict_metrics_for_test();
    }

    #[test]
    fn galaxy_settlement_payout_batch_gauge_reflects_counter_ph_s427() {
        use crate::grid::galaxy_settlement_metrics::{
            record_settlement_payout_batch, reset_settlement_metrics_for_test,
        };

        reset_settlement_metrics_for_test();
        init_prometheus();
        record_settlement_payout_batch();
        refresh_galaxy_verification_gauges();
        let body = encode_metrics_text().expect("encode");
        assert!(body.contains(METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL));
        assert!(body.contains(&format!("{METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL} 1")));
        reset_settlement_metrics_for_test();
    }
}
