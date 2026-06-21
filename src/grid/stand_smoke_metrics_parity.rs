//! Stand smoke JSON `/api/v1/grid/*-metrics` vs Prometheus `/metrics` parity (PH-S710…S714).

use serde_json::Value;

/// Required JSON export keys for stand smoke verification-metrics API (PH-S710).
pub const VERIFICATION_JSON_KEYS: &[&str] = &[
    "sample_total",
    "mismatch_total",
    "match_total",
    "checker_pending_total",
];

/// Required JSON export keys for stand smoke replay-metrics API (PH-S710).
pub const REPLAY_JSON_KEYS: &[&str] = &[
    "replay_pending",
    "replay_pending_scheduled_total",
    "verification_replay_record_total",
];

/// Required JSON export keys for stand smoke settlement-metrics API (PH-S711).
pub const SETTLEMENT_JSON_KEYS: &[&str] = &[
    "pending_verification_total",
    "cleared_total",
    "resolved_total",
    "payout_batch_total",
];

/// Required JSON export keys for stand smoke trust-metrics API (PH-S711).
pub const TRUST_JSON_KEYS: &[&str] = &[
    "payout_eligible_total",
    "payout_held_total",
    "last_trust_score",
    "gate_min_threshold",
];

/// Required JSON export keys for stand smoke replication-metrics API (PH-S711).
pub const REPLICATION_JSON_KEYS: &[&str] = &[
    "strict_total",
    "enqueue_total",
    "executor_enqueue_total",
    "rate_limited_total",
];

/// Required JSON export keys for stand smoke pricing-metrics API (PH-S711).
pub const PRICING_JSON_KEYS: &[&str] = &[
    "fresh_served_total",
    "stale_served_total",
    "forced_fallback_total",
    "provider_catalog_lookups_total",
];

/// Required JSON export keys for stand smoke prefetch-metrics API (PH-S753).
pub const PREFETCH_JSON_KEYS: &[&str] = &[
    "pull_bytes_total",
    "backpressure_total",
    "plan_total",
    "enqueue_total",
    "peer_fetch_total",
];

/// Prometheus gauge name ↔ JSON metrics field pairs — prefetch live pull (PH-S750).
pub const PREFETCH_LIVE_PULL_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_prefetch_metrics::METRIC_PREFETCH_PULL_BYTES_TOTAL,
        "pull_bytes_total",
    ),
    (
        crate::grid::galaxy_prefetch_metrics::METRIC_PREFETCH_BACKPRESSURE_TOTAL,
        "backpressure_total",
    ),
];

/// Required JSON export keys for stand smoke locality-metrics API (PH-S763).
pub const LOCALITY_JSON_KEYS: &[&str] = &[
    "shard_local_hit_ratio_bps",
    "hot_tier_hit_ratio_bps",
    "cross_region_egress_mb",
    "hot_promote_total",
    "hot_evict_total",
];

/// Prometheus gauge name ↔ JSON metrics field pairs — locality hot-tier (PH-S761).
pub const LOCALITY_HOT_TIER_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_locality::METRIC_SHARD_LOCAL_HIT_RATIO,
        "shard_local_hit_ratio_bps",
    ),
    (
        crate::grid::galaxy_locality::METRIC_HOT_TIER_HIT_RATIO,
        "hot_tier_hit_ratio_bps",
    ),
    (
        crate::grid::galaxy_locality::METRIC_CROSS_REGION_EGRESS_MB,
        "cross_region_egress_mb",
    ),
    (
        crate::grid::galaxy_prefetch_metrics::METRIC_HOT_PROMOTE_TOTAL,
        "hot_promote_total",
    ),
    (
        crate::grid::galaxy_prefetch_metrics::METRIC_HOT_EVICT_TOTAL,
        "hot_evict_total",
    ),
];

/// Required JSON export keys for stand smoke payout-batch-metrics API (PH-S772).
pub const PAYOUT_BATCH_JSON_KEYS: &[&str] = &[
    "payout_batch_total",
    "payout_batch_queue_depth",
    "onchain_submit_total",
];

