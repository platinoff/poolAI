//! Topology management admin page

use super::admin_layout;

/// Admin topology page
pub async fn admin_topology() -> axum::response::Html<String> {
    let body = r#"
    <div class="admin-section">
      <h3>Network Topology</h3>
      <p>View network topology, latency matrix, and node resources.</p>
      
      <div class="admin-stats-grid">
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-node-count">-</div>
          <div class="stat-label">Nodes</div>
        </div>
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-latency-measurements">-</div>
          <div class="stat-label">Latency Measurements</div>
        </div>
        <div class="admin-stat-card">
          <div class="stat-value" id="topology-last-updated">-</div>
          <div class="stat-label">Last Updated</div>
        </div>
      </div>

      <div class="admin-section-header">
        <h4>Topology Overview</h4>
        <button class="btn btn-primary" onclick="refreshTopology()">Refresh</button>
      </div>
      
      <div id="topology-nodes-list" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th>Node ID</th>
              <th>Available GPU Memory</th>
              <th>Available CPU Cores</th>
              <th>Current Load</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody id="topology-nodes-tbody">
            <tr><td colspan="5">Loading...</td></tr>
          </tbody>
        </table>
      </div>

      <div class="admin-section-header">
        <h4>Latency Matrix</h4>
      </div>
      <div id="topology-latency-matrix" class="admin-table-container">
        <table class="admin-table">
          <thead>
            <tr>
              <th>From Node</th>
              <th>To Node</th>
              <th>Latency (ms)</th>
            </tr>
          </thead>
          <tbody id="topology-latency-tbody">
            <tr><td colspan="3">Loading...</td></tr>
          </tbody>
        </table>
      </div>
    </div>
  "#;

    let script = r#"
    async function loadTopology() {
      try {
        const response = await fetch('/api/v1/topology');
        if (!response.ok) throw new Error('Failed to load topology');
        const data = await response.json();
        
        document.getElementById('topology-node-count').textContent = data.node_count || 0;
        document.getElementById('topology-latency-measurements').textContent = data.latency_measurements || 0;
        document.getElementById('topology-last-updated').textContent = new Date(data.last_updated).toLocaleString() || '-';
        
        // Load nodes
        await loadTopologyNodes();
        
        // Load latency matrix
        await loadLatencyMatrix();
      } catch (error) {
        console.error('Error loading topology:', error);
        showNotification('Error loading topology: ' + error.message, 'error');
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
          tbody.innerHTML = '<tr><td colspan="5">No nodes found</td></tr>';
          return;
        }
        
        for (const [nodeId, node] of Object.entries(data.nodes)) {
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(nodeId)}</td>
            <td>${node.available_gpu_memory_mb} / ${node.total_gpu_memory_mb} MB</td>
            <td>${node.available_cpu_cores} / ${node.total_cpu_cores}</td>
            <td>${(node.current_load * 100).toFixed(1)}%</td>
            <td>
              <button class="btn btn-sm" onclick="viewNodeResources('${escapeHtml(nodeId)}')">View Details</button>
            </td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading nodes:', error);
        document.getElementById('topology-nodes-tbody').innerHTML = '<tr><td colspan="5">Error loading nodes</td></tr>';
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
          tbody.innerHTML = '<tr><td colspan="3">No latency measurements available</td></tr>';
          return;
        }
        
        for (const [key, latency] of Object.entries(data.latency_matrix)) {
          const [fromNode, toNode] = key.split(':');
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${escapeHtml(fromNode)}</td>
            <td>${escapeHtml(toNode)}</td>
            <td>${latency.toFixed(2)} ms</td>
          `;
          tbody.appendChild(row);
        }
      } catch (error) {
        console.error('Error loading latency matrix:', error);
        document.getElementById('topology-latency-tbody').innerHTML = '<tr><td colspan="3">Error loading latency matrix</td></tr>';
      }
    }

    async function viewNodeResources(nodeId) {
      try {
        const response = await fetch(`/api/v1/topology/nodes/${encodeURIComponent(nodeId)}`);
        if (!response.ok) throw new Error('Failed to load node resources');
        const node = await response.json();
        
        const modalContent = `
          <h3>Node Resources: ${escapeHtml(nodeId)}</h3>
          <div class="form-group">
            <label>GPU Memory:</label>
            <div>${node.available_gpu_memory_mb} / ${node.total_gpu_memory_mb} MB</div>
          </div>
          <div class="form-group">
            <label>CPU Cores:</label>
            <div>${node.available_cpu_cores} / ${node.total_cpu_cores}</div>
          </div>
          <div class="form-group">
            <label>System Memory:</label>
            <div>${node.available_memory_mb} / ${node.total_memory_mb} MB</div>
          </div>
          <div class="form-group">
            <label>Current Load:</label>
            <div>${(node.current_load * 100).toFixed(1)}%</div>
          </div>
        `;
        showModal('Node Resources', modalContent);
      } catch (error) {
        showNotification('Error loading node resources: ' + error.message, 'error');
      }
    }

    function refreshTopology() {
      loadTopology();
      showNotification('Topology refresh initiated', 'info');
    }

    function escapeHtml(text) {
      const div = document.createElement('div');
      div.textContent = text;
      return div.innerHTML;
    }

    // Load topology on page load
    loadTopology();
  "#;

    admin_layout("Topology", body, script)
}
