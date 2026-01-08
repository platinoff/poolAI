//! Admin Panel module
//!
//! Provides comprehensive administrative interface with full system management capabilities.
//!
//! # Features
//!
//! - **System Overview**: Real-time system status, health, and metrics
//! - **Tenant Management**: Create, update, delete tenants, manage quotas
//! - **Security Management**: OAuth2/SAML providers, security policies
//! - **Audit Logs**: View and query audit events
//! - **Monitoring Dashboard**: Real-time metrics, alerts, custom dashboards
//! - **VM Management**: Full VM instance lifecycle management
//! - **Worker Management**: Worker pool configuration and monitoring
//! - **Library Management**: Model library administration
//! - **RAID Management**: Artifact storage and replication
//! - **User Management**: User accounts, roles, permissions
//! - **System Configuration**: Advanced settings and policies
//!
//! # Routes
//!
//! - `/ui/admin` - Admin dashboard home
//! - `/ui/admin/tenants` - Tenant management
//! - `/ui/admin/security` - Security settings
//! - `/ui/admin/audit` - Audit logs viewer
//! - `/ui/admin/monitoring` - Monitoring dashboard
//! - `/ui/admin/vm` - VM management
//! - `/ui/admin/workers` - Worker management
//! - `/ui/admin/libs` - Library management
//! - `/ui/admin/raid` - RAID management
//! - `/ui/admin/users` - User management
//! - `/ui/admin/config` - System configuration

use axum::{response::Html, routing::get, Router};

/// Admin panel routes
pub fn create_admin_routes() -> Router {
    Router::new()
        .route("/admin", get(admin_dashboard))
        .route("/admin/tenants", get(admin_tenants))
        .route("/admin/security", get(admin_security))
        .route("/admin/audit", get(admin_audit))
        .route("/admin/monitoring", get(admin_monitoring))
        .route("/admin/vm", get(admin_vm))
        .route("/admin/workers", get(admin_workers))
        .route("/admin/libs", get(admin_libs))
        .route("/admin/raid", get(admin_raid))
        .route("/admin/users", get(admin_users))
        .route("/admin/config", get(admin_config))
}

