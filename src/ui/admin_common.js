// Admin Panel Common JavaScript

/** i18n (FM-012): `PoolAiI18n` from i18n_core.js loaded before this file. */
function poolaiT(key, enFallback) {
  if (typeof PoolAiI18n !== 'undefined' && PoolAiI18n.t) {
    const v = PoolAiI18n.t(key);
    if (v !== key || enFallback === undefined) return v;
  }
  return enFallback !== undefined ? enFallback : key;
}

/** PH-S12 / PH-S14: dark + light + high-contrast tokens (aligned with themes.rs). */
const POOLAI_UI_THEMES = {
  dark: {
    bg: '#0f1216',
    surface: '#171b22',
    surfaceSecondary: '#1e2329',
    text: '#e8e8e8',
    textMuted: '#a8b0bf',
    border: '#262b36',
    primary: '#67e480',
    primaryHover: '#50fa7b',
    secondary: '#6272a4',
    secondaryHover: '#7a8bc4',
    danger: '#c62828',
    dangerHover: '#e53935',
    warning: '#ffb86c',
    info: '#8be9fd',
    success: '#50fa7b',
    link: '#77c7ff',
    linkHover: '#8bd5ff',
  },
  light: {
    bg: '#ffffff',
    surface: '#f5f5f5',
    surfaceSecondary: '#e8e8e8',
    text: '#1a1a1a',
    textMuted: '#666666',
    border: '#d0d0d0',
    primary: '#00a86b',
    primaryHover: '#00c47a',
    secondary: '#6c757d',
    secondaryHover: '#5a6268',
    danger: '#dc3545',
    dangerHover: '#c82333',
    warning: '#ffc107',
    info: '#17a2b8',
    success: '#28a745',
    link: '#007bff',
    linkHover: '#0056b3',
  },
  'high-contrast': {
    bg: '#000000',
    surface: '#1a1a1a',
    surfaceSecondary: '#000000',
    text: '#ffffff',
    textMuted: '#e0e0e0',
    border: '#ffffff',
    primary: '#00ff00',
    primaryHover: '#00cc00',
    secondary: '#ffff00',
    secondaryHover: '#cccc00',
    danger: '#ff0000',
    dangerHover: '#cc0000',
    warning: '#ffff00',
    info: '#00ffff',
    success: '#00ff00',
    link: '#00aaff',
    linkHover: '#0088cc',
  },
};

function poolaiNormalizeTheme(name) {
  if (name === 'light' || name === 'high-contrast') return name;
  return 'dark';
}

function poolaiApplyTheme(themeName) {
  const normalized = poolaiNormalizeTheme(themeName);
  const theme = POOLAI_UI_THEMES[normalized] || POOLAI_UI_THEMES.dark;
  const root = document.documentElement;
  root.style.setProperty('--bg', theme.bg);
  root.style.setProperty('--surface', theme.surface);
  root.style.setProperty('--surface-secondary', theme.surfaceSecondary);
  root.style.setProperty('--text', theme.text);
  root.style.setProperty('--text-muted', theme.textMuted);
  root.style.setProperty('--border', theme.border);
  root.style.setProperty('--primary', theme.primary);
  root.style.setProperty('--primary-hover', theme.primaryHover);
  root.style.setProperty('--secondary', theme.secondary);
  root.style.setProperty('--secondary-hover', theme.secondaryHover);
  root.style.setProperty('--danger', theme.danger);
  root.style.setProperty('--danger-hover', theme.dangerHover);
  root.style.setProperty('--warning', theme.warning);
  root.style.setProperty('--info', theme.info);
  root.style.setProperty('--success', theme.success);
  root.style.setProperty('--link', theme.link);
  root.style.setProperty('--link-hover', theme.linkHover);
  root.dataset.poolaiTheme = normalized;
}

function poolaiInitThemeFromStorage() {
  let name = 'dark';
  try {
    name = localStorage.getItem('poolai_theme') || 'dark';
  } catch (e) {
    name = 'dark';
  }
  poolaiApplyTheme(poolaiNormalizeTheme(name));
}

window.poolaiApplyTheme = poolaiApplyTheme;
window.poolaiInitThemeFromStorage = poolaiInitThemeFromStorage;
window.poolaiNormalizeTheme = poolaiNormalizeTheme;

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