/// Required JSON export keys for stand smoke fee-split-metrics API (PH-S782).
pub const FEE_SPLIT_JSON_KEYS: &[&str] = &[
    "fee_split_applied_total",
    "primary_dev_fee_bps",
    "secondary_admin_fee_min_bps",
    "secondary_admin_fee_max_bps",
];

/// Required JSON export keys for stand smoke governance-metrics API (PH-S793).
pub const GOVERNANCE_JSON_KEYS: &[&str] = &[
    "release_verify_total",
    "release_verify_fail_total",
    "update_notify_pending",
    "advisory_acknowledged_total",
];

/// Required JSON export keys for stand smoke update-policy API (PH-S790).
pub const UPDATE_POLICY_JSON_KEYS: &[&str] = &["mode", "env_update_policy", "env_manifest_url"];

/// Prometheus gauge name ↔ JSON metrics field pairs — fee split production (PH-S780).
pub const FEE_SPLIT_APPLIED_PARITY: &[(&str, &str)] = &[(
    crate::grid::galaxy_fee_split_metrics::METRIC_FEE_SPLIT_APPLIED_TOTAL,
    "fee_split_applied_total",
)];

/// Prometheus gauge name ↔ JSON metrics field pairs — governance ops (PH-S791).
pub const GOVERNANCE_METRICS_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_governance_metrics::METRIC_RELEASE_VERIFY_TOTAL,
        "release_verify_total",
    ),
    (
        crate::grid::galaxy_governance_metrics::METRIC_RELEASE_VERIFY_FAIL_TOTAL,
        "release_verify_fail_total",
    ),
    (
        crate::grid::galaxy_governance_metrics::METRIC_UPDATE_NOTIFY_PENDING,
        "update_notify_pending",
    ),
    (
        crate::grid::galaxy_security_advisory::METRIC_ADVISORY_ACKNOWLEDGED_TOTAL,
        "advisory_acknowledged_total",
    ),
];

/// Prometheus gauge name ↔ JSON metrics field pairs — payout batch settlement (PH-S771).
pub const PAYOUT_BATCH_SETTLEMENT_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_settlement_metrics::METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL,
        "payout_batch_total",
    ),
    (
        crate::grid::galaxy_settlement_payout_batch_queue::METRIC_SETTLEMENT_PAYOUT_BATCH_QUEUE_DEPTH,
        "payout_batch_queue_depth",
    ),
    (
        crate::grid::galaxy_settlement_onchain::METRIC_SETTLEMENT_ONCHAIN_SUBMIT_TOTAL,
        "onchain_submit_total",
    ),
];

/// Prometheus gauge name ↔ JSON metrics field pairs — verification + replay (PH-S710).
pub const VERIFICATION_REPLAY_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_verification_metrics::METRIC_VERIFICATION_SAMPLE_TOTAL,
        "sample_total",
    ),
    (
        crate::grid::galaxy_verification_metrics::METRIC_VERIFICATION_CHECKER_PENDING_TOTAL,
        "checker_pending_total",
    ),
    (
        crate::grid::galaxy_replay_metrics::METRIC_REPLAY_PENDING,
        "replay_pending",
    ),
    (
        crate::grid::galaxy_replay_metrics::METRIC_VERIFICATION_REPLAY_RECORD_TOTAL,
        "verification_replay_record_total",
    ),
];

/// Prometheus gauge name ↔ JSON metrics field pairs — settlement + trust (PH-S711).
pub const SETTLEMENT_TRUST_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_settlement_metrics::METRIC_SETTLEMENT_CLEARED_TOTAL,
        "cleared_total",
    ),
    (
        crate::grid::galaxy_settlement_metrics::METRIC_SETTLEMENT_PAYOUT_BATCH_TOTAL,
        "payout_batch_total",
    ),
    (
        crate::grid::galaxy_trust_score::METRIC_PAYOUT_ELIGIBLE_TOTAL,
        "payout_eligible_total",
    ),
    (
        crate::grid::galaxy_trust_score::METRIC_TRUST_SCORE,
        "last_trust_score",
    ),
];

