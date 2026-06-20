//! Galaxy Grid replication/pricing metrics admin page (PH-S692).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Replication/pricing metrics page (`/ui/admin/grid-replication-pricing`).
pub async fn admin_grid_replication_pricing() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function renderReplicationPricingPanel(replicationMetrics, pricingMetrics, strictGauge) {
      const el = document.getElementById('grid-replication-pricing-panel');
      if (!el) return;
      const rm = (replicationMetrics && replicationMetrics.metrics) ? replicationMetrics.metrics : {};
      const pm = (pricingMetrics && pricingMetrics.metrics) ? pricingMetrics.metrics : {};
      const strict = rm.strict_total != null ? rm.strict_total : strictGauge;
      el.innerHTML =
        '<div class="admin-card admin-metrics-strip">' +
        '<span>' + escapeHtml(T('admin.gridReplicationPricing.strict', 'Strict tier')) +
        ': <strong>' + escapeHtml(String(strict || 0)) + '</strong></span>' +
        '<span>' + escapeHtml(T('admin.gridReplicationPricing.enqueue', 'Enqueue')) +
        ': <strong>' + escapeHtml(String(rm.enqueue_total || 0)) + '</strong></span>' +
        '<span>' + escapeHtml(T('admin.gridReplicationPricing.freshServed', 'Fresh served')) +
        ': <strong>' + escapeHtml(String(pm.fresh_served_total || 0)) + '</strong></span>' +
        '<span>' + escapeHtml(T('admin.gridReplicationPricing.staleServed', 'Stale served')) +
        ': <strong>' + escapeHtml(String(pm.stale_served_total || 0)) + '</strong></span>' +
        '</div>';
    }

    async function loadGridReplicationPricingPanel() {
      adminShowLoading('grid-replication-pricing-panel', T('admin.gridReplicationPricing.loading', 'Loading metrics…'));
      try {
        const [replicationMetrics, pricingMetrics] = await Promise.all([
          fetchJson('/api/v1/grid/replication-metrics'),
          fetchJson('/api/v1/grid/pricing-metrics'),
        ]);
        let strictGauge = 0;
        try {
          const metrics = await fetch('/metrics').then(function(r) { return r.text(); });
          var wasm = poolaiChartsWasm();
          if (wasm && typeof wasm.parsePrometheusGauge === 'function') {
            strictGauge = wasm.parsePrometheusGauge(metrics, 'galaxy_replication_strict_total');
          }
        } catch (_) {}
        renderReplicationPricingPanel(replicationMetrics, pricingMetrics, strictGauge);
      } catch (e) {
        adminShowInlineError('grid-replication-pricing-panel', e);
        showNotification(T('admin.gridReplicationPricing.errLoad', 'Error loading metrics: ') + e.message, 'error');
      }
    }

    loadGridReplicationPricingPanel();
    setInterval(loadGridReplicationPricingPanel, 15000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.gridReplicationPricing",
        "Grid replication/pricing",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.gridReplicationPricing.section">Replication &amp; pricing metrics</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadGridReplicationPricingPanel()" data-i18n="admin.gridReplicationPricing.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.gridReplicationPricing.hint">
            Read-only Galaxy replication tier and pricing oracle counters (PH-S690/S691).
          </p>
          <div id="grid-replication-pricing-panel" class="admin-panel-body" aria-live="polite"></div>
        </div>
        "#,
        script,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn admin_grid_replication_pricing_wasm_glue_ph_s692() {
        let html = admin_grid_replication_pricing().await.0;
        assert!(html.contains("/api/v1/grid/replication-metrics"));
        assert!(html.contains("/api/v1/grid/pricing-metrics"));
        assert!(html.contains("grid-replication-pricing-panel"));
        assert!(html.contains("parsePrometheusGauge"));
        assert!(html.contains("poolaiChartsWasm"));
    }
}
