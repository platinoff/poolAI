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
}

/// Classify stand smoke metrics parity depth from optional feature stub (PH-S714/PH-S724).
pub fn stand_smoke_metrics_parity_depth_stub(
    features: Option<&Value>,
) -> StandSmokeMetricsParityDepth {
    let Some(f) = features else {
        return StandSmokeMetricsParityDepth::None;
    };
    if f.get("network_profile_persist")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return StandSmokeMetricsParityDepth::NetworkProfile;
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