/// Prometheus gauge name ↔ JSON metrics field pairs — replication + pricing (PH-S711).
pub const REPLICATION_PRICING_PARITY: &[(&str, &str)] = &[
    (
        crate::grid::galaxy_replication_metrics::METRIC_REPLICATION_STRICT_TOTAL,
        "strict_total",
    ),
    (
        crate::grid::galaxy_replication_metrics::METRIC_REPLICATION_ENQUEUE_TOTAL,
        "enqueue_total",
    ),
    (
        crate::grid::galaxy_pricing_oracle::METRIC_FRESH_SERVED_TOTAL,
        "fresh_served_total",
    ),
    (
        crate::grid::galaxy_pricing_oracle::METRIC_STALE_SERVED_TOTAL,
        "stale_served_total",
    ),
];

/// Stand smoke metrics parity depth classification (PH-S714; band 7 PH-S724).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandSmokeMetricsParityDepth {
    None,
    JsonExport,
    PrometheusJson,
    ReMigratePolicy,
    RoutingLocality,
    NetworkProfile,
    CapabilityAdmission,
    PrefetchLivePull,
    LocalityHotTier,
    PayoutBatchSettlement,
    FeeSplitProduction,
    GovernanceOps,
    /// Stand smoke v2 — full grid JSON↔Prom parity (PH-S830 band 18).
    FullGridParityV2,
    /// Memory shard persist + seed-inventory depth (PH-S863 band 21).
    MemoryShardPersist,
    /// On-chain cleared settlement depth + metrics (PH-S873 band 22).
    OnChainSettlement,
}

/// Classify stand smoke metrics parity depth from optional feature stub (PH-S714/PH-S724).
pub fn stand_smoke_metrics_parity_depth_stub(
    features: Option<&Value>,
) -> StandSmokeMetricsParityDepth {
    let Some(f) = features else {
        return StandSmokeMetricsParityDepth::None;
    };
    if f.get("full_grid_parity_v2")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::FullGridParityV2;
    }
    if f.get("memory_shard_persist")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::MemoryShardPersist;
    }
    if f.get("on_chain_settlement")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::OnChainSettlement;
    }
    if f.get("network_profile_persist")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::NetworkProfile;
    }
    if f.get("prefetch_live_pull")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::PrefetchLivePull;
    }
    if f.get("locality_hot_tier")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::LocalityHotTier;
    }
    if f.get("fee_split_production")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::FeeSplitProduction;
    }
    if f.get("governance_ops")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::GovernanceOps;
    }
    if f.get("payout_batch_settlement")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::PayoutBatchSettlement;
    }
    if f.get("capability_admission")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::CapabilityAdmission;
    }
    if f.get("routing_locality_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::RoutingLocality;
    }
    if f.get("re_migrate_policy")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::ReMigratePolicy;
    }
    if f.get("prometheus_json_parity")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::PrometheusJson;
    }
    if f.get("json_export_shape")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::JsonExport;
    }
    StandSmokeMetricsParityDepth::None
}

fn parse_prometheus_gauge(metrics_text: &str, metric_name: &str) -> u64 {
    let needle = format!("{metric_name} ");
    for line in metrics_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || !trimmed.starts_with(&needle) {
            continue;
        }
        if let Some(value) = trimmed.strip_prefix(&needle) {
            if let Ok(parsed) = value.trim().parse::<f64>() {
                return parsed.max(0.0) as u64;
            }
        }
    }
    0
}

/// Validate `GET /api/v1/grid/*-metrics` JSON export shape (PH-S710/S711).
pub fn validate_grid_metrics_json_export(
    body: &Value,
    required_keys: &[&str],
) -> Result<(), String> {
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("metrics body missing ok:true: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("metrics body missing metrics: {body}"))?;
    for key in required_keys {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("metrics missing u64 key {key}: {body}"));
        }
    }
    Ok(())
}

/// Validate Prometheus gauge matches JSON metrics field (PH-S710/S711).
pub fn validate_prometheus_json_parity(
    prom_text: &str,
    json_body: &Value,
    prom_name: &str,
    json_key: &str,
) -> Result<(), String> {
    let metrics = json_body
        .get("metrics")
        .ok_or_else(|| format!("json missing metrics: {json_body}"))?;
    let prom_val = parse_prometheus_gauge(prom_text, prom_name);
    let json_val = metrics.get(json_key).and_then(|v| v.as_u64()).unwrap_or(0);
    if prom_val != json_val {
        return Err(format!(
            "parity mismatch {prom_name} prom={prom_val} vs json {json_key}={json_val}"
        ));
    }
    Ok(())
}

