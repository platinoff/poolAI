//! Monitoring Dashboard page
//!
//! Provides real-time monitoring with alerts and dashboards.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// Monitoring dashboard page
pub async fn admin_monitoring() -> Html<String> {
    let script = r#"
    async function loadMonitoring() {
      try {
        const [alerts, dashboards] = await Promise.all([
          fetchJson('/api/enterprise/monitoring/alerts?limit=20'),
          fetchJson('/api/enterprise/monitoring/dashboards')
        ]);
        renderMonitoring(alerts, dashboards);
      } catch (e) {
        showNotification('Error loading monitoring: ' + e.message, 'error');
      }
    }
    
    function renderMonitoring(alerts, dashboards) {
      const el = document.getElementById('monitoring-content');
      if (!el) return;
      el.innerHTML = `
        <div class="admin-card">
          <h3>Active Alerts (${alerts.length})</h3>
          <div>${alerts.map(a => `<div>${a.metric}: ${a.current_value} (threshold: ${a.threshold})</div>`).join('')}</div>
        </div>
        <div class="admin-card">
          <h3>Dashboards (${dashboards.length})</h3>
          <div>${dashboards.map(d => `<div>${d.name}</div>`).join('')}</div>
        </div>
      `;
    }
    
    loadMonitoring();
    setInterval(loadMonitoring, 5000);
    "#;

    admin_layout(
        "Monitoring Dashboard",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Monitoring</h2>
            <button class="btn btn-primary" onclick="showNotification('Create dashboard - to be implemented', 'info')">Create Dashboard</button>
            <button class="btn" onclick="showNotification('Create alert rule - to be implemented', 'info')">Create Alert Rule</button>
          </div>
          <div id="monitoring-content"></div>
        </div>
        "#,
        script,
    )
}
