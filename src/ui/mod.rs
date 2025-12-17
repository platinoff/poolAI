//! UI module
//!
//! Concept alignment (planned in `poolAI_concept.txt`):
//! - Web dashboard (basic)
//! - UI components/themes/layouts (planned)

use crate::core::error::AppError;
use axum::{response::Html, routing::get, Router};
use tracing::info;

pub struct UiManager;

impl UiManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing UI module");
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down UI module");
        Ok(())
    }
}

pub fn create_ui_routes() -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/status", get(status_page))
        .route("/health", get(health_page))
        .route("/metrics", get(metrics_page))
        .route("/workers", get(workers_page))
        .route("/libs", get(libs_page))
        .route("/vm", get(vm_page))
        .route("/raid", get(raid_page))
}

const BASE_CSS: &str = r#"
  body { font-family: Segoe UI, Arial, sans-serif; background:#0f1216; color:#e8e8e8; margin:0; }
  a { color:#77c7ff; text-decoration:none; }
  a:hover { text-decoration:underline; }
  code { background:#0f1216; padding:2px 6px; border-radius:6px; border:1px solid #262b36; }
  .wrap { max-width: 1080px; margin: 28px auto; padding: 0 16px; }
  .topbar { display:flex; justify-content:space-between; align-items:center; gap:16px; padding: 14px 16px; border:1px solid #262b36; border-radius:14px; background:#171b22; box-shadow: 0 12px 40px rgba(0,0,0,.20); }
  .brand { display:flex; align-items:center; gap:12px; }
  .brand h1 { margin:0; font-size: 18px; color:#67e480; }
  .brand .muted { color:#a8b0bf; font-size: 0.95em; }
  .nav { display:flex; flex-wrap:wrap; gap:10px; align-items:center; }
  .nav a { padding: 6px 10px; border:1px solid #262b36; border-radius: 10px; background:#0f1216; }
  .content { margin-top: 14px; }
  .card { background:#171b22; border:1px solid #262b36; border-radius:14px; padding: 16px; box-shadow: 0 12px 40px rgba(0,0,0,.20); }
  .grid { display:grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-top: 12px; }
  .item { padding: 12px; border-radius: 12px; border:1px solid #262b36; background:#0f1216; }
  .muted { color:#a8b0bf; font-size: 0.95em; }
  .row { display:flex; align-items:center; justify-content:space-between; gap:12px; }
  pre { white-space: pre-wrap; word-break: break-word; background:#0b0d10; border:1px solid #262b36; border-radius: 12px; padding: 12px; margin: 12px 0 0; }
  table { width:100%; border-collapse: collapse; margin-top: 12px; }
  th, td { border:1px solid #262b36; padding: 8px; text-align:left; vertical-align: top; }
  th { background:#0f1216; color:#cfe3ff; }
  .pill { display:inline-block; padding: 2px 8px; border-radius: 999px; background:#0f1216; border:1px solid #262b36; color:#a8b0bf; font-size: 0.9em; }
  @media (max-width: 860px) { .grid { grid-template-columns: 1fr; } }
"#;

fn layout(title: &str, body_html: &str, script_js: &str) -> Html<String> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
  <style>{css}</style>
</head>
<body>
  <div class="wrap">
    <div class="topbar">
      <div class="brand">
        <div>
          <h1>PoolAI UI</h1>
          <div class="muted">Read-only dashboard (Stage 3)</div>
        </div>
      </div>
      <div class="nav">
        <a href="/ui">Home</a>
        <a href="/ui/status">Status</a>
        <a href="/ui/health">Health</a>
        <a href="/ui/metrics">Metrics</a>
        <a href="/ui/workers">Workers</a>
        <a href="/ui/libs">Libs</a>
        <a href="/ui/vm">VM</a>
        <a href="/ui/raid">RAID</a>
      </div>
    </div>

    <div class="content">
      <div class="card">
        <div class="row">
          <div>
            <h2 style="margin:0 0 6px">{title}</h2>
            <div class="muted">Auto-refresh is enabled (5s). This UI does not perform write operations.</div>
          </div>
          <div class="pill" id="last_updated">—</div>
        </div>
        {body}
      </div>
    </div>
  </div>

  <script>
  {script}
  </script>
</body>
</html>"#,
        title = title,
        css = BASE_CSS,
        body = body_html,
        script = script_js
    );

    Html(html)
}

fn common_js() -> &'static str {
    r#"
function setUpdated() {
  const el = document.getElementById('last_updated');
  if (el) el.textContent = 'Updated: ' + new Date().toLocaleTimeString();
}

function renderJsonPre(containerId, data) {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = '';
  const pre = document.createElement('pre');
  pre.textContent = JSON.stringify(data, null, 2);
  el.appendChild(pre);
}

function renderTable(containerId, data) {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = '';

  if (!Array.isArray(data)) {
    renderJsonPre(containerId, data);
    return;
  }

  if (data.length === 0) {
    el.innerHTML = '<div class=\"muted\">No items.</div>';
    return;
  }

  const keys = new Set();
  for (const row of data) {
    if (row && typeof row === 'object') {
      Object.keys(row).forEach(k => keys.add(k));
    }
  }
  const cols = Array.from(keys);
  if (cols.length === 0) {
    renderJsonPre(containerId, data);
    return;
  }

  const table = document.createElement('table');
  const thead = document.createElement('thead');
  const hr = document.createElement('tr');
  cols.forEach(k => {
    const th = document.createElement('th');
    th.textContent = k;
    hr.appendChild(th);
  });
  thead.appendChild(hr);
  table.appendChild(thead);

  const tbody = document.createElement('tbody');
  for (const row of data) {
    const tr = document.createElement('tr');
    cols.forEach(k => {
      const td = document.createElement('td');
      const v = row ? row[k] : null;
      td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
      tr.appendChild(td);
    });
    tbody.appendChild(tr);
  }
  table.appendChild(tbody);
  el.appendChild(table);
}

async function fetchJson(url) {
  const res = await fetch(url, { headers: { 'accept': 'application/json' } });
  if (!res.ok) throw new Error('HTTP ' + res.status);
  return await res.json();
}

async function poll(url, renderFn, containerId) {
  try {
    const data = await fetchJson(url);
    renderFn(containerId, data);
    setUpdated();
  } catch (e) {
    const el = document.getElementById(containerId);
    if (el) el.innerHTML = '<pre>' + String(e) + '</pre>';
  }
}
"#
}

async fn home_handler() -> Html<String> {
    layout(
        "Home",
        r#"
<div class="grid">
  <div class="item">
    <div><b>API</b></div>
    <div class="muted">Base: <code>/api/v1</code></div>
    <div style="margin-top:8px"><a href="/api/v1/status">/api/v1/status</a></div>
  </div>
  <div class="item">
    <div><b>UI</b></div>
    <div class="muted">Pages under <code>/ui</code></div>
    <div style="margin-top:8px"><a href="/ui/status">Open read-only dashboard</a></div>
  </div>
</div>

<div class="grid">
  <div class="item"><b>Quick links</b><div style="margin-top:8px">
    <a href="/ui/metrics">Metrics</a> ·
    <a href="/ui/workers">Workers</a> ·
    <a href="/ui/libs">Libs</a> ·
    <a href="/ui/vm">VM</a> ·
    <a href="/ui/raid">RAID</a>
  </div></div>
  <div class="item">
    <div><b>Notes</b></div>
    <div class="muted">This UI is intentionally read-only for safety. Write operations remain API-only for now.</div>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), "setUpdated();"),
    )
}

async fn status_page() -> Html<String> {
    layout(
        "Status",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/status', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn health_page() -> Html<String> {
    layout(
        "Health",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/health', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn metrics_page() -> Html<String> {
    layout(
        "Metrics",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/metrics', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn workers_page() -> Html<String> {
    layout(
        "Workers",
        r#"<div class="muted">Source: <code>/api/v1/workers</code></div><div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/workers', renderTable, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn libs_page() -> Html<String> {
    layout(
        "Libs",
        r#"<div class="muted">Source: <code>/api/v1/libraries</code></div><div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/libraries', renderTable, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn vm_page() -> Html<String> {
    layout(
        "VM",
        r#"<div class="muted">Source: <code>/api/v1/vm/instances</code></div><div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/vm/instances', renderTable, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn raid_page() -> Html<String> {
    layout(
        "RAID",
        r#"
<div class="grid">
  <div class="item">
    <div class="muted">Nodes: <code>/api/v1/raid/nodes</code></div>
    <div id="nodes"></div>
  </div>
  <div class="item">
    <div class="muted">Artifacts: <code>/api/v1/raid/artifacts</code></div>
    <div id="artifacts"></div>
  </div>
</div>
"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/raid/nodes', renderTable, 'nodes'); await poll('/api/v1/raid/artifacts', renderTable, 'artifacts'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

pub async fn initialize() -> Result<(), AppError> {
    UiManager::new().initialize().await
}

pub async fn shutdown() -> Result<(), AppError> {
    UiManager::new().shutdown().await
}


