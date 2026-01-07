//! UI module
//!
//! Provides web dashboard interface with read/write operations, authentication,
//! theme customization, and reusable UI components.
//!
//! # Features
//!
//! - **Dashboard Pages**: Status, health, metrics, workers, libs, VM, RAID
//! - **Authentication**: JWT-based authentication with role-based access control
//! - **Write Operations**: Create/delete artifacts and workers through UI
//! - **Theme Support**: Dark, light, and high-contrast themes
//! - **Components**: Reusable UI components (buttons, cards, forms, modals)
//!
//! # Routes
//!
//! - `/ui` - Home page
//! - `/ui/status` - System status
//! - `/ui/health` - Health check
//! - `/ui/metrics` - System metrics
//! - `/ui/workers` - Worker management
//! - `/ui/libs` - Library management
//! - `/ui/vm` - VM instance management
//! - `/ui/raid` - RAID artifact management
//!
//! Concept alignment (planned in `docs/concept/poolAI_concept.txt`):
//! - Web dashboard (basic)
//! - UI components/themes/layouts (planned)

mod components;
pub use components::get_component_styles;

mod themes;
pub use themes::{get_all_themes, get_theme, Theme, DARK_THEME, HIGH_CONTRAST_THEME, LIGHT_THEME};

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
  /* Box-sizing для правильного позиціонування */
  *, *::before, *::after {
    box-sizing: border-box;
  }
  
  body { 
    font-family: Segoe UI, Arial, sans-serif; 
    background: var(--bg, #0f1216); 
    color: var(--text, #e8e8e8); 
    margin: 0; 
    padding: 0;
  }
  a { color: var(--link, #77c7ff); text-decoration: none; }
  a:hover { color: var(--link-hover, #8bd5ff); text-decoration: underline; }
  code { background: var(--bg, #0f1216); padding: 2px 6px; border-radius: 6px; border: 1px solid var(--border, #262b36); }
  
  /* Wrap контейнер з автоматичним вирівнюванням */
  .wrap { 
    max-width: 1080px; 
    margin: 28px auto; 
    padding: 0 16px; 
    width: 100%;
  }
  
  /* Topbar з правильним вирівнюванням */
  .topbar { 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    gap: 16px; 
    padding: 14px 16px; 
    border: 1px solid var(--border, #262b36); 
    border-radius: 14px; 
    background: var(--surface, #171b22); 
    box-shadow: 0 12px 40px rgba(0,0,0,.20); 
    width: 100%;
    flex-wrap: wrap;
  }
  .brand { display: flex; align-items: center; gap: 12px; flex: 0 0 auto; }
  .brand h1 { margin: 0; font-size: 18px; color: var(--primary, #67e480); }
  .brand .muted { color: var(--text-muted, #a8b0bf); font-size: 0.95em; }
  
  /* Navigation з автоматичним вирівнюванням */
  .nav { 
    display: flex; 
    flex-wrap: wrap; 
    gap: 10px; 
    align-items: center; 
    flex: 1 1 auto;
    justify-content: flex-end;
  }
  .nav a { 
    padding: 6px 10px; 
    border: 1px solid var(--border, #262b36); 
    border-radius: 10px; 
    background: var(--bg, #0f1216); 
    white-space: nowrap;
  }
  
  /* Content з правильним spacing */
  .content { 
    margin-top: 14px; 
    width: 100%;
  }
  
  /* Grid з автоматичним вирівнюванням */
  .grid { 
    display: grid; 
    grid-template-columns: 1fr 1fr; 
    gap: 12px; 
    margin-top: 12px; 
    width: 100%;
  }
  .item { 
    padding: 12px; 
    border-radius: 12px; 
    border: 1px solid var(--border, #262b36); 
    background: var(--bg, #0f1216); 
    width: 100%;
  }
  .muted { color: var(--text-muted, #a8b0bf); font-size: 0.9em; }
  
  /* Row з автоматичним вирівнюванням до кордонів */
  .row { 
    display: flex; 
    align-items: center; 
    justify-content: space-between; 
    gap: 12px; 
    width: 100%;
    flex-wrap: wrap;
  }
  .row > * {
    flex: 0 0 auto;
  }
  .row > *:last-child {
    margin-left: auto;
  }
  
  pre { 
    white-space: pre-wrap; 
    word-break: break-word; 
    background: var(--bg, #0b0d10); 
    border: 1px solid var(--border, #262b36); 
    border-radius: 12px; 
    padding: 12px; 
    margin: 12px 0 0; 
    width: 100%;
    overflow-x: auto;
  }
  
  @media (max-width: 860px) { 
    .grid { grid-template-columns: 1fr; } 
  }
  /* Responsive Design */
  @media (max-width: 768px) {
    .wrap { padding: 0 12px; }
    .topbar { flex-direction: column; align-items: flex-start; gap: 12px; }
    .nav { width: 100%; flex-direction: column; align-items: stretch; }
    .nav a { width: 100%; text-align: center; padding: 10px; }
    .row { flex-direction: column; align-items: flex-start; }
    .card { padding: 12px; }
    table { font-size: 0.85em; }
    th, td { padding: 6px; }
  }
  @media (max-width: 480px) {
    .brand h1 { font-size: 16px; }
    .brand .muted { font-size: 0.85em; }
    .card { padding: 10px; }
    .btn { padding: 10px 16px; font-size: 0.9em; }
  }
  /* Accessibility: Skip links */
  .skip_link {
    position: absolute;
    top: -40px;
    left: 0;
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    padding: 8px 16px;
    text-decoration: none;
    border-radius: 4px;
    z-index: 10000;
    font-weight: bold;
  }
  .skip_link:focus {
    top: 0;
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  /* Accessibility: Focus indicators */
  a:focus, button:focus, input:focus, select:focus, textarea:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  a:focus-visible, button:focus-visible, input:focus-visible, select:focus-visible, textarea:focus-visible {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
"#;

fn layout(title: &str, body_html: &str, script_js: &str) -> Html<String> {
    let auth_url = "/ui/auth";
    let nav_auth_link = format!(r#"<a href="{}" id="authLoginBtn">Login</a>"#, auth_url);
    let user_info_html = "<div class=\"user-info\" id=\"userInfo\" style=\"display:none;\">\n          <span class=\"role\" id=\"userRole\"></span>\n          <a href=\"#\" id=\"logoutBtn\">Logout</a>\n        </div>";
    let component_styles = get_component_styles();
    let theme = DARK_THEME; // Default theme
    let theme_css = theme.to_css();
    let high_contrast_value = "high-contrast";

    // Prepare navigation links and attributes with dashes
    let nav_id = "navigation";
    let main_content_id = "main_content";
    let skip_link_class = "skip_link";
    let skip_to_main_href = format!("#{}", main_content_id);
    let skip_to_nav_href = format!("#{}", nav_id);
    let ui_base = "/ui";
    let ui_status = "/ui/status";
    let ui_health = "/ui/health";
    let ui_metrics = "/ui/metrics";
    let ui_workers = "/ui/workers";
    let ui_libs = "/ui/libs";
    let ui_vm = "/ui/vm";
    let ui_raid = "/ui/raid";
    let aria_label_nav = "Main navigation";
    let aria_label_home = "Home page";
    let aria_label_status = "System status";
    let aria_label_health = "Health check";
    let aria_label_metrics = "System metrics";
    let aria_label_workers = "Worker management";
    let aria_label_libs = "Library management";
    let aria_label_vm = "VM instance management";
    let aria_label_raid = "RAID artifact management";
    let aria_label_theme = "Select theme";
    let title_theme = "Select theme";
    let style_select = "padding: 4px 8px; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); color: var(--text); font-size: 0.9em; cursor: pointer;";

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title}</title>
  <style>{base_css}
{component_css}
{theme_css}</style>
</head>
<body>
  <!-- Skip links for accessibility -->
  <a href="{skip_to_main_href}" class="{skip_link_class}">Skip to main content</a>
  <a href="{skip_to_nav_href}" class="{skip_link_class}">Skip to navigation</a>
  
  <div class="wrap">
    <header class="topbar" role="banner">
      <div class="brand">
        <div>
          <h1>PoolAI UI</h1>
          <div class="muted">Dashboard with Write Operations (Stage 3)</div>
        </div>
      </div>
      <button class="mobile-menu-toggle" id="mobileMenuToggle" aria-label="Open navigation menu" aria-expanded="false">
        ☰
      </button>
      <nav class="nav" id="{nav_id}" role="navigation" aria-label="{aria_label_nav}">
        <a href="{ui_base}" aria-label="{aria_label_home}">Home</a>
        <a href="{ui_status}" aria-label="{aria_label_status}">Status</a>
        <a href="{ui_health}" aria-label="{aria_label_health}">Health</a>
        <a href="{ui_metrics}" aria-label="{aria_label_metrics}">Metrics</a>
        <a href="{ui_workers}" aria-label="{aria_label_workers}">Workers</a>
        <a href="{ui_libs}" aria-label="{aria_label_libs}">Libs</a>
        <a href="{ui_vm}" aria-label="{aria_label_vm}">VM</a>
        <a href="{ui_raid}" aria-label="{aria_label_raid}">RAID</a>
        <select id="themeSelector" aria-label="{aria_label_theme}" style="{style_select}" title="{title_theme}">
          <option value="dark">🌙 Dark</option>
          <option value="light">☀️ Light</option>
          <option value="{high_contrast_value}">🔆 High Contrast</option>
        </select>
        {user_info_html}
        {nav_auth_link}
      </nav>
    </header>

    <main class="content" id="{main_content_id}" role="main">
      <div class="card">
        <div class="row">
          <div>
            <h2 style="margin:0 0 6px">{title}</h2>
            <div class="muted">Auto-refresh is enabled (5s). Write operations available for authenticated users with appropriate permissions.</div>
          </div>
          <div class="pill" id="last_updated" aria-live="polite" aria-atomic="true">—</div>
        </div>
        {body}
      </div>
    </main>
  </div>

  <!-- Mobile Navigation Drawer -->
  <div class="mobile-nav-overlay" id="mobileNavOverlay"></div>
  <div class="mobile-nav-drawer" id="mobileNavDrawer" role="navigation" aria-label="Mobile navigation">
    <div class="mobile-nav-header">
      <h2 style="margin: 0; color: var(--primary, #67e480);">Menu</h2>
      <button class="mobile-nav-close" id="mobileNavClose" aria-label="Close navigation menu">×</button>
    </div>
    <div class="mobile-nav-content">
      <a href="{ui_base}" class="mobile-nav-item" aria-label="{aria_label_home}">Home</a>
      <a href="{ui_status}" class="mobile-nav-item" aria-label="{aria_label_status}">Status</a>
      <a href="{ui_health}" class="mobile-nav-item" aria-label="{aria_label_health}">Health</a>
      <a href="{ui_metrics}" class="mobile-nav-item" aria-label="{aria_label_metrics}">Metrics</a>
      <a href="{ui_workers}" class="mobile-nav-item" aria-label="{aria_label_workers}">Workers</a>
      <a href="{ui_libs}" class="mobile-nav-item" aria-label="{aria_label_libs}">Libs</a>
      <a href="{ui_vm}" class="mobile-nav-item" aria-label="{aria_label_vm}">VM</a>
      <a href="{ui_raid}" class="mobile-nav-item" aria-label="{aria_label_raid}">RAID</a>
      <div class="mobile-nav-item" style="flex-direction: column; align-items: flex-start; gap: 8px;">
        <label for="mobileThemeSelector" style="font-size: 0.9em; color: var(--text-muted, #a8b0bf);">Theme:</label>
        <select id="mobileThemeSelector" aria-label="{aria_label_theme}" style="width: 100%; padding: 8px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg); color: var(--text); font-size: 0.9em;">
          <option value="dark">🌙 Dark</option>
          <option value="light">☀️ Light</option>
          <option value="high-contrast">🔆 High Contrast</option>
        </select>
      </div>
      {user_info_html}
      {nav_auth_link}
    </div>
  </div>

  <!-- ARIA live region for notifications -->
  <div id="aria_live_region" aria-live="polite" aria-atomic="true" style="position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;"></div>

  <script>
  {script}
  </script>
</body>
</html>"#,
        title = title,
        base_css = BASE_CSS,
        component_css = component_styles,
        body = body_html,
        script = script_js,
        nav_auth_link = nav_auth_link,
        user_info_html = user_info_html,
        skip_to_main_href = skip_to_main_href,
        skip_link_class = skip_link_class,
        skip_to_nav_href = skip_to_nav_href,
        nav_id = nav_id,
        aria_label_nav = aria_label_nav,
        ui_base = ui_base,
        aria_label_home = aria_label_home,
        ui_status = ui_status,
        aria_label_status = aria_label_status,
        ui_health = ui_health,
        aria_label_health = aria_label_health,
        ui_metrics = ui_metrics,
        aria_label_metrics = aria_label_metrics,
        ui_workers = ui_workers,
        aria_label_workers = aria_label_workers,
        ui_libs = ui_libs,
        aria_label_libs = aria_label_libs,
        ui_vm = ui_vm,
        aria_label_vm = aria_label_vm,
        ui_raid = ui_raid,
        aria_label_raid = aria_label_raid,
        aria_label_theme = aria_label_theme,
        style_select = style_select,
        title_theme = title_theme,
        high_contrast_value = high_contrast_value,
        main_content_id = main_content_id
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

// Enhanced notification system with stacking and actions
let notificationStack = [];
let notificationIdCounter = 0;

function showNotification(message, type = 'info', duration = 3000, actions = null) {
  const notificationId = 'notification-' + (notificationIdCounter++);
  const notification = document.createElement('div');
  notification.id = notificationId;
  notification.className = 'notification notification-' + type;
  notification.setAttribute('role', 'alert');
  notification.setAttribute('aria-live', type === 'error' ? 'assertive' : 'polite');
  notification.setAttribute('aria-atomic', 'true');
  
  const notificationContent = document.createElement('div');
  notificationContent.style.cssText = 'display: flex; align-items: center; justify-content: space-between; gap: 12px;';
  
  const messageDiv = document.createElement('div');
  messageDiv.textContent = message;
  messageDiv.style.flex = '1';
  notificationContent.appendChild(messageDiv);
  
  if (actions && actions.length > 0) {
    const actionsDiv = document.createElement('div');
    actionsDiv.style.cssText = 'display: flex; gap: 8px;';
    actions.forEach(action => {
      const btn = document.createElement('button');
      btn.textContent = action.label;
      btn.style.cssText = 'padding: 4px 8px; border: none; background: rgba(255,255,255,0.2); color: inherit; border-radius: 4px; cursor: pointer; font-size: 0.85em;';
      btn.onclick = () => {
        if (action.onClick) action.onClick();
        removeNotification(notificationId);
      };
      actionsDiv.appendChild(btn);
    });
    notificationContent.appendChild(actionsDiv);
  }
  
  const closeBtn = document.createElement('button');
  closeBtn.innerHTML = '&times;';
  closeBtn.style.cssText = 'background: none; border: none; color: inherit; font-size: 20px; cursor: pointer; padding: 0; width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;';
  closeBtn.onclick = () => removeNotification(notificationId);
  closeBtn.setAttribute('aria-label', 'Close notification');
  notificationContent.appendChild(closeBtn);
  
  notification.appendChild(notificationContent);
  
  // Position notification
  const top = 20 + (notificationStack.length * 70);
  notification.style.cssText = `
    position: fixed; top: ${top}px; right: 20px; z-index: ${10000 + notificationStack.length};
    padding: 12px 20px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    background: ${type === 'success' ? '#50fa7b' : type === 'error' ? '#ff5555' : type === 'warning' ? '#f1fa8c' : '#8be9fd'};
    color: ${type === 'error' ? '#fff' : '#0f1216'};
    font-weight: 500; max-width: 400px; word-wrap: break-word;
    animation: slideIn 0.3s ease-out;
  `;
  
  document.body.appendChild(notification);
  notificationStack.push({ id: notificationId, element: notification });
  
  // Announce to screen readers
  const liveRegion = document.getElementById('aria_live_region');
  if (liveRegion) {
    liveRegion.textContent = message;
    setTimeout(() => {
      liveRegion.textContent = '';
    }, duration + 300);
  }
  
  if (duration > 0) {
    setTimeout(() => {
      removeNotification(notificationId);
    }, duration);
  }
  
  return notificationId;
}

function removeNotification(notificationId) {
  const index = notificationStack.findIndex(n => n.id === notificationId);
  if (index === -1) return;
  
  const notification = notificationStack[index].element;
  notification.style.animation = 'slideOut 0.3s ease-out';
  setTimeout(() => {
    notification.remove();
    notificationStack.splice(index, 1);
    // Reposition remaining notifications
    notificationStack.forEach((n, i) => {
      n.element.style.top = (20 + (i * 70)) + 'px';
      n.element.style.zIndex = (10000 + i).toString();
    });
  }, 300);
}

// Enhanced loading functions with skeleton support
function showLoading(elementId, message = 'Loading...', useSkeleton = false) {
  const el = document.getElementById(elementId);
  if (!el) return;
  el.dataset.loading = 'true';
  
  if (useSkeleton) {
    el.innerHTML = createSkeletonLoader(message);
  } else {
    el.innerHTML = `<div style="text-align:center; padding:20px; color:var(--text-muted, #a8b0bf);"><div class="spinner"></div><div style="margin-top:12px;">${message}</div></div>`;
  }
}

function hideLoading(elementId) {
  const el = document.getElementById(elementId);
  if (el && el.dataset.loading === 'true') {
    el.dataset.loading = 'false';
  }
}

function createSkeletonLoader(type = 'table') {
  if (type === 'table') {
    return `
      <div class="skeleton-card">
        <div class="skeleton skeleton-title"></div>
        <div class="skeleton skeleton-text"></div>
        <div class="skeleton skeleton-text" style="width: 80%;"></div>
        <div style="margin-top: 16px;">
          ${Array(5).fill('<div class="skeleton skeleton-table-row"></div>').join('')}
        </div>
      </div>
    `;
  } else if (type === 'card') {
    return `
      <div class="skeleton-card">
        <div class="skeleton skeleton-title"></div>
        <div class="skeleton skeleton-text"></div>
        <div class="skeleton skeleton-text" style="width: 90%;"></div>
        <div class="skeleton skeleton-text" style="width: 70%;"></div>
      </div>
    `;
  } else if (type === 'list') {
    return `
      <div>
        ${Array(3).fill(`
          <div class="skeleton-card" style="margin-bottom: 12px;">
            <div style="display: flex; align-items: center; gap: 12px;">
              <div class="skeleton skeleton-avatar"></div>
              <div style="flex: 1;">
                <div class="skeleton skeleton-text"></div>
                <div class="skeleton skeleton-text" style="width: 60%; margin-top: 8px;"></div>
              </div>
            </div>
          </div>
        `).join('')}
      </div>
    `;
  }
  return `<div class="skeleton skeleton-text"></div>`;
}

function showSpinner(containerId, message = 'Loading...', size = 'medium') {
  const container = document.getElementById(containerId);
  if (!container) return;
  
  const sizeClass = size === 'small' ? 'spinner-small' : size === 'large' ? 'spinner-large' : '';
  container.innerHTML = `
    <div style="display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 20px;">
      <div class="spinner ${sizeClass}"></div>
      ${message ? `<div class="loading-text">${message}</div>` : ''}
    </div>
  `;
}

function showLoadingOverlay(message = 'Loading...') {
  const overlay = document.createElement('div');
  overlay.id = 'loadingOverlay';
  overlay.className = 'loading-overlay';
  overlay.innerHTML = `
    <div class="loading-spinner-container">
      <div class="spinner"></div>
      <div class="loading-text">${message}</div>
    </div>
  `;
  document.body.appendChild(overlay);
}

function hideLoadingOverlay() {
  const overlay = document.getElementById('loadingOverlay');
  if (overlay) {
    overlay.remove();
  }
}

// Error handling functions with retry support
function showErrorBoundary(containerId, error, retryFn = null) {
  const container = document.getElementById(containerId);
  if (!container) return;
  
  container.innerHTML = `
    <div class="error-boundary">
      <div class="error-boundary-title">⚠️ Error</div>
      <div class="error-boundary-message">${escapeHtml(error.message || String(error))}</div>
      ${retryFn ? `
        <div class="error-boundary-actions">
          <button class="error-retry" onclick="(${retryFn.toString()})()">Retry</button>
        </div>
      ` : ''}
    </div>
  `;
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// Enhanced fetchJson with retry support
async function fetchJsonWithRetry(url, options = {}, maxRetries = 3, retryDelay = 1000) {
  let lastError;
  
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      const res = await fetchJson(url, options);
      return res;
    } catch (error) {
      lastError = error;
      
      if (attempt < maxRetries) {
        // Exponential backoff
        const delay = retryDelay * Math.pow(2, attempt);
        await new Promise(resolve => setTimeout(resolve, delay));
        continue;
      }
    }
  }
  
  throw lastError;
}

// Search & Filter functions
function initSearchFilter(searchInputId, tableId, filterOptions = {}) {
  const searchInput = document.getElementById(searchInputId);
  const table = document.getElementById(tableId);
  if (!searchInput || !table) return;
  
  let originalData = [];
  const tbody = table.querySelector('tbody');
  if (tbody) {
    // Store original rows
    originalData = Array.from(tbody.querySelectorAll('tr')).map(row => ({
      element: row,
      text: row.textContent.toLowerCase()
    }));
  }
  
  searchInput.addEventListener('input', function(e) {
    const query = e.target.value.toLowerCase().trim();
    filterTable(table, query, filterOptions);
  });
  
  // Add search icon if not present
  if (!searchInput.parentElement.querySelector('.search-icon')) {
    const icon = document.createElement('span');
    icon.className = 'search-icon';
    icon.innerHTML = '🔍';
    icon.style.cssText = 'position: absolute; right: 12px; top: 50%; transform: translateY(-50%); color: var(--text-muted, #a8b0bf); pointer-events: none;';
    searchInput.parentElement.style.position = 'relative';
    searchInput.parentElement.appendChild(icon);
  }
}

function filterTable(table, query, options = {}) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return;
  
  const rows = tbody.querySelectorAll('tr');
  let visibleCount = 0;
  
  rows.forEach(row => {
    const text = row.textContent.toLowerCase();
    const matches = !query || text.includes(query);
    
    if (matches) {
      row.style.display = '';
      visibleCount++;
    } else {
      row.style.display = 'none';
    }
  });
  
  // Show "no results" message if needed
  let noResultsRow = tbody.querySelector('.no-results-row');
  if (visibleCount === 0 && query) {
    if (!noResultsRow) {
      noResultsRow = document.createElement('tr');
      noResultsRow.className = 'no-results-row';
      const td = document.createElement('td');
      td.colSpan = table.querySelectorAll('th').length;
      td.textContent = 'No results found';
      td.style.cssText = 'text-align: center; padding: 20px; color: var(--text-muted, #a8b0bf);';
      noResultsRow.appendChild(td);
      tbody.appendChild(noResultsRow);
    }
    noResultsRow.style.display = '';
  } else if (noResultsRow) {
    noResultsRow.style.display = 'none';
  }
}

function sortTable(table, columnIndex, ascending = true) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return;
  
  const rows = Array.from(tbody.querySelectorAll('tr:not(.no-results-row)'));
  const isNumeric = rows.every(row => {
    const cell = row.cells[columnIndex];
    return cell && !isNaN(parseFloat(cell.textContent));
  });
  
  rows.sort((a, b) => {
    const aCell = a.cells[columnIndex];
    const bCell = b.cells[columnIndex];
    
    if (!aCell || !bCell) return 0;
    
    const aValue = isNumeric ? parseFloat(aCell.textContent) : aCell.textContent.trim();
    const bValue = isNumeric ? parseFloat(bCell.textContent) : bCell.textContent.trim();
    
    if (aValue < bValue) return ascending ? -1 : 1;
    if (aValue > bValue) return ascending ? 1 : -1;
    return 0;
  });
  
  // Remove all rows
  rows.forEach(row => row.remove());
  
  // Re-append sorted rows
  rows.forEach(row => tbody.appendChild(row));
  
  // Update sort indicators
  const headers = table.querySelectorAll('th');
  headers.forEach((header, index) => {
    if (index === columnIndex) {
      header.setAttribute('data-sort', ascending ? 'asc' : 'desc');
    } else {
      header.removeAttribute('data-sort');
    }
  });
}

function initTableSorting(tableId) {
  const table = document.getElementById(tableId);
  if (!table) return;
  
  const headers = table.querySelectorAll('th');
  headers.forEach((header, index) => {
    header.style.cursor = 'pointer';
    header.style.userSelect = 'none';
    header.addEventListener('click', function() {
      const currentSort = header.getAttribute('data-sort');
      const ascending = currentSort !== 'asc';
      sortTable(table, index, ascending);
    });
  });
}

// Modal dialog functions with keyboard navigation
let activeModal = null;
let modalFocusableElements = [];
let previousActiveElement = null;

function showModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) return;
  
  // Store previous active element for focus restoration
  previousActiveElement = document.activeElement;
  
  // Set ARIA attributes
  modal.setAttribute('aria-hidden', 'false');
  modal.setAttribute('aria-modal', 'true');
  modal.classList.add('active');
  activeModal = modal;
  
  // Get all focusable elements in modal
  modalFocusableElements = modal.querySelectorAll(
    'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
  );
  
  // Focus first focusable element
  if (modalFocusableElements.length > 0) {
    modalFocusableElements[0].focus();
  }
  
  // Trap focus within modal
  modal.addEventListener('keydown', trapModalFocus);
  
  // Prevent body scroll
  document.body.style.overflow = 'hidden';
}

function hideModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) return;
  
  // Remove ARIA attributes
  modal.setAttribute('aria-hidden', 'true');
  modal.setAttribute('aria-modal', 'false');
  modal.classList.remove('active');
  
  // Remove focus trap
  modal.removeEventListener('keydown', trapModalFocus);
  
  // Restore previous focus
  if (previousActiveElement) {
    previousActiveElement.focus();
    previousActiveElement = null;
  }
  
  activeModal = null;
  modalFocusableElements = [];
  
  // Restore body scroll
  document.body.style.overflow = '';
}

function trapModalFocus(e) {
  if (!activeModal || e.key !== 'Tab') return;
  
  const focusableElements = Array.from(activeModal.querySelectorAll(
    'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
  ));
  
  if (focusableElements.length === 0) return;
  
  const firstElement = focusableElements[0];
  const lastElement = focusableElements[focusableElements.length - 1];
  
  if (e.shiftKey) {
    // Shift + Tab
    if (document.activeElement === firstElement) {
      e.preventDefault();
      lastElement.focus();
    }
  } else {
    // Tab
    if (document.activeElement === lastElement) {
      e.preventDefault();
      firstElement.focus();
    }
  }
}

// Keyboard shortcuts
document.addEventListener('keydown', function(e) {
  // Esc key closes modals
  if (e.key === 'Escape' && activeModal) {
    hideModal(activeModal.id);
  }
  
  // Ctrl+K or Cmd+K for global search
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    showSearchModal();
  }
});

function confirmAction(message, onConfirm) {
  if (confirm(message)) {
    onConfirm();
  }
}

// Enhanced confirmation dialog with ARIA support
function showConfirmDialog(message, onConfirm, onCancel = null) {
  const dialogId = 'confirmDialog';
  let dialog = document.getElementById(dialogId);
  
  if (!dialog) {
    dialog = document.createElement('div');
    dialog.id = dialogId;
    dialog.className = 'modal';
    dialog.setAttribute('role', 'dialog');
    dialog.setAttribute('aria-labelledby', 'confirmDialogTitle');
    dialog.setAttribute('aria-describedby', 'confirmMessage');
    dialog.setAttribute('aria-modal', 'true');
    dialog.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h3 id="confirmDialogTitle">Confirm Action</h3>
          <button class="modal-close" aria-label="Close dialog" onclick="hideModal('${dialogId}')">&times;</button>
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
        startBtn.setAttribute('aria-label', `Start VM instance ${instanceId}`);
        startBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        startBtn.onclick = () => handleVmAction(instanceId, 'start');
        actionsTd.appendChild(startBtn);
      }
      
      // Stop button
      if (status === 'Running') {
        const stopBtn = document.createElement('button');
        stopBtn.className = 'btn';
        stopBtn.textContent = 'Stop';
        stopBtn.setAttribute('aria-label', `Stop VM instance ${instanceId}`);
        stopBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        stopBtn.onclick = () => handleVmAction(instanceId, 'stop');
        actionsTd.appendChild(stopBtn);
      }
      
      // Restart button
      if (status === 'Running') {
        const restartBtn = document.createElement('button');
        restartBtn.className = 'btn';
        restartBtn.textContent = 'Restart';
        restartBtn.setAttribute('aria-label', `Restart VM instance ${instanceId}`);
        restartBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        restartBtn.onclick = () => handleVmAction(instanceId, 'restart');
        actionsTd.appendChild(restartBtn);
      }
      
      // Delete button
      const deleteBtn = document.createElement('button');
      deleteBtn.className = 'btn btn-danger';
      deleteBtn.textContent = 'Delete';
      deleteBtn.setAttribute('aria-label', `Delete VM instance ${instanceId}`);
      deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
      deleteBtn.onclick = () => handleVmDelete(instanceId, row.name || instanceId);
      actionsTd.appendChild(deleteBtn);
      
      tr.appendChild(actionsTd);
    }
    
    tbody.appendChild(tr);
  }
  table.appendChild(tbody);
  
  // Обгортаємо таблицю в container для автоматичного вирівнювання
  const container = document.createElement('div');
  container.className = 'table-container';
  container.appendChild(table);
  el.appendChild(container);
}

// Enhanced form validation with real-time feedback
function validateForm(formId) {
  const form = document.getElementById(formId);
  if (!form) return false;
  
  const inputs = form.querySelectorAll('input[required], select[required], textarea[required]');
  let isValid = true;
  
  inputs.forEach(input => {
    const fieldValid = validateField(input);
    if (!fieldValid) {
      isValid = false;
    }
  });
  
  return isValid;
}

function validateField(input) {
  let isValid = true;
  let errorMessage = '';
  
  // Remove previous error message
  const existingError = input.parentElement.querySelector('.error-text');
  if (existingError) {
    existingError.remove();
  }
  
  // Required validation
  if (input.hasAttribute('required') && !input.value.trim()) {
    isValid = false;
    errorMessage = 'This field is required';
  }
  
  // Number validation
  if (input.type === 'number' && input.value) {
    const min = input.getAttribute('min');
    const max = input.getAttribute('max');
    const value = parseFloat(input.value);
    
    if (isNaN(value)) {
      isValid = false;
      errorMessage = 'Please enter a valid number';
    } else if (min && value < parseFloat(min)) {
      isValid = false;
      errorMessage = `Value must be at least ${min}`;
    } else if (max && value > parseFloat(max)) {
      isValid = false;
      errorMessage = `Value must be at most ${max}`;
    }
  }
  
  // Email validation
  if (input.type === 'email' && input.value) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(input.value)) {
      isValid = false;
      errorMessage = 'Please enter a valid email address';
    }
  }
  
  // Pattern validation
  if (input.hasAttribute('pattern') && input.value) {
    const pattern = new RegExp(input.getAttribute('pattern'));
    if (!pattern.test(input.value)) {
      isValid = false;
      errorMessage = input.getAttribute('data-pattern-error') || 'Invalid format';
    }
  }
  
  // Update UI
  if (isValid) {
    input.style.borderColor = '';
    input.classList.remove('error');
  } else {
    input.style.borderColor = 'var(--danger, #ff5555)';
    input.classList.add('error');
    
    // Show error message
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error-text';
    errorDiv.textContent = errorMessage;
    input.parentElement.appendChild(errorDiv);
  }
  
  return isValid;
}

// Real-time form validation
function initRealTimeValidation(formId) {
  const form = document.getElementById(formId);
  if (!form) return;
  
  const inputs = form.querySelectorAll('input, select, textarea');
  inputs.forEach(input => {
    // Validate on blur
    input.addEventListener('blur', function() {
      validateField(input);
    });
    
    // Validate on input (for immediate feedback)
    input.addEventListener('input', function() {
      if (input.classList.contains('error')) {
        validateField(input);
      }
    });
  });
}

// Form auto-save functionality
function initFormAutoSave(formId, storageKey, interval = 30000) {
  const form = document.getElementById(formId);
  if (!form) return;
  
  // Load saved data
  const savedData = localStorage.getItem(storageKey);
  if (savedData) {
    try {
      const data = JSON.parse(savedData);
      Object.keys(data).forEach(key => {
        const input = form.querySelector(`[name="${key}"]`);
        if (input && !input.value) {
          input.value = data[key];
        }
      });
    } catch (e) {
      console.error('Failed to load auto-saved form data:', e);
    }
  }
  
  // Auto-save on input
  let saveTimeout;
  form.addEventListener('input', function() {
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      const formData = new FormData(form);
      const data = {};
      for (const [key, value] of formData.entries()) {
        data[key] = value;
      }
      localStorage.setItem(storageKey, JSON.stringify(data));
    }, 1000); // Debounce 1 second
  });
  
  // Clear saved data on successful submit
  form.addEventListener('submit', function() {
    localStorage.removeItem(storageKey);
  });
}

// Form wizard functionality
function initFormWizard(wizardId) {
  const wizard = document.getElementById(wizardId);
  if (!wizard) return;
  
  const steps = wizard.querySelectorAll('.wizard-step');
  let currentStep = 0;
  
  steps.forEach((step, index) => {
    step.style.display = index === 0 ? 'block' : 'none';
    step.setAttribute('data-step', index.toString());
  });
  
  window.nextWizardStep = function() {
    if (currentStep < steps.length - 1) {
      steps[currentStep].style.display = 'none';
      currentStep++;
      steps[currentStep].style.display = 'block';
      updateWizardProgress(wizard, currentStep, steps.length);
    }
  };
  
  window.prevWizardStep = function() {
    if (currentStep > 0) {
      steps[currentStep].style.display = 'none';
      currentStep--;
      steps[currentStep].style.display = 'block';
      updateWizardProgress(wizard, currentStep, steps.length);
    }
  };
  
  updateWizardProgress(wizard, currentStep, steps.length);
}

function updateWizardProgress(wizard, current, total) {
  const progress = wizard.querySelector('.wizard-progress');
  if (progress) {
    const percentage = ((current + 1) / total) * 100;
    progress.style.width = percentage + '%';
  }
  
  const stepIndicator = wizard.querySelector('.wizard-step-indicator');
  if (stepIndicator) {
    stepIndicator.textContent = `Step ${current + 1} of ${total}`;
  }
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

// Theme management
function getTheme() {
  return localStorage.getItem('poolai_theme') || 'dark';
}

function setTheme(themeName) {
  localStorage.setItem('poolai_theme', themeName);
  applyTheme(themeName);
}

function applyTheme(themeName) {
  const themes = {
    dark: {
      bg: '#0f1216', surface: '#171b22', surfaceSecondary: '#0f1216',
      text: '#e8e8e8', textMuted: '#a8b0bf', border: '#262b36',
      primary: '#50fa7b', primaryHover: '#67e480',
      danger: '#ff5555', dangerHover: '#ff6e6e',
      secondary: '#6272a4', secondaryHover: '#7a8bc4',
      success: '#50fa7b', warning: '#f1fa8c', info: '#8be9fd',
      link: '#77c7ff', linkHover: '#8bd5ff'
    },
    light: {
      bg: '#ffffff', surface: '#f5f5f5', surfaceSecondary: '#e8e8e8',
      text: '#1a1a1a', textMuted: '#666666', border: '#d0d0d0',
      primary: '#00a86b', primaryHover: '#00c47a',
      danger: '#dc3545', dangerHover: '#c82333',
      secondary: '#6c757d', secondaryHover: '#5a6268',
      success: '#28a745', warning: '#ffc107', info: '#17a2b8',
      link: '#007bff', linkHover: '#0056b3'
    },
    'high-contrast': {
      bg: '#000000', surface: '#1a1a1a', surfaceSecondary: '#000000',
      text: '#ffffff', textMuted: '#cccccc', border: '#ffffff',
      primary: '#00ff00', primaryHover: '#00cc00',
      danger: '#ff0000', dangerHover: '#cc0000',
      secondary: '#ffff00', secondaryHover: '#cccc00',
      success: '#00ff00', warning: '#ffff00', info: '#00ffff',
      link: '#00aaff', linkHover: '#0088cc'
    }
  };
  
  const theme = themes[themeName] || themes.dark;
  const root = document.documentElement;
  
  root.style.setProperty('--bg', theme.bg);
  root.style.setProperty('--surface', theme.surface);
  root.style.setProperty('--surface-secondary', theme.surfaceSecondary);
  root.style.setProperty('--text', theme.text);
  root.style.setProperty('--text-muted', theme.textMuted);
  root.style.setProperty('--border', theme.border);
  root.style.setProperty('--primary', theme.primary);
  root.style.setProperty('--primary-hover', theme.primaryHover);
  root.style.setProperty('--danger', theme.danger);
  root.style.setProperty('--danger-hover', theme.dangerHover);
  root.style.setProperty('--secondary', theme.secondary);
  root.style.setProperty('--secondary-hover', theme.secondaryHover);
  root.style.setProperty('--success', theme.success);
  root.style.setProperty('--warning', theme.warning);
  root.style.setProperty('--info', theme.info);
  root.style.setProperty('--link', theme.link);
  root.style.setProperty('--link-hover', theme.linkHover);
}

// Progress Bar functions
function updateProgressBar(barId, value, max = 100) {
  const bar = document.getElementById(barId);
  if (!bar) return;
  
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100);
  const fill = bar.querySelector('.progress-bar-fill');
  if (fill) {
    fill.style.width = percentage + '%';
  }
  
  const labelValue = bar.querySelector('.progress-bar-label-value');
  if (labelValue) {
    labelValue.textContent = Math.round(percentage) + '%';
  }
}

function updateCircularProgress(circleId, value, max = 100) {
  const circle = document.getElementById(circleId);
  if (!circle) return;
  
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100);
  const circumference = 2 * Math.PI * 30; // radius = 30
  const offset = circumference - (percentage / 100) * circumference;
  
  const fill = circle.querySelector('.progress-bar-circular-fill');
  if (fill) {
    fill.style.strokeDashoffset = offset;
  }
  
  const text = circle.querySelector('.progress-bar-circular-text');
  if (text) {
    text.textContent = Math.round(percentage) + '%';
  }
}

// Tooltip functions
function initTooltips() {
  const tooltips = document.querySelectorAll('[data-tooltip]');
  tooltips.forEach(tooltip => {
    const text = tooltip.getAttribute('data-tooltip');
    const position = tooltip.getAttribute('data-tooltip-position') || 'top';
    const delay = parseInt(tooltip.getAttribute('data-tooltip-delay')) || 0;
    
    if (!tooltip.querySelector('.tooltip-content')) {
      const content = document.createElement('div');
      content.className = 'tooltip-content';
      content.textContent = text;
      tooltip.classList.add('tooltip', 'tooltip-' + position);
      tooltip.appendChild(content);
      
      if (delay > 0) {
        let timeout;
        tooltip.addEventListener('mouseenter', function() {
          timeout = setTimeout(() => {
            content.style.visibility = 'visible';
            content.style.opacity = '1';
          }, delay);
        });
        tooltip.addEventListener('mouseleave', function() {
          clearTimeout(timeout);
          content.style.visibility = 'hidden';
          content.style.opacity = '0';
        });
      }
    }
  });
}

// Dropdown functions
function initDropdowns() {
  const dropdowns = document.querySelectorAll('.dropdown');
  dropdowns.forEach(dropdown => {
    const toggle = dropdown.querySelector('.dropdown-toggle');
    const menu = dropdown.querySelector('.dropdown-menu');
    if (!toggle || !menu) return;
    
    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      const isActive = menu.classList.contains('active');
      closeAllDropdowns();
      if (!isActive) {
        menu.classList.add('active');
        toggle.setAttribute('aria-expanded', 'true');
      }
    });
    
    // Keyboard navigation
    toggle.addEventListener('keydown', function(e) {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        toggle.click();
      } else if (e.key === 'Escape') {
        closeAllDropdowns();
      }
    });
    
    // Close on outside click
    document.addEventListener('click', function(e) {
      if (!dropdown.contains(e.target)) {
        menu.classList.remove('active');
        toggle.setAttribute('aria-expanded', 'false');
      }
    });
    
    // Item selection
    const items = menu.querySelectorAll('.dropdown-item');
    items.forEach((item, index) => {
      item.addEventListener('click', function() {
        const value = item.getAttribute('data-value');
        if (value !== null) {
          toggle.textContent = item.textContent;
          toggle.setAttribute('data-value', value);
          menu.classList.remove('active');
          toggle.setAttribute('aria-expanded', 'false');
          
          // Trigger change event
          const event = new CustomEvent('dropdown-change', { detail: { value, text: item.textContent } });
          dropdown.dispatchEvent(event);
        }
      });
      
      // Keyboard navigation
      item.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          item.click();
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          const next = items[index + 1] || items[0];
          next.focus();
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          const prev = items[index - 1] || items[items.length - 1];
          prev.focus();
        } else if (e.key === 'Escape') {
          closeAllDropdowns();
          toggle.focus();
        }
      });
      
      item.setAttribute('tabindex', '0');
      item.setAttribute('role', 'option');
    });
  });
}

function closeAllDropdowns() {
  const dropdowns = document.querySelectorAll('.dropdown-menu');
  dropdowns.forEach(menu => {
    menu.classList.remove('active');
    const toggle = menu.parentElement.querySelector('.dropdown-toggle');
    if (toggle) {
      toggle.setAttribute('aria-expanded', 'false');
    }
  });
}

// Tabs functions
function initTabs() {
  const tabContainers = document.querySelectorAll('.tabs-container');
  tabContainers.forEach(container => {
    const tabs = container.querySelectorAll('.tab');
    const contents = container.querySelectorAll('.tab-content');
    
    tabs.forEach((tab, index) => {
      tab.addEventListener('click', function() {
        // Remove active class from all tabs and contents
        tabs.forEach(t => t.classList.remove('active'));
        contents.forEach(c => c.classList.remove('active'));
        
        // Add active class to clicked tab and corresponding content
        tab.classList.add('active');
        const contentId = tab.getAttribute('data-tab');
        if (contentId) {
          const content = container.querySelector('#' + contentId);
          if (content) {
            content.classList.add('active');
          }
        } else if (contents[index]) {
          contents[index].classList.add('active');
        }
        
        // Update ARIA attributes
        tabs.forEach((t, i) => {
          t.setAttribute('aria-selected', i === index ? 'true' : 'false');
          if (contents[i]) {
            contents[i].setAttribute('aria-hidden', i === index ? 'false' : 'true');
          }
        });
      });
      
      // Keyboard navigation
      tab.addEventListener('keydown', function(e) {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
          e.preventDefault();
          const direction = e.key === 'ArrowRight' ? 1 : -1;
          const nextIndex = (index + direction + tabs.length) % tabs.length;
          tabs[nextIndex].click();
          tabs[nextIndex].focus();
        } else if (e.key === 'Home') {
          e.preventDefault();
          tabs[0].click();
          tabs[0].focus();
        } else if (e.key === 'End') {
          e.preventDefault();
          tabs[tabs.length - 1].click();
          tabs[tabs.length - 1].focus();
        }
      });
      
      tab.setAttribute('role', 'tab');
      tab.setAttribute('aria-selected', index === 0 ? 'true' : 'false');
      if (contents[index]) {
        contents[index].setAttribute('role', 'tabpanel');
        contents[index].setAttribute('aria-hidden', index === 0 ? 'false' : 'true');
      }
    });
  });
}

// Accordion functions
function initAccordions() {
  const accordions = document.querySelectorAll('.accordion');
  accordions.forEach(accordion => {
    const items = accordion.querySelectorAll('.accordion-item');
    
    items.forEach(item => {
      const header = item.querySelector('.accordion-header');
      const content = item.querySelector('.accordion-content');
      if (!header || !content) return;
      
      header.addEventListener('click', function() {
        const isActive = item.classList.contains('active');
        
        // Close all items if not allowing multiple open
        if (!accordion.hasAttribute('data-multiple')) {
          items.forEach(i => i.classList.remove('active'));
        }
        
        // Toggle current item
        if (isActive) {
          item.classList.remove('active');
          header.setAttribute('aria-expanded', 'false');
        } else {
          item.classList.add('active');
          header.setAttribute('aria-expanded', 'true');
        }
      });
      
      // Keyboard navigation
      header.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          header.click();
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          const next = item.nextElementSibling;
          if (next) {
            next.querySelector('.accordion-header').focus();
          }
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          const prev = item.previousElementSibling;
          if (prev) {
            prev.querySelector('.accordion-header').focus();
          }
        } else if (e.key === 'Home') {
          e.preventDefault();
          items[0].querySelector('.accordion-header').focus();
        } else if (e.key === 'End') {
          e.preventDefault();
          items[items.length - 1].querySelector('.accordion-header').focus();
        }
      });
      
      header.setAttribute('role', 'button');
      header.setAttribute('aria-expanded', 'false');
      header.setAttribute('tabindex', '0');
      content.setAttribute('role', 'region');
    });
  });
}

