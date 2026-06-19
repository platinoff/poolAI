//! Admin security advisories panel (PH-S586).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Security advisories page (`/ui/admin/security-advisories`).
pub async fn admin_security_advisories() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function renderSecurityAdvisoriesPanel(rows) {
      const el = document.getElementById('security-advisories-panel');
      if (!el) return;
      const list = Array.isArray(rows) ? rows : [];
      if (!list.length) {
        el.innerHTML = '<p class="muted">' + escapeHtml(T('admin.securityAdvisories.empty', 'No advisories.')) + '</p>';
        return;
      }
      let body = '';
      list.forEach((row) => {
        const ack = row.acknowledged ? 'ack' : 'open';
        const btn = row.acknowledged
          ? '<span class="badge badge-success">' + escapeHtml(T('admin.securityAdvisories.acknowledged', 'Acknowledged')) + '</span>'
          : '<button type="button" class="btn btn-secondary btn-sm" onclick="ackSecurityAdvisory(\'' + escapeHtml(String(row.id)).replace(/'/g, "\\'") + '\')">' +
            escapeHtml(T('admin.securityAdvisories.ack', 'Acknowledge')) + '</button>';
        body += '<tr><td><code>' + escapeHtml(String(row.id)) + '</code></td>' +
          '<td><span class="badge">' + escapeHtml(String(row.severity || '—')) + '</span></td>' +
          '<td>' + escapeHtml(String(row.summary || '')) + '</td>' +
          '<td>' + btn + '</td></tr>';
      });
      el.innerHTML =
        '<table class="admin-table" aria-label="' + escapeHtml(T('admin.securityAdvisories.table', 'Security advisories')) + '">' +
        '<thead><tr><th>' + escapeHtml(T('admin.securityAdvisories.colId', 'ID')) + '</th>' +
        '<th>' + escapeHtml(T('admin.securityAdvisories.colSeverity', 'Severity')) + '</th>' +
        '<th>' + escapeHtml(T('admin.securityAdvisories.colSummary', 'Summary')) + '</th>' +
        '<th>' + escapeHtml(T('admin.securityAdvisories.colAction', 'Action')) + '</th></tr></thead>' +
        '<tbody>' + body + '</tbody></table>';
    }

    async function loadSecurityAdvisoriesPanel() {
      adminShowLoading('security-advisories-panel', T('admin.securityAdvisories.loading', 'Loading advisories…'));
      try {
        const rows = await fetchJson('/api/v1/admin/security-advisories');
        renderSecurityAdvisoriesPanel(rows || []);
      } catch (e) {
        adminShowInlineError('security-advisories-panel', e);
        showNotification(T('admin.securityAdvisories.errLoad', 'Error loading advisories: ') + e.message, 'error');
      }
    }

    async function ackSecurityAdvisory(id) {
      try {
        await fetchJson('/api/v1/admin/security-advisories/' + encodeURIComponent(id) + '/acknowledge', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: '{}',
        });
        showNotification(T('admin.securityAdvisories.ackOk', 'Advisory acknowledged'), 'success');
        await loadSecurityAdvisoriesPanel();
      } catch (e) {
        showNotification(T('admin.securityAdvisories.ackErr', 'Ack failed: ') + e.message, 'error');
      }
    }

    loadSecurityAdvisoriesPanel();
    "#;

    admin_layout_grid_pricing(
        "admin.page.securityAdvisories",
        "Security advisories",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.securityAdvisories.section">Security advisories</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadSecurityAdvisoriesPanel()" data-i18n="admin.securityAdvisories.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.securityAdvisories.hint">
            Galaxy §9.6 advisory stub list; acknowledge records audit metric (PH-S573).
          </p>
          <div id="security-advisories-panel" class="security-advisories-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_security_advisories_page_api_ph_s586() {
    let html = admin_security_advisories().await.0;
    assert!(html.contains("/api/v1/admin/security-advisories"));
    assert!(html.contains("security-advisories-panel"));
    assert!(html.contains("ackSecurityAdvisory"));
}
