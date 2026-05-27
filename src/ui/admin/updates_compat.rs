//! Galaxy Grid updates & compatibility admin page (PH-S93) — read-only ops pointers.

use crate::grid::protocol_compat::{
    negotiate, DEFAULT_COORDINATOR_PROTOCOL, MIN_COORDINATOR_VERSION_DOCS_URL,
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

/// Updates & compatibility page (`/ui/admin/updates-compat`).
pub async fn admin_updates_compat() -> Html<String> {
    let negotiation = negotiate(None);
    let coordinator_protocol = negotiation.coordinator_protocol_version;
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
    );

    admin_layout(
        "admin.page.updatesCompat",
        "Updates & compatibility",
        &body,
        "",
    )
}

#[tokio::test]
async fn admin_updates_compat_page_includes_protocol_and_doc_blocks() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("id=\"updates-compat-panel\""));
    assert!(html.contains("id=\"updates-compat-protocol\""));
    assert!(html.contains("id=\"updates-compat-verify-release\""));
    assert!(html.contains("id=\"updates-compat-matrix\""));
    assert!(html.contains("poolai-verify-release"));
    assert!(html.contains("POOLAI_COORDINATOR_PROTOCOL_VERSION"));
    assert!(html.contains(MIN_COORDINATOR_VERSION_DOCS_URL));
    assert!(html.contains(DEFAULT_COORDINATOR_PROTOCOL));
}
