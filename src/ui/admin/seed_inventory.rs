//! Admin seed inventory panel (PH-S584).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Seed inventory page (`/ui/admin/seed-inventory`).
pub async fn admin_seed_inventory() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function renderSeedInventoryPanel(snapshot) {
      const el = document.getElementById('seed-inventory-panel');
      if (!el) return;
      const entries = Array.isArray(snapshot.entries) ? snapshot.entries : [];
      if (!entries.length) {
        el.innerHTML = '<p class="muted">' + escapeHtml(T('admin.seedInventory.empty', 'No seed inventory entries.')) + '</p>';
        return;
      }
      let rows = '';
      entries.forEach((row) => {
        const inv = row.seed_inventory || {};
        const shards = (inv.shard_ids || []).join(', ') || '—';
        const ram = inv.hot_tier && inv.hot_tier.ram_bytes_used != null ? inv.hot_tier.ram_bytes_used : '—';
        rows += '<tr><td><code>' + escapeHtml(String(row.peer_id || '—')) + '</code></td>' +
          '<td>' + escapeHtml(shards) + '</td>' +
          '<td>' + escapeHtml(String(ram)) + '</td></tr>';
      });
      el.innerHTML =
        '<p class="muted">' + escapeHtml(T('admin.seedInventory.generated', 'Generated')) + ': ' +
        escapeHtml(String(snapshot.generated_at || '—')) + '</p>' +
        '<table class="admin-table" aria-label="' + escapeHtml(T('admin.seedInventory.table', 'Seed inventory')) + '">' +
        '<thead><tr><th>' + escapeHtml(T('admin.seedInventory.colPeer', 'Peer')) + '</th>' +
        '<th>' + escapeHtml(T('admin.seedInventory.colShards', 'Shards')) + '</th>' +
        '<th>' + escapeHtml(T('admin.seedInventory.colRam', 'Hot RAM bytes')) + '</th></tr></thead>' +
        '<tbody>' + rows + '</tbody></table>';
    }

    async function loadSeedInventoryPanel() {
      adminShowLoading('seed-inventory-panel', T('admin.seedInventory.loading', 'Loading seed inventory…'));
      try {
        const data = await fetchJson('/api/v1/grid/seed-inventory');
        renderSeedInventoryPanel(data || {});
      } catch (e) {
        adminShowInlineError('seed-inventory-panel', e);
        showNotification(T('admin.seedInventory.errLoad', 'Error loading inventory: ') + e.message, 'error');
      }
    }

    loadSeedInventoryPanel();
    setInterval(loadSeedInventoryPanel, 15000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.seedInventory",
        "Seed inventory",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.seedInventory.section">Seed inventory</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadSeedInventoryPanel()" data-i18n="admin.seedInventory.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.seedInventory.hint">
            Read-only coordinator seed inventory stub (Galaxy §5.5, PH-S195).
          </p>
          <div id="seed-inventory-panel" class="seed-inventory-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_seed_inventory_page_api_ph_s584() {
    let html = admin_seed_inventory().await.0;
    assert!(html.contains("/api/v1/grid/seed-inventory"));
    assert!(html.contains("seed-inventory-panel"));
    assert!(html.contains("loadSeedInventoryPanel"));
}
