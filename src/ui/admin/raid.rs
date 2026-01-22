//! RAID Management page
//!
//! Provides artifact storage and replication management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// RAID management page
pub async fn admin_raid() -> Html<String> {
    let script = r#"
    async function loadRaidData() {
      try {
        const [artifacts, snapshot, status, burstMetrics, smallworldMetrics] = await Promise.all([
          fetchJson('/api/v1/raid/artifacts'),
          loadSnapshot().catch(() => null),
          fetchJson('/api/raid/admin/status').catch(() => null),
          fetchJson('/api/raid/admin/metrics/burst').catch(() => null),
          fetchJson('/api/raid/admin/metrics/smallworld').catch(() => null)
        ]);
        renderRaidArtifacts(artifacts, snapshot);
        renderRaidAdmin(status, burstMetrics, smallworldMetrics);
      } catch (e) {
        showNotification('Error loading RAID data: ' + e.message, 'error');
      }
    }
    
    function renderRaidAdmin(status, burstMetrics, smallworldMetrics) {
      const el = document.getElementById('raid-admin');
      if (!el) return;
      
      let html = '';
      
      if (status) {
        html += `
          <div class="admin-card">
            <h3>RAID Strategy Status</h3>
            <div class="stat-item">
              <span class="stat-label">Mode:</span>
              <span class="stat-value">${status.mode || 'Unknown'}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Initialized:</span>
              <span class="stat-value status-badge ${status.initialized ? 'active' : 'inactive'}">${status.initialized ? 'Yes' : 'No'}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Active:</span>
              <span class="stat-value status-badge ${status.active ? 'active' : 'inactive'}">${status.active ? 'Yes' : 'No'}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Rebalancing Enabled:</span>
              <span class="stat-value status-badge ${status.rebalancing_enabled ? 'active' : 'inactive'}">${status.rebalancing_enabled ? 'Yes' : 'No'}</span>
            </div>
          </div>
        `;
      }
      
      if (burstMetrics) {
        html += `
          <div class="admin-card">
            <h3>BurstRAID Metrics</h3>
            <div class="stat-item">
              <span class="stat-label">Total Artifacts:</span>
              <span class="stat-value">${burstMetrics.total_artifacts || 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Burst Artifacts:</span>
              <span class="stat-value">${burstMetrics.burst_artifacts || 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Avg Replication Factor:</span>
              <span class="stat-value">${(burstMetrics.avg_replication_factor || 0).toFixed(2)}</span>
            </div>
          </div>
        `;
      }
      
      if (smallworldMetrics) {
        html += `
          <div class="admin-card">
            <h3>SmallWorld Network Metrics</h3>
            <div class="stat-item">
              <span class="stat-label">Total Nodes:</span>
              <span class="stat-value">${smallworldMetrics.total_nodes || 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Avg Clustering Coefficient:</span>
              <span class="stat-value">${(smallworldMetrics.avg_clustering_coefficient || 0).toFixed(3)}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Avg Path Length:</span>
              <span class="stat-value">${(smallworldMetrics.avg_path_length || 0).toFixed(2)}</span>
            </div>
          </div>
        `;
      }
      
      if (html) {
        el.innerHTML = html;
      }
    }
    
    async function triggerRebalance() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Are you sure you want to trigger RAID rebalancing? This may impact system performance.')) {
        return;
      }
      
      try {
        await fetchJson('/api/raid/admin/rebalance', { method: 'POST' });
        showNotification('Rebalancing triggered successfully', 'success');
        setTimeout(() => loadRaidData(), 2000);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    async function loadSnapshot() {
      try {
        const snapshot = await fetchJson('/api/v1/raid/snapshot');
        return snapshot;
      } catch (e) {
        return null;
      }
    }
    
    function renderRaidArtifacts(artifacts, snapshot) {
      const el = document.getElementById('raid-artifacts');
      if (!el) return;
      
      const artifactsHtml = !artifacts || artifacts.length === 0 
        ? '<div class="muted">No artifacts found</div>'
        : `
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
      
      const snapshotHtml = snapshot 
        ? `
          <div class="admin-card">
            <h3>Current Snapshot</h3>
            <div class="muted">Sequence: ${snapshot.sequence || 'N/A'}</div>
            <div class="muted">Created: ${snapshot.timestamp ? new Date(snapshot.timestamp).toLocaleString() : 'N/A'}</div>
            <button class="btn" onclick="restoreFromSnapshot()">Restore from Snapshot</button>
          </div>
        `
        : '<div class="admin-card"><div class="muted">No snapshot available</div></div>';
      
      el.innerHTML = `
        <div class="admin-card">
          <h3>Artifacts (${artifacts?.length || 0})</h3>
          ${artifactsHtml}
        </div>
        ${snapshotHtml}
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
        loadRaidData();
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
        loadRaidData();
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      }
    }
    
    async function createSnapshot() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Create a new snapshot? This will capture the current state of all artifacts.')) {
        return;
      }
      
      try {
        await fetchJson('/api/v1/raid/snapshot/create', {
          method: 'POST'
        });
        showNotification('Snapshot created successfully', 'success');
        loadRaidData();
      } catch (e) {
        showNotification('Error creating snapshot: ' + e.message, 'error');
      }
    }
    
    async function restoreFromSnapshot() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Restore from snapshot? This will restore the RAID state from the latest snapshot. This action cannot be undone.')) {
        return;
      }
      
      try {
        showLoading('raid-artifacts', 'Restoring from snapshot...');
        await fetchJson('/api/v1/raid/snapshot/restore', { method: 'POST' });
        showNotification('Restored from snapshot successfully', 'success');
        setTimeout(() => {
          loadRaidData();
        }, 1000);
      } catch (e) {
        showNotification('Error restoring from snapshot: ' + e.message, 'error');
        hideLoading('raid-artifacts');
      }
    }
    
    async function syncArtifacts() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      try {
        await fetchJson('/api/v1/raid/distributed/artifacts/sync', {
          method: 'POST',
          body: JSON.stringify({})
        });
        showNotification('Artifacts sync started', 'success');
        setTimeout(() => loadRaidArtifacts(), 2000);
      } catch (e) {
        showNotification('Error syncing artifacts: ' + e.message, 'error');
      }
    }
    
    async function runGc() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      if (!confirm('Run garbage collection? This will remove old artifacts that are no longer referenced.')) {
        return;
      }
      
      try {
        const result = await fetchJson('/api/v1/raid/gc', {
          method: 'POST'
        });
        showNotification(`Garbage collection completed. Removed ${result.removed_count || 0} artifacts.`, 'success');
        loadRaidData();
      } catch (e) {
        showNotification('Error running GC: ' + e.message, 'error');
      }
    }
    
    loadRaidData();
    setInterval(loadRaidData, 10000);
    "#;

    admin_layout(
        "RAID Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>RAID Artifacts</h2>
            <div>
              <button class="btn btn-primary" onclick="showUploadArtifactModal()" aria-label="Upload artifact">Upload Artifact</button>
              <button class="btn" onclick="createSnapshot()" aria-label="Create snapshot">Create Snapshot</button>
              <button class="btn" onclick="syncArtifacts()" aria-label="Sync artifacts">Sync Artifacts</button>
              <button class="btn" onclick="runGc()" aria-label="Run garbage collection">Run GC</button>
            </div>
          </div>
          <div id="raid-admin"></div>
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
