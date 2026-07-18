//! Admin network profiles panel (PH-S582 read, PH-S596 upsert).

use crate::ui::admin::admin_layout_grid_pricing;
use axum::response::Html;

/// Network profiles page (`/ui/admin/network-profiles`).
pub async fn admin_network_profiles() -> Html<String> {
    let script = r#"
    function T(k, fb) { return typeof poolaiT === 'function' ? poolaiT(k, fb) : fb; }

    function networkProfilesI18n(key, fallback) {
      return T(key, fallback);
    }

    function renderNetworkProfilesList(peerIds, profiles) {
      const el = document.getElementById('network-profiles-panel');
      if (!el) return;
      const ids = Array.isArray(peerIds) ? peerIds : [];
      const rows = ids.map((id) => {
        const snap = profiles[id] || {};
        return {
          peer_id: id,
          network_profile: snap.network_profile || snap,
        };
      });
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.renderNetworkProfilesPanel === 'function') {
        el.innerHTML = wasm.renderNetworkProfilesPanel(
          JSON.stringify(rows),
          networkProfilesI18n('admin.networkProfiles.colPeer', 'Peer'),
          networkProfilesI18n('admin.networkProfiles.colRegion', 'Region'),
          networkProfilesI18n('admin.networkProfiles.colLatency', 'Latency p50'),
          networkProfilesI18n('admin.networkProfiles.colBandwidth', 'Bandwidth Mbps'),
          networkProfilesI18n('admin.networkProfiles.table', 'Network profiles'),
          networkProfilesI18n('admin.networkProfiles.empty', 'No persisted network profiles.')
        );
        return;
      }
      if (!ids.length) {
        el.innerHTML = adminEmptyStateHtml(T('admin.networkProfiles.empty', 'No persisted network profiles.'));
        return;
      }
      let tableRows = '';
      ids.forEach((id) => {
        const snap = profiles[id] || {};
        const region = snap.region || snap.network_profile?.region || '—';
        const latency = snap.latency_ms_p50 ?? snap.network_profile?.latency_ms_p50 ?? '—';
        const bandwidth = snap.bandwidth_mbps ?? snap.network_profile?.bandwidth_mbps ?? '—';
        tableRows += '<tr><td><code>' + escapeHtml(String(id)) + '</code></td>' +
          '<td>' + escapeHtml(String(region)) + '</td>' +
          '<td>' + escapeHtml(String(latency)) + '</td>' +
          '<td>' + escapeHtml(String(bandwidth)) + '</td></tr>';
      });
      el.innerHTML =
        '<div class="admin-table-container"><table class="admin-table" aria-label="' + escapeHtml(T('admin.networkProfiles.table', 'Network profiles')) + '">' +
        '<thead><tr><th scope="col">' + escapeHtml(T('admin.networkProfiles.colPeer', 'Peer')) + '</th>' +
        '<th scope="col">' + escapeHtml(T('admin.networkProfiles.colRegion', 'Region')) + '</th>' +
        '<th scope="col">' + escapeHtml(T('admin.networkProfiles.colLatency', 'Latency p50')) + '</th>' +
        '<th scope="col">' + escapeHtml(T('admin.networkProfiles.colBandwidth', 'Bandwidth Mbps')) + '</th></tr></thead>' +
        '<tbody>' + tableRows + '</tbody></table></div>';
      if (typeof adminInitTablesIn === 'function') adminInitTablesIn(el);
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

    async function saveNetworkProfileUpsert(ev) {
      if (ev && ev.preventDefault) ev.preventDefault();
      const peerId = document.getElementById('network-profile-peer')?.value?.trim() || '';
      const region = document.getElementById('network-profile-region')?.value?.trim() || '';
      const latency = Number(document.getElementById('network-profile-latency')?.value || '0');
      const bandwidth = Number(document.getElementById('network-profile-bandwidth')?.value || '0');
      const egress = document.getElementById('network-profile-egress')?.value || 'direct';
      const statusEl = document.getElementById('network-profile-upsert-status');
      if (!peerId || !region) {
        if (statusEl) statusEl.textContent = T('admin.networkProfiles.errRequired', 'Peer id and region required.');
        return;
      }
      const body = {
        network_profile: {
          region: region,
          latency_ms_p50: latency,
          bandwidth_mbps: bandwidth,
          egress_policy: egress,
        },
      };
      try {
        const resp = await fetch('/api/v1/grid/network-profiles/' + encodeURIComponent(peerId), {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        });
        if (!resp.ok) {
          const errText = await resp.text();
          throw new Error(errText || resp.statusText);
        }
        if (statusEl) statusEl.textContent = T('admin.networkProfiles.saved', 'Profile saved.');
        await loadNetworkProfilesPanel();
      } catch (e) {
        if (statusEl) statusEl.textContent = T('admin.networkProfiles.errSave', 'Save failed: ') + e.message;
        showNotification(T('admin.networkProfiles.errSave', 'Save failed: ') + e.message, 'error');
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
            Persisted peer profiles (Galaxy §8.1). Upsert via PUT (PH-S596).
          </p>
          <form id="network-profile-upsert-form" class="admin-form admin-form-inline" onsubmit="saveNetworkProfileUpsert(event)">
            <label for="network-profile-peer" data-i18n="admin.networkProfiles.colPeer">Peer</label>
            <input id="network-profile-peer" name="peer_id" type="text" required aria-required="true" autocomplete="off" />
            <label for="network-profile-region" data-i18n="admin.networkProfiles.colRegion">Region</label>
            <input id="network-profile-region" name="region" type="text" required aria-required="true" autocomplete="off" />
            <label for="network-profile-latency" data-i18n="admin.networkProfiles.colLatency">Latency p50</label>
            <input id="network-profile-latency" name="latency_ms_p50" type="number" min="0" value="20" />
            <label for="network-profile-bandwidth" data-i18n="admin.networkProfiles.colBandwidth">Bandwidth Mbps</label>
            <input id="network-profile-bandwidth" name="bandwidth_mbps" type="number" min="0" value="500" />
            <label for="network-profile-egress" data-i18n="admin.networkProfiles.colEgress">Egress</label>
            <select id="network-profile-egress" name="egress_policy">
              <option value="direct">direct</option>
              <option value="lan_only">lan_only</option>
              <option value="vpn_proxy">vpn_proxy</option>
              <option value="white_ip">white_ip</option>
            </select>
            <button type="submit" class="btn btn-secondary" id="network-profile-save-btn" data-i18n="admin.networkProfiles.save">Save profile</button>
            <span id="network-profile-upsert-status" class="muted" aria-live="polite"></span>
          </form>
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
    assert!(html.contains("network-profile-upsert-form"));
    assert!(html.contains("saveNetworkProfileUpsert"));
    assert!(html.contains("renderNetworkProfilesPanel"));
}
