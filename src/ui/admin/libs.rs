//! Library Management page
//!
//! Provides model library administration.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Library management page
pub async fn admin_libs() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    async function loadLibraries() {
      adminShowLoading('libraries-list', T('admin.lib.loading', 'Loading libraries…'));
      try {
        const libs = await fetchJson('/api/v1/libraries');
        renderLibraries(libs);
      } catch (e) {
        adminShowInlineError('libraries-list', e);
        showNotification(T('admin.lib.errLoad', 'Error loading libraries: ') + e.message, 'error');
      }
    }
    
    function renderLibraries(libs) {
      const el = document.getElementById('libraries-list');
      if (!el) return;
      if (!libs || libs.length === 0) {
        el.innerHTML = '<div class="muted">' + escapeHtml(T('admin.lib.empty', 'No libraries found')) + '</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>${escapeHtml(T('admin.lib.label.name', 'Library Name'))}</th>
              <th>${escapeHtml(T('admin.lib.label.version', 'Version'))}</th>
              <th>${escapeHtml(T('admin.wrk.col.status', 'Status'))}</th>
              <th>${escapeHtml(T('admin.wrk.col.actions', 'Actions'))}</th>
            </tr>
          </thead>
          <tbody>
            ${libs.map(l => {
              const key = l.name || l.id || 'unknown';
              const isInstalled =
                l.installed === true ||
                Boolean(l.metadata && l.metadata.installed_at);
              const keyJs = JSON.stringify(key);
              return `
              <tr>
                <td><strong>${escapeHtml(String(key))}</strong></td>
                <td>${escapeHtml(String(l.version || l.installed_version || T('admin.na', 'N/A')))}</td>
                <td><span class="status-badge ${isInstalled ? 'active' : 'inactive'}">${isInstalled ? escapeHtml(T('admin.lib.installed', 'Installed')) : escapeHtml(T('admin.lib.notInstalled', 'Not Installed'))}</span></td>
                <td>
                  ${isInstalled ? `
                    <button type="button" class="btn" onclick="uninstallLibrary(${keyJs})">${escapeHtml(T('ui.uninstall', 'Uninstall'))}</button>
                    <button type="button" class="btn" onclick="updateLibrary(${keyJs})">${escapeHtml(T('ui.update', 'Update'))}</button>
                  ` : `
                    <button type="button" class="btn btn-primary" onclick="installLibrary(${keyJs})">${escapeHtml(T('ui.install', 'Install'))}</button>
                  `}
                </td>
              </tr>
            `;
            }).join('')}
          </tbody>
        </table>
      `;
    }
    
    async function installLibrary(name) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      const version = prompt(T('admin.lib.promptInstall', 'Enter version to install (leave empty for latest):'));
      if (version === null) return;
      
      try {
        const payload = version ? { version: version } : {};
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/install`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        showNotification(T('admin.lib.installStart', 'Library installation started'), 'success');
        setTimeout(() => loadLibraries(), 1000);
      } catch (e) {
        showNotification(T('admin.lib.errInstall', 'Error installing library: ') + e.message, 'error');
      }
    }
    
    async function uninstallLibrary(name) {
      if (!confirm(T('admin.lib.confirmUn', 'Uninstall library "{name}"? This action cannot be undone.').replace(/\{name\}/g, name))) {
        return;
      }
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/uninstall`, {
          method: 'POST'
        });
        showNotification(T('admin.lib.uninstOk', 'Library uninstalled successfully'), 'success');
        loadLibraries();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    async function updateLibrary(name) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      const version = prompt(T('admin.lib.promptUpdate', 'Enter version to update to (leave empty for latest):'));
      if (version === null) return;
      
      try {
        const payload = version ? { version: version } : {};
        await fetchJson(`/api/v1/libraries/${encodeURIComponent(name)}/update`, {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        showNotification(T('admin.lib.updateStart', 'Library update started'), 'success');
        setTimeout(() => loadLibraries(), 1000);
      } catch (e) {
        showNotification(T('admin.lib.errUpdate', 'Error updating library: ') + e.message, 'error');
      }
    }
    
    function showUploadLibraryModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('uploadLibraryModal');
    }

    async function handleUploadLibrary(event) {
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
      btn.textContent = T('admin.lib.uploading', 'Uploading…');

      try {
        const name = document.getElementById('libraryUploadName').value;
        const version = document.getElementById('libraryUploadVersion').value;
        const fileInput = document.getElementById('libraryUploadFile');
        const file = fileInput.files[0];

        if (!name || !version || !file) {
          showNotification(T('admin.lib.reqFields', 'Name, version, and file are required'), 'error');
          btn.disabled = false;
          btn.textContent = originalText;
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

            showNotification(T('admin.lib.uploadOk', 'Library uploaded successfully'), 'success');
            hideModal('uploadLibraryModal');
            form.reset();
            loadLibraries();
          } catch (uploadError) {
            showNotification(T('admin.lib.errUpload', 'Error uploading library: ') + uploadError.message, 'error');
          } finally {
            btn.disabled = false;
            btn.textContent = originalText;
          }
        };
        reader.onerror = function() {
          showNotification(T('err.readFileFailed', 'Error reading file'), 'error');
          btn.disabled = false;
          btn.textContent = originalText;
        };
        reader.readAsBinaryString(file);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    loadLibraries();
    setInterval(loadLibraries, 10000);
    "#;

    admin_layout(
        "admin.page.libs",
        "Library Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.lib.section">Libraries</h2>
            <button type="button" class="btn btn-primary" onclick="showUploadLibraryModal()" data-i18n="admin.lib.uploadBtn" data-i18n-aria="ui.upload">Upload Library</button>
          </div>
          <div id="libraries-list"></div>
        </div>

        <div id="uploadLibraryModal" class="modal" role="dialog" aria-labelledby="uploadLibraryModalTitle" aria-modal="false" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="uploadLibraryModalTitle" data-i18n="admin.lib.uploadTitle">Upload Library</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('uploadLibraryModal')">&times;</button>
            </div>
            <form id="uploadLibraryForm" onsubmit="handleUploadLibrary(event)">
              <div class="form-group">
                <label for="libraryUploadName"><span data-i18n="admin.lib.label.name">Library Name</span> <span class="required">*</span></label>
                <input type="text" id="libraryUploadName" name="name" required data-i18n-placeholder="admin.lib.ph.name" placeholder="my-model-lib" />
              </div>
              <div class="form-group">
                <label for="libraryUploadVersion"><span data-i18n="admin.lib.label.version">Version</span> <span class="required">*</span></label>
                <input type="text" id="libraryUploadVersion" name="version" required data-i18n-placeholder="admin.lib.ph.version" placeholder="1.0.0" />
              </div>
              <div class="form-group">
                <label for="libraryUploadFile"><span data-i18n="admin.lib.label.file">Library File (e.g., .zip, .tar.gz)</span> <span class="required">*</span></label>
                <input type="file" id="libraryUploadFile" name="file" required />
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('uploadLibraryModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.upload">Upload</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