// Mobile Navigation functions
function initMobileNavigation() {
  const toggle = document.getElementById('mobileMenuToggle');
  const drawer = document.getElementById('mobileNavDrawer');
  const overlay = document.getElementById('mobileNavOverlay');
  const closeBtn = document.getElementById('mobileNavClose');
  const mobileThemeSelector = document.getElementById('mobileThemeSelector');
  
  if (!toggle || !drawer || !overlay) return;
  
  function openDrawer() {
    drawer.classList.add('active');
    overlay.classList.add('active');
    toggle.setAttribute('aria-expanded', 'true');
    document.body.style.overflow = 'hidden';
  }
  
  function closeDrawer() {
    drawer.classList.remove('active');
    overlay.classList.remove('active');
    toggle.setAttribute('aria-expanded', 'false');
    document.body.style.overflow = '';
  }
  
  toggle.addEventListener('click', openDrawer);
  if (closeBtn) closeBtn.addEventListener('click', closeDrawer);
  overlay.addEventListener('click', closeDrawer);
  
  // Close on escape key
  document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && drawer.classList.contains('active')) {
      closeDrawer();
    }
  });
  
  // Close drawer when clicking on nav item
  const navItems = drawer.querySelectorAll('.mobile-nav-item');
  navItems.forEach(item => {
    item.addEventListener('click', function() {
      setTimeout(closeDrawer, 100);
    });
  });
  
  // Sync mobile theme selector with main theme selector
  if (mobileThemeSelector) {
    const mainThemeSelector = document.getElementById('themeSelector');
    if (mainThemeSelector) {
      mobileThemeSelector.value = mainThemeSelector.value;
      mobileThemeSelector.addEventListener('change', function(e) {
        const newTheme = e.target.value;
        setTheme(newTheme);
        if (mainThemeSelector) {
          mainThemeSelector.value = newTheme;
        }
      });
    }
  }
}