/** PH-S09: apply canonical admin-table / admin-form classes to dynamic markup. */
function adminApplyDesignSystem(root) {
  const scope =
    root && typeof root.querySelectorAll === 'function'
      ? root
      : document.getElementById('admin_main_content') || document;

  scope.querySelectorAll('table').forEach((table) => {
    if (!table.closest('.admin-wrapper')) return;
    table.classList.add('admin-table', 'admin-table--striped');
    const parent = table.parentElement;
    if (parent && !parent.classList.contains('admin-table-container')) {
      const wrap = document.createElement('div');
      wrap.className = 'admin-table-container';
      parent.insertBefore(wrap, table);
      wrap.appendChild(table);
    }
  });

  scope.querySelectorAll('form').forEach((form) => {
    if (!form.closest('.admin-wrapper')) return;
    form.classList.add('admin-form');
  });
}

function adminEscapeRegex(str) {
  return String(str).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function adminTableDataRows(table) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return [];
  return Array.from(
    tbody.querySelectorAll('tr:not(.no-results-row):not(.search-status-row)'),
  );
}

function adminFilterTable(table, query, options) {
  const opts = options || {};
  const tbody = table.querySelector('tbody');
  if (!tbody) return 0;

  const rows = adminTableDataRows(table);
  let visibleCount = 0;
  const highlightMatches = opts.highlightMatches !== false;
  const matchColumns = opts.matchColumns || null;
  const q = (query || '').toLowerCase().trim();

  rows.forEach((row) => {
    let matches;
    if (matchColumns && Array.isArray(matchColumns)) {
      const rowText = Array.from(row.cells)
        .filter((cell, index) => matchColumns.includes(index))
        .map((cell) => cell.textContent)
        .join(' ')
        .toLowerCase();
      matches = !q || rowText.includes(q);
    } else {
      matches = !q || row.textContent.toLowerCase().includes(q);
    }

    if (matches) {
      row.style.display = '';
      row.setAttribute('aria-hidden', 'false');
      visibleCount++;

      if (highlightMatches && q) {
        row.querySelectorAll('td').forEach((cell) => {
          const originalText = cell.dataset.originalText || cell.textContent;
          if (!cell.dataset.originalText) {
            cell.dataset.originalText = cell.textContent;
          }
          const regex = new RegExp('(' + adminEscapeRegex(q) + ')', 'gi');
          const highlighted = originalText.replace(
            regex,
            '<mark class="admin-table-highlight">$1</mark>',
          );
          cell.innerHTML = highlighted;
        });
      } else {
        row.querySelectorAll('td').forEach((cell) => {
          if (cell.dataset.originalText) {
            cell.textContent = cell.dataset.originalText;
            delete cell.dataset.originalText;
          }
        });
      }
    } else {
      row.style.display = 'none';
      row.setAttribute('aria-hidden', 'true');
    }
  });

  let noResultsRow = tbody.querySelector('.no-results-row');
  if (visibleCount === 0 && q) {
    const colCount = table.querySelectorAll('thead th').length || 1;
    if (!noResultsRow) {
      noResultsRow = document.createElement('tr');
      noResultsRow.className = 'no-results-row';
      noResultsRow.setAttribute('role', 'status');
      noResultsRow.setAttribute('aria-live', 'polite');
      const td = document.createElement('td');
      td.colSpan = colCount;
      noResultsRow.appendChild(td);
      tbody.appendChild(noResultsRow);
    }
    noResultsRow.querySelector('td').textContent = poolaiT(
      'ui.searchNoResultsFor',
      'No results found for "{query}"',
    ).replace('{query}', query);
    noResultsRow.style.display = '';
  } else if (noResultsRow) {
    noResultsRow.style.display = 'none';
  }

  return visibleCount;
}

function adminUpdateTableSearchStatus(table, visibleCount, totalCount, query) {
  const existingStatus = table.querySelector('.search-status-row');
  if (existingStatus) existingStatus.remove();

  if (!query) return;

  const tbody = table.querySelector('tbody');
  const colCount = table.querySelectorAll('thead th').length || 1;
  if (!tbody) return;

  const statusRow = document.createElement('tr');
  statusRow.className = 'search-status-row';
  statusRow.setAttribute('role', 'status');
  statusRow.setAttribute('aria-live', 'polite');
  const statusCell = document.createElement('td');
  statusCell.colSpan = colCount;
  statusCell.textContent = poolaiT('ui.searchStatusSimple', '{visible} of {total} results')
    .replace('{visible}', String(visibleCount))
    .replace('{total}', String(totalCount));
  statusRow.appendChild(statusCell);
  tbody.appendChild(statusRow);
}

