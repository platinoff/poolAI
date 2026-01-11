//! Monitoring Dashboard page
//!
//! Provides real-time monitoring with alerts and dashboards.

use crate::ui::admin::admin_layout;
use axum::response::Html;

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
    
    async function loadAlertRules() {
      try {
        const rules = await fetchJson('/api/enterprise/monitoring/alert-rules');
        return rules;
      } catch (e) {
        console.error('Error loading alert rules:', e);
        return [];
      }
    }
    
    async function loadAlertRules() {
      try {
        const rules = await fetchJson('/api/enterprise/monitoring/alert-rules');
        return rules;
      } catch (e) {
        console.error('Error loading alert rules:', e);
        return [];
      }
    }
    
    function renderMonitoring(alerts, dashboards) {
      const el = document.getElementById('monitoring-content');
      if (!el) return;
      
      const alertsHtml = alerts.length === 0 
        ? '<div class="muted">No active alerts</div>'
        : `
          <table class="admin-table">
            <thead>
              <tr>
                <th>Severity</th>
                <th>Metric</th>
                <th>Current Value</th>
                <th>Threshold</th>
                <th>Triggered</th>
                <th>Status</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              ${alerts.map(a => `
                <tr>
                  <td><span class="status-badge ${a.severity?.toLowerCase() || 'warning'}">${a.severity || 'WARNING'}</span></td>
                  <td><strong>${a.metric || 'unknown'}</strong></td>
                  <td>${a.current_value || 'N/A'}</td>
                  <td>${a.threshold || 'N/A'}</td>
                  <td>${a.triggered_at ? new Date(a.triggered_at).toLocaleString() : 'N/A'}</td>
                  <td>${a.acknowledged ? '<span class="muted">Acknowledged</span>' : '<span class="status-badge active">Active</span>'}</td>
                  <td>${a.acknowledged ? '' : '<button class="btn btn-sm" onclick="acknowledgeAlert(\'' + a.id + '\')">Acknowledge</button>'}</td>
                </tr>
              `).join('')}
            </tbody>
          </table>
        `;
      
      const dashboardsHtml = dashboards.length === 0
        ? '<div class="muted">No dashboards created</div>'
        : `
          <table class="admin-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Description</th>
                <th>Metrics</th>
                <th>Public</th>
                <th>Created</th>
              </tr>
            </thead>
            <tbody>
              ${dashboards.map(d => `
                <tr>
                  <td><strong>${d.name || 'unnamed'}</strong></td>
                  <td>${d.description || '—'}</td>
                  <td>${d.metrics?.length || 0} metrics</td>
                  <td><span class="status-badge ${d.is_public ? 'active' : 'inactive'}">${d.is_public ? 'Public' : 'Private'}</span></td>
                  <td>${d.created_at ? new Date(d.created_at).toLocaleDateString() : 'N/A'}</td>
                </tr>
              `).join('')}
            </tbody>
          </table>
        `;
      
      el.innerHTML = `
        <div class="admin-card">
          <h3>Active Alerts (${alerts.length})</h3>
          ${alertsHtml}
        </div>
        <div class="admin-card">
          <h3>Dashboards (${dashboards.length})</h3>
          ${dashboardsHtml}
        </div>
      `;
      
      // Load alert rules asynchronously
      loadAlertRules().then(rules => {
        if (rules && rules.length > 0) {
          const rulesHtml = `
            <div class="admin-card">
              <h3>Alert Rules (${rules.length})</h3>
              <table class="admin-table">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>Metric</th>
                    <th>Operator</th>
                    <th>Threshold</th>
                    <th>Severity</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody>
                  ${rules.map(r => `
                    <tr>
                      <td><strong>${r.name || 'unnamed'}</strong></td>
                      <td>${r.metric || 'N/A'}</td>
                      <td><code>${r.operator || '>'}</code></td>
                      <td>${r.threshold || 'N/A'}</td>
                      <td><span class="status-badge ${r.severity?.toLowerCase() || 'warning'}">${r.severity || 'WARNING'}</span></td>
                      <td><span class="status-badge ${r.enabled ? 'active' : 'inactive'}">${r.enabled ? 'Enabled' : 'Disabled'}</span></td>
                    </tr>
                  `).join('')}
                </tbody>
              </table>
            </div>
          `;
          el.innerHTML += rulesHtml;
        }
      }).catch(e => console.error('Error loading alert rules:', e));
    }
    
    function showCreateDashboardModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createDashboardModal');
    }
    
    async function handleCreateDashboard(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const metrics = document.getElementById('dashboardMetrics').value.split(',').map(s => s.trim()).filter(s => s);
        
        const payload = {
          name: document.getElementById('dashboardName').value,
          description: document.getElementById('dashboardDescription').value || null,
          metrics: metrics,
          layout: document.getElementById('dashboardLayout').value || null,
          is_public: document.getElementById('dashboardIsPublic').checked
        };
        
        await fetchJson('/api/enterprise/monitoring/dashboards', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Dashboard created successfully', 'success');
        hideModal('createDashboardModal');
        form.reset();
        loadMonitoring();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    function showCreateAlertRuleModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createAlertRuleModal');
    }
    
    async function handleCreateAlertRule(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const payload = {
          name: document.getElementById('alertRuleName').value,
          metric: document.getElementById('alertRuleMetric').value,
          threshold: parseFloat(document.getElementById('alertRuleThreshold').value),
          operator: document.getElementById('alertRuleOperator').value,
          severity: document.getElementById('alertRuleSeverity').value,
          enabled: document.getElementById('alertRuleEnabled').checked
        };
        
        await fetchJson('/api/enterprise/monitoring/alert-rules', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Alert rule created successfully', 'success');
        hideModal('createAlertRuleModal');
        form.reset();
        loadMonitoring();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function acknowledgeAlert(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/monitoring/alerts/${id}/acknowledge`, {
          method: 'POST'
        });
        showNotification('Alert acknowledged', 'success');
        loadMonitoring();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
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
            <button class="btn btn-primary" onclick="showCreateDashboardModal()" aria-label="Create dashboard">Create Dashboard</button>
            <button class="btn" onclick="showCreateAlertRuleModal()" aria-label="Create alert rule">Create Alert Rule</button>
          </div>
          <div id="monitoring-content"></div>
        </div>

        <!-- Create Dashboard Modal -->
        <div id="createDashboardModal" class="modal" role="dialog" aria-labelledby="createDashboardModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createDashboardModalTitle">Create Dashboard</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createDashboardModal')">&times;</button>
            </div>
            <form id="createDashboardForm" onsubmit="handleCreateDashboard(event)">
              <div class="form-group">
                <label for="dashboardName">Dashboard Name <span class="required">*</span></label>
                <input type="text" id="dashboardName" name="name" required placeholder="My Dashboard" />
              </div>
              <div class="form-group">
                <label for="dashboardDescription">Description</label>
                <textarea id="dashboardDescription" name="description" rows="3" placeholder="Dashboard description"></textarea>
              </div>
              <div class="form-group">
                <label for="dashboardMetrics">Metrics (comma-separated) <span class="required">*</span></label>
                <input type="text" id="dashboardMetrics" name="metrics" required placeholder="cpu_usage, memory_usage, request_rate" />
                <small class="form-hint">Enter metric names separated by commas</small>
              </div>
              <div class="form-group">
                <label for="dashboardLayout">Layout (JSON, optional)</label>
                <textarea id="dashboardLayout" name="layout" rows="5" placeholder='{"widgets": []}'></textarea>
              </div>
              <div class="form-group">
                <label for="dashboardIsPublic">
                  <input type="checkbox" id="dashboardIsPublic" name="is_public" />
                  Public Dashboard
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createDashboardModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>

        <!-- Create Alert Rule Modal -->
        <div id="createAlertRuleModal" class="modal" role="dialog" aria-labelledby="createAlertRuleModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createAlertRuleModalTitle">Create Alert Rule</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createAlertRuleModal')">&times;</button>
            </div>
            <form id="createAlertRuleForm" onsubmit="handleCreateAlertRule(event)">
              <div class="form-group">
                <label for="alertRuleName">Rule Name <span class="required">*</span></label>
                <input type="text" id="alertRuleName" name="name" required placeholder="high-cpu-alert" />
              </div>
              <div class="form-group">
                <label for="alertRuleMetric">Metric Name <span class="required">*</span></label>
                <input type="text" id="alertRuleMetric" name="metric" required placeholder="cpu_usage" />
              </div>
              <div class="form-group">
                <label for="alertRuleOperator">Operator <span class="required">*</span></label>
                <select id="alertRuleOperator" name="operator" required>
                  <option value=">">Greater than (&gt;)</option>
                  <option value="<">Less than (&lt;)</option>
                  <option value=">=">Greater or equal (&gt;=)</option>
                  <option value="<=">Less or equal (&lt;=)</option>
                  <option value="==">Equal (==)</option>
                </select>
              </div>
              <div class="form-group">
                <label for="alertRuleThreshold">Threshold <span class="required">*</span></label>
                <input type="number" id="alertRuleThreshold" name="threshold" required step="0.1" placeholder="90.0" />
              </div>
              <div class="form-group">
                <label for="alertRuleSeverity">Severity <span class="required">*</span></label>
                <select id="alertRuleSeverity" name="severity" required>
                  <option value="Info">Info</option>
                  <option value="Warning" selected>Warning</option>
                  <option value="Error">Error</option>
                  <option value="Critical">Critical</option>
                </select>
              </div>
              <div class="form-group">
                <label for="alertRuleEnabled">
                  <input type="checkbox" id="alertRuleEnabled" name="enabled" checked />
                  Enabled
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createAlertRuleModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
