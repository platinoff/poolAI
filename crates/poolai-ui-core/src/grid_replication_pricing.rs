//! Grid replication/pricing metrics admin panel HTML (PH-S700).

pub use crate::admin_wasm_slim_depth::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};

use crate::format::escape_html;
use serde_json::Value;

fn t(i18n: &Value, key: &str, fallback: &str) -> String {
    i18n.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

fn metric_u64(metrics: &Value, key: &str) -> u64 {
    metrics.get(key).and_then(|v| v.as_u64()).unwrap_or(0)
}

/// Replication/pricing metrics strip for admin panel (PH-S700).
pub fn render_grid_replication_pricing_panel_html(
    replication_metrics_json: &str,
    pricing_metrics_json: &str,
    strict_gauge: u64,
    i18n_json: &str,
) -> String {
    let replication: Value = serde_json::from_str(replication_metrics_json).unwrap_or(Value::Null);
    let pricing: Value = serde_json::from_str(pricing_metrics_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);

    let rm = replication.get("metrics").cloned().unwrap_or(replication);
    let pm = pricing.get("metrics").cloned().unwrap_or(pricing);

    let strict = rm
        .get("strict_total")
        .and_then(|v| v.as_u64())
        .unwrap_or(strict_gauge);

    format!(
        r#"<div class="admin-card admin-metrics-strip">
<span>{strict_lbl}: <strong>{strict}</strong></span>
<span>{enqueue_lbl}: <strong>{enqueue}</strong></span>
<span>{rate_limited_lbl}: <strong>{rate_limited}</strong></span>
<span>{fresh_lbl}: <strong>{fresh}</strong></span>
<span>{stale_lbl}: <strong>{stale}</strong></span>
</div>"#,
        strict_lbl = escape_html(t(
            &i18n,
            "admin.gridReplicationPricing.strict",
            "Strict tier"
        )),
        strict = escape_html(strict.to_string()),
        enqueue_lbl = escape_html(t(&i18n, "admin.gridReplicationPricing.enqueue", "Enqueue")),
        enqueue = escape_html(metric_u64(&rm, "enqueue_total").to_string()),
        rate_limited_lbl = escape_html(t(
            &i18n,
            "admin.gridReplicationPricing.rateLimited",
            "Rate limited"
        )),
        rate_limited = escape_html(metric_u64(&rm, "rate_limited_total").to_string()),
        fresh_lbl = escape_html(t(
            &i18n,
            "admin.gridReplicationPricing.freshServed",
            "Fresh served"
        )),
        fresh = escape_html(metric_u64(&pm, "fresh_served_total").to_string()),
        stale_lbl = escape_html(t(
            &i18n,
            "admin.gridReplicationPricing.staleServed",
            "Stale served"
        )),
        stale = escape_html(metric_u64(&pm, "stale_served_total").to_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_grid_replication_pricing_panel_ph_s700() {
        let html = render_grid_replication_pricing_panel_html(
            r#"{"metrics":{"strict_total":2,"enqueue_total":5}}"#,
            r#"{"metrics":{"fresh_served_total":3,"stale_served_total":1}}"#,
            0,
            r#"{"admin.gridReplicationPricing.strict":"Strict"}"#,
        );
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("<strong>2</strong>"));
        assert!(html.contains("<strong>5</strong>"));
        assert!(html.contains("<strong>3</strong>"));
        assert!(html.contains("<strong>1</strong>"));
    }

    #[test]
    fn render_grid_replication_pricing_panel_rate_limited_ph_s892() {
        let html = render_grid_replication_pricing_panel_html(
            r#"{"metrics":{"strict_total":1,"enqueue_total":2,"rate_limited_total":3}}"#,
            r#"{"metrics":{"fresh_served_total":0,"stale_served_total":0}}"#,
            0,
            r#"{"admin.gridReplicationPricing.rateLimited":"Rate limited"}"#,
        );
        assert!(html.contains("<strong>3</strong>"));
    }
}
