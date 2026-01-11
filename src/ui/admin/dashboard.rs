//! Admin Dashboard page
//!
//! Provides system overview with real-time status, metrics, alerts, and recent activity.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Admin dashboard home page
pub async fn admin_dashboard() -> Html<String> {
    let script = r#"
    async function loadSystemOverview() {
      try {
        const [status, metrics, alerts, audit] = await Promise.all([
          fetchJson('/api/v1/status'),
          fetchJson('/api/v1/metrics'),
          fetchJson('/api/enterprise/monitoring/alerts?acknowledged=false&limit=5'),
          fetchJson('/api/enterprise/audit/events?limit=10')
        ]);
        
        renderSystemOverview(status);
        renderQuickStats(metrics);
        renderActiveAlerts(alerts);
        renderRecentActivity(audit);
      } catch (e) {
        showNotification('Error loading dashboard: ' + e.message, 'error');
      }
    }
    
    function renderSystemOverview(data) {
      const el = document.getElementById('system-overview');
      if (!el) return;
      el.innerHTML = `
        <div class="stat-item">
          <span class="stat-label">Status:</span>
          <span class="stat-value status-badge ${data.status === 'healthy' ? 'active' : 'error'}">${data.status || 'unknown'}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">Uptime:</span>
          <span class="stat-value">${formatUptime(data.uptime_seconds || 0)}</span>
        </div>
      `;
    }
    
    function renderQuickStats(data) {
      const el = document.getElementById('quick-stats');
      if (!el) return;
      el.innerHTML = `
        <div class="stat-item">
          <span class="stat-label">Workers:</span>
          <span class="stat-value">${data.workers || 0}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">VM Instances:</span>
          <span class="stat-value">${data.vm_instances || 0}</span>
        </div>
      `;
    }
    
    function renderActiveAlerts(data) {
      const el = document.getElementById('active-alerts');
      if (!el) return;
      if (!data || data.length === 0) {
        el.innerHTML = '<div class="muted">No active alerts</div>';
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
        el.innerHTML = '<div class="muted">No recent activity</div>';
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
    setInterval(loadSystemOverview, 10000);
    "#;

    admin_layout(
        "Admin Dashboard",
        r#"
        <div class="admin-grid">
          <div class="admin-card">
            <h3>System Overview</h3>
            <div id="system-overview"></div>
          </div>
          <div class="admin-card">
            <h3>Quick Stats</h3>
            <div id="quick-stats"></div>
          </div>
          <div class="admin-card">
            <h3>Active Alerts</h3>
            <div id="active-alerts"></div>
          </div>
          <div class="admin-card">
            <h3>Recent Activity</h3>
            <div id="recent-activity"></div>
          </div>
        </div>
        "#,
        script,
    )
}
