//! Audit Logs viewer page
//!
//! Provides query interface for viewing and filtering audit events.

use crate::ui::admin::admin_layout;
use axum::response::Html;

/// Audit logs viewer page
pub async fn admin_audit() -> Html<String> {
    let script = r#"
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
        const events = await fetchJson(`/api/enterprise/audit/events?${params}`);
        renderAuditEvents(events);
      } catch (e) {
        showNotification('Error loading audit logs: ' + e.message, 'error');
      }
    }
    
    function renderAuditEvents(events) {
      const el = document.getElementById('audit-events');
      if (!el) return;
      if (!events || events.length === 0) {
        el.innerHTML = '<div class="muted">No audit events found</div>';
        return;
      }
      el.innerHTML = `
        <table class="admin-table">
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>Level</th>
              <th>User</th>
              <th>Action</th>
              <th>Resource</th>
              <th>Result</th>
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
        </table>
      `;
    }
    
    queryAuditLogs();
    "#;

    admin_layout(
        "Audit Logs",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2>Audit Events</h2>
            <div class="admin-filters">
              <input type="text" id="audit-search" placeholder="Search..." />
              <select id="audit-level">
                <option value="">All Levels</option>
                <option value="Info">Info</option>
                <option value="Warning">Warning</option>
                <option value="Error">Error</option>
                <option value="Critical">Critical</option>
              </select>
              <input type="date" id="audit-start-date" />
              <input type="date" id="audit-end-date" />
              <button class="btn" onclick="queryAuditLogs()">Query</button>
            </div>
          </div>
          <div id="audit-events"></div>
        </div>
        "#,
        script,
    )
}