/// Run parity pairs against one JSON metrics body (PH-S713).
pub fn validate_prometheus_json_parity_pairs(
    prom_text: &str,
    json_body: &Value,
    pairs: &[(&str, &str)],
) -> Result<(), String> {
    for (prom_name, json_key) in pairs {
        validate_prometheus_json_parity(prom_text, json_body, prom_name, json_key)?;
    }
    Ok(())
}

/// Full band-6 stand smoke JSON export + Prometheus parity gate (PH-S713).
pub fn validate_band6_metrics_parity(
    prom_text: &str,
    verification: &Value,
    replay: &Value,
    settlement: &Value,
    trust: &Value,
    replication: &Value,
    pricing: &Value,
) -> Result<(), String> {
    validate_grid_metrics_json_export(verification, VERIFICATION_JSON_KEYS)?;
    validate_grid_metrics_json_export(replay, REPLAY_JSON_KEYS)?;
    validate_grid_metrics_json_export(settlement, SETTLEMENT_JSON_KEYS)?;
    validate_grid_metrics_json_export(trust, TRUST_JSON_KEYS)?;
    validate_grid_metrics_json_export(replication, REPLICATION_JSON_KEYS)?;
    validate_grid_metrics_json_export(pricing, PRICING_JSON_KEYS)?;

    validate_prometheus_json_parity_pairs(
        prom_text,
        verification,
        &VERIFICATION_REPLAY_PARITY[..2],
    )?;
    validate_prometheus_json_parity_pairs(prom_text, replay, &VERIFICATION_REPLAY_PARITY[2..])?;
    validate_prometheus_json_parity_pairs(prom_text, settlement, &SETTLEMENT_TRUST_PARITY[..2])?;
    validate_prometheus_json_parity_pairs(prom_text, trust, &SETTLEMENT_TRUST_PARITY[2..])?;
    validate_prometheus_json_parity_pairs(
        prom_text,
        replication,
        &REPLICATION_PRICING_PARITY[..2],
    )?;
    validate_prometheus_json_parity_pairs(prom_text, pricing, &REPLICATION_PRICING_PARITY[2..])?;
    Ok(())
}

/// Full grid stand smoke v2 — band-6 core + prefetch/locality/fee/governance/payout-batch (PH-S830).
pub fn validate_band6_metrics_parity_v2(
    prom_text: &str,
    verification: &Value,
    replay: &Value,
    settlement: &Value,
    trust: &Value,
    replication: &Value,
    pricing: &Value,
    prefetch: &Value,
    locality: &Value,
    fee_split: &Value,
    governance: &Value,
    payout_batch: &Value,
) -> Result<(), String> {
    validate_band6_metrics_parity(
        prom_text,
        verification,
        replay,
        settlement,
        trust,
        replication,
        pricing,
    )?;
    validate_prefetch_metrics_parity(prom_text, prefetch)?;
    validate_locality_metrics_parity(prom_text, locality)?;
    validate_fee_split_metrics_parity(prom_text, fee_split)?;
    validate_governance_metrics_parity(prom_text, governance)?;
    validate_payout_batch_metrics_parity(prom_text, payout_batch)?;
    Ok(())
}

/// Settlement + trust JSON export + Prometheus parity gate (PH-S723).
pub fn validate_settlement_trust_metrics_parity(
    prom_text: &str,
    settlement: &Value,
    trust: &Value,
) -> Result<(), String> {
    validate_grid_metrics_json_export(settlement, SETTLEMENT_JSON_KEYS)?;
    validate_grid_metrics_json_export(trust, TRUST_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, settlement, &SETTLEMENT_TRUST_PARITY[..2])?;
    validate_prometheus_json_parity_pairs(prom_text, trust, &SETTLEMENT_TRUST_PARITY[2..])?;
    Ok(())
}

