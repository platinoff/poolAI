//! Monitoring Dashboard page
//!
//! PH-S220: monitoring page uses slim `admin_layout_monitoring` + `admin_monitoring_patch`.

use crate::ui::admin::admin_layout_monitoring;
use axum::response::Html;

/// Monitoring dashboard page
pub async fn admin_monitoring() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadMonitoring() {
      adminShowLoading('monitoring-content', T('admin.mon.loading', 'Loading monitoring…'));
      try {
        const [alerts, dashboards, mlPipelines] = await Promise.all([
          poolaiFetchMonitoringAlerts({ limit: 20, acknowledged: false }),
          fetchJson(poolaiMonitoringDashboardsUrl()),
          poolaiFetchMlPipelines(),
        ]);
        await renderMonitoring(alerts, dashboards, mlPipelines);
      } catch (e) {
        adminShowInlineError('monitoring-content', e);
        showNotification(T('admin.mon.errLoad', 'Error loading monitoring: ') + e.message, 'error');
      }
    }
    
    async function runMlPipelineDemo() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      const btn = document.getElementById('ml-demo-btn');
      if (btn) {
        btn.disabled = true;
        btn.textContent = T('admin.mon.mlDemoRunning', 'Running demo…');
      }
      try {
        await poolaiRunMlPipelineDemo();
        showNotification(T('admin.mon.mlDemoOk', 'ML demo pipeline completed'), 'success');
        loadMonitoring();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        if (btn) {
          btn.disabled = false;
          btn.textContent = T('admin.mon.mlDemoBtn', 'Run ML Demo');
        }
      }
    }
    
    
    async function loadAlertRules() {
      try {
        return await poolaiFetchAlertRules();
      } catch (e) {
        console.error('Error loading alert rules:', e);
        return [];
      }
    }
    
    async function renderMonitoring(alerts, dashboards, mlPipelines) {
      const el = document.getElementById('monitoring-content');
      if (!el) return;
      
      const chartsHtml = await poolaiRenderMetricsChartGrid(
        ['cpu_usage', 'memory_usage', 'request_rate'],
        { hours: 24, title: T('admin.mon.vizTitle', 'Metrics Visualization (Last 24 Hours)') }
      );

      const mlPanelHtml =
        mlPipelines === null
          ? '<div class="admin-card ml-pipeline-metrics-panel"><h3>' +
            escapeHtml(T('admin.mon.mlTitle', 'ML Pipeline Step Metrics')) +
            '</h3>' +
            adminEmptyStateHtml(T('admin.mon.mlUnavailable', 'ML pipeline API unavailable'), {
              hint: T('admin.mon.mlUnavailableHint', 'Build with enterprise + ml features to enable AI/ML pipelines.'),
              icon: '🧠',
            }) +
            '</div>'
          : poolaiRenderMlPipelineMetricsPanel(mlPipelines, {
              title: T('admin.mon.mlTitle', 'ML Pipeline Step Metrics'),
            });
      
      const alertsHtml = poolaiRenderMonitoringAlertsPanel(alerts, {
        na: T('admin.na', 'N/A'),
        ack: T('admin.mon.statusAck', 'Acknowledged'),
        active: T('admin.mon.statusActiveLbl', 'Active'),
        ackBtn: T('admin.mon.ackBtn', 'Acknowledge'),
        severity: T('admin.mon.col.severity', 'Severity'),
        metric: T('admin.mon.col.metric', 'Metric'),
        current: T('admin.mon.col.currentVal', 'Current Value'),
        threshold: T('admin.mon.col.threshold', 'Threshold'),
        triggered: T('admin.mon.col.triggered', 'Triggered'),
        status: T('admin.mon.col.statusCol', 'Status'),
        actions: T('admin.mon.col.actions', 'Actions'),
        tableAria: T('admin.mon.activeAlertsTitle', 'Active Alerts'),
        empty: T('admin.mon.noAlerts', 'No active alerts'),
      });
      
      const dashboardsHtml = dashboards.length === 0
        ? adminEmptyStateHtml(T('admin.mon.noDashboards', 'No dashboards created'), { icon: '📊' })
        : `
          <div class="admin-table-container"><table class="admin-table" aria-label="${escapeHtml(T('admin.mon.dashboardsTitle', 'Dashboards'))}">
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
          </table></div>
        `;
      
      el.innerHTML = `
        ${chartsHtml}
        ${mlPanelHtml}
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
              <div class="admin-table-container"><table class="admin-table" aria-label="${escapeHtml(T('admin.mon.alertRulesTitle', 'Alert Rules'))}">
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
              </table></div>
            </div>
          `;
          el.innerHTML += rulesHtml;
        }
        adminInitTablesIn(el);
      }).catch(e => {
        console.error('Error loading alert rules:', e);
        adminInitTablesIn(el);
      });
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
        
        await fetchJson(poolaiMonitoringDashboardsUrl(), {
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
        
        await fetchJson(poolaiAlertRulesUrl(), {
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
        await fetchJson(poolaiMonitoringAlertAcknowledgeUrl(id), {
          method: 'POST'
        });
        showNotification(T('admin.mon.alertAckOk', 'Alert acknowledged'), 'success');
        loadMonitoring();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    loadMonitoring();
    poolaiStartMetricsPolling(loadMonitoring, 5000);
    "#;

    admin_layout_monitoring(
        "admin.page.monitoring",
        "Monitoring Dashboard",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.mon.section">Monitoring</h2>
            <button type="button" class="btn btn-secondary" id="ml-demo-btn" onclick="runMlPipelineDemo()" data-i18n="admin.mon.mlDemoBtn" data-i18n-aria="admin.mon.mlDemoBtn">Run ML Demo</button>
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

#[tokio::test]
async fn admin_monitoring_page_slim_monitoring_i18n_patch_ph_s220() {
    let html = admin_monitoring().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.monitoring""#));
    assert!(html.contains(r#""admin.mon.mlTitle""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.updatesCompat.section""#));
}
