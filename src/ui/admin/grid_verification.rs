//! Galaxy Grid verification checker admin page (PH-S512).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Verification checker tasks page (`/ui/admin/grid-verification`).
pub async fn admin_grid_verification() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    async function loadGridVerificationTasks() {
      adminShowLoading('grid-verification-panel', T('admin.gridVerification.loading', 'Loading checker tasks…'));
      try {
        const data = await fetchJson('/api/v1/grid/verification-checker/tasks');
        const tasks = (data && data.tasks) ? data.tasks : [];
        let pending = 0;
        let metricsJson = '{}';
        try {
          const metricsResp = await fetchJson('/api/v1/grid/verification-metrics');
          metricsJson = JSON.stringify(metricsResp || {});
          if (metricsResp && metricsResp.metrics && metricsResp.metrics.checker_pending_total != null) {
            pending = metricsResp.metrics.checker_pending_total;
          }
        } catch (_) {}
        try {
          const metrics = await fetch('/metrics').then(function(r) { return r.text(); });
          var wasm = poolaiChartsWasm();
          if (wasm && typeof wasm.parsePrometheusGauge === 'function') {
            var promPending = wasm.parsePrometheusGauge(metrics, 'galaxy_verification_checker_pending_total');
            if (promPending > 0) pending = promPending;
          } else {
            var m = metrics.match(/galaxy_verification_checker_pending_total\s+(\d+)/);
            if (m) pending = parseInt(m[1], 10) || pending;
          }
        } catch (_) {}
        renderGridVerificationPanel(tasks, pending, metricsJson);
      } catch (e) {
        adminShowInlineError('grid-verification-panel', e);
        showNotification(T('admin.gridVerification.errLoad', 'Error loading checker tasks: ') + e.message, 'error');
      }
    }

    function renderGridVerificationPanel(tasks, pendingTotal, metricsJson) {
      const el = document.getElementById('grid-verification-panel');
      if (!el) return;
      var wasm = poolaiChartsWasm();
      var strip = '';
      if (wasm && typeof wasm.renderGridVerificationMetricsStrip === 'function') {
        strip = wasm.renderGridVerificationMetricsStrip(metricsJson || '{}', pendingTotal || 0);
      }
      el.innerHTML = strip + poolaiRenderGridVerificationPanel(tasks, pendingTotal, {
        job: T('admin.gridVerification.colJob', 'Job ID'),
        type: T('admin.gridVerification.colType', 'Task type'),
        pending: T('admin.gridVerification.pending', 'Pending total'),
        tableAria: T('admin.gridVerification.section', 'Verification checker'),
        empty: T('admin.gridVerification.empty', 'No pending checker tasks'),
      });
    }

    loadGridVerificationTasks();
    setInterval(loadGridVerificationTasks, 10000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.gridVerification",
        "Grid verification",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.gridVerification.section">Verification checker</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadGridVerificationTasks()" data-i18n="admin.gridVerification.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.gridVerification.hint">
            Read-only view of pending Galaxy verification checker tasks (PH-S494).
          </p>
          <div id="grid-verification-panel" class="grid-verification-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_grid_verification_page_api_ph_s512() {
    let html = admin_grid_verification().await.0;
    assert!(html.contains("/api/v1/grid/verification-checker/tasks"));
    assert!(html.contains("/api/v1/grid/verification-metrics"));
    assert!(html.contains("grid-verification-panel"));
    assert!(html.contains("poolaiRenderGridVerificationPanel"));
    assert!(html.contains("parsePrometheusGauge"));
    assert!(html.contains("renderGridVerificationMetricsStrip"));
}

#[tokio::test]
async fn admin_grid_verification_wasm_glue_ph_s712() {
    let html = admin_grid_verification().await.0;
    assert!(html.contains("verification-metrics"));
    assert!(html.contains("renderGridVerificationMetricsStrip"));
}
