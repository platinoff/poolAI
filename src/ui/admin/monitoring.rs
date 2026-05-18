//! Monitoring Dashboard page
//!
//! Provides real-time monitoring with alerts and dashboards.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Monitoring dashboard page
pub async fn admin_monitoring() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadMonitoring() {
      adminShowLoading('monitoring-content', T('admin.mon.loading', 'Loading monitoring…'));
      try {
        const [alerts, dashboards, metrics] = await Promise.all([
          fetchJson('/api/enterprise/monitoring/alerts?limit=20'),
          fetchJson('/api/enterprise/monitoring/dashboards'),
          fetchJson('/api/enterprise/monitoring/metrics?limit=100')
        ]);
        renderMonitoring(alerts, dashboards, metrics);
      } catch (e) {
        adminShowInlineError('monitoring-content', e);
        showNotification(T('admin.mon.errLoad', 'Error loading monitoring: ') + e.message, 'error');
      }
    }
    
    async function loadMetricHistory(metricName, hours = 24) {
      try {
        const endTime = new Date().toISOString();
        const startTime = new Date(Date.now() - hours * 60 * 60 * 1000).toISOString();
        const url = `/api/enterprise/monitoring/metrics?metric=${encodeURIComponent(metricName)}&start_time=${encodeURIComponent(startTime)}&end_time=${encodeURIComponent(endTime)}&limit=200`;
        const data = await fetchJson(url);
        return data || [];
      } catch (e) {
        console.error('Error loading metric history:', e);
        return [];
      }
    }
    
    function renderMetricChart(metricName, data) {
      if (!data || data.length === 0) {
        return '<div class="muted">' + escapeHtml(T('admin.mon.noData', 'No data available')) + '</div>';
      }
      
      const width = 600;
      const height = 200;
      const padding = 40;
      const chartWidth = width - padding * 2;
      const chartHeight = height - padding * 2;
      
      const values = data.map(d => d.value || 0);
      const min = Math.min(...values);
      const max = Math.max(...values);
      const range = max - min || 1;
      
      const points = values.map((v, i) => {
        const x = padding + (i / (values.length - 1 || 1)) * chartWidth;
        const y = padding + chartHeight - ((v - min) / range) * chartHeight;
        return `${x},${y}`;
      }).join(' ');
      
      const pointsLabel = T('admin.mon.chartPoints', '{n} points').replace(/\{n\}/g, String(data.length));
      
      return `
        <div class="metric-chart-container">
          <h4>${escapeHtml(metricName)}</h4>
          <svg width="${width}" height="${height}" style="max-width: 100%; height: auto;">
            <defs>
              <linearGradient id="grad-${metricName.replace(/[^a-zA-Z0-9_-]/g, '_')}" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:var(--primary, #67e480);stop-opacity:0.3" />
                <stop offset="100%" style="stop-color:var(--primary, #67e480);stop-opacity:0.05" />
              </linearGradient>
            </defs>
            <rect x="${padding}" y="${padding}" width="${chartWidth}" height="${chartHeight}" fill="url(#grad-${metricName.replace(/[^a-zA-Z0-9_-]/g, '_')})" />
            <polyline points="${points}" fill="none" stroke="var(--primary, #67e480)" stroke-width="2" />
            ${values.map((v, i) => {
              const x = padding + (i / (values.length - 1 || 1)) * chartWidth;
              const y = padding + chartHeight - ((v - min) / range) * chartHeight;
              return `<circle cx="${x}" cy="${y}" r="3" fill="var(--primary, #67e480)" />`;
            }).join('')}
            <text x="${padding}" y="${padding - 10}" fill="var(--text, #f8f8f2)" font-size="12">${max.toFixed(1)}</text>
            <text x="${padding}" y="${height - padding + 20}" fill="var(--text, #f8f8f2)" font-size="12">${min.toFixed(1)}</text>
            <text x="${width - padding}" y="${height - padding + 20}" fill="var(--text-muted, #a8b0bf)" font-size="10" text-anchor="end">${escapeHtml(pointsLabel)}</text>
          </svg>
          <div class="metric-stats" style="margin-top: 8px; display: flex; gap: 16px; font-size: 0.9em;">
            <span>${escapeHtml(T('admin.mon.statMin', 'Min:'))} <strong>${min.toFixed(2)}</strong></span>
            <span>${escapeHtml(T('admin.mon.statMax', 'Max:'))} <strong>${max.toFixed(2)}</strong></span>
            <span>${escapeHtml(T('admin.mon.statAvg', 'Avg:'))} <strong>${(values.reduce((a, b) => a + b, 0) / values.length).toFixed(2)}</strong></span>
          </div>
        </div>
      `;
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
    
    async function renderMonitoring(alerts, dashboards, metrics) {
      const el = document.getElementById('monitoring-content');
      if (!el) return;
      
      const commonMetrics = ['cpu_usage', 'memory_usage', 'request_rate'];
      const metricCharts = {};
      for (const metric of commonMetrics) {
        const history = await loadMetricHistory(metric, 24);
        if (history.length > 0) {
          metricCharts[metric] = renderMetricChart(metric, history);
        }
      }
      
      const alertsHtml = alerts.length === 0 
        ? '<div class="muted">' + escapeHtml(T('admin.mon.noAlerts', 'No active alerts')) + '</div>'
        : `
          <table class="admin-table">
            <thead>
              <tr>
                <th>${escapeHtml(T('admin.mon.col.severity', 'Severity'))}</th>
                <th>${escapeHtml(T('admin.mon.col.metric', 'Metric'))}</th>
                <th>${escapeHtml(T('admin.mon.col.currentVal', 'Current Value'))}</th>
                <th>${escapeHtml(T('admin.mon.col.threshold', 'Threshold'))}</th>
                <th>${escapeHtml(T('admin.mon.col.triggered', 'Triggered'))}</th>
                <th>${escapeHtml(T('admin.mon.col.statusCol', 'Status'))}</th>
                <th>${escapeHtml(T('admin.mon.col.actions', 'Actions'))}</th>
              </tr>
            </thead>
            <tbody>
              ${alerts.map(a => `
                <tr>
                  <td><span class="status-badge ${a.severity?.toLowerCase() || 'warning'}">${escapeHtml(a.severity || 'WARNING')}</span></td>
                  <td><strong>${escapeHtml(a.metric || 'unknown')}</strong></td>
                  <td>${escapeHtml(String(a.current_value != null ? a.current_value : T('admin.na', 'N/A')))}</td>
                  <td>${escapeHtml(String(a.threshold != null ? a.threshold : T('admin.na', 'N/A')))}</td>
                  <td>${a.triggered_at ? escapeHtml(new Date(a.triggered_at).toLocaleString()) : escapeHtml(T('admin.na', 'N/A'))}</td>
                  <td>${a.acknowledged ? '<span class="muted">' + escapeHtml(T('admin.mon.statusAck', 'Acknowledged')) + '</span>' : '<span class="status-badge active">' + escapeHtml(T('admin.mon.statusActiveLbl', 'Active')) + '</span>'}</td>
                  <td>${a.acknowledged ? '' : '<button type="button" class="btn btn-sm" onclick="acknowledgeAlert(' + JSON.stringify(a.id) + ')">' + escapeHtml(T('admin.mon.ackBtn', 'Acknowledge')) + '</button>'}</td>
                </tr>
              `).join('')}
            </tbody>
          </table>
        `;
      
      const dashboardsHtml = dashboards.length === 0
        ? '<div class="muted">' + escapeHtml(T('admin.mon.noDashboards', 'No dashboards created')) + '</div>'
        : `
          <table class="admin-table">
            <thead>
              <tr>
                <th>${escapeHtml(T('admin.mon.col.name', 'Name'))}</th>
                <th>${escapeHtml(T('admin.mon.col.description', 'Description'))}</th>
                <th>${escapeHtml(T('admin.mon.col.metrics', 'Metrics'))}</th>
                <th>${escapeHtml(T('admin.mon.col.public', 'Public'))}</th>
                <th>${escapeHtml(T('admin.mon.col.created', 'Created'))}</th>
              </tr>
            </thead>
            <tbody>
              ${dashboards.map(d => `
                <tr>
                  <td><strong>${escapeHtml(d.name || 'unnamed')}</strong></td>
                  <td>${escapeHtml(d.description || T('admin.sec.emDash', '—'))}</td>
                  <td>${escapeHtml(T('admin.mon.metricsN', '{n} metrics').replace(/\{n\}/g, String(d.metrics?.length || 0)))}</td>
                  <td><span class="status-badge ${d.is_public ? 'active' : 'inactive'}">${d.is_public ? escapeHtml(T('admin.mon.public', 'Public')) : escapeHtml(T('admin.mon.private', 'Private'))}</span></td>
                  <td>${d.created_at ? escapeHtml(new Date(d.created_at).toLocaleDateString()) : escapeHtml(T('admin.na', 'N/A'))}</td>
                </tr>
              `).join('')}
            </tbody>
          </table>
        `;
      
      const chartsHtml = Object.keys(metricCharts).length > 0
        ? `
          <div class="admin-card">
            <h3>${escapeHtml(T('admin.mon.vizTitle', 'Metrics Visualization (Last 24 Hours)'))}</h3>
            <div class="metrics-charts-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px;">
              ${Object.values(metricCharts).join('')}
            </div>
          </div>
        `
        : '';
      
      el.innerHTML = `
        ${chartsHtml}
        <div class="admin-card">
          <h3>${escapeHtml(T('admin.mon.activeAlertsTitle', 'Active Alerts'))} (${alerts.length})</h3>
          ${alertsHtml}
        </div>
        <div class="admin-card">
          <h3>${escapeHtml(T('admin.mon.dashboardsTitle', 'Dashboards'))} (${dashboards.length})</h3>
          ${dashboardsHtml}
        </div>
      `;
      
      loadAlertRules().then(rules => {
        if (rules && rules.length > 0) {
          const rulesHtml = `
            <div class="admin-card">
              <h3>${escapeHtml(T('admin.mon.alertRulesTitle', 'Alert Rules'))} (${rules.length})</h3>
              <table class="admin-table">
                <thead>
                  <tr>
                    <th>${escapeHtml(T('admin.mon.col.name', 'Name'))}</th>
                    <th>${escapeHtml(T('admin.mon.col.metric', 'Metric'))}</th>
                    <th>${escapeHtml(T('admin.mon.col.operator', 'Operator'))}</th>
                    <th>${escapeHtml(T('admin.mon.lbl.threshold', 'Threshold'))}</th>
                    <th>${escapeHtml(T('admin.mon.lbl.severity', 'Severity'))}</th>
                    <th>${escapeHtml(T('admin.mon.col.ruleStatus', 'Status'))}</th>
                  </tr>
                </thead>
                <tbody>
                  ${rules.map(r => `
                    <tr>
                      <td><strong>${escapeHtml(r.name || 'unnamed')}</strong></td>
                      <td>${escapeHtml(String(r.metric || T('admin.na', 'N/A')))}</td>
                      <td><code>${escapeHtml(String(r.operator || '>'))}</code></td>
                      <td>${escapeHtml(String(r.threshold != null ? r.threshold : T('admin.na', 'N/A')))}</td>
                      <td><span class="status-badge ${r.severity?.toLowerCase() || 'warning'}">${escapeHtml(r.severity || 'WARNING')}</span></td>
                      <td><span class="status-badge ${r.enabled ? 'active' : 'inactive'}">${r.enabled ? escapeHtml(T('admin.mon.enabled', 'Enabled')) : escapeHtml(T('admin.mon.disabled', 'Disabled'))}</span></td>
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
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('createDashboardModal');
    }
    
    async function handleCreateDashboard(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.mon.creatingDash', 'Creating…');
      
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
        
        showNotification(T('admin.mon.dashCreatedOk', 'Dashboard created successfully'), 'success');
        hideModal('createDashboardModal');
        form.reset();
        loadMonitoring();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    function showCreateAlertRuleModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('createAlertRuleModal');
    }
    
    async function handleCreateAlertRule(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.mon.creatingRule', 'Creating…');
      
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
        
        showNotification(T('admin.mon.ruleCreatedOk', 'Alert rule created successfully'), 'success');
        hideModal('createAlertRuleModal');
        form.reset();
        loadMonitoring();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function acknowledgeAlert(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/monitoring/alerts/${encodeURIComponent(id)}/acknowledge`, {
          method: 'POST'
        });
        showNotification(T('admin.mon.alertAckOk', 'Alert acknowledged'), 'success');
        loadMonitoring();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    loadMonitoring();
    setInterval(loadMonitoring, 5000);
    "#;

    admin_layout(
        "admin.page.monitoring",
        "Monitoring Dashboard",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.mon.section">Monitoring</h2>
            <button type="button" class="btn btn-primary" onclick="showCreateDashboardModal()" data-i18n="admin.mon.createDashBtn" data-i18n-aria="admin.mon.createDashBtn">Create Dashboard</button>
            <button type="button" class="btn" onclick="showCreateAlertRuleModal()" data-i18n="admin.mon.createRuleBtn" data-i18n-aria="admin.mon.createRuleBtn">Create Alert Rule</button>
          </div>
          <div id="monitoring-content"></div>
        </div>

        <div id="createDashboardModal" class="modal" role="dialog" aria-labelledby="createDashboardModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createDashboardModalTitle" data-i18n="admin.mon.modalCreateDash">Create Dashboard</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createDashboardModal')">&times;</button>
            </div>
            <form id="createDashboardForm" onsubmit="handleCreateDashboard(event)">
              <div class="form-group">
                <label for="dashboardName"><span data-i18n="admin.mon.lbl.dashName">Dashboard Name</span> <span class="required">*</span></label>
                <input type="text" id="dashboardName" name="name" required data-i18n-placeholder="admin.mon.ph.dashboard" placeholder="My Dashboard" />
              </div>
              <div class="form-group">
                <label for="dashboardDescription" data-i18n="admin.mon.lbl.dashDesc">Description</label>
                <textarea id="dashboardDescription" name="description" rows="3" data-i18n-placeholder="admin.mon.ph.dashDesc" placeholder="Dashboard description"></textarea>
              </div>
              <div class="form-group">
                <label for="dashboardMetrics"><span data-i18n="admin.mon.lbl.dashMetrics">Metrics (comma-separated)</span> <span class="required">*</span></label>
                <input type="text" id="dashboardMetrics" name="metrics" required data-i18n-placeholder="admin.mon.ph.metricsCsv" placeholder="cpu_usage, memory_usage, request_rate" />
                <small class="form-hint" data-i18n="admin.mon.hint.dashMetrics">Enter metric names separated by commas</small>
              </div>
              <div class="form-group">
                <label for="dashboardLayout" data-i18n="admin.mon.lbl.dashLayout">Layout (JSON, optional)</label>
                <textarea id="dashboardLayout" name="layout" rows="5" data-i18n-placeholder="admin.mon.ph.layoutJson" placeholder='{"widgets": []}'></textarea>
              </div>
              <div class="form-group">
                <label for="dashboardIsPublic">
                  <input type="checkbox" id="dashboardIsPublic" name="is_public" />
                  <span data-i18n="admin.mon.lbl.dashPublic">Public Dashboard</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createDashboardModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>

        <div id="createAlertRuleModal" class="modal" role="dialog" aria-labelledby="createAlertRuleModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createAlertRuleModalTitle" data-i18n="admin.mon.modalCreateRule">Create Alert Rule</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('createAlertRuleModal')">&times;</button>
            </div>
            <form id="createAlertRuleForm" onsubmit="handleCreateAlertRule(event)">
              <div class="form-group">
                <label for="alertRuleName"><span data-i18n="admin.mon.lbl.ruleName">Rule Name</span> <span class="required">*</span></label>
                <input type="text" id="alertRuleName" name="name" required data-i18n-placeholder="admin.mon.ph.ruleName" placeholder="high-cpu-alert" />
              </div>
              <div class="form-group">
                <label for="alertRuleMetric"><span data-i18n="admin.mon.lbl.metricName">Metric Name</span> <span class="required">*</span></label>
                <input type="text" id="alertRuleMetric" name="metric" required data-i18n-placeholder="admin.mon.ph.metric" placeholder="cpu_usage" />
              </div>
              <div class="form-group">
                <label for="alertRuleOperator"><span data-i18n="admin.mon.lbl.operator">Operator</span> <span class="required">*</span></label>
                <select id="alertRuleOperator" name="operator" required>
                  <option value=">" data-i18n="admin.mon.op.gt">Greater than (&gt;)</option>
                  <option value="<" data-i18n="admin.mon.op.lt">Less than (&lt;)</option>
                  <option value=">=" data-i18n="admin.mon.op.ge">Greater or equal (&gt;=)</option>
                  <option value="<=" data-i18n="admin.mon.op.le">Less or equal (&lt;=)</option>
                  <option value="==" data-i18n="admin.mon.op.eq">Equal (==)</option>
                </select>
              </div>
              <div class="form-group">
                <label for="alertRuleThreshold"><span data-i18n="admin.mon.lbl.threshold">Threshold</span> <span class="required">*</span></label>
                <input type="number" id="alertRuleThreshold" name="threshold" required step="0.1" placeholder="90.0" />
              </div>
              <div class="form-group">
                <label for="alertRuleSeverity"><span data-i18n="admin.mon.lbl.severity">Severity</span> <span class="required">*</span></label>
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
                  <span data-i18n="admin.mon.lbl.ruleEnabled">Enabled</span>
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createAlertRuleModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.create">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
