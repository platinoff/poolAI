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
        .route("/auth", get(login_page))
        .route("/login", get(login_page))
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
  @keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
  @keyframes slideOut { from { transform: translateX(0); opacity: 1; } to { transform: translateX(100%); opacity: 0; } }
  @media (max-width: 860px) { .grid { grid-template-columns: 1fr; } }
"#;

fn layout(title: &str, body_html: &str, script_js: &str) -> Html<String> {
    let auth_url = "/ui/auth";
    let nav_auth_link = format!(r#"<a href="{}" id="authLoginBtn">Login</a>"#, auth_url);
    let user_info_html = "<div class=\"user-info\" id=\"userInfo\" style=\"display:none;\">\n          <span class=\"role\" id=\"userRole\"></span>\n          <a href=\"#\" id=\"logoutBtn\">Logout</a>\n        </div>";
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
        {user_info_html}
        {nav_auth_link}
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
        script = script_js,
        nav_auth_link = nav_auth_link,
        user_info_html = user_info_html
    );

    Html(html)
}

fn common_js() -> &'static str {
    r#"
// Token management
function getToken() {
  return localStorage.getItem('poolai_token');
}

function setToken(token) {
  localStorage.setItem('poolai_token', token);
}

function removeToken() {
  localStorage.removeItem('poolai_token');
  localStorage.removeItem('poolai_user');
  localStorage.removeItem('poolai_role');
  localStorage.removeItem('poolai_token_exp');
}

function getUser() {
  const user = localStorage.getItem('poolai_user');
  const role = localStorage.getItem('poolai_role');
  return user ? { username: user, role: role || 'Viewer' } : null;
}

function setUser(username, role) {
  localStorage.setItem('poolai_user', username);
  localStorage.setItem('poolai_role', role);
}

function updateUI() {
  const user = getUser();
  const userInfo = document.getElementById('userInfo');
  const loginLinkEl = document.getElementById('authLoginBtn');
  const userRole = document.getElementById('userRole');
  
  if (user) {
    if (userInfo) {
      userInfo.style.display = 'flex';
      if (userRole) userRole.textContent = user.role;
    }
    if (loginLinkEl) loginLinkEl.style.display = 'none';
  } else {
    if (userInfo) userInfo.style.display = 'none';
    if (loginLinkEl) loginLinkEl.style.display = 'inline';
  }
}

function getAuthHeaders() {
  const token = getToken();
  const headers = { 'accept': 'application/json', 'content-type': 'application/json' };
  if (token) {
    headers['authorization'] = 'Bearer ' + token;
  }
  return headers;
}

