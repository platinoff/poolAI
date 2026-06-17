//! Galaxy Grid pricing snapshot admin page (PH-S82) — read-only `GET /api/v1/grid/pricing`.
//! PH-S151/PH-S152: USD/time formatters via shared `poolai-ui-wasm` bootstrap; thin JS fallback otherwise.
//! PH-S154: EN/UK i18n subset in `poolai-ui-core::i18n` (injected via admin layout).
//! PH-S217: grid-pricing page uses slim `admin_layout_grid_pricing` + `admin_grid_pricing_patch`.

use crate::ui::admin::admin_layout_grid_pricing;
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

    function formatUsdMicroFallback(usdMicro) {
      const n = Number(usdMicro);
      if (!Number.isFinite(n)) return '—';
      return (n / 1000000).toFixed(6) + ' USD';
    }

    function formatUnixSecsFallback(secs) {
      const n = Number(secs);
      if (!Number.isFinite(n) || n <= 0) return '—';
      try {
        return new Date(n * 1000).toISOString();
      } catch (_) {
        return String(secs);
      }
    }

    function formatUsdMicro(usdMicro) {
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.formatUsdMicro === 'function') {
        return wasm.formatUsdMicro(Number(usdMicro));
      }
      return formatUsdMicroFallback(usdMicro);
    }

    function formatUnixSecs(secs) {
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.formatUnixSecs === 'function') {
        return wasm.formatUnixSecs(Number(secs));
      }
      return formatUnixSecsFallback(secs);
    }

    function renderGridPricingSnapshot(data) {
      const el = document.getElementById('grid-pricing-panel');
      if (!el) return;
      const snap = data.snapshot || {};
      const source = String(data.source || '—');
      const freshness = String(data.freshness || '—');
      const task = String(snap.task_profile || '—');
      const model = String(snap.model_profile || '—');
      const unit = String(snap.unit_key || '—');
      const usdMicro = snap.usd_micro;
      const updatedAt = snap.updated_at;
      el.innerHTML =
        '<div id="grid-pricing-result" class="admin-card">' +
        '<h3>' + escapeHtml(T('admin.gridPricing.result', 'Pricing snapshot')) + '</h3>' +
        '<dl class="admin-dl">' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.task', 'Task profile')) + '</dt>' +
        '<dd><code>' + escapeHtml(task) + '</code></dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.model', 'Model profile')) + '</dt>' +
        '<dd><code>' + escapeHtml(model) + '</code></dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.unit', 'Unit key')) + '</dt>' +
        '<dd><code>' + escapeHtml(unit) + '</code></dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.price', 'Price (USD)')) + '</dt>' +
        '<dd>' + escapeHtml(formatUsdMicro(usdMicro)) + '</dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.updated', 'Updated at')) + '</dt>' +
        '<dd>' + escapeHtml(formatUnixSecs(updatedAt)) + '</dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.source', 'Source')) + '</dt>' +
        '<dd>' + escapeHtml(source) + '</dd>' +
        '<dt>' + escapeHtml(T('admin.gridPricing.col.freshness', 'Freshness')) + '</dt>' +
        '<dd>' + escapeHtml(freshness) + '</dd>' +
        '</dl></div>';
    }

    async function loadGridPricingSnapshot() {
      adminShowLoading('grid-pricing-panel', T('admin.gridPricing.loading', 'Loading pricing…'));
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

    function startGridPricingPage() {
      loadGridPricingSnapshot();
    }

    if (window.poolaiUiWasm && (window.poolaiUiWasm.ready || window.poolaiUiWasm.failed)) {
      startGridPricingPage();
    } else {
      window.addEventListener('poolai-ui-wasm-ready', startGridPricingPage, { once: true });
      setTimeout(startGridPricingPage, 2500);
    }
    "#;

    admin_layout_grid_pricing(
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
async fn admin_grid_pricing_page_slim_grid_pricing_i18n_patch_ph_s217() {
    let html = admin_grid_pricing().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.gridPricing""#));
    assert!(html.contains(r#""admin.gridPricing.section""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.mon.mlTitle""#));
}

#[tokio::test]
async fn admin_grid_pricing_page_includes_rust_i18n_patch_ph_s154() {
    let html = admin_grid_pricing().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.gridPricing.section""#));
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

#[tokio::test]
async fn admin_grid_pricing_page_wires_poolai_ui_wasm_module() {
    let html = admin_grid_pricing().await.0;
    assert!(html.contains("type=\"module\""));
    assert!(html.contains("/ui/wasm/poolai_ui_wasm.js"));
    assert!(html.contains("window.poolaiUiWasm"));
    assert!(html.contains("poolai-ui-wasm-ready"));
    assert!(html.contains("formatUsdMicroFallback"));
}
