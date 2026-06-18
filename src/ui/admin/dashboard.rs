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
              : formatUptime(data.uptime_seconds || 0)
          }</span>
        </div>
      `;
    }
    
    
    function renderQuickStats(data) {
      const el = document.getElementById('quick-stats');
      if (!el) return;
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
          <span class="stat-value">${(data.cpu_usage_percent ?? 0).toFixed(1)}%</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">${T('admin.dash.quick.memory', 'Memory (tracked):')}</span>
          <span class="stat-value">${(data.memory_usage_mb ?? 0).toFixed(0)} MB</span>
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
        el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.dash.noAlerts', 'No active alerts')) + '</div>';
        return;
      }
      el.innerHTML = data.map(alert => `
        <div class="alert-item">
          <span class="status-badge ${alert.severity.toLowerCase()}">${alert.severity}</span>
          <span>${alert.metric}: ${alert.current_value}</span>
        </div>
      `).join('');
    }
    
    function renderRecentActivity(data) {
      const el = document.getElementById('recent-activity');
      if (!el) return;
      if (!data || data.length === 0) {
        el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.dash.noActivity', 'No recent activity')) + '</div>';
        return;
      }
      el.innerHTML = data.map(event => `
        <div class="activity-item">
          <span class="muted">${new Date(event.timestamp).toLocaleString()}</span>
          <span>${event.action}</span>
        </div>
      `).join('');
    }
    
    function formatUptime(seconds) {
      const days = Math.floor(seconds / 86400);
      const hours = Math.floor((seconds % 86400) / 3600);
      const mins = Math.floor((seconds % 3600) / 60);
      return `${days}d ${hours}h ${mins}m`;
    }
    
    loadSystemOverview();
    renderMetricsChart();
    poolaiStartMetricsPolling(loadSystemOverview, 10000);
    poolaiStartMetricsPolling(renderMetricsChart, 30000);
    "#;

    admin_layout_dashboard(
        "admin.page.dashboard",
        "Admin Dashboard",
        r#"
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
