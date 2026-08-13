//! Admin Dashboard page
//!
//! Provides system overview with real-time status, metrics, alerts, and recent activity.

use crate::ui::admin::admin_layout_dashboard;
use axum::response::Html;

/// Admin dashboard home page
pub async fn admin_dashboard() -> Html<String> {
    let script = r#"
    function T(k, fb) {
      return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb;
    }

    function setDashboardLoading() {
      ['system-overview', 'quick-stats', 'active-alerts', 'recent-activity', 'metrics-chart'].forEach(id => {
        adminShowLoading(id);
      });
    }

    async function loadSystemOverview() {
      setDashboardLoading();
      try {
        const overview = await fetchJson(
          (typeof window !== 'undefined' && window.poolaiUiWasm && typeof window.poolaiUiWasm.buildAdminOverviewUrl === 'function')
            ? window.poolaiUiWasm.buildAdminOverviewUrl()
            : '/api/v1/admin/overview'
        );
        renderSystemOverview(overview);
        renderQuickStats(overview);
        const [alerts, audit] = await Promise.all([
          fetchJson(
            (typeof window !== 'undefined' && window.poolaiUiWasm && typeof window.poolaiUiWasm.buildMonitoringActiveAlertsUrl === 'function')
              ? window.poolaiUiWasm.buildMonitoringActiveAlertsUrl(5)
              : '/api/enterprise/monitoring/alerts?acknowledged=false&limit=5'
          ),
          fetchJson(
            (typeof window !== 'undefined' && window.poolaiUiWasm && typeof window.poolaiUiWasm.buildAuditEventsUrl === 'function')
              ? window.poolaiUiWasm.buildAuditEventsUrl(10)
              : '/api/enterprise/audit/events?limit=10'
          )
        ]);
        renderActiveAlerts(alerts);
        renderRecentActivity(audit);
        updateDashboardRefreshedAt();
      } catch (e) {
        ['system-overview', 'quick-stats', 'active-alerts', 'recent-activity'].forEach(id => adminShowInlineError(id, e));
        adminShowInlineError('metrics-chart', e);
        showNotification(T('admin.dash.errLoad', 'Error loading dashboard: ') + e.message, 'error');
      }
    }
    
    function renderSystemOverview(data) {
      const el = document.getElementById('system-overview');
      if (!el) return;
      el.innerHTML = `
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.label.status', 'Status:')}</span>
          <span class="stat-value status-badge ${data.status === 'healthy' ? 'active' : 'error'}">${data.status || 'unknown'}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.label.uptime', 'Uptime:')}</span>
          <span class="stat-value">${
            (typeof window !== 'undefined' && window.poolaiUiWasm && typeof window.poolaiUiWasm.formatUptime === 'function')
              ? window.poolaiUiWasm.formatUptime(data.uptime_seconds || 0)
              : String(data.uptime_seconds || 0) + 's'
          }</span>
        </div>
      `;
    }
    
    
    function renderQuickStats(data) {
      const el = document.getElementById('quick-stats');
      if (!el) return;
      const wasm = typeof window !== 'undefined' ? window.poolaiUiWasm : null;
      const cpuPct =
        wasm && typeof wasm.formatPercent === 'function'
          ? wasm.formatPercent(data.cpu_usage_percent ?? 0)
          : String((data.cpu_usage_percent ?? 0).toFixed(1)) + '%';
      const memMb =
        wasm && typeof wasm.formatMegabytes === 'function'
          ? wasm.formatMegabytes(data.memory_usage_mb ?? 0)
          : String(Math.round(data.memory_usage_mb ?? 0)) + ' MB';
      el.innerHTML = `
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.quick.workers', 'Workers (active):')}</span>
          <span class="stat-value">${data.workers ?? 0} / ${data.workers_total ?? 0}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.quick.vm', 'VM Instances:')}</span>
          <span class="stat-value">${data.vm_instances ?? 0}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.quick.cpu', 'CPU Usage:')}</span>
          <span class="stat-value">${cpuPct}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.quick.memory', 'Memory (tracked):')}</span>
          <span class="stat-value">${memMb}</span>
        </div>
      `;
    }
    
    async function renderMetricsChart() {
      const endTime = new Date().toISOString();
      const wasm = typeof window !== 'undefined' ? window.poolaiUiWasm : null;
      const url =
        wasm && typeof wasm.buildDashboardMetricsWindowUrl === 'function'
          ? wasm.buildDashboardMetricsWindowUrl(1, 60, endTime)
          : wasm && typeof wasm.buildMetricsWindowUrlWithHours === 'function'
            ? wasm.buildMetricsWindowUrlWithHours(1, 60, endTime)
            : '/api/enterprise/monitoring/metrics?start_time=' +
              encodeURIComponent(new Date(Date.now() - 60 * 60 * 1000).toISOString()) +
              '&end_time=' +
              encodeURIComponent(endTime) +
              '&limit=60';
      const metrics = await fetchJson(url).then((data) => data || []).catch(() => []);
      if (metrics.length === 0) return;
      
      const el = document.getElementById('metrics-chart');
      if (!el) return;
      
      const byMetric = poolaiGroupMetricsByName(metrics);
      const cpuData = byMetric['cpu_usage'] || [];
      const memData = byMetric['memory_usage'] || [];
      const parts = [];
      if (cpuData.length > 0) {
        parts.push(poolaiRenderSparkline(
          T('admin.dash.spark.cpu', 'CPU Usage'),
          poolaiMetricPointValues(cpuData)
        ));
      }
      if (memData.length > 0) {
        parts.push(poolaiRenderSparkline(
          T('admin.dash.spark.memory', 'Memory Usage'),
          poolaiMetricPointValues(memData)
        ));
      }
      if (parts.length > 0) {
        el.innerHTML = '<div class="metrics-sparklines-grid">' + parts.join('') + '</div>';
      }
    }
    
    function renderActiveAlerts(data) {
      const el = document.getElementById('active-alerts');
      if (!el) return;
      if (!data || data.length === 0) {
        el.innerHTML = adminEmptyStateHtml(T('admin.dash.noAlerts', 'No active alerts'));
        return;
      }
      el.innerHTML = data.map(alert => {
        const wasm = typeof window !== 'undefined' ? window.poolaiUiWasm : null;
        const severityClass =
          wasm && typeof wasm.alertSeverityBadgeClass === 'function'
            ? wasm.alertSeverityBadgeClass(alert.severity || '')
            : (alert.severity || 'info').toLowerCase();
        return `
        <div class="alert-item">
          <span class="status-badge ${severityClass}">${alert.severity}</span>
          <span>${alert.metric}: ${alert.current_value}</span>
        </div>
      `;
      }).join('');
    }
    
    function renderRecentActivity(data) {
      const el = document.getElementById('recent-activity');
      if (!el) return;
      if (!data || data.length === 0) {
        el.innerHTML = adminEmptyStateHtml(T('admin.dash.noActivity', 'No recent activity'));
        return;
      }
      el.innerHTML = data.map(event => `
        <div class="activity-item">
          <span class="muted">${escapeHtml(formatAuditTimestamp(event.timestamp))}</span>
          <span>${escapeHtml(event.action || '')}</span>
        </div>
      `).join('');
    }

    function formatAuditTimestamp(raw) {
      return window.poolaiUiWasm.formatIsoDatetime(raw == null ? '' : String(raw));
    }
    
    function updateDashboardRefreshedAt() {
      const el = document.getElementById('dash-refreshed-at');
      if (!el) return;
      const wasm = typeof window !== 'undefined' ? window.poolaiUiWasm : null;
      const iso = new Date().toISOString();
      const formatted =
        wasm && typeof wasm.formatLocaleTimeHms === 'function'
          ? wasm.formatLocaleTimeHms(iso)
          : typeof formatLocaleTimeHms === 'function'
            ? formatLocaleTimeHms(iso)
            : new Date(iso).toLocaleTimeString();
      const p = T('admin.dash.refreshedAt', 'Refreshed: ');
      el.textContent = p + formatted;
    }
    
    function renderRatio96StoreBadge(wire) {
      const el = document.getElementById('ratio96-store-badge');
      if (!el) return;
      const avail = !!(wire && wire.available);
      const stretch = !!(wire && wire.stretch_gate_met);
      const hold = !!(wire && wire.hold_gate_met);
      const hint = T('admin.ratio96.storeHint', 'Phase-F stretch gate read from rust_ratio.json');
      const state = avail
        ? (stretch ? T('admin.ratio96.stretchMet', 'stretch met') : T('admin.ratio96.stretchPending', 'below stretch')) + ' · ' +
          (hold ? T('admin.ratio96.holdMet', 'hold met') : T('admin.ratio96.holdPending', 'below hold'))
        : T('admin.ratio96.storeMissing', 'store unavailable');
      const badge = avail ? (stretch ? 'active' : 'inactive') : 'error';
      el.innerHTML =
        '<span class="status-badge ' + badge + '" title="' + escapeHtml(hint) + '">' +
        escapeHtml(T('admin.ratio96.storeLabel', 'Rust ratio (96% stretch):')) + ' ' +
        escapeHtml(state) + '</span>';
    }

    async function loadRatio96StoreWire() {
      const el = document.getElementById('ratio96-store-badge');
      if (el) {
        el.textContent = T('admin.ratio96.storeLoading', 'Loading ratio store…');
      }
      try {
        const wire = await fetchJson('/api/v1/ops/ratio96');
        renderRatio96StoreBadge(wire);
      } catch (e) {
        if (el) {
          el.innerHTML = '<span class="status-badge error">' +
            escapeHtml(T('admin.ratio96.storeErr', 'Ratio store wire unavailable')) + '</span>';
        }
      }
    }

    async function refreshRatio96() {
      try {
        await loadRatio96StoreWire();
        showNotification(T('admin.ratio96.refreshOk', 'Ratio store refreshed'), 'success');
      } catch (e) {
        showNotification(T('admin.ratio96.refreshErr', 'Ratio store refresh failed: ') + e.message, 'error');
      }
    }

    function renderGpuLimitsStoreBadge(wire) {
      const el = document.getElementById('debug-limits-store-badge') || document.getElementById('gpu-limits-store-badge');
      if (!el) return;
      const avail = !!(wire && wire.available);
      const active = !!(wire && wire.admission_active);
      const hint = T('admin.gpuLimits.storeHint', 'GPU admission limits read from gpu_limits.json');
      const label = avail
        ? T('admin.gpuLimits.storeLabel', 'GPU limits:')
        : T('admin.debug.migrationLabel', 'GPU limits:');
      const state = avail
        ? label + ' ' +
          (active ? T('admin.gpuLimits.admissionOn', 'admission on') : T('admin.gpuLimits.admissionOff', 'admission off'))
        : T('admin.gpuLimits.storeMissing', 'store unavailable');
      const badge = avail ? (active ? 'active' : 'inactive') : 'error';
      el.innerHTML =
        '<span class="status-badge ' + badge + '" title="' + escapeHtml(hint) + '">' +
        escapeHtml(state) + '</span>';
    }

    async function loadGpuLimitsStoreWire() {
      const el = document.getElementById('gpu-limits-store-badge') || document.getElementById('debug-limits-store-badge');
      if (el) {
        el.textContent = T('admin.gpuLimits.storeLoading', 'Loading GPU limits…');
      }
      try {
        const wire = await fetchJson('/api/v1/gpu-limits');
        renderGpuLimitsStoreBadge(wire);
      } catch (e) {
        if (el) {
          el.innerHTML = '<span class="status-badge error">' +
            escapeHtml(T('admin.gpuLimits.storeErr', 'GPU limits store wire unavailable')) + '</span>';
        }
      }
    }

    async function loadDebugLimitsStoreWire() {
      const el = document.getElementById('debug-limits-store-badge') || document.getElementById('gpu-limits-store-badge');
      if (el) {
        el.textContent = T('admin.gpuLimits.storeLoading', 'Loading GPU limits…');
      }
      try {
        const wire = await fetchJson('/api/v1/debug/ui');
        renderGpuLimitsStoreBadge(wire);
      } catch (e) {
        if (el) {
          el.innerHTML = '<span class="status-badge error">' +
            escapeHtml(T('admin.gpuLimits.storeErr', 'GPU limits store wire unavailable')) + '</span>';
        }
      }
    }

    async function refreshGpuLimits() {
      try {
        await loadGpuLimitsStoreWire();
        showNotification(T('admin.gpuLimits.refreshOk', 'GPU limits refreshed'), 'success');
      } catch (e) {
        showNotification(T('admin.gpuLimits.refreshErr', 'GPU limits refresh failed: ') + e.message, 'error');
      }
    }

    async function refreshDebugLimits() {
      try {
        await loadDebugLimitsStoreWire();
        showNotification(T('admin.gpuLimits.refreshOk', 'GPU limits refreshed'), 'success');
      } catch (e) {
        showNotification(T('admin.gpuLimits.refreshErr', 'GPU limits refresh failed: ') + e.message, 'error');
      }
    }

    function renderGpuLimitsMigrationBadge(wire) {
      const el = document.getElementById('gpu-limits-migration-badge');
      if (!el) return;
      const avail = !!(wire && wire.available);
      const active = !!(wire && wire.admission_active);
      const hint = T('admin.gpuLimits.migrationHint', 'GPU limits migration read from gpu_limits.json');
      const state = avail
        ? T('admin.gpuLimits.migrationLabel', 'GPU limits migration:') + ' ' +
          (active ? T('admin.gpuLimits.migrationOn', 'migration on') : T('admin.gpuLimits.migrationOff', 'migration off'))
        : T('admin.gpuLimits.migrationMissing', 'migration unavailable');
      const badge = avail ? (active ? 'active' : 'inactive') : 'error';
      el.innerHTML =
        '<span class="status-badge ' + badge + '" title="' + escapeHtml(hint) + '">' +
        escapeHtml(state) + '</span>';
    }

    async function loadGpuLimitsMigrationWire() {
      const el = document.getElementById('gpu-limits-migration-badge');
      if (el) {
        el.textContent = T('admin.gpuLimits.migrationLoading', 'Loading migration…');
      }
      try {
        const wire = await fetchJson('/api/v1/debug/ui-migration');
        renderGpuLimitsMigrationBadge(wire);
      } catch (e) {
        if (el) {
          el.innerHTML = '<span class="status-badge error">' +
            escapeHtml(T('admin.gpuLimits.migrationErr', 'GPU limits migration wire unavailable')) + '</span>';
        }
      }
}

    async function refreshDebugLimitsMigration() {
      try {
        await loadGpuLimitsMigrationWire();
        showNotification(T('admin.gpuLimits.migrationRefreshOk', 'GPU limits migration refreshed'), 'success');
      } catch (e) {
        showNotification(T('admin.gpuLimits.migrationRefreshErr', 'GPU limits migration refresh failed: ') + e.message, 'error');
      }
    }

    async function refreshDebugLimitsMigration() {
      try {
        await loadGpuLimitsMigrationWire();
        showNotification(T('admin.gpuLimits.migrationRefreshOk', 'GPU limits migration refreshed'), 'success');
      } catch (e) {
        showNotification(T('admin.gpuLimits.migrationRefreshErr', 'GPU limits migration refresh failed: ') + e.message, 'error');
      }
    }

    loadSystemOverview();
    renderMetricsChart();
    loadRatio96StoreWire();
    loadGpuLimitsStoreWire();
    loadGpuLimitsMigrationWire();
    poolaiStartMetricsPolling(loadSystemOverview, 30000);
    "#;

    admin_layout_dashboard(
        "admin.page.dashboard",
        "Admin Dashboard",
        r#"
        <p id="dash-refreshed-at" class="muted"></p>
        <div class="admin-grid">
          <div class="admin-card">
            <h3 data-i18n="admin.dash.card.overview">System Overview</h3>
            <div id="system-overview"></div>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.dash.card.quickStats">Quick Stats</h3>
            <div id="quick-stats"></div>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.dash.card.alerts">Active Alerts</h3>
            <div id="active-alerts"></div>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.dash.card.activity">Recent Activity</h3>
            <div id="recent-activity"></div>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.dash.card.metrics">Metrics Overview (Last Hour)</h3>
            <div id="metrics-chart"></div>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.ratio96.cardTitle">Rust Ratio (Phase F)</h3>
            <span id="ratio96-store-badge" class="muted" data-i18n="admin.ratio96.storeLoading">Loading ratio store…</span>
            <button type="button" class="btn" onclick="refreshRatio96()" data-i18n="admin.ratio96.btn.refresh" data-i18n-aria="admin.ratio96.btn.refresh">Refresh</button>
          </div>
          <div class="admin-card">
            <h3 data-i18n="admin.gpuLimits.cardTitle">GPU Limits (Phase H)</h3>
            <span id="gpu-limits-store-badge" class="muted" data-debug-id="debug-limits-store-badge" data-i18n="admin.gpuLimits.storeLabel">Loading GPU limits…</span>
            <button type="button" class="btn" onclick="refreshGpuLimits()" data-i18n="admin.gpuLimits.btn.refresh" data-i18n-aria="admin.gpuLimits.btn.refresh" data-i18n-debug="admin.debug.btn.refresh">Refresh</button>
          </div>
        </div>
        <div class="admin-card">
            <h3 data-i18n="admin.gpuLimits.cardTitle">GPU Limits Migration (Phase H)</h3>
            <span id="gpu-limits-migration-badge" class="muted" data-i18n="admin.gpuLimits.migrationLabel">Loading migration…</span>
            <button type="button" class="btn" onclick="refreshDebugLimitsMigration()" data-i18n="admin.debug.btn.refresh" data-i18n-aria="admin.debug.btn.refresh">Refresh</button>
          </div>
          "#,
        script,
    )
}

