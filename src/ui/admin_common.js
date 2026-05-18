// Admin Panel Common JavaScript

/** i18n (FM-012): `PoolAiI18n` from i18n_core.js loaded before this file. */
function poolaiT(key, enFallback) {
  if (typeof PoolAiI18n !== 'undefined' && PoolAiI18n.t) {
    const v = PoolAiI18n.t(key);
    if (v !== key || enFallback === undefined) return v;
  }
  return enFallback !== undefined ? enFallback : key;
}

// API base URL
const API_BASE = '/api/v1';

// Enterprise API base URL
const ENTERPRISE_API_BASE = '/api/enterprise';

/** Parse API error body: legacy flat `error` string or `{ error: { code, message } }`. */
function apiErrorMessageFromBody(payload) {
  if (!payload || typeof payload !== 'object') return null;
  const e = payload.error;
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && typeof e.message === 'string') return e.message;
  if (typeof payload.message === 'string') return payload.message;
  return null;
}

function apiErrorDetailFromBody(payload) {
  const message = apiErrorMessageFromBody(payload);
  let code = null;
  let hint = null;
  if (payload && typeof payload === 'object') {
    const e = payload.error;
    if (e && typeof e === 'object' && typeof e.code === 'string') code = e.code;
    const ctx = payload.context;
    if (ctx && typeof ctx === 'object' && typeof ctx.hint === 'string') hint = ctx.hint;
  }
  return { message, code, hint };
}

function hintFor503(code, message) {
  if (code === 'RAID_MANAGER_UNAVAILABLE') {
    return poolaiT(
      'err.hint503.raid',
      'RAID manager is not initialized on this server.',
    );
  }
  const m = message || '';
  if (/library/i.test(m))
    return poolaiT('err.hint503.library', 'Library subsystem may not be initialized.');
  if (/\bvm\b/i.test(m)) return poolaiT('err.hint503.vm', 'VM manager may not be attached.');
  return poolaiT('err.hint503.generic', 'A subsystem may still be starting or unavailable.');
}

function formatFetchError(status, url, payload) {
  const { message, code, hint } = apiErrorDetailFromBody(payload);
  const base = message || ('HTTP ' + status);
  let extra = hint || '';
  if (status === 403 && !extra) {
    extra = poolaiT(
      'err.hint403',
      'You may need Admin or Operator role, or sign in again.',
    );
  }
  if (status === 503 && !extra) {
    extra = hintFor503(code, base);
  }
  if (status === 404 && url && url.indexOf('/api/enterprise') !== -1 && !extra) {
    extra = poolaiT(
      'err.hint404.enterprise',
      'Build and run the server with the enterprise feature for this API.',
    );
  }
  if (extra) return base + ' — ' + extra;
  return base;
}

function escapeHtml(s) {
  return String(s)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function adminShowLoading(containerId, text) {
  const el = document.getElementById(containerId);
  if (el)
    el.innerHTML =
      '<div class="muted">' +
      escapeHtml(text || poolaiT('common.loading', 'Loading…')) +
      '</div>';
}

function adminAnnounceLive(message, priority) {
  const live = document.getElementById('admin-aria-live');
  if (!live || !message) return;
  live.setAttribute('aria-live', priority === 'assertive' ? 'assertive' : 'polite');
  live.textContent = message;
}

function adminShowInlineError(containerId, err) {
  const el = document.getElementById(containerId);
  if (!el) return;
  const msg = err instanceof Error ? err.message : String(err);
  el.innerHTML =
    '<div class="admin-fetch-error" role="alert">' + escapeHtml(msg) + '</div>';
  adminAnnounceLive(msg, 'assertive');
}

async function adminRefreshAccessToken() {
  try {
    const token = localStorage.getItem('poolai_token');
    if (!token) return false;
    const res = await fetch('/api/v1/refresh', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: 'Bearer ' + token,
      },
    });
    if (!res.ok) return false;
    const data = await res.json();
    if (!data.token) return false;
    localStorage.setItem('poolai_token', data.token);
    try {
      if (data.bootstrap_default_admin === true) {
        localStorage.setItem('poolai_bootstrap_admin_show', '1');
      } else if (data.bootstrap_default_admin === false) {
        localStorage.removeItem('poolai_bootstrap_admin_show');
      }
    } catch (e) { /* ignore */ }
    if (data.role) {
      const username = localStorage.getItem('poolai_user');
      if (username) localStorage.setItem('poolai_role', data.role);
    }
    if (data.expires_in) {
      const exp = Math.floor(Date.now() / 1000) + data.expires_in;
      localStorage.setItem('poolai_token_exp', exp.toString());
    }
    return true;
  } catch (e) {
    return false;
  }
}

