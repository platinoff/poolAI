/* PoolAI docs vision — interactive manifest graph */
(function () {
  "use strict";

  const VISION_BASE = "/docs/vision/";
  const LAYER_Y = { L0: 95, L1: 195, L2: 295, L3: 395 };
  const LAYER_COLORS = {
    L0: "#3d6a9e",
    L1: "#3d6a4a",
    L2: "#8a7040",
    L3: "#8a4068",
  };

  const WATCH_INTERVAL_MS = 1500;

  let manifest = null;
  let extensions = null;
  let activeSprint = null;
  let sprintPathSet = null;
  let selectedId = null;
  let fullscreenPanel = null;
  let nodePositions = new Map();
  let watchState = null;
  let watchTimer = null;
  let autoReloadEnabled = true;
  let reloadInFlight = false;

  function repoUrl(relPath) {
    return "/" + relPath.replace(/^\//, "");
  }

  function fileExt(path) {
    const m = path.match(/\.([a-z0-9]+)$/i);
    return m ? m[1].toLowerCase() : "";
  }

  function extClass(path) {
    const e = fileExt(path);
    if (e === "md") return "ext-md";
    if (e === "rs") return "ext-rs";
    if (e === "json") return "ext-json";
    return "ext-other";
  }

  function extLabel(path) {
    const e = fileExt(path);
    return e ? "." + e : "?";
  }

  function nodeById(id) {
    return manifest.nodes.find((n) => n.id === id);
  }

  function sprintTokenMatches(token, sprint) {
    if (!token || !sprint) return false;
    if (token === sprint) return true;
    if (token.endsWith("*")) {
      return sprint.startsWith(token.slice(0, -1));
    }
    return false;
  }

  function nodeInActiveSprint(node) {
    if (!activeSprint || !node) return false;
    if (node.sprints && node.sprints.some((t) => sprintTokenMatches(t, activeSprint))) {
      return true;
    }
    return sprintPathSet && sprintPathSet.has(node.path);
  }

  function globToRegExp(glob) {
    const esc = glob.replace(/[.+^${}()|[\]\\]/g, "\\$&");
    const re = "^" + esc.replace(/\*\*/g, "§§").replace(/\*/g, "[^/]*").replace(/§§/g, ".*") + "$";
    return new RegExp(re);
  }

  function pathMatchesGlob(path, glob) {
    if (!glob) return false;
    if (glob.endsWith("/**")) {
      return path.startsWith(glob.slice(0, -3));
    }
    return globToRegExp(glob).test(path);
  }

  function buildSprintPathSet(ext, sprint) {
    const paths = new Set();
    if (!sprint) return paths;
    if (ext) {
      Object.values(ext.scopes || {}).forEach((scope) => {
        if (!(scope.sprints || []).some((t) => sprintTokenMatches(t, sprint))) return;
        (scope.docs || []).forEach((p) => paths.add(p.replace(/^\//, "")));
        const globs = [...(scope.code_globs || []), ...(scope.also_update || [])];
        manifest.nodes.forEach((n) => {
          if (globs.some((g) => pathMatchesGlob(n.path, g))) paths.add(n.path);
        });
      });
    }
    manifest.nodes.forEach((n) => {
      if (n.sprints && n.sprints.some((t) => sprintTokenMatches(t, sprint))) {
        paths.add(n.path);
      }
    });
    return paths;
  }

  function resolveActiveSprint(m, ext) {
    return (ext && ext.active_sprint) || m.next_sprint || null;
  }

  function updateSidebarSprintPill() {
    const pill = document.getElementById("sidebar-sprint");
    if (!pill) return;
    if (!activeSprint) {
      pill.hidden = true;
      return;
    }
    pill.hidden = false;
    pill.textContent = activeSprint;
    pill.title = "Files in scope for this sprint are highlighted";
  }

  async function loadJson(name) {
    const r = await fetch(VISION_BASE + name + "?t=" + Date.now());
    if (!r.ok) throw new Error(name + " HTTP " + r.status);
    return r.json();
  }

  function buildTree(nodes) {
    const root = {};
    nodes.forEach((n) => {
      const parts = n.path.split("/");
      let cur = root;
      parts.forEach((p, i) => {
        if (!cur[p]) cur[p] = i === parts.length - 1 ? { _node: n } : {};
        cur = cur[p];
      });
    });
    return root;
  }

  function renderTree(obj, container, depth) {
    Object.keys(obj)
      .sort()
      .forEach((key) => {
        const val = obj[key];
        if (val._node) {
          const n = val._node;
          const div = document.createElement("div");
          div.className = "tree-file";
          if (nodeInActiveSprint(n)) div.classList.add("sprint-scope");
          div.dataset.id = n.id;
          div.dataset.path = n.path;
          div.innerHTML =
            '<span class="ext-dot ' +
            extClass(n.path) +
            '"></span><span>' +
            escapeHtml(n.label) +
            "</span>";
          div.addEventListener("click", () => selectNode(n));
          container.appendChild(div);
          return;
        }
        const det = document.createElement("details");
        det.className = "tree-folder";
        det.open = depth < 2;
        const sum = document.createElement("summary");
        sum.textContent = key;
        det.appendChild(sum);
        const inner = document.createElement("div");
        renderTree(val, inner, depth + 1);
        det.appendChild(inner);
        container.appendChild(det);
      });
  }

  function escapeHtml(s) {
    return s
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function renderLayers(m) {
    const stack = document.getElementById("layer-stack");
    stack.innerHTML = "";
    (m.layers || [])
      .slice()
      .reverse()
      .forEach((layer) => {
        const el = document.createElement("div");
        el.className = "layer-plane";
        el.dataset.layer = layer.id;
        el.textContent = layer.id + " · " + layer.name;
        stack.appendChild(el);
      });
  }

  function highlightLayer(layerId) {
    document.querySelectorAll(".layer-plane").forEach((el) => {
      el.classList.toggle("highlight", el.dataset.layer === layerId);
    });
  }

  function layoutNodes() {
    nodePositions.clear();
    const byLayer = {};
    manifest.nodes.forEach((n) => {
      if (!byLayer[n.layer]) byLayer[n.layer] = [];
      byLayer[n.layer].push(n);
    });
    Object.keys(byLayer).forEach((layer) => {
      const list = byLayer[layer];
      const y = LAYER_Y[layer] || 200;
      const count = list.length;
      const span = Math.min(520, Math.max(120, count * 100));
      const x0 = 450 - span / 2;
      list.forEach((n, i) => {
        const x =
          count === 1 ? 450 : x0 + (i / Math.max(1, count - 1)) * span;
        nodePositions.set(n.id, { x, y, layer });
      });
    });
  }

  function planePath(cx, cy, w, h) {
    const hw = w / 2;
    const hh = h / 2;
    return (
      "M" +
      cx +
      "," +
      (cy + hh) +
      " L" +
      (cx + hw) +
      "," +
      cy +
      " L" +
      cx +
      "," +
      (cy - hh) +
      " L" +
      (cx - hw) +
      "," +
      cy +
      " Z"
    );
  }

  function renderMap() {
    layoutNodes();
    const svg = document.getElementById("map-svg");
    const ns = "http://www.w3.org/2000/svg";

    while (svg.firstChild) svg.removeChild(svg.firstChild);

    const defs = document.createElementNS(ns, "defs");
    defs.innerHTML = `
      <linearGradient id="gradL0" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#2d4a6f" stop-opacity="0.5"/>
        <stop offset="100%" stop-color="#1a2840" stop-opacity="0.25"/>
      </linearGradient>
      <linearGradient id="gradL1" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#3d5a40" stop-opacity="0.45"/>
        <stop offset="100%" stop-color="#243828" stop-opacity="0.2"/>
      </linearGradient>
      <linearGradient id="gradL2" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#5a4a2d" stop-opacity="0.4"/>
        <stop offset="100%" stop-color="#302818" stop-opacity="0.18"/>
      </linearGradient>
      <linearGradient id="gradL3" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#5a2d4a" stop-opacity="0.4"/>
        <stop offset="100%" stop-color="#301820" stop-opacity="0.18"/>
      </linearGradient>
      <filter id="nodeGlow" x="-50%" y="-50%" width="200%" height="200%">
        <feGaussianBlur stdDeviation="2" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>
      <filter id="nodeGlowStrong" x="-80%" y="-80%" width="260%" height="260%">
        <feGaussianBlur stdDeviation="4" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>
      <marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">
        <path d="M0,0 L7,3 L0,6 Z" fill="#a78bfa"/>
      </marker>
    `;
    svg.appendChild(defs);

    const planes = document.createElementNS(ns, "g");
    planes.setAttribute("class", "planes");
    [
      { layer: "L0", cx: 450, w: 520, h: 56 },
      { layer: "L1", cx: 450, w: 480, h: 50 },
      { layer: "L2", cx: 450, w: 500, h: 50 },
      { layer: "L3", cx: 450, w: 540, h: 52 },
    ].forEach((p) => {
      const path = document.createElementNS(ns, "path");
      path.setAttribute("d", planePath(p.cx, LAYER_Y[p.layer], p.w, p.h));
      path.setAttribute("class", "plane plane-" + p.layer);
      planes.appendChild(path);
      const label = document.createElementNS(ns, "text");
      label.setAttribute("x", 28);
      label.setAttribute("y", LAYER_Y[p.layer] + 4);
      label.setAttribute("fill", "#6a7a8a");
      label.setAttribute("font-size", "9");
      label.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
      label.textContent =
        p.layer +
        " " +
        (manifest.layers.find((l) => l.id === p.layer) || {}).name;
      planes.appendChild(label);
    });
    svg.appendChild(planes);

    const edgesG = document.createElementNS(ns, "g");
    edgesG.setAttribute("class", "edges");
    (manifest.edges || []).forEach((e) => {
      const a = nodePositions.get(e.from);
      const b = nodePositions.get(e.to);
      if (!a || !b) return;
      const path = document.createElementNS(ns, "path");
      const midY = (a.y + b.y) / 2;
      const d =
        "M" +
        a.x +
        "," +
        a.y +
        " C" +
        a.x +
        "," +
        midY +
        " " +
        b.x +
        "," +
        midY +
        " " +
        b.x +
        "," +
        b.y;
      path.setAttribute("d", d);
      path.setAttribute("class", "edge");
      path.dataset.from = e.from;
      path.dataset.to = e.to;
      edgesG.appendChild(path);
    });
    svg.appendChild(edgesG);

    const nodesG = document.createElementNS(ns, "g");
    nodesG.setAttribute("class", "nodes");
    manifest.nodes.forEach((n) => {
      const pos = nodePositions.get(n.id);
      if (!pos) return;
      const g = document.createElementNS(ns, "g");
      g.setAttribute("class", "node");
      g.dataset.id = n.id;
      const r = n.id === "galaxy_grid" ? 14 : 10;
      const circle = document.createElementNS(ns, "circle");
      circle.setAttribute("cx", pos.x);
      circle.setAttribute("cy", pos.y);
      circle.setAttribute("r", r);
      circle.setAttribute("fill", LAYER_COLORS[n.layer] || "#555");
      const text = document.createElementNS(ns, "text");
      text.setAttribute("x", pos.x);
      text.setAttribute("y", pos.y + r + 12);
      const short =
        n.label.length > 22 ? n.label.slice(0, 20) + "…" : n.label;
      text.textContent = short;
      g.appendChild(circle);
      g.appendChild(text);
      g.addEventListener("click", (ev) => {
        ev.stopPropagation();
        selectNode(n);
      });
      nodesG.appendChild(g);
    });
    svg.appendChild(nodesG);

    svg.addEventListener("click", () => {});
    updateMapSelection();
  }

  function updateMapSelection() {
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const id = el.dataset.id;
      const isSel = id === selectedId;
      const isLinked = isNodeLinked(id);
      el.classList.toggle("selected", isSel);
      el.classList.toggle("dim", selectedId && !isSel && !isLinked);
      const c = el.querySelector("circle");
      if (c && isSel) c.setAttribute("r", id === "galaxy_grid" ? 18 : 14);
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = el.dataset.from;
      const to = el.dataset.to;
      const hi =
        selectedId && (from === selectedId || to === selectedId);
      el.classList.toggle("highlight", hi);
    });
  }

  function isNodeLinked(id) {
    if (!selectedId || id === selectedId) return id === selectedId;
    return (manifest.edges || []).some(
      (e) =>
        (e.from === selectedId && e.to === id) ||
        (e.to === selectedId && e.from === id)
    );
  }

  function relatedEdges(nodeId) {
    const out = [];
    (manifest.edges || []).forEach((e) => {
      if (e.from === nodeId) {
        const t = nodeById(e.to);
        if (t) out.push({ kind: e.kind, dir: "out", node: t });
      }
      if (e.to === nodeId) {
        const t = nodeById(e.from);
        if (t) out.push({ kind: e.kind, dir: "in", node: t });
      }
    });
    return out;
  }

  function renderLinkGraph(node) {
    const svg = document.getElementById("link-graph");
    const ns = "http://www.w3.org/2000/svg";
    while (svg.firstChild) svg.removeChild(svg.firstChild);

    const w = svg.clientWidth || 220;
    const h = svg.clientHeight || 180;
    svg.setAttribute("viewBox", "0 0 " + w + " " + h);

    const cx = w / 2;
    const cy = h / 2;
    const links = relatedEdges(node.id);
    const others = links.map((l) => l.node);
    const r = Math.min(w, h) * 0.32;

    links.forEach((l, i) => {
      const angle = (i / Math.max(1, links.length)) * Math.PI * 2 - Math.PI / 2;
      const x2 = cx + Math.cos(angle) * r;
      const y2 = cy + Math.sin(angle) * r;
      const line = document.createElementNS(ns, "line");
      line.setAttribute("x1", cx);
      line.setAttribute("y1", cy);
      line.setAttribute("x2", x2);
      line.setAttribute("y2", y2);
      line.setAttribute("stroke", "rgba(167,139,250,0.5)");
      line.setAttribute("stroke-width", "1.5");
      svg.appendChild(line);
      const dot = document.createElementNS(ns, "circle");
      dot.setAttribute("cx", x2);
      dot.setAttribute("cy", y2);
      dot.setAttribute("r", 8);
      dot.setAttribute("fill", LAYER_COLORS[l.node.layer] || "#666");
      dot.setAttribute("stroke", "#fff");
      dot.setAttribute("stroke-width", "1");
      dot.style.cursor = "pointer";
      dot.addEventListener("click", () => selectNode(l.node));
      svg.appendChild(dot);
      const t = document.createElementNS(ns, "text");
      t.setAttribute("x", x2);
      t.setAttribute("y", y2 + 18);
      t.setAttribute("text-anchor", "middle");
      t.setAttribute("fill", "#9aa0a6");
      t.setAttribute("font-size", "8");
      t.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
      const lbl =
        l.node.label.length > 14
          ? l.node.label.slice(0, 12) + "…"
          : l.node.label;
      t.textContent = lbl;
      svg.appendChild(t);
    });

    const center = document.createElementNS(ns, "circle");
    center.setAttribute("cx", cx);
    center.setAttribute("cy", cy);
    center.setAttribute("r", 16);
    center.setAttribute("fill", LAYER_COLORS[node.layer] || "#3d6a9e");
    center.setAttribute("stroke", "#a78bfa");
    center.setAttribute("stroke-width", "2.5");
    svg.appendChild(center);
    const label = document.createElementNS(ns, "text");
    label.setAttribute("x", cx);
    label.setAttribute("y", cy + 4);
    label.setAttribute("text-anchor", "middle");
    label.setAttribute("fill", "#fff");
    label.setAttribute("font-size", "9");
    label.setAttribute("font-weight", "600");
    label.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
    label.textContent =
      node.label.length > 16 ? node.label.slice(0, 14) + "…" : node.label;
    svg.appendChild(label);

    if (!others.length) {
      const empty = document.createElementNS(ns, "text");
      empty.setAttribute("x", cx);
      empty.setAttribute("y", h - 12);
      empty.setAttribute("text-anchor", "middle");
      empty.setAttribute("fill", "#6a7a8a");
      empty.setAttribute("font-size", "9");
      empty.textContent = "no edges in manifest";
      svg.appendChild(empty);
    }
  }

  function renderLinksList(node) {
    const ul = document.getElementById("links-list");
    ul.innerHTML = "";
    const links = relatedEdges(node.id);
    if (!links.length) {
      ul.innerHTML =
        "<li style='cursor:default;color:var(--muted)'>no manifest edges</li>";
      return;
    }
    links.forEach((l) => {
      const li = document.createElement("li");
      li.innerHTML =
        '<span class="kind">' +
        l.dir +
        " · " +
        l.kind +
        "</span><br/>" +
        escapeHtml(l.node.label);
      li.addEventListener("click", () => selectNode(l.node));
      ul.appendChild(li);
    });
  }

  function renderSprintChips(node) {
    const box = document.getElementById("sprint-chips");
    box.innerHTML = "";
    if (!node.sprints || !node.sprints.length) return;
    node.sprints.slice(0, 12).forEach((s) => {
      const span = document.createElement("span");
      span.className = "sprint-chip";
      span.textContent = s;
      box.appendChild(span);
    });
    if (node.sprints.length > 12) {
      const more = document.createElement("span");
      more.className = "sprint-chip";
      more.textContent = "+" + (node.sprints.length - 12);
      box.appendChild(more);
    }
  }

  function formatPreview(text) {
    return text
      .split("\n")
      .map((line) => {
        if (/^### /.test(line))
          return '<span class="md-h2">' + escapeHtml(line) + "</span>";
        if (/^## /.test(line))
          return '<span class="md-h">' + escapeHtml(line) + "</span>";
        if (/^# /.test(line))
          return '<span class="md-h">' + escapeHtml(line) + "</span>";
        return escapeHtml(line);
      })
      .join("\n");
  }

  async function openDoc(node) {
    const hint = document.getElementById("doc-hint");
    const pre = document.getElementById("doc-preview");
    hint.textContent = node.path;
    pre.textContent = "loading…";
    try {
      const r = await fetch(repoUrl(node.path));
      if (!r.ok) throw new Error("HTTP " + r.status);
      const text = await r.text();
      const lines = text.split("\n");
      const slice =
        lines.length > 120
          ? lines.slice(0, 120).join("\n") +
            "\n\n… (" +
            lines.length +
            " lines)"
          : text;
      pre.innerHTML = formatPreview(slice);
    } catch (e) {
      pre.textContent = "Error: " + e.message;
    }
  }

  function setPanelFullscreen(panel, on) {
    if (!panel) return;
    const isPreview = panel.classList.contains("preview-panel");
    if (on) {
      document.querySelectorAll(".panel.panel-fullscreen").forEach((p) => {
        p.classList.remove("panel-fullscreen");
      });
      document.querySelectorAll(".btn-panel-fs").forEach((b) => {
        b.textContent = "⛶";
        b.title = "Fullscreen (Esc)";
      });
      panel.classList.add("panel-fullscreen");
      const btn = panel.querySelector(".btn-panel-fs");
      if (btn) {
        btn.textContent = "⤢";
        btn.title = "Exit fullscreen (Esc)";
      }
      document.body.classList.add("panel-fs-active");
      document.body.classList.toggle("panel-fs-preview", isPreview);
      fullscreenPanel = panel;
    } else {
      panel.classList.remove("panel-fullscreen");
      const btn = panel.querySelector(".btn-panel-fs");
      if (btn) {
        btn.textContent = "⛶";
        btn.title = "Fullscreen (Esc)";
      }
      document.body.classList.remove("panel-fs-active", "panel-fs-preview");
      fullscreenPanel = null;
    }
    if (panel.querySelector("#link-graph") && selectedId) {
      const n = nodeById(selectedId);
      if (n) renderLinkGraph(n);
    }
    window.dispatchEvent(new Event("resize"));
  }

  function exitPanelFullscreen() {
    if (fullscreenPanel) setPanelFullscreen(fullscreenPanel, false);
  }

  function initPanelFullscreen() {
    document.querySelectorAll(".panel .btn-panel-fs").forEach((btn) => {
      btn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        const panel = btn.closest(".panel");
        if (!panel) return;
        const on = !panel.classList.contains("panel-fullscreen");
        if (!on) exitPanelFullscreen();
        else setPanelFullscreen(panel, true);
      });
    });
    document.addEventListener("keydown", (ev) => {
      if (ev.key === "Escape") exitPanelFullscreen();
    });
  }

  function selectNode(node) {
    selectedId = node.id;
    document.querySelectorAll(".tree-file").forEach((el) => {
      el.classList.toggle("active", el.dataset.id === node.id);
    });
    highlightLayer(node.layer);
    renderSprintChips(node);
    renderLinksList(node);
    renderLinkGraph(node);
    updateMapSelection();
    openDoc(node);
  }

  function initStarfield() {
    const canvas = document.getElementById("starfield");
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    let w, h, stars;

    function resize() {
      w = canvas.width = window.innerWidth;
      h = canvas.height = window.innerHeight;
      stars = Array.from({ length: 120 }, () => ({
        x: Math.random() * w,
        y: Math.random() * h,
        r: Math.random() * 1.2 + 0.2,
        a: Math.random(),
        sp: Math.random() * 0.02 + 0.005,
      }));
    }

    function draw() {
      ctx.clearRect(0, 0, w, h);
      stars.forEach((s) => {
        s.a += s.sp;
        if (s.a > 1) s.a = 0;
        ctx.beginPath();
        ctx.arc(s.x, s.y, s.r, 0, Math.PI * 2);
        ctx.fillStyle = "rgba(200, 220, 255, " + (0.15 + s.a * 0.5) + ")";
        ctx.fill();
      });
      requestAnimationFrame(draw);
    }

    resize();
    window.addEventListener("resize", resize);
    draw();
  }

  function showAutoToast(msg) {
    let el = document.getElementById("auto-toast");
    if (!el) {
      el = document.createElement("div");
      el.id = "auto-toast";
      el.className = "auto-toast";
      el.setAttribute("role", "status");
      document.body.appendChild(el);
    }
    el.textContent = msg;
    el.classList.add("show");
    clearTimeout(showAutoToast._t);
    showAutoToast._t = setTimeout(() => el.classList.remove("show"), 2200);
  }

  async function reloadAll(keepSelection) {
    const prevId = keepSelection ? selectedId : null;
    const fsPanelId = fullscreenPanel && fullscreenPanel.dataset.panel;
    manifest = await loadJson("manifest.json");
    try {
      extensions = await loadJson("extensions.json");
    } catch (_) {
      extensions = null;
    }
    activeSprint = resolveActiveSprint(manifest, extensions);
    sprintPathSet = buildSprintPathSet(extensions, activeSprint);
    updateSidebarSprintPill();

    document.getElementById("meta-rev").innerHTML =
      "rev <strong>" +
      manifest.revision +
      "</strong> · " +
      manifest.last_sprint_closed +
      ' <span class="sprint-pill">→ ' +
      manifest.next_sprint +
      "</span>";

    renderLayers(manifest);
    renderMap();

    const tree = document.getElementById("file-tree");
    tree.innerHTML = "";
    renderTree(buildTree(manifest.nodes), tree, 0);

    const target =
      (prevId && nodeById(prevId)) ||
      manifest.nodes.find((n) => n.id === "galaxy_grid") ||
      manifest.nodes[0];
    if (target) selectNode(target);

    if (fsPanelId) {
      const panel = document.querySelector('.panel[data-panel="' + fsPanelId + '"]');
      if (panel) setPanelFullscreen(panel, true);
    }
  }

  async function pollWatch() {
    if (!autoReloadEnabled || reloadInFlight) return;
    try {
      const r = await fetch(VISION_BASE + "__watch?t=" + Date.now());
      if (!r.ok) return;
      const w = await r.json();
      if (!watchState) {
        watchState = w;
        return;
      }
      if (w.token === watchState.token) return;

      const bundleChanged = w.bundle !== watchState.bundle;
      const prev = watchState;
      watchState = w;
      reloadInFlight = true;

      if (bundleChanged) {
        showAutoToast("Auto-reload: UI assets changed…");
        location.reload();
        return;
      }

      showAutoToast("Auto-reload: manifest / data");
      await reloadAll(true);
      reloadInFlight = false;
    } catch (_) {
      reloadInFlight = false;
    }
  }

  function startAutoReload() {
    stopAutoReload();
    if (!autoReloadEnabled) return;
    watchState = null;
    pollWatch();
    watchTimer = setInterval(pollWatch, WATCH_INTERVAL_MS);
  }

  function stopAutoReload() {
    if (watchTimer) {
      clearInterval(watchTimer);
      watchTimer = null;
    }
  }

  function toggleAutoReload() {
    autoReloadEnabled = !autoReloadEnabled;
    const btn = document.getElementById("btn-auto");
    btn.classList.toggle("on", autoReloadEnabled);
    btn.title = autoReloadEnabled
      ? "Auto-reload ON (" + WATCH_INTERVAL_MS / 1000 + "s)"
      : "Auto-reload OFF";
    if (autoReloadEnabled) startAutoReload();
    else stopAutoReload();
  }

  function toggleSidebar() {
    document.body.classList.toggle("sidebar-collapsed");
  }

  document.getElementById("btn-sidebar").addEventListener("click", toggleSidebar);
  document.getElementById("btn-sidebar2").addEventListener("click", toggleSidebar);
  document.getElementById("btn-reload").addEventListener("click", () => reloadAll(true));
  document.getElementById("btn-auto").addEventListener("click", toggleAutoReload);

  initStarfield();
  initPanelFullscreen();
  reloadAll(false)
    .then(() => startAutoReload())
    .catch((err) => {
      document.getElementById("meta-rev").innerHTML =
        '<span class="error">' +
        escapeHtml(err.message) +
        " — run .\\bin\\open-docs-vision.ps1</span>";
    });
})();
