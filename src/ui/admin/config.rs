//! System Configuration page
//!
//! Provides system configuration interface with tabs for different settings.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// System configuration page
pub async fn admin_config() -> Html<String> {
    let script = r#"
    function showTab(tabName) {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadConfigTab(tabName);
    }
    
    async function loadConfigTab(tabName) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      try {
        const config = await fetchJson('/api/v1/config');
        renderConfigTab(tabName, config);
      } catch (e) {
        el.innerHTML = '<div class="muted">Error loading configuration: ' + e.message + '</div>';
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
          el.innerHTML = '<div class="muted">Unknown tab: ' + tabName + '</div>';
      }
    }
    
    function renderGeneralConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="generalConfigForm" onsubmit="handleSaveConfig(event, 'general')">
          <div class="form-group">
            <label for="configName">System Name</label>
            <input type="text" id="configName" name="name" value="${config.system?.name || ''}" required />
          </div>
          <div class="form-group">
            <label for="configLogLevel">Log Level</label>
            <select id="configLogLevel" name="log_level" required>
              <option value="trace" ${config.system?.log_level === 'trace' ? 'selected' : ''}>Trace</option>
              <option value="debug" ${config.system?.log_level === 'debug' ? 'selected' : ''}>Debug</option>
              <option value="info" ${config.system?.log_level === 'info' ? 'selected' : ''}>Info</option>
              <option value="warn" ${config.system?.log_level === 'warn' ? 'selected' : ''}>Warn</option>
              <option value="error" ${config.system?.log_level === 'error' ? 'selected' : ''}>Error</option>
            </select>
          </div>
          <div class="form-group">
            <label for="configMaxWorkers">Max Workers</label>
            <input type="number" id="configMaxWorkers" name="max_workers" value="${config.system?.max_workers || 16}" min="1" max="1024" required />
          </div>
          <div class="form-group">
            <label for="configQueueSize">Queue Size</label>
            <input type="number" id="configQueueSize" name="queue_size" value="${config.system?.queue_size || 2000}" min="1" max="100000" required />
          </div>
          <div class="form-group">
            <label for="configMetricsInterval">Metrics Interval (seconds)</label>
            <input type="number" id="configMetricsInterval" name="metrics_interval" value="${config.system?.metrics_interval || 10}" min="1" max="3600" required />
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
        </form>
      `;
    }
    
    function renderPerformanceConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="performanceConfigForm" onsubmit="handleSaveConfig(event, 'performance')">
          <div class="form-group">
            <label for="configPoolMaxWorkers">Pool Max Workers</label>
            <input type="number" id="configPoolMaxWorkers" name="max_workers" value="${config.pool?.max_workers || 16}" min="1" max="1024" required />
          </div>
          <div class="form-group">
            <label for="configPoolQueueSize">Pool Queue Size</label>
            <input type="number" id="configPoolQueueSize" name="queue_size" value="${config.pool?.queue_size || 2000}" min="1" max="100000" required />
          </div>
          <div class="form-group">
            <label for="configPoolAutoScaling">
              <input type="checkbox" id="configPoolAutoScaling" name="auto_scaling" ${config.pool?.auto_scaling ? 'checked' : ''} />
              Auto Scaling
            </label>
          </div>
          <div class="form-group">
            <label for="configPoolScalingThreshold">Scaling Threshold (0.0-1.0)</label>
            <input type="number" id="configPoolScalingThreshold" name="scaling_threshold" value="${config.pool?.scaling_threshold || 0.8}" min="0" max="1" step="0.1" required />
          </div>
          <div class="form-group">
            <label for="configPoolRequestTimeout">Request Timeout (seconds)</label>
            <input type="number" id="configPoolRequestTimeout" name="request_timeout" value="${config.pool?.request_timeout || 30}" min="1" max="3600" required />
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
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
              Enable HTTPS
            </label>
          </div>
          <div class="form-group">
            <label for="configHttpsCertPath">Certificate Path</label>
            <input type="text" id="configHttpsCertPath" name="cert_path" value="${config.https?.cert_path || ''}" />
          </div>
          <div class="form-group">
            <label for="configHttpsKeyPath">Key Path</label>
            <input type="text" id="configHttpsKeyPath" name="key_path" value="${config.https?.key_path || ''}" />
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
        </form>
      `;
    }
    
    function renderMonitoringConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="monitoringConfigForm" onsubmit="handleSaveConfig(event, 'monitoring')">
          <div class="form-group">
            <label for="configMetricsInterval">Metrics Interval (seconds)</label>
            <input type="number" id="configMetricsInterval" name="metrics_interval" value="${config.monitoring?.metrics_interval || 10}" min="1" max="3600" required />
          </div>
          <div class="form-group">
            <label for="configAlertThreshold">Alert Threshold (0.0-1.0)</label>
            <input type="number" id="configAlertThreshold" name="alert_threshold" value="${config.monitoring?.alert_threshold || 0.8}" min="0" max="1" step="0.1" required />
          </div>
          <div class="form-group">
            <label for="configRetentionDays">Retention Days</label>
            <input type="number" id="configRetentionDays" name="retention_days" value="${config.monitoring?.retention_days || 30}" min="1" max="365" required />
          </div>
          <div class="form-group">
            <label for="configDetailedLogging">
              <input type="checkbox" id="configDetailedLogging" name="detailed_logging" ${config.monitoring?.detailed_logging ? 'checked' : ''} />
              Detailed Logging
            </label>
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
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
              Enable GPU
            </label>
          </div>
          <div class="form-group">
            <label for="configGpuMemoryLimit">GPU Memory Limit (MB)</label>
            <input type="number" id="configGpuMemoryLimit" name="memory_limit" value="${config.gpu?.memory_limit || 8192}" min="256" max="131072" required />
          </div>
          <div class="form-group">
            <label for="configGpuTemperatureLimit">Temperature Limit (°C)</label>
            <input type="number" id="configGpuTemperatureLimit" name="temperature_limit" value="${config.gpu?.temperature_limit || 85}" min="50" max="120" required />
          </div>
          <div class="form-group">
            <label for="configGpuPowerLimit">Power Limit (Watts)</label>
            <input type="number" id="configGpuPowerLimit" name="power_limit" value="${config.gpu?.power_limit || 200}" min="50" max="1000" required />
          </div>
          <div class="form-group">
            <label for="configGpuCount">GPU Count</label>
            <input type="number" id="configGpuCount" name="gpu_count" value="${config.gpu?.gpu_count || 1}" min="1" max="16" required />
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
        </form>
      `;
    }
    
    function renderHealthConfig(config) {
      const el = document.getElementById('config-content');
      if (!el) return;
      
      el.innerHTML = `
        <form id="healthConfigForm" onsubmit="handleSaveConfig(event, 'health')">
          <div class="form-group">
            <label for="configExpectedWorkers">Expected Workers</label>
            <input type="number" id="configExpectedWorkers" name="expected_workers" value="${config.health?.expected_workers || 8}" min="1" max="1024" required />
            <small class="form-hint">Number of workers expected for health checks</small>
          </div>
          <button type="submit" class="btn btn-primary">Save Configuration</button>
        </form>
      `;
    }
    
    async function handleSaveConfig(event, tabName) {
      event.preventDefault();
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Saving...';
      
      try {
        // Get current config
        const currentConfig = await fetchJson('/api/v1/config');
        
        // Update based on tab
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
            metrics_interval: parseInt(document.getElementById('configMetricsInterval').value, 10),
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
        
        showNotification('Configuration saved successfully', 'success');
        loadConfigTab(tabName);
      } catch (e) {
        showNotification('Error saving configuration: ' + e.message, 'error');
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
        "System Configuration",
        r#"
        <div class="admin-section">
          <div class="admin-tabs">
            <button class="tab active" data-tab="general">General</button>
            <button class="tab" data-tab="performance">Performance</button>
            <button class="tab" data-tab="gpu">GPU</button>
            <button class="tab" data-tab="security">Security</button>
            <button class="tab" data-tab="monitoring">Monitoring</button>
            <button class="tab" data-tab="health">Health</button>
          </div>
          <div id="config-content"></div>
        </div>
        "#,
        script,
    )
}
