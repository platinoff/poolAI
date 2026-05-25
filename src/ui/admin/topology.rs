//! Topology management admin page (FM-037: SVG graph + latency heatmap).

use super::admin_layout;

/// Admin topology page
pub async fn admin_topology() -> axum::response::Html<String> {
    let graph_js = include_str!("../topology_graph.js");
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
        <h4 data-i18n="admin.topo.sectionGraph">Cluster Graph</h4>
        <button type="button" class="btn btn-primary" onclick="refreshTopology()" data-i18n="admin.topo.refresh">Refresh</button>
      </div>
      <div id="topology-graph-wrap" class="topology-graph-wrap">
        <svg id="topology-graph-svg" role="img" aria-labelledby="topology-graph-title" xmlns="http://www.w3.org/2000/svg">
          <title id="topology-graph-title" data-i18n="admin.topo.graphTitle">Cluster topology graph</title>
        </svg>
        <div class="topology-graph-legend" data-i18n="admin.topo.graphLegend">
          Edge color: lower latency (greener) → higher (warmer). Node size: load.
        </div>
      </div>

      <div class="admin-section-header">
        <h4 data-i18n="admin.topo.sectionOverview">Topology Overview</h4>
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
        <h4 data-i18n="admin.topo.sectionHeatmap">Latency Heatmap</h4>
      </div>
      <div id="topology-latency-heatmap" class="admin-table-container">
        <p class="muted" data-i18n="admin.topo.loading">Loading…</p>
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

    let script = format!(
        r#"
    function T(k, fb) {{ return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }}

    let topologyNodesCache = {{}};
    let topologyLatencyCache = {{}};
    let topologyWs = null;
    let topologyWsRetryMs = 1000;
    let topologyWsMaxRetryMs = 30000;

    function renderNodesTableFromCache() {{
      const tbody = document.getElementById('topology-nodes-tbody');
      tbody.innerHTML = '';
      if (Object.keys(topologyNodesCache).length === 0) {{
        tbody.innerHTML = '<tr><td colspan="5">' + escapeHtml(T('admin.topo.noNodes', 'No nodes found')) + '</td></tr>';
        return;
      }}
      for (const [nodeId, node] of Object.entries(topologyNodesCache)) {{
        const row = document.createElement('tr');
        const ag = node.available_gpu_memory_mb ?? 0;
        const tg = node.total_gpu_memory_mb ?? 0;
        const ac = node.available_cpu_cores ?? 0;
        const tc = node.total_cpu_cores ?? 0;
        row.innerHTML = `
          <td>${{escapeHtml(nodeId)}}</td>
          <td>${{ag}} / ${{tg}} MB</td>
          <td>${{ac}} / ${{tc}}</td>
          <td>${{formatLoadFraction(node.current_load)}}</td>
          <td>
            <button type="button" class="btn btn-sm" onclick='viewNodeResources(${{JSON.stringify(nodeId)}})'>${{escapeHtml(T('admin.topo.viewDetails', 'View Details'))}}</button>
          </td>
        `;
        tbody.appendChild(row);
      }}
    }}

    function renderLatencyTableFromCache() {{
      const tbody = document.getElementById('topology-latency-tbody');
      tbody.innerHTML = '';
      if (Object.keys(topologyLatencyCache).length === 0) {{
        tbody.innerHTML = '<tr><td colspan="3">' + escapeHtml(T('admin.topo.noLatency', 'No latency measurements available')) + '</td></tr>';
        return;
      }}
      for (const [key, latency] of Object.entries(topologyLatencyCache)) {{
        const [fromNode, toNode] = key.split(':');
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${{escapeHtml(fromNode)}}</td>
          <td>${{escapeHtml(toNode)}}</td>
          <td>${{formatLatencyMs(latency)}}</td>
        `;
        tbody.appendChild(row);
      }}
    }}

    function applyTopologyLiveUpdate(data) {{
      if (!data) return;
      document.getElementById('topology-node-count').textContent = data.node_count || 0;
      document.getElementById('topology-latency-measurements').textContent = data.latency_measurements || 0;
      document.getElementById('topology-last-updated').textContent = formatTopologyTimestamp(data.last_updated);
      topologyNodesCache = data.nodes || {{}};
      topologyLatencyCache = data.latency_matrix || {{}};
      renderNodesTableFromCache();
      renderLatencyTableFromCache();
      renderTopologyVisualizations();
    }}

    function topologyWsUrl() {{
      const token = localStorage.getItem('poolai_token');
      if (!token) return null;
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
      return proto + '//' + location.host + '/api/v1/ws/metrics?token=' + encodeURIComponent(token);
    }}

    function connectTopologyWebSocket() {{
      const url = topologyWsUrl();
      if (!url) return;
      if (topologyWs && (topologyWs.readyState === WebSocket.OPEN || topologyWs.readyState === WebSocket.CONNECTING)) {{
        return;
      }}
      try {{
        topologyWs = new WebSocket(url);
      }} catch (err) {{
        console.warn('Topology WebSocket connect failed:', err);
        scheduleTopologyWsReconnect();
        return;
      }}
      topologyWs.onopen = function () {{
        topologyWsRetryMs = 1000;
        topologyWs.send(JSON.stringify({{
          message_type: 'subscribe_topology',
          data: {{}},
          timestamp: Math.floor(Date.now() / 1000),
        }}));
      }};
      topologyWs.onmessage = function (event) {{
        try {{
          const msg = JSON.parse(event.data);
          if (msg && msg.message_type === 'topology_update') {{
            applyTopologyLiveUpdate(msg.data);
          }}
        }} catch (err) {{
          console.warn('Topology WebSocket message parse error:', err);
        }}
      }};
      topologyWs.onclose = function () {{
        topologyWs = null;
        scheduleTopologyWsReconnect();
      }};
      topologyWs.onerror = function () {{
        if (topologyWs) {{
          topologyWs.close();
        }}
      }};
    }}

    function scheduleTopologyWsReconnect() {{
      setTimeout(function () {{
        connectTopologyWebSocket();
      }}, topologyWsRetryMs);
      topologyWsRetryMs = Math.min(topologyWsMaxRetryMs, topologyWsRetryMs * 2);
    }}

    async function loadTopology() {{
      try {{
        const response = await fetch('/api/v1/topology');
        if (!response.ok) throw new Error('Failed to load topology');
        const data = await response.json();
        
        document.getElementById('topology-node-count').textContent = data.node_count || 0;
        document.getElementById('topology-latency-measurements').textContent = data.latency_measurements || 0;
        document.getElementById('topology-last-updated').textContent = formatTopologyTimestamp(data.last_updated);
        
        await loadTopologyNodes();
        await loadLatencyMatrix();
        renderTopologyVisualizations();
      }} catch (error) {{
        console.error('Error loading topology:', error);
        showNotification(T('admin.topo.errLoad', 'Error loading topology: ') + error.message, 'error');
      }}
    }}

    async function loadTopologyNodes() {{
      try {{
        const response = await fetch('/api/v1/topology/nodes');
        if (!response.ok) throw new Error('Failed to load nodes');
        const data = await response.json();
        topologyNodesCache = data.nodes || {{}};
        renderNodesTableFromCache();
      }} catch (error) {{
        console.error('Error loading nodes:', error);
        document.getElementById('topology-nodes-tbody').innerHTML = '<tr><td colspan="5">' + escapeHtml(T('admin.topo.errNodesRow', 'Error loading nodes')) + '</td></tr>';
      }}
    }}

    async function loadLatencyMatrix() {{
      try {{
        const response = await fetch('/api/v1/topology/latency');
        if (!response.ok) throw new Error('Failed to load latency matrix');
        const data = await response.json();
        topologyLatencyCache = data.latency_matrix || {{}};
        renderLatencyTableFromCache();
      }} catch (error) {{
        console.error('Error loading latency matrix:', error);
        document.getElementById('topology-latency-tbody').innerHTML = '<tr><td colspan="3">' + escapeHtml(T('admin.topo.errLatencyRow', 'Error loading latency matrix')) + '</td></tr>';
      }}
    }}

    function renderTopologyVisualizations() {{
      if (typeof PoolAiTopologyGraph === 'undefined') return;
      const svg = document.getElementById('topology-graph-svg');
      const heatmap = document.getElementById('topology-latency-heatmap');
      const wrap = document.getElementById('topology-graph-wrap');
      const width = wrap ? Math.max(320, wrap.clientWidth - 32) : 640;
      PoolAiTopologyGraph.render(svg, topologyNodesCache, topologyLatencyCache, {{
        width: width,
        height: 360,
        graphTitle: T('admin.topo.graphTitle', 'Cluster topology graph'),
        emptyLabel: T('admin.topo.noNodes', 'No nodes found'),
        onNodeClick: function (nodeId) {{ viewNodeResources(nodeId); }},
      }});
      PoolAiTopologyGraph.renderLatencyHeatmap(heatmap, topologyNodesCache, topologyLatencyCache);
      if (!Object.keys(topologyNodesCache).length) {{
        heatmap.innerHTML = '<p class="muted">' + escapeHtml(T('admin.topo.noNodes', 'No nodes found')) + '</p>';
      }}
    }}

    async function viewNodeResources(nodeId) {{
      try {{
        const response = await fetch(`/api/v1/topology/nodes/${{encodeURIComponent(nodeId)}}`);
        if (!response.ok) throw new Error('Failed to load node resources');
        const node = await response.json();
        
        const modalContent = `
          <h3>${{escapeHtml(T('admin.topo.modalNodeTitle', 'Node Resources'))}}: ${{escapeHtml(nodeId)}}</h3>
          <div class="form-group">
            <label>${{escapeHtml(T('admin.topo.lbl.gpu', 'GPU Memory:'))}}</label>
            <div>${{node.available_gpu_memory_mb ?? 0}} / ${{node.total_gpu_memory_mb ?? 0}} MB</div>
          </div>
          <div class="form-group">
            <label>${{escapeHtml(T('admin.topo.lbl.cpu', 'CPU Cores:'))}}</label>
            <div>${{node.available_cpu_cores ?? 0}} / ${{node.total_cpu_cores ?? 0}}</div>
          </div>
          <div class="form-group">
            <label>${{escapeHtml(T('admin.topo.lbl.sysMem', 'System Memory:'))}}</label>
            <div>${{node.available_memory_mb ?? 0}} / ${{node.total_memory_mb ?? 0}} MB</div>
          </div>
          <div class="form-group">
            <label>${{escapeHtml(T('admin.topo.lbl.load', 'Current Load:'))}}</label>
            <div>${{formatLoadFraction(node.current_load)}}</div>
          </div>
        `;
        showModal(T('admin.topo.modalNodeTitle', 'Node Resources'), modalContent);
      }} catch (error) {{
        showNotification(T('admin.topo.errNodeRes', 'Error loading node resources: ') + error.message, 'error');
      }}
    }}

    function refreshTopology() {{
      loadTopology();
      showNotification(T('admin.topo.refreshInfo', 'Topology refresh initiated'), 'info');
    }}

    function formatTopologyTimestamp(iso) {{
      if (iso == null || iso === '') return '-';
      const t = Date.parse(iso);
      return Number.isFinite(t) ? new Date(t).toLocaleString() : '-';
    }}

    function formatLoadFraction(x) {{
      const n = Number(x);
      if (!Number.isFinite(n)) return '-';
      return (n * 100).toFixed(1) + '%';
    }}

    function formatLatencyMs(latency) {{
      const n = Number(latency);
      if (!Number.isFinite(n)) return escapeHtml(String(latency)) + ' ms';
      return n.toFixed(2) + ' ms';
    }}

    {graph_js}
    loadTopology();
    connectTopologyWebSocket();
    "#,
        graph_js = graph_js
    );

    admin_layout("admin.page.topology", "Topology", body, &script)
}

#[cfg(test)]
mod fm037_tests {
    use super::admin_topology;

    #[tokio::test]
    async fn topology_page_includes_graph_and_heatmap_fm037() {
        let html = admin_topology().await.0;
        assert!(html.contains("id=\"topology-graph-svg\""));
        assert!(html.contains("id=\"topology-latency-heatmap\""));
        assert!(html.contains("PoolAiTopologyGraph"));
        assert!(html.contains("renderTopologyVisualizations"));
        assert!(html.contains("connectTopologyWebSocket"));
        assert!(html.contains("subscribe_topology"));
    }
}
