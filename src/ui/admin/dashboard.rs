//! Admin Dashboard page
//!
//! Provides system overview with real-time status, metrics, alerts, and recent activity.

use crate::ui::admin::admin_layout;
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
        const overview = await fetchJson('/api/v1/admin/overview');
        renderSystemOverview(overview);
        renderQuickStats(overview);
        const [alerts, audit] = await Promise.all([
          fetchJson('/api/enterprise/monitoring/alerts?acknowledged=false&limit=5'),
          fetchJson('/api/enterprise/audit/events?limit=10')
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
          <span class="stat-value">${formatUptime(data.uptime_seconds || 0)}</span>
        </div>
      `;
    }
    
    async function loadMetricsHistory() {
      try {
        const endTime = new Date().toISOString();
        const startTime = new Date(Date.now() - 1 * 60 * 60 * 1000).toISOString(); // Last hour
        const metrics = await fetchJson(`/api/enterprise/monitoring/metrics?start_time=${encodeURIComponent(startTime)}&end_time=${encodeURIComponent(endTime)}&limit=60`);
        return metrics || [];
      } catch (e) {
        console.error('Error loading metrics history:', e);
        return [];
      }
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
      const metrics = await loadMetricsHistory();
      if (metrics.length === 0) return;
      
      const el = document.getElementById('metrics-chart');
      if (!el) return;
      
      // Group metrics by name
      const byMetric = {};
      metrics.forEach(m => {
        if (!byMetric[m.metric]) byMetric[m.metric] = [];
        byMetric[m.metric].push(m);
      });
      
      // Render simple sparkline for CPU and Memory
      const cpuData = byMetric['cpu_usage'] || [];
      const memData = byMetric['memory_usage'] || [];
      
      if (cpuData.length > 0 || memData.length > 0) {
        el.innerHTML = `
          <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-top: 16px;">
            ${cpuData.length > 0 ? renderSparkline(T('admin.dash.spark.cpu', 'CPU Usage'), cpuData.map(d => d.value || 0)) : ''}
            ${memData.length > 0 ? renderSparkline(T('admin.dash.spark.memory', 'Memory Usage'), memData.map(d => d.value || 0)) : ''}
          </div>
        `;
      }
    }
    
    function renderSparkline(label, values) {
      if (values.length === 0) return '';
      const width = 200;
      const height = 40;
      const padding = 4;
      const chartWidth = width - padding * 2;
      const chartHeight = height - padding * 2;
      
      const min = Math.min(...values);
      const max = Math.max(...values);
      const range = max - min || 1;
      
      const points = values.map((v, i) => {
        const x = padding + (i / (values.length - 1 || 1)) * chartWidth;
        const y = padding + chartHeight - ((v - min) / range) * chartHeight;
        return `${x},${y}`;
      }).join(' ');
      
      const avg = values.reduce((a, b) => a + b, 0) / values.length;
      
      return `
        <div style="background: var(--surface, #171b22); padding: 12px; border-radius: 8px; border: 1px solid var(--border, #262b36);">
          <div style="font-size: 0.85em; color: var(--text-muted, #a8b0bf); margin-bottom: 4px;">${label}</div>
          <svg width="${width}" height="${height}" style="display: block;">
            <polyline points="${points}" fill="none" stroke="var(--primary, #67e480)" stroke-width="1.5" />
          </svg>
          <div style="font-size: 0.9em; margin-top: 4px;">
            <span style="color: var(--text-muted, #a8b0bf);">${T('admin.dash.avg', 'Avg: ')}</span>
            <strong style="color: var(--text, #e8e8e8);">${avg.toFixed(1)}</strong>
          </div>
        </div>
      `;
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
    setInterval(loadSystemOverview, 10000);
    setInterval(renderMetricsChart, 30000); // Update charts every 30s
    "#;

    admin_layout(
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
