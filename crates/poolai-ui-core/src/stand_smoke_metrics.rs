//! Stand smoke JSON metrics export shape helpers (PH-S712).

use crate::prometheus::parse_prometheus_gauge;
use serde_json::Value;

/// Read u64 counter from grid metrics JSON object.
pub fn grid_metrics_u64(metrics: &Value, key: &str) -> u64 {
    metrics.get(key).and_then(|v| v.as_u64()).unwrap_or(0)
}

/// Validate `GET /api/v1/grid/*-metrics` JSON export shape.
pub fn validate_grid_metrics_json_shape(
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

/// Reconcile one Prometheus gauge with a JSON metrics field (PH-S712 admin glue).
pub fn reconcile_prometheus_json_gauge(
    prom_text: &str,
    json_body: &Value,
    prom_name: &str,
    json_key: &str,
) -> Result<(), String> {
    let metrics = json_body
        .get("metrics")
        .ok_or_else(|| format!("json missing metrics: {json_body}"))?;
    let prom_val = parse_prometheus_gauge(prom_text, prom_name);
    let json_val = grid_metrics_u64(metrics, json_key);
    if prom_val != json_val {
        return Err(format!(
            "parity mismatch {prom_name} prom={prom_val} vs json {json_key}={json_val}"
        ));
    }
    Ok(())
}

/// Verification metrics strip for admin panel when JSON + Prometheus agree (PH-S712).
pub fn render_grid_verification_metrics_strip_html(
    verification_metrics_json: &str,
    pending_total: u64,
) -> String {
    use crate::format::escape_html;

    let body: Value = serde_json::from_str(verification_metrics_json).unwrap_or(Value::Null);
    let metrics = body.get("metrics").cloned().unwrap_or(body);
    let sample = grid_metrics_u64(&metrics, "sample_total");
    let pending_json = grid_metrics_u64(&metrics, "checker_pending_total");
    let pending = if pending_json > 0 {
        pending_json
    } else {
        pending_total
    };

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>Sample: <strong>{sample}</strong></span>
<span>Pending: <strong>{pending}</strong></span>
</div>"#,
        sample = escape_html(&sample.to_string()),
        pending = escape_html(&pending.to_string()),
    )
}

/// Settlement + trust metrics strip for admin payout panel (PH-S722).
pub fn render_grid_settlement_trust_metrics_strip_html(
    settlement_metrics_json: &str,
    trust_metrics_json: &str,
    trust_score_gauge: u64,
) -> String {
    use crate::format::escape_html;

    let settlement_body: Value =
        serde_json::from_str(settlement_metrics_json).unwrap_or(Value::Null);
    let trust_body: Value = serde_json::from_str(trust_metrics_json).unwrap_or(Value::Null);
    let sm = settlement_body
        .get("metrics")
        .cloned()
        .unwrap_or(settlement_body);
    let tm = trust_body.get("metrics").cloned().unwrap_or(trust_body);
    let cleared = grid_metrics_u64(&sm, "cleared_total");
    let eligible = grid_metrics_u64(&tm, "payout_eligible_total");
    let score = if grid_metrics_u64(&tm, "last_trust_score") > 0 {
        grid_metrics_u64(&tm, "last_trust_score")
    } else {
        trust_score_gauge
    };

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>Cleared: <strong>{cleared}</strong></span>
<span>Eligible: <strong>{eligible}</strong></span>
<span>Trust score: <strong>{score}</strong></span>
</div>"#,
        cleared = escape_html(&cleared.to_string()),
        eligible = escape_html(&eligible.to_string()),
        score = escape_html(&score.to_string()),
    )
}

/// Prefetch live pull metrics strip for admin updates panel (PH-S752).
pub fn render_grid_prefetch_metrics_strip_html(
    prefetch_metrics_json: &str,
    pull_bytes_gauge: u64,
) -> String {
    use crate::format::escape_html;

    let body: Value = serde_json::from_str(prefetch_metrics_json).unwrap_or(Value::Null);
    let metrics = body.get("metrics").cloned().unwrap_or(body);
    let pull_bytes = if grid_metrics_u64(&metrics, "pull_bytes_total") > 0 {
        grid_metrics_u64(&metrics, "pull_bytes_total")
    } else {
        pull_bytes_gauge
    };
    let backpressure = grid_metrics_u64(&metrics, "backpressure_total");
    let peer_fetch = grid_metrics_u64(&metrics, "peer_fetch_total");

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>Pull bytes: <strong>{pull_bytes}</strong></span>
<span>Backpressure: <strong>{backpressure}</strong></span>
<span>Peer fetch: <strong>{peer_fetch}</strong></span>
</div>"#,
        pull_bytes = escape_html(&pull_bytes.to_string()),
        backpressure = escape_html(&backpressure.to_string()),
        peer_fetch = escape_html(&peer_fetch.to_string()),
    )
}

