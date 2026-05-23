/**
 * FM-037 — cluster topology graph (vanilla SVG force layout, no external CDN).
 * Used by admin topology page (`src/ui/admin/topology.rs`).
 */
(function (global) {
  'use strict';

  function buildGraph(nodesMap, latencyMatrix) {
    const nodeIds = Object.keys(nodesMap || {}).sort();
    const nodes = nodeIds.map(function (id, i) {
      const n = nodesMap[id] || {};
      const angle = nodeIds.length ? (2 * Math.PI * i) / nodeIds.length : 0;
      return {
        id: id,
        label: id,
        load: Number(n.current_load) || 0,
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
        seedAngle: angle,
      };
    });
    const links = [];
    Object.entries(latencyMatrix || {}).forEach(function (entry) {
      const key = entry[0];
      const lat = Number(entry[1]);
      const parts = key.split(':');
      if (parts.length !== 2) return;
      const from = parts[0];
      const to = parts[1];
      if (!from || !to || from === to) return;
      links.push({ from: from, to: to, latency: lat });
    });
    return { nodes: nodes, links: links };
  }

  function layoutGraph(graph, width, height, iterations) {
    const nodes = graph.nodes;
    const links = graph.links;
    const cx = width / 2;
    const cy = height / 2;
    const radius = Math.min(width, height) * 0.32;
    nodes.forEach(function (n, i) {
      const a = n.seedAngle != null ? n.seedAngle : (2 * Math.PI * i) / Math.max(nodes.length, 1);
      n.x = cx + radius * Math.cos(a);
      n.y = cy + radius * Math.sin(a);
      n.vx = 0;
      n.vy = 0;
    });
    const nodeById = {};
    nodes.forEach(function (n) {
      nodeById[n.id] = n;
    });
    const iters = iterations || 80;
    for (let k = 0; k < iters; k++) {
      for (let i = 0; i < nodes.length; i++) {
        for (let j = i + 1; j < nodes.length; j++) {
          const a = nodes[i];
          const b = nodes[j];
          let dx = a.x - b.x;
          let dy = a.y - b.y;
          let dist = Math.sqrt(dx * dx + dy * dy) || 0.01;
          const repulse = 4200 / (dist * dist);
          dx = (dx / dist) * repulse;
          dy = (dy / dist) * repulse;
          a.vx += dx;
          a.vy += dy;
          b.vx -= dx;
          b.vy -= dy;
        }
      }
      links.forEach(function (link) {
        const a = nodeById[link.from];
        const b = nodeById[link.to];
        if (!a || !b) return;
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let dist = Math.sqrt(dx * dx + dy * dy) || 0.01;
        const strength = Math.min(dist * 0.04, 12);
        dx = (dx / dist) * strength;
        dy = (dy / dist) * strength;
        a.vx += dx;
        a.vy += dy;
        b.vx -= dx;
        b.vy -= dy;
      });
      nodes.forEach(function (n) {
        n.vx += (cx - n.x) * 0.002;
        n.vy += (cy - n.y) * 0.002;
        n.vx *= 0.85;
        n.vy *= 0.85;
        n.x += n.vx;
        n.y += n.vy;
        n.x = Math.max(40, Math.min(width - 40, n.x));
        n.y = Math.max(40, Math.min(height - 40, n.y));
      });
    }
    return graph;
  }

  function latencyColor(latency, maxLat) {
    const max = maxLat > 0 ? maxLat : 1;
    const t = Math.min(1, Math.max(0, Number(latency) / max));
    const r = Math.round(80 + t * 175);
    const g = Math.round(200 - t * 140);
    const b = Math.round(120 - t * 80);
    return 'rgb(' + r + ',' + g + ',' + b + ')';
  }

  function render(svgEl, nodesMap, latencyMatrix, options) {
    if (!svgEl) return;
    const opts = options || {};
    const width = opts.width || svgEl.clientWidth || 640;
    const height = opts.height || 360;
    svgEl.setAttribute('viewBox', '0 0 ' + width + ' ' + height);
    svgEl.setAttribute('width', String(width));
    svgEl.setAttribute('height', String(height));
    svgEl.innerHTML = '';

    const graph = layoutGraph(buildGraph(nodesMap, latencyMatrix), width, height, opts.iterations);
    if (!graph.nodes.length) {
      const empty = document.createElementNS('http://www.w3.org/2000/svg', 'text');
      empty.setAttribute('x', String(width / 2));
      empty.setAttribute('y', String(height / 2));
      empty.setAttribute('text-anchor', 'middle');
      empty.setAttribute('fill', 'var(--text-muted, #a8b0bf)');
      empty.setAttribute('font-size', '14');
      empty.textContent = opts.emptyLabel || 'No topology nodes';
      svgEl.appendChild(empty);
      return graph;
    }

    const maxLat = graph.links.reduce(function (m, l) {
      return Math.max(m, l.latency || 0);
    }, 1);
    const nodeById = {};
    graph.nodes.forEach(function (n) {
      nodeById[n.id] = n;
    });

    const gLinks = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    gLinks.setAttribute('class', 'topology-graph-links');
    graph.links.forEach(function (link) {
      const a = nodeById[link.from];
      const b = nodeById[link.to];
      if (!a || !b) return;
      const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
      line.setAttribute('x1', String(a.x));
      line.setAttribute('y1', String(a.y));
      line.setAttribute('x2', String(b.x));
      line.setAttribute('y2', String(b.y));
      line.setAttribute('stroke', latencyColor(link.latency, maxLat));
      line.setAttribute('stroke-width', String(1 + Math.min(4, (link.latency || 0) / Math.max(maxLat, 1) * 4)));
      line.setAttribute('stroke-opacity', '0.75');
      gLinks.appendChild(line);
    });
    svgEl.appendChild(gLinks);

    const gNodes = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    gNodes.setAttribute('class', 'topology-graph-nodes');
    graph.nodes.forEach(function (n) {
      const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
      g.setAttribute('class', 'topology-graph-node');
      g.style.cursor = 'pointer';
      const r = 10 + Math.min(14, (n.load || 0) * 18);
      const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
      circle.setAttribute('cx', String(n.x));
      circle.setAttribute('cy', String(n.y));
      circle.setAttribute('r', String(r));
      circle.setAttribute('fill', 'var(--primary, #67e480)');
      circle.setAttribute('stroke', 'var(--surface-secondary, #1e2329)');
      circle.setAttribute('stroke-width', '2');
      const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
      label.setAttribute('x', String(n.x));
      label.setAttribute('y', String(n.y + r + 14));
      label.setAttribute('text-anchor', 'middle');
      label.setAttribute('fill', 'var(--text, #e8e8e8)');
      label.setAttribute('font-size', '11');
      label.textContent = n.label;
      g.appendChild(circle);
      g.appendChild(label);
      if (typeof opts.onNodeClick === 'function') {
        g.addEventListener('click', function () {
          opts.onNodeClick(n.id);
        });
      }
      gNodes.appendChild(g);
    });
    svgEl.appendChild(gNodes);
    return graph;
  }

  function renderLatencyHeatmap(container, nodesMap, latencyMatrix) {
    if (!container) return;
    const nodeIds = Object.keys(nodesMap || {}).sort();
    if (!nodeIds.length) {
      container.innerHTML = '';
      return;
    }
    const values = [];
    nodeIds.forEach(function (row) {
      nodeIds.forEach(function (col) {
        if (row === col) return;
        const key = row + ':' + col;
        const rev = col + ':' + row;
        const v = latencyMatrix[key] != null ? latencyMatrix[key] : latencyMatrix[rev];
        if (v != null) values.push(Number(v));
      });
    });
    const maxLat = values.length ? Math.max.apply(null, values) : 1;

    let html = '<table class="admin-table topology-heatmap-table"><thead><tr><th></th>';
    nodeIds.forEach(function (id) {
      html += '<th scope="col">' + escapeHtml(id) + '</th>';
    });
    html += '</tr></thead><tbody>';
    nodeIds.forEach(function (row) {
      html += '<tr><th scope="row">' + escapeHtml(row) + '</th>';
      nodeIds.forEach(function (col) {
        if (row === col) {
          html += '<td class="topo-heat-diagonal">—</td>';
          return;
        }
        const key = row + ':' + col;
        const rev = col + ':' + row;
        const raw = latencyMatrix[key] != null ? latencyMatrix[key] : latencyMatrix[rev];
        if (raw == null) {
          html += '<td class="topo-heat-empty">—</td>';
          return;
        }
        const lat = Number(raw);
        const t = Math.min(1, Math.max(0, lat / maxLat));
        const bg = latencyColor(lat, maxLat);
        html +=
          '<td class="topo-heat-cell" style="background:' +
          bg +
          '22" title="' +
          escapeHtml(row + ' → ' + col + ': ' + lat.toFixed(2) + ' ms') +
          '">' +
          escapeHtml(lat.toFixed(1)) +
          '</td>';
      });
      html += '</tr>';
    });
    html += '</tbody></table>';
    container.innerHTML = html;
  }

  function escapeHtml(s) {
    return String(s)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  global.PoolAiTopologyGraph = {
    buildGraph: buildGraph,
    layoutGraph: layoutGraph,
    render: render,
    renderLatencyHeatmap: renderLatencyHeatmap,
  };
})(typeof window !== 'undefined' ? window : globalThis);
