//! Grid replication/pricing metrics admin panel HTML (PH-S700).

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

/// Admin wasm slim depth classification from panel feature flags (PH-S704).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminWasmSlimDepth {
    None,
    PanelRenderer,
    ChartsGlue,
    /// ML pipeline metrics panel wasm renderer (PH-S804).
    MlPipelinePanel,
    /// Payout batch admin panel wasm renderer (PH-S804).
    PayoutBatchPanel,
    /// Secret rotation admin panel wasm renderer (PH-S814).
    SecurityRotationPanel,
    /// Topology stats strip wasm renderer (PH-S814).
    TopologyStatsStrip,
    /// VM instances admin panel wasm renderer (PH-S824).
    VmPanel,
    /// Workers admin panel wasm renderer (PH-S824).
    WorkersPanel,
    /// Libraries admin panel wasm renderer (PH-S824).
    LibsPanel,
}

/// Classify admin wasm slim depth from optional feature stub (PH-S704).
pub fn admin_wasm_slim_depth_stub(features: Option<&Value>) -> AdminWasmSlimDepth {
    let Some(f) = features else {
        return AdminWasmSlimDepth::None;
    };
    if f.get("libs_panel")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::LibsPanel;
    }
    if f.get("workers_panel")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::WorkersPanel;
    }
    if f.get("vm_panel").and_then(|v| v.as_bool()).unwrap_or(false) {
        return AdminWasmSlimDepth::VmPanel;
    }
    if f.get("charts_glue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::ChartsGlue;
    }
    if f.get("topology_stats_strip")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::TopologyStatsStrip;
    }
    if f.get("security_rotation_panel")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::SecurityRotationPanel;
    }
    if f.get("payout_batch_panel")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::PayoutBatchPanel;
    }
    if f.get("ml_pipeline_panel")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::MlPipelinePanel;
    }
    if f.get("panel_renderer")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return AdminWasmSlimDepth::PanelRenderer;
    }
    AdminWasmSlimDepth::None
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
<span>{fresh_lbl}: <strong>{fresh}</strong></span>
<span>{stale_lbl}: <strong>{stale}</strong></span>
</div>"#,
        strict_lbl = escape_html(&t(
            &i18n,
            "admin.gridReplicationPricing.strict",
            "Strict tier"
        )),
        strict = escape_html(&strict.to_string()),
        enqueue_lbl = escape_html(&t(&i18n, "admin.gridReplicationPricing.enqueue", "Enqueue")),
        enqueue = escape_html(&metric_u64(&rm, "enqueue_total").to_string()),
        fresh_lbl = escape_html(&t(
            &i18n,
            "admin.gridReplicationPricing.freshServed",
            "Fresh served"
        )),
        fresh = escape_html(&metric_u64(&pm, "fresh_served_total").to_string()),
        stale_lbl = escape_html(&t(
            &i18n,
            "admin.gridReplicationPricing.staleServed",
            "Stale served"
        )),
        stale = escape_html(&metric_u64(&pm, "stale_served_total").to_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
    fn admin_wasm_slim_depth_stub_ph_s704() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
            AdminWasmSlimDepth::PanelRenderer
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"charts_glue": true}))),
            AdminWasmSlimDepth::ChartsGlue
        );
        assert_eq!(admin_wasm_slim_depth_stub(None), AdminWasmSlimDepth::None);
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s804() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"ml_pipeline_panel": true}))),
            AdminWasmSlimDepth::MlPipelinePanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"payout_batch_panel": true}))),
            AdminWasmSlimDepth::PayoutBatchPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(
                &json!({"charts_glue": true, "ml_pipeline_panel": true})
            )),
            AdminWasmSlimDepth::ChartsGlue
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s814() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"security_rotation_panel": true}))),
            AdminWasmSlimDepth::SecurityRotationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"topology_stats_strip": true}))),
            AdminWasmSlimDepth::TopologyStatsStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s824() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"workers_panel": true}))),
            AdminWasmSlimDepth::WorkersPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"libs_panel": true}))),
            AdminWasmSlimDepth::LibsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true, "workers_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
    }
}
