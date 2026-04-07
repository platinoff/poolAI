//! Library Management page
//!
//! Provides model library administration.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Library management page
pub async fn admin_libs() -> Html<String> {
    let script = r#"
    async function loadLibraries() {
      adminShowLoading('libraries-list', 'Loading libraries…');
      try {
        const libs = await fetchJson('/api/v1/libraries');
        renderLibraries(libs);
      } catch (e) {
        adminShowInlineError('libraries-list', e);
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
            ${libs.map(l => {
              const key = l.name || l.id || 'unknown';
              // GET /libraries returns installed catalog entries; treat missing flag as installed.
              const isInstalled = l.installed !== false;
              return `
              <tr>
                <td><strong>${key}</strong></td>
                <td>${l.version || l.installed_version || 'N/A'}</td>
                <td><span class="status-badge ${isInstalled ? 'active' : 'inactive'}">${isInstalled ? 'Installed' : 'Not Installed'}</span></td>
                <td>
                  ${isInstalled ? `
                    <button class="btn" onclick="uninstallLibrary(${JSON.stringify(key)})">Uninstall</button>
                    <button class="btn" onclick="updateLibrary(${JSON.stringify(key)})">Update</button>
                  ` : `
                    <button class="btn btn-primary" onclick="installLibrary(${JSON.stringify(key)})">Install</button>
                  `}
                </td>
              </tr>
            `}).join('')}
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
    
    function showUploadLibraryModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('uploadLibraryModal');
    }

    async function handleUploadLibrary(event) {
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
      btn.textContent = 'Uploading...';

      try {
        const name = document.getElementById('libraryUploadName').value;
        const version = document.getElementById('libraryUploadVersion').value;
        const fileInput = document.getElementById('libraryUploadFile');
        const file = fileInput.files[0];

        if (!name || !version || !file) {
          showNotification('Name, version, and file are required', 'error');
          return;
        }

        const reader = new FileReader();
        reader.onload = async function(e) {
          try {
            const binaryString = e.target.result;
            const base64Data = btoa(binaryString);

            const payload = {
              name: name,
              version: version,
              data: base64Data
            };

            await fetchJson('/api/v1/libraries/upload', {
              method: 'POST',
              body: JSON.stringify(payload)
            });

            showNotification('Library uploaded successfully', 'success');
            hideModal('uploadLibraryModal');
            form.reset();
            loadLibraries();
          } catch (uploadError) {
            showNotification('Error uploading library: ' + uploadError.message, 'error');
          } finally {
            btn.disabled = false;
            btn.textContent = originalText;
          }
        };
        reader.onerror = function() {
          showNotification('Error reading file', 'error');
          btn.disabled = false;
          btn.textContent = originalText;
        };
        reader.readAsBinaryString(file);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        btn.disabled = false;
        btn.textContent = originalText;
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
            <button class="btn btn-primary" onclick="showUploadLibraryModal()" aria-label="Upload library">Upload Library</button>
          </div>
          <div id="libraries-list"></div>
        </div>

        <!-- Upload Library Modal -->
        <div id="uploadLibraryModal" class="modal" role="dialog" aria-labelledby="uploadLibraryModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="uploadLibraryModalTitle">Upload Library</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('uploadLibraryModal')">&times;</button>
            </div>
            <form id="uploadLibraryForm" onsubmit="handleUploadLibrary(event)">
              <div class="form-group">
                <label for="libraryUploadName">Library Name <span class="required">*</span></label>
                <input type="text" id="libraryUploadName" name="name" required placeholder="my-model-lib" />
              </div>
              <div class="form-group">
                <label for="libraryUploadVersion">Version <span class="required">*</span></label>
                <input type="text" id="libraryUploadVersion" name="version" required placeholder="1.0.0" />
              </div>
              <div class="form-group">
                <label for="libraryUploadFile">Library File (e.g., .zip, .tar.gz) <span class="required">*</span></label>
                <input type="file" id="libraryUploadFile" name="file" required />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('uploadLibraryModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Upload</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
