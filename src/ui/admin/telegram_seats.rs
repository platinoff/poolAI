//! Admin Telegram seats panel (PH-S517).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Telegram seats snapshot page (`/ui/admin/telegram-seats`).
pub async fn admin_telegram_seats() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    async function loadTelegramSeatsPanel() {
      adminShowLoading('telegram-seats-panel', T('admin.telegramSeats.loading', 'Loading seat snapshot…'));
      try {
        const data = await fetchJson('/api/v1/grid/telegram-seats');
        renderTelegramSeatsPanel(data || {});
      } catch (e) {
        adminShowInlineError('telegram-seats-panel', e);
        showNotification(T('admin.telegramSeats.errLoad', 'Error loading seats: ') + e.message, 'error');
      }
    }

    function renderTelegramSeatsPanel(snapshot) {
      const el = document.getElementById('telegram-seats-panel');
      if (!el) return;
      el.innerHTML = poolaiRenderTelegramSeatsPanel(JSON.stringify(snapshot || {}), {
        policy: T('admin.telegramSeats.colPolicy', 'Policy'),
        limit: T('admin.telegramSeats.colLimit', 'Seat limit'),
        active: T('admin.telegramSeats.colActive', 'Active workers'),
        bound: T('admin.telegramSeats.colBound', 'Bound wallets'),
        tableAria: T('admin.telegramSeats.section', 'Telegram seats'),
      });
    }

    loadTelegramSeatsPanel();
    setInterval(loadTelegramSeatsPanel, 10000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.telegramSeats",
        "Telegram seats",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.telegramSeats.section">Telegram seats</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadTelegramSeatsPanel()" data-i18n="admin.telegramSeats.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.telegramSeats.hint">
            Read-only coordinator seat snapshot (Galaxy §3.1, PH-S505).
          </p>
          <div id="telegram-seats-panel" class="telegram-seats-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_telegram_seats_page_api_ph_s517() {
    let html = admin_telegram_seats().await.0;
    assert!(html.contains("/api/v1/grid/telegram-seats"));
    assert!(html.contains("telegram-seats-panel"));
    assert!(html.contains("poolaiRenderTelegramSeatsPanel"));
}
