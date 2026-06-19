//! Admin payout batch read panel (PH-S553).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Payout batch snapshot page (`/ui/admin/payout-batch`).
pub async fn admin_payout_batch() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function formatLamports(n) {
      const v = Number(n);
      if (!Number.isFinite(v)) return '—';
      return v.toLocaleString('en-US');
    }

    async function loadPayoutBatchPanel() {
      adminShowLoading('payout-batch-panel', T('admin.payoutBatch.loading', 'Loading payout batch…'));
      try {
        const [latest, history] = await Promise.all([
          fetchJson('/api/v1/grid/payout-batch'),
          fetchJson('/api/v1/grid/payout-batch/history?limit=5'),
        ]);
        renderPayoutBatchPanel(latest || {}, history || {});
      } catch (e) {
        adminShowInlineError('payout-batch-panel', e);
        showNotification(T('admin.payoutBatch.errLoad', 'Error loading payout batch: ') + e.message, 'error');
      }
    }

    function renderPayoutBatchPanel(latest, history) {
      const el = document.getElementById('payout-batch-panel');
      if (!el) return;
      const entry = latest.entry || null;
      const rows = (history.entries || []).map(function (row) {
        return '<tr>' +
          '<td><code>' + escapeHtml(String(row.job_id || '—')) + '</code></td>' +
          '<td>' + escapeHtml(String(row.cleared_at || '—')) + '</td>' +
          '<td>' + escapeHtml(formatLamports(row.gross_lamports)) + '</td>' +
          '<td><code>' + escapeHtml(String(row.payout_pubkey || '—')) + '</code></td>' +
          '</tr>';
      }).join('');
      el.innerHTML =
        '<div class="admin-card">' +
        '<h3>' + escapeHtml(T('admin.payoutBatch.latest', 'Latest cleared entry')) + '</h3>' +
        '<dl class="admin-dl">' +
        '<dt>' + escapeHtml(T('admin.payoutBatch.mode', 'Settlement mode')) + '</dt>' +
        '<dd><code>' + escapeHtml(String(latest.settlement_mode || 'offline_batch')) + '</code></dd>' +
        '<dt>' + escapeHtml(T('admin.payoutBatch.onChain', 'On-chain pending')) + '</dt>' +
        '<dd>' + escapeHtml(entry && latest.on_chain_pending ? 'yes' : 'no') + '</dd>' +
        '<dt>' + escapeHtml(T('admin.payoutBatch.jobId', 'Job ID')) + '</dt>' +
        '<dd><code>' + escapeHtml(String(entry && entry.job_id || '—')) + '</code></dd>' +
        '<dt>' + escapeHtml(T('admin.payoutBatch.pubkey', 'Payout pubkey')) + '</dt>' +
        '<dd><code>' + escapeHtml(String(entry && entry.payout_pubkey || '—')) + '</code></dd>' +
        '</dl></div>' +
        '<div class="admin-card">' +
        '<h3>' + escapeHtml(T('admin.payoutBatch.history', 'Recent history')) + '</h3>' +
        '<table class="admin-table"><thead><tr>' +
        '<th>' + escapeHtml(T('admin.payoutBatch.colJob', 'Job')) + '</th>' +
        '<th>' + escapeHtml(T('admin.payoutBatch.colCleared', 'Cleared')) + '</th>' +
        '<th>' + escapeHtml(T('admin.payoutBatch.colGross', 'Gross lamports')) + '</th>' +
        '<th>' + escapeHtml(T('admin.payoutBatch.colPubkey', 'Pubkey')) + '</th>' +
        '</tr></thead><tbody>' + (rows || '<tr><td colspan="4">—</td></tr>') + '</tbody></table>' +
        '</div>';
    }

    loadPayoutBatchPanel();
    setInterval(loadPayoutBatchPanel, 15000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.payoutBatch",
        "Payout batch",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.payoutBatch.section">Payout batch</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadPayoutBatchPanel()" data-i18n="admin.payoutBatch.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.payoutBatch.hint">
            Read-only coordinator payout batch ledger (Galaxy §8.2, PH-S553).
          </p>
          <div id="payout-batch-panel" class="payout-batch-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_payout_batch_page_api_ph_s553() {
    let html = admin_payout_batch().await.0;
    assert!(html.contains("payout-batch-panel"));
    assert!(html.contains("/api/v1/grid/payout-batch"));
}