// Utility functions
// Compatible with main UI module storage format
function getUser() {
  // Try new format first (poolai_user, poolai_role)
  const username = localStorage.getItem('poolai_user');
  const role = localStorage.getItem('poolai_role');
  const token = localStorage.getItem('poolai_token');
  
  if (username && role) {
    return { username, role, token };
  }
  
  // Fallback to old format (user JSON)
  const userStr = localStorage.getItem('user');
  if (!userStr) return null;
  try {
    return JSON.parse(userStr);
  } catch (e) {
    return null;
  }
}

function setUser(user) {
  if (typeof user === 'string') {
    // Legacy format: setUser(username, role) - called from login
    const role = arguments[1] || 'Viewer';
    localStorage.setItem('poolai_user', user);
    localStorage.setItem('poolai_role', role);
  } else if (user && typeof user === 'object') {
    // New format: setUser({username, role, token})
    if (user.username) localStorage.setItem('poolai_user', user.username);
    if (user.role) localStorage.setItem('poolai_role', user.role);
    if (user.token) localStorage.setItem('poolai_token', user.token);
    // Also store in old format for compatibility
    localStorage.setItem('user', JSON.stringify(user));
  }
}

function clearUser() {
  localStorage.removeItem('user');
  localStorage.removeItem('poolai_user');
  localStorage.removeItem('poolai_role');
  localStorage.removeItem('poolai_token');
  localStorage.removeItem('poolai_token_exp');
}

function isAdmin() {
  const user = getUser();
  return user && (user.role === 'Admin' || user.role === 'admin');
}

function requireAdmin() {
  const user = getUser();
  if (!user) {
    window.location.href = '/ui/auth';
    return false;
  }
  if (!isAdmin()) {
    alert(poolaiT('admin.accessRequired', 'Admin access required'));
    window.location.href = '/ui/auth';
    return false;
  }
  return true;
}

// API helpers
async function fetchJson(url, options = {}) {
  const user = getUser();
  const token = user?.token || localStorage.getItem('poolai_token');
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = 'Bearer ' + token;
  }

  const doFetch = () =>
    fetch(url, {
      ...options,
      headers,
    });

  let response = await doFetch();

  if (response.status === 401) {
    const refreshed = await adminRefreshAccessToken();
    if (refreshed) {
      const newTok = localStorage.getItem('poolai_token');
      if (newTok) headers['Authorization'] = 'Bearer ' + newTok;
      response = await doFetch();
    }
    if (response.status === 401) {
      clearUser();
      window.location.href = '/ui/auth';
      throw new Error(
        poolaiT(
          'common.unauthorized',
          'Unauthorized — session expired. Please sign in again.',
        ),
      );
    }
  }

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new Error(formatFetchError(response.status, url, error));
  }

  return response.json();
}

// Notification system
function showNotification(message, type = 'info') {
  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.textContent = message;
  notification.style.cssText = `
    position: fixed;
    top: 20px;
    right: 20px;
    padding: 15px 20px;
    background: ${type === 'error' ? '#ff5555' : type === 'success' ? '#67e480' : '#8be9fd'};
    color: ${type === 'error' ? 'white' : '#0f1216'};
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    z-index: 3000;
    animation: slideIn 0.3s ease-out;
  `;
  
  document.body.appendChild(notification);
  
  setTimeout(() => {
    notification.style.animation = 'slideOut 0.3s ease-out';
    setTimeout(() => notification.remove(), 300);
  }, 3000);
}

