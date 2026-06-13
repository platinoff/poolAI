//! Job layer admin page (PH-S53) — list jobs + persistence backend badge.
//! PH-S96/PH-S105/PH-S119: read-only Galaxy lease columns, tooltips, `#epoch` display.
//! PH-S141: `migrating` status badge + i18n EN/UK.
//! PH-S152: lease state badge via shared `poolai-ui-wasm` `leaseStateLabel`; thin JS fallback.

use crate::ui::admin::{admin_layout_with_module_script, POOLAI_UI_WASM_MODULE};
use axum::response::Html;

/// Jobs management page (`/ui/admin/jobs`).
pub async fn admin_jobs() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }
    function Ep() { return typeof poolaiT === 'function' ? poolaiT('err.errorPrefix', 'Error: ') : 'Error: '; }

    function renderStoreBadge(backend) {
      const el = document.getElementById('jobs-store-badge');
      if (!el) return;
      const key = String(backend || 'json').toLowerCase();
      const label = T('admin.jobs.store.' + key, key);
      el.innerHTML =
        '<span class="status-badge active" title="' + escapeHtml(T('admin.jobs.storeHint', 'Job persistence backend')) + '">' +
        escapeHtml(T('admin.jobs.storeLabel', 'Store:')) + ' ' + escapeHtml(label) +
        '</span>';
    }

    function statusBadgeClass(status) {
      const s = String(status || '').toLowerCase();
      if (s === 'completed' || s === 'rewarded') return 'active';
      if (s === 'failed') return 'error';
      if (s === 'executing' || s === 'verifying' || s === 'migrating') return 'warning';
      return '';
    }

    function statusBadgeLabel(status) {
      const s = String(status || '').toLowerCase();
      if (s === 'migrating') {
        return T('admin.jobs.status.migrating', 'Migrating');
      }
      return String(status || '');
    }

    function statusBadge(status) {
      const s = String(status || '').toLowerCase();
      const cls = statusBadgeClass(status);
      const label = statusBadgeLabel(status);
      let titleAttr = '';
      if (s === 'migrating') {
        const tip = T(
          'admin.jobs.tooltip.statusMigrating',
          'Galaxy re-migrate: worker handoff in progress (PH-S104).',
        );
        titleAttr = ' title="' + escapeHtml(tip) + '" data-job-status="migrating"';
      }
      return (
        '<span class="status-badge ' + cls + '"' + titleAttr + '>' +
        escapeHtml(label) +
        '</span>'
      );
    }

    function formatLeaseCell(value) {
      if (value === null || value === undefined || value === '') return '—';
      return String(value);
    }

    function thLease(colKey, labelKey, tipKey) {
      const label = T(labelKey, colKey);
      const tip = T(tipKey, '');
      const titleAttr = tip ? ' title="' + escapeHtml(tip) + '"' : '';
      return '<th' + titleAttr + '>' + escapeHtml(label) + '</th>';
    }

    function leaseOwnerCell(owner) {
      if (owner === null || owner === undefined || owner === '') {
        return '<span class="muted">—</span>';
      }
      const text = String(owner);
      const tip = T(
        'admin.jobs.tooltip.leaseOwner',
        'Galaxy §4.3.1: worker or peer id holding the active lease (acquire/renew CAS owner).',
      );
      return (
        '<code class="lease-owner-cell" title="' +
        escapeHtml(tip) +
        '">' +
        escapeHtml(text) +
        '</code>'
      );
    }

    function leaseEpochCell(epoch) {
      if (epoch === null || epoch === undefined || epoch === '') {
        return '<span class="muted">—</span>';
      }
      const n = Number(epoch);
      const display = Number.isFinite(n) ? '#' + String(n) : String(epoch);
      const tip = T(
        'admin.jobs.tooltip.leaseEpoch',
        'Monotonic CAS generation; PATCH, renew, and grid result must match this epoch.',
      );
      return (
        '<span class="lease-epoch-cell" title="' +
        escapeHtml(tip) +
        '">' +
        escapeHtml(display) +
        '</span>'
      );
    }

    function leaseStateFallback(expiresAt) {
      if (!expiresAt) return 'none';
      const ts = Date.parse(String(expiresAt));
      if (Number.isNaN(ts)) return 'none';
      return Date.now() < ts ? 'active' : 'expired';
    }

    function leaseStateKey(expiresAt) {
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.leaseStateLabel === 'function') {
        return wasm.leaseStateLabel(String(expiresAt || ''), new Date().toISOString());
      }
      return leaseStateFallback(expiresAt);
    }

    function leaseStateBadge(expiresAt) {
      const state = leaseStateKey(expiresAt);
      if (state === 'none') return '—';
      const cls = state === 'active' ? 'active' : 'warning';
      const label = state === 'active'
        ? T('admin.jobs.leaseState.active', 'Active')
        : T('admin.jobs.leaseState.expired', 'Expired');
      return '<span class="status-badge ' + cls + '">' + escapeHtml(label) + '</span>';
    }

    async function loadJobs() {
      adminShowLoading('jobs-list', T('admin.jobs.loading', 'Loading jobs…'));
      try {
        const data = await fetchJson('/api/v1/jobs');
        renderStoreBadge(data.store_backend);
        renderJobs(data.jobs || []);
      } catch (e) {
        adminShowInlineError('jobs-list', e);
        showNotification(T('admin.jobs.errLoad', 'Error loading jobs: ') + e.message, 'error');
      }
    }

    function renderJobs(jobs) {
      const el = document.getElementById('jobs-list');
      if (!el) return;
      if (!jobs || jobs.length === 0) {
        el.innerHTML = adminEmptyStateHtml(T('admin.jobs.empty', 'No jobs yet'));
        if (typeof adminInitTablesIn === 'function') adminInitTablesIn(el);
        return;
      }
      el.innerHTML = `
        <table class="admin-table admin-table--striped">
          <thead>
            <tr>
              <th>${escapeHtml(T('admin.jobs.col.id', 'ID'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.kind', 'Kind'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.status', 'Status'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.created', 'Created'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.worker', 'Worker'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.vm', 'VM'))}</th>
              ${thLease('Lease owner', 'admin.jobs.col.leaseOwner', 'admin.jobs.tooltip.leaseOwnerCol')}
              ${thLease('Lease epoch', 'admin.jobs.col.leaseEpoch', 'admin.jobs.tooltip.leaseEpochCol')}
              ${thLease('Lease state', 'admin.jobs.col.leaseState', 'admin.jobs.tooltip.leaseStateCol')}
              ${thLease('Lease expires', 'admin.jobs.col.leaseExpires', 'admin.jobs.tooltip.leaseExpiresCol')}
            </tr>
          </thead>
          <tbody>
            ${jobs.map((j) => {
              const id = j.id || '';
              const status = j.status || '';
              return `
              <tr>
                <td><code>${escapeHtml(String(id))}</code></td>
                <td>${escapeHtml(String(j.kind || ''))}</td>
                <td>${statusBadge(status)}</td>
                <td>${escapeHtml(String(j.created_at || ''))}</td>
                <td>${escapeHtml(String(j.worker_id || '—'))}</td>
                <td>${escapeHtml(String(j.vm_id || '—'))}</td>
                <td>${leaseOwnerCell(j.lease_owner)}</td>
                <td>${leaseEpochCell(j.lease_epoch)}</td>
                <td>${leaseStateBadge(j.lease_expires_at)}</td>
                <td>${escapeHtml(formatLeaseCell(j.lease_expires_at))}</td>
              </tr>`;
            }).join('')}
          </tbody>
        </table>
      `;
      if (typeof adminInitTablesIn === 'function') adminInitTablesIn(el);
    }

    function startJobsPage() {
      loadJobs();
      setInterval(loadJobs, 10000);
    }

    if (window.poolaiUiWasm && (window.poolaiUiWasm.ready || window.poolaiUiWasm.failed)) {
      startJobsPage();
    } else {
      window.addEventListener('poolai-ui-wasm-ready', startJobsPage, { once: true });
      setTimeout(startJobsPage, 2500);
    }
    "#;

    admin_layout_with_module_script(
        "admin.page.jobs",
        "Jobs",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.jobs.section">Jobs</h2>
            <div class="admin-header-actions">
              <span id="jobs-store-badge" class="jobs-store-badge muted" data-i18n="admin.jobs.storeLoading">Loading store…</span>
              <button type="button" class="btn" onclick="loadJobs()" data-i18n="ui.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.jobs.hint">
            Job queue from the coordinator store. Backend is set at startup via <code>POOLAI_JOB_STORE</code>. Lease columns are read-only; hover owner/epoch for Galaxy §4.3.1 CAS hints (PH-S119).
          </p>
          <div id="jobs-list"></div>
        </div>
        "#,
        POOLAI_UI_WASM_MODULE,
        script,
    )
}

#[tokio::test]
async fn admin_jobs_page_includes_store_badge_and_list() {
    let html = admin_jobs().await.0;
    assert!(html.contains("id=\"jobs-list\""));
    assert!(html.contains("id=\"jobs-store-badge\""));
    assert!(html.contains("/api/v1/jobs"));
    assert!(html.contains("renderStoreBadge"));
    assert!(html.contains("Lease owner"));
    assert!(html.contains("lease_owner"));
    assert!(html.contains("lease_epoch"));
    assert!(html.contains("leaseStateBadge"));
    assert!(html.contains("leaseStateKey"));
    assert!(html.contains("leaseStateFallback"));
    assert!(html.contains("poolai-ui-wasm-ready"));
    assert!(html.contains("leaseStateLabel"));
    assert!(html.contains("lease_expires_at"));
    assert!(html.contains("leaseOwnerCell"));
    assert!(html.contains("leaseEpochCell"));
    assert!(html.contains("admin.jobs.tooltip.leaseEpoch"));
    assert!(html.contains("statusBadge"));
    assert!(html.contains("admin.jobs.status.migrating"));
    assert!(html.contains("data-job-status=\"migrating\""));
}
