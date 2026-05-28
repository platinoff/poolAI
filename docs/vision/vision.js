/* PoolAI docs vision — interactive manifest graph */
(function () {
  "use strict";

  const VISION_BASE = "/docs/vision/";
  const MAP_W = 900;
  const MAP_H = 520;
  const MAP_MIN_SCALE = 0.35;
  const MAP_MAX_SCALE = 4;
  /** Trackpad wheel: half of original fixed ±12% → ~6% max per event */
  const MAP_WHEEL_SENSITIVITY = 0.00145;
  const MAP_WHEEL_STEP_MIN = 0.943;
  const MAP_WHEEL_STEP_MAX = 1.06;

  let LAYER_Y = {};
  const LAYER_COLORS = {
    L0: "#3d6a9e",
    L1: "#3d6a4a",
    L2: "#8a7040",
    L3: "#8a4068",
    L4: "#6a5088",
    L5: "#4a6880",
  };

  let mapView = { tx: 0, ty: 0, scale: 1 };
  let mapNavBound = false;

  const CLUSTER_COLLAPSE_MIN = 5;
  const MAP_PREFS_KEY = "poolai-vision-map-prefs";
  let autoCollapseDense = true;
  let mapSprintFocus = false;
  let expandedClusters = new Set();
  let clusterLabelPositions = [];

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
    if (e === "toml") return "ext-toml";
    return "ext-other";
  }

  function syncLayerGeometry(m) {
    const ids = (m.layers || []).map((l) => l.id);
    const top = 58;
    const bottom = MAP_H - 48;
    if (!ids.length) return;
    ids.forEach((id, i) => {
      LAYER_Y[id] =
        ids.length === 1
          ? (top + bottom) / 2
          : top + (i / (ids.length - 1)) * (bottom - top);
    });
  }

  function loadMapPrefs() {
    try {
      const raw = localStorage.getItem(MAP_PREFS_KEY);
      if (!raw) return;
      const p = JSON.parse(raw);
      if (typeof p.autoCollapseDense === "boolean") {
        autoCollapseDense = p.autoCollapseDense;
      }
      if (typeof p.mapSprintFocus === "boolean") {
        mapSprintFocus = p.mapSprintFocus;
      }
      expandedClusters = new Set(p.expandedClusters || []);
    } catch (_) {
      /* ignore */
    }
  }

  function saveMapPrefs() {
    try {
      localStorage.setItem(
        MAP_PREFS_KEY,
        JSON.stringify({
          autoCollapseDense,
          mapSprintFocus,
          expandedClusters: Array.from(expandedClusters),
        })
      );
    } catch (_) {
      /* ignore */
    }
  }

  function clusterStoreId(layer, key) {
    return layer + "::" + key;
  }

  function isClusterCollapsed(layer, key, count) {
    if (count < CLUSTER_COLLAPSE_MIN) return false;
    if (!autoCollapseDense) return false;
    return !expandedClusters.has(clusterStoreId(layer, key));
  }

  function toggleCluster(layer, key) {
    const id = clusterStoreId(layer, key);
    if (expandedClusters.has(id)) expandedClusters.delete(id);
    else expandedClusters.add(id);
    saveMapPrefs();
    renderMap();
  }

  function clusterDisplayName(key) {
    if (!key || key === "other") return "other";
    const parts = key.split("/");
    return parts.length > 2 ? parts.slice(-2).join("/") : key;
  }

  function mapPosForNode(nodeId) {
    const p = nodePositions.get(nodeId);
    if (!p) return null;
    if (p.collapsedHidden && p.hubId) return nodePositions.get(p.hubId);
    return p;
  }

  function mapNodeDimmed(node) {
    return (
      mapSprintFocus && activeSprint && !nodeInActiveSprint(node)
    );
  }

  function folderCluster(path) {
    if (!path) return "other";
    const parts = path.split("/").filter(Boolean);
    if (parts[0] === "docs" && parts.length >= 2) {
      return parts.slice(0, 2).join("/");
    }
    if (parts[0] === "src" && parts.length >= 2) {
      return parts[0] + "/" + parts[1];
    }
    if (parts[0] === "crates" && parts.length >= 2) {
      return parts.slice(0, 2).join("/");
    }
    if (parts[0] === ".cargo") return ".cargo";
    return parts[0] || "root";
  }

  function pathKind(path) {
    if (!path) return "other";
    if (path.startsWith("docs/")) return "docs";
    if (
      path.startsWith("src/") ||
      path.startsWith("crates/") ||
      /\.rs$/i.test(path)
    ) {
      return "code";
    }
    if (/\.toml$/i.test(path) || path.startsWith(".cargo/")) return "toml";
    return "other";
  }

  function commonPathPrefix(a, b) {
    if (!a || !b) return "";
    const pa = a.split("/");
    const pb = b.split("/");
    const out = [];
    for (let i = 0; i < pa.length && i < pb.length; i++) {
      if (pa[i] !== pb[i]) break;
      out.push(pa[i]);
    }
    return out.join("/");
  }

  function folderHubX(key) {
    const s = key || "root";
    let h = 0;
    for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) | 0;
    return 120 + (Math.abs(h) % 660);
  }

  function folderLaneY(kind, ay, by) {
    if (kind === "docs") return LAYER_Y.L1 || (ay + by) / 2;
    if (kind === "code") return LAYER_Y.L3 || (ay + by) / 2;
    if (kind === "toml") return LAYER_Y.L5 || (ay + by) / 2;
    return (ay + by) / 2;
  }

  function edgeRouteKind(nodeA, nodeB) {
    const ka = pathKind(nodeA && nodeA.path);
    const kb = pathKind(nodeB && nodeB.path);
    if (ka === kb && ka !== "other") return ka;
    if (ka === "docs" || kb === "docs") {
      if (ka === "code" || kb === "code") return "mixed";
      if (ka === "docs" || kb === "docs") return "docs";
    }
    if (ka === "code" || kb === "code") return "code";
    if (ka === "toml" || kb === "toml") return "toml";
    return "mixed";
  }

  function buildEdgePath(a, b, nodeA, nodeB) {
    const prefix = commonPathPrefix(nodeA.path, nodeB.path);
    const kind = edgeRouteKind(nodeA, nodeB);
    const hubX = folderHubX(prefix || nodeA.path.split("/")[0]);
    const laneY = folderLaneY(kind, a.y, b.y);
    const y1 = a.y < b.y ? Math.min(a.y, laneY) : Math.max(a.y, laneY);
    const y2 = b.y < a.y ? Math.min(b.y, laneY) : Math.max(b.y, laneY);
    const midY = (y1 + y2) / 2;
    return (
      "M" +
      a.x +
      "," +
      a.y +
      " L" +
      a.x +
      "," +
      midY +
      " L" +
      hubX +
      "," +
      midY +
      " L" +
      hubX +
      "," +
      b.y +
      " L" +
      b.x +
      "," +
      b.y
    );
  }

  function applyMapTransform() {
    const world = document.getElementById("map-world");
    if (!world) return;
    world.setAttribute(
      "transform",
      "translate(" +
        mapView.tx +
        "," +
        mapView.ty +
        ") scale(" +
        mapView.scale +
        ")"
    );
  }

  function resetMapView() {
    mapView = { tx: 0, ty: 0, scale: 1 };
    applyMapTransform();
  }

  function normalizeWheelDelta(deltaY, deltaMode) {
    if (deltaMode === 1) return deltaY * 18;
    if (deltaMode === 2) return deltaY * 280;
    return deltaY;
  }

  function wheelZoomFactor(deltaY, deltaMode) {
    const dy = normalizeWheelDelta(deltaY, deltaMode);
    let factor = Math.exp(-dy * MAP_WHEEL_SENSITIVITY);
    return Math.min(
      MAP_WHEEL_STEP_MAX,
      Math.max(MAP_WHEEL_STEP_MIN, factor)
    );
  }

  function zoomMapAt(sx, sy, factor) {
    const ns = Math.min(
      MAP_MAX_SCALE,
      Math.max(MAP_MIN_SCALE, mapView.scale * factor)
    );
    const ratio = ns / mapView.scale;
    mapView.tx = sx - ratio * (sx - mapView.tx);
    mapView.ty = sy - ratio * (sy - mapView.ty);
    mapView.scale = ns;
    applyMapTransform();
  }

  function focusMapNode(node) {
    const pos = nodePositions.get(node.id);
    if (!pos) return;
    mapView.scale = 1.65;
    mapView.tx = MAP_W / 2 - pos.x * mapView.scale;
    mapView.ty = MAP_H / 2 - pos.y * mapView.scale;
    applyMapTransform();
  }

  function bindMapNavigation(svg) {
    const wrap = svg.closest(".map-wrap");
    if (!wrap || mapNavBound) return;
    mapNavBound = true;

    wrap.addEventListener(
      "wheel",
      (ev) => {
        if (!ev.target.closest("#map-svg")) return;
        ev.preventDefault();
        const rect = svg.getBoundingClientRect();
        const sx = ((ev.clientX - rect.left) / rect.width) * MAP_W;
        const sy = ((ev.clientY - rect.top) / rect.height) * MAP_H;
        zoomMapAt(sx, sy, wheelZoomFactor(ev.deltaY, ev.deltaMode));
      },
      { passive: false }
    );

    let dragging = false;
    let lastX = 0;
    let lastY = 0;

    wrap.addEventListener("mousedown", (ev) => {
      if (ev.button !== 0 || ev.target.closest(".node")) return;
      dragging = true;
      lastX = ev.clientX;
      lastY = ev.clientY;
      wrap.classList.add("map-panning");
    });
    window.addEventListener("mousemove", (ev) => {
      if (!dragging) return;
      const rect = svg.getBoundingClientRect();
      const scaleX = MAP_W / rect.width;
      const scaleY = MAP_H / rect.height;
      mapView.tx += (ev.clientX - lastX) * scaleX;
      mapView.ty += (ev.clientY - lastY) * scaleY;
      lastX = ev.clientX;
      lastY = ev.clientY;
      applyMapTransform();
    });
    window.addEventListener("mouseup", () => {
      dragging = false;
      wrap.classList.remove("map-panning");
    });

    const zi = document.getElementById("map-zoom-in");
    const zo = document.getElementById("map-zoom-out");
    const zr = document.getElementById("map-zoom-reset");
    if (zi) {
      zi.addEventListener("click", (ev) => {
        ev.stopPropagation();
        zoomMapAt(MAP_W / 2, MAP_H / 2, 1.16);
      });
    }
    if (zo) {
      zo.addEventListener("click", (ev) => {
        ev.stopPropagation();
        zoomMapAt(MAP_W / 2, MAP_H / 2, 1 / 1.16);
      });
    }
    if (zr) {
      zr.addEventListener("click", (ev) => {
        ev.stopPropagation();
        resetMapView();
      });
    }
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

  function layoutLayerRow(list, baseY, layer) {
    const count = list.length;
    const span = Math.min(MAP_W - 64, Math.max(140, count * 96));
    const x0 = MAP_W / 2 - span / 2;
    list.forEach((n, i) => {
      const x =
        count === 1 ? MAP_W / 2 : x0 + (i / Math.max(1, count - 1)) * span;
      nodePositions.set(n.id, { x, y: baseY, layer });
    });
  }

  function layoutLayerClusters(list, baseY, layer) {
    const clusters = new Map();
    list.forEach((n) => {
      const key = folderCluster(n.path);
      if (!clusters.has(key)) clusters.set(key, []);
      clusters.get(key).push(n);
    });

    const clusterKeys = Array.from(clusters.keys()).sort();
    const maxCols = layer === "L3" ? 3 : 4;
    const cellW = layer === "L3" ? 78 : 68;
    const cellH = 34;
    const clusterGap = 28;
    const clusterPad = 12;
    const collapsedW = 64;

    const clusterLayouts = clusterKeys.map((key) => {
      const nodes = clusters
        .get(key)
        .slice()
        .sort((a, b) => a.label.localeCompare(b.label));
      const collapsed = isClusterCollapsed(layer, key, nodes.length);
      const cols = Math.min(maxCols, nodes.length);
      const rows = Math.ceil(nodes.length / cols);
      const w = collapsed ? collapsedW : cols * cellW + clusterPad * 2;
      return { key, nodes, cols, rows, w, collapsed };
    });

    const totalW =
      clusterLayouts.reduce((s, c) => s + c.w, 0) +
      clusterGap * Math.max(0, clusterLayouts.length - 1);
    let xCursor = MAP_W / 2 - totalW / 2;
    clusterLabelPositions = [];

    clusterLayouts.forEach((cl) => {
      const cx = xCursor + cl.w / 2;
      if (cl.collapsed) {
        const hub = cl.nodes[0];
        const hubId = hub.id;
        cl.nodes.forEach((n, i) => {
          if (i === 0) {
            nodePositions.set(n.id, {
              x: cx,
              y: baseY,
              layer,
              cluster: cl.key,
              clusterHub: true,
              clusterCount: cl.nodes.length,
            });
          } else {
            nodePositions.set(n.id, {
              x: cx,
              y: baseY,
              layer,
              cluster: cl.key,
              collapsedHidden: true,
              hubId,
            });
          }
        });
        clusterLabelPositions.push({
          x: cx,
          y: baseY - 22,
          key: cl.key,
          layer,
          collapsed: true,
          count: cl.nodes.length,
        });
      } else {
        const topY = baseY - ((cl.rows - 1) / 2) * cellH;
        if (cl.nodes.length > 1) {
          clusterLabelPositions.push({
            x: cx,
            y: topY - 18,
            key: cl.key,
            layer,
            collapsed: false,
            count: cl.nodes.length,
          });
        }
        cl.nodes.forEach((n, i) => {
          const col = i % cl.cols;
          const row = Math.floor(i / cl.cols);
          const x = xCursor + clusterPad + col * cellW + cellW / 2;
          const y = baseY + (row - (cl.rows - 1) / 2) * cellH;
          nodePositions.set(n.id, {
            x,
            y,
            layer,
            cluster: cl.key,
          });
        });
      }
      xCursor += cl.w + clusterGap;
    });
  }

  function layoutNodes() {
    nodePositions.clear();
    clusterLabelPositions = [];
    const byLayer = {};
    manifest.nodes.forEach((n) => {
      if (!byLayer[n.layer]) byLayer[n.layer] = [];
      byLayer[n.layer].push(n);
    });
    Object.keys(byLayer).forEach((layer) => {
      const list = byLayer[layer];
      const baseY = LAYER_Y[layer] || 200;
      if (list.length <= 4) {
        layoutLayerRow(list, baseY, layer);
      } else {
        layoutLayerClusters(list, baseY, layer);
      }
    });
  }

  function layerPlaneBounds(layer) {
    let minY = Infinity;
    let maxY = -Infinity;
    let minX = Infinity;
    let maxX = -Infinity;
    manifest.nodes.forEach((n) => {
      if (n.layer !== layer) return;
      const p = mapPosForNode(n.id);
      if (!p || p.collapsedHidden) return;
      minY = Math.min(minY, p.y);
      maxY = Math.max(maxY, p.y);
      minX = Math.min(minX, p.x);
      maxX = Math.max(maxX, p.x);
    });
    if (minY === Infinity) {
      const y = LAYER_Y[layer] || MAP_H / 2;
      return { cx: MAP_W / 2, cy: y, w: 480, h: 44 };
    }
    const spreadY = maxY - minY;
    const spreadX = maxX - minX;
    return {
      cx: (minX + maxX) / 2,
      cy: (minY + maxY) / 2,
      w: Math.min(MAP_W - 40, Math.max(420, spreadX + 120)),
      h: Math.max(44, spreadY + 52),
    };
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
    syncLayerGeometry(manifest);
    layoutNodes();
    const svg = document.getElementById("map-svg");
    svg.setAttribute("viewBox", "0 0 " + MAP_W + " " + MAP_H);
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
      <linearGradient id="gradL4" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#4a3d5a" stop-opacity="0.42"/>
        <stop offset="100%" stop-color="#281830" stop-opacity="0.18"/>
      </linearGradient>
      <linearGradient id="gradL5" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#2d4a5a" stop-opacity="0.45"/>
        <stop offset="100%" stop-color="#182830" stop-opacity="0.2"/>
      </linearGradient>
      <filter id="nodeGlow" x="-50%" y="-50%" width="200%" height="200%">
        <feGaussianBlur stdDeviation="2" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>
      <filter id="nodeGlowStrong" x="-80%" y="-80%" width="260%" height="260%">
        <feGaussianBlur stdDeviation="4" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>
      <marker id="arrow-docs" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">
        <path d="M0,0 L7,3 L0,6 Z" fill="#90c490"/>
      </marker>
      <marker id="arrow-code" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">
        <path d="M0,0 L7,3 L0,6 Z" fill="#c49ab0"/>
      </marker>
      <marker id="arrow-toml" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">
        <path d="M0,0 L7,3 L0,6 Z" fill="#7eb8c4"/>
      </marker>
      <marker id="arrow-mixed" markerWidth="8" markerHeight="8" refX="7" refY="3" orient="auto">
        <path d="M0,0 L7,3 L0,6 Z" fill="#a78bfa"/>
      </marker>
    `;
    svg.appendChild(defs);

    const world = document.createElementNS(ns, "g");
    world.setAttribute("id", "map-world");

    const planes = document.createElementNS(ns, "g");
    planes.setAttribute("class", "planes");
    (manifest.layers || []).forEach((layer) => {
      const bounds = layerPlaneBounds(layer.id);
      const y = bounds.cy;
      const w = bounds.w;
      const h = bounds.h;
      const path = document.createElementNS(ns, "path");
      path.setAttribute("d", planePath(bounds.cx, y, w, h));
      path.setAttribute("class", "plane plane-" + layer.id);
      planes.appendChild(path);
      const label = document.createElementNS(ns, "text");
      label.setAttribute("x", 28);
      label.setAttribute("y", y + 4);
      label.setAttribute("fill", "#6a7a8a");
      label.setAttribute("font-size", "9");
      label.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
      label.textContent = layer.id + " " + layer.name;
      planes.appendChild(label);
    });
    world.appendChild(planes);

    const edgesG = document.createElementNS(ns, "g");
    edgesG.setAttribute("class", "edges");
    (manifest.edges || []).forEach((e) => {
      const nodeA = nodeById(e.from);
      const nodeB = nodeById(e.to);
      if (!nodeA || !nodeB) return;
      const a = mapPosForNode(e.from);
      const b = mapPosForNode(e.to);
      if (!a || !b) return;
      const routeKind = edgeRouteKind(nodeA, nodeB);
      const path = document.createElementNS(ns, "path");
      path.setAttribute("d", buildEdgePath(a, b, nodeA, nodeB));
      let edgeClass = "edge edge-" + routeKind;
      if (mapNodeDimmed(nodeA) && mapNodeDimmed(nodeB)) {
        edgeClass += " sprint-dim";
      }
      path.setAttribute("class", edgeClass);
      path.dataset.from = e.from;
      path.dataset.to = e.to;
      path.dataset.route = routeKind;
      edgesG.appendChild(path);
    });
    world.appendChild(edgesG);

    const clusterLabelsG = document.createElementNS(ns, "g");
    clusterLabelsG.setAttribute("class", "cluster-labels");
    clusterLabelPositions.forEach((cl) => {
      const t = document.createElementNS(ns, "text");
      t.setAttribute("x", cl.x);
      t.setAttribute("y", cl.y);
      t.setAttribute("class", "cluster-label" + (cl.collapsed ? " collapsed" : ""));
      t.textContent =
        clusterDisplayName(cl.key) +
        (cl.count > 1 ? " · " + cl.count : "");
      clusterLabelsG.appendChild(t);
    });
    world.appendChild(clusterLabelsG);

    const nodesG = document.createElementNS(ns, "g");
    nodesG.setAttribute("class", "nodes");
    manifest.nodes.forEach((n) => {
      const pos = nodePositions.get(n.id);
      if (!pos || pos.collapsedHidden) return;
      const g = document.createElementNS(ns, "g");
      g.setAttribute("class", "node");
      if (pos.clusterHub) g.classList.add("cluster-hub");
      if (mapNodeDimmed(n)) g.classList.add("sprint-dim");
      g.dataset.id = n.id;
      if (pos.cluster) g.dataset.cluster = pos.cluster;
      const r = pos.clusterHub ? 13 : n.id === "galaxy_grid" ? 14 : 10;
      const circle = document.createElementNS(ns, "circle");
      circle.setAttribute("cx", pos.x);
      circle.setAttribute("cy", pos.y);
      circle.setAttribute("r", r);
      circle.setAttribute("fill", LAYER_COLORS[n.layer] || "#555");
      const text = document.createElementNS(ns, "text");
      text.setAttribute("x", pos.x);
      text.setAttribute("y", pos.y + r + 12);
      let label;
      if (pos.clusterHub) {
        label = clusterDisplayName(pos.cluster) + " (" + pos.clusterCount + ")";
      } else {
        const maxLen = pos.cluster && n.layer === "L3" ? 16 : 22;
        label =
          n.label.length > maxLen
            ? n.label.slice(0, maxLen - 1) + "…"
            : n.label;
      }
      text.textContent = label;
      if (n.layer === "L3" && pos.cluster && !pos.clusterHub) {
        text.setAttribute("font-size", "9");
      }
      if (pos.clusterHub) {
        text.setAttribute("font-size", "9");
        text.setAttribute("font-weight", "700");
      }
      g.appendChild(circle);
      g.appendChild(text);
      if (pos.clusterHub) {
        g.setAttribute("title", "Click: expand folder · Shift+click: open file");
      }
      g.addEventListener("click", (ev) => {
        ev.stopPropagation();
        if (pos.clusterHub && !ev.shiftKey) {
          toggleCluster(n.layer, pos.cluster);
          return;
        }
        selectNode(n);
      });
      g.addEventListener("dblclick", (ev) => {
        ev.stopPropagation();
        if (pos.clusterHub) {
          expandedClusters.add(clusterStoreId(n.layer, pos.cluster));
          saveMapPrefs();
          renderMap();
        }
        focusMapNode(n);
        selectNode(n);
      });
      nodesG.appendChild(g);
    });
    world.appendChild(nodesG);
    svg.appendChild(world);

    applyMapTransform();
    bindMapNavigation(svg);
    updateMapSelection();
  }

  function updateMapSelection() {
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const id = el.dataset.id;
      const isSel = id === selectedId;
      const isLinked = isNodeLinked(id);
      el.classList.toggle("selected", isSel);
      el.classList.toggle(
        "dim",
        selectedId && !isSel && !isLinked && !el.classList.contains("sprint-dim")
      );
      const c = el.querySelector("circle");
      const hub = el.classList.contains("cluster-hub");
      if (c && isSel) {
        c.setAttribute("r", id === "galaxy_grid" ? 18 : hub ? 16 : 14);
      }
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = el.dataset.from;
      const to = el.dataset.to;
      const hi = selectedId && (from === selectedId || to === selectedId);
      el.classList.toggle("highlight", hi);
    });
  }

  function isNodeLinked(id) {
    if (!selectedId || id === selectedId) return id === selectedId;
    if (
      (manifest.edges || []).some(
        (e) =>
          (e.from === selectedId && e.to === id) ||
          (e.to === selectedId && e.from === id)
      )
    ) {
      return true;
    }
    const sel = nodeById(selectedId);
    const tgt = nodeById(id);
    if (!sel || !tgt || sel.layer !== tgt.layer) return false;
    return folderCluster(sel.path) === folderCluster(tgt.path);
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
        ctx.fillStyle =
          "rgba(200, 220, 255, " + (0.12 + s.a * 0.4) + ")";
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

    syncLayerGeometry(manifest);
    renderLayers(manifest);
    syncMapToolbar();
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

  function syncMapToolbar() {
    const btnSprint = document.getElementById("btn-map-sprint");
    const btnClusters = document.getElementById("btn-map-clusters");
    if (btnSprint) btnSprint.classList.toggle("on", mapSprintFocus);
    if (btnClusters) btnClusters.classList.toggle("on", autoCollapseDense);
  }

  function initMapToolbar() {
    syncMapToolbar();
    const btnSprint = document.getElementById("btn-map-sprint");
    const btnClusters = document.getElementById("btn-map-clusters");
    if (btnSprint) {
      btnSprint.addEventListener("click", () => {
        mapSprintFocus = !mapSprintFocus;
        saveMapPrefs();
        syncMapToolbar();
        renderMap();
      });
    }
    if (btnClusters) {
      btnClusters.addEventListener("click", () => {
        autoCollapseDense = !autoCollapseDense;
        if (!autoCollapseDense) {
          manifest.nodes.forEach((n) => {
            const key = folderCluster(n.path);
            expandedClusters.add(clusterStoreId(n.layer, key));
          });
        } else {
          expandedClusters = new Set();
        }
        saveMapPrefs();
        syncMapToolbar();
        renderMap();
      });
    }
  }

  document.getElementById("btn-sidebar").addEventListener("click", toggleSidebar);
  document.getElementById("btn-sidebar2").addEventListener("click", toggleSidebar);
  document.getElementById("btn-reload").addEventListener("click", () => reloadAll(true));
  document.getElementById("btn-auto").addEventListener("click", toggleAutoReload);

  initStarfield();
  initPanelFullscreen();
  loadMapPrefs();
  initMapToolbar();
  reloadAll(false)
    .then(() => startAutoReload())
    .catch((err) => {
      document.getElementById("meta-rev").innerHTML =
        '<span class="error">' +
        escapeHtml(err.message) +
        " — run .\\bin\\open-docs-vision.ps1</span>";
    });
})();