// Touch Gesture functions
function initTouchGestures() {
  // Swipe detection for swipeable elements
  const swipeables = document.querySelectorAll('.swipeable');
  swipeables.forEach(element => {
    let startX = 0;
    let startY = 0;
    let currentX = 0;
    let isSwiping = false;
    
    element.addEventListener('touchstart', function(e) {
      startX = e.touches[0].clientX;
      startY = e.touches[0].clientY;
      isSwiping = false;
    });
    
    element.addEventListener('touchmove', function(e) {
      if (!startX || !startY) return;
      
      currentX = e.touches[0].clientX;
      const currentY = e.touches[0].clientY;
      const diffX = startX - currentX;
      const diffY = startY - currentY;
      
      if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 10) {
        isSwiping = true;
        const content = element.querySelector('.swipeable-content');
        if (content) {
          const translateX = Math.max(-100, Math.min(0, -diffX));
          content.style.transform = 'translateX(' + translateX + 'px)';
        }
      }
    });
    
    element.addEventListener('touchend', function(e) {
      if (!isSwiping) return;
      
      const diffX = startX - currentX;
      const threshold = 50;
      
      if (Math.abs(diffX) > threshold) {
        if (diffX > 0) {
          element.classList.add('swiped');
          const content = element.querySelector('.swipeable-content');
          if (content) {
            content.style.transform = 'translateX(-80px)';
          }
        } else {
          element.classList.remove('swiped');
          const content = element.querySelector('.swipeable-content');
          if (content) {
            content.style.transform = 'translateX(0)';
          }
        }
      } else {
        const content = element.querySelector('.swipeable-content');
        if (content) {
          content.style.transform = 'translateX(0)';
        }
      }
      
      startX = 0;
      startY = 0;
      currentX = 0;
      isSwiping = false;
    });
  });
  
  // Touch feedback for buttons
  const touchElements = document.querySelectorAll('.btn, .nav a, .dropdown-toggle, .tab, .accordion-header');
  touchElements.forEach(element => {
    element.classList.add('touch-feedback');
    element.addEventListener('touchstart', function() {
      this.classList.add('touch-active');
    });
    element.addEventListener('touchend', function() {
      const self = this;
      setTimeout(function() {
        self.classList.remove('touch-active');
      }, 150);
    });
  });
}