#[tokio::test]
async fn admin_dashboard_page_slim_dashboard_i18n_patch_ph_s228() {
    let html = admin_dashboard().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.dashboard""#));
    assert!(html.contains(r#""admin.dash.card.overview""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.mon.mlTitle""#));
}

#[tokio::test]
async fn admin_dashboard_ratio96_store_strip_ph_s1682() {
    let html = admin_dashboard().await.0;
    assert!(html.contains("ratio96-store-badge"));
    assert!(html.contains("loadRatio96StoreWire"));
    assert!(html.contains("/api/v1/ops/ratio96"));
    assert!(html.contains("refreshRatio96"));
    assert!(html.contains("admin.ratio96.storeLabel"));
    assert!(html.contains("admin.ratio96.btn.refresh"));
}

#[tokio::test]
async fn admin_dashboard_gpu_limits_store_strip_ph_s1882() {
    let html = admin_dashboard().await.0;
    assert!(html.contains("debug-limits-store-badge"));
    assert!(html.contains("loadDebugLimitsStoreWire"));
    assert!(html.contains("/api/v1/debug/ui"));
    assert!(html.contains("refreshDebugLimits"));
    assert!(html.contains("admin.debug.migrationLabel"));
    assert!(html.contains("admin.debug.btn.refresh"));
}
