//! RAID Management page
//!
//! Provides artifact storage and replication management.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// RAID management page
pub async fn admin_raid() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    function raidStrategyStatusFromResponse(raw) {
      if (!raw || typeof raw !== 'object') return null;
      return raw.status != null ? raw.status : raw;
    }
    function raidMetricsFromResponse(raw) {
      if (!raw || typeof raw !== 'object') return null;
      return raw.metrics != null ? raw.metrics : raw;
    }

    async function loadRaidData() {
      adminShowLoading('raid-admin', T('admin.raidadm.loading', 'Loading RAID admin…'));
      adminShowLoading('raid-artifacts', T('admin.raidadm.loadingArt', 'Loading artifacts…'));
      try {
        const [artifacts, snapshot, statusRaw, burstRaw, smallworldRaw] = await Promise.all([
          fetchJson('/api/v1/raid/artifacts'),
          loadSnapshot().catch(() => null),
          fetchJson('/api/v1/raid/admin/status').catch(() => null),
          fetchJson('/api/v1/raid/admin/metrics/burst').catch(() => null),
          fetchJson('/api/v1/raid/admin/metrics/smallworld').catch(() => null)
        ]);
        const status = raidStrategyStatusFromResponse(statusRaw);
        const burstMetrics = raidMetricsFromResponse(burstRaw);
        const smallworldMetrics = raidMetricsFromResponse(smallworldRaw);
        renderRaidArtifacts(artifacts, snapshot);
        renderRaidAdmin(status, burstMetrics, smallworldMetrics);
      } catch (e) {
        adminShowInlineError('raid-admin', e);
        adminShowInlineError('raid-artifacts', e);
        showNotification(T('admin.raidadm.errLoad', 'Error loading RAID data: ') + e.message, 'error');
      }
    }
    
    function renderRaidAdmin(status, burstMetrics, smallworldMetrics) {
      const el = document.getElementById('raid-admin');
      if (!el) return;
      
      let html = '';
      
      if (status) {
        html += `
          <div class="admin-card">
            <h3>${escapeHtml(T('admin.raidadm.strategyTitle', 'RAID Strategy Status'))}</h3>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.mode', 'Mode:'))}</span>
              <span class="stat-value">${escapeHtml(status.mode || T('admin.raidadm.unknown', 'Unknown'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.init', 'Initialized:'))}</span>
              <span class="stat-value status-badge ${status.initialized ? 'active' : 'inactive'}">${status.initialized ? escapeHtml(T('admin.status.yes', 'Yes')) : escapeHtml(T('admin.status.no', 'No'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.active', 'Active:'))}</span>
              <span class="stat-value status-badge ${status.active ? 'active' : 'inactive'}">${status.active ? escapeHtml(T('admin.status.yes', 'Yes')) : escapeHtml(T('admin.status.no', 'No'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.rebal', 'Rebalancing Enabled:'))}</span>
              <span class="stat-value status-badge ${status.rebalancing_enabled ? 'active' : 'inactive'}">${status.rebalancing_enabled ? escapeHtml(T('admin.status.yes', 'Yes')) : escapeHtml(T('admin.status.no', 'No'))}</span>
            </div>
          </div>
        `;
      }
      
      if (burstMetrics) {
        html += `
          <div class="admin-card">
            <h3>${escapeHtml(T('admin.raidadm.burstTitle', 'BurstRAID Metrics'))}</h3>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.totalArt', 'Total Artifacts:'))}</span>
              <span class="stat-value">${burstMetrics.total_artifacts ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.artBurst', 'Artifacts in Burst:'))}</span>
              <span class="stat-value">${burstMetrics.artifacts_in_burst ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.repl', 'Replication (base / max):'))}</span>
              <span class="stat-value">${burstMetrics.base_replication_factor ?? 0} / ${burstMetrics.max_replication_factor ?? 0}</span>
            </div>
          </div>
        `;
      }
      
      if (smallworldMetrics) {
        html += `
          <div class="admin-card">
            <h3>${escapeHtml(T('admin.raidadm.swTitle', 'SmallWorld Network Metrics'))}</h3>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.totalArt', 'Total Artifacts:'))}</span>
              <span class="stat-value">${smallworldMetrics.total_artifacts ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.totalNodes', 'Total Nodes:'))}</span>
              <span class="stat-value">${smallworldMetrics.total_nodes ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.avgClust', 'Avg Clustering Coefficient:'))}</span>
              <span class="stat-value">${(smallworldMetrics.avg_clustering_coefficient ?? 0).toFixed(3)}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.tgtClust', 'Target Clustering:'))}</span>
              <span class="stat-value">${(smallworldMetrics.target_clustering_coefficient ?? 0).toFixed(3)}</span>
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
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      if (!confirm(T('admin.raidadm.confirmRebal', 'Are you sure you want to trigger RAID rebalancing? This may impact system performance.'))) {
        return;
      }
      
      try {
        await fetchJson('/api/v1/raid/admin/rebalance', { method: 'POST' });
        showNotification(T('admin.raidadm.rebalOk', 'Rebalancing triggered successfully'), 'success');
        setTimeout(() => loadRaidData(), 2000);
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
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
        ? '<div class="muted">' + escapeHtml(T('admin.raidadm.emptyArt', 'No artifacts found')) + '</div>'
        : `
          <table class="admin-table">
            <thead>
              <tr>
                <th>${escapeHtml(T('admin.raidadm.col.id', 'ID'))}</th>
                <th>${escapeHtml(T('admin.raidadm.col.name', 'Name'))}</th>
                <th>${escapeHtml(T('admin.raidadm.col.size', 'Size'))}</th>
                <th>${escapeHtml(T('admin.wrk.col.actions', 'Actions'))}</th>
              </tr>
            </thead>
            <tbody>
              ${artifacts.map(a => {
                const aid = a.id || a.artifact_id || 'unknown';
                return `
                <tr>
                  <td><code>${escapeHtml(String(aid))}</code></td>
                  <td>${escapeHtml(a.name || 'unnamed')}</td>
                  <td>${escapeHtml(formatBytes(a.size || 0))}</td>
                  <td>
                    <button type="button" class="btn btn-danger" onclick="deleteArtifact(${JSON.stringify(aid)})">${escapeHtml(T('ui.delete', 'Delete'))}</button>
                  </td>
                </tr>
              `;
              }).join('')}
            </tbody>
          </table>
        `;
      
      const snapshotHtml = snapshot 
        ? `
          <div class="admin-card">
            <h3>${escapeHtml(T('admin.raidadm.snapshotTitle', 'Current Snapshot'))}</h3>
            <div class="muted">${escapeHtml(T('admin.raidadm.seq', 'Sequence:'))} ${escapeHtml(String(snapshot.sequence != null ? snapshot.sequence : T('admin.na', 'N/A')))}</div>
            <div class="muted">${escapeHtml(T('admin.raidadm.created', 'Created:'))} ${snapshot.timestamp ? escapeHtml(new Date(snapshot.timestamp).toLocaleString()) : escapeHtml(T('admin.na', 'N/A'))}</div>
            <button type="button" class="btn" onclick="restoreFromSnapshot()">${escapeHtml(T('admin.raidadm.restoreBtn', 'Restore from Snapshot'))}</button>
          </div>
        `
        : '<div class="admin-card"><div class="muted">' + escapeHtml(T('admin.raidadm.noSnap', 'No snapshot available')) + '</div></div>';
      
      el.innerHTML = `
        <div class="admin-card">
          <h3>${escapeHtml(T('admin.raidadm.artTitle', 'Artifacts'))} (${artifacts?.length || 0})</h3>
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
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      showModal('uploadArtifactModal');
    }
    
    async function handleUploadArtifact(event) {
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
      btn.textContent = T('admin.raidadm.uploading', 'Uploading…');
      
      try {
        const name = document.getElementById('artifactName').value;
        const data = document.getElementById('artifactData').value;
        
        if (!name || !data) {
          showNotification(T('admin.raidadm.reqNameData', 'Name and data are required'), 'error');
          btn.disabled = false;
          btn.textContent = originalText;
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
        
        showNotification(T('admin.raidadm.uploadOk', 'Artifact uploaded successfully'), 'success');
        hideModal('uploadArtifactModal');
        form.reset();
        loadRaidData();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function deleteArtifact(id) {
      if (!confirm(T('admin.raidadm.confirmDelArt', 'Delete artifact "{id}"? This action cannot be undone.').replace(/\{id\}/g, id))) {
        return;
      }
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissions', 'Insufficient permissions.'), 'error');
        return;
      }
      
      try {
        await fetchJson(`/api/v1/raid/artifacts/${encodeURIComponent(id)}`, {
          method: 'DELETE'
        });
        showNotification(T('admin.raidadm.delOk', 'Artifact deleted successfully'), 'success');
        loadRaidData();
      } catch (e) {
        showNotification(Ep() + e.message, 'error');
      }
    }
    
    async function createSnapshot() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      if (!confirm(T('admin.raidadm.confirmSnap', 'Create a new snapshot? This will capture the current state of all artifacts.'))) {
        return;
      }
      
      try {
        await fetchJson('/api/v1/raid/snapshot/create', {
          method: 'POST'
        });
        showNotification(T('admin.raidadm.snapOk', 'Snapshot created successfully'), 'success');
        loadRaidData();
      } catch (e) {
        showNotification(T('admin.raidadm.errSnap', 'Error creating snapshot: ') + e.message, 'error');
      }
    }
    
    async function restoreFromSnapshot() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      if (!confirm(T('admin.raidadm.confirmRestore', 'Restore from snapshot? This will restore the RAID state from the latest snapshot. This action cannot be undone.'))) {
        return;
      }
      
      try {
        showLoading('raid-artifacts', T('admin.raidadm.restoring', 'Restoring from snapshot…'));
        await fetchJson('/api/v1/raid/snapshot/restore', { method: 'POST' });
        hideLoading('raid-artifacts');
        showNotification(T('admin.raidadm.restoreOk', 'Restored from snapshot successfully'), 'success');
        setTimeout(() => {
          loadRaidData();
        }, 1000);
      } catch (e) {
        showNotification(T('admin.raidadm.errRestore', 'Error restoring from snapshot: ') + e.message, 'error');
        hideLoading('raid-artifacts');
      }
    }
    
    async function syncArtifacts() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      try {
        const syncBody = {
          type: 'sync_artifacts',
          id: (typeof crypto !== 'undefined' && crypto.randomUUID) ? crypto.randomUUID() : String(Date.now()),
          timestamp: new Date().toISOString(),
          node_id: 'ui-admin',
          payload: { direction: 'bidirectional' }
        };
        await fetchJson('/api/v1/raid/distributed/artifacts/sync', {
          method: 'POST',
          body: JSON.stringify(syncBody)
        });
        showNotification(T('admin.raidadm.syncOk', 'Artifacts sync started'), 'success');
        setTimeout(() => loadRaidData(), 2000);
      } catch (e) {
        showNotification(T('admin.raidadm.errSync', 'Error syncing artifacts: ') + e.message, 'error');
      }
    }
    
    async function runGc() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification(T('err.insufficientPermissionsAdminOp', 'Insufficient permissions. Admin or Operator role required.'), 'error');
        return;
      }
      
      if (!confirm(T('admin.raidadm.confirmGc', 'Run garbage collection? This will remove old artifacts that are no longer referenced.'))) {
        return;
      }
      
      try {
        const result = await fetchJson('/api/v1/raid/gc', {
          method: 'POST'
        });
        const n = result.removed_count || 0;
        showNotification(T('admin.raidadm.gcOk', 'Garbage collection completed. Removed {n} artifacts.').replace(/\{n\}/g, String(n)), 'success');
        loadRaidData();
      } catch (e) {
        showNotification(T('admin.raidadm.errGc', 'Error running GC: ') + e.message, 'error');
      }
    }
    
    loadRaidData();
    setInterval(loadRaidData, 10000);
    "#;

    admin_layout(
        "admin.page.raid",
        "RAID Management",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.raidadm.section">RAID Artifacts</h2>
            <div>
              <button type="button" class="btn btn-primary" onclick="showUploadArtifactModal()" data-i18n="admin.raidadm.btn.upload" data-i18n-aria="admin.raidadm.btn.upload">Upload Artifact</button>
              <button type="button" class="btn" onclick="createSnapshot()" data-i18n="admin.raidadm.btn.snapshot" data-i18n-aria="admin.raidadm.btn.snapshot">Create Snapshot</button>
              <button type="button" class="btn" onclick="syncArtifacts()" data-i18n="admin.raidadm.btn.sync" data-i18n-aria="admin.raidadm.btn.sync">Sync Artifacts</button>
              <button type="button" class="btn" onclick="runGc()" data-i18n="admin.raidadm.btn.gc" data-i18n-aria="admin.raidadm.btn.gc">Run GC</button>
            </div>
          </div>
          <div id="raid-admin"></div>
          <div id="raid-artifacts"></div>
        </div>
        
        <div id="uploadArtifactModal" class="modal" role="dialog" aria-labelledby="uploadArtifactModalTitle" aria-modal="true" aria-hidden="true">
          <div class="modal-content">
            <div class="modal-header">
              <h3 id="uploadArtifactModalTitle" data-i18n="admin.raidadm.uploadTitle">Upload Artifact</h3>
              <button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal('uploadArtifactModal')">&times;</button>
            </div>
            <form id="uploadArtifactForm" onsubmit="handleUploadArtifact(event)">
              <div class="form-group">
                <label for="artifactName"><span data-i18n="admin.raidadm.label.artName">Artifact Name</span> <span class="required">*</span></label>
                <input type="text" id="artifactName" name="name" required data-i18n-placeholder="admin.raidadm.ph.artName" placeholder="my-artifact" />
              </div>
              <div class="form-group">
                <label for="artifactData"><span data-i18n="admin.raidadm.label.artData">Artifact Data (Base64)</span> <span class="required">*</span></label>
                <textarea id="artifactData" name="data" required rows="10" data-i18n-placeholder="admin.raidadm.ph.b64" placeholder="Paste base64-encoded data here"></textarea>
                <small class="form-hint" data-i18n="admin.raidadm.hint.b64">Paste base64-encoded artifact data</small>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn" onclick="hideModal('uploadArtifactModal')" data-i18n="ui.cancel">Cancel</button>
                <button type="submit" class="btn btn-primary" data-i18n="ui.upload">Upload</button>
              </div>
            </form>
          </div>
        </div>
        "#,
        script,
    )
}
