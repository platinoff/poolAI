/**
 * FM-037 / PH-S157 — topology SVG paint from Rust layout JSON (`GET /api/v1/topology/graph`).
 */
(function (global) {
  'use strict';

  function renderLayout(svgEl, layout, options) {
    if (!svgEl || !layout) return;
    const opts = options || {};
    const width = layout.width || opts.width || svgEl.clientWidth || 640;
    const height = layout.height || opts.height || 360;
    svgEl.setAttribute('viewBox', '0 0 ' + width + ' ' + height);
    svgEl.setAttribute('width', String(width));
    svgEl.setAttribute('height', String(height));
    svgEl.innerHTML = '';

    const titleEl = document.createElementNS('http://www.w3.org/2000/svg', 'title');
    titleEl.setAttribute('id', 'topology-graph-title');
    titleEl.textContent = opts.graphTitle || 'Cluster topology graph';
    svgEl.appendChild(titleEl);

    if (layout.empty || !layout.nodes || !layout.nodes.length) {
      const empty = document.createElementNS('http://www.w3.org/2000/svg', 'text');
      empty.setAttribute('x', String(width / 2));
      empty.setAttribute('y', String(height / 2));
      empty.setAttribute('text-anchor', 'middle');
      empty.setAttribute('fill', 'var(--text-muted, #a8b0bf)');
      empty.setAttribute('font-size', '14');
      empty.textContent = opts.emptyLabel || 'No topology nodes';
      svgEl.appendChild(empty);
      return layout;
    }

    const gLinks = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    gLinks.setAttribute('class', 'topology-graph-links');
    (layout.links || []).forEach(function (link) {
      const from = layout.nodes.find(function (n) { return n.id === link.from; });
      const to = layout.nodes.find(function (n) { return n.id === link.to; });
      if (!from || !to) return;
      const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
      line.setAttribute('x1', String(from.x));
      line.setAttribute('y1', String(from.y));
      line.setAttribute('x2', String(to.x));
      line.setAttribute('y2', String(to.y));
      line.setAttribute('stroke', link.stroke || 'var(--primary, #67e480)');
      line.setAttribute('stroke-width', String(link.stroke_width || 1));
      line.setAttribute('stroke-opacity', '0.75');
      gLinks.appendChild(line);
    });
    svgEl.appendChild(gLinks);

    const gNodes = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    gNodes.setAttribute('class', 'topology-graph-nodes');
    layout.nodes.forEach(function (n) {
      const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
      g.setAttribute('class', 'topology-graph-node');
      g.style.cursor = 'pointer';
      const r = n.radius || 10;
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
      label.textContent = n.label || n.id;
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
    return layout;
  }

  global.PoolAiTopologyGraph = {
    renderLayout: renderLayout,
  };
})(typeof window !== 'undefined' ? window : globalThis);