// Modal system — FM-019: overlay, aria-modal, focus trap, Esc (admin users/security + dynamic)
const MODAL_FOCUSABLE_SELECTOR =
  'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])';
const ADMIN_DYNAMIC_MODAL_ID = 'adminDynamicModal';

let activeModal = null;
let activeOverlay = null;
let previousActiveElement = null;

function getModalFocusableElements(modal) {
  return Array.from(modal.querySelectorAll(MODAL_FOCUSABLE_SELECTOR)).filter(
    (el) => el.offsetParent !== null || el === document.activeElement,
  );
}

function focusInitialModalElement(modal) {
  const focusable = getModalFocusableElements(modal);
  if (focusable.length > 0) {
    focusable[0].focus();
    return;
  }
  if (!modal.hasAttribute('tabindex')) {
    modal.setAttribute('tabindex', '-1');
  }
  modal.focus();
}

function attachModalA11y(modal) {
  modal.removeEventListener('keydown', trapModalFocus);
  modal.removeEventListener('focusin', keepFocusInModal);
  modal.addEventListener('keydown', trapModalFocus);
  modal.addEventListener('focusin', keepFocusInModal);
  document.removeEventListener('keydown', handleModalEscape);
  document.addEventListener('keydown', handleModalEscape);
}

function detachModalA11y(modal) {
  if (modal) {
    modal.removeEventListener('keydown', trapModalFocus);
    modal.removeEventListener('focusin', keepFocusInModal);
  }
  document.removeEventListener('keydown', handleModalEscape);
}

/** Static modal by id, or dynamic: showModal(title, htmlContent) for instances/topology. */
function showModal(modalIdOrTitle, optionalContent) {
  if (typeof optionalContent === 'string') {
    showModalContent(modalIdOrTitle, optionalContent);
    return;
  }
  const modalId = modalIdOrTitle;
  const modal = document.getElementById(modalId);
  if (!modal) {
    console.warn('Modal not found:', modalId);
    return;
  }

  if (activeModal && activeModal !== modal) {
    detachModalA11y(activeModal);
    activeModal.setAttribute('aria-hidden', 'true');
    activeModal.setAttribute('aria-modal', 'false');
  }

  previousActiveElement = document.activeElement;

  let overlay = modal.closest('.modal-overlay');
  if (!overlay) {
    overlay = createModalOverlay(modal);
  }

  modal.setAttribute('aria-hidden', 'false');
  modal.setAttribute('aria-modal', 'true');
  overlay.classList.add('active');
  activeModal = modal;
  activeOverlay = overlay;
  document.body.style.overflow = 'hidden';

  attachModalA11y(modal);
  setTimeout(() => {
    focusInitialModalElement(modal);
    adminEnhanceFormA11y(modal);
  }, 100);
}

function hideModal(modalId) {
  const id = modalId || (activeModal && activeModal.id);
  if (!id) return;
  const modal = document.getElementById(id);
  if (!modal) {
    console.warn('Modal not found:', id);
    return;
  }

  modal.setAttribute('aria-hidden', 'true');
  modal.setAttribute('aria-modal', 'false');

  const overlay = modal.closest('.modal-overlay');
  if (overlay) {
    overlay.classList.remove('active');
  }

  detachModalA11y(modal);

  if (previousActiveElement && typeof previousActiveElement.focus === 'function') {
    previousActiveElement.focus();
  }
  previousActiveElement = null;
  activeModal = null;
  activeOverlay = null;
  document.body.style.overflow = '';
}

