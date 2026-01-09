//! RAID Management page
//!
//! Provides artifact storage and replication management.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// RAID management page
pub async fn admin_raid() -> Html<String> {
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
                  <button class="btn btn-danger" onclick="deleteArtifact('${a.id || a.artifact_id}')">Delete</button>
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
    
    function showUploadArtifactModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('uploadArtifactModal');
    }
    
    async function handleUploadArtifact(event) {
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
        const name = document.getElementById('artifactName').value;
        const data = document.getElementById('artifactData').value;
        
        if (!name || !data) {
          showNotification('Name and data are required', 'error');
          return;
        }
        
        const payload = {
          name: name,
          data: data
        };
        
        await fetchJson('/api/v1/raid/artifacts', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Artifact uploaded successfully', 'success');
        hideModal('uploadArtifactModal');
        form.reset();
        loadRaidArtifacts();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteArtifact(id) {
      if (!confirm('Delete artifact "' + id + '"? This action cannot be undone.')) {
        return;
      }
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/raid/artifacts/${encodeURIComponent(id)}`, {
          method: 'DELETE'
        });
        showNotification('Artifact deleted successfully', 'success');
        loadRaidArtifacts();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    loadRaidArtifacts();
    setInterval(loadRaidArtifacts, 10000);
    "#;

    admin_layout(
        "RAID Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>RAID Artifacts</h2>
            <button class="btn btn-primary" onclick="showUploadArtifactModal()" aria-label="Upload artifact">Upload Artifact</button>
          </div>
          <div id="raid-artifacts"></div>
        </div>
        
        <!-- Upload Artifact Modal -->
        <div id="uploadArtifactModal" class="modal" role="dialog" aria-labelledby="uploadArtifactModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="uploadArtifactModalTitle">Upload Artifact</h3>
              <button class="modal-close" aria-label="Close dialog" onclick="hideModal('uploadArtifactModal')">&times;</button>
            </div>
            <form id="uploadArtifactForm" onsubmit="handleUploadArtifact(event)">
              <div class="form-group">
                <label for="artifactName">Artifact Name <span class="required">*</span></label>
                <input type="text" id="artifactName" name="name" required placeholder="my-artifact" />
              </div>
              <div class="form-group">
                <label for="artifactData">Artifact Data (Base64) <span class="required">*</span></label>
                <textarea id="artifactData" name="data" required rows="10" placeholder="Paste base64-encoded data here"></textarea>
                <small class="form-hint">Paste base64-encoded artifact data</small>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('uploadArtifactModal')">Cancel</button>
                <button type="submit" class="btn btn-primary">Upload</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