// Responsive Tables functions
function initResponsiveTables() {
  if (window.innerWidth <= 768) {
    const tables = document.querySelectorAll('table');
    tables.forEach(table => {
      if (table.classList.contains('responsive-table-card')) return;
      
      const container = table.parentElement;
      if (!container || container.classList.contains('responsive-table-container')) return;
      
      const wrapper = document.createElement('div');
      wrapper.className = 'responsive-table-container';
      table.parentElement.insertBefore(wrapper, table);
      wrapper.appendChild(table);
      
      const headers = table.querySelectorAll('th');
      const headerTexts = Array.from(headers).map(function(th) {
        return th.textContent.trim();
      });
      
      table.classList.add('responsive-table-card');
      const rows = table.querySelectorAll('tbody tr');
      rows.forEach(function(row) {
        const cells = row.querySelectorAll('td');
        cells.forEach(function(cell, index) {
          if (headerTexts[index]) {
            cell.setAttribute('data-label', headerTexts[index]);
          }
        });
      });
    });
  }
}

// Handle window resize
let resizeTimeout;
window.addEventListener('resize', function() {
  clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(function() {
    initResponsiveTables();
  }, 250);
});

// Setup logout link and theme selector
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
  
  // Setup theme selector
  const themeSelector = document.getElementById('themeSelector');
  if (themeSelector) {
    const currentTheme = getTheme();
    themeSelector.value = currentTheme;
    applyTheme(currentTheme);
    
    themeSelector.addEventListener('change', function(e) {
      const newTheme = e.target.value;
      setTheme(newTheme);
    });
  }
  
  // Initialize new UI components
  initTooltips();
  initDropdowns();
  initTabs();
  initAccordions();
  initMobileNavigation();
  initTouchGestures();
  initResponsiveTables();
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
    let workers_js = r#"
    window.refreshWorkers = async function() {
      await poll('/api/v1/workers', renderWorkersTable, 'data');
    };
    
    function renderWorkersTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No workers available.</div>';
        return;
      }
      
      const table = document.createElement('table');
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      ['id', 'status', 'actions'].forEach(k => {
        const th = document.createElement('th');
        th.textContent = k.charAt(0).toUpperCase() + k.slice(1);
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      for (const worker of data) {
        const tr = document.createElement('tr');
        
        ['id', 'status'].forEach(k => {
          const td = document.createElement('td');
          const v = worker ? worker[k] : null;
          td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
          tr.appendChild(td);
        });
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const workerId = worker.id;
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Delete button
          const deleteBtn = document.createElement('button');
          deleteBtn.className = 'btn btn-danger';
          deleteBtn.textContent = 'Delete';
          deleteBtn.setAttribute('aria-label', `Delete worker ${workerId}`);
          deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          deleteBtn.onclick = () => handleWorkerDelete(workerId);
          actionsTd.appendChild(deleteBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
    }
    
    async function showCreateWorkerModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createWorkerModal');
    }
    
    async function handleCreateWorker(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createWorkerForm')) {
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
          worker_id: document.getElementById('workerId').value,
          max_concurrent_requests: parseInt(document.getElementById('workerMaxConcurrent').value, 10) || 10,
          request_timeout_ms: parseInt(document.getElementById('workerTimeout').value, 10) || 5000,
          health_check_interval_ms: parseInt(document.getElementById('workerHealthInterval').value, 10) || 1000,
          enable_caching: document.getElementById('workerEnableCache').checked,
          cache_size: parseInt(document.getElementById('workerCacheSize').value, 10) || 1000,
          max_memory_mb: parseInt(document.getElementById('workerMaxMemory').value, 10) || 2048,
          cpu_priority: parseInt(document.getElementById('workerCpuPriority').value, 10) || 5,
          gpu_device: document.getElementById('workerGpuDevice').value ? parseInt(document.getElementById('workerGpuDevice').value, 10) : null,
          auto_restart: document.getElementById('workerAutoRestart').checked,
          resource_monitoring: document.getElementById('workerResourceMonitoring').checked
        };
        
        const result = await fetchJson('/api/v1/workers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Worker created successfully', 'success');
        hideModal('createWorkerModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshWorkers();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleWorkerDelete(workerId) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to delete worker "${workerId}"? This action cannot be undone.`,
        async () => {
          try {
            showLoading('data', 'Deleting worker...');
            
            await fetchJson(`/api/v1/workers/${workerId}`, {
              method: 'DELETE'
            });
            
            showNotification('Worker deleted successfully', 'success');
            setTimeout(() => {
              window.refreshWorkers();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('data');
          }
        }
      );
    }
    
    async function refresh() {
      await window.refreshWorkers();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "Workers",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/workers</code></div>
  <button class="btn btn-primary" onclick="showCreateWorkerModal()" aria-label="Create new worker">Create Worker</button>
</div>
<div id="data"></div>

<!-- Create Worker Modal -->
<div id="createWorkerModal" class="modal" role="dialog" aria-labelledby="createWorkerModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createWorkerModalTitle">Create Worker</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createWorkerModal')">&times;</button>
    </div>
    <form id="createWorkerForm" onsubmit="handleCreateWorker(event)">
      <div class="form-group">
        <label for="workerId">Worker ID</label>
        <input type="text" id="workerId" name="worker_id" required placeholder="worker-1" />
      </div>
      <div class="form-group">
        <label for="workerMaxConcurrent">Max Concurrent Requests</label>
        <input type="number" id="workerMaxConcurrent" name="max_concurrent_requests" min="1" max="100" value="10" />
      </div>
      <div class="form-group">
        <label for="workerTimeout">Request Timeout (ms)</label>
        <input type="number" id="workerTimeout" name="request_timeout_ms" min="1000" max="60000" value="5000" />
      </div>
      <div class="form-group">
        <label for="workerHealthInterval">Health Check Interval (ms)</label>
        <input type="number" id="workerHealthInterval" name="health_check_interval_ms" min="100" max="10000" value="1000" />
      </div>
      <div class="form-group">
        <label for="workerMaxMemory">Max Memory (MB)</label>
        <input type="number" id="workerMaxMemory" name="max_memory_mb" min="256" max="131072" value="2048" />
      </div>
      <div class="form-group">
        <label for="workerCpuPriority">CPU Priority (1-10)</label>
        <input type="number" id="workerCpuPriority" name="cpu_priority" min="1" max="10" value="5" />
      </div>
      <div class="form-group">
        <label for="workerGpuDevice">GPU Device ID (optional)</label>
        <input type="number" id="workerGpuDevice" name="gpu_device" min="0" placeholder="Leave empty for no GPU" />
      </div>
      <div class="form-group">
        <label for="workerEnableCache">
          <input type="checkbox" id="workerEnableCache" name="enable_caching" checked />
          Enable Caching
        </label>
      </div>
      <div class="form-group">
        <label for="workerCacheSize">Cache Size</label>
        <input type="number" id="workerCacheSize" name="cache_size" min="100" max="10000" value="1000" />
      </div>
      <div class="form-group">
        <label for="workerAutoRestart">
          <input type="checkbox" id="workerAutoRestart" name="auto_restart" checked />
          Auto Restart on Failure
        </label>
      </div>
      <div class="form-group">
        <label for="workerResourceMonitoring">
          <input type="checkbox" id="workerResourceMonitoring" name="resource_monitoring" checked />
          Resource Monitoring
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createWorkerModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), workers_js),
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
          updateBtn.setAttribute('aria-label', `Update library ${libName}`);
          updateBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          updateBtn.onclick = () => handleLibraryAction(libName, 'update');
          actionsTd.appendChild(updateBtn);
          
          // Uninstall button
          const uninstallBtn = document.createElement('button');
          uninstallBtn.className = 'btn btn-danger';
          uninstallBtn.textContent = 'Uninstall';
          uninstallBtn.setAttribute('aria-label', `Uninstall library ${libName}`);
          uninstallBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          uninstallBtn.onclick = () => handleLibraryUninstall(libName);
          actionsTd.appendChild(uninstallBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
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
  <button class="btn btn-primary" onclick="showCreateVmModal()" aria-label="Create new VM instance">Create VM Instance</button>
</div>
<div id="data"></div>

<!-- Create VM Modal -->
<div id="createVmModal" class="modal" role="dialog" aria-labelledby="createVmModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createVmModalTitle">Create VM Instance</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createVmModal')">&times;</button>
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
    let raid_js = r#"
    window.refreshRaidArtifacts = async function() {
      await poll('/api/v1/raid/artifacts', renderRaidArtifactsTable, 'artifacts');
    };
    
    function renderRaidArtifactsTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No artifacts stored.</div>';
        return;
      }
      
      const table = document.createElement('table');
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      ['id', 'name', 'stored_at', 'actions'].forEach(k => {
        const th = document.createElement('th');
        th.textContent = k.charAt(0).toUpperCase() + k.slice(1).replace('_', ' ');
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      for (const artifact of data) {
        const tr = document.createElement('tr');
        
        ['id', 'name', 'stored_at'].forEach(k => {
          const td = document.createElement('td');
          let v = artifact ? artifact[k] : null;
          if (k === 'stored_at' && v) {
            try {
              const date = new Date(v);
              v = date.toLocaleString();
            } catch (e) {
              // Keep original value
            }
          }
          td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
          tr.appendChild(td);
        });
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const artifactId = artifact.id;
        const artifactName = artifact.name || artifactId;
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Delete button
          const deleteBtn = document.createElement('button');
          deleteBtn.className = 'btn btn-danger';
          deleteBtn.textContent = 'Delete';
          deleteBtn.setAttribute('aria-label', `Delete artifact ${artifactName}`);
          deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          deleteBtn.onclick = () => handleArtifactDelete(artifactId, artifactName);
          actionsTd.appendChild(deleteBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
    }
    
    async function showCreateArtifactModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createArtifactModal');
    }
    
    async function handleCreateArtifact(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createArtifactForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const name = document.getElementById('artifactName').value;
        const fileInput = document.getElementById('artifactFile');
        
        if (!fileInput.files || fileInput.files.length === 0) {
          showNotification('Please select a file to upload.', 'error');
          btn.disabled = false;
          btn.textContent = originalText;
          return;
        }
        
        const file = fileInput.files[0];
        const reader = new FileReader();
        
        reader.onload = async function(e) {
          try {
            const arrayBuffer = e.target.result;
            const base64 = btoa(String.fromCharCode(...new Uint8Array(arrayBuffer)));
            
            const result = await fetchJson('/api/v1/raid/artifacts', {
              method: 'POST',
              body: JSON.stringify({
                name: name,
                data: base64
              })
            });
            
            showNotification('Artifact created successfully', 'success');
            hideModal('createArtifactModal');
            form.reset();
            
            setTimeout(() => {
              window.refreshRaidArtifacts();
            }, 500);
          } catch (err) {
            showNotification('Error: ' + err.message, 'error');
          } finally {
            btn.disabled = false;
            btn.textContent = originalText;
          }
        };
        
        reader.onerror = function() {
          showNotification('Error reading file', 'error');
          btn.disabled = false;
          btn.textContent = originalText;
        };
        
        reader.readAsArrayBuffer(file);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleArtifactDelete(artifactId, artifactName) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to delete artifact "${artifactName}" (${artifactId})? This action cannot be undone.`,
        async () => {
          try {
            showLoading('artifacts', 'Deleting artifact...');
            
            await fetchJson(`/api/v1/raid/artifacts/${artifactId}`, {
              method: 'DELETE'
            });
            
            showNotification('Artifact deleted successfully', 'success');
            setTimeout(() => {
              window.refreshRaidArtifacts();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('artifacts');
          }
        }
      );
    }
    
    async function refresh() {
      await poll('/api/v1/raid/nodes', renderTable, 'nodes');
      await window.refreshRaidArtifacts();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "RAID",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Artifacts: <code>/api/v1/raid/artifacts</code></div>
  <button class="btn btn-primary" onclick="showCreateArtifactModal()" aria-label="Create new artifact">Create Artifact</button>
</div>

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

<!-- Create Artifact Modal -->
<div id="createArtifactModal" class="modal" role="dialog" aria-labelledby="createArtifactModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createArtifactModalTitle">Create Artifact</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createArtifactModal')">&times;</button>
    </div>
    <form id="createArtifactForm" onsubmit="handleCreateArtifact(event)">
      <div class="form-group">
        <label for="artifactName">Artifact Name</label>
        <input type="text" id="artifactName" name="name" required placeholder="my-artifact" />
      </div>
      <div class="form-group">
        <label for="artifactFile">File</label>
        <input type="file" id="artifactFile" name="file" required />
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createArtifactModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), raid_js),
    )
}

pub async fn initialize() -> Result<(), AppError> {
    UiManager::new().initialize().await
}

pub async fn shutdown() -> Result<(), AppError> {
    UiManager::new().shutdown().await
}
