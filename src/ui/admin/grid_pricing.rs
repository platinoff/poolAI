//! Galaxy Grid pricing snapshot admin page (PH-S82) — read-only `GET /api/v1/grid/pricing`.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Grid pricing snapshot page (`/ui/admin/grid-pricing`).
pub async fn admin_grid_pricing() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function gridPricingQueryParams() {
      const task = document.getElementById('grid-pricing-task')?.value?.trim() || '';
      const model = document.getElementById('grid-pricing-model')?.value?.trim() || '';
      const unit = document.getElementById('grid-pricing-unit')?.value || 'inference_blended_token';
      return { task, model, unit };
    }

    function gridPricingApiUrl() {
      const p = gridPricingQueryParams();
      const qs = new URLSearchParams({
        task_profile: p.task,
        model_profile: p.model,
        unit_key: p.unit,
      });
      return '/api/v1/grid/pricing?' + qs.toString();
    }

    function formatUsdMicro(usdMicro) {
      const n = Number(usdMicro);
      if (!Number.isFinite(n)) return '—';
      return (n / 1000000).toFixed(6) + ' USD';
    }

    function formatUnixSecs(secs) {
      const n = Number(secs);
      if (!Number.isFinite(n) || n <= 0) return '—';
      try {
        return new Date(n * 1000).toISOString();
      } catch (_) {
        return String(secs);
      }
    }

    function renderGridPricingSnapshot(data) {
      const el = document.getElementById('grid-pricing-panel');
      if (!el) return;
      const snap = data.snapshot || {};
      const source = String(data.source || '—');
      const freshness = String(data.freshness || '—');
      el.innerHTML =
        '<div class="admin-card" id="grid-pricing-result">' +
        '<h3>' + escapeHtml(T('admin.gridPricing.resultTitle', 'Pricing snapshot')) + '</h3>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.source', 'Source')) + '</span>' +
        '<span class="stat-value"><code>' + escapeHtml(source) + '</code></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.freshness', 'Freshness')) + '</span>' +
        '<span class="stat-value"><code>' + escapeHtml(freshness) + '</code></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.unitKey', 'Unit key')) + '</span>' +
        '<span class="stat-value"><code>' + escapeHtml(String(snap.unit_key || '')) + '</code></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.marketMin', 'Market min (μUSD)')) + '</span>' +
        '<span class="stat-value">' + escapeHtml(String(snap.market_min_usd_micro ?? '—')) +
        ' <span class="muted">(' + escapeHtml(formatUsdMicro(snap.market_min_usd_micro)) + ')</span></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.poolaiQuote', 'PoolAI quote (μUSD)')) + '</span>' +
        '<span class="stat-value">' + escapeHtml(String(snap.poolai_quote_usd_micro ?? '—')) +
        ' <span class="muted">(' + escapeHtml(formatUsdMicro(snap.poolai_quote_usd_micro)) + ')</span></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.provider', 'Provider at min')) + '</span>' +
        '<span class="stat-value"><code>' + escapeHtml(String(snap.provider_id_at_min || '—')) + '</code></span></div>' +
        '<div class="stat-item"><span class="stat-label">' +
        escapeHtml(T('admin.gridPricing.col.cachedAt', 'Cached at')) + '</span>' +
        '<span class="stat-value">' + escapeHtml(formatUnixSecs(snap.cached_at_secs)) + '</span></div>' +
        '</div>';
    }

    async function loadGridPricingSnapshot() {
      const panel = document.getElementById('grid-pricing-panel');
      if (!panel) return;
      const p = gridPricingQueryParams();
      if (!p.task || !p.model) {
        adminShowInlineError('grid-pricing-panel', T('admin.gridPricing.errParams', 'Task and model profile are required.'));
        return;
      }
      adminShowLoading('grid-pricing-panel', T('admin.gridPricing.loading', 'Loading pricing snapshot…'));
      try {
        const data = await fetchJson(gridPricingApiUrl());
        renderGridPricingSnapshot(data);
      } catch (e) {
        adminShowInlineError('grid-pricing-panel', e);
        showNotification(T('admin.gridPricing.errLoad', 'Error loading pricing: ') + e.message, 'error');
      }
    }

    document.getElementById('grid-pricing-form')?.addEventListener('submit', function(ev) {
      ev.preventDefault();
      loadGridPricingSnapshot();
    });

    loadGridPricingSnapshot();
    "#;

    admin_layout(
        "admin.page.gridPricing",
        "Grid pricing",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.gridPricing.section">Grid pricing</h2>
            <div class="admin-header-actions">
              <button type="submit" form="grid-pricing-form" id="grid-pricing-fetch-btn" class="btn btn-primary" data-i18n="admin.gridPricing.fetch">Fetch snapshot</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.gridPricing.hint">
            Read-only Galaxy pricing oracle snapshot. Configure L2 fallback via <code>POOLAI_GALAXY_PRICING_FALLBACK_JSON</code> when providers are unavailable.
          </p>
          <form id="grid-pricing-form" class="admin-form admin-form--inline" autocomplete="off">
            <label for="grid-pricing-task">
              <span data-i18n="admin.gridPricing.taskProfile">Task profile</span>
              <input type="text" id="grid-pricing-task" name="task_profile" value="inference:text" required />
            </label>
            <label for="grid-pricing-model">
              <span data-i18n="admin.gridPricing.modelProfile">Model profile</span>
              <input type="text" id="grid-pricing-model" name="model_profile" value="default" required />
            </label>
            <label for="grid-pricing-unit">
              <span data-i18n="admin.gridPricing.unitKey">Unit key</span>
              <select id="grid-pricing-unit" name="unit_key">
                <option value="inference_blended_token">inference_blended_token</option>
                <option value="inference_input_token">inference_input_token</option>
                <option value="inference_output_token">inference_output_token</option>
                <option value="gpu_second">gpu_second</option>
                <option value="job_flat">job_flat</option>
              </select>
            </label>
          </form>
          <div id="grid-pricing-panel" class="grid-pricing-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_grid_pricing_page_includes_form_and_api() {
    let html = admin_grid_pricing().await.0;
    assert!(html.contains("id=\"grid-pricing-panel\""));
    assert!(html.contains("id=\"grid-pricing-form\""));
    assert!(html.contains("id=\"grid-pricing-fetch-btn\""));
    assert!(html.contains("/api/v1/grid/pricing"));
    assert!(html.contains("loadGridPricingSnapshot"));
    assert!(html.contains("inference_blended_token"));
}
