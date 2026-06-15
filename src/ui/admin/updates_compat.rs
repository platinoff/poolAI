//! Galaxy Grid updates & compatibility admin page (PH-S93) — read-only ops pointers.

use crate::grid::protocol_compat::{
    negotiate, CompatStatus, DEFAULT_COORDINATOR_PROTOCOL, MIN_COORDINATOR_VERSION_DOCS_URL,
};
use crate::ui::admin::admin_layout;
use axum::response::Html;

const DOC_SECURITY_HARDENING: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/security/SECURITY_HARDENING.md";
const DOC_VERIFY_QUICKSTART: &str = concat!(
    "https://github.com/platinoff/poolAI/blob/main/docs/security/SECURITY_HARDENING.md",
    "#operator-quickstart-verify-signed-release-ph-s71"
);
const DOC_RELEASE_MANIFEST: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/development/RELEASE_MANIFEST_SAMPLE.md";
const DOC_FIXTURES_README: &str =
    "https://github.com/platinoff/poolAI/blob/main/tests/fixtures/release/dev/README.md";

fn compat_status_wire(status: CompatStatus) -> &'static str {
    match status {
        CompatStatus::Accepted => "accepted",
        CompatStatus::UpgradeRequired => "upgrade_required",
        CompatStatus::Unsupported => "unsupported",
    }
}