function adminSortTable(table, columnIndex, ascending) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return;

  const rows = adminTableDataRows(table);
  const isNumeric = rows.every((row) => {
    const cell = row.cells[columnIndex];
    return cell && !isNaN(parseFloat(cell.textContent));
  });

  rows.sort((a, b) => {
    const aCell = a.cells[columnIndex];
    const bCell = b.cells[columnIndex];
    if (!aCell || !bCell) return 0;

    const aValue = isNumeric
      ? parseFloat(aCell.textContent)
      : aCell.textContent.trim().toLowerCase();
    const bValue = isNumeric
      ? parseFloat(bCell.textContent)
      : bCell.textContent.trim().toLowerCase();

    if (aValue < bValue) return ascending ? -1 : 1;
    if (aValue > bValue) return ascending ? 1 : -1;
    return 0;
  });

  rows.forEach((row) => row.remove());
  rows.forEach((row) => tbody.appendChild(row));

  table.querySelectorAll('thead th').forEach((header, index) => {
    if (index === columnIndex) {
      header.setAttribute('data-sort', ascending ? 'asc' : 'desc');
      header.setAttribute('aria-sort', ascending ? 'ascending' : 'descending');
    } else {
      header.removeAttribute('data-sort');
      header.setAttribute('aria-sort', 'none');
    }
  });
}

function adminInitTableSorting(table) {
  if (!table || table.dataset.poolaiTableSort === '1') return;
  table.dataset.poolaiTableSort = '1';

  table.querySelectorAll('thead th').forEach((header, index) => {
    if (header.dataset.noSort === '1' || header.classList.contains('admin-table-actions-col')) {
      return;
    }
    const label = (header.textContent || '').trim().toLowerCase();
    if (label === 'actions' || label === 'дії') {
      header.classList.add('admin-table-actions-col');
      return;
    }

    header.classList.add('admin-table-sortable');
    header.setAttribute('tabindex', '0');
    header.setAttribute('role', 'columnheader');
    header.setAttribute('aria-sort', 'none');

    const activateSort = () => {
      const currentSort = header.getAttribute('data-sort');
      const ascending = currentSort !== 'asc';
      adminSortTable(table, index, ascending);
      adminAnnounceLive(
        poolaiT('admin.table.sortedBy', 'Sorted by {column} {direction}')
          .replace('{column}', header.textContent.trim())
          .replace('{direction}', ascending ? '↑' : '↓'),
        'polite',
      );
    };

    header.addEventListener('click', activateSort);
    header.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        activateSort();
      }
    });
  });
}

function adminTableVisibleRows(table) {
  return adminTableDataRows(table).filter(
    (row) => row.style.display !== 'none' && row.getAttribute('aria-hidden') !== 'true',
  );
}

