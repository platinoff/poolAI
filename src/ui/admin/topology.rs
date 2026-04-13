//! Topology management admin page

use super::admin_layout;

/// Admin topology page
pub async fn admin_topology() -> axum::response::Html<String> {
    let body = r#"
    <div class="admin-section">
      <h3 data-i18n="admin.topo.title">Network Topology</h3>
      <p data-i18n="admin.topo.intro">View network topology, latency matrix, and node resources.</p>
      
      <div class="admin-stats-grid">
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-node-count">-</div>
          <div class="stat-label" data-i18n="admin.topo.stat.nodes">Nodes</div>
        </div>
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-latency-measurements">-</div>
          <div class="stat-label" data-i18n="admin.topo.stat.latencyMs">Latency Measurements</div>
        </div>
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-last-updated">-</div>
          <div class="stat-label" data-i18n="admin.topo.stat.lastUpd">Last Updated</div>
        </div>
      </div>

      <div class="admin-section-header">
        <h4 data-i18n="admin.topo.sectionOverview">Topology Overview</h4>
        <button type="button" class="btn btn-primary" onclick="refreshTopology()" data-i18n="admin.topo.refresh">Refresh</button>
      </div>
      
      <div id="topology-nodes-list" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th data-i18n="admin.topo.col.nodeId">Node ID</th>
              <th data-i18n="admin.topo.col.gpuMem">Available GPU Memory</th>
              <th data-i18n="admin.topo.col.cpu">Available CPU Cores</th>
              <th data-i18n="admin.topo.col.load">Current Load</th>
              <th data-i18n="admin.topo.col.actions">Actions</th>
            </tr>
          </thead>
          <tbody id="topology-nodes-tbody">
            <tr><td colspan="5" data-i18n="admin.topo.loading">Loading…</td></tr>
          </tbody>
        </table>
      </div>

      <div class="admin-section-header">
        <h4 data-i18n="admin.topo.sectionLatency">Latency Matrix</h4>
      </div>
      <div id="topology-latency-matrix" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th data-i18n="admin.topo.col.from">From Node</th>
              <th data-i18n="admin.topo.col.to">To Node</th>
              <th data-i18n="admin.topo.col.latency">Latency (ms)</th>
            </tr>
          </thead>
          <tbody id="topology-latency-tbody">
            <tr><td colspan="3" data-i18n="admin.topo.loading">Loading…</td></tr>
          </tbody>
        </table>
      </div>
    </div>
  "#;

    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    async function loadTopology() {
      try {
        const response = await fetch('/api/v1/topology');
        if (!response.ok) throw new Error('Failed to load topology');
        const data = await response.json();
        
        document.getElementById('topology-node-count').textContent = data.node_count || 0;
        document.getElementById('topology-latency-measurements').textContent = data.latency_measurements || 0;
        document.getElementById('topology-last-updated').textContent = formatTopologyTimestamp(data.last_updated);
        
        await loadTopologyNodes();
        await loadLatencyMatrix();
      } catch (error) {
        console.error('Error loading topology:', error);
        showNotification(T('admin.topo.errLoad', 'Error loading topology: ') + error.message, 'error');
      }
    }

    async function loadTopologyNodes() {
      try {
        const response = await fetch('/api/v1/topology/nodes');
        if (!response.ok) throw new Error('Failed to load nodes');
        const data = await response.json();
        
        const tbody = document.getElementById('topology-nodes-tbody');
        tbody.innerHTML = '';
        
        if (Object.keys(data.nodes || {}).length === 0) {
          tbody.innerHTML = '<tr><td colspan="5">' + escapeHtml(T('admin.topo.noNodes', 'No nodes found')) + '</td></tr>';
          return;
        }
        
        for (const [nodeId, node] of Object.entries(data.nodes)) {
          const row = document.createElement('tr');
          const ag = node.available_gpu_memory_mb ?? 0;
          const tg = node.total_gpu_memory_mb ?? 0;
          const ac = node.available_cpu_cores ?? 0;
          const tc = node.total_cpu_cores ?? 0;
          row.innerHTML = `
            <td>${escapeHtml(nodeId)}</td>
            <td>${ag} / ${tg} MB</td>
            <td>${ac} / ${tc}</td>
            <td>${formatLoadFraction(node.current_load)}</td>
            <td>
              <button type="button" class="btn btn-sm" onclick="viewNodeResources(${JSON.stringify(nodeId)})">${escapeHtml(T('admin.topo.viewDetails', 'View Details'))}</button>
            </td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading nodes:', error);
        document.getElementById('topology-nodes-tbody').innerHTML = '<tr><td colspan="5">' + escapeHtml(T('admin.topo.errNodesRow', 'Error loading nodes')) + '</td></tr>';
      }
    }

    async function loadLatencyMatrix() {
      try {
        const response = await fetch('/api/v1/topology/latency');
        if (!response.ok) throw new Error('Failed to load latency matrix');
        const data = await response.json();
        
        const tbody = document.getElementById('topology-latency-tbody');
        tbody.innerHTML = '';
        
        if (Object.keys(data.latency_matrix || {}).length === 0) {
          tbody.innerHTML = '<tr><td colspan="3">' + escapeHtml(T('admin.topo.noLatency', 'No latency measurements available')) + '</td></tr>';
          return;
        }
        
        for (const [key, latency] of Object.entries(data.latency_matrix)) {
          const [fromNode, toNode] = key.split(':');
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(fromNode)}</td>
            <td>${escapeHtml(toNode)}</td>
            <td>${formatLatencyMs(latency)}</td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading latency matrix:', error);
        document.getElementById('topology-latency-tbody').innerHTML = '<tr><td colspan="3">' + escapeHtml(T('admin.topo.errLatencyRow', 'Error loading latency matrix')) + '</td></tr>';
      }
    }

    async function viewNodeResources(nodeId) {
      try {
        const response = await fetch(`/api/v1/topology/nodes/${encodeURIComponent(nodeId)}`);
        if (!response.ok) throw new Error('Failed to load node resources');
        const node = await response.json();
        
        const modalContent = `
          <h3>${escapeHtml(T('admin.topo.modalNodeTitle', 'Node Resources'))}: ${escapeHtml(nodeId)}</h3>
          <div class="form-group">
            <label>${escapeHtml(T('admin.topo.lbl.gpu', 'GPU Memory:'))}</label>
            <div>${node.available_gpu_memory_mb ?? 0} / ${node.total_gpu_memory_mb ?? 0} MB</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.topo.lbl.cpu', 'CPU Cores:'))}</label>
            <div>${node.available_cpu_cores ?? 0} / ${node.total_cpu_cores ?? 0}</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.topo.lbl.sysMem', 'System Memory:'))}</label>
            <div>${node.available_memory_mb ?? 0} / ${node.total_memory_mb ?? 0} MB</div>
          </div>
          <div class="form-group">
            <label>${escapeHtml(T('admin.topo.lbl.load', 'Current Load:'))}</label>
            <div>${formatLoadFraction(node.current_load)}</div>
          </div>
        `;
        showModal(T('admin.topo.modalNodeTitle', 'Node Resources'), modalContent);
      } catch (error) {
        showNotification(T('admin.topo.errNodeRes', 'Error loading node resources: ') + error.message, 'error');
      }
    }

    function refreshTopology() {
      loadTopology();
      showNotification(T('admin.topo.refreshInfo', 'Topology refresh initiated'), 'info');
    }

    function formatTopologyTimestamp(iso) {
      if (iso == null || iso === '') return '-';
      const t = Date.parse(iso);
      return Number.isFinite(t) ? new Date(t).toLocaleString() : '-';
    }

    function formatLoadFraction(x) {
      const n = Number(x);
      if (!Number.isFinite(n)) return '-';
      return (n * 100).toFixed(1) + '%';
    }

    function formatLatencyMs(latency) {
      const n = Number(latency);
      if (!Number.isFinite(n)) return escapeHtml(String(latency)) + ' ms';
      return n.toFixed(2) + ' ms';
    }

    loadTopology();
  "#;

    admin_layout("admin.page.topology", "Topology", body, script)
}