/// Updates & compatibility page (`/ui/admin/updates-compat`).
pub async fn admin_updates_compat() -> Html<String> {
    let negotiation = negotiate(None);
    let coordinator_protocol = negotiation.coordinator_protocol_version;
    let default_negotiation_status = compat_status_wire(negotiation.status);
    let build_id = env!("CARGO_PKG_VERSION");
    let default_protocol = DEFAULT_COORDINATOR_PROTOCOL;
    let compat_docs_url = MIN_COORDINATOR_VERSION_DOCS_URL;

    let body = format!(
        r#"
        <div class="admin-section" id="updates-compat-panel">
          <div class="admin-header">
            <h2 data-i18n="admin.updatesCompat.section">Updates &amp; compatibility</h2>
          </div>
          <p class="muted admin-hint" data-i18n="admin.updatesCompat.hint">
            Read-only Galaxy governance pointers. Policy prose lives in docs — not duplicated here.
          </p>

          <div class="admin-card" id="updates-compat-protocol">
            <h3 data-i18n="admin.updatesCompat.protocolTitle">Protocol version</h3>
            <p class="muted" data-i18n="admin.updatesCompat.protocolHint">
              Coordinator wire baseline (PH-S65 <code>protocol_compat</code>). Workers send
              <code>protocol_version</code> on <code>POST /api/v1/discovery/register-remote</code>.
            </p>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.coordinator">Coordinator protocol</span>
              <span class="stat-value"><code id="updates-compat-coordinator-protocol">{coordinator_protocol}</code></span>
            </div>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.build">Coordinator build</span>
              <span class="stat-value"><code>{build_id}</code></span>
            </div>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.env">Env override</span>
              <span class="stat-value"><code>POOLAI_COORDINATOR_PROTOCOL_VERSION</code>
                <span class="muted">(default {default_protocol})</span></span>
            </div>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.negotiation">Default negotiation (no worker version)</span>
              <span class="stat-value"><span id="updates-compat-negotiation-status" class="badge badge-success" data-status="{default_negotiation_status}">{default_negotiation_status}</span></span>
            </div>
            <p class="muted admin-hint" data-i18n="admin.updatesCompat.compatStatusHint">
              Registration may return <code>compat_status</code> with HTTP 403/426 when the worker is outside the matrix window.
            </p>
          </div>

          <div class="admin-card" id="updates-compat-verify-release">
            <h3 data-i18n="admin.updatesCompat.verifyTitle">Verify signed release</h3>
            <p class="muted" data-i18n="admin.updatesCompat.verifyHint">
              Operator quickstart — <code>poolai-verify-release</code> (PH-S66/S85). See SECURITY_HARDENING for the full checklist.
            </p>
            <ul>
              <li><a href="{doc_security}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.security">SECURITY_HARDENING.md</a></li>
              <li><a href="{doc_verify}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.verifyQuickstart">Verify-release quickstart (§ PH-S71)</a></li>
              <li><a href="{doc_manifest}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.manifest">RELEASE_MANIFEST_SAMPLE.md</a></li>
              <li><a href="{doc_fixtures}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.fixtures">Dev fixtures README</a></li>
            </ul>
            <pre id="updates-compat-verify-cmd"><code>cargo run --bin poolai-verify-release -- \
  --manifest tests/fixtures/release/dev/release-manifest.json \
  --signature tests/fixtures/release/dev/release-manifest.json.sig \
  --trust-root tests/fixtures/release/dev/maintainer_keys.json</code></pre>
          </div>

          <div class="admin-card" id="updates-compat-matrix">
            <h3 data-i18n="admin.updatesCompat.matrixTitle">Protocol compatibility matrix</h3>
            <p class="muted" data-i18n="admin.updatesCompat.matrixHint">
              Canonical compat matrix and negotiation rules — Galaxy §9.3. Implementation: <code>src/grid/protocol_compat.rs</code>.
            </p>
            <p>
              <a href="{compat_docs_url}" target="_blank" rel="noopener noreferrer" class="btn btn-secondary" data-i18n="admin.updatesCompat.link.matrix">
                Galaxy §9.3 compat matrix (docs)
              </a>
            </p>
          </div>
        </div>
        "#,
        coordinator_protocol = coordinator_protocol,
        build_id = build_id,
        default_protocol = default_protocol,
        doc_security = DOC_SECURITY_HARDENING,
        doc_verify = DOC_VERIFY_QUICKSTART,
        doc_manifest = DOC_RELEASE_MANIFEST,
        doc_fixtures = DOC_FIXTURES_README,
        compat_docs_url = compat_docs_url,
        default_negotiation_status = default_negotiation_status,
    );

    let script = r#"
    function compatStatusLabelFallback(status) {
      const key = String(status || '').toLowerCase();
      const map = {
        accepted: 'Accepted',
        upgrade_required: 'Upgrade required',
        unsupported: 'Unsupported',
      };
      return map[key] || '—';
    }

    function protocolVersionLabelFallback(raw) {
      const v = String(raw || '').trim();
      if (!v) return '—';
      const core = v.split(/\s+/)[0] || v;
      const part = core.split('-')[0] || core;
      return part.includes('.') ? part : v;
    }

    function compatStatusLabel(status) {
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.compatStatusLabel === 'function') {
        return wasm.compatStatusLabel(String(status || ''));
      }
      return compatStatusLabelFallback(status);
    }

    function protocolVersionLabel(raw) {
      const wasm = window.poolaiUiWasm;
      if (wasm && wasm.ready && typeof wasm.protocolVersionLabel === 'function') {
        return wasm.protocolVersionLabel(String(raw || ''));
      }
      return protocolVersionLabelFallback(raw);
    }

    function wireUpdatesCompatLabels() {
      const protoEl = document.getElementById('updates-compat-coordinator-protocol');
      if (protoEl) {
        protoEl.textContent = protocolVersionLabel(protoEl.textContent || '');
      }
      const statusEl = document.getElementById('updates-compat-negotiation-status');
      if (statusEl) {
        const raw = statusEl.dataset.status || statusEl.textContent || '';
        statusEl.textContent = compatStatusLabel(raw);
      }
    }

    function startUpdatesCompatPage() {
      wireUpdatesCompatLabels();
    }

    if (window.poolaiUiWasm && (window.poolaiUiWasm.ready || window.poolaiUiWasm.failed)) {
      startUpdatesCompatPage();
    } else {
      window.addEventListener('poolai-ui-wasm-ready', startUpdatesCompatPage, { once: true });
    }
    "#;

    admin_layout(
        "admin.page.updatesCompat",
        "Updates & compatibility",
        &body,
        script,
    )
}

#[tokio::test]
async fn admin_updates_compat_page_includes_protocol_and_doc_blocks() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("id=\"updates-compat-panel\""));
    assert!(html.contains("id=\"updates-compat-protocol\""));
    assert!(html.contains("id=\"updates-compat-verify-release\""));
    assert!(html.contains("id=\"updates-compat-matrix\""));
    assert!(html.contains("id=\"updates-compat-negotiation-status\""));
    assert!(html.contains("poolai-verify-release"));
    assert!(html.contains("POOLAI_COORDINATOR_PROTOCOL_VERSION"));
    assert!(html.contains(MIN_COORDINATOR_VERSION_DOCS_URL));
    assert!(html.contains(DEFAULT_COORDINATOR_PROTOCOL));
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains("admin.updatesCompat.section"));
}

#[tokio::test]
async fn admin_updates_compat_page_wires_poolai_ui_wasm_labels() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("type=\"module\""));
    assert!(html.contains("/ui/wasm/poolai_ui_wasm.js"));
    assert!(html.contains("window.poolaiUiWasm"));
    assert!(html.contains("poolai-ui-wasm-ready"));
    assert!(html.contains("compatStatusLabelFallback"));
    assert!(html.contains("protocolVersionLabel"));
}
