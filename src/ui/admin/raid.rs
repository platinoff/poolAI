//! RAID Management page
//!
//! Provides artifact storage and replication management.
//! PH-S214: raid page uses slim `admin_layout_raid` + `admin_raid_patch`.

use crate::ui::admin::admin_layout_raid;
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
    function raidClusterBadgeClass(status) {
      const s = String(status || '').toLowerCase();
      if (s === 'healthy') return 'active';
      if (s === 'degraded') return 'pending';
      return 'inactive';
    }
    function raftRoleBadgeClass(role) {
      const r = String(role || '').toLowerCase();
      if (r === 'leader') return 'active';
      if (r === 'candidate') return 'pending';
      return 'inactive';
    }

    var raidBurstPctHistory = [];
    var raidSwClusterHistory = [];
    var RAID_METRICS_SPARK_MAX = 20;

    function raidHistoryPush(buf, val) {
      const n = Number(val);
      if (!Number.isFinite(n)) return;
      buf.push(n);
      if (buf.length > RAID_METRICS_SPARK_MAX) buf.shift();
    }

    function raidMetricBar(pct) {
      const p = Math.max(0, Math.min(100, Number(pct) || 0));
      return '<div class="raid-metric-bar" role="progressbar" aria-valuenow="' + p + '" aria-valuemin="0" aria-valuemax="100">'
        + '<div class="raid-metric-bar-fill" style="width:' + p + '%"></div></div>';
    }

    function renderRaidBurstSection(burstMetrics) {
      const title = escapeHtml(T('admin.raidadm.burstTitle', 'BurstRAID Metrics'));
      if (!burstMetrics) {
        return '<div class="admin-card raid-metrics-card" id="raid-burst-metrics">'
          + '<h3>' + title + '</h3>'
          + '<p class="muted">' + escapeHtml(T('admin.raidadm.burstInactive', 'BurstRAID strategy not active on this node.')) + '</p></div>';
      }
      const total = burstMetrics.total_artifacts ?? 0;
      const inBurst = burstMetrics.artifacts_in_burst ?? 0;
      const burstPct = total > 0 ? (inBurst / total) * 100 : 0;
      raidHistoryPush(raidBurstPctHistory, burstPct);
      const spark = typeof poolaiRenderSparkline === 'function'
        ? poolaiRenderSparkline(T('admin.raidadm.sparkBurstPct', 'Burst load %'), raidBurstPctHistory.slice())
        : '';
      return '<div class="admin-card raid-metrics-card" id="raid-burst-metrics">'
        + '<h3>' + title + '</h3>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.totalArt', 'Total Artifacts:')) + '</span>'
        + '<span class="stat-value">' + total + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.artBurst', 'Artifacts in Burst:')) + '</span>'
        + '<span class="stat-value">' + inBurst + ' (' + burstPct.toFixed(1) + '%)</span></div>'
        + raidMetricBar(burstPct)
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.totalReq', 'Total Requests:')) + '</span>'
        + '<span class="stat-value">' + (burstMetrics.total_requests ?? 0) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.burstRps', 'Burst threshold (RPS):')) + '</span>'
        + '<span class="stat-value">' + Number(burstMetrics.burst_threshold_rps ?? 0).toFixed(2) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.repl', 'Replication (base / max):')) + '</span>'
        + '<span class="stat-value">' + (burstMetrics.base_replication_factor ?? 0) + ' / ' + (burstMetrics.max_replication_factor ?? 0) + '</span></div>'
        + (spark ? '<div class="metrics-sparklines-grid">' + spark + '</div>' : '')
        + '</div>';
    }

    function renderRaidSmallworldSection(smallworldMetrics) {
      const title = escapeHtml(T('admin.raidadm.swTitle', 'SmallWorld Network Metrics'));
      if (!smallworldMetrics) {
        return '<div class="admin-card raid-metrics-card" id="raid-smallworld-metrics">'
          + '<h3>' + title + '</h3>'
          + '<p class="muted">' + escapeHtml(T('admin.raidadm.swInactive', 'SmallWorld strategy not active on this node.')) + '</p></div>';
      }
      const avg = smallworldMetrics.avg_clustering_coefficient ?? 0;
      const tgt = smallworldMetrics.target_clustering_coefficient ?? 0;
      const clustPct = tgt > 0 ? Math.min(100, (avg / tgt) * 100) : 0;
      raidHistoryPush(raidSwClusterHistory, avg * 100);
      const spark = typeof poolaiRenderSparkline === 'function'
        ? poolaiRenderSparkline(T('admin.raidadm.sparkClust', 'Avg clustering ×100'), raidSwClusterHistory.slice())
        : '';
      return '<div class="admin-card raid-metrics-card" id="raid-smallworld-metrics">'
        + '<h3>' + title + '</h3>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.totalArt', 'Total Artifacts:')) + '</span>'
        + '<span class="stat-value">' + (smallworldMetrics.total_artifacts ?? 0) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.totalNodes', 'Total Nodes:')) + '</span>'
        + '<span class="stat-value">' + (smallworldMetrics.total_nodes ?? 0) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.avgClust', 'Avg Clustering Coefficient:')) + '</span>'
        + '<span class="stat-value">' + avg.toFixed(3) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.tgtClust', 'Target Clustering:')) + '</span>'
        + '<span class="stat-value">' + tgt.toFixed(3) + '</span></div>'
        + '<div class="stat-item"><span class="stat-label">' + escapeHtml(T('admin.raidadm.label.swRepl', 'Base replication:')) + '</span>'
        + '<span class="stat-value">' + (smallworldMetrics.base_replication_factor ?? 0) + '</span></div>'
        + raidMetricBar(clustPct)
        + (spark ? '<div class="metrics-sparklines-grid">' + spark + '</div>' : '')
        + '</div>';
    }

    async function loadRaidData() {
      adminShowLoading('raid-admin', T('admin.raidadm.loading', 'Loading RAID admin…'));
      adminShowLoading('raid-artifacts', T('admin.raidadm.loadingArt', 'Loading artifacts…'));
      try {
        const [artifacts, snapshot, statusRaw, clusterRaw, burstRaw, smallworldRaw] = await Promise.all([
          fetchJson('/api/v1/raid/artifacts'),
          loadSnapshot().catch(() => null),
          fetchJson('/api/v1/raid/admin/status').catch(() => null),
          fetchJson('/api/v1/raid/status').catch(() => null),
          fetchJson('/api/v1/raid/admin/metrics/burst').catch(() => null),
          fetchJson('/api/v1/raid/admin/metrics/smallworld').catch(() => null)
        ]);
        const status = raidStrategyStatusFromResponse(statusRaw);
        const burstMetrics = raidMetricsFromResponse(burstRaw);
        const smallworldMetrics = raidMetricsFromResponse(smallworldRaw);
        renderRaidArtifacts(artifacts, snapshot);
        renderRaidAdmin(status, clusterRaw, burstMetrics, smallworldMetrics);
      } catch (e) {
        adminShowInlineError('raid-admin', e);
        adminShowInlineError('raid-artifacts', e);
        showNotification(T('admin.raidadm.errLoad', 'Error loading RAID data: ') + e.message, 'error');
      }
    }
    
    function renderRaidAdmin(status, cluster, burstMetrics, smallworldMetrics) {
      const el = document.getElementById('raid-admin');
      if (!el) return;
      
      let html = '';

      if (cluster && typeof cluster === 'object') {
        const raft = cluster.raft_status;
        const storage = cluster.storage || {};
        const usagePct = storage.usage_percent != null
          ? Number(storage.usage_percent).toFixed(1) + '%'
          : T('admin.na', 'N/A');
        html += `
          <div class="admin-card" id="raid-cluster-status">
            <h3>${escapeHtml(T('admin.raidadm.clusterTitle', 'Cluster status'))}</h3>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.cluster', 'Cluster:'))}</span>
              <span class="stat-value status-badge ${raidClusterBadgeClass(cluster.cluster_status)}">${escapeHtml(cluster.cluster_status || T('admin.raidadm.unknown', 'Unknown'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.mode', 'Mode:'))}</span>
              <span class="stat-value">${escapeHtml(cluster.mode || T('admin.raidadm.unknown', 'Unknown'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.nodes', 'Nodes:'))}</span>
              <span class="stat-value">${cluster.node_count ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.artCount', 'Artifacts:'))}</span>
              <span class="stat-value">${cluster.artifact_count ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.storageUse', 'Storage used:'))}</span>
              <span class="stat-value">${escapeHtml(usagePct)}</span>
            </div>
            ${cluster.replication_status ? `
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.replStatus', 'Replication:'))}</span>
              <span class="stat-value">${escapeHtml(cluster.replication_status)}</span>
            </div>` : ''}
            <h4>${escapeHtml(T('admin.raidadm.raftTitle', 'Raft consensus'))}</h4>
            ${raft && typeof raft === 'object' ? `
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.raftRole', 'Role:'))}</span>
              <span class="stat-value status-badge ${raftRoleBadgeClass(raft.role)}">${escapeHtml(raft.role || T('admin.raidadm.unknown', 'Unknown'))}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.raftTerm', 'Term:'))}</span>
              <span class="stat-value">${raft.term ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">${escapeHtml(T('admin.raidadm.label.raftLeader', 'Leader ID:'))}</span>
              <span class="stat-value">${escapeHtml(raft.leader_id != null ? String(raft.leader_id) : T('admin.na', 'N/A'))}</span>
            </div>` : `
            <div class="muted">${escapeHtml(T('admin.raidadm.raftAbsent', 'Raft not attached (build without consensus or node not initialized).'))}</div>`}
          </div>
        `;
      }
      
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
      
      html += renderRaidBurstSection(burstMetrics);
      html += renderRaidSmallworldSection(smallworldMetrics);

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
                  <td>${escapeHtml(
                    (typeof window !== 'undefined' && window.poolaiUiWasm && typeof window.poolaiUiWasm.formatBytes === 'function')
                      ? window.poolaiUiWasm.formatBytes(a.size || 0)
                      : formatBytes(a.size || 0)
                  )}</td>
                  <td>
                    <button type="button" class="btn btn-danger" onclick='deleteArtifact(${JSON.stringify(aid)})'>${escapeHtml(T('ui.delete', 'Delete'))}</button>
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

    admin_layout_raid(
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
              <button type="button" class="btn" onclick="triggerRebalance()" data-i18n="admin.raidadm.btn.rebalance" data-i18n-aria="admin.raidadm.btn.rebalance">Rebalance</button>
            </div>
          </div>
          <div id="raid-admin"></div>
          <div id="raid-artifacts"></div>
        </div>
        
        <div id="uploadArtifactModal" class="modal" role="dialog" aria-labelledby="uploadArtifactModalTitle" aria-modal="false" aria-hidden="true">
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

#[tokio::test]
async fn admin_raid_page_slim_raid_i18n_patch_ph_s214() {
    let html = admin_raid().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.raid""#));
    assert!(html.contains(r#""admin.raidadm.section""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.gridPricing.col.price""#));
}
