//! Admin payout batch read panel (PH-S553, PH-S564 wasm renderer).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Payout batch snapshot page (`/ui/admin/payout-batch`).
pub async fn admin_payout_batch() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function payoutBatchI18nJson() {
      return JSON.stringify(window.__poolaiAdminI18nRust || {});
    }

    function renderPayoutBatchPanel(latest, history, settlementMetrics, trustMetrics, trustScoreGauge) {
      const el = document.getElementById('payout-batch-panel');
      if (!el) return;
      const entry = latest.entry || null;
      const metricsStrip = renderPayoutBatchMetricsStrip(settlementMetrics, trustMetrics, trustScoreGauge);
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.renderPayoutBatchPanelHtml === 'function') {
        el.innerHTML = metricsStrip + wasm.renderPayoutBatchPanelHtml(
          JSON.stringify(latest || {}),
          JSON.stringify(history || {}),
          payoutBatchI18nJson()
        );
        return;
      }
      el.innerHTML = metricsStrip +
        '<div class="admin-card"><h3>' + escapeHtml(T('admin.payoutBatch.latest', 'Latest cleared entry')) + '</h3>' +
        '<dl class="admin-dl"><dt>Job</dt><dd><code>' + escapeHtml(String(entry && entry.job_id || '—')) + '</code></dd></dl></div>';
    }

    async function loadPayoutBatchPanel() {
      adminShowLoading('payout-batch-panel', T('admin.payoutBatch.loading', 'Loading payout batch…'));
      try {
        const [latest, history, settlementMetrics, trustMetrics] = await Promise.all([
          fetchJson('/api/v1/grid/payout-batch'),
          fetchJson('/api/v1/grid/payout-batch/history?limit=5'),
          fetchJson('/api/v1/grid/settlement-metrics'),
          fetchJson('/api/v1/grid/trust-metrics'),
        ]);
        let trustScoreGauge = 0;
        try {
          const metrics = await fetch('/metrics').then(function(r) { return r.text(); });
          var wasm = poolaiChartsWasm();
          if (wasm && typeof wasm.parsePrometheusGauge === 'function') {
            trustScoreGauge = wasm.parsePrometheusGauge(metrics, 'galaxy_trust_score');
          }
        } catch (_) {}
        renderPayoutBatchPanel(latest || {}, history || {}, settlementMetrics || {}, trustMetrics || {}, trustScoreGauge);
      } catch (e) {
        adminShowInlineError('payout-batch-panel', e);
        showNotification(T('admin.payoutBatch.errLoad', 'Error loading payout batch: ') + e.message, 'error');
      }
    }

    function renderPayoutBatchMetricsStrip(settlementMetrics, trustMetrics, trustScoreGauge) {
      const sm = (settlementMetrics && settlementMetrics.metrics) ? settlementMetrics.metrics : {};
      const tm = (trustMetrics && trustMetrics.metrics) ? trustMetrics.metrics : {};
      const score = tm.last_trust_score != null ? tm.last_trust_score : trustScoreGauge;
      return '<div class="admin-card admin-metrics-strip">' +
        '<span>' + escapeHtml(T('admin.payoutBatch.cleared', 'Cleared')) + ': <strong>' + escapeHtml(String(sm.cleared_total || 0)) + '</strong></span>' +
        '<span>' + escapeHtml(T('admin.payoutBatch.eligible', 'Eligible')) + ': <strong>' + escapeHtml(String(tm.payout_eligible_total || 0)) + '</strong></span>' +
        '<span>' + escapeHtml(T('admin.payoutBatch.trustScore', 'Trust score')) + ': <strong>' + escapeHtml(String(score || 0)) + '</strong></span>' +
        '</div>';
    }

    loadPayoutBatchPanel();
    setInterval(loadPayoutBatchPanel, 15000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.payoutBatch",
        "Payout batch",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.payoutBatch.section">Payout batch</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadPayoutBatchPanel()" data-i18n="admin.payoutBatch.refresh">Refresh</button>
            </div>
          </div>
          <div id="payout-batch-panel" class="admin-panel-body" aria-live="polite"></div>
        </div>
        "#,
        script,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn admin_payout_batch_includes_wasm_renderer_ph_s564() {
        let html = admin_payout_batch().await.0;
        assert!(html.contains("/ui/wasm/poolai_ui_wasm.js"));
        assert!(html.contains("renderPayoutBatchPanelHtml"));
        assert!(html.contains("payout-batch-panel"));
        assert!(html.contains("/api/v1/grid/settlement-metrics"));
        assert!(html.contains("/api/v1/grid/trust-metrics"));
        assert!(html.contains("parsePrometheusGauge"));
    }
}
