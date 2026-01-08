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
      showNotification('Create tenant modal - to be implemented', 'info');
    }
    
    loadTenants();
    "#;
    
    admin_layout(
        "Tenant Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Tenants</h2>
            <button class="btn btn-primary" onclick="showCreateTenantModal()">Create Tenant</button>
          </div>
          <div id="tenants-list"></div>
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
    
    loadVmInstances();
    "#;
    
    admin_layout(
        "VM Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>VM Instances</h2>
            <button class="btn btn-primary" onclick="showNotification('Create VM - to be implemented', 'info')">Create VM Instance</button>
          </div>
          <div id="vm-instances"></div>
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
    
    loadWorkers();
    setInterval(loadWorkers, 5000);
    "#;
    
    admin_layout(
        "Worker Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Workers</h2>
            <button class="btn btn-primary" onclick="showNotification('Create worker - to be implemented', 'info')">Create Worker</button>
          </div>
          <div id="workers-list"></div>
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
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${users.map(u => `
              <tr>
                <td>${u.username || u.id}</td>
                <td>${u.role || 'Viewer'}</td>
                <td><span class="status-badge active">Active</span></td>
                <td>
                  <button class="btn" onclick="showNotification('User actions - to be implemented', 'info')">Edit</button>
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    loadUsers();
    "#;
    
    admin_layout(
        "User Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Users</h2>
            <button class="btn btn-primary" onclick="showNotification('Create user - to be implemented', 'info')">Create User</button>
          </div>
          <div id="users-list"></div>
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
  <script>{script}</script>
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