function adminExportTableCsv(table, filename) {
  const headers = Array.from(table.querySelectorAll('thead th')).map((th) =>
    th.textContent.trim(),
  );
  const rows = adminTableVisibleRows(table).map((row) =>
    Array.from(row.cells).map((cell) => {
      const text = cell.dataset.originalText || cell.textContent.trim();
      if (/[",\n]/.test(text)) return '"' + text.replace(/"/g, '""') + '"';
      return text;
    }),
  );
  const lines = [headers.join(',')].concat(rows.map((r) => r.join(',')));
  const blob = new Blob([lines.join('\n')], { type: 'text/csv;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || 'poolai-export.csv';
  a.click();
  URL.revokeObjectURL(url);
  showNotification(poolaiT('admin.table.exportedCsv', 'Table exported as CSV'), 'success');
}

function adminExportTableJson(table, filename) {
  const headers = Array.from(table.querySelectorAll('thead th')).map((th) =>
    th.textContent.trim(),
  );
  const data = adminTableVisibleRows(table).map((row) => {
    const obj = {};
    headers.forEach((h, i) => {
      const cell = row.cells[i];
      obj[h] = cell ? (cell.dataset.originalText || cell.textContent.trim()) : '';
    });
    return obj;
  });
  const blob = new Blob([JSON.stringify(data, null, 2)], {
    type: 'application/json;charset=utf-8',
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || 'poolai-export.json';
  a.click();
  URL.revokeObjectURL(url);
  showNotification(poolaiT('admin.table.exportedJson', 'Table exported as JSON'), 'success');
}

function adminCreateTableToolbar(table, options) {
  const opts = options || {};
  const container = table.closest('.admin-table-container') || table.parentElement;
  if (!container || container.querySelector('.admin-table-toolbar')) return null;

  const toolbar = document.createElement('div');
  toolbar.className = 'admin-table-toolbar';

  let searchInput = null;
  if (!opts.noFilter) {
    const searchWrap = document.createElement('div');
    searchWrap.className = 'admin-table-search';
    searchInput = document.createElement('input');
    searchInput.type = 'search';
    searchInput.className = 'admin-table-search-input';
    searchInput.placeholder = poolaiT('admin.table.searchPh', 'Filter table…');
    searchInput.setAttribute('aria-label', poolaiT('ui.searchTableAria', 'Search table'));
    searchInput.setAttribute('role', 'searchbox');
    searchWrap.appendChild(searchInput);
    toolbar.appendChild(searchWrap);
  }

  const actions = document.createElement('div');
  actions.className = 'admin-table-toolbar-actions';

  if (!opts.noExport) {
    const csvBtn = document.createElement('button');
    csvBtn.type = 'button';
    csvBtn.className = 'btn btn-secondary btn-sm';
    csvBtn.textContent = poolaiT('admin.table.exportCsv', 'Export CSV');
    csvBtn.setAttribute(
      'aria-label',
      poolaiT('admin.table.exportCsvAria', 'Export visible rows as CSV'),
    );
    csvBtn.addEventListener('click', () => {
      const name =
        (table.getAttribute('aria-label') || 'poolai-table').replace(/\s+/g, '-').toLowerCase() +
        '.csv';
      adminExportTableCsv(table, name);
    });

    const jsonBtn = document.createElement('button');
    jsonBtn.type = 'button';
    jsonBtn.className = 'btn btn-secondary btn-sm';
    jsonBtn.textContent = poolaiT('admin.table.exportJson', 'Export JSON');
    jsonBtn.setAttribute(
      'aria-label',
      poolaiT('admin.table.exportJsonAria', 'Export visible rows as JSON'),
    );
    jsonBtn.addEventListener('click', () => {
      const name =
        (table.getAttribute('aria-label') || 'poolai-table').replace(/\s+/g, '-').toLowerCase() +
        '.json';
      adminExportTableJson(table, name);
    });

    actions.appendChild(csvBtn);
    actions.appendChild(jsonBtn);
  }

  toolbar.appendChild(actions);
  container.insertBefore(toolbar, table);

  if (searchInput) {
    adminBindTableSearch(searchInput, table, opts.filterOptions);
  }

  return toolbar;
}

function adminBindTableSearch(searchInputOrId, tableOrId, filterOptions) {
  const searchInput =
    typeof searchInputOrId === 'string'
      ? document.getElementById(searchInputOrId)
      : searchInputOrId;
  const table =
    typeof tableOrId === 'string' ? document.getElementById(tableOrId) : tableOrId;
  if (!searchInput || !table) return;

  const opts = filterOptions || {};
  const debounceDelay = opts.debounceDelay || 300;
  let debounceTimer = null;
  const totalCount = adminTableDataRows(table).length;

  searchInput.setAttribute('role', 'searchbox');
  if (table.id) searchInput.setAttribute('aria-controls', table.id);

  searchInput.addEventListener('input', function (e) {
    const query = e.target.value.trim();
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      const visibleCount = adminFilterTable(table, query, opts);
      adminUpdateTableSearchStatus(table, visibleCount, totalCount, query);
      adminAnnounceLive(
        query
          ? poolaiT('ui.searchStatusFound', '{visible} of {total} results found')
              .replace('{visible}', String(visibleCount))
              .replace('{total}', String(totalCount))
          : poolaiT('ui.searchStatusAll', 'All results shown'),
        'polite',
      );
    }, debounceDelay);
  });

  searchInput.addEventListener('keydown', function (e) {
    if (e.key === 'Escape' && searchInput.value) {
      searchInput.value = '';
      adminFilterTable(table, '', opts);
      adminUpdateTableSearchStatus(table, totalCount, totalCount, '');
      searchInput.focus();
    }
  });
}

function adminEnhanceAdminTable(table, options) {
  if (!table || table.dataset.poolaiTableEnhanced === '1') return;
  if (table.classList.contains('topology-heatmap-table')) return;
  if (table.dataset.poolaiTableStatic === '1') return;

  const opts = options || {};
  table.dataset.poolaiTableEnhanced = '1';

  if (!opts.noSort) adminInitTableSorting(table);
  if (!opts.noToolbar && !opts.externalSearchEl) {
    adminCreateTableToolbar(table, opts);
  } else if (opts.externalSearchEl) {
    adminBindTableSearch(opts.externalSearchEl, table, opts.filterOptions);
  }
}

function adminInitTablesIn(root) {
  const scope =
    root && typeof root.querySelectorAll === 'function'
      ? root
      : document.getElementById('admin_main_content') || document;
  scope.querySelectorAll('table.admin-table').forEach((table) => {
    adminEnhanceAdminTable(table);
  });
}

window.adminEmptyStateHtml = adminEmptyStateHtml;
window.adminFilterTable = adminFilterTable;
window.adminSortTable = adminSortTable;
window.adminInitTableSorting = adminInitTableSorting;
window.adminBindTableSearch = adminBindTableSearch;
window.adminEnhanceAdminTable = adminEnhanceAdminTable;
window.adminInitTablesIn = adminInitTablesIn;
window.adminExportTableCsv = adminExportTableCsv;
window.adminExportTableJson = adminExportTableJson;

/** PH-S42: canonical empty state for admin lists/tables. */
function adminEmptyStateHtml(message, options) {
  const opts = options || {};
  const title = escapeHtml(message || poolaiT('admin.table.empty', 'No data to display'));
  const hint = opts.hint
    ? '<p class="admin-empty-state-hint">' + escapeHtml(opts.hint) + '</p>'
    : '';
  const action = opts.actionHtml
    ? '<div class="admin-empty-state-action">' + opts.actionHtml + '</div>'
    : '';
  return (
    '<div class="admin-empty-state" role="status">' +
    '<div class="admin-empty-state-icon" aria-hidden="true">' +
    (opts.icon || '📋') +
    '</div>' +
    '<p class="admin-empty-state-title">' +
    title +
    '</p>' +
    hint +
    action +
    '</div>'
  );
}

/** PH-S09 / PH-S42: build a striped data table from header labels and row cell HTML. */
function adminRenderTable(headers, rows, options) {
  const opts = options || {};
  if (!rows || rows.length === 0) {
    return adminEmptyStateHtml(
      opts.emptyMessage || poolaiT('admin.table.empty', 'No data to display'),
      opts.emptyOptions,
    );
  }
  const cols = (headers || []).map((h) =>
    typeof h === 'string' ? { label: h } : h,
  );
  let html = '<div class="admin-table-container"><table class="admin-table admin-table--striped">';
  html += '<thead><tr>';
  cols.forEach((c) => {
    const noSort = c.noSort ? ' data-no-sort="1"' : '';
    const cls = c.actions ? ' class="admin-table-actions-col"' : '';
    html += '<th scope="col"' + noSort + cls + '>' + escapeHtml(c.label || '') + '</th>';
  });
  html += '</tr></thead><tbody>';
  (rows || []).forEach((row) => {
    html += '<tr>';
    (row || []).forEach((cell) => {
      html += '<td>' + (cell == null ? '' : cell) + '</td>';
    });
    html += '</tr>';
  });
  html += '</tbody></table></div>';
  return html;
}

/** PH-S09: one labeled field using design-system form-group markup. */
function adminFormFieldHtml(spec) {
  const id =
    spec.id ||
    'fld_' + Math.random().toString(36).slice(2, 11);
  const name = escapeHtml(spec.name || id);
  const required = spec.required
    ? ' required aria-required="true"'
    : '';
  let label =
    '<label for="' +
    id +
    '">' +
    escapeHtml(spec.label || '');
  if (spec.required) {
    label += ' <span class="required" aria-hidden="true">*</span>';
  }
  label += '</label>';

  let control = '';
  if (spec.type === 'select') {
    control =
      '<select id="' +
      id +
      '" name="' +
      name +
      '"' +
      required +
      '>';
    (spec.options || []).forEach((o) => {
      control +=
        '<option value="' +
        escapeHtml(o.value) +
        '">' +
        escapeHtml(o.label) +
        '</option>';
    });
    control += '</select>';
  } else if (spec.type === 'textarea') {
    control =
      '<textarea id="' +
      id +
      '" name="' +
      name +
      '"' +
      required +
      '></textarea>';
  } else {
    control =
      '<input type="' +
      escapeHtml(spec.type || 'text') +
      '" id="' +
      id +
      '" name="' +
      name +
      '"' +
      required;
    if (spec.placeholder) {
      control +=
        ' placeholder="' + escapeHtml(spec.placeholder) + '"';
    }
    control += ' />';
  }
  return '<div class="form-group">' + label + control + '</div>';
}

function adminObserveDynamicA11y() {
  const root = document.getElementById('admin_main_content');
  if (!root || root.dataset.poolaiA11yObs === '1') return;
  const obs = new MutationObserver(function () {
    adminApplyDesignSystem(root);
    adminEnhanceTablesA11y(root);
    adminEnhanceFormA11y(root);
    adminInitTablesIn(root);
  });
  obs.observe(root, { childList: true, subtree: true });
  root.dataset.poolaiA11yObs = '1';
  adminApplyDesignSystem(root);
  adminEnhanceTablesA11y(root);
  adminEnhanceFormA11y(root);
  adminInitTablesIn(root);
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
  poolaiInitThemeFromStorage();
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
