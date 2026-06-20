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
}
