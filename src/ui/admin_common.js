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

function poolaiUiWasmCall(name) {
  const w = window.poolaiUiWasm;
  return w && w.ready && typeof w[name] === 'function' ? w[name] : null;
}

/** Parse API error body via wasm (PH-S273); minimal JS fallback when wasm absent. */
function apiErrorMessageFromBody(payload) {
  const fn = poolaiUiWasmCall('apiErrorMessageFromBody');
  if (fn) {
    const msg = fn(JSON.stringify(payload || {}));
    return msg || null;
  }
  if (!payload || typeof payload !== 'object') return null;
  const e = payload.error;
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && typeof e.message === 'string') return e.message;
  if (typeof payload.message === 'string') return payload.message;
  return null;
}

function apiErrorDetailFromBody(payload) {
  const fn = poolaiUiWasmCall('apiErrorDetailFromBody');
  if (fn) {
    const detail = fn(JSON.stringify(payload || {}));
    if (detail && typeof detail === 'object') {
      return {
        message: detail.message || null,
        code: detail.code || null,
        hint: detail.hint || null,
      };
    }
  }
  return { message: apiErrorMessageFromBody(payload), code: null, hint: null };
}

function formatFetchError(status, url, payload) {
  const fn = poolaiUiWasmCall('formatFetchError');
  if (fn) return fn(status, url || '', JSON.stringify(payload || {}));
  const message = apiErrorMessageFromBody(payload);
  return message || 'HTTP ' + status;
}

function escapeHtml(s) {
  const fn = poolaiUiWasmCall('escapeHtml');
  if (fn) return fn(String(s));
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
  const q = (query || '').trim();
  const matchFn = poolaiUiWasmCall('rowMatchesQuery');
  const highlightFn = poolaiUiWasmCall('highlightQueryHtml');

  rows.forEach((row) => {
    let rowText = row.textContent || '';
    if (matchColumns && Array.isArray(matchColumns)) {
      rowText = Array.from(row.cells)
        .filter((cell, index) => matchColumns.includes(index))
        .map((cell) => cell.textContent)
        .join(' ');
    }
    const matches = matchFn
      ? matchFn(rowText, q)
      : !q || rowText.toLowerCase().includes(q.toLowerCase());

    if (matches) {
      row.style.display = '';
      row.setAttribute('aria-hidden', 'false');
      visibleCount++;
      if (highlightMatches && q) {
        row.querySelectorAll('td').forEach((cell) => {
          const originalText = cell.dataset.originalText || cell.textContent;
          if (!cell.dataset.originalText) cell.dataset.originalText = cell.textContent;
          cell.innerHTML = highlightFn
            ? highlightFn(originalText, q)
            : originalText.replace(
                new RegExp('(' + q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + ')', 'gi'),
                '<mark class="admin-table-highlight">$1</mark>',
              );
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
  const cmpFn = poolaiUiWasmCall('compareSortValues');
  rows.sort((a, b) => {
    const aCell = a.cells[columnIndex];
    const bCell = b.cells[columnIndex];
    if (!aCell || !bCell) return 0;
    if (cmpFn) return cmpFn(aCell.textContent, bCell.textContent, isNumeric, ascending);
    const aValue = isNumeric ? parseFloat(aCell.textContent) : aCell.textContent.trim().toLowerCase();
    const bValue = isNumeric ? parseFloat(bCell.textContent) : bCell.textContent.trim().toLowerCase();
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
  const headers = Array.from(table.querySelectorAll('thead th')).map((th) => th.textContent.trim());
  const rows = adminTableVisibleRows(table).map((row) =>
    Array.from(row.cells).map((cell) => cell.dataset.originalText || cell.textContent.trim()),
  );
  const buildFn = poolaiUiWasmCall('buildTableCsv');
  const csv = buildFn
    ? buildFn(JSON.stringify(headers), JSON.stringify(rows))
    : [headers.join(',')].concat(rows.map((r) => r.join(','))).join('\n');
  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename || 'poolai-export.csv';
  a.click();
  URL.revokeObjectURL(url);
  showNotification(poolaiT('admin.table.exportedCsv', 'Table exported as CSV'), 'success');
}

function adminExportTableJson(table, filename) {
  const headers = Array.from(table.querySelectorAll('thead th')).map((th) => th.textContent.trim());
  const rows = adminTableVisibleRows(table).map((row) =>
    Array.from(row.cells).map((cell) => cell.dataset.originalText || cell.textContent.trim()),
  );
  const buildFn = poolaiUiWasmCall('buildTableJson');
  const json = buildFn
    ? buildFn(JSON.stringify(headers), JSON.stringify(rows))
    : JSON.stringify(
        rows.map((row) => {
          const obj = {};
          headers.forEach((h, i) => {
            obj[h] = row[i] || '';
          });
          return obj;
        }),
        null,
        2,
      );
  const blob = new Blob([json], { type: 'application/json;charset=utf-8' });
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

/** PH-S42 / PH-S153: empty state via poolai-ui-wasm when ready. */
function adminEmptyStateHtml(message, options) {
  const opts = options || {};
  const fn = poolaiUiWasmCall('emptyStateHtml');
  const msg = message || poolaiT('admin.table.empty', 'No data to display');
  if (fn) {
    return fn(msg, opts.hint || '', opts.icon || '📋', opts.actionHtml || '');
  }
  return (
    '<div class="admin-empty-state" role="status"><div class="admin-empty-state-icon" aria-hidden="true">' +
    (opts.icon || '📋') +
    '</div><p class="admin-empty-state-title">' +
    escapeHtml(msg) +
    '</p></div>'
  );
}

/** PH-S09 / PH-S42 / PH-S153: table HTML via poolai-ui-wasm when ready. */
function adminRenderTable(headers, rows, options) {
  const fn = poolaiUiWasmCall('renderTableHtml');
  if (fn) {
    return fn(JSON.stringify(headers || []), JSON.stringify(rows || []), JSON.stringify(options || {}));
  }
  if (!rows || rows.length === 0) {
    return adminEmptyStateHtml(
      (options || {}).emptyMessage || poolaiT('admin.table.empty', 'No data to display'),
      (options || {}).emptyOptions,
    );
  }
  return '<div class="admin-table-container"><table class="admin-table admin-table--striped"><tbody><tr><td>' +
    escapeHtml(String(rows.length)) +
    ' rows</td></tr></tbody></table></div>';
}

/** PH-S09 / PH-S153: form field markup via poolai-ui-wasm when ready. */
function adminFormFieldHtml(spec) {
  const fn = poolaiUiWasmCall('formFieldHtml');
  const id = (spec && spec.id) || 'fld_' + Math.random().toString(36).slice(2, 11);
  if (fn) return fn(JSON.stringify(spec || {}), id);
  return (
    '<div class="form-group"><label for="' +
    id +
    '">' +
    escapeHtml((spec && spec.label) || '') +
    '</label><input type="text" id="' +
    id +
    '" name="' +
    escapeHtml((spec && spec.name) || id) +
    '" /></div>'
  );
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