/// Prefetch live pull JSON export + Prometheus parity gate (PH-S750).
pub fn validate_prefetch_metrics_parity(prom_text: &str, prefetch: &Value) -> Result<(), String> {
    validate_grid_metrics_json_export(prefetch, PREFETCH_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, prefetch, PREFETCH_LIVE_PULL_PARITY)?;
    Ok(())
}

/// Validate locality-metrics JSON export vs Prometheus gauges (PH-S761).
pub fn validate_locality_metrics_parity(prom_text: &str, locality: &Value) -> Result<(), String> {
    validate_grid_metrics_json_export(locality, LOCALITY_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, locality, LOCALITY_HOT_TIER_PARITY)?;
    Ok(())
}

/// Validate payout-batch-metrics JSON export vs Prometheus gauges (PH-S771).
pub fn validate_payout_batch_metrics_parity(
    prom_text: &str,
    payout_batch: &Value,
) -> Result<(), String> {
    validate_grid_metrics_json_export(payout_batch, PAYOUT_BATCH_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, payout_batch, PAYOUT_BATCH_SETTLEMENT_PARITY)?;
    Ok(())
}

/// Validate fee-split-metrics JSON export vs Prometheus gauges (PH-S780).
pub fn validate_fee_split_metrics_parity(prom_text: &str, fee_split: &Value) -> Result<(), String> {
    validate_grid_metrics_json_export(fee_split, FEE_SPLIT_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, fee_split, FEE_SPLIT_APPLIED_PARITY)?;
    Ok(())
}

/// Validate governance-metrics JSON export vs Prometheus gauges (PH-S791).
pub fn validate_governance_metrics_parity(
    prom_text: &str,
    governance: &Value,
) -> Result<(), String> {
    validate_grid_metrics_json_export(governance, GOVERNANCE_JSON_KEYS)?;
    validate_prometheus_json_parity_pairs(prom_text, governance, GOVERNANCE_METRICS_PARITY)?;
    Ok(())
}

