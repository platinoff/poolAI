//! System Configuration page
//!
//! Provides system configuration interface with tabs for different settings.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// System configuration page
pub async fn admin_config() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    function showTab(tabName) {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadConfigTab(tabName);
    }
    
    async function loadConfigTab(tabName) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      try {
        adminShowLoading('config-content', T('admin.cfg.loading', 'Loading configuration…'));
        const config = await fetchJson('/api/v1/config');
        renderConfigTab(tabName, config);
      } catch (e) {
        adminShowInlineError('config-content', e);
      }
    }
    
    function renderConfigTab(tabName, config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      switch(tabName) {
        case 'general':
          renderGeneralConfig(config);
          break;
        case 'performance':
          renderPerformanceConfig(config);
          break;
        case 'security':
          renderSecurityConfig(config);
          break;
        case 'monitoring':
          renderMonitoringConfig(config);
          break;
        case 'gpu':
          renderGpuConfig(config);
          break;
        case 'health':
          renderHealthConfig(config);
          break;
        default:
          el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.cfg.unknownTab', 'Unknown tab: ') + tabName) + '</div>';
      }
    }
    
    function renderGeneralConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="generalConfigForm" onsubmit="handleSaveConfig(event, 'general')">
          <div class="form-group">
            <label for="configName">${escapeHtml(T('admin.cfg.gen.systemName', 'System Name'))}</label>
            <input type="text" id="configName" name="name" value="${escapeHtml(config.system?.name || '')}" required />
          </div>
          <div class="form-group">
            <label for="configLogLevel">${escapeHtml(T('admin.cfg.gen.logLevel', 'Log Level'))}</label>
            <select id="configLogLevel" name="log_level" required>
              <option value="trace" ${config.system?.log_level === 'trace' ? 'selected' : ''}>${escapeHtml(T('admin.cfg.log.trace', 'Trace'))}</option>
              <option value="debug" ${config.system?.log_level === 'debug' ? 'selected' : ''}>${escapeHtml(T('admin.cfg.log.debug', 'Debug'))}</option>
              <option value="info" ${config.system?.log_level === 'info' ? 'selected' : ''}>${escapeHtml(T('admin.cfg.log.info', 'Info'))}</option>
              <option value="warn" ${config.system?.log_level === 'warn' ? 'selected' : ''}>${escapeHtml(T('admin.cfg.log.warn', 'Warn'))}</option>
              <option value="error" ${config.system?.log_level === 'error' ? 'selected' : ''}>${escapeHtml(T('admin.cfg.log.error', 'Error'))}</option>
            </select>
          </div>
          <div class="form-group">
            <label for="configMaxWorkers">${escapeHtml(T('admin.cfg.gen.maxWorkers', 'Max Workers'))}</label>
            <input type="number" id="configMaxWorkers" name="max_workers" value="${config.system?.max_workers || 16}" min="1" max="1024" required />
          </div>
          <div class="form-group">
            <label for="configQueueSize">${escapeHtml(T('admin.cfg.gen.queueSize', 'Queue Size'))}</label>
            <input type="number" id="configQueueSize" name="queue_size" value="${config.system?.queue_size || 2000}" min="1" max="100000" required />
          </div>
          <div class="form-group">
            <label for="configMetricsInterval">${escapeHtml(T('admin.cfg.gen.metricsInterval', 'Metrics Interval (seconds)'))}</label>
            <input type="number" id="configMetricsInterval" name="metrics_interval" value="${config.system?.metrics_interval || 10}" min="1" max="3600" required />
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    function renderPerformanceConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="performanceConfigForm" onsubmit="handleSaveConfig(event, 'performance')">
          <div class="form-group">
            <label for="configPoolMaxWorkers">${escapeHtml(T('admin.cfg.perf.poolMaxWorkers', 'Pool Max Workers'))}</label>
            <input type="number" id="configPoolMaxWorkers" name="max_workers" value="${config.pool?.max_workers || 16}" min="1" max="1024" required />
          </div>
          <div class="form-group">
            <label for="configPoolQueueSize">${escapeHtml(T('admin.cfg.perf.poolQueue', 'Pool Queue Size'))}</label>
            <input type="number" id="configPoolQueueSize" name="queue_size" value="${config.pool?.queue_size || 2000}" min="1" max="100000" required />
          </div>
          <div class="form-group">
            <label for="configPoolAutoScaling">
              <input type="checkbox" id="configPoolAutoScaling" name="auto_scaling" ${config.pool?.auto_scaling ? 'checked' : ''} />
              ${escapeHtml(T('admin.cfg.perf.autoScaling', 'Auto Scaling'))}
            </label>
          </div>
          <div class="form-group">
            <label for="configPoolScalingThreshold">${escapeHtml(T('admin.cfg.perf.scalingThreshold', 'Scaling Threshold (0.0-1.0)'))}</label>
            <input type="number" id="configPoolScalingThreshold" name="scaling_threshold" value="${config.pool?.scaling_threshold || 0.8}" min="0" max="1" step="0.1" required />
          </div>
          <div class="form-group">
            <label for="configPoolRequestTimeout">${escapeHtml(T('admin.cfg.perf.requestTimeout', 'Request Timeout (seconds)'))}</label>
            <input type="number" id="configPoolRequestTimeout" name="request_timeout" value="${config.pool?.request_timeout || 30}" min="1" max="3600" required />
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    function renderSecurityConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="securityConfigForm" onsubmit="handleSaveConfig(event, 'security')">
          <div class="form-group">
            <label for="configHttpsEnabled">
              <input type="checkbox" id="configHttpsEnabled" name="enabled" ${config.https?.enabled ? 'checked' : ''} />
              ${escapeHtml(T('admin.cfg.https.enable', 'Enable HTTPS'))}
            </label>
          </div>
          <div class="form-group">
            <label for="configHttpsCertPath">${escapeHtml(T('admin.cfg.https.certPath', 'Certificate Path'))}</label>
            <input type="text" id="configHttpsCertPath" name="cert_path" value="${escapeHtml(config.https?.cert_path || '')}" />
          </div>
          <div class="form-group">
            <label for="configHttpsKeyPath">${escapeHtml(T('admin.cfg.https.keyPath', 'Key Path'))}</label>
            <input type="text" id="configHttpsKeyPath" name="key_path" value="${escapeHtml(config.https?.key_path || '')}" />
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    function renderMonitoringConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="monitoringConfigForm" onsubmit="handleSaveConfig(event, 'monitoring')">
          <div class="form-group">
            <label for="configMetricsIntervalMon">${escapeHtml(T('admin.cfg.mon.metricsInterval', 'Metrics Interval (seconds)'))}</label>
            <input type="number" id="configMetricsIntervalMon" name="metrics_interval" value="${config.monitoring?.metrics_interval || 10}" min="1" max="3600" required />
          </div>
          <div class="form-group">
            <label for="configAlertThreshold">${escapeHtml(T('admin.cfg.mon.alertThreshold', 'Alert Threshold (0.0-1.0)'))}</label>
            <input type="number" id="configAlertThreshold" name="alert_threshold" value="${config.monitoring?.alert_threshold || 0.8}" min="0" max="1" step="0.1" required />
          </div>
          <div class="form-group">
            <label for="configRetentionDays">${escapeHtml(T('admin.cfg.mon.retentionDays', 'Retention Days'))}</label>
            <input type="number" id="configRetentionDays" name="retention_days" value="${config.monitoring?.retention_days || 30}" min="1" max="365" required />
          </div>
          <div class="form-group">
            <label for="configDetailedLogging">
              <input type="checkbox" id="configDetailedLogging" name="detailed_logging" ${config.monitoring?.detailed_logging ? 'checked' : ''} />
              ${escapeHtml(T('admin.cfg.mon.detailedLogging', 'Detailed Logging'))}
            </label>
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    function renderGpuConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="gpuConfigForm" onsubmit="handleSaveConfig(event, 'gpu')">
          <div class="form-group">
            <label for="configGpuEnabled">
              <input type="checkbox" id="configGpuEnabled" name="enabled" ${config.gpu?.enabled ? 'checked' : ''} />
              ${escapeHtml(T('admin.cfg.gpu.enable', 'Enable GPU'))}
            </label>
          </div>
          <div class="form-group">
            <label for="configGpuMemoryLimit">${escapeHtml(T('admin.cfg.gpu.memLimit', 'GPU Memory Limit (MB)'))}</label>
            <input type="number" id="configGpuMemoryLimit" name="memory_limit" value="${config.gpu?.memory_limit || 8192}" min="256" max="131072" required />
          </div>
          <div class="form-group">
            <label for="configGpuTemperatureLimit">${escapeHtml(T('admin.cfg.gpu.tempLimit', 'Temperature Limit (°C)'))}</label>
            <input type="number" id="configGpuTemperatureLimit" name="temperature_limit" value="${config.gpu?.temperature_limit || 85}" min="50" max="120" required />
          </div>
          <div class="form-group">
            <label for="configGpuPowerLimit">${escapeHtml(T('admin.cfg.gpu.powerLimit', 'Power Limit (Watts)'))}</label>
            <input type="number" id="configGpuPowerLimit" name="power_limit" value="${config.gpu?.power_limit || 200}" min="50" max="1000" required />
          </div>
          <div class="form-group">
            <label for="configGpuCount">${escapeHtml(T('admin.cfg.gpu.count', 'GPU Count'))}</label>
            <input type="number" id="configGpuCount" name="gpu_count" value="${config.gpu?.gpu_count || 1}" min="1" max="16" required />
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    function renderHealthConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="healthConfigForm" onsubmit="handleSaveConfig(event, 'health')">
          <div class="form-group">
            <label for="configExpectedWorkers">${escapeHtml(T('admin.cfg.health.expectedWorkers', 'Expected Workers'))}</label>
            <input type="number" id="configExpectedWorkers" name="expected_workers" value="${config.health?.expected_workers || 8}" min="1" max="1024" required />
            <small class="form-hint">${escapeHtml(T('admin.cfg.health.hint', 'Number of workers expected for health checks'))}</small>
          </div>
          <button type="submit" class="btn btn-primary">${escapeHtml(T('admin.cfg.saveBtn', 'Save Configuration'))}</button>
        </form>
      `;
    }
    
    async function handleSaveConfig(event, tabName) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification(T('err.insufficientAdmin', 'Insufficient permissions. Admin role required.'), 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = T('admin.cfg.saving', 'Saving…');
      
      try {
        const currentConfig = await fetchJson('/api/v1/config');
        
        let updatedConfig = { ...currentConfig };
        
        if (tabName === 'general') {
          updatedConfig.system = {
            ...updatedConfig.system,
            name: document.getElementById('configName').value,
            log_level: document.getElementById('configLogLevel').value,
            max_workers: parseInt(document.getElementById('configMaxWorkers').value, 10),
            queue_size: parseInt(document.getElementById('configQueueSize').value, 10),
            metrics_interval: parseInt(document.getElementById('configMetricsInterval').value, 10),
            version: updatedConfig.system?.version || '0.1.0'
          };
        } else if (tabName === 'performance') {
          updatedConfig.pool = {
            ...updatedConfig.pool,
            max_workers: parseInt(document.getElementById('configPoolMaxWorkers').value, 10),
            queue_size: parseInt(document.getElementById('configPoolQueueSize').value, 10),
            auto_scaling: document.getElementById('configPoolAutoScaling').checked,
            scaling_threshold: parseFloat(document.getElementById('configPoolScalingThreshold').value),
            request_timeout: parseInt(document.getElementById('configPoolRequestTimeout').value, 10)
          };
        } else if (tabName === 'security') {
          updatedConfig.https = {
            ...updatedConfig.https,
            enabled: document.getElementById('configHttpsEnabled').checked,
            cert_path: document.getElementById('configHttpsCertPath').value || null,
            key_path: document.getElementById('configHttpsKeyPath').value || null
          };
        } else if (tabName === 'monitoring') {
          updatedConfig.monitoring = {
            ...updatedConfig.monitoring,
            metrics_interval: parseInt(document.getElementById('configMetricsIntervalMon').value, 10),
            alert_threshold: parseFloat(document.getElementById('configAlertThreshold').value),
            retention_days: parseInt(document.getElementById('configRetentionDays').value, 10),
            detailed_logging: document.getElementById('configDetailedLogging').checked
          };
        } else if (tabName === 'gpu') {
          updatedConfig.gpu = {
            ...updatedConfig.gpu,
            enabled: document.getElementById('configGpuEnabled').checked,
            memory_limit: parseInt(document.getElementById('configGpuMemoryLimit').value, 10),
            temperature_limit: parseInt(document.getElementById('configGpuTemperatureLimit').value, 10),
            power_limit: parseInt(document.getElementById('configGpuPowerLimit').value, 10),
            gpu_count: parseInt(document.getElementById('configGpuCount').value, 10)
          };
        } else if (tabName === 'health') {
          updatedConfig.health = {
            ...updatedConfig.health,
            expected_workers: parseInt(document.getElementById('configExpectedWorkers').value, 10)
          };
        }
        
        await fetchJson('/api/v1/config', {
          method: 'PUT',
          body: JSON.stringify(updatedConfig)
        });
        
        showNotification(T('admin.cfg.savedOk', 'Configuration saved successfully'), 'success');
        loadConfigTab(tabName);
      } catch (e) {
        showNotification(T('admin.cfg.saveErr', 'Error saving configuration: ') + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => showTab(tab.dataset.tab));
    });
    
    loadConfigTab('general');
    "#;

    admin_layout(
        "admin.page.config",
        "System Configuration",
        r#"
        <div class="admin-section">
          <div class="admin-tabs">
            <button type="button" class="tab active" data-tab="general" data-i18n="admin.cfg.tab.general">General</button>
            <button type="button" class="tab" data-tab="performance" data-i18n="admin.cfg.tab.performance">Performance</button>
            <button type="button" class="tab" data-tab="gpu" data-i18n="admin.cfg.tab.gpu">GPU</button>
            <button type="button" class="tab" data-tab="security" data-i18n="admin.cfg.tab.security">Security</button>
            <button type="button" class="tab" data-tab="monitoring" data-i18n="admin.cfg.tab.monitoring">Monitoring</button>
            <button type="button" class="tab" data-tab="health" data-i18n="admin.cfg.tab.health">Health</button>
          </div>
          <div id="config-content"></div>
        </div>
        "#,
        script,
    )
}
