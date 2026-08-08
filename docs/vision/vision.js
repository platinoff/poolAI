/* =====================================================================
 * DEACTIVATED — band 117 (2026-08-07).
 * Legacy vision UI superseded by Galaxy StarWalker Vision (GSV).
 * docs/vision/index.html is now a pointer page and no longer loads this
 * file. Assets kept as canon archive — do not remove.
 * See GSV/docs/LEGACY_PARITY.md and docs/gsv/GSV_MIGRATION.md.
 * ===================================================================== */
/* PoolAI docs vision — interactive manifest graph */
(function () {
  "use strict";

  const VISION_BASE = "/docs/vision/";
  const MAP_W = 900;
  const MAP_H = 580;
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
  const MAP_ORBIT_DEFAULT = { rotX: 48, rotY: -28, rotZ: -8 };
  const MAP_ORBIT_STEP_DEG = 2;
  /** Approx. browser key-repeat when holding WASD (keydown ~33 Hz). */
  const MAP_ORBIT_KEY_REPEAT_HZ = 33;
  /** Auto-orbit: ~30% of held WASD (~3× slower than prior 90% band, PH-S579). */
  const MAP_ORBIT_AUTO_SPEED_FACTOR = 0.3;
  const MAP_ORBIT_AUTO_DEG_PER_SEC =
    MAP_ORBIT_STEP_DEG * MAP_ORBIT_KEY_REPEAT_HZ * MAP_ORBIT_AUTO_SPEED_FACTOR;
  const MAP_FIT_PADDING = 36;
  const MAP_ORBIT_PAD_SENS = 0.08;
  const MAP_ORBIT_CLAMP_X = 72;
  const MAP_LAYER_Z_STEP = 52;
  const MAP_3D_FOCAL = 880;
  const MAP_PREFS_ORBIT_KEY = "mapOrbit";
  const MAP_PREFS_ORBIT_AUTO_KEY = "mapOrbitAuto";
  let mapOrbit = { rotX: MAP_ORBIT_DEFAULT.rotX, rotY: MAP_ORBIT_DEFAULT.rotY, rotZ: MAP_ORBIT_DEFAULT.rotZ };
  let mapOrbitBound = false;
  let mapOrbitAutoPlay = true;
  let mapOrbitAutoRaf = 0;
  let mapOrbitAutoLastTs = 0;
  let mapOrbitAutoTicking = false;
  let mapInitialFitDone = false;
  /** Projected screen coords after layer Z + orbit (PH-S556). */
  let map3DProjected = new Map();
  const mapViewStack = [];
  let mapNavBound = false;
  let mapHoverRaf = 0;
  let clickFocusTimer = null;

  const CLUSTER_COLLAPSE_MIN = 3;
  const GOLDEN_ANGLE = Math.PI * (3 - Math.sqrt(5));
  /** Leaf labels (auto-synced / dense map) appear above this zoom. */
  const LABEL_ZOOM_LEAF = 1.55;
  const LABEL_ZOOM_NORMAL = 1.15;
  /** Target on-screen label size when focusing a node (≈14px at 96dpi). */
  const MAP_FOCUS_LABEL_PX = 14;
  /** Below this scale: overview LOD (hub nodes/edges, hub-only labels). PH-S192 */
  const MAP_OVERVIEW_ZOOM = 0.98;
  const MAP_DENSE_NODE_THRESHOLD = 80;
  const CONSTELLATION_HUB_IDS = new Set([
    "galaxy_grid",
    "fm",
    "handoff",
    "digest",
    "docs_index",
    "cargo_toml",
    "next_session",
    "dev_readme",
  ]);
  const MAP_PREFS_KEY = "poolai-vision-map-prefs";
  let autoCollapseDense = true;
  let mapSprintFocus = false;
  /** Layer id (L0–L5) → highlighted on map; null = all on. */
  let enabledLayers = null;
  /** File ext bucket → highlighted; null = all on. */
  let enabledExts = null;
  const MAP_EXT_BUCKETS = [
    { id: "md", label: "md" },
    { id: "rs", label: "rs" },
    { id: "ts", label: "ts" },
    { id: "yaml", label: "yaml" },
    { id: "json", label: "json" },
    { id: "toml", label: "toml" },
    { id: "other", label: "·" },
  ];
  /** Single-layer solo focus on map (Shift+layer chip); independent of 3D stack. */
  let mapLayerFocus = null;
  /** 3D stack / legend tier focus — decoupled from map filter chips (PH-S188). */
  let stackLayerFocus = null;
  let expandedClusters = new Set();
  let clusterLabelPositions = [];

  const WATCH_INTERVAL_MS = 1500;
  const WATCH_INTERVAL_ECO_MS = 4000;
  const STAR_COUNT_FX = 42;
  const STAR_FRAME_MS = 56;
  const VISION_MODES = ["eco", "fx", "ms"];
  const MAP_PREFS_MODE_KEY = "visionMode";
  const MAP_PREFS_MODE_PIN_KEY = "visionModePinned";
  /** @deprecated legacy boolean — migrated on load */
  const MAP_PREFS_ECO_KEY = "visionEco";
  const MAP_PREFS_ECO_PIN_KEY = "visionEcoPinned";

  let manifest = null;
  let extensions = null;
  let activeSprint = null;
  let nextSprint = null;
  let activeQueueSprintId = null;
  let sprintPathSet = null;
  let selectedId = null;
  let mapNavNeighborIdx = -1;
  let mapNavNeighborKey = null;
  let edgeSelectTraceKey = null;
  let edgeSelectPartnerId = null;
  let fullscreenPanel = null;
  let nodePositions = new Map();
  let watchState = null;
  let watchTimer = null;
  let autoReloadEnabled = true;
  let reloadInFlight = false;
  /** Tri-mode: eco (low GPU) → fx (glow) → ms (1-hop hover trace). PH-S189 */
  let visionMode = "eco";
  /** User explicitly cycled mode — do not auto-switch on reload. */
  let visionModePinned = false;
  /** Hovered node id for Ms-mode 1-hop trace. */
  let hoverTraceId = null;
  let starfieldStop = null;
  let labelZoomRaf = null;
  let lastLabelZoomScale = 0;
  let nodeIndex = new Map();
  let degreeById = new Map();
  let layerNodeCounts = new Map();
  let adjacency = null;
  let constellationHubIdSet = new Set();
  let activeTreeFileEl = null;
  let collapsedPanels = new Set();
  const DIAGRAM_PANELS = ["layers", "queue", "map", "speeds", "rust", "links"];
  const ALL_PANELS = [...DIAGRAM_PANELS, "preview"];
  const SPEED_INDEX_URL = "/docs/vision/speed_index.json";
  const SPEED_INDEX_FALLBACK = "/docs/development/speed_index.json";
  const RUST_DIAG_URL = "/docs/vision/rust_diagnostics.json";
  const RUST_DIAG_FALLBACK = "/docs/development/rust_diagnostics.json";
  const panelAnchors = new Map();
  let openFilterDrop = null;
  let mapNodesBound = false;
  let minimapBound = false;
  /** Live git HEAD from __watch (falls back to manifest.git_head). */
  let headerGitHead = null;

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

  function nodeExtBucket(path) {
    const e = fileExt(path);
    if (e === "yaml" || e === "yml") return "yaml";
    if (e === "ts") return "ts";
    if (e === "md" || e === "rs" || e === "json" || e === "toml") return e;
    return "other";
  }

  function allLayerIds() {
    return (manifest && manifest.layers ? manifest.layers : []).map((l) => l.id);
  }

  function isLayerHighlighted(layerId) {
    if (mapLayerFocus) return layerId === mapLayerFocus;
    if (!enabledLayers) return true;
    return enabledLayers.has(layerId);
  }

  function isExtHighlighted(path) {
    if (!enabledExts) return true;
    return enabledExts.has(nodeExtBucket(path));
  }

  function hasActiveMapFilters() {
    return (
      !!mapLayerFocus ||
      enabledLayers !== null ||
      enabledExts !== null
    );
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
      if (
        typeof p[MAP_PREFS_MODE_KEY] === "string" &&
        VISION_MODES.includes(p[MAP_PREFS_MODE_KEY])
      ) {
        visionMode = p[MAP_PREFS_MODE_KEY];
      } else if (typeof p[MAP_PREFS_ECO_KEY] === "boolean") {
        visionMode = p[MAP_PREFS_ECO_KEY] ? "eco" : "fx";
      }
      if (typeof p[MAP_PREFS_MODE_PIN_KEY] === "boolean") {
        visionModePinned = p[MAP_PREFS_MODE_PIN_KEY];
      } else if (typeof p[MAP_PREFS_ECO_PIN_KEY] === "boolean") {
        visionModePinned = p[MAP_PREFS_ECO_PIN_KEY];
      }
      expandedClusters = new Set(p.expandedClusters || []);
      if (Array.isArray(p.collapsedPanels)) {
        collapsedPanels = new Set(p.collapsedPanels);
      }
      if (p[MAP_PREFS_ORBIT_KEY] && typeof p[MAP_PREFS_ORBIT_KEY] === "object") {
        const o = p[MAP_PREFS_ORBIT_KEY];
        if (typeof o.rotX === "number") mapOrbit.rotX = o.rotX;
        if (typeof o.rotY === "number") mapOrbit.rotY = o.rotY;
        if (typeof o.rotZ === "number") mapOrbit.rotZ = o.rotZ;
        clampMapOrbit();
      }
      if (typeof p[MAP_PREFS_ORBIT_AUTO_KEY] === "boolean") {
        mapOrbitAutoPlay = p[MAP_PREFS_ORBIT_AUTO_KEY];
      }
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
          collapsedPanels: Array.from(collapsedPanels),
          [MAP_PREFS_MODE_KEY]: visionMode,
          [MAP_PREFS_MODE_PIN_KEY]: visionModePinned,
          [MAP_PREFS_ECO_KEY]: visionMode === "eco",
          [MAP_PREFS_ECO_PIN_KEY]: visionModePinned,
          [MAP_PREFS_ORBIT_KEY]: mapOrbit,
          [MAP_PREFS_ORBIT_AUTO_KEY]: mapOrbitAutoPlay,
        })
      );
    } catch (_) {
      /* ignore */
    }
  }

  function visionSoftReload() {
    saveMapPrefs();
    location.reload();
  }

  function visionHardReset() {
    try {
      localStorage.removeItem(MAP_PREFS_KEY);
    } catch (_) {
      /* ignore */
    }
    location.reload();
  }

  function visionAnnouncePower(message) {
    const el = document.getElementById("vision-power-status");
    if (el && message) el.textContent = message;
  }

  function visionPowerMenuItems() {
    const menu = document.getElementById("vision-power-menu");
    if (!menu) return [];
    return Array.from(menu.querySelectorAll('[role="menuitem"]'));
  }

  function focusVisionPowerItem(index) {
    const items = visionPowerMenuItems();
    if (!items.length) return;
    const i = ((index % items.length) + items.length) % items.length;
    items[i].focus();
  }

  function visionPowerPoolai(action) {
    saveMapPrefs();
    try {
      localStorage.setItem(
        "poolai.vision.lastPower",
        JSON.stringify({ action: action, saved_at: Date.now() })
      );
      localStorage.setItem(
        "poolai.vision.lastLaunch",
        JSON.stringify({ saved_at: Date.now(), path: window.location.pathname })
      );
    } catch (_) {
      /* ignore */
    }
    return fetch("http://127.0.0.1:8080/api/v1/ops/power", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ action: action }),
    })
      .then(function (r) {
        return r.json();
      })
      .then(function (body) {
        visionAnnouncePower("Power " + action + ": " + (body.note || "accepted"));
      })
      .catch(function () {
        visionAnnouncePower("Power " + action + ": queued (stand may be down)");
      });
  }

  function closeVisionPowerMenu() {
    const menu = document.getElementById("vision-power-menu");
    const btn = document.getElementById("btn-power");
    if (menu) menu.hidden = true;
    if (btn) {
      btn.classList.remove("on");
      btn.setAttribute("aria-expanded", "false");
    }
  }

  function toggleVisionPowerMenu() {
    const menu = document.getElementById("vision-power-menu");
    const btn = document.getElementById("btn-power");
    if (!menu || !btn) return;
    const open = menu.hidden;
    if (open) {
      menu.hidden = false;
      btn.classList.add("on");
      btn.setAttribute("aria-expanded", "true");
      const first = visionPowerMenuItems()[0];
      if (first) first.focus();
    } else {
      closeVisionPowerMenu();
    }
  }

  function bindVisionPowerMenu() {
    const btn = document.getElementById("btn-power");
    const menu = document.getElementById("vision-power-menu");
    if (!btn || !menu) return;
    btn.addEventListener("click", function (ev) {
      ev.stopPropagation();
      toggleVisionPowerMenu();
    });
    document.getElementById("vision-power-shutdown")?.addEventListener("click", function () {
      visionPowerPoolai("shutdown");
      closeVisionPowerMenu();
    });
    document.getElementById("vision-power-reboot")?.addEventListener("click", function () {
      visionPowerPoolai("reboot");
      closeVisionPowerMenu();
    });
    document.getElementById("vision-power-soft")?.addEventListener("click", function () {
      closeVisionPowerMenu();
      visionSoftReload();
    });
    document.getElementById("vision-power-hard")?.addEventListener("click", function () {
      closeVisionPowerMenu();
      visionHardReset();
    });
    document.addEventListener("click", function (ev) {
      if (menu.hidden) return;
      if (ev.target === btn || menu.contains(ev.target)) return;
      closeVisionPowerMenu();
    });
    document.addEventListener("keydown", function (ev) {
      if (menu.hidden) return;
      if (ev.key === "Escape") {
        closeVisionPowerMenu();
        btn.focus();
        return;
      }
      const items = visionPowerMenuItems();
      const idx = items.indexOf(document.activeElement);
      if (ev.key === "ArrowDown") {
        ev.preventDefault();
        focusVisionPowerItem(idx < 0 ? 0 : idx + 1);
      } else if (ev.key === "ArrowUp") {
        ev.preventDefault();
        focusVisionPowerItem(idx <= 0 ? items.length - 1 : idx - 1);
      } else if (ev.key === "Home") {
        ev.preventDefault();
        focusVisionPowerItem(0);
      } else if (ev.key === "End") {
        ev.preventDefault();
        focusVisionPowerItem(items.length - 1);
      }
    });
  }

  function clusterStoreId(layer, key) {
    return layer + "::" + key;
  }

  function isClusterCollapsed(layer, key, count) {
    const min = collapseThresholdForLayer(layer);
    if (count < min) return false;
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
    const projected = map3DProjected.get(nodeId);
    if (projected) return projected;
    const p = nodePositions.get(nodeId);
    if (!p) return null;
    if (p.collapsedHidden && p.hubId) {
      return map3DProjected.get(p.hubId) || nodePositions.get(p.hubId);
    }
    return p;
  }

  function layerZDepth(layerId) {
    const layers = manifest && manifest.layers ? manifest.layers : [];
    const idx = layers.findIndex((l) => l.id === layerId);
    return (idx >= 0 ? idx : 0) * MAP_LAYER_Z_STEP;
  }

  function rotateProject3D(bx, by, bz, rotX, rotY, rotZ) {
    const px = bx - MAP_W / 2;
    const py = by - MAP_H / 2;
    const pz = bz;
    const rx = (rotX * Math.PI) / 180;
    const ry = (rotY * Math.PI) / 180;
    const rz = (rotZ * Math.PI) / 180;
    let x = px * Math.cos(ry) + pz * Math.sin(ry);
    let z = -px * Math.sin(ry) + pz * Math.cos(ry);
    let y = py;
    const y2 = y * Math.cos(rx) - z * Math.sin(rx);
    const z2 = y * Math.sin(rx) + z * Math.cos(rx);
    const x3 = x * Math.cos(rz) - y2 * Math.sin(rz);
    const y3 = x * Math.sin(rz) + y2 * Math.cos(rz);
    const depth = z2;
    const persp = MAP_3D_FOCAL / (MAP_3D_FOCAL + z2);
    return {
      x: x3 * persp + MAP_W / 2,
      y: y3 * persp + MAP_H / 2,
      depth: depth,
      scale: persp,
    };
  }

  function projectPlanePathD(bounds, layerId) {
    const z = layerZDepth(layerId);
    const cx = bounds.cx;
    const cy = bounds.cy;
    const hw = bounds.w / 2;
    const hh = bounds.h / 2;
    const corners = [
      [cx, cy + hh, z],
      [cx + hw, cy, z],
      [cx, cy - hh, z],
      [cx - hw, cy, z],
    ].map((c) =>
      rotateProject3D(c[0], c[1], c[2], mapOrbit.rotX, mapOrbit.rotY, mapOrbit.rotZ)
    );
    return (
      "M" +
      corners[0].x +
      "," +
      corners[0].y +
      " L" +
      corners[1].x +
      "," +
      corners[1].y +
      " L" +
      corners[2].x +
      "," +
      corners[2].y +
      " L" +
      corners[3].x +
      "," +
      corners[3].y +
      " Z"
    );
  }

  function applyMap3DProjection() {
    const world = document.getElementById("map-world");
    if (!world || !manifest) return;
    map3DProjected.clear();
    manifest.nodes.forEach((n) => {
      const base = nodePositions.get(n.id);
      if (!base || base.collapsedHidden) return;
      map3DProjected.set(
        n.id,
        rotateProject3D(
          base.x,
          base.y,
          layerZDepth(n.layer),
          mapOrbit.rotX,
          mapOrbit.rotY,
          mapOrbit.rotZ
        )
      );
    });

    world.querySelectorAll(".plane").forEach((path) => {
      const layer = path.dataset.layer;
      if (!layer) return;
      path.setAttribute("d", projectPlanePathD(layerPlaneBounds(layer), layer));
    });

    world.querySelectorAll(".layer-tier-label").forEach((label) => {
      const layer = label.dataset.layer;
      if (!layer) return;
      const bounds = layerPlaneBounds(layer);
      const p = rotateProject3D(
        28,
        bounds.cy + 4,
        layerZDepth(layer),
        mapOrbit.rotX,
        mapOrbit.rotY,
        mapOrbit.rotZ
      );
      label.setAttribute("x", p.x.toFixed(1));
      label.setAttribute("y", p.y.toFixed(1));
    });

    world.querySelectorAll(".edge").forEach((path) => {
      const a = mapPosForNode(path.dataset.from);
      const b = mapPosForNode(path.dataset.to);
      if (a && b) path.setAttribute("d", buildConstellationEdgePath(a, b));
    });

    clusterLabelPositions.forEach((cl, i) => {
      const label = world.querySelectorAll(".cluster-labels text")[i];
      if (!label) return;
      const p = rotateProject3D(
        cl.x,
        cl.y,
        layerZDepth(cl.layer),
        mapOrbit.rotX,
        mapOrbit.rotY,
        mapOrbit.rotZ
      );
      label.setAttribute("x", p.x.toFixed(1));
      label.setAttribute("y", p.y.toFixed(1));
    });

    const nodesG = world.querySelector(".nodes");
    if (!nodesG) return;
    const nodeEls = Array.from(nodesG.querySelectorAll(".node"));
    nodeEls.sort((ea, eb) => {
      const da = map3DProjected.get(ea.dataset.id);
      const db = map3DProjected.get(eb.dataset.id);
      return (da ? da.depth : 0) - (db ? db.depth : 0);
    });
    nodeEls.forEach((el) => nodesG.appendChild(el));
    nodeEls.forEach((el) => {
      const id = el.dataset.id;
      const base = nodePositions.get(id);
      const proj = map3DProjected.get(id);
      if (!base || !proj) return;
      const n = nodeById(id);
      const circle = el.querySelector("circle");
      const text = el.querySelector("text") || el._labelEl;
      const r = nodeRenderRadius(n, base) * Math.max(0.62, Math.min(1.15, proj.scale));
      if (circle) {
        circle.setAttribute("cx", proj.x.toFixed(2));
        circle.setAttribute("cy", proj.y.toFixed(2));
        circle.setAttribute("r", r.toFixed(2));
      }
      if (text) {
        text.setAttribute("x", proj.x.toFixed(2));
        text.setAttribute("y", (proj.y + r + (r <= 5 ? 8 : 11)).toFixed(2));
      }
    });
    updateMapSelectionCallout3D();
  }

  function updateMapSelectionCallout3D() {
    const world = document.querySelector("#map-svg #map-world");
    if (!world) return;
    const callout = world.querySelector("#map-selection-callout");
    if (!callout || !selectedId) return;
    const n = nodeById(selectedId);
    const pos = mapPosForNode(selectedId);
    const base = nodePositions.get(selectedId);
    if (!n || !pos || !base) return;
    const hub = !!base.clusterHub;
    const r = nodeBaseRadius(selectedId, hub) + 4;
    const caption = mapNodeFullLabel(n, base);
    const fontSize = 11;
    const padX = 8;
    const padY = 5;
    const estW = Math.min(MAP_W - 48, Math.max(72, caption.length * (fontSize * 0.58)));
    const estH = fontSize + padY * 2;
    const cx = Math.min(MAP_W - 24 - estW / 2, Math.max(24 + estW / 2, pos.x));
    const cy = pos.y - r - 10 - estH / 2;
    const rect = callout.querySelector("rect");
    const text = callout.querySelector("text");
    if (rect) {
      rect.setAttribute("x", (cx - estW / 2).toFixed(1));
      rect.setAttribute("y", (cy - estH / 2).toFixed(1));
    }
    if (text) {
      text.setAttribute("x", cx.toFixed(1));
      text.setAttribute("y", (cy + fontSize * 0.35).toFixed(1));
    }
  }

  function mapNodeDimmed(node) {
    return (
      mapSprintFocus && activeSprint && !nodeInActiveSprint(node)
    );
  }

  /** PH-S1043: sprint-dim without full map rebuild. */
  function updateMapSprintDim() {
    const svg = document.getElementById("map-svg");
    if (!svg || !manifest) return;
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const n = nodeById(el.dataset.id);
      if (!n) return;
      el.classList.toggle("sprint-dim", mapNodeDimmed(n));
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const a = nodeById(el.dataset.from);
      const b = nodeById(el.dataset.to);
      el.classList.toggle(
        "sprint-dim",
        !!(a && b && mapNodeDimmed(a) && mapNodeDimmed(b))
      );
    });
    updateMapSelection();
  }

  function mapNodeShortLabel(n, pos) {
    if (pos.clusterHub) {
      return clusterDisplayName(pos.cluster) + " (" + pos.clusterCount + ")";
    }
    const maxLen = pos.cluster && n.layer === "L3" ? 16 : 22;
    return n.label.length > maxLen
      ? n.label.slice(0, maxLen - 1) + "…"
      : n.label;
  }

  function mapNodeFullLabel(n, pos) {
    if (pos.clusterHub) {
      return clusterDisplayName(pos.cluster) + " (" + pos.clusterCount + ")";
    }
    return n.label;
  }

  function layerDisplayName(layerId) {
    const layers = (manifest && manifest.layers) || [];
    const found = layers.find((l) => l.id === layerId);
    return found ? found.name : layerId || "unknown";
  }

  function announceMapSelection(node) {
    const live = document.getElementById("map-selection-live");
    if (!live) return;
    if (!node) {
      live.textContent = "";
      return;
    }
    const pos = nodePositions.get(node.id);
    const label = pos ? mapNodeFullLabel(node, pos) : node.label || node.id;
    const layer = layerDisplayName(node.layer);
    const path = node.path || node.id;
    live.textContent = "Selected " + label + ", layer " + layer + ", " + path;
  }

  function nodeBaseRadius(id, hub) {
    const n = nodeById(id);
    const pos = nodePositions.get(id);
    if (n && pos) return nodeRenderRadius(n, pos);
    if (hub) return 11;
    if (id === "galaxy_grid") return 14;
    return 8;
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

  function hashUnit(str) {
    let h = 0;
    for (let i = 0; i < str.length; i++) h = (h * 31 + str.charCodeAt(i)) | 0;
    return (Math.abs(h) % 10000) / 10000;
  }

  function nodeDegree(id) {
    return degreeById.get(id) || 0;
  }

  function edgeKey(a, b) {
    return a < b ? a + "|" + b : b + "|" + a;
  }

  function rebuildManifestIndexes() {
    if (!manifest) {
      nodeIndex = new Map();
      degreeById = new Map();
      layerNodeCounts = new Map();
      adjacency = null;
      constellationHubIdSet = new Set();
      return;
    }
    nodeIndex = new Map();
    degreeById = new Map();
    layerNodeCounts = new Map();
    manifest.nodes.forEach((n) => {
      nodeIndex.set(n.id, n);
      degreeById.set(n.id, 0);
      layerNodeCounts.set(n.layer, (layerNodeCounts.get(n.layer) || 0) + 1);
    });
    (manifest.edges || []).forEach((e) => {
      degreeById.set(e.from, (degreeById.get(e.from) || 0) + 1);
      degreeById.set(e.to, (degreeById.get(e.to) || 0) + 1);
    });
    constellationHubIdSet = new Set();
    manifest.nodes.forEach((n) => {
      if (CONSTELLATION_HUB_IDS.has(n.id) || (degreeById.get(n.id) || 0) >= 4) {
        constellationHubIdSet.add(n.id);
      }
    });
    adjacency = buildAdjacency();
  }

  function buildAdjacency() {
    const adj = new Map();
    function link(a, b) {
      if (!adj.has(a)) adj.set(a, []);
      adj.get(a).push(b);
    }
    (manifest.edges || []).forEach((e) => {
      link(e.from, e.to);
      link(e.to, e.from);
    });
    return adj;
  }

  function pickClusterHub(nodes) {
    for (let i = 0; i < nodes.length; i++) {
      if (CONSTELLATION_HUB_IDS.has(nodes[i].id)) return nodes[i];
    }
    const manual = nodes.filter((n) => !nodeIsAutoSynced(n));
    const pool = manual.length ? manual : nodes;
    return pool
      .slice()
      .sort((a, b) => nodeDegree(b.id) - nodeDegree(a.id))[0];
  }

  function constellationHubIds() {
    return Array.from(constellationHubIdSet);
  }

  function computeConstellationHighlight(fromId) {
    const adj = adjacency || buildAdjacency();
    const hubs = constellationHubIds();
    const parent = new Map([[fromId, null]]);
    const visited = new Set([fromId]);
    const queue = [fromId];
    while (queue.length) {
      const u = queue.shift();
      (adj.get(u) || []).forEach((v) => {
        if (visited.has(v)) return;
        visited.add(v);
        parent.set(v, u);
        queue.push(v);
      });
    }
    const litNodes = new Set([fromId]);
    const litEdges = new Set();
    hubs.forEach((hub) => {
      if (!visited.has(hub)) return;
      let cur = hub;
      while (cur && cur !== fromId) {
        const p = parent.get(cur);
        if (!p) break;
        litNodes.add(cur);
        litNodes.add(p);
        litEdges.add(edgeKey(p, cur));
        cur = p;
      }
    });
    (adj.get(fromId) || []).forEach((v) => {
      litNodes.add(v);
      litEdges.add(edgeKey(fromId, v));
    });
    return { nodes: litNodes, edges: litEdges };
  }

  function buildConstellationEdgePath(a, b) {
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const dist = Math.hypot(dx, dy) || 1;
    const bend = Math.min(56, dist * 0.28) * (hashUnit(a.x + "," + b.y) > 0.5 ? 1 : -1);
    const nx = -dy / dist;
    const ny = dx / dist;
    const mx = (a.x + b.x) / 2 + nx * bend;
    const my = (a.y + b.y) / 2 + ny * bend;
    return (
      "M" + a.x + "," + a.y + " Q" + mx + "," + my + " " + b.x + "," + b.y
    );
  }

  function nodeIsAutoSynced(n) {
    return n && n.auto_synced === true;
  }

  function nodeVisualWeight(n) {
    if (!n) return 0.3;
    if (n.id === "galaxy_grid") return 1;
    if (CONSTELLATION_HUB_IDS.has(n.id)) return 0.92;
    const deg = nodeDegree(n.id);
    if (deg >= 5) return 0.82;
    if (deg >= 2) return 0.62;
    if (nodeIsAutoSynced(n)) return 0.22;
    return 0.52;
  }

  function nodeRenderRadius(n, pos) {
    if (pos && pos.clusterHub) {
      const c = pos.clusterCount || pos.mass || 1;
      return Math.min(20, 8 + Math.sqrt(c) * 1.75);
    }
    if (n.id === "galaxy_grid") return 14;
    const w = nodeVisualWeight(n);
    if (w >= 0.9) return 12;
    if (w >= 0.75) return 10;
    if (w >= 0.55) return 8;
    if (w >= 0.35) return 6;
    return 4;
  }

  function labelPriority(n) {
    if (!n) return 0;
    if (n.id === "galaxy_grid") return 100;
    if (CONSTELLATION_HUB_IDS.has(n.id)) return 90;
    return Math.round(nodeVisualWeight(n) * 60 + nodeDegree(n.id) * 4);
  }

  function collapseThresholdForLayer(layer) {
    const count = layerNodeCounts.get(layer) || 0;
    if (count > 120) return 2;
    if (count > 50) return 3;
    return CLUSTER_COLLAPSE_MIN;
  }

  function isMapOverviewMode() {
    const dense =
      manifest && manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD;
    let threshold = dense ? 1.05 : MAP_OVERVIEW_ZOOM;
    const maxLayerCount = layerNodeCounts
      ? Math.max(0, ...Array.from(layerNodeCounts.values()))
      : 0;
    if (maxLayerCount > 120) threshold = Math.max(threshold, 1.12);
    return mapView.scale <= threshold;
  }

  function isMapOverviewHub(n, pos) {
    if (!n) return false;
    if (n.id === "galaxy_grid") return true;
    if (CONSTELLATION_HUB_IDS.has(n.id)) return true;
    if (pos && pos.clusterHub) return true;
    if (nodeDegree(n.id) >= 5) return true;
    return nodeVisualWeight(n) >= 0.82;
  }

  function shouldShowNodeLabel(n, pos) {
    if (isMapOverviewMode()) {
      return isMapOverviewHub(n, pos);
    }
    if (pos && pos.clusterHub) return true;
    if (CONSTELLATION_HUB_IDS.has(n.id)) return true;
    if (nodeDegree(n.id) >= 4) return true;
    if (nodeVisualWeight(n) >= 0.75) return true;
    if (nodeIsAutoSynced(n)) {
      return mapView.scale >= LABEL_ZOOM_LEAF;
    }
    const layerCount = layerNodeCounts.get(n.layer) || 0;
    if (layerCount > MAP_DENSE_NODE_THRESHOLD) {
      return mapView.scale >= LABEL_ZOOM_NORMAL;
    }
    return mapView.scale >= 0.92;
  }

  function labelFontSize(n, pos) {
    const w = nodeVisualWeight(n);
    if (pos && pos.clusterHub) return 8;
    if (w >= 0.85) return 10;
    if (w >= 0.55) return 9;
    return 8;
  }

  function rectsOverlap(a, b) {
    return !(
      a.x + a.w < b.x ||
      b.x + b.w < a.x ||
      a.y + a.h < b.y ||
      b.y + b.h < a.y
    );
  }

  function declutterNodeLabels(svg) {
    const nodes = Array.from(svg.querySelectorAll("#map-svg .node text"))
      .filter((t) => t.style.display !== "none" && t.getAttribute("opacity") !== "0")
      .map((text) => ({
        text,
        priority: labelPriority(nodeById(text.parentElement.dataset.id)),
      }))
      .sort((a, b) => b.priority - a.priority);
    const placed = [];
    nodes.forEach(({ text, priority }) => {
      if (priority < 35 && mapView.scale < LABEL_ZOOM_NORMAL) {
        text.style.display = "none";
        return;
      }
      let bb;
      try {
        bb = text.getBBox();
      } catch (_) {
        return;
      }
      const rect = { x: bb.x - 2, y: bb.y - 1, w: bb.width + 4, h: bb.height + 2 };
      if (placed.some((p) => rectsOverlap(p, rect))) {
        text.style.display = "none";
      } else {
        text.style.display = "";
        placed.push(rect);
      }
    });
  }

  function updateMapLabelZoom() {
    const svg = document.getElementById("map-svg");
    if (!svg || !manifest) return;
    const scaleDelta = Math.abs(mapView.scale - lastLabelZoomScale);
    if (isLowGpuMode() && scaleDelta < 0.08 && lastLabelZoomScale > 0) {
      return;
    }
    lastLabelZoomScale = mapView.scale;
    svg.classList.toggle(
      "map-dense",
      manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD
    );
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const n = nodeById(el.dataset.id);
      const pos = nodePositions.get(el.dataset.id);
      const text = el.querySelector("text");
      const circle = el.querySelector("circle");
      if (!n || !pos || !text || !circle) return;
      const hub = el.classList.contains("cluster-hub");
      const r = nodeRenderRadius(n, pos);
      circle.setAttribute("r", el.classList.contains("selected") ? r + 3 : r);
      const show =
        shouldShowNodeLabel(n, pos) || el.classList.contains("label-hover");
      text.style.display = show ? "" : "none";
      if (show) {
        text.setAttribute("font-size", String(labelFontSize(n, pos)));
        text.setAttribute("y", pos.y + r + (r <= 5 ? 8 : 11));
      }
      el.classList.toggle("auto-leaf", nodeIsAutoSynced(n) && !hub);
    });
    if (!isLowGpuMode() || mapView.scale >= LABEL_ZOOM_NORMAL) {
      declutterNodeLabels(svg);
    }
    updateMapOverviewLod();
  }

  function updateMapOverviewLod() {
    const svg = document.getElementById("map-svg");
    const wrap = svg && svg.closest(".map-wrap");
    if (!svg) return;
    const overview = isMapOverviewMode();
    svg.classList.toggle("map-overview", overview);
    if (wrap) wrap.classList.toggle("map-overview", overview);
    if (!manifest) return;
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const n = nodeById(el.dataset.id);
      const pos = nodePositions.get(el.dataset.id);
      el.classList.toggle("overview-hidden", overview && !isMapOverviewHub(n, pos));
    });
    document.querySelectorAll("#map-svg .cluster-label").forEach((el) => {
      el.classList.toggle("overview-hidden", overview);
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = nodeById(el.dataset.from);
      const to = nodeById(el.dataset.to);
      const pf = nodePositions.get(el.dataset.from);
      const pt = nodePositions.get(el.dataset.to);
      const hubEdge = isMapOverviewHub(from, pf) && isMapOverviewHub(to, pt);
      el.classList.toggle("overview-hidden", overview && !hubEdge);
    });
    updateMinimapViewport();
  }

  function updateMinimapViewport() {
    const vp = document.getElementById("minimap-viewport");
    if (!vp) return;
    const s = mapView.scale;
    vp.setAttribute("x", String(-mapView.tx / s));
    vp.setAttribute("y", String(-mapView.ty / s));
    vp.setAttribute("width", String(MAP_W / s));
    vp.setAttribute("height", String(MAP_H / s));
    updateMinimapSelection();
  }

  function updateMinimapSelection() {
    const mini = document.getElementById("map-minimap-svg");
    if (!mini || !manifest) return;
    const ring = document.getElementById("minimap-selection-ring");
    if (!ring) return;

    const dense = manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD;
    if (
      selectedId &&
      dense &&
      !mini.querySelector('.minimap-node[data-id="' + selectedId + '"]')
    ) {
      renderMinimap();
      return;
    }

    mini.querySelectorAll(".minimap-node").forEach((el) => {
      el.classList.toggle("minimap-selected", el.dataset.id === selectedId);
    });

    if (!selectedId) {
      ring.setAttribute("visibility", "hidden");
      return;
    }
    const pos = nodePositions.get(selectedId);
    if (!pos || pos.collapsedHidden) {
      ring.setAttribute("visibility", "hidden");
      return;
    }
    const n = nodeById(selectedId);
    const hub = isMapOverviewHub(n, pos);
    const r = hub ? 8 : 6;
    ring.setAttribute("cx", String(pos.x));
    ring.setAttribute("cy", String(pos.y));
    ring.setAttribute("r", String(r));
    ring.setAttribute("visibility", "visible");
  }

  function renderMinimap() {
    const mini = document.getElementById("map-minimap-svg");
    if (!mini || !manifest) return;
    const ns = "http://www.w3.org/2000/svg";
    while (mini.firstChild) mini.removeChild(mini.firstChild);
    mini.setAttribute("viewBox", "0 0 " + MAP_W + " " + MAP_H);

    const dense = manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD;
    const nodesG = document.createElementNS(ns, "g");
    nodesG.setAttribute("class", "minimap-nodes");
    manifest.nodes.forEach((n) => {
      const pos = nodePositions.get(n.id);
      if (!pos || pos.collapsedHidden) return;
      const hub = isMapOverviewHub(n, pos);
      const isSel = n.id === selectedId;
      if (dense && !hub && !isSel) return;
      const dot = document.createElementNS(ns, "circle");
      dot.setAttribute("class", "minimap-node" + (isSel ? " minimap-selected" : ""));
      dot.setAttribute("cx", String(pos.x));
      dot.setAttribute("cy", String(pos.y));
      dot.setAttribute("r", isSel ? (hub ? "3.2" : "2.4") : hub ? "3" : "1.6");
      dot.setAttribute("fill", LAYER_COLORS[n.layer] || "#666");
      dot.dataset.id = n.id;
      nodesG.appendChild(dot);
    });
    mini.appendChild(nodesG);

    const ring = document.createElementNS(ns, "circle");
    ring.setAttribute("id", "minimap-selection-ring");
    ring.setAttribute("class", "minimap-selection-ring");
    ring.setAttribute("fill", "none");
    ring.setAttribute("visibility", "hidden");
    mini.appendChild(ring);

    const vp = document.createElementNS(ns, "rect");
    vp.setAttribute("id", "minimap-viewport");
    vp.setAttribute("class", "minimap-viewport");
    vp.setAttribute("fill", "none");
    mini.appendChild(vp);
    updateMinimapViewport();
    bindMinimap(mini);
  }

  function bindMinimap(svg) {
    if (minimapBound) return;
    minimapBound = true;
    svg.addEventListener("click", (ev) => {
      const rect = svg.getBoundingClientRect();
      const mx = ((ev.clientX - rect.left) / rect.width) * MAP_W;
      const my = ((ev.clientY - rect.top) / rect.height) * MAP_H;
      mapView.tx = MAP_W / 2 - mx * mapView.scale;
      mapView.ty = MAP_H / 2 - my * mapView.scale;
      applyMapTransform();
    });
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
    scheduleMapLabelZoom();
    updateMinimapViewport();
  }

  function scheduleMapLabelZoom() {
    if (labelZoomRaf) return;
    labelZoomRaf = requestAnimationFrame(() => {
      labelZoomRaf = null;
      updateMapLabelZoom();
    });
  }

  function isVisionEco() {
    return visionMode === "eco";
  }

  function isVisionMs() {
    return visionMode === "ms";
  }

  /** Eco + Ms share low-GPU rendering (Ms keeps edges for hover trace). */
  function isLowGpuMode() {
    return visionMode === "eco" || visionMode === "ms";
  }

  function prefersReducedMotion() {
    return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  }

  /** Eco/Ms + OS reduced-motion skip map glow and edge animation (PH-S212). */
  function isMapFxOff() {
    return isLowGpuMode() || prefersReducedMotion();
  }

  function watchIntervalMs() {
    return isLowGpuMode() ? WATCH_INTERVAL_ECO_MS : WATCH_INTERVAL_MS;
  }

  function resolveVisionModeDefault() {
    if (visionModePinned || !manifest) return;
    if (manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD) {
      visionMode = "eco";
    }
  }

  function applyVisionMode() {
    document.body.classList.toggle("vision-eco", isVisionEco());
    document.body.classList.toggle("vision-fx", visionMode === "fx");
    document.body.classList.toggle("vision-ms", isVisionMs());
    document.body.classList.toggle("vision-reduced-motion", prefersReducedMotion());
    const svg = document.getElementById("map-svg");
    if (svg) svg.classList.toggle("map-fx-off", isMapFxOff());
    const btn = document.getElementById("btn-eco");
    if (btn) {
      btn.classList.toggle("on", isVisionEco());
      btn.classList.toggle("fx", visionMode === "fx");
      btn.classList.toggle("ms", isVisionMs());
      btn.textContent =
        visionMode === "eco" ? "Eco" : visionMode === "fx" ? "FX" : "Ms";
      btn.title =
        visionMode === "eco"
          ? "Eco — low GPU. Click → FX."
          : visionMode === "fx"
            ? "FX — full glow. Click → Ms (hover trace)."
            : "Ms — 1-hop edge highlight on hover. Click → Eco.";
      btn.setAttribute("aria-label", "Vision GPU mode — " + visionMode.toUpperCase());
      btn.setAttribute("aria-pressed", visionMode === "eco" ? "true" : "false");
    }
    const autoBtn = document.getElementById("btn-auto");
    if (autoBtn) {
      autoBtn.setAttribute("aria-pressed", autoReloadEnabled ? "true" : "false");
    }
    if (!isVisionMs()) hoverTraceId = null;
    restartStarfield();
    if (autoReloadEnabled) startAutoReload();
    updateMapHoverTrace();
  }

  function cycleVisionMode() {
    const idx = VISION_MODES.indexOf(visionMode);
    visionMode = VISION_MODES[(idx + 1) % VISION_MODES.length];
    visionModePinned = true;
    saveMapPrefs();
    applyVisionMode();
    if (manifest) renderMap();
  }

  function computeOneHopHighlight(centerId) {
    const adj = adjacency || buildAdjacency();
    const litNodes = new Set([centerId]);
    const litEdges = new Set();
    (adj.get(centerId) || []).forEach((v) => {
      litNodes.add(v);
      litEdges.add(edgeKey(centerId, v));
    });
    return { nodes: litNodes, edges: litEdges };
  }

  function clearMapHoverTraceClasses() {
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      el.classList.remove("trace-center", "trace-lit", "trace-dim");
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      el.classList.remove("trace-edge", "trace-dim", "edge-reveal");
    });
  }

  function updateMapEdgeSelectTrace() {
    if (!edgeSelectTraceKey || !selectedId) return;
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = el.dataset.from;
      const to = el.dataset.to;
      const key = el.dataset.edgeKey || edgeKey(from, to);
      const active = key === edgeSelectTraceKey;
      el.classList.toggle("edge-click-active", active);
      el.classList.toggle("trace-edge", active);
      if (active) el.classList.toggle("edge-reveal", true);
    });
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const id = el.dataset.id;
      const isEndpoint = id === selectedId;
      const isPartner = id === edgeSelectPartnerId;
      el.classList.toggle("trace-center", isEndpoint);
      el.classList.toggle("trace-lit", isPartner);
    });
  }

  function handleMapEdgeClick(edgeEl, ev) {
    ev.stopPropagation();
    clearTimeout(clickFocusTimer);
    clickFocusTimer = null;
    const from = edgeEl.dataset.from;
    const to = edgeEl.dataset.to;
    const pt = mapSvgClientToMap(ev);
    const endpointId = edgeTraceNodeId(
      edgeEl,
      pt ? pt.x : 0,
      pt ? pt.y : 0
    );
    const n = nodeById(endpointId);
    if (!n || !isNodeOnMap(endpointId)) return;
    const partnerId = endpointId === from ? to : from;
    selectNode(n);
    edgeSelectTraceKey = edgeEl.dataset.edgeKey || edgeKey(from, to);
    edgeSelectPartnerId = partnerId;
    hoverTraceId = endpointId;
    updateMapSelection();
    focusMapNode(n, { pushHistory: true });
  }

  function updateMapHoverTrace() {
    if (edgeSelectTraceKey) return;
    if (!isVisionMs() || !hoverTraceId) {
      clearMapHoverTraceClasses();
      return;
    }
    const trace = computeOneHopHighlight(hoverTraceId);
    const hasSel = !!selectedId;
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const id = el.dataset.id;
      const onTrace = trace.nodes.has(id);
      el.classList.toggle("trace-center", id === hoverTraceId);
      el.classList.toggle("trace-lit", onTrace && id !== hoverTraceId);
      el.classList.toggle("trace-dim", !hasSel && !onTrace);
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = el.dataset.from;
      const to = el.dataset.to;
      const key = el.dataset.edgeKey || edgeKey(from, to);
      const onTrace = trace.edges.has(key);
      el.classList.toggle("trace-edge", onTrace);
      el.classList.toggle("edge-reveal", onTrace);
      el.classList.toggle("trace-dim", !hasSel && !onTrace);
    });
  }

  function mapEdgeSparse(e) {
    if (!isVisionEco() || !manifest) return false;
    if (manifest.nodes.length < MAP_DENSE_NODE_THRESHOLD) return false;
    if (CONSTELLATION_HUB_IDS.has(e.from) || CONSTELLATION_HUB_IDS.has(e.to)) {
      return false;
    }
    const pa = nodePositions.get(e.from);
    const pb = nodePositions.get(e.to);
    if (pa && pa.clusterHub) return false;
    if (pb && pb.clusterHub) return false;
    return true;
  }

  function computeMapProjectedBounds() {
    if (!manifest) return null;
    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;
    let count = 0;
    manifest.nodes.forEach((n) => {
      const base = nodePositions.get(n.id);
      if (!base || base.collapsedHidden) return;
      const proj = rotateProject3D(
        base.x,
        base.y,
        layerZDepth(n.layer),
        mapOrbit.rotX,
        mapOrbit.rotY,
        mapOrbit.rotZ
      );
      const r =
        nodeRenderRadius(n, base) * Math.max(0.62, Math.min(1.15, proj.scale));
      const pad = r + 10;
      minX = Math.min(minX, proj.x - pad);
      minY = Math.min(minY, proj.y - pad);
      maxX = Math.max(maxX, proj.x + pad);
      maxY = Math.max(maxY, proj.y + pad);
      count++;
    });
    if (!count || !Number.isFinite(minX)) return null;
    return { minX, minY, maxX, maxY, count };
  }

  function fitMapToContent(opts) {
    const padding =
      opts && opts.padding != null ? opts.padding : MAP_FIT_PADDING;
    const bounds = computeMapProjectedBounds();
    if (!bounds) return;
    const contentW = bounds.maxX - bounds.minX + padding * 2;
    const contentH = bounds.maxY - bounds.minY + padding * 2;
    if (contentW <= 0 || contentH <= 0) return;
    const scale = Math.min(
      MAP_MAX_SCALE,
      Math.max(MAP_MIN_SCALE, Math.min(MAP_W / contentW, MAP_H / contentH))
    );
    const cx = (bounds.minX + bounds.maxX) / 2;
    const cy = (bounds.minY + bounds.maxY) / 2;
    if (opts && opts.pushHistory) pushMapViewState();
    mapView.scale = scale;
    mapView.tx = MAP_W / 2 - cx * scale;
    mapView.ty = MAP_H / 2 - cy * scale;
    applyMapTransform();
  }

  function resetMapView() {
    resetMapOrbit();
    fitMapToContent();
  }

  function syncMapOrbitAutoBtn() {
    const btn = document.getElementById("map-orbit-auto");
    if (!btn) return;
    btn.disabled = false;
    btn.classList.toggle("on", mapOrbitAutoPlay);
    btn.textContent = mapOrbitAutoPlay ? "⏸" : "▶";
    btn.title = mapOrbitAutoPlay
      ? "Pause auto-orbit (~30% WASD speed)"
      : "Play auto-orbit (~30% WASD speed)";
    btn.setAttribute(
      "aria-label",
      mapOrbitAutoPlay ? "Pause auto-orbit" : "Play auto-orbit"
    );
    btn.setAttribute("aria-pressed", mapOrbitAutoPlay ? "true" : "false");
  }

  function stopMapOrbitAutoLoop() {
    if (mapOrbitAutoRaf) {
      cancelAnimationFrame(mapOrbitAutoRaf);
      mapOrbitAutoRaf = 0;
    }
    mapOrbitAutoLastTs = 0;
    mapOrbitAutoTicking = false;
  }

  function pauseMapOrbitAuto() {
    mapOrbitAutoPlay = false;
    stopMapOrbitAutoLoop();
    syncMapOrbitAutoBtn();
    saveMapPrefs();
  }

  function scheduleMapOrbitAutoFrame(tick) {
    if (!mapOrbitAutoPlay) {
      mapOrbitAutoRaf = 0;
      return;
    }
    mapOrbitAutoRaf = requestAnimationFrame(tick);
  }

  function startMapOrbitAuto() {
    mapOrbitAutoPlay = true;
    syncMapOrbitAutoBtn();
    saveMapPrefs();
    if (mapOrbitAutoRaf) return;
    if (!manifest) return;
    function tick(ts) {
      if (!mapOrbitAutoPlay) {
        stopMapOrbitAutoLoop();
        return;
      }
      if (document.hidden) {
        mapOrbitAutoLastTs = 0;
        scheduleMapOrbitAutoFrame(tick);
        return;
      }
      if (!mapOrbitAutoLastTs) mapOrbitAutoLastTs = ts;
      const dt = Math.min(0.1, (ts - mapOrbitAutoLastTs) / 1000);
      mapOrbitAutoLastTs = ts;
      mapOrbitAutoTicking = true;
      mapOrbit.rotY += MAP_ORBIT_AUTO_DEG_PER_SEC * dt;
      clampMapOrbit();
      applyMapOrbitTransform();
      mapOrbitAutoTicking = false;
      scheduleMapOrbitAutoFrame(tick);
    }
    scheduleMapOrbitAutoFrame(tick);
  }

  function toggleMapOrbitAuto() {
    if (mapOrbitAutoPlay) {
      pauseMapOrbitAuto();
      return;
    }
    stopMapOrbitAutoLoop();
    startMapOrbitAuto();
  }

  function ensureMapOrbitAutoAfterFit() {
    if (!mapInitialFitDone) return;
    if (mapOrbitAutoPlay) startMapOrbitAuto();
  }

  function mapOrbitTransformString() {
    return (
      "rotateX(" +
      mapOrbit.rotX.toFixed(2) +
      "deg) rotateY(" +
      mapOrbit.rotY.toFixed(2) +
      "deg) rotateZ(" +
      mapOrbit.rotZ.toFixed(2) +
      "deg)"
    );
  }

  function clampMapOrbit() {
    mapOrbit.rotX = Math.max(-MAP_ORBIT_CLAMP_X, Math.min(MAP_ORBIT_CLAMP_X, mapOrbit.rotX));
    while (mapOrbit.rotY > 180) mapOrbit.rotY -= 360;
    while (mapOrbit.rotY < -180) mapOrbit.rotY += 360;
    mapOrbit.rotZ = Math.max(-40, Math.min(40, mapOrbit.rotZ));
  }

  function applyMapOrbitTransform() {
    applyMap3DProjection();
    const stack = document.getElementById("layer-stack");
    if (stack) stack.style.transform = mapOrbitTransformString();
    syncLayerStack3D();
  }

  function syncLayerStack3D() {
    const stack = document.getElementById("layer-stack");
    if (!stack || !manifest) return;
    const layers = manifest.layers || [];
    stack.querySelectorAll(".layer-plane").forEach((el) => {
      const idx = layers.findIndex((l) => l.id === el.dataset.layer);
      const z = (idx >= 0 ? idx : 0) * 18;
      const lift =
        !stackLayerFocus || el.dataset.layer === stackLayerFocus ? 10 : 0;
      el.style.transform = "translateZ(" + (z + lift) + "px)";
    });
  }

  function resetMapOrbit() {
    mapOrbit.rotX = MAP_ORBIT_DEFAULT.rotX;
    mapOrbit.rotY = MAP_ORBIT_DEFAULT.rotY;
    mapOrbit.rotZ = MAP_ORBIT_DEFAULT.rotZ;
    applyMapOrbitTransform();
    updateOrbitPadStick(0, 0);
  }

  function adjustMapOrbit(dRotX, dRotY) {
    if (!mapOrbitAutoTicking) pauseMapOrbitAuto();
    mapOrbit.rotX += dRotX;
    mapOrbit.rotY += dRotY;
    clampMapOrbit();
    applyMapOrbitTransform();
  }

  function orbitKeyBlocked(ev) {
    const t = ev.target;
    if (!t) return false;
    if (t.isContentEditable) return true;
    const tag = t.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
    return false;
  }

  function updateOrbitPadStick(dx, dy) {
    const stick = document.getElementById("map-orbit-stick");
    if (!stick) return;
    const max = 22;
    const len = Math.hypot(dx, dy);
    const scale = len > max ? max / len : 1;
    stick.style.transform =
      "translate(" + (dx * scale).toFixed(1) + "px," + (dy * scale).toFixed(1) + "px)";
  }

  function initMapOrbitControls() {
    if (mapOrbitBound) return;
    mapOrbitBound = true;
    applyMapOrbitTransform();

    document.addEventListener("keydown", (ev) => {
      if (orbitKeyBlocked(ev)) return;
      if (ev.metaKey || ev.ctrlKey || ev.altKey) return;
      let handled = false;
      switch (ev.key) {
        case "w":
        case "W":
          adjustMapOrbit(-MAP_ORBIT_STEP_DEG, 0);
          handled = true;
          break;
        case "s":
        case "S":
          adjustMapOrbit(MAP_ORBIT_STEP_DEG, 0);
          handled = true;
          break;
        case "a":
        case "A":
          adjustMapOrbit(0, -MAP_ORBIT_STEP_DEG);
          handled = true;
          break;
        case "d":
        case "D":
          adjustMapOrbit(0, MAP_ORBIT_STEP_DEG);
          handled = true;
          break;
        default:
          break;
      }
      if (handled) {
        ev.preventDefault();
        saveMapPrefs();
      }
    });

    const pad = document.getElementById("map-orbit-pad");
    if (!pad) return;

    let padActive = false;
    let padCx = 0;
    let padCy = 0;

    function orbitFromPadPointer(clientX, clientY) {
      const dx = clientX - padCx;
      const dy = clientY - padCy;
      adjustMapOrbit(dy * MAP_ORBIT_PAD_SENS, dx * MAP_ORBIT_PAD_SENS);
      updateOrbitPadStick(dx, dy);
    }

    function startPad(ev) {
      if (ev.button !== undefined && ev.button !== 0) return;
      padActive = true;
      pad.classList.add("orbit-active");
      const r = pad.getBoundingClientRect();
      padCx = r.left + r.width / 2;
      padCy = r.top + r.height / 2;
      orbitFromPadPointer(ev.clientX, ev.clientY);
      ev.preventDefault();
    }

    function movePad(ev) {
      if (!padActive) return;
      orbitFromPadPointer(ev.clientX, ev.clientY);
      ev.preventDefault();
    }

    function endPad() {
      if (!padActive) return;
      padActive = false;
      pad.classList.remove("orbit-active");
      updateOrbitPadStick(0, 0);
      saveMapPrefs();
    }

    pad.addEventListener("mousedown", startPad);
    window.addEventListener("mousemove", movePad);
    window.addEventListener("mouseup", endPad);
    pad.addEventListener(
      "touchstart",
      (ev) => {
        if (!ev.touches.length) return;
        startPad(ev.touches[0]);
      },
      { passive: false }
    );
    window.addEventListener(
      "touchmove",
      (ev) => {
        if (!padActive || !ev.touches.length) return;
        movePad(ev.touches[0]);
      },
      { passive: false }
    );
    window.addEventListener("touchend", endPad);
    window.addEventListener("touchcancel", endPad);
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

  function cloneMapView() {
    return { tx: mapView.tx, ty: mapView.ty, scale: mapView.scale };
  }

  function syncMapZoomBackBtn() {
    const btn = document.getElementById("map-zoom-back");
    if (!btn) return;
    btn.disabled = mapViewStack.length === 0;
  }

  function pushMapViewState() {
    mapViewStack.push(cloneMapView());
    if (mapViewStack.length > 24) mapViewStack.shift();
    syncMapZoomBackBtn();
  }

  function popMapViewState() {
    const prev = mapViewStack.pop();
    if (!prev) return;
    mapView = prev;
    applyMapTransform();
    syncMapZoomBackBtn();
  }

  function mapSvgClientToMap(ev) {
    const svg = document.getElementById("map-svg");
    if (!svg) return null;
    const rect = svg.getBoundingClientRect();
    if (!rect.width || !rect.height) return null;
    return {
      x: ((ev.clientX - rect.left) / rect.width) * MAP_W,
      y: ((ev.clientY - rect.top) / rect.height) * MAP_H,
    };
  }

  function edgeTraceNodeId(edgeEl, mx, my) {
    const from = edgeEl.dataset.from;
    const to = edgeEl.dataset.to;
    const pf = mapPosForNode(from);
    const pt = mapPosForNode(to);
    if (!pf || !pt) return from;
    const df = (pf.x - mx) ** 2 + (pf.y - my) ** 2;
    const dt = (pt.x - mx) ** 2 + (pt.y - my) ** 2;
    return df <= dt ? from : to;
  }

  function mapHoverTargetFromEvent(ev) {
    const svg = document.getElementById("map-svg");
    if (!svg) return null;
    const stack = document.elementsFromPoint(ev.clientX, ev.clientY);
    for (const el of stack) {
      if (!el.closest) continue;
      const node = el.closest(".node");
      if (node && svg.contains(node)) return { kind: "node", id: node.dataset.id };
      const edge = el.closest(".edge");
      if (edge && svg.contains(edge)) return { kind: "edge", el: edge };
    }
    return null;
  }

  function applyMapHoverTarget(target, ev) {
    let nextId = null;
    if (target && target.kind === "node") nextId = target.id;
    else if (target && target.kind === "edge") {
      const pt = mapSvgClientToMap(ev);
      nextId = edgeTraceNodeId(target.el, pt ? pt.x : 0, pt ? pt.y : 0);
    }
    if (nextId === hoverTraceId) return;
    hoverTraceId = nextId;
    updateMapHoverTrace();
  }

  function revealTreeFile(el) {
    if (!el) return;
    let parent = el.parentElement;
    while (parent) {
      if (parent.tagName === "DETAILS") parent.open = true;
      parent = parent.parentElement;
    }
    el.scrollIntoView({ block: "nearest", behavior: "smooth" });
  }

  function focusMapNodeEl(nodeId) {
    if (!nodeId) return;
    const el = document.querySelector(
      '#map-svg .node[data-id="' + nodeId.replace(/"/g, "") + '"]'
    );
    if (el && typeof el.focus === "function") {
      el.focus({ preventScroll: true });
    }
  }

  function focusMapNode(node, opts) {
    const pushHistory = !opts || opts.pushHistory !== false;
    pauseMapOrbitAuto();
    const pos = mapPosForNode(node.id);
    if (!pos) return;
    if (pushHistory) pushMapViewState();
    const n = nodeById(node.id) || node;
    const base = nodePositions.get(node.id);
    const fs = labelFontSize(n, base || pos);
    const svg = document.getElementById("map-svg");
    const rect = svg ? svg.getBoundingClientRect() : { width: MAP_W };
    const pxPerSvgUnit = rect.width > 0 ? rect.width / MAP_W : 1;
    const targetScale = Math.min(
      MAP_MAX_SCALE,
      Math.max(MAP_MIN_SCALE, MAP_FOCUS_LABEL_PX / (fs * pxPerSvgUnit))
    );
    mapView.scale = targetScale;
    mapView.tx = MAP_W / 2 - pos.x * mapView.scale;
    mapView.ty = MAP_H / 2 - pos.y * mapView.scale;
    applyMapTransform();
  }

  function bindMapNodeEvents(svg) {
    if (mapNodesBound || !svg) return;
    mapNodesBound = true;
    let hoverNode = null;

    svg.addEventListener("click", (ev) => {
      const edgeEl = ev.target.closest(".edge");
      if (edgeEl && svg.contains(edgeEl)) {
        handleMapEdgeClick(edgeEl, ev);
        return;
      }
      const g = ev.target.closest(".node");
      if (!g || !svg.contains(g)) return;
      ev.stopPropagation();
      const n = nodeById(g.dataset.id);
      if (!n) return;
      const pos = nodePositions.get(n.id);
      if (pos && pos.clusterHub && !ev.shiftKey) {
        toggleCluster(n.layer, pos.cluster);
        return;
      }
      selectNode(n);
      clearTimeout(clickFocusTimer);
      clickFocusTimer = setTimeout(() => {
        clickFocusTimer = null;
        focusMapNode(n, { pushHistory: true });
      }, 220);
    });

    svg.addEventListener("dblclick", (ev) => {
      const g = ev.target.closest(".node");
      if (!g || !svg.contains(g)) return;
      ev.stopPropagation();
      clearTimeout(clickFocusTimer);
      clickFocusTimer = null;
      const n = nodeById(g.dataset.id);
      if (!n) return;
      const pos = nodePositions.get(n.id);
      if (pos && pos.clusterHub) {
        expandedClusters.add(clusterStoreId(n.layer, pos.cluster));
        saveMapPrefs();
        renderMap();
      }
      focusMapNode(n, { pushHistory: false });
      selectNode(n);
    });

    svg.addEventListener("focusin", (ev) => {
      const g = ev.target.closest(".node");
      if (!g || !svg.contains(g)) return;
      const n = nodeById(g.dataset.id);
      if (!n || selectedId === n.id) return;
      selectNode(n);
    });

    svg.addEventListener("keydown", (ev) => {
      const g = ev.target.closest(".node");
      if (!g || !svg.contains(g)) return;
      if (ev.key !== "Enter" && ev.key !== " ") return;
      ev.preventDefault();
      const n = nodeById(g.dataset.id);
      if (!n) return;
      const pos = nodePositions.get(n.id);
      if (pos && pos.clusterHub && !ev.shiftKey) {
        toggleCluster(n.layer, pos.cluster);
        return;
      }
      selectNode(n);
      clearTimeout(clickFocusTimer);
      clickFocusTimer = null;
      focusMapNode(n, { pushHistory: true });
    });

    svg.addEventListener("mousemove", (ev) => {
      if (!isVisionMs()) return;
      if (mapHoverRaf) return;
      mapHoverRaf = requestAnimationFrame(() => {
        mapHoverRaf = 0;
        const target = mapHoverTargetFromEvent(ev);
        applyMapHoverTarget(target, ev);
        const nodeEl =
          target && target.kind === "node"
            ? svg.querySelector('.node[data-id="' + target.id + '"]')
            : null;
        if (nodeEl === hoverNode) return;
        if (hoverNode) hoverNode.classList.remove("label-hover");
        hoverNode = nodeEl;
        if (!hoverNode) {
          scheduleMapLabelZoom();
          return;
        }
        hoverNode.classList.add("label-hover");
        const n = nodeById(hoverNode.dataset.id);
        const pos = nodePositions.get(hoverNode.dataset.id);
        if (!n || !pos) {
          scheduleMapLabelZoom();
          return;
        }
        let t = hoverNode.querySelector("text");
        if (!t && hoverNode._labelEl) {
          hoverNode.appendChild(hoverNode._labelEl);
          t = hoverNode._labelEl;
        }
        if (t) {
          t.style.display = "";
          t.textContent = hoverNode.dataset.fullLabel || n.label;
          t.setAttribute(
            "font-size",
            String(Math.max(labelFontSize(n, pos), 9))
          );
        }
        scheduleMapLabelZoom();
      });
    });

    svg.addEventListener("mouseleave", () => {
      if (hoverNode) hoverNode.classList.remove("label-hover");
      hoverNode = null;
      if (isVisionMs()) {
        hoverTraceId = null;
        updateMapHoverTrace();
      }
      scheduleMapLabelZoom();
    });

    svg.addEventListener("mouseover", (ev) => {
      if (isVisionMs()) return;
      const g = ev.target.closest(".node");
      if (!g || !svg.contains(g) || g === hoverNode) return;
      if (hoverNode) hoverNode.classList.remove("label-hover");
      hoverNode = g;
      g.classList.add("label-hover");
      const n = nodeById(g.dataset.id);
      const pos = nodePositions.get(g.dataset.id);
      if (!n || !pos) return;
      let t = g.querySelector("text");
      if (!t && g._labelEl) {
        g.appendChild(g._labelEl);
        t = g._labelEl;
      }
      if (t) {
        t.style.display = "";
        t.textContent = g.dataset.fullLabel || n.label;
        t.setAttribute(
          "font-size",
          String(Math.max(labelFontSize(n, pos), 9))
        );
      }
    });

    svg.addEventListener("mouseout", (ev) => {
      if (isVisionMs()) return;
      const g = ev.target.closest(".node");
      if (!g || (ev.relatedTarget && g.contains(ev.relatedTarget))) return;
      if (hoverNode === g) hoverNode = null;
      g.classList.remove("label-hover");
      if (isVisionMs() && hoverTraceId === g.dataset.id) {
        hoverTraceId = null;
        updateMapHoverTrace();
      }
      scheduleMapLabelZoom();
    });
  }

  function mapChromeTarget(ev) {
    return !!(
      ev.target &&
      ev.target.closest &&
      (ev.target.closest(".map-controls") ||
        ev.target.closest(".map-bottom-bar") ||
        ev.target.closest(".map-toolbar") ||
        ev.target.closest(".map-filter-bar") ||
        ev.target.closest(".map-minimap") ||
        ev.target.closest(".map-orbit-pad") ||
        ev.target.closest("button"))
    );
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
        pauseMapOrbitAuto();
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
      if (
        ev.button !== 0 ||
        mapChromeTarget(ev) ||
        ev.target.closest(".node") ||
        ev.target.closest(".edge")
      ) {
        return;
      }
      dragging = true;
      pauseMapOrbitAuto();
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
        pauseMapOrbitAuto();
        zoomMapAt(MAP_W / 2, MAP_H / 2, 1.16);
      });
    }
    if (zo) {
      zo.addEventListener("click", (ev) => {
        ev.stopPropagation();
        pauseMapOrbitAuto();
        zoomMapAt(MAP_W / 2, MAP_H / 2, 1 / 1.16);
      });
    }
    if (zr) {
      zr.addEventListener("click", (ev) => {
        ev.stopPropagation();
        mapViewStack.length = 0;
        syncMapZoomBackBtn();
        resetMapView();
        if (!prefersReducedMotion()) startMapOrbitAuto();
      });
    }
    const za = document.getElementById("map-orbit-auto");
    if (za) {
      za.addEventListener("click", (ev) => {
        ev.stopPropagation();
        toggleMapOrbitAuto();
      });
    }
    const zb = document.getElementById("map-zoom-back");
    if (zb) {
      zb.addEventListener("click", (ev) => {
        ev.stopPropagation();
        popMapViewState();
      });
    }
  }

  function extLabel(path) {
    const e = fileExt(path);
    return e ? "." + e : "?";
  }

  function nodeById(id) {
    return nodeIndex.get(id);
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

  function nodeInNextSprint(node) {
    if (!nextSprint || !node) return false;
    return !!(node.sprints && node.sprints.some((t) => sprintTokenMatches(t, nextSprint)));
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

  function nodesForSprint(sprintId) {
    if (!manifest || !sprintId) return [];
    const paths = buildSprintPathSet(extensions, sprintId);
    const out = [];
    manifest.nodes.forEach((n) => {
      if (n.sprints && n.sprints.some((t) => sprintTokenMatches(t, sprintId))) {
        out.push(n);
      } else if (paths.has(n.path)) {
        out.push(n);
      }
    });
    return out;
  }

  function pickMapNodeForSprint(sprintId) {
    const candidates = nodesForSprint(sprintId);
    if (!candidates.length) return null;
    const onMap = candidates.filter((n) => nodePositions.has(n.id));
    const pool = onMap.length ? onMap : candidates;
    const priority = ["galaxy_grid", "fm", "handoff", "next_session"];
    for (let i = 0; i < priority.length; i++) {
      const hit = pool.find((n) => n.id === priority[i]);
      if (hit && nodePositions.has(hit.id)) return hit;
    }
    pool.sort((a, b) => {
      const la = a.layer || "L9";
      const lb = b.layer || "L9";
      if (la !== lb) return la.localeCompare(lb);
      return (a.path || "").localeCompare(b.path || "");
    });
    return pool.find((n) => nodePositions.has(n.id)) || pool[0];
  }

  function ensurePanelExpanded(panelId) {
    if (collapsedPanels.has(panelId)) {
      collapsedPanels.delete(panelId);
      saveMapPrefs();
      syncPanelCollapseLayout();
    }
  }

  function ensureMapPanelVisible() {
    ensurePanelExpanded("map");
  }

  function setActiveQueueSprint(sprintId) {
    activeQueueSprintId = sprintId || null;
    const box = document.getElementById("sprint-queue");
    if (!box) return;
    box.querySelectorAll(".sprint-queue-item").forEach((li) => {
      li.classList.toggle(
        "queue-active",
        li.dataset.sprintId === activeQueueSprintId
      );
    });
  }

  function focusSprintOnMap(sprintId) {
    if (!sprintId) return;
    const node = pickMapNodeForSprint(sprintId);
    setActiveQueueSprint(sprintId);
    if (!node) return;
    ensureMapPanelVisible();
    selectNode(node);
    focusMapNode(node, { pushHistory: true });
  }

  function sprintMapFocusAriaLabel(sprintId) {
    return "Focus sprint " + sprintId + " on documentation map";
  }

  /** PH-S233: shared a11y + keyboard for sprint chips linked to a map node. */
  function bindMapLinkedSprintChip(el, sprintId) {
    if (!el || !sprintId || !pickMapNodeForSprint(sprintId)) return;
    el.classList.add("map-linked");
    el.setAttribute("role", "button");
    el.setAttribute("tabindex", "0");
    el.setAttribute("aria-label", sprintMapFocusAriaLabel(sprintId));
    const activate = (ev) => {
      if (ev) ev.stopPropagation();
      focusSprintOnMap(sprintId);
    };
    el.addEventListener("click", activate);
    el.addEventListener("keydown", (ev) => {
      if (ev.key === "Enter" || ev.key === " ") {
        ev.preventDefault();
        activate(ev);
      }
    });
    el.style.cursor = "pointer";
  }

  function bindSprintQueueItems(box) {
    if (!box) return;
    box.querySelectorAll(".sprint-queue-item.map-linked").forEach((li) => {
      bindMapLinkedSprintChip(li, li.dataset.sprintId || "");
    });
  }

  function syncQueueHighlightFromNode(node) {
    if (!node || !node.sprints || !node.sprints.length) return;
    for (let i = 0; i < node.sprints.length; i++) {
      const s = node.sprints[i];
      const li = document.querySelector(
        '.sprint-queue-item[data-sprint-id="' + s + '"]'
      );
      if (li) {
        setActiveQueueSprint(s);
        return;
      }
    }
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
          div.setAttribute("role", "treeitem");
          div.setAttribute("tabindex", "-1");
          div.setAttribute("aria-label", n.label + " — " + (n.path || n.id));
          if (nodeInActiveSprint(n)) div.classList.add("sprint-scope");
          if (nodeInNextSprint(n)) div.classList.add("sprint-next");
          div.dataset.id = n.id;
          div.dataset.path = n.path;
          div.innerHTML =
            '<span class="ext-dot ' +
            extClass(n.path) +
            '"></span><span>' +
            escapeHtml(n.label) +
            "</span>";
          div.addEventListener("click", () => selectNode(n));
          div.addEventListener("keydown", (ev) => {
            if (ev.key === "Enter" || ev.key === " ") {
              ev.preventDefault();
              selectNode(n);
            }
          });
          container.appendChild(div);
          return;
        }
        const det = document.createElement("details");
        det.className = "tree-folder";
        det.setAttribute("role", "group");
        det.open = depth < 2;
        const sum = document.createElement("summary");
        sum.textContent = key;
        det.appendChild(sum);
        const inner = document.createElement("div");
        inner.setAttribute("role", "group");
        renderTree(val, inner, depth + 1);
        det.appendChild(inner);
        container.appendChild(det);
      });
  }

  function treeFileItems() {
    const tree = document.getElementById("file-tree");
    if (!tree) return [];
    return Array.from(tree.querySelectorAll('.tree-file[role="treeitem"]'));
  }

  function focusTreeFileItem(el) {
    if (!el) return;
    treeFileItems().forEach((item) => item.setAttribute("tabindex", "-1"));
    el.setAttribute("tabindex", "0");
    el.focus();
  }

  function initTreeKeyboardNav() {
    const tree = document.getElementById("file-tree");
    if (!tree || tree.dataset.kbBound) return;
    tree.dataset.kbBound = "1";
    tree.addEventListener("keydown", (ev) => {
      const items = treeFileItems();
      if (!items.length) return;
      const idx = items.indexOf(document.activeElement);
      if (ev.key === "ArrowDown") {
        ev.preventDefault();
        const next = items[Math.min(items.length - 1, idx < 0 ? 0 : idx + 1)];
        focusTreeFileItem(next);
      } else if (ev.key === "ArrowUp") {
        ev.preventDefault();
        const prev = items[Math.max(0, idx <= 0 ? 0 : idx - 1)];
        focusTreeFileItem(prev);
      } else if (ev.key === "Home") {
        ev.preventDefault();
        focusTreeFileItem(items[0]);
      } else if (ev.key === "End") {
        ev.preventDefault();
        focusTreeFileItem(items[items.length - 1]);
      }
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
        el.setAttribute("role", "button");
        el.setAttribute("tabindex", "0");
        el.title = "Click: focus this tier in the layer stack";
        el.setAttribute("aria-label", layer.id + " — " + layer.name);
        el.setAttribute(
          "aria-pressed",
          stackLayerFocus === layer.id ? "true" : "false"
        );
        el.addEventListener("click", () => setStackLayerFocus(layer.id));
        el.addEventListener("keydown", (ev) => {
          if (ev.key === "Enter" || ev.key === " ") {
            ev.preventDefault();
            setStackLayerFocus(layer.id);
          }
        });
        stack.appendChild(el);
      });

    const legend = document.getElementById("layer-legend");
    if (legend) {
      legend.innerHTML = "";
      (m.layers || []).forEach((layer) => {
        const chip = document.createElement("button");
        chip.type = "button";
        chip.className = "legend-chip";
        chip.dataset.layer = layer.id;
        chip.title = "Click: focus this tier in the layer stack";
        const swatch = document.createElement("i");
        swatch.className = "legend-swatch legend-swatch-" + layer.id;
        chip.appendChild(swatch);
        chip.appendChild(
          document.createTextNode(layer.id + " " + layer.name)
        );
        chip.addEventListener("click", () => setStackLayerFocus(layer.id));
        chip.addEventListener("keydown", (ev) => {
          if (ev.key === "Enter" || ev.key === " ") {
            ev.preventDefault();
            setStackLayerFocus(layer.id);
          }
        });
        legend.appendChild(chip);
      });
    }

    syncLayerStackHighlight();
    syncLayerStack3D();
  }

  function syncLayerStackHighlight() {
    document.querySelectorAll(".layer-plane, .legend-chip").forEach((el) => {
      const lid = el.dataset.layer;
      if (!lid) return;
      const on = !stackLayerFocus || lid === stackLayerFocus;
      el.classList.toggle("highlight", on);
      el.classList.toggle("filter-off", !!stackLayerFocus && lid !== stackLayerFocus);
      el.setAttribute("aria-pressed", on ? "true" : "false");
    });
  }

  function setStackLayerFocus(layerId) {
    if (stackLayerFocus === layerId) {
      stackLayerFocus = null;
      mapLayerFocus = null;
    } else {
      stackLayerFocus = layerId;
      mapLayerFocus = layerId;
    }
    syncLayerStackHighlight();
    syncLayerStack3D();
    syncMapFilterDock();
    updateMapFilters();
  }

  function setMapLayerFocus(layerId) {
    if (mapLayerFocus === layerId) {
      mapLayerFocus = null;
      stackLayerFocus = null;
    } else {
      mapLayerFocus = layerId;
      stackLayerFocus = layerId;
    }
    syncMapFilterDock();
    syncLayerStackHighlight();
    syncLayerStack3D();
    updateMapFilters();
  }

  function setMapLayersAll() {
    mapLayerFocus = null;
    enabledLayers = null;
    syncMapFilterDock();
    updateMapFilters();
  }

  function setMapLayersNone() {
    mapLayerFocus = null;
    enabledLayers = new Set();
    syncMapFilterDock();
    updateMapFilters();
  }

  function setMapExtsAll() {
    enabledExts = null;
    syncMapFilterDock();
    updateMapFilters();
  }

  function setMapExtsNone() {
    enabledExts = new Set();
    syncMapFilterDock();
    updateMapFilters();
  }

  function highlightLayer(layerId) {
    stackLayerFocus = layerId || null;
    syncLayerStackHighlight();
  }

  function toggleLayerChip(layerId) {
    mapLayerFocus = null;
    const all = allLayerIds();
    if (!enabledLayers) enabledLayers = new Set(all);
    if (enabledLayers.has(layerId)) enabledLayers.delete(layerId);
    else enabledLayers.add(layerId);
    if (enabledLayers.size === all.length) enabledLayers = null;
    syncMapFilterDock();
    updateMapFilters();
  }

  function toggleExtChip(extId) {
    if (!enabledExts) {
      enabledExts = new Set(MAP_EXT_BUCKETS.map((b) => b.id));
    }
    if (enabledExts.has(extId)) enabledExts.delete(extId);
    else enabledExts.add(extId);
    if (enabledExts.size === MAP_EXT_BUCKETS.length) enabledExts = null;
    syncMapFilterDock();
    updateMapFilters();
  }

  let mapFilterBulkBound = false;

  function bindMapFilterBulkActions() {
    if (mapFilterBulkBound) return;
    mapFilterBulkBound = true;
    const layersAll = document.getElementById("map-layers-all");
    const layersNone = document.getElementById("map-layers-none");
    const extsAll = document.getElementById("map-exts-all");
    const extsNone = document.getElementById("map-exts-none");
    if (layersAll) {
      layersAll.addEventListener("click", (ev) => {
        ev.stopPropagation();
        setMapLayersAll();
      });
    }
    if (layersNone) {
      layersNone.addEventListener("click", (ev) => {
        ev.stopPropagation();
        setMapLayersNone();
      });
    }
    if (extsAll) {
      extsAll.addEventListener("click", (ev) => {
        ev.stopPropagation();
        setMapExtsAll();
      });
    }
    if (extsNone) {
      extsNone.addEventListener("click", (ev) => {
        ev.stopPropagation();
        setMapExtsNone();
      });
    }
  }

  function renderMapFilterDock() {
    bindMapFilterBulkActions();
    const layerHost = document.getElementById("map-layer-chips");
    const extHost = document.getElementById("map-ext-chips");
    if (!layerHost || !extHost || !manifest) return;

    layerHost.innerHTML = "";
    (manifest.layers || []).forEach((layer) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "map-filter-chip layer-chip";
      btn.dataset.layer = layer.id;
      btn.title = "Toggle " + layer.id + " highlight on map";
      const sw = document.createElement("i");
      sw.className = "legend-swatch legend-swatch-" + layer.id;
      btn.appendChild(sw);
      btn.appendChild(document.createTextNode(layer.id));
      btn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        if (ev.shiftKey) {
          setMapLayerFocus(layer.id);
        } else {
          toggleLayerChip(layer.id);
        }
      });
      layerHost.appendChild(btn);
    });

    extHost.innerHTML = "";
    MAP_EXT_BUCKETS.forEach((bucket) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "map-filter-chip ext-chip ext-" + bucket.id;
      btn.dataset.ext = bucket.id;
      btn.title = "Toggle ." + bucket.id + " files on map";
      if (bucket.id !== "other") {
        const dot = document.createElement("i");
        dot.className = "ext-dot " + extClass("x." + bucket.id);
        btn.appendChild(dot);
      }
      btn.appendChild(document.createTextNode(bucket.label));
      btn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        toggleExtChip(bucket.id);
      });
      extHost.appendChild(btn);
    });

    syncMapFilterDock();
  }

  function syncMapFilterDock() {
    document.querySelectorAll("#map-layer-chips .map-filter-chip").forEach((btn) => {
      const on = isLayerHighlighted(btn.dataset.layer);
      btn.classList.toggle("on", on);
      btn.classList.toggle(
        "solo",
        !!mapLayerFocus && btn.dataset.layer === mapLayerFocus
      );
      btn.setAttribute("aria-pressed", on ? "true" : "false");
    });
    document.querySelectorAll("#map-ext-chips .map-filter-chip").forEach((btn) => {
      const on = !enabledExts || enabledExts.has(btn.dataset.ext);
      btn.classList.toggle("on", on);
      btn.setAttribute("aria-pressed", on ? "true" : "false");
    });
    const bar = document.getElementById("map-filter-bar");
    if (bar) {
      const layerActive =
        !!mapLayerFocus ||
        (enabledLayers !== null && enabledLayers.size < allLayerIds().length);
      const extActive =
        enabledExts !== null && enabledExts.size < MAP_EXT_BUCKETS.length;
      bar.classList.toggle("filters-active-layers", layerActive);
      bar.classList.toggle("filters-active-exts", extActive);
    }
    document.querySelectorAll(".map-filter-drop-btn").forEach((btn) => {
      const drop = btn.dataset.drop;
      let active = false;
      if (drop === "layers") {
        active =
          !!mapLayerFocus ||
          (enabledLayers !== null && enabledLayers.size < allLayerIds().length);
      } else if (drop === "exts") {
        active =
          enabledExts !== null && enabledExts.size < MAP_EXT_BUCKETS.length;
      }
      btn.classList.toggle("filters-active", active);
    });
  }

  function closeMapFilterDrops(except) {
    document.querySelectorAll(".map-filter-drop-menu").forEach((menu) => {
      if (except && menu.id === except) return;
      menu.hidden = true;
    });
    document.querySelectorAll(".map-filter-drop-btn").forEach((btn) => {
      if (except && btn.getAttribute("aria-controls") === except) return;
      btn.setAttribute("aria-expanded", "false");
    });
    if (!except) openFilterDrop = null;
  }

  function toggleMapFilterDrop(menuId) {
    const menu = document.getElementById(menuId);
    const btn = document.querySelector(
      '.map-filter-drop-btn[aria-controls="' + menuId + '"]'
    );
    if (!menu || !btn) return;
    const open = menu.hidden;
    closeMapFilterDrops(open ? menuId : null);
    menu.hidden = !open;
    btn.setAttribute("aria-expanded", open ? "true" : "false");
    openFilterDrop = open ? menuId : null;
  }

  let mapFilterDropBound = false;

  function initMapFilterDropdowns() {
    if (mapFilterDropBound) return;
    mapFilterDropBound = true;
    const layersBtn = document.getElementById("map-filter-layers-btn");
    const extsBtn = document.getElementById("map-filter-exts-btn");
    if (layersBtn) {
      layersBtn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        toggleMapFilterDrop("map-filter-layers-menu");
      });
    }
    if (extsBtn) {
      extsBtn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        toggleMapFilterDrop("map-filter-exts-menu");
      });
    }
    document.addEventListener("click", (ev) => {
      if (!ev.target.closest(".map-filter-drop")) {
        closeMapFilterDrops();
      }
    });
    document.addEventListener("keydown", (ev) => {
      if (ev.key === "Escape" && openFilterDrop) {
        closeMapFilterDrops();
      }
    });
  }

  function rememberPanelAnchor(panel) {
    const id = panel.dataset.panel;
    if (!id || panelAnchors.has(id)) return;
    const dock = document.getElementById("panel-dock");
    if (dock && panel.parentElement === dock) return;
    panelAnchors.set(id, { parent: panel.parentElement });
  }

  function restorePanelFromDock(panel) {
    const id = panel.dataset.panel;
    const anchor = panelAnchors.get(id);
    if (!anchor || !anchor.parent) return;
    if (panel.parentElement === anchor.parent) return;

    if (id === "preview") {
      const row = document.querySelector(".diagram-row");
      const after = row ? row.nextElementSibling : null;
      if (after !== panel) anchor.parent.insertBefore(panel, after);
      return;
    }

    const parent = anchor.parent;
    const myIdx = DIAGRAM_PANELS.indexOf(id);
    let insertBefore = null;
    for (let i = myIdx + 1; i < DIAGRAM_PANELS.length; i++) {
      const sib = parent.querySelector('.panel[data-panel="' + DIAGRAM_PANELS[i] + '"]');
      if (sib) {
        insertBefore = sib;
        break;
      }
    }
    parent.insertBefore(panel, insertBefore);
  }

  function columnSpecForPanel(id, expanded) {
    if (expanded.length === 1) return "minmax(0, 1fr)";
    if (id === "map") return "minmax(0, 2.2fr)";
    if (id === "queue") return "minmax(120px, 18%)";
    if (id === "layers") return "minmax(100px, 16%)";
    return "minmax(130px, 20%)";
  }

  function syncPanelCollapseLayout() {
    const dock = document.getElementById("panel-dock");
    const row = document.querySelector(".diagram-row");
    const mainCol = document.querySelector(".main-col");

    ALL_PANELS.forEach((id) => {
      const panel = document.querySelector('.panel[data-panel="' + id + '"]');
      if (!panel) return;
      const collapsed = collapsedPanels.has(id);
      panel.classList.toggle("panel-collapsed", collapsed);
      panel.classList.toggle("panel-docked", collapsed);
      panel.querySelectorAll(".btn-panel-collapse").forEach((btn) => {
        btn.textContent = collapsed ? "+" : "−";
        btn.setAttribute("aria-expanded", collapsed ? "false" : "true");
        btn.title = collapsed
          ? "Restore " + (panel.querySelector("h2 > span")?.textContent || id)
          : "Collapse to dock bar";
      });
      const titleSpan = panel.querySelector("h2 > span[data-short]");
      if (titleSpan) {
        if (!titleSpan.dataset.fullTitle) {
          titleSpan.dataset.fullTitle = titleSpan.textContent;
        }
        titleSpan.textContent = collapsed
          ? titleSpan.dataset.short
          : titleSpan.dataset.fullTitle;
      }

      if (collapsed && dock) {
        rememberPanelAnchor(panel);
        if (panel.parentElement !== dock) dock.appendChild(panel);
      } else {
        restorePanelFromDock(panel);
      }
    });

    if (dock) {
      dock.hidden = dock.childElementCount === 0;
    }
    if (mainCol) {
      mainCol.classList.toggle("has-panel-dock", dock && !dock.hidden);
      mainCol.classList.toggle("preview-collapsed", collapsedPanels.has("preview"));
    }

    if (row) {
      const expanded = DIAGRAM_PANELS.filter((id) => !collapsedPanels.has(id));
      if (expanded.length) {
        row.style.gridTemplateColumns = expanded
          .map((id) => columnSpecForPanel(id, expanded))
          .join(" ");
      } else {
        row.style.gridTemplateColumns = "1fr";
      }
    }

    window.dispatchEvent(new Event("resize"));
  }

  function focusPanelToggle(panelId) {
    const panel = document.querySelector('.panel[data-panel="' + panelId + '"]');
    if (!panel) return;
    const btn = panel.querySelector(".btn-panel-collapse");
    if (btn && typeof btn.focus === "function") {
      btn.focus({ preventScroll: true });
    }
  }

  function togglePanelCollapse(panelId) {
    if (fullscreenPanel) exitPanelFullscreen();
    const willCollapse = !collapsedPanels.has(panelId);
    if (collapsedPanels.has(panelId)) collapsedPanels.delete(panelId);
    else collapsedPanels.add(panelId);
    saveMapPrefs();
    syncPanelCollapseLayout();
    if (willCollapse) {
      requestAnimationFrame(() => focusPanelToggle(panelId));
    }
  }

  function initPanelCollapse() {
    document.querySelectorAll(".panel[data-panel]").forEach(rememberPanelAnchor);
    document.querySelectorAll(".btn-panel-collapse").forEach((btn) => {
      btn.addEventListener("click", (ev) => {
        ev.stopPropagation();
        const panel = btn.closest(".panel");
        if (!panel || !panel.dataset.panel) return;
        togglePanelCollapse(panel.dataset.panel);
      });
    });
    const dock = document.getElementById("panel-dock");
    if (dock) {
      dock.addEventListener("click", (ev) => {
        const panel = ev.target.closest(".panel.panel-docked");
        if (!panel || !panel.dataset.panel) return;
        if (ev.target.closest(".btn-panel-collapse")) return;
        togglePanelCollapse(panel.dataset.panel);
      });
    }
    syncPanelCollapseLayout();
  }

  function updateMapFilters() {
    const solo = mapLayerFocus;
    const filterLayers = !solo && enabledLayers;
    const filterExts = enabledExts;
    const svg = document.getElementById("map-svg");
    if (!svg) return;
    svg.classList.toggle("has-layer-focus", !!(solo || filterLayers || filterExts));

    document
      .querySelectorAll("#map-svg .plane, #map-svg .layer-tier-label")
      .forEach((el) => {
        const lid = el.dataset.layer;
        if (!lid) return;
        const on = isLayerHighlighted(lid);
        el.classList.toggle("layer-focus", !!solo && lid === solo);
        el.classList.toggle("layer-dim", !on);
      });

    document.querySelectorAll("#map-svg .cluster-label").forEach((el) => {
      const lid = el.dataset.layer;
      const on = lid ? isLayerHighlighted(lid) : true;
      el.classList.toggle("layer-dim", !on);
    });

    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const n = nodeById(el.dataset.id);
      if (!n) return;
      const layerOn = isLayerHighlighted(n.layer);
      const extOn = isExtHighlighted(n.path);
      const on = layerOn && extOn;
      el.classList.toggle("layer-dim", !on);
      el.classList.toggle("ext-dim", !extOn);
      el.classList.toggle(
        "layer-focus",
        !!solo && n.layer === solo
      );
    });

    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const a = nodeById(el.dataset.from);
      const b = nodeById(el.dataset.to);
      const on =
        a &&
        b &&
        isLayerHighlighted(a.layer) &&
        isLayerHighlighted(b.layer) &&
        isExtHighlighted(a.path) &&
        isExtHighlighted(b.path);
      el.classList.toggle("layer-dim", !on);
    });
  }

  /** @deprecated alias */
  function updateMapLayerFocus() {
    updateMapFilters();
  }

  function placeConstellationNode(n, cx, cy, i, layer, cluster, opts) {
    const count = (opts && opts.count) || 1;
    if (i === 0) {
      nodePositions.set(n.id, {
        x: cx,
        y: cy,
        layer,
        cluster,
        clusterHub: !!(opts && opts.hub),
        clusterCount: count,
        mass: count,
      });
      return;
    }
    const orbitIndex = i - 1;
    const ring = Math.floor(Math.sqrt(orbitIndex + 0.5));
    const ringStep = 14 + Math.sqrt(count) * 2.4;
    const ringRadius = 20 + ring * ringStep;
    const perRing = Math.max(4, Math.ceil(Math.sqrt(count + orbitIndex + 2)));
    const angle =
      (orbitIndex / perRing) * Math.PI * 2 +
      ring * GOLDEN_ANGLE +
      hashUnit(n.id) * 0.55;
    const flatten = 0.68 + Math.min(0.12, count * 0.004);
    const wobble = (hashUnit(n.id + "y") - 0.5) * 10;
    const deg = nodeDegree(n.id);
    const massBias = 1 - Math.min(0.35, deg * 0.04);
    const x = cx + Math.cos(angle) * ringRadius * massBias;
    const y = cy + Math.sin(angle) * ringRadius * flatten + wobble;
    nodePositions.set(n.id, {
      x,
      y,
      layer,
      cluster,
      constellation: true,
      satelliteRing: ring,
      mass: Math.max(1, deg),
    });
  }

  function layoutOrphanStars(orphans, baseY, layer) {
    if (!orphans.length) return;
    const rimY = baseY + 38;
    orphans.forEach((meta, oi) => {
      const n = meta.nodes[0];
      const t = orphans.length === 1 ? 0.5 : oi / Math.max(1, orphans.length - 1);
      const edgeX = marginXForOrbit(t);
      const cx = edgeX;
      const cy = rimY + (hashUnit(meta.key + "y") - 0.5) * 22;
      nodePositions.set(n.id, {
        x: cx,
        y: cy,
        layer,
        cluster: meta.key,
        orphan: true,
        clusterCount: 1,
        mass: 1,
      });
    });
  }

  function marginXForOrbit(t) {
    const margin = 56;
    const usable = MAP_W - margin * 2;
    return margin + t * usable;
  }

  function solarSystemFootprint(mass) {
    return 36 + Math.sqrt(mass) * 22;
  }

  function layoutLayerConstellation(list, baseY, layer) {
    const clusters = new Map();
    list.forEach((n) => {
      const key = folderCluster(n.path);
      if (!clusters.has(key)) clusters.set(key, []);
      clusters.get(key).push(n);
    });

    const clusterMeta = Array.from(clusters.entries())
      .map(([key, nodes]) => ({
        key,
        nodes: nodes.slice().sort((a, b) => a.label.localeCompare(b.label)),
        mass: nodes.length,
      }))
      .sort((a, b) => b.mass - a.mass);

    const solarSystems = clusterMeta.filter((c) => c.mass > 1);
    const orphans = clusterMeta.filter((c) => c.mass === 1);
    layoutOrphanStars(orphans, baseY, layer);

    const nSystems = solarSystems.length;
    if (!nSystems) return;

    const totalFootprint = solarSystems.reduce(
      (sum, c) => sum + solarSystemFootprint(c.mass),
      0
    );
    const arcSpan = Math.min(
      Math.PI * 1.42,
      Math.max(0.65, nSystems * 0.38 + totalFootprint / MAP_W)
    );
    const xSpread = MAP_W * (0.28 + Math.min(0.28, nSystems * 0.008));

    let massCursor = 0;
    const massTotal = solarSystems.reduce((s, c) => s + c.mass, 0) || 1;

    solarSystems.forEach((meta) => {
      const { key, nodes, mass } = meta;
      const collapsed = isClusterCollapsed(layer, key, nodes.length);
      const slot = (massCursor + mass * 0.5) / massTotal;
      massCursor += mass;
      const t = nSystems === 1 ? 0.5 : slot;
      const angle = -arcSpan / 2 + t * arcSpan;
      const cx = MAP_W / 2 + Math.sin(angle) * xSpread;
      const cy =
        baseY +
        Math.cos(angle) * (28 + nSystems * 0.5) -
        10 +
        (hashUnit(key) - 0.5) * 14;

      if (collapsed) {
        const hub = pickClusterHub(nodes);
        const hubId = hub.id;
        nodes.forEach((n) => {
          if (n.id === hubId) {
            nodePositions.set(n.id, {
              x: cx,
              y: cy,
              layer,
              cluster: key,
              clusterHub: true,
              clusterCount: nodes.length,
              mass: nodes.length,
            });
          } else {
            nodePositions.set(n.id, {
              x: cx,
              y: cy,
              layer,
              cluster: key,
              collapsedHidden: true,
              hubId,
            });
          }
        });
        clusterLabelPositions.push({
          x: cx,
          y: cy - 26 - Math.sqrt(mass) * 2,
          key,
          layer,
          collapsed: true,
          count: nodes.length,
        });
        return;
      }

      const hub = pickClusterHub(nodes);
      const satellites = nodes
        .filter((n) => n.id !== hub.id)
        .sort((a, b) => nodeDegree(b.id) - nodeDegree(a.id));
      const ordered = [hub].concat(satellites);
      ordered.forEach((n, i) => {
        placeConstellationNode(n, cx, cy, i, layer, key, {
          hub: i === 0 && nodes.length >= collapseThresholdForLayer(layer),
          count: nodes.length,
        });
      });
      if (nodes.length > 1) {
        clusterLabelPositions.push({
          x: cx,
          y: cy - 24 - Math.sqrt(mass) * 10 - Math.min(36, mass * 1.5),
          key,
          layer,
          collapsed: false,
          count: nodes.length,
        });
      }
    });
  }

  function nudgeSolarSystemsApart() {
    const hubs = [];
    nodePositions.forEach((p, id) => {
      if (!p.clusterHub || p.collapsedHidden) return;
      hubs.push({ id, p, footprint: solarSystemFootprint(p.clusterCount || 1) });
    });
    for (let pass = 0; pass < 6; pass++) {
      for (let i = 0; i < hubs.length; i++) {
        for (let j = i + 1; j < hubs.length; j++) {
          const a = hubs[i];
          const b = hubs[j];
          if (a.p.layer !== b.p.layer) continue;
          const dx = b.p.x - a.p.x;
          const dy = b.p.y - a.p.y;
          const d = Math.hypot(dx, dy) || 1;
          const minD = a.footprint + b.footprint + 24;
          if (d >= minD) continue;
          const push = (minD - d) * 0.42;
          const ax = -(dx / d) * push;
          const ay = -(dy / d) * push;
          const bx = (dx / d) * push;
          const by = (dy / d) * push;
          shiftClusterByDelta(a.p.cluster, a.p.layer, ax, ay);
          shiftClusterByDelta(b.p.cluster, b.p.layer, bx, by);
        }
      }
    }
  }

  function shiftClusterByDelta(cluster, layer, dx, dy) {
    if (!dx && !dy) return;
    nodePositions.forEach((p) => {
      if (p.cluster !== cluster || p.layer !== layer) return;
      if (p.collapsedHidden) return;
      p.x += dx;
      p.y += dy;
    });
    clusterLabelPositions.forEach((cl) => {
      if (cl.key === cluster && cl.layer === layer) {
        cl.x += dx;
        cl.y += dy;
      }
    });
  }

  function nudgeConstellationApart() {
    const ids = Array.from(nodePositions.keys());
    const dense = manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD;
    const minDot = dense ? 20 : 28;
    const minHub = dense ? 36 : 44;
    const passes = dense ? (isLowGpuMode() ? 4 : 8) : 8;
    for (let pass = 0; pass < passes; pass++) {
      for (let i = 0; i < ids.length; i++) {
        for (let j = i + 1; j < ids.length; j++) {
          const pi = nodePositions.get(ids[i]);
          const pj = nodePositions.get(ids[j]);
          if (!pi || !pj || pi.collapsedHidden || pj.collapsedHidden) continue;
          if (pi.layer !== pj.layer) continue;
          const ni = nodeById(ids[i]);
          const nj = nodeById(ids[j]);
          const ri = nodeRenderRadius(ni, pi);
          const rj = nodeRenderRadius(nj, pj);
          const minD =
            pi.clusterHub || pj.clusterHub
              ? minHub
              : Math.max(minDot, ri + rj + (dense ? 10 : 14));
          const dx = pj.x - pi.x;
          const dy = pj.y - pi.y;
          const d = Math.hypot(dx, dy);
          if (d >= minD || d < 0.5) continue;
          const push = (minD - d) * (dense ? 0.55 : 0.48);
          pi.x -= (dx / d) * push;
          pi.y -= (dy / d) * push;
          pj.x += (dx / d) * push;
          pj.y += (dy / d) * push;
        }
      }
    }
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
      layoutLayerConstellation(byLayer[layer], LAYER_Y[layer] || 200, layer);
    });
    nudgeSolarSystemsApart();
    nudgeConstellationApart();
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
        <stop offset="0%" stop-color="#2d4a6f" stop-opacity="0.25"/>
        <stop offset="100%" stop-color="#1a2840" stop-opacity="0.125"/>
      </linearGradient>
      <linearGradient id="gradL1" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#3d5a40" stop-opacity="0.225"/>
        <stop offset="100%" stop-color="#243828" stop-opacity="0.1"/>
      </linearGradient>
      <linearGradient id="gradL2" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#5a4a2d" stop-opacity="0.2"/>
        <stop offset="100%" stop-color="#302818" stop-opacity="0.09"/>
      </linearGradient>
      <linearGradient id="gradL3" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#5a2d4a" stop-opacity="0.2"/>
        <stop offset="100%" stop-color="#301820" stop-opacity="0.09"/>
      </linearGradient>
      <linearGradient id="gradL4" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#4a3d5a" stop-opacity="0.21"/>
        <stop offset="100%" stop-color="#281830" stop-opacity="0.09"/>
      </linearGradient>
      <linearGradient id="gradL5" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#2d4a5a" stop-opacity="0.225"/>
        <stop offset="100%" stop-color="#182830" stop-opacity="0.1"/>
      </linearGradient>
      ${
        isLowGpuMode()
          ? ""
          : `<filter id="nodeGlow" x="-50%" y="-50%" width="200%" height="200%">
        <feGaussianBlur stdDeviation="2" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>
      <filter id="nodeGlowStrong" x="-80%" y="-80%" width="260%" height="260%">
        <feGaussianBlur stdDeviation="4" result="b"/>
        <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
      </filter>`
      }
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
      path.dataset.layer = layer.id;
      planes.appendChild(path);
      const label = document.createElementNS(ns, "text");
      label.setAttribute("class", "layer-tier-label");
      label.dataset.layer = layer.id;
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
      path.setAttribute("d", buildConstellationEdgePath(a, b));
      let edgeClass = "edge edge-" + routeKind;
      if (mapNodeDimmed(nodeA) && mapNodeDimmed(nodeB)) {
        edgeClass += " sprint-dim";
      }
      if (mapEdgeSparse(e)) {
        edgeClass += " edge-sparse";
      }
      path.setAttribute("class", edgeClass);
      path.dataset.from = e.from;
      path.dataset.to = e.to;
      path.dataset.route = routeKind;
      path.dataset.edgeKey = edgeKey(e.from, e.to);
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
      t.dataset.layer = cl.layer;
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
      if (nodeIsAutoSynced(n)) g.classList.add("auto-leaf");
      if (mapNodeDimmed(n)) g.classList.add("sprint-dim");
      if (nodeInNextSprint(n)) g.classList.add("sprint-next");
      g.dataset.id = n.id;
      if (pos.cluster) g.dataset.cluster = pos.cluster;
      const r = nodeRenderRadius(n, pos);
      const circle = document.createElementNS(ns, "circle");
      circle.setAttribute("cx", pos.x);
      circle.setAttribute("cy", pos.y);
      circle.setAttribute("r", r);
      const fill = LAYER_COLORS[n.layer] || "#555";
      circle.setAttribute("fill", fill);
      if (nodeIsAutoSynced(n) && !pos.clusterHub) {
        circle.setAttribute("opacity", String(0.45 + nodeVisualWeight(n) * 0.4));
      }
      const shortLabel = mapNodeShortLabel(n, pos);
      const fullLabel = mapNodeFullLabel(n, pos);
      g.dataset.shortLabel = shortLabel;
      g.dataset.fullLabel = fullLabel;
      g.setAttribute("role", "button");
      g.setAttribute("tabindex", selectedId === n.id ? "0" : "-1");
      g.setAttribute("aria-label", fullLabel);
      const text = document.createElementNS(ns, "text");
      text.setAttribute("x", pos.x);
      text.setAttribute("y", pos.y + r + (r <= 5 ? 8 : 11));
      text.textContent = shortLabel;
      text.setAttribute("font-size", String(labelFontSize(n, pos)));
      if (pos.clusterHub) {
        text.setAttribute("font-weight", "700");
      }
      g.appendChild(circle);
      if (shouldShowNodeLabel(n, pos)) {
        g.appendChild(text);
      } else {
        g._labelEl = text;
      }
      if (pos.clusterHub) {
        g.setAttribute(
          "title",
          "Click: expand folder · Shift+click: open file"
        );
        g.setAttribute(
          "aria-label",
          fullLabel + " — folder; Enter expand, Shift+Enter open"
        );
      }
      nodesG.appendChild(g);
    });
    world.appendChild(nodesG);
    svg.appendChild(world);
    svg.classList.toggle(
      "map-dense",
      manifest.nodes.length >= MAP_DENSE_NODE_THRESHOLD
    );
    svg.classList.toggle("map-fx-off", isMapFxOff());

    applyMapTransform();
    applyMap3DProjection();
    bindMapNavigation(svg);
    bindMapNodeEvents(svg);
    renderMinimap();
    updateMapSelection();
    updateMapLayerFocus();
    if (!mapInitialFitDone) {
      mapInitialFitDone = true;
      fitMapToContent();
      ensureMapOrbitAutoAfterFit();
    }
  }

  function syncMapSelectionCallout() {
    const world = document.querySelector("#map-svg #map-world");
    if (!world) return;
    const prev = world.querySelector("#map-selection-callout");
    if (prev) prev.remove();
    if (!selectedId) return;

    const n = nodeById(selectedId);
    const base = nodePositions.get(selectedId);
    const pos = mapPosForNode(selectedId);
    if (!n || !base || !pos) return;

    const ns = "http://www.w3.org/2000/svg";
    const hub = !!base.clusterHub;
    const r = nodeBaseRadius(selectedId, hub) + 4;
    const caption = mapNodeFullLabel(n, base);
    const fontSize = 11;
    const padX = 8;
    const padY = 5;
    const estW = Math.min(MAP_W - 48, Math.max(72, caption.length * (fontSize * 0.58)));
    const estH = fontSize + padY * 2;
    const cx = Math.min(MAP_W - 24 - estW / 2, Math.max(24 + estW / 2, pos.x));
    const cy = pos.y - r - 10 - estH / 2;

    const g = document.createElementNS(ns, "g");
    g.setAttribute("id", "map-selection-callout");
    g.setAttribute("class", "map-selection-callout");
    g.setAttribute("pointer-events", "none");

    const rect = document.createElementNS(ns, "rect");
    rect.setAttribute("x", cx - estW / 2);
    rect.setAttribute("y", cy - estH / 2);
    rect.setAttribute("width", estW);
    rect.setAttribute("height", estH);
    rect.setAttribute("rx", 4);

    const text = document.createElementNS(ns, "text");
    text.setAttribute("x", cx);
    text.setAttribute("y", cy + fontSize * 0.35);
    text.setAttribute("text-anchor", "middle");
    text.setAttribute("font-size", String(fontSize));
    text.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
    text.setAttribute("font-weight", "700");
    text.textContent = caption;

    g.appendChild(rect);
    g.appendChild(text);
    world.appendChild(g);
  }

  function updateMapSelection() {
    const hasSel = !!selectedId;
    const constellation = hasSel
      ? computeConstellationHighlight(selectedId)
      : null;
    document.querySelectorAll("#map-svg .node").forEach((el) => {
      const id = el.dataset.id;
      const isSel = id === selectedId;
      const onPipe =
        constellation && constellation.nodes.has(id);
      el.classList.toggle("selected", isSel);
      el.classList.toggle("constellation-lit", !!onPipe && !isSel);
      el.classList.toggle(
        "dim",
        hasSel &&
          !isSel &&
          !onPipe &&
          !el.classList.contains("sprint-dim")
      );
      el.setAttribute("tabindex", isSel ? "0" : "-1");
      const text = el.querySelector("text");
      if (text) {
        const shortL = el.dataset.shortLabel || "";
        const fullL = el.dataset.fullLabel || shortL;
        text.textContent = isSel ? fullL : shortL;
        text.classList.toggle("label-full", isSel);
        if (isSel) {
          const pos = nodePositions.get(id);
          const hub = el.classList.contains("cluster-hub");
          const r = nodeBaseRadius(id, hub) + 4;
          const ty = pos ? pos.y - r - 6 : parseFloat(text.getAttribute("y"));
          text.setAttribute("y", ty);
          text.setAttribute("opacity", "0");
        } else {
          const pos = nodePositions.get(id);
          const hub = el.classList.contains("cluster-hub");
          const r = nodeBaseRadius(id, hub);
          if (pos) text.setAttribute("y", pos.y + r + 12);
          text.removeAttribute("opacity");
        }
      }
      const c = el.querySelector("circle");
      const hub = el.classList.contains("cluster-hub");
      const baseR = nodeBaseRadius(id, hub);
      if (c) c.setAttribute("r", isSel ? baseR + 4 : baseR);
    });
    document.querySelectorAll("#map-svg .cluster-label").forEach((el) => {
      el.classList.toggle("selection-dim", hasSel);
    });
    document.querySelectorAll("#map-svg .edge").forEach((el) => {
      const from = el.dataset.from;
      const to = el.dataset.to;
      const key = el.dataset.edgeKey || edgeKey(from, to);
      const onPipe = constellation && constellation.edges.has(key);
      const hi = selectedId && (from === selectedId || to === selectedId);
      el.classList.toggle("highlight", hi && !onPipe);
      el.classList.toggle("constellation-pipe", !!onPipe);
      el.classList.toggle("edge-reveal", !!(hi || onPipe));
      el.classList.toggle(
        "selection-dim",
        hasSel && !onPipe && !hi
      );
    });
    syncMapSelectionCallout();
    if (edgeSelectTraceKey) updateMapEdgeSelectTrace();
    else updateMapHoverTrace();
    updateMinimapSelection();
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

  function isNodeOnMap(nodeId) {
    const pos = nodePositions.get(nodeId);
    return !!(pos && !pos.collapsedHidden);
  }

  function linkedMapNeighbors(nodeId) {
    const seen = new Set();
    const out = [];
    relatedEdges(nodeId).forEach((l) => {
      const id = l.node.id;
      if (seen.has(id) || !isNodeOnMap(id)) return;
      seen.add(id);
      out.push(l.node);
    });
    const pos = nodePositions.get(nodeId);
    if (pos && out.length > 1) {
      out.sort((a, b) => {
        const pa = nodePositions.get(a.id);
        const pb = nodePositions.get(b.id);
        return (
          Math.atan2(pa.y - pos.y, pa.x - pos.x) -
          Math.atan2(pb.y - pos.y, pb.x - pos.x)
        );
      });
    }
    return out;
  }

  function shouldHandleMapNavKey(ev) {
    const key = ev.key;
    if (
      key !== "ArrowLeft" &&
      key !== "ArrowRight" &&
      key !== "ArrowUp" &&
      key !== "ArrowDown"
    ) {
      return false;
    }
    const t = ev.target;
    if (!t) return true;
    const tag = (t.tagName || "").toLowerCase();
    if (tag === "input" || tag === "textarea" || tag === "select") return false;
    if (t.isContentEditable) return false;
    if (t.closest && t.closest(".sprint-queue-item[tabindex]")) return false;
    return true;
  }

  function navigateMapLinkedNeighbor(ev) {
    if (!shouldHandleMapNavKey(ev)) return;
    if (!selectedId) return;
    const neighbors = linkedMapNeighbors(selectedId);
    if (!neighbors.length) return;
    if (mapNavNeighborKey !== selectedId) {
      mapNavNeighborKey = selectedId;
      mapNavNeighborIdx = -1;
    }
    const step =
      ev.key === "ArrowLeft" || ev.key === "ArrowUp" ? -1 : 1;
    mapNavNeighborIdx =
      (mapNavNeighborIdx + step + neighbors.length) % neighbors.length;
    const next = neighbors[mapNavNeighborIdx];
    ev.preventDefault();
    selectNode(next);
    focusMapNode(next, { pushHistory: true });
    focusMapNodeEl(next.id);
  }

  function initMapKeyboardNav() {
    document.addEventListener("keydown", navigateMapLinkedNeighbor);
  }

  function renderLinkGraph(node) {
    const svg = document.getElementById("link-graph");
    const ns = "http://www.w3.org/2000/svg";
    const prevFocusId = document.activeElement?.dataset?.neighborId || null;
    while (svg.firstChild) svg.removeChild(svg.firstChild);

    const w = svg.clientWidth || 220;
    const h = svg.clientHeight || 180;
    svg.setAttribute("viewBox", "0 0 " + w + " " + h);
    svg.setAttribute("role", "group");
    svg.setAttribute("aria-label", "Neighbour graph for " + node.label);

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
      dot.setAttribute("class", "link-neighbor");
      dot.setAttribute("role", "button");
      dot.setAttribute("tabindex", "0");
      dot.dataset.neighborId = l.node.id;
      const lbl =
        l.node.label.length > 14
          ? l.node.label.slice(0, 12) + "…"
          : l.node.label;
      dot.setAttribute("aria-label", "Open neighbour " + l.node.label);
      dot.style.cursor = "pointer";
      dot.addEventListener("click", () => selectNode(l.node));
      dot.addEventListener("keydown", (ev) => {
        if (ev.key === "Enter" || ev.key === " ") {
          ev.preventDefault();
          selectNode(l.node);
        }
      });
      svg.appendChild(dot);
      const t = document.createElementNS(ns, "text");
      t.setAttribute("x", x2);
      t.setAttribute("y", y2 + 18);
      t.setAttribute("text-anchor", "middle");
      t.setAttribute("fill", "#9aa0a6");
      t.setAttribute("font-size", "8");
      t.setAttribute("font-family", "Segoe UI, system-ui, sans-serif");
      if (links.length <= 5) {
        t.textContent = lbl;
        svg.appendChild(t);
      } else {
        dot.setAttribute("title", lbl);
      }
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

    if (prevFocusId) {
      const restore = svg.querySelector('[data-neighbor-id="' + prevFocusId + '"]');
      if (restore) restore.focus();
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

  /** Sprint queue eye filter: `open` | `open_queued` (localStorage). */
  const QUEUE_FILTER_KEY = "poolai-vision-queue-filter";
  let queueFilterMode = "open";
  try {
    const stored = localStorage.getItem(QUEUE_FILTER_KEY);
    if (stored === "open" || stored === "open_queued") queueFilterMode = stored;
  } catch (_) {
    /* ignore */
  }

  function queueFilterLabel(mode) {
    return mode === "open_queued" ? "open + queued" : "open only";
  }

  function syncQueueEyeButton() {
    const btn = document.getElementById("btn-queue-eye");
    if (!btn) return;
    const label = queueFilterLabel(queueFilterMode);
    btn.setAttribute("aria-label", "Sprint queue filter: " + label);
    btn.title =
      queueFilterMode === "open"
        ? "Filter: open only — click for open + queued"
        : "Filter: open + queued — click for open only";
    btn.setAttribute("aria-pressed", queueFilterMode === "open" ? "true" : "false");
    btn.classList.toggle("filter-queued", queueFilterMode === "open_queued");
  }

  function bindQueueEyeButton() {
    const btn = document.getElementById("btn-queue-eye");
    if (!btn || btn.dataset.bound === "1") return;
    btn.dataset.bound = "1";
    btn.addEventListener("click", (ev) => {
      ev.preventDefault();
      ev.stopPropagation();
      queueFilterMode = queueFilterMode === "open" ? "open_queued" : "open";
      try {
        localStorage.setItem(QUEUE_FILTER_KEY, queueFilterMode);
      } catch (_) {
        /* ignore */
      }
      syncQueueEyeButton();
      if (manifest) renderSprintQueue(manifest);
    });
    syncQueueEyeButton();
  }

  function renderSprintQueue(manifest) {
    const box = document.getElementById("sprint-queue");
    if (!box) return;
    bindQueueEyeButton();
    const hasQueueField = manifest && Array.isArray(manifest.sprint_queue);
    const queue = hasQueueField ? manifest.sprint_queue : [];
    // Missing sprint_queue → sync never ran / stale client. Empty [] after drain is valid.
    if (!hasQueueField) {
      box.innerHTML =
        '<p class="sprint-queue-empty" style="color:var(--muted);margin:0">Run <code>poolai-vision-sync</code> to load FM §5.12.</p>';
      return;
    }
    if (!queue.length) {
      const last = manifest.last_sprint_closed || "—";
      const next = manifest.next_sprint || "—";
      box.innerHTML =
        '<div class="sprint-queue-meta"><span><strong>0</strong> open</span>' +
        '<span>last <strong>' +
        escapeHtml(String(last)) +
        "</strong></span>" +
        "<span>→ <strong>" +
        escapeHtml(String(next)) +
        "</strong></span></div>" +
        '<p class="sprint-queue-empty" style="color:var(--muted);margin:0.5rem 0 0">No open sprints in FM §5.12 (drained). Next session: <code>абракадабра</code>.</p>';
      return;
    }
    const openCount =
      manifest.sprint_queue_open_count != null
        ? manifest.sprint_queue_open_count
        : queue.filter((e) => e.open).length;
    const nextId = manifest.next_sprint || null;
    const openOnly = queue.filter((e) => e.open);
    const queuedOnly = queue.filter(
      (e) =>
        !e.open &&
        (e.status === "queued" ||
          e.status === "closed" ||
          e.status === "in_queue" ||
          !e.status)
    );
    const sprintSerial = (id) => {
      const m = String(id || "").match(/^PH-S(\d+)$/i);
      return m ? parseInt(m[1], 10) : 0;
    };
    // Eye: open only | open + queued. When drained: recent closed by serial.
    let show;
    if (openOnly.length) {
      show =
        queueFilterMode === "open_queued"
          ? openOnly.concat(
              queuedOnly
                .slice()
                .sort((a, b) => sprintSerial(b.id) - sprintSerial(a.id))
            )
          : openOnly;
    } else {
      show = queuedOnly
        .slice()
        .sort((a, b) => sprintSerial(b.id) - sprintSerial(a.id))
        .slice(0, 12);
    }
    let html =
      '<div class="sprint-queue-meta"><span><strong>' +
      escapeHtml(String(openCount)) +
      "</strong> open</span>";
    html +=
      '<span class="sprint-queue-filter-tag" title="Eye filter">' +
      escapeHtml(queueFilterLabel(queueFilterMode)) +
      "</span>";
    if (manifest.last_sprint_closed) {
      html +=
        '<span>last <strong>' +
        escapeHtml(String(manifest.last_sprint_closed)) +
        "</strong></span>";
    }
    html += '</div><ul class="sprint-queue-list">';
    if (!show.length) {
      html +=
        '<li class="sprint-queue-empty-row" style="color:var(--muted)">No sprints for this filter.</li>';
    }
    show.forEach((entry) => {
      const id = entry.id || "";
      const mapNode = pickMapNodeForSprint(id);
      const status =
        entry.status ||
        (entry.open ? "open" : "queued");
      const cls = [
        "sprint-queue-item",
        entry.open ? "open" : "closed",
        !entry.open ? "queued" : "",
        nextId && id === nextId ? "next" : "",
        mapNode ? "map-linked" : "no-map",
        activeQueueSprintId === id ? "queue-active" : "",
      ]
        .filter(Boolean)
        .join(" ");
      const mapHint = mapNode
        ? " · click → map: " + (mapNode.label || mapNode.path)
        : " · no map node";
      html +=
        '<li class="' +
        cls +
        '" data-sprint-id="' +
        escapeHtml(id) +
        '" title="' +
        escapeHtml((entry.acceptance || entry.deps || "").slice(0, 200) + mapHint) +
        '"><span class="sprint-queue-id">' +
        escapeHtml(id) +
        '</span><span class="sprint-queue-title">' +
        escapeHtml(entry.title || "") +
        '</span><span class="sprint-queue-status">' +
        escapeHtml(status) +
        "</span></li>";
    });
    html += "</ul>";
    box.innerHTML = html;
    bindSprintQueueItems(box);
  }

  function formatWallSecs(secs) {
    if (secs == null || Number.isNaN(Number(secs))) return "—";
    const s = Number(secs);
    if (s < 90) return s.toFixed(1) + "s";
    const m = Math.floor(s / 60);
    const r = Math.round(s % 60);
    return m + "m " + r + "s";
  }

  function formatNs(ns) {
    if (ns == null) return "—";
    const n = Number(ns);
    if (n >= 1e9) return (n / 1e9).toFixed(2) + " s";
    if (n >= 1e6) return (n / 1e6).toFixed(2) + " ms";
    if (n >= 1e3) return (n / 1e3).toFixed(1) + " µs";
    return n + " ns";
  }

  async function loadSpeedIndex() {
    try {
      const r = await fetch(SPEED_INDEX_URL + "?t=" + Date.now());
      if (r.ok) return await r.json();
    } catch (_) {
      /* try fallback */
    }
    try {
      const r2 = await fetch(SPEED_INDEX_FALLBACK + "?t=" + Date.now());
      if (r2.ok) return await r2.json();
    } catch (_) {
      /* empty */
    }
    return null;
  }

  async function loadRustDiagnostics() {
    try {
      const r = await fetch(RUST_DIAG_URL + "?t=" + Date.now());
      if (r.ok) return await r.json();
    } catch (_) {
      /* try fallback */
    }
    try {
      const r2 = await fetch(RUST_DIAG_FALLBACK + "?t=" + Date.now());
      if (r2.ok) return await r2.json();
    } catch (_) {
      /* empty */
    }
    return null;
  }

  function renderRustDiagnostics(data) {
    const box = document.getElementById("rust-diagnostics");
    if (!box) return;
    if (!data || data.latest == null || data.latest.warnings == null) {
      box.innerHTML =
        '<p class="rust-diagnostics-empty" style="color:var(--muted);margin:0">No <code>rust_diagnostics.json</code> yet. Run <code>bash bin/record-rust-diagnostics.sh</code> (local drain or CI).</p>';
      return;
    }
    const latest = data.latest || {};
    const hist = Array.isArray(data.history)
      ? data.history.slice().reverse().slice(0, 10)
      : [];
    const w = latest.warnings;
    const e = latest.errors;
    const ok = latest.ok;
    let statusCls = "unknown";
    if (e > 0 || ok === false) statusCls = "fail";
    else if (w > 0) statusCls = "warn";
    else if (ok === true) statusCls = "ok";
    let html = '<div class="speed-index-meta">';
    html +=
      "<span>host <strong>" +
      escapeHtml(String(data.host_label || "—")) +
      "</strong></span>";
    html +=
      "<span>git <strong>" +
      escapeHtml(String(data.git_head || "—")) +
      "</strong></span>";
    html +=
      "<span>src <strong>" +
      escapeHtml(String(data.source || "—")) +
      "</strong></span>";
    html +=
      "<span>at <strong>" +
      escapeHtml(String(data.generated_at || "—")) +
      "</strong></span>";
    html += "</div>";
    html += '<div class="speed-index-latest rust-diagnostics-latest">';
    html +=
      '<div class="speed-card ' +
      statusCls +
      '"><div class="speed-card-label">warnings</div><div class="speed-card-value">' +
      escapeHtml(String(w ?? "—")) +
      '</div><div class="speed-card-sub">' +
      escapeHtml(latest.recorded_at || "not recorded") +
      "</div></div>";
    html +=
      '<div class="speed-card ' +
      (e > 0 ? "fail" : "ok") +
      '"><div class="speed-card-label">errors</div><div class="speed-card-value">' +
      escapeHtml(String(e ?? "—")) +
      '</div><div class="speed-card-sub">' +
      escapeHtml(
        (ok === true ? "ok" : ok === false ? "fail" : "—") +
          (latest.wall_secs != null
            ? " · " + formatWallSecs(latest.wall_secs)
            : "")
      ) +
      "</div></div>";
    html += "</div>";
    const codes = Array.isArray(latest.top_codes) ? latest.top_codes : [];
    if (codes.length) {
      html += '<h3 class="speed-index-h">top codes</h3><ul class="speed-index-list">';
      codes.forEach((c) => {
        html +=
          '<li><span class="speed-id">lint</span><span class="speed-title">' +
          escapeHtml(String(c)) +
          "</span></li>";
      });
      html += "</ul>";
    }
    html += '<h3 class="speed-index-h">history</h3><ul class="speed-index-list">';
    if (!hist.length) {
      html +=
        '<li class="speed-index-empty-row" style="color:var(--muted)">Empty — record after drain / CI.</li>';
    }
    hist.forEach((entry) => {
      html +=
        '<li><span class="speed-id">' +
        escapeHtml(String(entry.warnings ?? 0) + "/" + String(entry.errors ?? 0)) +
        '</span><span class="speed-title">' +
        escapeHtml(
          (entry.ok ? "ok" : "fail") +
            " · " +
            (entry.source || "") +
            " · " +
            (entry.recorded_at || "")
        ) +
        "</span></li>";
    });
    html += "</ul>";
    box.innerHTML = html;
  }

  function renderSpeedIndex(data) {
    const box = document.getElementById("speed-index");
    if (!box) return;
    if (!data) {
      box.innerHTML =
        '<p class="speed-index-empty" style="color:var(--muted);margin:0">No <code>speed_index.json</code> yet. Run <code>bash bin/record-test-ci-speed.sh</code> after <code>cargo test-ci</code>.</p>';
      return;
    }
    const latest = data.latest || {};
    const testHist = Array.isArray(data.test_ci_history)
      ? data.test_ci_history.slice().reverse().slice(0, 8)
      : [];
    const benchHist = Array.isArray(data.bench_history)
      ? data.bench_history.slice().reverse().slice(0, 8)
      : [];
    let html = '<div class="speed-index-meta">';
    html +=
      "<span>host <strong>" +
      escapeHtml(String(data.host_label || "—")) +
      "</strong></span>";
    html +=
      "<span>git <strong>" +
      escapeHtml(String(data.git_head || "—")) +
      "</strong></span>";
    html +=
      "<span>at <strong>" +
      escapeHtml(String(data.generated_at || "—")) +
      "</strong></span>";
    html += "</div>";
    html += '<div class="speed-index-latest">';
    const ok = latest.test_ci_ok;
    const okCls = ok === true ? "ok" : ok === false ? "fail" : "unknown";
    html +=
      '<div class="speed-card ' +
      okCls +
      '"><div class="speed-card-label">cargo test-ci</div><div class="speed-card-value">' +
      escapeHtml(formatWallSecs(latest.test_ci_wall_secs)) +
      '</div><div class="speed-card-sub">' +
      escapeHtml(
        (ok === true ? "ok" : ok === false ? "fail" : "—") +
          " · " +
          (latest.test_ci_recorded_at || "not recorded")
      ) +
      "</div></div>";
    html +=
      '<div class="speed-card bench"><div class="speed-card-label">last Criterion</div><div class="speed-card-value">' +
      escapeHtml(formatNs(latest.last_bench_median_ns)) +
      '</div><div class="speed-card-sub">' +
      escapeHtml(
        (latest.last_bench_label || "—") +
          " · " +
          (latest.last_bench_recorded_at || "not recorded")
      ) +
      "</div></div>";
    html += "</div>";
    html += '<h3 class="speed-index-h">test-ci history</h3><ul class="speed-index-list">';
    if (!testHist.length) {
      html +=
        '<li class="speed-index-empty-row" style="color:var(--muted)">Empty — record after drain.</li>';
    }
    testHist.forEach((e) => {
      html +=
        '<li><span class="speed-id">' +
        escapeHtml(formatWallSecs(e.wall_secs)) +
        '</span><span class="speed-title">' +
        escapeHtml(e.ok ? "ok" : "fail") +
        " · " +
        escapeHtml(e.recorded_at || "") +
        '</span></li>';
    });
    html += '</ul><h3 class="speed-index-h">Criterion medians</h3><ul class="speed-index-list">';
    if (!benchHist.length) {
      html +=
        '<li class="speed-index-empty-row" style="color:var(--muted)">Empty — optional short benches.</li>';
    }
    benchHist.forEach((e) => {
      html +=
        '<li><span class="speed-id">' +
        escapeHtml(formatNs(e.median_ns)) +
        '</span><span class="speed-title">' +
        escapeHtml((e.bench || "") + "::" + (e.group || "")) +
        "</span></li>";
    });
    html += "</ul>";
    box.innerHTML = html;
  }

  function scrollToSprintQueue() {
    ensurePanelExpanded("queue");
    const panel = document.querySelector('.panel[data-panel="queue"]');
    if (!panel) return;
    panel.scrollIntoView({ behavior: "smooth", block: "nearest" });
    panel.classList.add("panel-flash");
    window.setTimeout(() => panel.classList.remove("panel-flash"), 900);
  }

  function renderRssTicker(feed) {
    const bar = document.getElementById("rss-ticker");
    const track = document.getElementById("rss-ticker-track");
    if (!bar || !track) return;
    const items = feed && Array.isArray(feed.items) ? feed.items : [];
    if (!items.length) {
      bar.hidden = true;
      track.innerHTML = "";
      return;
    }
    bar.hidden = false;
    const buildLi = (item) => {
      const id = item.id || "";
      const title = item.title || "";
      const summary = (item.summary || "").slice(0, 80);
      const mapLinked = !!pickMapNodeForSprint(id);
      const cls = [
        "rss-ticker-item",
        item.category === "open" ? "open" : "closed",
        item.next ? "next" : "",
        mapLinked ? "map-linked" : "",
      ]
        .filter(Boolean)
        .join(" ");
      const hint = summary ? " — " + summary : "";
      const mapHint = mapLinked ? " · click → map" : "";
      return (
        '<li class="' +
        cls +
        '" data-sprint-id="' +
        escapeHtml(id) +
        '" title="' +
        escapeHtml(id + ": " + title + hint + mapHint) +
        '"><strong>' +
        escapeHtml(id) +
        "</strong><span>" +
        escapeHtml(title) +
        "</span></li>"
      );
    };
    const chunk = items.map(buildLi).join("");
    track.innerHTML = chunk + chunk;
    track.querySelectorAll(".rss-ticker-item").forEach((el) => {
      const sprintId = el.dataset.sprintId || "";
      if (el.classList.contains("map-linked")) {
        bindMapLinkedSprintChip(el, sprintId);
      } else {
        el.setAttribute("aria-label", "Open sprint queue for " + sprintId);
        el.addEventListener("click", () => scrollToSprintQueue());
        el.style.cursor = "pointer";
      }
    });
    const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    bar.classList.toggle("rss-ticker-manual", reduced);
    if (reduced) {
      track.innerHTML = chunk;
      bar.style.removeProperty("--rss-ticker-duration");
    } else {
      const halfWidth = track.scrollWidth / 2;
      const pxPerSec = 42;
      const durationSec = Math.max(24, Math.min(120, halfWidth / pxPerSec));
      bar.style.setProperty("--rss-ticker-duration", durationSec + "s");
      track.style.animation = "none";
      void track.offsetWidth;
      track.style.animation = "";
    }
  }

  async function loadFeed() {
    try {
      return await loadJson("feed.json");
    } catch (_) {
      return null;
    }
  }

  function renderSprintChips(node) {
    const box = document.getElementById("sprint-chips");
    box.innerHTML = "";
    if (!node.sprints || !node.sprints.length) return;
    box.setAttribute("role", "list");
    box.setAttribute("aria-label", "Sprints for selected file");
    node.sprints.slice(0, 12).forEach((s) => {
      const span = document.createElement("span");
      span.className = "sprint-chip";
      span.setAttribute("role", "listitem");
      span.textContent = s;
      if (pickMapNodeForSprint(s)) {
        bindMapLinkedSprintChip(span, s);
      } else {
        span.setAttribute("aria-label", "Sprint " + s);
      }
      box.appendChild(span);
    });
    if (node.sprints.length > 12) {
      const more = document.createElement("span");
      more.className = "sprint-chip";
      more.setAttribute("role", "listitem");
      more.textContent = "+" + (node.sprints.length - 12);
      more.setAttribute(
        "aria-label",
        node.sprints.length - 12 + " more sprints not shown"
      );
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
      document.querySelectorAll(".panel").forEach((p) => {
        p.classList.remove("panel-fullscreen", "map-panel-fs");
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
      document.body.classList.remove("sidebar-overlay-open");
      document.body.classList.toggle("panel-fs-preview", isPreview);
      fullscreenPanel = panel;
      if (panel.dataset.panel === "map") {
        panel.classList.add("map-panel-fs");
      }
    } else {
      panel.classList.remove("map-panel-fs");
      panel.classList.remove("panel-fullscreen");
      const btn = panel.querySelector(".btn-panel-fs");
      if (btn) {
        btn.textContent = "⛶";
        btn.title = "Fullscreen (Esc)";
      }
      document.body.classList.remove(
        "panel-fs-active",
        "panel-fs-preview",
        "sidebar-overlay-open"
      );
      fullscreenPanel = null;
    }
    syncPanelCollapseLayout();
    if (panel.querySelector("#link-graph") && selectedId) {
      const n = nodeById(selectedId);
      if (n) renderLinkGraph(n);
    }
    window.dispatchEvent(new Event("resize"));
    syncMapFilterDock();
  }

  function exitPanelFullscreen() {
    const panelId = fullscreenPanel?.dataset?.panel;
    if (fullscreenPanel) setPanelFullscreen(fullscreenPanel, false);
    if (panelId) focusPanelToggle(panelId);
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
      if (ev.key === "Escape") {
        if (openFilterDrop) {
          closeMapFilterDrops();
          return;
        }
        if (document.body.classList.contains("sidebar-overlay-open")) {
          document.body.classList.remove("sidebar-overlay-open");
          return;
        }
        if (hasActiveMapFilters()) {
          mapLayerFocus = null;
          enabledLayers = null;
          enabledExts = null;
          syncMapFilterDock();
          updateMapFilters();
          return;
        }
        if (stackLayerFocus) {
          stackLayerFocus = null;
          syncLayerStackHighlight();
          return;
        }
        exitPanelFullscreen();
      }
    });
  }

  function selectNode(node) {
    if (!node) return;
    if (selectedId === node.id) return;
    mapNavNeighborKey = null;
    mapNavNeighborIdx = -1;
    edgeSelectTraceKey = null;
    edgeSelectPartnerId = null;
    selectedId = node.id;

    if (activeTreeFileEl) activeTreeFileEl.classList.remove("active");
    activeTreeFileEl = document.querySelector(
      '.tree-file[data-id="' + node.id + '"]'
    );
    if (activeTreeFileEl) {
      activeTreeFileEl.classList.add("active");
      revealTreeFile(activeTreeFileEl);
      focusTreeFileItem(activeTreeFileEl);
    }

    highlightLayer(node.layer);
    updateMapSelection();
    announceMapSelection(node);
    syncQueueHighlightFromNode(node);

    const picked = node;
    requestAnimationFrame(() => {
      renderSprintChips(picked);
      renderLinksList(picked);
      renderLinkGraph(picked);
    });
    openDoc(node);
  }

  function galaxyBackgroundFile() {
    const fromManifest =
      manifest && manifest.galaxy_background
        ? String(manifest.galaxy_background)
        : "";
    return fromManifest || "vision2.webp";
  }

  function galaxyBackgroundFallbackFile(file) {
    if (/\.webp$/i.test(file)) return file.replace(/\.webp$/i, ".png");
    return "";
  }

  function galaxyImageUrl() {
    const file = galaxyBackgroundFile();
    if (location.protocol === "file:") {
      return new URL("../../" + file, location.href).href;
    }
    const origin = location.origin || "http://127.0.0.1:8765";
    return origin + "/" + file.replace(/^\//, "");
  }

  function initGalaxyBackdrop() {
    const url = galaxyImageUrl();
    const file = galaxyBackgroundFile();
    document.querySelectorAll(".galaxy-backdrop-img, .galaxy-bg").forEach((img) => {
      if (img.dataset.galaxySrc === url) return;
      img.dataset.galaxySrc = url;
      img.src = url;
      img.alt = "";
      img.onerror = () => {
        if (img.dataset.fallbackTried) return;
        const fallback = galaxyBackgroundFallbackFile(file);
        if (fallback) {
          img.dataset.fallbackTried = "1";
          const fbUrl =
            location.protocol === "file:"
              ? new URL("../../" + fallback, location.href).href
              : (location.origin || "http://127.0.0.1:8765") +
                "/" +
                fallback.replace(/^\//, "");
          img.src = fbUrl;
          return;
        }
        console.warn(
          "[vision] Galaxy background not found:",
          file,
          "— place",
          file,
          "in repo root (not PoolAIGalaxy.png — that is the layer schema)."
        );
      };
    });
  }

  function restartStarfield() {
    if (starfieldStop) {
      starfieldStop();
      starfieldStop = null;
    }
    initStarfield();
  }

  function initStarfield() {
    const canvas = document.getElementById("starfield");
    if (!canvas) return;
    if (isLowGpuMode()) return;

    const ctx = canvas.getContext("2d", { alpha: true });
    let w = 0;
    let h = 0;
    let stars = [];
    let running = true;
    let lastFrame = 0;
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

    function dprCap() {
      return Math.min(window.devicePixelRatio || 1, 1.25);
    }

    function resize() {
      const cap = dprCap();
      w = window.innerWidth;
      h = window.innerHeight;
      canvas.width = Math.floor(w * cap);
      canvas.height = Math.floor(h * cap);
      canvas.style.width = w + "px";
      canvas.style.height = h + "px";
      ctx.setTransform(cap, 0, 0, cap, 0, 0);
      stars = Array.from({ length: STAR_COUNT_FX }, () => ({
        x: Math.random() * w,
        y: Math.random() * h,
        r: Math.random() * 1.1 + 0.25,
        a: Math.random(),
        sp: Math.random() * 0.018 + 0.004,
      }));
    }

    function drawStatic() {
      ctx.clearRect(0, 0, w, h);
      stars.forEach((s) => {
        ctx.beginPath();
        ctx.arc(s.x, s.y, s.r, 0, Math.PI * 2);
        ctx.fillStyle = "rgba(200, 220, 255, 0.35)";
        ctx.fill();
      });
    }

    function draw(ts) {
      if (!running) return;
      if (document.hidden) {
        return;
      }
      if (reducedMotion.matches) {
        drawStatic();
        return;
      }
      if (ts - lastFrame < STAR_FRAME_MS) {
        requestAnimationFrame(draw);
        return;
      }
      lastFrame = ts;
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
    const onResize = () => resize();
    window.addEventListener("resize", onResize);
    reducedMotion.addEventListener("change", () => {
      if (reducedMotion.matches) drawStatic();
    });
    requestAnimationFrame(draw);

    starfieldStop = () => {
      running = false;
      window.removeEventListener("resize", onResize);
      ctx.clearRect(0, 0, w, h);
    };
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

  function resolveHeaderGitHead(manifest, watchGitHead) {
    const fromWatch =
      watchGitHead && String(watchGitHead).trim()
        ? String(watchGitHead).trim()
        : null;
    if (fromWatch) return fromWatch;
    const fromManifest =
      manifest && manifest.git_head && String(manifest.git_head).trim()
        ? String(manifest.git_head).trim()
        : null;
    return fromManifest;
  }

  function updateHeaderMeta(manifest, watchGitHead) {
    if (!manifest) return;
    if (watchGitHead !== undefined) {
      const live = watchGitHead && String(watchGitHead).trim();
      if (live) headerGitHead = live;
    }
    const displayGit =
      headerGitHead || resolveHeaderGitHead(manifest, null) || null;

    const revEl = document.getElementById("meta-rev");
    const trailEl = document.getElementById("meta-trail");
    if (!revEl || !trailEl) return;

    let revHtml =
      "rev <strong>" + escapeHtml(String(manifest.revision)) + "</strong>";
    if (manifest.last_sprint_closed) {
      revHtml += " · " + escapeHtml(String(manifest.last_sprint_closed));
    }
    revEl.innerHTML = revHtml;

    let trailHtml = "";
    if (displayGit) {
      trailHtml +=
        '<span class="commit-pill" title="git HEAD (short)">' +
        escapeHtml(displayGit) +
        "</span>";
    }
    if (manifest.next_sprint) {
      trailHtml +=
        '<span class="sprint-pill" title="Next sprint (FM §5.12)">→ ' +
        escapeHtml(String(manifest.next_sprint)) +
        "</span>";
    }
    trailEl.innerHTML = trailHtml;
  }

  async function reloadAll(keepSelection) {
    const prevId = keepSelection ? selectedId : null;
    const fsPanelId = fullscreenPanel && fullscreenPanel.dataset.panel;
    try {
      await fetch(VISION_BASE + "__sync?t=" + Date.now());
    } catch (_) {
      /* sync optional when server has no __sync route */
    }
    manifest = await loadJson("manifest.json");
    try {
      extensions = await loadJson("extensions.json");
    } catch (_) {
      extensions = null;
    }
    activeSprint = resolveActiveSprint(manifest, extensions);
    nextSprint = manifest.next_sprint || null;
    sprintPathSet = buildSprintPathSet(extensions, activeSprint);
    rebuildManifestIndexes();
    updateSidebarSprintPill();

    updateHeaderMeta(manifest, headerGitHead);

    resolveVisionModeDefault();
    applyVisionMode();
    saveMapPrefs();

    syncLayerGeometry(manifest);
    renderLayers(manifest);
    renderMapFilterDock();
    syncMapToolbar();
    syncPanelCollapseLayout();
    const feed = await loadFeed();
    renderRssTicker(feed);
    renderMap();
    renderSprintQueue(manifest);
    const speed = await loadSpeedIndex();
    renderSpeedIndex(speed);
    const rustDiag = await loadRustDiagnostics();
    renderRustDiagnostics(rustDiag);

    const tree = document.getElementById("file-tree");
    tree.innerHTML = "";
    renderTree(buildTree(manifest.nodes), tree, 0);
    initTreeKeyboardNav();

    const target =
      (prevId && nodeById(prevId)) ||
      manifest.nodes.find((n) => n.id === "galaxy_grid") ||
      manifest.nodes[0];
    if (target) selectNode(target);

    initGalaxyBackdrop();

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
        if (manifest && w.git_head) {
          updateHeaderMeta(manifest, w.git_head);
        }
        return;
      }
      if (w.token === watchState.token) return;

      const bundleChanged = w.bundle !== watchState.bundle;
      const gitChanged =
        w.git_head && watchState.git_head && w.git_head !== watchState.git_head;
      const prev = watchState;
      watchState = w;

      if (gitChanged && manifest) {
        updateHeaderMeta(manifest, w.git_head);
        showAutoToast("HEAD → " + w.git_head);
      }

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
    watchTimer = setInterval(pollWatch, watchIntervalMs());
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
      ? "Auto-reload ON (" + watchIntervalMs() / 1000 + "s)"
      : "Auto-reload OFF";
    btn.setAttribute("aria-pressed", autoReloadEnabled ? "true" : "false");
    if (autoReloadEnabled) startAutoReload();
    else stopAutoReload();
  }

  function toggleSidebar() {
    if (document.body.classList.contains("panel-fs-active")) {
      document.body.classList.toggle("sidebar-overlay-open");
      return;
    }
    document.body.classList.remove("sidebar-overlay-open");
    document.body.classList.toggle("sidebar-collapsed");
  }

  function initSidebarOverlayBackdrop() {
    let backdrop = document.getElementById("sidebar-overlay-backdrop");
    const workspace = document.querySelector(".workspace");
    if (!backdrop) {
      backdrop = document.createElement("div");
      backdrop.id = "sidebar-overlay-backdrop";
      backdrop.className = "sidebar-overlay-backdrop";
      backdrop.setAttribute("aria-hidden", "true");
      backdrop.addEventListener("click", () => {
        document.body.classList.remove("sidebar-overlay-open");
      });
    }
    if (workspace && backdrop.parentElement !== workspace) {
      workspace.insertBefore(backdrop, workspace.firstChild);
    }
  }

  function initVisionVisibilityPerf() {
    if (document.body.dataset.visPerfBound) return;
    document.body.dataset.visPerfBound = "1";
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) {
        pauseMapOrbitAuto();
        if (starfieldStop) starfieldStop();
      } else {
        applyVisionMode();
        if (mapOrbitAutoPlay && manifest) startMapOrbitAuto();
      }
    });
  }

  function syncMapToolbar() {
    const btnSprint = document.getElementById("btn-map-sprint");
    const btnClusters = document.getElementById("btn-map-clusters");
    if (btnSprint) {
      btnSprint.classList.toggle("on", mapSprintFocus);
      btnSprint.setAttribute("aria-pressed", mapSprintFocus ? "true" : "false");
      btnSprint.setAttribute(
        "aria-label",
        mapSprintFocus ? "Sprint focus on — dim out-of-scope nodes" : "Sprint focus off"
      );
    }
    if (btnClusters) {
      btnClusters.classList.toggle("on", autoCollapseDense);
      btnClusters.setAttribute("aria-pressed", autoCollapseDense ? "true" : "false");
      btnClusters.setAttribute(
        "aria-label",
        autoCollapseDense ? "Folder clustering on" : "Folder clustering off"
      );
    }
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
        updateMapSprintDim();
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
  const btnPower = document.getElementById("btn-power");
  if (btnPower) bindVisionPowerMenu();
  document.getElementById("btn-auto").addEventListener("click", toggleAutoReload);
  const btnEco = document.getElementById("btn-eco");
  if (btnEco) btnEco.addEventListener("click", cycleVisionMode);

  initGalaxyBackdrop();
  initPanelFullscreen();
  initPanelCollapse();
  initMapFilterDropdowns();
  initSidebarOverlayBackdrop();
  loadMapPrefs();
  syncMapZoomBackBtn();
  syncMapOrbitAutoBtn();
  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
    mapOrbitAutoPlay = false;
  }
  if (
    window.matchMedia("(prefers-reduced-motion: reduce)").matches &&
    !visionModePinned
  ) {
    visionMode = "eco";
  }
  const reducedMotionMq = window.matchMedia("(prefers-reduced-motion: reduce)");
  reducedMotionMq.addEventListener("change", () => {
    if (!visionModePinned && reducedMotionMq.matches) {
      visionMode = "eco";
    }
    if (reducedMotionMq.matches) pauseMapOrbitAuto();
    applyVisionMode();
    if (manifest) renderMap();
  });
  applyVisionMode();
  initMapToolbar();
  initMapKeyboardNav();
  initMapOrbitControls();
  initVisionVisibilityPerf();
  applyMapOrbitTransform();
  reloadAll(false)
    .then(() => startAutoReload())
    .catch((err) => {
      document.getElementById("meta-rev").innerHTML =
        '<span class="error">' +
        escapeHtml(err.message) +
        " — run .\\bin\\open-docs-vision.ps1</span>";
    });
})();
