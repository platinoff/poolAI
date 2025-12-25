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
  .btn { padding: 8px 16px; border:1px solid #262b36; border-radius: 8px; background:#171b22; color:#e8e8e8; cursor:pointer; font-size: 0.95em; }
  .btn:hover { background:#1e2329; border-color:#44475a; }
  .btn:disabled { opacity:0.5; cursor:not-allowed; }
  .btn-primary { background:#50fa7b; color:#0f1216; border-color:#50fa7b; }
  .btn-primary:hover { background:#67e480; }
  .btn-danger { background:#ff5555; color:#fff; border-color:#ff5555; }
  .btn-danger:hover { background:#ff6e6e; }
  .form-group { margin-bottom: 16px; }
  .form-group label { display:block; margin-bottom: 6px; color:#cfe3ff; font-size: 0.9em; }
  .form-group input, .form-group select { width:100%; padding: 8px 12px; border:1px solid #262b36; border-radius: 8px; background:#0f1216; color:#e8e8e8; font-size: 0.95em; }
  .form-group input:focus, .form-group select:focus { outline:none; border-color:#50fa7b; }
  .modal { display:none; position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,0.7); z-index:1000; align-items:center; justify-content:center; }
  .modal.active { display:flex; }
  .modal-content { background:#171b22; border:1px solid #262b36; border-radius:14px; padding:24px; max-width:500px; width:90%; max-height:90vh; overflow-y:auto; }
  .modal-header { display:flex; justify-content:space-between; align-items:center; margin-bottom:20px; }
  .modal-header h3 { margin:0; color:#67e480; }
  .modal-close { background:none; border:none; color:#a8b0bf; font-size:24px; cursor:pointer; padding:0; width:30px; height:30px; }
  .modal-close:hover { color:#e8e8e8; }
  .modal-footer { display:flex; gap:12px; justify-content:flex-end; margin-top:20px; }
  .action-buttons { display:flex; gap:8px; flex-wrap:wrap; }
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
          <div class="muted">Dashboard with Write Operations (Stage 3)</div>
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
            <div class="muted">Auto-refresh is enabled (5s). Write operations available for authenticated users with appropriate permissions.</div>
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

// Modal dialog functions
function showModal(modalId) {
  const modal = document.getElementById(modalId);
  if (modal) {
    modal.classList.add('active');
  }
}

function hideModal(modalId) {
  const modal = document.getElementById(modalId);
  if (modal) {
    modal.classList.remove('active');
  }
}

function confirmAction(message, onConfirm) {
  if (confirm(message)) {
    onConfirm();
  }
}

// Enhanced confirmation dialog
function showConfirmDialog(message, onConfirm, onCancel = null) {
  const dialogId = 'confirmDialog';
  let dialog = document.getElementById(dialogId);
  
  if (!dialog) {
    dialog = document.createElement('div');
    dialog.id = dialogId;
    dialog.className = 'modal';
    dialog.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h3>Confirm Action</h3>
          <button class="modal-close" onclick="hideModal('${dialogId}')">&times;</button>
        </div>
        <div id="confirmMessage" style="margin-bottom:20px; color:#e8e8e8;"></div>
        <div class="modal-footer">
          <button class="btn" onclick="hideModal('${dialogId}')">Cancel</button>
          <button class="btn btn-danger" id="confirmBtn">Confirm</button>
        </div>
      </div>
    `;
    document.body.appendChild(dialog);
  }
  
  document.getElementById('confirmMessage').textContent = message;
  const confirmBtn = document.getElementById('confirmBtn');
  const oldHandler = confirmBtn.onclick;
  confirmBtn.onclick = function() {
    hideModal(dialogId);
    if (onConfirm) onConfirm();
  };
  
  const closeBtn = dialog.querySelector('.modal-close');
  closeBtn.onclick = function() {
    hideModal(dialogId);
    if (onCancel) onCancel();
  };
  
  showModal(dialogId);
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
    
    // Add action buttons for VM instances
    if (row && row.id && window.location.pathname.includes('/vm')) {
      const actionsTd = document.createElement('td');
      actionsTd.className = 'action-buttons';
      actionsTd.style.cssText = 'white-space: nowrap;';
      
      const instanceId = row.id;
      const status = row.status || '';
      
      // Start button
      if (status !== 'Running') {
        const startBtn = document.createElement('button');
        startBtn.className = 'btn btn-primary';
        startBtn.textContent = 'Start';
        startBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        startBtn.onclick = () => handleVmAction(instanceId, 'start');
        actionsTd.appendChild(startBtn);
      }
      
      // Stop button
      if (status === 'Running') {
        const stopBtn = document.createElement('button');
        stopBtn.className = 'btn';
        stopBtn.textContent = 'Stop';
        stopBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        stopBtn.onclick = () => handleVmAction(instanceId, 'stop');
        actionsTd.appendChild(stopBtn);
      }
      
      // Restart button
      if (status === 'Running') {
        const restartBtn = document.createElement('button');
        restartBtn.className = 'btn';
        restartBtn.textContent = 'Restart';
        restartBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        restartBtn.onclick = () => handleVmAction(instanceId, 'restart');
        actionsTd.appendChild(restartBtn);
      }
      
      // Delete button
      const deleteBtn = document.createElement('button');
      deleteBtn.className = 'btn btn-danger';
      deleteBtn.textContent = 'Delete';
      deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
      deleteBtn.onclick = () => handleVmDelete(instanceId, row.name || instanceId);
      actionsTd.appendChild(deleteBtn);
      
      tr.appendChild(actionsTd);
    }
    
    tbody.appendChild(tr);
  }
  table.appendChild(tbody);
  el.appendChild(table);
}

// Form validation
function validateForm(formId) {
  const form = document.getElementById(formId);
  if (!form) return false;
  
  const inputs = form.querySelectorAll('input[required], select[required]');
  let isValid = true;
  
  inputs.forEach(input => {
    if (!input.value.trim()) {
      isValid = false;
      input.style.borderColor = '#ff5555';
      setTimeout(() => {
        input.style.borderColor = '';
      }, 2000);
    } else {
      input.style.borderColor = '';
    }
    
    // Number validation
    if (input.type === 'number') {
      const min = input.getAttribute('min');
      const max = input.getAttribute('max');
      const value = parseInt(input.value, 10);
      
      if (min && value < parseInt(min, 10)) {
        isValid = false;
        input.style.borderColor = '#ff5555';
        showNotification(`Value must be at least ${min}`, 'error');
      } else if (max && value > parseInt(max, 10)) {
        isValid = false;
        input.style.borderColor = '#ff5555';
        showNotification(`Value must be at most ${max}`, 'error');
      }
    }
  });
  
  return isValid;
}

// VM action handlers
async function handleVmAction(instanceId, action) {
  const user = getUser();
  if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
    showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
    return;
  }
  
  try {
    showLoading('data', `${action.charAt(0).toUpperCase() + action.slice(1)}ing VM instance...`);
    
    const res = await fetchJson(`/api/v1/vm/instances/${instanceId}/${action}`, {
      method: 'POST'
    });
    
    showNotification(res.message || `VM instance ${action}ed successfully`, 'success');
    setTimeout(() => {
      const refreshFn = window.refreshVmInstances || (() => location.reload());
      refreshFn();
    }, 1000);
  } catch (e) {
    showNotification('Error: ' + e.message, 'error');
    hideLoading('data');
  }
}

async function handleVmDelete(instanceId, instanceName) {
  const user = getUser();
  if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
    showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
    return;
  }
  
  showConfirmDialog(
    `Are you sure you want to delete VM instance "${instanceName}" (${instanceId})? This action cannot be undone.`,
    async () => {
      try {
        showLoading('data', 'Deleting VM instance...');
        
        await fetchJson(`/api/v1/vm/instances/${instanceId}`, {
          method: 'DELETE'
        });
        
        showNotification('VM instance deleted successfully', 'success');
        setTimeout(() => {
          const refreshFn = window.refreshVmInstances || (() => location.reload());
          refreshFn();
        }, 1000);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        hideLoading('data');
      }
    }
  );
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
    let libs_js = r#"
    window.refreshLibraries = async function() {
      await poll('/api/v1/libraries', renderLibrariesTable, 'data');
    };
    
    function renderLibrariesTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No libraries installed.</div>';
        return;
      }
      
      const table = document.createElement('table');
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      ['name', 'version', 'type', 'status', 'actions'].forEach(k => {
        const th = document.createElement('th');
        th.textContent = k.charAt(0).toUpperCase() + k.slice(1);
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      for (const lib of data) {
        const tr = document.createElement('tr');
        
        ['name', 'version', 'type', 'status'].forEach(k => {
          const td = document.createElement('td');
          const v = lib ? lib[k] : null;
          td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
          tr.appendChild(td);
        });
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const libName = lib.name;
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Update button
          const updateBtn = document.createElement('button');
          updateBtn.className = 'btn';
          updateBtn.textContent = 'Update';
          updateBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          updateBtn.onclick = () => handleLibraryAction(libName, 'update');
          actionsTd.appendChild(updateBtn);
          
          // Uninstall button
          const uninstallBtn = document.createElement('button');
          uninstallBtn.className = 'btn btn-danger';
          uninstallBtn.textContent = 'Uninstall';
          uninstallBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          uninstallBtn.onclick = () => handleLibraryUninstall(libName);
          actionsTd.appendChild(uninstallBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      el.appendChild(table);
    }
    
    async function showInstallLibraryModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('installLibraryModal');
    }
    
    async function handleInstallLibrary(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('installLibraryForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Installing...';
      
      try {
        const libName = document.getElementById('libName').value;
        const version = document.getElementById('libVersion').value || 'latest';
        
        const result = await fetchJson(`/api/v1/libraries/${libName}/install`, {
          method: 'POST',
          body: JSON.stringify({ version })
        });
        
        showNotification('Library installed successfully', 'success');
        hideModal('installLibraryModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshLibraries();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleLibraryAction(libName, action) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        showLoading('data', `${action.charAt(0).toUpperCase() + action.slice(1)}ing library...`);
        
        const result = await fetchJson(`/api/v1/libraries/${libName}/${action}`, {
          method: 'POST'
        });
        
        showNotification(result.message || `Library ${action}d successfully`, 'success');
        setTimeout(() => {
          window.refreshLibraries();
        }, 1000);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        hideLoading('data');
      }
    }
    
    async function handleLibraryUninstall(libName) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to uninstall library "${libName}"? This action cannot be undone.`,
        async () => {
          try {
            showLoading('data', 'Uninstalling library...');
            
            await fetchJson(`/api/v1/libraries/${libName}/uninstall`, {
              method: 'POST'
            });
            
            showNotification('Library uninstalled successfully', 'success');
            setTimeout(() => {
              window.refreshLibraries();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('data');
          }
        }
      );
    }
    
    async function refresh() {
      await window.refreshLibraries();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;
    
    layout(
        "Libraries",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/libraries</code></div>
  <button class="btn btn-primary" onclick="showInstallLibraryModal()">Install Library</button>
</div>
<div id="data"></div>

<!-- Install Library Modal -->
<div id="installLibraryModal" class="modal">
  <div class="modal-content">
    <div class="modal-header">
      <h3>Install Library</h3>
      <button class="modal-close" onclick="hideModal('installLibraryModal')">&times;</button>
    </div>
    <form id="installLibraryForm" onsubmit="handleInstallLibrary(event)">
      <div class="form-group">
        <label for="libName">Library Name</label>
        <input type="text" id="libName" name="name" required placeholder="libtorch" />
      </div>
      <div class="form-group">
        <label for="libVersion">Version (optional, defaults to latest)</label>
        <input type="text" id="libVersion" name="version" placeholder="1.13.0" />
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('installLibraryModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Install</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), libs_js),
    )
}

async fn vm_page() -> Html<String> {
    let vm_js = r#"
    window.refreshVmInstances = async function() {
      await poll('/api/v1/vm/instances', renderTable, 'data');
    };
    
    async function showCreateVmModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createVmModal');
    }
    
    async function handleCreateVm(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createVmForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const payload = {
          name: document.getElementById('vmName').value,
          resources: {
            cpu_cores: parseInt(document.getElementById('vmCpuCores').value, 10),
            memory_mb: parseInt(document.getElementById('vmMemoryMb').value, 10),
            gpu_required: document.getElementById('vmGpuRequired').checked
          },
          isolation: document.getElementById('vmIsolation').value
        };
        
        const result = await fetchJson('/api/v1/vm/instances', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('VM instance created successfully', 'success');
        hideModal('createVmModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshVmInstances();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function refresh() {
      await window.refreshVmInstances();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;
    
    layout(
        "VM Instances",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/vm/instances</code></div>
  <button class="btn btn-primary" onclick="showCreateVmModal()">Create VM Instance</button>
</div>
<div id="data"></div>

<!-- Create VM Modal -->
<div id="createVmModal" class="modal">
  <div class="modal-content">
    <div class="modal-header">
      <h3>Create VM Instance</h3>
      <button class="modal-close" onclick="hideModal('createVmModal')">&times;</button>
    </div>
    <form id="createVmForm" onsubmit="handleCreateVm(event)">
      <div class="form-group">
        <label for="vmName">Instance Name</label>
        <input type="text" id="vmName" name="name" required placeholder="my-vm-instance" />
      </div>
      <div class="form-group">
        <label for="vmCpuCores">CPU Cores</label>
        <input type="number" id="vmCpuCores" name="cpu_cores" required min="1" max="64" value="2" />
      </div>
      <div class="form-group">
        <label for="vmMemoryMb">Memory (MB)</label>
        <input type="number" id="vmMemoryMb" name="memory_mb" required min="256" max="131072" value="2048" />
      </div>
      <div class="form-group">
        <label for="vmGpuRequired">
          <input type="checkbox" id="vmGpuRequired" name="gpu_required" />
          GPU Required
        </label>
      </div>
      <div class="form-group">
        <label for="vmIsolation">Isolation Type</label>
        <select id="vmIsolation" name="isolation" required>
          <option value="ProcessSandbox">Process Sandbox</option>
          <option value="HardwareVm">Hardware VM</option>
        </select>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createVmModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), vm_js),
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