/// Admin dashboard home page
async fn admin_dashboard() -> Html<String> {
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

/// Tenant management page
async fn admin_tenants() -> Html<String> {
    let script = r#"
    async function loadTenants() {
      try {
        const tenants = await fetchJson('/api/enterprise/tenants');
        renderTenants(tenants);
      } catch (e) {
        showNotification('Error loading tenants: ' + e.message, 'error');
      }
    }
    
    function renderTenants(tenants) {
      const el = document.getElementById('tenants-list');
      if (!el) return;
      if (!tenants || tenants.length === 0) {
        el.innerHTML = '<div class="muted">No tenants found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>ID</th>
              <th>Status</th>
              <th>Resources</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${tenants.map(t => `
              <tr>
                <td>${t.name}</td>
                <td><code>${t.id}</code></td>
                <td><span class="status-badge ${t.config.active ? 'active' : 'inactive'}">${t.config.active ? 'Active' : 'Inactive'}</span></td>
                <td>Workers: ${t.usage.workers}/${t.config.max_workers || '∞'}, Memory: ${t.usage.memory_mb}MB/${t.config.max_memory_mb || '∞'}MB</td>
                <td>
                  <button class="btn" onclick="editTenant('${t.id}')">Edit</button>
                  <button class="btn btn-danger" onclick="deleteTenant('${t.id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function showCreateTenantModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createTenantModal');
    }
    
    async function handleCreateTenant(event) {
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
          name: document.getElementById('tenantName').value,
          config: {
            max_workers: document.getElementById('tenantMaxWorkers').value ? parseInt(document.getElementById('tenantMaxWorkers').value, 10) : null,
            max_memory_mb: document.getElementById('tenantMaxMemory').value ? parseInt(document.getElementById('tenantMaxMemory').value, 10) : null,
            max_cpu_cores: document.getElementById('tenantMaxCpuCores').value ? parseInt(document.getElementById('tenantMaxCpuCores').value, 10) : null,
            max_storage_mb: document.getElementById('tenantMaxStorage').value ? parseInt(document.getElementById('tenantMaxStorage').value, 10) : null,
            max_vm_instances: document.getElementById('tenantMaxVmInstances').value ? parseInt(document.getElementById('tenantMaxVmInstances').value, 10) : null,
            active: document.getElementById('tenantActive').checked
          }
        };
        
        // Remove null fields
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        const result = await fetchJson('/api/enterprise/tenants', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Tenant created successfully', 'success');
        hideModal('createTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      try {
        const tenant = await fetchJson(`/api/enterprise/tenants/${id}`);
        
        // Populate edit form
        document.getElementById('editTenantId').value = tenant.id;
        document.getElementById('editTenantName').value = tenant.name;
        document.getElementById('editTenantMaxWorkers').value = tenant.config.max_workers || '';
        document.getElementById('editTenantMaxMemory').value = tenant.config.max_memory_mb || '';
        document.getElementById('editTenantMaxCpuCores').value = tenant.config.max_cpu_cores || '';
        document.getElementById('editTenantMaxStorage').value = tenant.config.max_storage_mb || '';
        document.getElementById('editTenantMaxVmInstances').value = tenant.config.max_vm_instances || '';
        document.getElementById('editTenantActive').checked = tenant.config.active;
        
        showModal('editTenantModal');
      } catch (e) {
        showNotification('Error loading tenant: ' + e.message, 'error');
      }
    }
    
    async function handleEditTenant(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      const tenantId = document.getElementById('editTenantId').value;
      
      btn.disabled = true;
      btn.textContent = 'Updating...';
      
      try {
        const payload = {
          name: document.getElementById('editTenantName').value,
          config: {
            max_workers: document.getElementById('editTenantMaxWorkers').value ? parseInt(document.getElementById('editTenantMaxWorkers').value, 10) : null,
            max_memory_mb: document.getElementById('editTenantMaxMemory').value ? parseInt(document.getElementById('editTenantMaxMemory').value, 10) : null,
            max_cpu_cores: document.getElementById('editTenantMaxCpuCores').value ? parseInt(document.getElementById('editTenantMaxCpuCores').value, 10) : null,
            max_storage_mb: document.getElementById('editTenantMaxStorage').value ? parseInt(document.getElementById('editTenantMaxStorage').value, 10) : null,
            max_vm_instances: document.getElementById('editTenantMaxVmInstances').value ? parseInt(document.getElementById('editTenantMaxVmInstances').value, 10) : null,
            active: document.getElementById('editTenantActive').checked
          }
        };
        
        // Remove null fields
        Object.keys(payload.config).forEach(key => {
          if (payload.config[key] === null) {
            delete payload.config[key];
          }
        });
        
        const result = await fetchJson(`/api/enterprise/tenants/${tenantId}`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Tenant updated successfully', 'success');
        hideModal('editTenantModal');
        form.reset();
        
        setTimeout(() => {
          loadTenants();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteTenant(id) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Are you sure you want to delete this tenant? This action cannot be undone.')) {
        return;
      }
      
      try {
        await fetchJson(`/api/enterprise/tenants/${id}`, { method: 'DELETE' });
        showNotification('Tenant deleted successfully', 'success');
        loadTenants();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    loadTenants();
    "#;

    admin_layout(
        "Tenant Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Tenants</h2>
            <button class="btn btn-primary" onclick="showCreateTenantModal()" aria-label="Create new tenant">Create Tenant</button>
          </div>
          <div id="tenants-list"></div>
        </div>
        
        <!-- Create Tenant Modal -->
        <div id="createTenantModal" class="modal" role="dialog" aria-labelledby="createTenantModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createTenantModalTitle">Create Tenant</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createTenantModal')">&times;</button>
            </div>
            <form id="createTenantForm" onsubmit="handleCreateTenant(event)">
              <div class="form-group">
                <label for="tenantName">Tenant Name <span class="required">*</span></label>
                <input type="text" id="tenantName" name="name" required placeholder="tenant-abc" />
              </div>
              <div class="form-group">
                <label for="tenantMaxWorkers">Max Workers</label>
                <input type="number" id="tenantMaxWorkers" name="max_workers" min="0" placeholder="10" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxMemory">Max Memory (MB)</label>
                <input type="number" id="tenantMaxMemory" name="max_memory_mb" min="0" placeholder="1024" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxCpuCores">Max CPU Cores</label>
                <input type="number" id="tenantMaxCpuCores" name="max_cpu_cores" min="0" placeholder="4" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxStorage">Max Storage (MB)</label>
                <input type="number" id="tenantMaxStorage" name="max_storage_mb" min="0" placeholder="10000" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantMaxVmInstances">Max VM Instances</label>
                <input type="number" id="tenantMaxVmInstances" name="max_vm_instances" min="0" placeholder="5" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="tenantActive">
                  <input type="checkbox" id="tenantActive" name="active" checked />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createTenantModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit Tenant Modal -->
        <div id="editTenantModal" class="modal" role="dialog" aria-labelledby="editTenantModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editTenantModalTitle">Edit Tenant</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editTenantModal')">&times;</button>
            </div>
            <form id="editTenantForm" onsubmit="handleEditTenant(event)">
              <input type="hidden" id="editTenantId" />
              <div class="form-group">
                <label for="editTenantName">Tenant Name <span class="required">*</span></label>
                <input type="text" id="editTenantName" name="name" required />
              </div>
              <div class="form-group">
                <label for="editTenantMaxWorkers">Max Workers</label>
                <input type="number" id="editTenantMaxWorkers" name="max_workers" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxMemory">Max Memory (MB)</label>
                <input type="number" id="editTenantMaxMemory" name="max_memory_mb" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxCpuCores">Max CPU Cores</label>
                <input type="number" id="editTenantMaxCpuCores" name="max_cpu_cores" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxStorage">Max Storage (MB)</label>
                <input type="number" id="editTenantMaxStorage" name="max_storage_mb" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantMaxVmInstances">Max VM Instances</label>
                <input type="number" id="editTenantMaxVmInstances" name="max_vm_instances" min="0" />
                <small class="form-hint">Leave empty for unlimited</small>
              </div>
              <div class="form-group">
                <label for="editTenantActive">
                  <input type="checkbox" id="editTenantActive" name="active" />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editTenantModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Update</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

/// Security management page
async fn admin_security() -> Html<String> {
    let script = r#"
    function showTab(tabName) {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadTabContent(tabName);
    }
    
    async function loadTabContent(tabName) {
      const el = document.getElementById('security-content');
      if (!el) return;
      
      switch(tabName) {
        case 'oauth2':
          el.innerHTML = '<div class="muted">OAuth2 providers management - to be implemented</div>';
          break;
        case 'saml':
          el.innerHTML = '<div class="muted">SAML providers management - to be implemented</div>';
          break;
        case 'policies':
          el.innerHTML = '<div class="muted">Security policies management - to be implemented</div>';
          break;
      }
    }
    
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => showTab(tab.dataset.tab));
    });
    
    loadTabContent('oauth2');
    "#;

    admin_layout(
        "Security Management",
        r#"
        <div class="admin-section">
          <div class="admin-tabs">
            <button class="tab active" data-tab="oauth2">OAuth2 Providers</button>
            <button class="tab" data-tab="saml">SAML Providers</button>
            <button class="tab" data-tab="policies">Security Policies</button>
          </div>
          <div id="security-content"></div>
        </div>
        "#,
        script,
    )
}

/// Audit logs viewer page
async fn admin_audit() -> Html<String> {
    let script = r#"
    async function queryAuditLogs() {
      const level = document.getElementById('audit-level').value;
      const startDate = document.getElementById('audit-start-date').value;
      const endDate = document.getElementById('audit-end-date').value;
      
      const params = new URLSearchParams();
      if (level) params.append('min_level', level);
      if (startDate) params.append('start_time', startDate + 'T00:00:00Z');
      if (endDate) params.append('end_time', endDate + 'T23:59:59Z');
      params.append('limit', '100');
      
      try {
        const events = await fetchJson(`/api/enterprise/audit/events?${params}`);
        renderAuditEvents(events);
      } catch (e) {
        showNotification('Error loading audit logs: ' + e.message, 'error');
      }
    }
    
    function renderAuditEvents(events) {
      const el = document.getElementById('audit-events');
      if (!el) return;
      if (!events || events.length === 0) {
        el.innerHTML = '<div class="muted">No audit events found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>Level</th>
              <th>User</th>
              <th>Action</th>
              <th>Resource</th>
              <th>Result</th>
            </tr>
          </thead>
          <tbody>
            ${events.map(e => `
              <tr>
                <td>${new Date(e.timestamp).toLocaleString()}</td>
                <td><span class="status-badge ${e.level.toLowerCase()}">${e.level}</span></td>
                <td>${e.user_id || '—'}</td>
                <td>${e.action}</td>
                <td>${e.resource_type}${e.resource_id ? ': ' + e.resource_id : ''}</td>
                <td>${e.result}</td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    queryAuditLogs();
    "#;

    admin_layout(
        "Audit Logs",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Audit Events</h2>
            <div class="admin-filters">
              <input type="text" id="audit-search" placeholder="Search..." />
              <select id="audit-level">
                <option value="">All Levels</option>
                <option value="Info">Info</option>
                <option value="Warning">Warning</option>
                <option value="Error">Error</option>
                <option value="Critical">Critical</option>
              </select>
              <input type="date" id="audit-start-date" />
              <input type="date" id="audit-end-date" />
              <button class="btn" onclick="queryAuditLogs()">Query</button>
            </div>
          </div>
          <div id="audit-events"></div>
        </div>
        "#,
        script,
    )
}

/// Monitoring dashboard page
async fn admin_monitoring() -> Html<String> {
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

/// VM management page
async fn admin_vm() -> Html<String> {
    let script = r#"
    async function loadVmInstances() {
      try {
        const instances = await fetchJson('/api/v1/vm/instances');
        renderVmInstances(instances);
      } catch (e) {
        showNotification('Error loading VM instances: ' + e.message, 'error');
      }
    }
    
    function renderVmInstances(instances) {
      const el = document.getElementById('vm-instances');
      if (!el) return;
      if (!instances || instances.length === 0) {
        el.innerHTML = '<div class="muted">No VM instances found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Status</th>
              <th>Resources</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${instances.map(i => `
              <tr>
                <td>${i.name}</td>
                <td><span class="status-badge ${i.status.toLowerCase()}">${i.status}</span></td>
                <td>CPU: ${i.resources.cpu_cores}, Memory: ${i.resources.memory_mb}MB</td>
                <td>
                  <button class="btn" onclick="vmAction('${i.id}', 'start')">Start</button>
                  <button class="btn" onclick="vmAction('${i.id}', 'stop')">Stop</button>
                  <button class="btn btn-danger" onclick="vmAction('${i.id}', 'delete')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function vmAction(id, action) {
      try {
        await fetchJson(`/api/v1/vm/instances/${id}/${action}`, { method: 'POST' });
        showNotification(`VM ${action} successful`, 'success');
        loadVmInstances();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    function showCreateVmModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createVmModal');
    }
    
    async function handleCreateVm(event) {
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
          name: document.getElementById('vmName').value,
          resources: {
            cpu_cores: parseInt(document.getElementById('vmCpuCores').value, 10),
            memory_mb: parseInt(document.getElementById('vmMemoryMb').value, 10),
            gpu_required: document.getElementById('vmGpuRequired').checked
          },
          isolation: document.getElementById('vmIsolation').value
        };
        
        const result = await fetchJson('/api/v1/vm/instances', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('VM instance created successfully', 'success');
        hideModal('createVmModal');
        form.reset();
        
        setTimeout(() => {
          loadVmInstances();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadVmInstances();
    "#;

    admin_layout(
        "VM Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>VM Instances</h2>
            <button class="btn btn-primary" onclick="showCreateVmModal()" aria-label="Create new VM instance">Create VM Instance</button>
          </div>
          <div id="vm-instances"></div>
        </div>
        
        <!-- Create VM Modal -->
        <div id="createVmModal" class="modal" role="dialog" aria-labelledby="createVmModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createVmModalTitle">Create VM Instance</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createVmModal')">&times;</button>
            </div>
            <form id="createVmForm" onsubmit="handleCreateVm(event)">
              <div class="form-group">
                <label for="vmName">Instance Name <span class="required">*</span></label>
                <input type="text" id="vmName" name="name" required placeholder="my-vm-instance" />
              </div>
              <div class="form-group">
                <label for="vmCpuCores">CPU Cores <span class="required">*</span></label>
                <input type="number" id="vmCpuCores" name="cpu_cores" required min="1" max="64" value="2" />
              </div>
              <div class="form-group">
                <label for="vmMemoryMb">Memory (MB) <span class="required">*</span></label>
                <input type="number" id="vmMemoryMb" name="memory_mb" required min="256" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="vmGpuRequired">
                  <input type="checkbox" id="vmGpuRequired" name="gpu_required" />
                  GPU Required
                </label>
              </div>
              <div class="form-group">
                <label for="vmIsolation">Isolation Type <span class="required">*</span></label>
                <select id="vmIsolation" name="isolation" required>
                  <option value="ProcessSandbox">Process Sandbox</option>
                  <option value="HardwareVm">Hardware VM</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createVmModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

/// Worker management page
async fn admin_workers() -> Html<String> {
    let script = r#"
    async function loadWorkers() {
      try {
        const workers = await fetchJson('/api/v1/workers');
        renderWorkers(workers);
      } catch (e) {
        showNotification('Error loading workers: ' + e.message, 'error');
      }
    }
    
    function renderWorkers(workers) {
      const el = document.getElementById('workers-list');
      if (!el) return;
      if (!workers || workers.length === 0) {
        el.innerHTML = '<div class="muted">No workers found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Status</th>
              <th>Metrics</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${workers.map(w => `
              <tr>
                <td>${w.id || w.worker_id || 'unknown'}</td>
                <td><span class="status-badge ${w.is_healthy ? 'active' : 'error'}">${w.is_healthy ? 'Healthy' : 'Unhealthy'}</span></td>
                <td>Requests: ${w.total_requests_processed || 0}</td>
                <td>
                  <button class="btn btn-danger" onclick="deleteWorker('${w.id || w.worker_id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function deleteWorker(id) {
      if (!confirm('Delete worker ' + id + '?')) return;
      try {
        await fetchJson(`/api/v1/workers/${id}`, { method: 'DELETE' });
        showNotification('Worker deleted', 'success');
        loadWorkers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    function showCreateWorkerModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createWorkerModal');
    }
    
    async function handleCreateWorker(event) {
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
          worker_id: document.getElementById('workerId').value,
          max_concurrent_requests: document.getElementById('workerMaxConcurrent').value ? parseInt(document.getElementById('workerMaxConcurrent').value, 10) : undefined,
          request_timeout_ms: document.getElementById('workerTimeout').value ? parseInt(document.getElementById('workerTimeout').value, 10) : undefined,
          health_check_interval_ms: document.getElementById('workerHealthCheck').value ? parseInt(document.getElementById('workerHealthCheck').value, 10) : undefined,
          enable_caching: document.getElementById('workerEnableCaching').checked,
          cache_size: document.getElementById('workerCacheSize').value ? parseInt(document.getElementById('workerCacheSize').value, 10) : undefined,
          max_memory_mb: document.getElementById('workerMaxMemory').value ? parseInt(document.getElementById('workerMaxMemory').value, 10) : undefined,
          cpu_priority: document.getElementById('workerCpuPriority').value ? parseInt(document.getElementById('workerCpuPriority').value, 10) : undefined,
          gpu_device: document.getElementById('workerGpuDevice').value ? parseInt(document.getElementById('workerGpuDevice').value, 10) : undefined,
          auto_restart: document.getElementById('workerAutoRestart').checked,
          resource_monitoring: document.getElementById('workerResourceMonitoring').checked
        };
        
        // Remove undefined fields
        Object.keys(payload).forEach(key => {
          if (payload[key] === undefined) {
            delete payload[key];
          }
        });
        
        const result = await fetchJson('/api/v1/workers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Worker created successfully', 'success');
        hideModal('createWorkerModal');
        form.reset();
        
        setTimeout(() => {
          loadWorkers();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadWorkers();
    setInterval(loadWorkers, 5000);
    "#;

    admin_layout(
        "Worker Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Workers</h2>
            <button class="btn btn-primary" onclick="showCreateWorkerModal()" aria-label="Create new worker">Create Worker</button>
          </div>
          <div id="workers-list"></div>
        </div>
        
        <!-- Create Worker Modal -->
        <div id="createWorkerModal" class="modal" role="dialog" aria-labelledby="createWorkerModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createWorkerModalTitle">Create Worker</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createWorkerModal')">&times;</button>
            </div>
            <form id="createWorkerForm" onsubmit="handleCreateWorker(event)">
              <div class="form-group">
                <label for="workerId">Worker ID <span class="required">*</span></label>
                <input type="text" id="workerId" name="worker_id" required placeholder="worker-001" pattern="[a-zA-Z0-9_-]+" />
                <small class="form-hint">Alphanumeric, hyphens, and underscores only</small>
              </div>
              <div class="form-group">
                <label for="workerMaxConcurrent">Max Concurrent Requests</label>
                <input type="number" id="workerMaxConcurrent" name="max_concurrent_requests" min="1" max="1000" value="10" />
              </div>
              <div class="form-group">
                <label for="workerTimeout">Request Timeout (ms)</label>
                <input type="number" id="workerTimeout" name="request_timeout_ms" min="100" max="300000" value="5000" />
              </div>
              <div class="form-group">
                <label for="workerHealthCheck">Health Check Interval (ms)</label>
                <input type="number" id="workerHealthCheck" name="health_check_interval_ms" min="100" max="60000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerEnableCaching">
                  <input type="checkbox" id="workerEnableCaching" name="enable_caching" checked />
                  Enable Caching
                </label>
              </div>
              <div class="form-group">
                <label for="workerCacheSize">Cache Size</label>
                <input type="number" id="workerCacheSize" name="cache_size" min="0" max="100000" value="1000" />
              </div>
              <div class="form-group">
                <label for="workerMaxMemory">Max Memory (MB)</label>
                <input type="number" id="workerMaxMemory" name="max_memory_mb" min="128" max="131072" value="2048" />
              </div>
              <div class="form-group">
                <label for="workerCpuPriority">CPU Priority (0-10)</label>
                <input type="number" id="workerCpuPriority" name="cpu_priority" min="0" max="10" value="5" />
              </div>
              <div class="form-group">
                <label for="workerGpuDevice">GPU Device ID (optional)</label>
                <input type="number" id="workerGpuDevice" name="gpu_device" min="0" />
              </div>
              <div class="form-group">
                <label for="workerAutoRestart">
                  <input type="checkbox" id="workerAutoRestart" name="auto_restart" checked />
                  Auto Restart
                </label>
              </div>
              <div class="form-group">
                <label for="workerResourceMonitoring">
                  <input type="checkbox" id="workerResourceMonitoring" name="resource_monitoring" checked />
                  Resource Monitoring
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createWorkerModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

/// Library management page
async fn admin_libs() -> Html<String> {
    let script = r#"
    async function loadLibraries() {
      try {
        const libs = await fetchJson('/api/v1/libs');
        renderLibraries(libs);
      } catch (e) {
        showNotification('Error loading libraries: ' + e.message, 'error');
      }
    }
    
    function renderLibraries(libs) {
      const el = document.getElementById('libraries-list');
      if (!el) return;
      if (!libs || libs.length === 0) {
        el.innerHTML = '<div class="muted">No libraries found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Type</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${libs.map(l => `
              <tr>
                <td>${l.name || l.id}</td>
                <td>${l.type || 'unknown'}</td>
                <td><span class="status-badge ${l.status === 'active' ? 'active' : 'inactive'}">${l.status || 'unknown'}</span></td>
                <td>
                  <button class="btn" onclick="showNotification('Library actions - to be implemented', 'info')">Manage</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    loadLibraries();
    "#;

    admin_layout(
        "Library Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Libraries</h2>
            <button class="btn btn-primary" onclick="showNotification('Upload library - to be implemented', 'info')">Upload Library</button>
          </div>
          <div id="libraries-list"></div>
        </div>
        "#,
        script,
    )
}

/// RAID management page
async fn admin_raid() -> Html<String> {
    let script = r#"
    async function loadRaidArtifacts() {
      try {
        const artifacts = await fetchJson('/api/v1/raid/artifacts');
        renderRaidArtifacts(artifacts);
      } catch (e) {
        showNotification('Error loading RAID artifacts: ' + e.message, 'error');
      }
    }
    
    function renderRaidArtifacts(artifacts) {
      const el = document.getElementById('raid-artifacts');
      if (!el) return;
      if (!artifacts || artifacts.length === 0) {
        el.innerHTML = '<div class="muted">No artifacts found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Size</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${artifacts.map(a => `
              <tr>
                <td><code>${a.id || a.artifact_id || 'unknown'}</code></td>
                <td>${a.name || 'unnamed'}</td>
                <td>${formatBytes(a.size || 0)}</td>
                <td>
                  <button class="btn" onclick="showNotification('Artifact actions - to be implemented', 'info')">Manage</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function formatBytes(bytes) {
      if (bytes === 0) return '0 B';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
    }
    
    loadRaidArtifacts();
    "#;

    admin_layout(
        "RAID Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>RAID Artifacts</h2>
            <button class="btn btn-primary" onclick="showNotification('Upload artifact - to be implemented', 'info')">Upload Artifact</button>
          </div>
          <div id="raid-artifacts"></div>
        </div>
        "#,
        script,
    )
}

/// User management page
async fn admin_users() -> Html<String> {
    let script = r#"
    async function loadUsers() {
      try {
        const users = await fetchJson('/api/v1/users');
        renderUsers(users);
      } catch (e) {
        showNotification('Error loading users: ' + e.message, 'error');
      }
    }
    
    function renderUsers(users) {
      const el = document.getElementById('users-list');
      if (!el) return;
      if (!users || users.length === 0) {
        el.innerHTML = '<div class="muted">No users found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Role</th>
              <th>Status</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${users.map(u => `
              <tr>
                <td>${u.username || u.id}</td>
                <td>${u.role || 'Viewer'}</td>
                <td><span class="status-badge ${u.active !== false ? 'active' : 'error'}">${u.active !== false ? 'Active' : 'Inactive'}</span></td>
                <td>${u.created_at ? new Date(u.created_at).toLocaleDateString() : 'N/A'}</td>
                <td>
                  <button class="btn" onclick="editUser('${u.id}')">Edit</button>
                  <button class="btn btn-danger" onclick="deleteUser('${u.id}')">Delete</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    function showCreateUserModal() {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      showModal('createUserModal');
    }
    
    async function handleCreateUser(event) {
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
      btn.textContent = 'Creating...';
      
      try {
        const payload = {
          username: document.getElementById('userUsername').value,
          password: document.getElementById('userPassword').value,
          role: document.getElementById('userRole').value
        };
        
        await fetchJson('/api/v1/users', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('User created successfully', 'success');
        hideModal('createUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function editUser(id) {
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      try {
        const userData = await fetchJson(`/api/v1/users/${id}`);
        document.getElementById('editUserId').value = userData.id;
        document.getElementById('editUserUsername').value = userData.username;
        document.getElementById('editUserRole').value = userData.role;
        document.getElementById('editUserActive').checked = userData.active !== false;
        showModal('editUserModal');
      } catch (e) {
        showNotification('Error loading user for edit: ' + e.message, 'error');
      }
    }
    
    async function handleEditUser(event) {
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
        const id = document.getElementById('editUserId').value;
        const payload = {
          username: document.getElementById('editUserUsername').value,
          role: document.getElementById('editUserRole').value,
          active: document.getElementById('editUserActive').checked
        };
        
        // Only include password if it's provided
        const password = document.getElementById('editUserPassword').value;
        if (password) {
          payload.password = password;
        }
        
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'PUT',
          body: JSON.stringify(payload)
        });
        
        showNotification('User updated successfully', 'success');
        hideModal('editUserModal');
        form.reset();
        loadUsers();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteUser(id) {
      if (!confirm('Are you sure you want to delete this user? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || user.role !== 'Admin') {
        showNotification('Insufficient permissions. Admin role required.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/users/${id}`, {
          method: 'DELETE'
        });
        showNotification('User deleted successfully', 'success');
        loadUsers();
      } catch (e) {
        showNotification('Error deleting user: ' + e.message, 'error');
      }
    }
    
    loadUsers();
    "#;

    admin_layout(
        "User Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Users</h2>
            <button class="btn btn-primary" onclick="showCreateUserModal()" aria-label="Create new user">Create User</button>
          </div>
          <div id="users-list"></div>
        </div>
        
        <!-- Create User Modal -->
        <div id="createUserModal" class="modal" role="dialog" aria-labelledby="createUserModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="createUserModalTitle">Create New User</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createUserModal')">&times;</button>
            </div>
            <form id="createUserForm" onsubmit="handleCreateUser(event)">
              <div class="form-group">
                <label for="userUsername">Username</label>
                <input type="text" id="userUsername" name="username" required placeholder="newuser" />
              </div>
              <div class="form-group">
                <label for="userPassword">Password</label>
                <input type="password" id="userPassword" name="password" required placeholder="Enter password" />
              </div>
              <div class="form-group">
                <label for="userRole">Role</label>
                <select id="userRole" name="role" required>
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer" selected>Viewer</option>
                </select>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('createUserModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Create User</button>
              </div>
            </form>
          </div>
        </div>
        
        <!-- Edit User Modal -->
        <div id="editUserModal" class="modal" role="dialog" aria-labelledby="editUserModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="editUserModalTitle">Edit User</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('editUserModal')">&times;</button>
            </div>
            <form id="editUserForm" onsubmit="handleEditUser(event)">
              <input type="hidden" id="editUserId" name="id" />
              <div class="form-group">
                <label for="editUserUsername">Username</label>
                <input type="text" id="editUserUsername" name="username" required />
              </div>
              <div class="form-group">
                <label for="editUserPassword">New Password (leave empty to keep current)</label>
                <input type="password" id="editUserPassword" name="password" placeholder="Enter new password" />
              </div>
              <div class="form-group">
                <label for="editUserRole">Role</label>
                <select id="editUserRole" name="role" required>
                  <option value="Admin">Admin</option>
                  <option value="Operator">Operator</option>
                  <option value="Viewer">Viewer</option>
                </select>
              </div>
              <div class="form-group">
                <label for="editUserActive">
                  <input type="checkbox" id="editUserActive" name="active" />
                  Active
                </label>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('editUserModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Save Changes</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}

/// System configuration page
async fn admin_config() -> Html<String> {
    let script = r#"
    function showTab(tabName) {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadConfigTab(tabName);
    }
    
    async function loadConfigTab(tabName) {
      const el = document.getElementById('config-content');
      if (!el) return;
      el.innerHTML = '<div class="muted">Configuration for ' + tabName + ' - to be implemented</div>';
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
            <button class="tab" data-tab="security">Security</button>
            <button class="tab" data-tab="monitoring">Monitoring</button>
          </div>
          <div id="config-content"></div>
        </div>
        "#,
        script,
    )
}

/// Admin panel layout
fn admin_layout(title: &str, body_html: &str, script_js: &str) -> Html<String> {
    let base_css = include_str!("admin_styles.css");
    let common_js = include_str!("admin_common.js");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title} - PoolAI Admin</title>
  <style>{base_css}</style>
</head>
<body>
  <div class="admin-wrapper">
    <aside class="admin-sidebar" role="navigation" aria-label="Admin navigation">
      <div class="admin-brand">
        <h1>PoolAI Admin</h1>
        <div class="admin-version">v0.1.0</div>
      </div>
      <nav class="admin-nav">
        <a href="/ui/admin" class="admin-nav-item">Dashboard</a>
        <a href="/ui/admin/tenants" class="admin-nav-item">Tenants</a>
        <a href="/ui/admin/security" class="admin-nav-item">Security</a>
        <a href="/ui/admin/audit" class="admin-nav-item">Audit Logs</a>
        <a href="/ui/admin/monitoring" class="admin-nav-item">Monitoring</a>
        <a href="/ui/admin/vm" class="admin-nav-item">VM Instances</a>
        <a href="/ui/admin/workers" class="admin-nav-item">Workers</a>
        <a href="/ui/admin/libs" class="admin-nav-item">Libraries</a>
        <a href="/ui/admin/raid" class="admin-nav-item">RAID</a>
        <a href="/ui/admin/users" class="admin-nav-item">Users</a>
        <a href="/ui/admin/config" class="admin-nav-item">Configuration</a>
      </nav>
    </aside>
    
    <main class="admin-main" role="main">
      <header class="admin-header-bar">
        <h2>{title}</h2>
        <div class="admin-user-menu">
          <span id="admin-user-name">Admin</span>
          <button class="btn-icon" onclick="logout()" aria-label="Logout">🚪</button>
        </div>
      </header>
      
      <div class="admin-content">
        {body}
      </div>
    </main>
  </div>
  
  <script>{common_js}</script>
  <script>
    // Check admin access on page load
    (function() {{
      if (!requireAdmin()) {{
        return;
      }}
      // Initialize admin panel
      {script}
    }})();
  </script>
</body>
</html>"#,
        title = title,
        base_css = base_css,
        body = body_html,
        common_js = common_js,
        script = script_js
    );

    Html(html)
}