function ensureAdminDynamicModal() {
  let modal = document.getElementById(ADMIN_DYNAMIC_MODAL_ID);
  if (modal) return modal;
  modal = document.createElement('div');
  modal.id = ADMIN_DYNAMIC_MODAL_ID;
  modal.className = 'modal';
  modal.setAttribute('role', 'dialog');
  modal.setAttribute('aria-labelledby', 'adminDynamicModalTitle');
  modal.setAttribute('aria-modal', 'false');
  modal.setAttribute('aria-hidden', 'true');
  modal.innerHTML =
    '<div class="modal-content">' +
    '<div class="modal-header">' +
    '<h3 id="adminDynamicModalTitle"></h3>' +
    '<button type="button" class="modal-close" data-i18n-aria="ui.closeDialogAria" onclick="hideModal(\'' +
    ADMIN_DYNAMIC_MODAL_ID +
    '\')">&times;</button>' +
    '</div>' +
    '<div id="adminDynamicModalBody" class="modal-body"></div>' +
    '</div>';
  document.body.appendChild(modal);
  return modal;
}

function showModalContent(title, bodyHtml) {
  ensureAdminDynamicModal();
  const titleEl = document.getElementById('adminDynamicModalTitle');
  const bodyEl = document.getElementById('adminDynamicModalBody');
  if (titleEl) titleEl.textContent = title;
  if (bodyEl) bodyEl.innerHTML = bodyHtml;
  adminEnhanceFormA11y(modal);
  showModal(ADMIN_DYNAMIC_MODAL_ID);
}

function createModalOverlay(modal) {
  let overlay = document.getElementById('modal-overlay-global');
  if (!overlay) {
    overlay = document.createElement('div');
    overlay.id = 'modal-overlay-global';
    overlay.className = 'modal-overlay';
    document.body.appendChild(overlay);
    overlay.addEventListener('click', function (e) {
      if (e.target === overlay && activeModal) {
        hideModal(activeModal.id);
      }
    });
  }
  if (modal.parentElement !== overlay) {
    overlay.appendChild(modal);
  }
  return overlay;
}

function keepFocusInModal(e) {
  if (!activeModal || activeModal.contains(e.target)) return;
  const focusable = getModalFocusableElements(activeModal);
  if (focusable.length > 0) {
    focusable[0].focus();
  } else {
    focusInitialModalElement(activeModal);
  }
}

function trapModalFocus(e) {
  if (!activeModal || e.key !== 'Tab') return;

  const focusable = getModalFocusableElements(activeModal);
  if (focusable.length === 0) {
    e.preventDefault();
    focusInitialModalElement(activeModal);
    return;
  }

  const first = focusable[0];
  const last = focusable[focusable.length - 1];
  const active = document.activeElement;
  const inside = activeModal.contains(active);

  if (e.shiftKey) {
    if (!inside || active === first) {
      e.preventDefault();
      last.focus();
    }
  } else if (!inside || active === last) {
    e.preventDefault();
    first.focus();
  }
}

function handleModalEscape(e) {
  if (e.key === 'Escape' && activeModal) {
    e.preventDefault();
    hideModal(activeModal.id);
  }
}

/** FM-019: sync aria-selected / aria-labelledby after tab change (security, config). */
function adminSyncTabA11y(tablist) {
  if (!tablist) return;
  const panelId = tablist.querySelector('.tab[aria-controls]')?.getAttribute('aria-controls');
  const panel = panelId ? document.getElementById(panelId) : null;
  tablist.querySelectorAll('.tab').forEach((tab) => {
    const selected = tab.classList.contains('active');
    tab.setAttribute('aria-selected', selected ? 'true' : 'false');
    tab.setAttribute('tabindex', selected ? '0' : '-1');
    if (selected && panel && tab.id) {
      panel.setAttribute('aria-labelledby', tab.id);
    }
  });
}

function initTabs() {
  document.querySelectorAll('.admin-tabs').forEach((tablist) => {
    const section = tablist.closest('.admin-section');
    if (!section) return;
    const panel = section.querySelector('#security-content, #config-content');
    if (!panel || !panel.id) return;
    if (!tablist.getAttribute('role')) tablist.setAttribute('role', 'tablist');
    panel.setAttribute('role', 'tabpanel');
    tablist.querySelectorAll('.tab').forEach((tab) => {
      if (!tab.id && tab.dataset.tab) {
        tab.id = panel.id + '-tab-' + tab.dataset.tab;
      }
      if (!tab.getAttribute('role')) tab.setAttribute('role', 'tab');
      if (!tab.getAttribute('aria-controls')) tab.setAttribute('aria-controls', panel.id);
    });
    adminSyncTabA11y(tablist);
  });
}

