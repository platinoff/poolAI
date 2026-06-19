//! Admin network profiles panel (PH-S582).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Network profiles page (`/ui/admin/network-profiles`).
pub async fn admin_network_profiles() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function renderNetworkProfilesList(peerIds, profiles) {
      const el = document.getElementById('network-profiles-panel');
      if (!el) return;
      const ids = Array.isArray(peerIds) ? peerIds : [];
      if (!ids.length) {
        el.innerHTML = '<p class="muted">' + escapeHtml(T('admin.networkProfiles.empty', 'No persisted network profiles.')) + '</p>';
        return;
      }
      let rows = '';
      ids.forEach((id) => {
        const snap = profiles[id] || {};
        const region = snap.region || snap.network_profile?.region || '—';
        const latency = snap.latency_ms_p50 ?? snap.network_profile?.latency_ms_p50 ?? '—';
        rows += '<tr><td><code>' + escapeHtml(String(id)) + '</code></td>' +
          '<td>' + escapeHtml(String(region)) + '</td>' +
          '<td>' + escapeHtml(String(latency)) + '</td></tr>';
      });
      el.innerHTML =
        '<table class="admin-table" aria-label="' + escapeHtml(T('admin.networkProfiles.table', 'Network profiles')) + '">' +
        '<thead><tr><th>' + escapeHtml(T('admin.networkProfiles.colPeer', 'Peer')) + '</th>' +
        '<th>' + escapeHtml(T('admin.networkProfiles.colRegion', 'Region')) + '</th>' +
        '<th>' + escapeHtml(T('admin.networkProfiles.colLatency', 'Latency p50')) + '</th></tr></thead>' +
        '<tbody>' + rows + '</tbody></table>';
    }

    async function loadNetworkProfilesPanel() {
      adminShowLoading('network-profiles-panel', T('admin.networkProfiles.loading', 'Loading network profiles…'));
      try {
        const list = await fetchJson('/api/v1/grid/network-profiles');
        const peerIds = list.peer_ids || [];
        const profiles = {};
        await Promise.all(peerIds.map(async (id) => {
          try {
            profiles[id] = await fetchJson('/api/v1/grid/network-profiles/' + encodeURIComponent(id));
          } catch (_) {
            profiles[id] = {};
          }
        }));
        renderNetworkProfilesList(peerIds, profiles);
      } catch (e) {
        adminShowInlineError('network-profiles-panel', e);
        showNotification(T('admin.networkProfiles.errLoad', 'Error loading profiles: ') + e.message, 'error');
      }
    }

    loadNetworkProfilesPanel();
    setInterval(loadNetworkProfilesPanel, 15000);
    "#;

    admin_layout_grid_pricing(
        "admin.page.networkProfiles",
        "Network profiles",
        r#"
        <div class="admin-section">
          <div class="admin-header">
            <h2 data-i18n="admin.networkProfiles.section">Network profiles</h2>
            <div class="admin-header-actions">
              <button type="button" class="btn btn-primary" onclick="loadNetworkProfilesPanel()" data-i18n="admin.networkProfiles.refresh">Refresh</button>
            </div>
          </div>
          <p class="muted admin-hint" data-i18n="admin.networkProfiles.hint">
            Read-only persisted peer profiles (Galaxy §8.1, PH-S570).
          </p>
          <div id="network-profiles-panel" class="network-profiles-panel"></div>
        </div>
        "#,
        script,
    )
}

#[tokio::test]
async fn admin_network_profiles_page_api_ph_s582() {
    let html = admin_network_profiles().await.0;
    assert!(html.contains("/api/v1/grid/network-profiles"));
    assert!(html.contains("network-profiles-panel"));
    assert!(html.contains("loadNetworkProfilesPanel"));
}
