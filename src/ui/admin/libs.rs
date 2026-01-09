//! Library Management page
//!
//! Provides model library administration.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// Library management page
pub async fn admin_libs() -> Html<String> {
    let script = r#"
    async function loadLibraries() {
      try {
        const libs = await fetchJson('/api/v1/libraries');
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
              <th>Version</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            ${libs.map(l => `
              <tr>
                <td><strong>${l.name || l.id || 'unknown'}</strong></td>
                <td>${l.version || l.installed_version || 'N/A'}</td>
                <td><span class="status-badge ${l.installed ? 'active' : 'inactive'}">${l.installed ? 'Installed' : 'Not Installed'}</span></td>
                <td>
                  ${l.installed ? `
                    <button class="btn" onclick="uninstallLibrary('${l.name || l.id}')">Uninstall</button>
                    <button class="btn" onclick="updateLibrary('${l.name || l.id}')">Update</button>
                  ` : `
                    <button class="btn btn-primary" onclick="installLibrary('${l.name || l.id}')">Install</button>
                  `}
                </td>
              </tr>
            `).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function installLibrary(name) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      const version = prompt('Enter version to install (leave empty for latest):');
      if (version === null) return;
      
      try {
        const payload = version ? { version: version } : {};
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/install`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        showNotification('Library installation started', 'success');
        setTimeout(() => loadLibraries(), 1000);
      } catch (e) {
        showNotification('Error installing library: ' + e.message, 'error');
      }
    }
    
    async function uninstallLibrary(name) {
      if (!confirm('Uninstall library "' + name + '"? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/uninstall`, {
          method: 'POST'
        });
        showNotification('Library uninstalled successfully', 'success');
        loadLibraries();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    async function updateLibrary(name) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      const version = prompt('Enter version to update to (leave empty for latest):');
      if (version === null) return;
      
      try {
        const payload = version ? { version: version } : {};
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/update`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        showNotification('Library update started', 'success');
        setTimeout(() => loadLibraries(), 1000);
      } catch (e) {
        showNotification('Error updating library: ' + e.message, 'error');
      }
    }
    
    loadLibraries();
    setInterval(loadLibraries, 10000);
    "#;

    admin_layout(
        "Library Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Libraries</h2>
          </div>
          <div id="libraries-list"></div>
        </div>
        "#,
        script,
    )
}