/** FM-019: scope on th, aria-label from section heading for dynamic admin tables. */
function adminEnhanceTablesA11y(root) {
  const scope = root && typeof root.querySelectorAll === 'function' ? root : document;
  scope.querySelectorAll('table.admin-table').forEach((table) => {
    if (!table.getAttribute('aria-label') && !table.querySelector('caption')) {
      const heading = table.closest('.admin-section, .admin-header')?.querySelector('h3, h2');
      if (heading && heading.textContent.trim()) {
        table.setAttribute('aria-label', heading.textContent.trim());
      }
    }
    table.querySelectorAll('thead th').forEach((th) => {
      if (!th.getAttribute('scope')) th.setAttribute('scope', 'col');
    });
  });
}

function adminObserveDynamicA11y() {
  const root = document.getElementById('admin_main_content');
  if (!root || root.dataset.poolaiA11yObs === '1') return;
  const obs = new MutationObserver(function () {
    adminEnhanceTablesA11y(root);
    adminEnhanceFormA11y(root);
  });
  obs.observe(root, { childList: true, subtree: true });
  root.dataset.poolaiA11yObs = '1';
  adminEnhanceTablesA11y(root);
  adminEnhanceFormA11y(root);
}

// Logout
function logout() {
  clearUser();
  window.location.href = '/ui/auth';
}

/** FM-019: required fields, decorative asterisks, label for= in admin forms. */
function adminEnhanceFormA11y(root) {
  const scope = root && typeof root.querySelectorAll === 'function' ? root : document;
  scope.querySelectorAll('form').forEach((form) => {
    form.querySelectorAll('input, select, textarea').forEach((field) => {
      if (field.hasAttribute('required') && field.getAttribute('aria-required') !== 'true') {
        field.setAttribute('aria-required', 'true');
      }
    });
    form.querySelectorAll('label .required').forEach((star) => {
      star.setAttribute('aria-hidden', 'true');
    });
    form.querySelectorAll('.form-group').forEach((group) => {
      const label = group.querySelector('label:not([for])');
      if (!label) return;
      const control = group.querySelector('input, select, textarea');
      if (control && control.id) {
        label.setAttribute('for', control.id);
      }
    });
  });
}

/** FM-018: highlight current nav link for screen readers and keyboard users. */
function adminMarkCurrentNav() {
  const currentPath = window.location.pathname;
  document.querySelectorAll('.admin-nav-item').forEach(item => {
    const isCurrent = item.getAttribute('href') === currentPath;
    item.classList.toggle('active', isCurrent);
    if (isCurrent) {
      item.setAttribute('aria-current', 'page');
    } else {
      item.removeAttribute('aria-current');
    }
  });
}

function adminShellOnReady() {
  if (!requireAdmin()) return;
  initTabs();
  adminObserveDynamicA11y();
  adminMarkCurrentNav();
  const user = getUser();
  if (user) {
    const userNameEl = document.getElementById('admin-user-name');
    if (userNameEl) {
      userNameEl.textContent = user.username || 'Admin';
    }
  }
  document.addEventListener('poolai:langchange', () => {
    if (typeof PoolAiI18n !== 'undefined') PoolAiI18n.apply(document.body);
    if (typeof PoolAiI18n !== 'undefined') PoolAiI18n.initAdminShell();
    adminMarkCurrentNav();
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', adminShellOnReady);
} else {
  adminShellOnReady();
}

// Add CSS animations
const style = document.createElement('style');
style.textContent = `
  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
  
  @keyframes slideOut {
    from {
      transform: translateX(0);
      opacity: 1;
    }
    to {
      transform: translateX(100%);
      opacity: 0;
    }
  }
`;
document.head.appendChild(style);
