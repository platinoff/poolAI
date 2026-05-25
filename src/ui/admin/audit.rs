//! Audit Logs viewer page
//!
//! Provides query interface for viewing and filtering audit events.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Audit logs viewer page
pub async fn admin_audit() -> Html<String> {
    let script = r#"
    function Ta(k, fb) {
      return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb;
    }

    async function queryAuditLogs() {
      const level = document.getElementById('audit-level').value;
      const startDate = document.getElementById('audit-start-date').value;
      const endDate = document.getElementById('audit-end-date').value;
      
      const params = new URLSearchParams();
      if (level) params.append('min_level', level);
      if (startDate) params.append('start_time', startDate + 'T00:00:00Z');
      if (endDate) params.append('end_time', endDate + 'T23:59:59Z');
      params.append('limit', '100');
      
      try {
        adminShowLoading('audit-events', Ta('admin.audit.loading', 'Loading audit events…'));
        const events = await fetchJson(`/api/enterprise/audit/events?${params}`);
        renderAuditEvents(events);
      } catch (e) {
        adminShowInlineError('audit-events', e);
        showNotification(Ta('admin.audit.errLoad', 'Error loading audit logs: ') + e.message, 'error');
      }
    }
    
    function renderAuditEvents(events) {
      const el = document.getElementById('audit-events');
      if (!el) return;
      if (!events || events.length === 0) {
        el.innerHTML = adminEmptyStateHtml(Ta('admin.audit.empty', 'No audit events found'));
        return;
      }
      el.innerHTML = `
        <div class="admin-table-container"><table class="admin-table" id="audit-events-table" aria-label="${escapeHtml(Ta('admin.audit.sectionTitle', 'Audit Events'))}">
          <thead>
            <tr>
              <th>${escapeHtml(Ta('admin.audit.col.time', 'Timestamp'))}</th>
              <th>${escapeHtml(Ta('admin.audit.col.level', 'Level'))}</th>
              <th>${escapeHtml(Ta('admin.audit.col.user', 'User'))}</th>
              <th>${escapeHtml(Ta('admin.audit.col.action', 'Action'))}</th>
              <th>${escapeHtml(Ta('admin.audit.col.resource', 'Resource'))}</th>
              <th>${escapeHtml(Ta('admin.audit.col.result', 'Result'))}</th>
            </tr>
          </thead>
          <tbody>
            ${events.map(e => `
              <tr>
                <td>${new Date(e.timestamp).toLocaleString()}</td>
                <td><span class="status-badge ${e.level.toLowerCase()}">${e.level}</span></td>
                <td>${e.user_id || '—'}</td>
                <td>${e.action}</td>
                <td>${e.resource_type}${e.resource_id ? ': ' + e.resource_id : ''}</td>
                <td>${e.result}</td>
              </tr>
            `).join('')}
          </tbody>
        </table></div>
      `;
      const table = el.querySelector('#audit-events-table');
      if (table) {
        adminEnhanceAdminTable(table, {
          noToolbar: true,
          externalSearchEl: document.getElementById('audit-search'),
        });
      }
    }
    
    queryAuditLogs();
    "#;

    admin_layout(
        "admin.page.audit",
        "Audit Logs",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.audit.sectionTitle">Audit Events</h2>
            <div class="admin-filters">
              <label for="audit-search" data-i18n="admin.audit.label.search">Search</label>
              <input type="text" id="audit-search" data-i18n-placeholder="admin.audit.searchPh" placeholder="Search…" />
              <label for="audit-level" data-i18n="admin.audit.label.level">Level</label>
              <select id="audit-level">
                <option value="" data-i18n="admin.audit.levelAll">All Levels</option>
                <option value="Info" data-i18n="admin.audit.levelInfo">Info</option>
                <option value="Warning" data-i18n="admin.audit.levelWarning">Warning</option>
                <option value="Error" data-i18n="admin.audit.levelError">Error</option>
                <option value="Critical" data-i18n="admin.audit.levelCritical">Critical</option>
              </select>
              <label for="audit-start-date" data-i18n="admin.audit.label.startDate">Start date</label>
              <input type="date" id="audit-start-date" />
              <label for="audit-end-date" data-i18n="admin.audit.label.endDate">End date</label>
              <input type="date" id="audit-end-date" />
              <button type="button" class="btn" onclick="queryAuditLogs()" data-i18n="admin.audit.query">Query</button>
            </div>
          </div>
          <div id="audit-events"></div>
        </div>
        "#,
        script,
    )
}
