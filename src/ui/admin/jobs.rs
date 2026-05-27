//! Job layer admin page (PH-S53) — list jobs + persistence backend badge.
//! PH-S96: read-only Galaxy lease columns (`lease_owner`, `lease_epoch`, `lease_expires_at`).

use crate::ui::admin::admin_layout;
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
      if (s === 'executing' || s === 'verifying') return 'warning';
      return '';
    }

    function formatLeaseCell(value) {
      if (value === null || value === undefined || value === '') return '—';
      return String(value);
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
              <th>${escapeHtml(T('admin.jobs.col.leaseOwner', 'Lease owner'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.leaseEpoch', 'Lease epoch'))}</th>
              <th>${escapeHtml(T('admin.jobs.col.leaseExpires', 'Lease expires'))}</th>
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
                <td><span class="status-badge ${statusBadgeClass(status)}">${escapeHtml(String(status))}</span></td>
                <td>${escapeHtml(String(j.created_at || ''))}</td>
                <td>${escapeHtml(String(j.worker_id || '—'))}</td>
                <td>${escapeHtml(String(j.vm_id || '—'))}</td>
                <td><code>${escapeHtml(formatLeaseCell(j.lease_owner))}</code></td>
                <td>${escapeHtml(formatLeaseCell(j.lease_epoch))}</td>
                <td>${escapeHtml(formatLeaseCell(j.lease_expires_at))}</td>
              </tr>`;
            }).join('')}
          </tbody>
        </table>
      `;
      if (typeof adminInitTablesIn === 'function') adminInitTablesIn(el);
    }

    loadJobs();
    setInterval(loadJobs, 10000);
    "#;

    admin_layout(
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
            Job queue from the coordinator store. Backend is set at startup via <code>POOLAI_JOB_STORE</code>. Lease columns are read-only (Galaxy §4.3.1, PH-S94/S95).
          </p>
          <div id="jobs-list"></div>
        </div>
        "#,
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
    assert!(html.contains("lease_expires_at"));
}