/// Locality / hot-tier metrics strip for admin updates panel (PH-S762).
pub fn render_grid_locality_metrics_strip_html(
    locality_metrics_json: &str,
    hot_promote_gauge: u64,
) -> String {
    use crate::format::escape_html;

    let body: Value = serde_json::from_str(locality_metrics_json).unwrap_or(Value::Null);
    let metrics = body.get("metrics").cloned().unwrap_or(body);
    let shard_bps = grid_metrics_u64(&metrics, "shard_local_hit_ratio_bps");
    let hot_bps = grid_metrics_u64(&metrics, "hot_tier_hit_ratio_bps");
    let promote = if grid_metrics_u64(&metrics, "hot_promote_total") > 0 {
        grid_metrics_u64(&metrics, "hot_promote_total")
    } else {
        hot_promote_gauge
    };
    let evict = grid_metrics_u64(&metrics, "hot_evict_total");

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>Shard local hit: <strong>{shard_bps}</strong> bps</span>
<span>Hot tier hit: <strong>{hot_bps}</strong> bps</span>
<span>Promote: <strong>{promote}</strong></span>
<span>Evict: <strong>{evict}</strong></span>
</div>"#,
        shard_bps = escape_html(&shard_bps.to_string()),
        hot_bps = escape_html(&hot_bps.to_string()),
        promote = escape_html(&promote.to_string()),
        evict = escape_html(&evict.to_string()),
    )
}

/// Primary/secondary fee hint + applied counter strip for grid-pricing admin (PH-S781).
pub fn render_grid_fee_split_metrics_strip_html(
    fee_split_metrics_json: &str,
    applied_gauge: u64,
) -> String {
    use crate::format::escape_html;

    let body: Value = serde_json::from_str(fee_split_metrics_json).unwrap_or(Value::Null);
    let metrics = body.get("metrics").cloned().unwrap_or(body);
    let applied = if grid_metrics_u64(&metrics, "fee_split_applied_total") > 0 {
        grid_metrics_u64(&metrics, "fee_split_applied_total")
    } else {
        applied_gauge
    };
    let primary_bps = grid_metrics_u64(&metrics, "primary_dev_fee_bps");
    let secondary_min = grid_metrics_u64(&metrics, "secondary_admin_fee_min_bps");
    let secondary_max = grid_metrics_u64(&metrics, "secondary_admin_fee_max_bps");
    let hint = crate::pricing::SECONDARY_FEE_UX_HINT;

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>Primary dev: <strong>{primary_bps}</strong> bps (0.1%)</span>
<span>Secondary admin: <strong>{secondary_min}</strong>–<strong>{secondary_max}</strong> bps</span>
<span>Applied: <strong>{applied}</strong></span>
<p class="muted admin-hint">{hint}</p>
</div>"#,
        primary_bps = escape_html(&primary_bps.to_string()),
        secondary_min = escape_html(&secondary_min.to_string()),
        secondary_max = escape_html(&secondary_max.to_string()),
        applied = escape_html(&applied.to_string()),
        hint = escape_html(hint),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_grid_metrics_json_shape_ph_s712() {
        let body = json!({
            "ok": true,
            "metrics": { "checker_pending_total": 2, "sample_total": 5 }
        });
        validate_grid_metrics_json_shape(&body, &["checker_pending_total", "sample_total"])
            .expect("shape");
    }

    #[test]
    fn reconcile_prometheus_json_gauge_ph_s712() {
        let prom = "galaxy_verification_checker_pending_total 3\n";
        let body = json!({
            "ok": true,
            "metrics": { "checker_pending_total": 3 }
        });
        reconcile_prometheus_json_gauge(
            prom,
            &body,
            "galaxy_verification_checker_pending_total",
            "checker_pending_total",
        )
        .expect("parity");
    }

    #[test]
    fn render_grid_verification_metrics_strip_ph_s712() {
        let json = r#"{"ok":true,"metrics":{"sample_total":4,"checker_pending_total":2}}"#;
        let html = render_grid_verification_metrics_strip_html(json, 0);
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("Sample"));
        assert!(html.contains("Pending"));
    }

    #[test]
    fn render_grid_settlement_trust_metrics_strip_ph_s722() {
        let settlement = r#"{"ok":true,"metrics":{"cleared_total":5}}"#;
        let trust = r#"{"ok":true,"metrics":{"payout_eligible_total":2,"last_trust_score":60}}"#;
        let html = render_grid_settlement_trust_metrics_strip_html(settlement, trust, 0);
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("Cleared"));
        assert!(html.contains("Eligible"));
        assert!(html.contains("Trust score"));
    }

    #[test]
    fn render_grid_prefetch_metrics_strip_ph_s752() {
        let json = r#"{"ok":true,"metrics":{"pull_bytes_total":4194304,"backpressure_total":1,"peer_fetch_total":2}}"#;
        let html = render_grid_prefetch_metrics_strip_html(json, 0);
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("Pull bytes"));
        assert!(html.contains("Backpressure"));
        assert!(html.contains("Peer fetch"));
    }

    #[test]
    fn render_grid_locality_metrics_strip_ph_s762() {
        let json = r#"{"ok":true,"metrics":{"shard_local_hit_ratio_bps":8000,"hot_tier_hit_ratio_bps":5000,"hot_promote_total":2,"hot_evict_total":1}}"#;
        let html = render_grid_locality_metrics_strip_html(json, 0);
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("Shard local hit"));
        assert!(html.contains("Hot tier hit"));
        assert!(html.contains("Promote"));
        assert!(html.contains("Evict"));
    }

    #[test]
    fn render_grid_fee_split_metrics_strip_ph_s781() {
        let json = r#"{"ok":true,"metrics":{"fee_split_applied_total":3,"primary_dev_fee_bps":10,"secondary_admin_fee_min_bps":100,"secondary_admin_fee_max_bps":500}}"#;
        let html = render_grid_fee_split_metrics_strip_html(json, 0);
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("Primary dev"));
        assert!(html.contains("Secondary admin"));
        assert!(html.contains("Applied"));
        assert!(html.contains(crate::pricing::SECONDARY_FEE_UX_HINT));
    }
}