/// Validate update-policy JSON export shape (PH-S790).
pub fn validate_update_policy_json_export(body: &Value) -> Result<(), String> {
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("update-policy ok=false: {body}"));
    }
    let policy = body
        .get("policy")
        .ok_or_else(|| "update-policy missing policy".to_string())?;
    for key in UPDATE_POLICY_JSON_KEYS {
        if policy.get(key).is_none() {
            return Err(format!("update-policy.policy missing {key}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_verification_replay_json_export_ph_s710() {
        let body = json!({
            "ok": true,
            "metrics": {
                "sample_total": 0,
                "mismatch_total": 0,
                "match_total": 0,
                "checker_pending_total": 0,
            }
        });
        validate_grid_metrics_json_export(&body, VERIFICATION_JSON_KEYS).expect("verification");

        let replay = json!({
            "ok": true,
            "metrics": {
                "replay_pending": 0,
                "replay_pending_scheduled_total": 0,
                "verification_replay_record_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replay, REPLAY_JSON_KEYS).expect("replay");
    }

    #[test]
    fn validate_settlement_trust_replication_pricing_json_export_ph_s711() {
        let settlement = json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 0,
                "resolved_total": 0,
                "payout_batch_total": 0,
            }
        });
        validate_grid_metrics_json_export(&settlement, SETTLEMENT_JSON_KEYS).expect("settlement");

        let trust = json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 0,
                "payout_held_total": 0,
                "last_trust_score": 0,
                "gate_min_threshold": 40,
            }
        });
        validate_grid_metrics_json_export(&trust, TRUST_JSON_KEYS).expect("trust");

        let replication = json!({
            "ok": true,
            "metrics": {
                "strict_total": 0,
                "enqueue_total": 0,
                "executor_enqueue_total": 0,
                "rate_limited_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replication, REPLICATION_JSON_KEYS)
            .expect("replication");

        let pricing = json!({
            "ok": true,
            "metrics": {
                "fresh_served_total": 0,
                "stale_served_total": 0,
                "forced_fallback_total": 0,
                "provider_catalog_lookups_total": 0,
            }
        });
        validate_grid_metrics_json_export(&pricing, PRICING_JSON_KEYS).expect("pricing");
    }

    #[test]
    fn prometheus_json_parity_sample_ph_s710() {
        let prom = concat!(
            "galaxy_verification_sample_total 2\n",
            "galaxy_verification_checker_pending_total 1\n",
        );
        let body = json!({
            "ok": true,
            "metrics": { "sample_total": 2, "checker_pending_total": 1 }
        });
        validate_prometheus_json_parity(
            prom,
            &body,
            "galaxy_verification_sample_total",
            "sample_total",
        )
        .expect("sample");
        validate_prometheus_json_parity(
            prom,
            &body,
            "galaxy_verification_checker_pending_total",
            "checker_pending_total",
        )
        .expect("pending");
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_ph_s714() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"json_export_shape": true}))),
            StandSmokeMetricsParityDepth::JsonExport
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"prometheus_json_parity": true}))),
            StandSmokeMetricsParityDepth::PrometheusJson
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(None),
            StandSmokeMetricsParityDepth::None
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band7_ph_s724() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"re_migrate_policy": true}))),
            StandSmokeMetricsParityDepth::ReMigratePolicy
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"routing_locality_gate": true}))),
            StandSmokeMetricsParityDepth::RoutingLocality
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band8_ph_s734() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"network_profile_persist": true}))),
            StandSmokeMetricsParityDepth::NetworkProfile
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band9_ph_s744() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"capability_admission": true}))),
            StandSmokeMetricsParityDepth::CapabilityAdmission
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band10_ph_s754() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"prefetch_live_pull": true}))),
            StandSmokeMetricsParityDepth::PrefetchLivePull
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band11_ph_s764() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"locality_hot_tier": true}))),
            StandSmokeMetricsParityDepth::LocalityHotTier
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band12_ph_s774() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"payout_batch_settlement": true}))),
            StandSmokeMetricsParityDepth::PayoutBatchSettlement
        );
    }

    #[test]
    fn payout_batch_metrics_parity_ph_s771() {
        let prom = concat!(
            "galaxy_settlement_payout_batch_total 3\n",
            "galaxy_settlement_payout_batch_queue_depth 2\n",
            "galaxy_settlement_onchain_submit_total 1\n",
        );
        let body = json!({
            "ok": true,
            "metrics": {
                "payout_batch_total": 3,
                "payout_batch_queue_depth": 2,
                "onchain_submit_total": 1,
                "settlement_mode": "offline_batch",
            }
        });
        validate_payout_batch_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band13_ph_s783() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"fee_split_production": true}))),
            StandSmokeMetricsParityDepth::FeeSplitProduction
        );
    }

    #[test]
    fn fee_split_metrics_parity_ph_s780() {
        let prom = "galaxy_fee_split_applied_total 4\n";
        let body = json!({
            "ok": true,
            "metrics": {
                "fee_split_applied_total": 4,
                "primary_dev_fee_bps": 10,
                "secondary_admin_fee_min_bps": 100,
                "secondary_admin_fee_max_bps": 500,
            }
        });
        validate_fee_split_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn prefetch_metrics_parity_ph_s750() {
        let prom = concat!(
            "galaxy_prefetch_pull_bytes_total 4194304\n",
            "galaxy_prefetch_backpressure_total 2\n",
        );
        let body = json!({
            "ok": true,
            "metrics": {
                "pull_bytes_total": 4194304,
                "backpressure_total": 2,
                "plan_total": 1,
                "enqueue_total": 0,
                "peer_fetch_total": 0,
            }
        });
        validate_prefetch_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn locality_metrics_parity_ph_s761() {
        let prom = concat!(
            "galaxy_shard_local_hit_ratio 8000\n",
            "galaxy_hot_tier_hit_ratio 5000\n",
            "galaxy_cross_region_egress_mb 10\n",
            "galaxy_hot_promote_total 2\n",
            "galaxy_hot_evict_total 1\n",
        );
        let body = json!({
            "ok": true,
            "metrics": {
                "shard_local_hit_ratio_bps": 8000,
                "hot_tier_hit_ratio_bps": 5000,
                "cross_region_egress_mb": 10,
                "hot_promote_total": 2,
                "hot_evict_total": 1,
            }
        });
        validate_locality_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band18_ph_s834() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"full_grid_parity_v2": true}))),
            StandSmokeMetricsParityDepth::FullGridParityV2
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band22_ph_s873() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"on_chain_settlement": true}))),
            StandSmokeMetricsParityDepth::OnChainSettlement
        );
    }

    #[test]
    fn stand_smoke_metrics_parity_depth_stub_band21_ph_s863() {
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"memory_shard_persist": true}))),
            StandSmokeMetricsParityDepth::MemoryShardPersist
        );
    }

    #[test]
    fn validate_band6_metrics_parity_v2_ph_s830() {
        let prom = concat!(
            "galaxy_verification_sample_total 0\n",
            "galaxy_verification_checker_pending_total 0\n",
            "galaxy_replay_pending 0\n",
            "galaxy_verification_replay_record_total 0\n",
            "galaxy_settlement_cleared_total 0\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "galaxy_trust_score 0\n",
            "galaxy_replication_strict_total 0\n",
            "galaxy_replication_enqueue_total 0\n",
            "galaxy_pricing_fresh_served 0\n",
            "galaxy_pricing_stale_served 0\n",
            "galaxy_prefetch_pull_bytes_total 0\n",
            "galaxy_prefetch_backpressure_total 0\n",
            "galaxy_shard_local_hit_ratio 0\n",
            "galaxy_hot_tier_hit_ratio 0\n",
            "galaxy_cross_region_egress_mb 0\n",
            "galaxy_hot_promote_total 0\n",
            "galaxy_hot_evict_total 0\n",
            "galaxy_fee_split_applied_total 0\n",
            "poolai_release_verify_total 0\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 0\n",
            "poolai_advisory_acknowledged_total 0\n",
            "galaxy_settlement_payout_batch_queue_depth 0\n",
            "galaxy_settlement_onchain_submit_total 0\n",
        );
        let verification = json!({"ok": true, "metrics": {"sample_total": 0, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
        let replay = json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
        let settlement = json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
        let trust = json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
        let replication = json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
        let pricing = json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0}});
        let prefetch = json!({"ok": true, "metrics": {"pull_bytes_total": 0, "backpressure_total": 0, "plan_total": 0, "enqueue_total": 0, "peer_fetch_total": 0}});
        let locality = json!({"ok": true, "metrics": {"shard_local_hit_ratio_bps": 0, "hot_tier_hit_ratio_bps": 0, "cross_region_egress_mb": 0, "hot_promote_total": 0, "hot_evict_total": 0}});
        let fee_split = json!({"ok": true, "metrics": {"fee_split_applied_total": 0, "primary_dev_fee_bps": 10, "secondary_admin_fee_min_bps": 100, "secondary_admin_fee_max_bps": 500}});
        let governance = json!({"ok": true, "metrics": {"release_verify_total": 0, "release_verify_fail_total": 0, "update_notify_pending": 0, "advisory_acknowledged_total": 0}});
        let payout_batch = json!({"ok": true, "metrics": {"payout_batch_total": 0, "payout_batch_queue_depth": 0, "onchain_submit_total": 0}});
        validate_band6_metrics_parity_v2(
            prom,
            &verification,
            &replay,
            &settlement,
            &trust,
            &replication,
            &pricing,
            &prefetch,
            &locality,
            &fee_split,
            &governance,
            &payout_batch,
        )
        .expect("v2 parity");
    }

    #[test]
    fn settlement_trust_metrics_parity_ph_s723() {
        let prom = concat!(
            "galaxy_settlement_cleared_total 2\n",
            "galaxy_settlement_payout_batch_total 1\n",
            "galaxy_trust_payout_eligible_total 3\n",
            "galaxy_trust_score 55\n",
        );
        let settlement = json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 2,
                "resolved_total": 0,
                "payout_batch_total": 1,
            }
        });
        let trust = json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 3,
                "payout_held_total": 0,
                "last_trust_score": 55,
                "gate_min_threshold": 40,
            }
        });
        validate_settlement_trust_metrics_parity(prom, &settlement, &trust).expect("parity");
    }
}