// Token validation and refresh
async function validateToken() {
  const token = getToken();
  if (!token) return false;
  
  try {
    // Decode token to check expiration (simple base64 decode for dev tokens)
    const parts = token.split('.');
    if (parts.length === 3) {
      // Real JWT format
      const payload = JSON.parse(atob(parts[1]));
      const now = Math.floor(Date.now() / 1000);
      if (payload.exp && payload.exp < now) {
        // Token expired, try to refresh
        return await refreshToken();
      }
      return true;
    } else if (token.startsWith('dev_token_')) {
      // Dev token format - check expiration from localStorage
      const tokenData = localStorage.getItem('poolai_token_exp');
      if (tokenData) {
        const exp = parseInt(tokenData, 10);
        const now = Math.floor(Date.now() / 1000);
        if (exp && exp < now) {
          return await refreshToken();
        }
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token validation error:', e);
    return false;
  }
}

async function refreshToken() {
  try {
    const token = getToken();
    if (!token) return false;
    
    // Try to refresh token via API (if endpoint exists)
    const res = await fetch('/api/v1/refresh', {
      method: 'POST',
      headers: getAuthHeaders(),
    });
    
    if (res.ok) {
      const data = await res.json();
      setToken(data.token);
      if (data.role) {
        const user = getUser();
        if (user) setUser(user.username, data.role);
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token refresh error:', e);
    return false;
  }
}

// Protected route check
async function requireAuth(requiredRole = null) {
  const user = getUser();
  if (!user) {
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  const isValid = await validateToken();
  if (!isValid) {
    removeToken();
    updateUI();
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  if (requiredRole) {
    const roleHierarchy = { 'Viewer': 1, 'Operator': 2, 'Admin': 3 };
    const userLevel = roleHierarchy[user.role] || 0;
    const requiredLevel = roleHierarchy[requiredRole] || 0;
    
    if (userLevel < requiredLevel) {
      alert('Insufficient permissions. Required role: ' + requiredRole);
      window.location.href = '/ui';
      return false;
    }
  }
  
  return true;
}

// User feedback functions
function showNotification(message, type = 'info', duration = 3000) {
  // Remove existing notification if any
  const existing = document.getElementById('globalNotification');
  if (existing) existing.remove();
  
  const notification = document.createElement('div');
  notification.id = 'globalNotification';
  notification.style.cssText = `
    position: fixed; top: 20px; right: 20px; z-index: 10000;
    padding: 12px 20px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    background: ${type === 'success' ? '#50fa7b' : type === 'error' ? '#ff5555' : '#8be9fd'};
    color: ${type === 'error' ? '#fff' : '#0f1216'};
    font-weight: 500; max-width: 400px; word-wrap: break-word;
    animation: slideIn 0.3s ease-out;
  `;
  notification.textContent = message;
  document.body.appendChild(notification);
  
  setTimeout(() => {
    notification.style.animation = 'slideOut 0.3s ease-out';
    setTimeout(() => notification.remove(), 300);
  }, duration);
}

function showLoading(elementId, message = 'Loading...') {
  const el = document.getElementById(elementId);
  if (!el) return;
  el.dataset.loading = 'true';
  el.innerHTML = `<div style="text-align:center; padding:20px; color:#a8b0bf;">${message}</div>`;
}

function hideLoading(elementId) {
  const el = document.getElementById(elementId);
  if (el && el.dataset.loading === 'true') {
    el.dataset.loading = 'false';
  }
}

async function fetchJson(url, options = {}) {
  const headers = getAuthHeaders();
  if (options.headers) {
    Object.assign(headers, options.headers);
  }
  const res = await fetch(url, { ...options, headers });
  if (res.status === 401) {
    const refreshed = await refreshToken();
    if (!refreshed) {
      removeToken();
      updateUI();
      if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
        window.location.href = '/ui/auth';
      }
      throw new Error('Unauthorized');
    }
    // Retry with new token
    headers['authorization'] = 'Bearer ' + getToken();
    const retryRes = await fetch(url, { ...options, headers });
    if (retryRes.status === 401) {
      removeToken();
      updateUI();
      if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
        window.location.href = '/ui/auth';
      }
      throw new Error('Unauthorized');
    }
    if (!retryRes.ok) {
      const errorData = await retryRes.json().catch(() => ({}));
      throw new Error(errorData.error || 'HTTP ' + retryRes.status);
    }
    return await retryRes.json();
  }
  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.error || 'HTTP ' + res.status);
  }
  return await res.json();
}

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

async function poll(url, renderFn, containerId) {
  try {
    const data = await fetchJson(url);
    renderFn(containerId, data);
    setUpdated();
  } catch (e) {
    const el = document.getElementById(containerId);
    if (el) {
      el.innerHTML = '<div style="color:#ff5555; padding:12px; border:1px solid #ff5555; border-radius:8px;">Error: ' + String(e) + '</div>';
    }
    console.error('Poll error:', e);
  }
}

// Token validation and refresh
async function validateToken() {
  const token = getToken();
  if (!token) return false;
  
  try {
    // Decode token to check expiration (simple base64 decode for dev tokens)
    const parts = token.split('.');
    if (parts.length === 3) {
      // Real JWT format
      const payload = JSON.parse(atob(parts[1]));
      const now = Math.floor(Date.now() / 1000);
      if (payload.exp && payload.exp < now) {
        // Token expired, try to refresh
        return await refreshToken();
      }
      return true;
    } else if (token.startsWith('dev_token_')) {
      // Dev token format - check expiration from localStorage
      const tokenData = localStorage.getItem('poolai_token_exp');
      if (tokenData) {
        const exp = parseInt(tokenData, 10);
        const now = Math.floor(Date.now() / 1000);
        if (exp && exp < now) {
          return await refreshToken();
        }
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token validation error:', e);
    return false;
  }
}

async function refreshToken() {
  try {
    const token = getToken();
    if (!token) return false;
    
    // Try to refresh token via API (if endpoint exists)
    const res = await fetch('/api/v1/refresh', {
      method: 'POST',
      headers: getAuthHeaders(),
    });
    
    if (res.ok) {
      const data = await res.json();
      setToken(data.token);
      if (data.role) {
        const user = getUser();
        if (user) setUser(user.username, data.role);
      }
      if (data.expires_in) {
        const exp = Math.floor(Date.now() / 1000) + data.expires_in;
        localStorage.setItem('poolai_token_exp', exp.toString());
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token refresh error:', e);
    return false;
  }
}

// Protected route check
async function requireAuth(requiredRole = null) {
  const user = getUser();
  if (!user) {
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  const isValid = await validateToken();
  if (!isValid) {
    removeToken();
    updateUI();
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  if (requiredRole) {
    const roleHierarchy = { 'Viewer': 1, 'Operator': 2, 'Admin': 3 };
    const userLevel = roleHierarchy[user.role] || 0;
    const requiredLevel = roleHierarchy[requiredRole] || 0;
    
    if (userLevel < requiredLevel) {
      alert('Insufficient permissions. Required role: ' + requiredRole);
      window.location.href = '/ui';
      return false;
    }
  }
  
  return true;
}

// Initialize UI on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', async function() {
    updateUI();
    // Check auth for protected routes (except login/auth pages)
    const path = window.location.pathname;
    if (path !== '/ui/auth' && path !== '/ui/login' && !path.endsWith('/auth') && !path.endsWith('/login')) {
      await requireAuth();
    }
  });
} else {
  updateUI();
  // Check auth for protected routes
  const path = window.location.pathname;
  if (path !== '/ui/auth' && path !== '/ui/login' && !path.endsWith('/auth') && !path.endsWith('/login')) {
    requireAuth();
  }
}

// Setup logout link
document.addEventListener('DOMContentLoaded', function() {
  const logoutLink = document.getElementById('logoutBtn');
  if (logoutLink) {
    logoutLink.addEventListener('click', function(e) {
      e.preventDefault();
      removeToken();
      updateUI();
      window.location.href = '/ui/auth';
    });
  }
});
"#
}

async fn login_page() -> Html<String> {
    let login_js = r#"
    function showAlert(message, type = 'error') {
      const alert = document.getElementById('alert');
      if (!alert) return;
      alert.className = 'alert alert-' + type;
      alert.textContent = message;
      alert.style.display = 'block';
    }
    
    function hideAlert() {
      const alert = document.getElementById('alert');
      if (alert) alert.style.display = 'none';
    }
    
    async function handleLogin(event) {
      event.preventDefault();
      hideAlert();
      
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      const btn = document.getElementById('loginBtn');
      
      btn.disabled = true;
      btn.textContent = 'Logging in...';
      
      try {
        const res = await fetch('/api/v1/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username, password })
        });
        
        if (!res.ok) {
          const data = await res.json();
          throw new Error(data.error || 'Login failed');
        }
        
        const data = await res.json();
        setToken(data.token);
        setUser(username, data.role);
        
        // Store token expiration time
        if (data.expires_in) {
          const exp = Math.floor(Date.now() / 1000) + data.expires_in;
          localStorage.setItem('poolai_token_exp', exp.toString());
        }
        
        updateUI();
        
        window.location.href = '/ui';
      } catch (e) {
        showAlert(e.message || 'Login failed', 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = 'Login';
      }
    }
    
    function logout() {
      removeToken();
      updateUI();
      window.location.href = '/ui/auth';
    }
    
    if (getUser()) {
      window.location.href = '/ui';
    }
    
    document.getElementById('loginForm').addEventListener('submit', handleLogin);
    "#;
    
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Login - PoolAI</title>
  <style>{css}</style>
</head>
<body>
  <div class="wrap">
    <div class="topbar">
      <div class="brand">
        <h1>PoolAI</h1>
      </div>
    </div>
    <div class="content">
      <div class="card" style="max-width: 400px; margin: 40px auto;">
        <h2 style="margin:0 0 20px">Login</h2>
        <div id="alert"></div>
        <form id="loginForm">
          <div class="form-group">
            <label for="username">Username</label>
            <input type="text" id="username" name="username" required autocomplete="username" />
          </div>
          <div class="form-group">
            <label for="password">Password</label>
            <input type="password" id="password" name="password" required autocomplete="current-password" />
          </div>
          <button type="submit" class="btn" id="loginBtn">Login</button>
        </form>
        <div style="margin-top: 20px; font-size: 0.9em; color:#a8b0bf;">
          <div><strong>Test accounts:</strong></div>
          <div>Admin: admin / admin123</div>
          <div>Operator: operator / op123</div>
          <div>Viewer: viewer / view123</div>
        </div>
      </div>
    </div>
  </div>
  <script>
    {common_js}
    {login_js}
  </script>
</body>
</html>"#,
        css = BASE_CSS,
        common_js = common_js(),
        login_js = login_js
    );
    Html(html)
}

async fn home_handler() -> Html<String> {
    let script = format!(
        r#"{}
// Protected route check for home
(async function() {{
  await requireAuth();
  setUpdated();
}})();
"#,
        common_js()
    );
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
          <div class="muted">Write operations are available for authenticated users with appropriate permissions.</div>
  </div>
</div>
"#,
        &script,
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


