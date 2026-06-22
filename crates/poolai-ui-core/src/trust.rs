//! Grid trust metrics / persist strip formatters (PH-S912).

use crate::format::escape_html;
use serde_json::Value;

fn grid_metrics_u64(metrics: &Value, key: &str) -> u64 {
    metrics.get(key).and_then(|v| v.as_u64()).unwrap_or(0)
}

/// Trust persist + gate counters strip for admin payout panel (PH-S912).
pub fn render_grid_trust_persist_strip_html(trust_metrics_json: &str, i18n_json: &str) -> String {
    let body: Value = serde_json::from_str(trust_metrics_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);
    let tm = body.get("metrics").cloned().unwrap_or_else(|| body.clone());
    let depth = body
        .get("trust_persist_depth")
        .and_then(|v| v.as_str())
        .unwrap_or("—");
    let backend = body
        .get("trust_store_backend")
        .and_then(|v| v.as_str())
        .unwrap_or("—");
    let peer_count = body
        .get("persisted_peer_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let held = grid_metrics_u64(&tm, "payout_held_total");
    let eligible = grid_metrics_u64(&tm, "payout_eligible_total");
    let score = grid_metrics_u64(&tm, "last_trust_score");

    let depth_lbl = i18n
        .get("admin.trust.col.persistDepth")
        .and_then(|v| v.as_str())
        .unwrap_or("Persist depth");
    let backend_lbl = i18n
        .get("admin.trust.col.storeBackend")
        .and_then(|v| v.as_str())
        .unwrap_or("Store");
    let peers_lbl = i18n
        .get("admin.trust.col.persistedPeers")
        .and_then(|v| v.as_str())
        .unwrap_or("Persisted peers");

    format!(
        r#"<div class="admin-card admin-metrics-strip grid-trust-persist-strip">
<span>{depth_lbl}: <strong>{depth}</strong></span>
<span>{backend_lbl}: <strong>{backend}</strong></span>
<span>{peers_lbl}: <strong>{peer_count}</strong></span>
<span>Held: <strong>{held}</strong></span>
<span>Eligible: <strong>{eligible}</strong></span>
<span>Trust score: <strong>{score}</strong></span>
</div>"#,
        depth_lbl = escape_html(depth_lbl),
        depth = escape_html(depth),
        backend_lbl = escape_html(backend_lbl),
        backend = escape_html(backend),
        peers_lbl = escape_html(peers_lbl),
        peer_count = peer_count,
        held = held,
        eligible = eligible,
        score = score,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_grid_trust_persist_strip_ph_s912() {
        let json = r#"{"ok":true,"trust_persist_depth":"sqlite_restart","trust_store_backend":"sqlite","persisted_peer_count":2,"metrics":{"payout_held_total":1,"payout_eligible_total":3,"last_trust_score":55}}"#;
        let html = render_grid_trust_persist_strip_html(json, "{}");
        assert!(html.contains("grid-trust-persist-strip"));
        assert!(html.contains("sqlite_restart"));
        assert!(html.contains("Held"));
    }
}
