//! Galaxy Grid updates & compatibility admin page (PH-S93) — read-only ops pointers.
//! PH-S221: updates-compat page uses slim `admin_layout_updates_compat` + `admin_updates_compat_patch`.

use crate::grid::galaxy_capability_doc::{
    DEV_CAPABILITY_VERIFY_PK_HEX, ENV_CAPABILITY_VERIFY_PK_HEX,
};
use crate::grid::galaxy_update_policy::{
    release_manifest_url_from_env, update_policy_from_env, UpdatePolicyMode,
    ENV_RELEASE_MANIFEST_URL, ENV_UPDATE_POLICY,
};
use crate::grid::protocol_compat::{
    negotiate, CompatStatus, DEFAULT_COORDINATOR_PROTOCOL, MIN_COORDINATOR_VERSION_DOCS_URL,
};
use crate::ui::admin::admin_layout_updates_compat;
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
const DOC_CAPABILITY_FIXTURE: &str =
    "https://github.com/platinoff/poolAI/blob/main/tests/fixtures/capability/dev_pubkey.hex";
const DOC_GALAXY_55: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/concept/POOLAI_GALAXY_GRID.md#55-task-driven-prefetch-ph-s61";
const DOC_GALAXY_52: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/concept/POOLAI_GALAXY_GRID.md#52-locality-placement-ph-s61-canonical";
const DOC_GALAXY_66: &str =
    "https://github.com/platinoff/poolAI/blob/main/docs/concept/POOLAI_GALAXY_GRID.md#66-untrusted-telegram_edge";

fn compat_status_wire(status: CompatStatus) -> &'static str {
    match status {
        CompatStatus::Accepted => "accepted",
        CompatStatus::UpgradeRequired => "upgrade_required",
        CompatStatus::Unsupported => "unsupported",
    }
}

fn update_policy_wire(mode: UpdatePolicyMode) -> &'static str {
    match mode {
        UpdatePolicyMode::Notify => "notify",
        UpdatePolicyMode::Auto => "auto",
        UpdatePolicyMode::Never => "never",
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
    let update_policy = update_policy_wire(update_policy_from_env());
    let manifest_url = release_manifest_url_from_env().unwrap_or_else(|| "—".to_string());

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

          <div class="admin-card" id="updates-compat-policy">
            <h3 data-i18n="admin.updatesCompat.policyTitle">Update policy</h3>
            <p class="muted" data-i18n="admin.updatesCompat.policyHint">
              Runtime env readout for Galaxy §9.5 opt-in update policy (PH-S549). Policy prose lives in docs.
            </p>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.updatePolicy">Update policy</span>
              <span class="stat-value"><code id="updates-compat-policy-mode">{update_policy}</code>
                <span class="muted">(<code>{env_update_policy}</code>)</span></span>
            </div>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.manifestUrl">Release manifest URL</span>
              <span class="stat-value"><code id="updates-compat-manifest-url">{manifest_url}</code>
                <span class="muted">(<code>{env_manifest_url}</code>)</span></span>
            </div>
          </div>

          <div class="admin-card" id="updates-compat-capability">
            <h3 data-i18n="admin.updatesCompat.capabilityTitle">Signed capability documents</h3>
            <p class="muted" data-i18n="admin.updatesCompat.capabilityHint">
              Galaxy §6.6 — <code>telegram_edge</code> workers must send a signed
              <code>capability_document</code> on <code>POST /api/v1/discovery/register-remote</code>
              (PH-S740). Unsigned requests return HTTP 403 and increment
              <code>galaxy_capability_unsigned_rejected_total</code>.
            </p>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.devVerifyPk">Dev verify public key</span>
              <span class="stat-value"><code id="updates-compat-capability-pk">{dev_verify_pk}</code></span>
            </div>
            <div class="stat-item">
              <span class="stat-label" data-i18n="admin.updatesCompat.col.capEnv">Env override</span>
              <span class="stat-value"><code>{env_capability_pk}</code>
                <span class="muted">(<code>{env_capability_key}</code> production alias)</span></span>
            </div>
            <ul>
              <li><a href="{doc_galaxy_66}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.galaxy66">Galaxy §6.6 untrusted telegram_edge</a></li>
              <li><a href="{doc_capability_fixture}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.capFixture">Dev capability verify key fixture</a></li>
            </ul>
          </div>

          <div class="admin-card" id="updates-compat-prefetch">
            <h3 data-i18n="admin.updatesCompat.prefetchTitle">Prefetch live pull metrics</h3>
            <p class="muted" data-i18n="admin.updatesCompat.prefetchHint">
              Galaxy §5.5 — read-only strip from <code>GET /api/v1/grid/prefetch-metrics</code>
              reconciled with <code>/metrics</code> (PH-S750…S752).
            </p>
            <div id="updates-compat-prefetch-strip" class="updates-compat-prefetch-strip muted" data-i18n="admin.updatesCompat.prefetchLoading">
              Loading prefetch metrics…
            </div>
            <p class="muted admin-hint">
              <a href="{doc_galaxy_55}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.galaxy55">Galaxy §5.5 task-driven prefetch</a>
            </p>
          </div>

          <div class="admin-card" id="updates-compat-locality">
            <h3 data-i18n="admin.updatesCompat.localityTitle">Locality / hot-tier metrics</h3>
            <p class="muted" data-i18n="admin.updatesCompat.localityHint">
              Galaxy §5.2–5.4 — read-only strip from <code>GET /api/v1/grid/locality-metrics</code>
              reconciled with <code>/metrics</code> (PH-S760…S762).
            </p>
            <div id="updates-compat-locality-strip" class="updates-compat-locality-strip muted" data-i18n="admin.updatesCompat.localityLoading">
              Loading locality metrics…
            </div>
            <p class="muted admin-hint">
              <a href="{doc_galaxy_52}" target="_blank" rel="noopener noreferrer" data-i18n="admin.updatesCompat.link.galaxy52">Galaxy §5.2 locality placement</a>
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
        update_policy = update_policy,
        manifest_url = manifest_url,
        env_update_policy = ENV_UPDATE_POLICY,
        env_manifest_url = ENV_RELEASE_MANIFEST_URL,
        dev_verify_pk = DEV_CAPABILITY_VERIFY_PK_HEX,
        env_capability_pk = ENV_CAPABILITY_VERIFY_PK_HEX,
        env_capability_key = "POOLAI_CAPABILITY_VERIFY_KEY",
        doc_galaxy_66 = DOC_GALAXY_66,
        doc_capability_fixture = DOC_CAPABILITY_FIXTURE,
        doc_galaxy_55 = DOC_GALAXY_55,
        doc_galaxy_52 = DOC_GALAXY_52,
    );

    let script = r#"
    function compatStatusLabel(status) {
      return window.poolaiUiWasm.compatStatusLabel(String(status || ''));
    }

    function protocolVersionLabel(raw) {
      return window.poolaiUiWasm.protocolVersionLabel(String(raw || ''));
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

    async function loadUpdatesCompatPrefetchStrip() {
      const el = document.getElementById('updates-compat-prefetch-strip');
      if (!el) return;
      try {
        let metricsJson = '{}';
        let pullBytes = 0;
        try {
          const metricsResp = await fetchJson('/api/v1/grid/prefetch-metrics');
          metricsJson = JSON.stringify(metricsResp || {});
        } catch (_) {}
        try {
          const promText = await fetch('/metrics').then(function(r) { return r.text(); });
          var wasm = window.poolaiUiWasm;
          if (wasm && typeof wasm.parsePrometheusGauge === 'function') {
            pullBytes = wasm.parsePrometheusGauge(promText, 'galaxy_prefetch_pull_bytes_total');
          } else {
            var m = promText.match(/galaxy_prefetch_pull_bytes_total\\s+(\\d+)/);
            if (m) pullBytes = parseInt(m[1], 10) || 0;
          }
        } catch (_) {}
        if (window.poolaiUiWasm && typeof window.poolaiUiWasm.renderGridPrefetchMetricsStrip === 'function') {
          el.innerHTML = window.poolaiUiWasm.renderGridPrefetchMetricsStrip(metricsJson, pullBytes || 0);
        } else {
          el.textContent = metricsJson;
        }
      } catch (e) {
        el.textContent = String(e && e.message ? e.message : e);
      }
    }

    async function loadUpdatesCompatLocalityStrip() {
      const el = document.getElementById('updates-compat-locality-strip');
      if (!el) return;
      try {
        const metricsResp = await fetchJson('/api/v1/grid/locality-metrics');
        const metricsJson = JSON.stringify(metricsResp);
        let hotPromote = 0;
        try {
          const promResp = await fetch('/metrics');
          const promText = await promResp.text();
          if (window.poolaiUiWasm && typeof window.poolaiUiWasm.parsePrometheusGauge === 'function') {
            hotPromote = wasm.parsePrometheusGauge(promText, 'galaxy_hot_promote_total');
          } else {
            var m = promText.match(/galaxy_hot_promote_total\\s+(\\d+)/);
            if (m) hotPromote = parseInt(m[1], 10) || 0;
          }
        } catch (_) {}
        if (window.poolaiUiWasm && typeof window.poolaiUiWasm.renderGridLocalityMetricsStrip === 'function') {
          el.innerHTML = window.poolaiUiWasm.renderGridLocalityMetricsStrip(metricsJson, hotPromote || 0);
        } else {
          el.textContent = metricsJson;
        }
      } catch (e) {
        el.textContent = String(e && e.message ? e.message : e);
      }
    }

    function startUpdatesCompatPage() {
      wireUpdatesCompatLabels();
      loadUpdatesCompatPrefetchStrip();
      loadUpdatesCompatLocalityStrip();
    }

    if (window.poolaiUiWasm && (window.poolaiUiWasm.ready || window.poolaiUiWasm.failed)) {
      startUpdatesCompatPage();
    } else {
      window.addEventListener('poolai-ui-wasm-ready', startUpdatesCompatPage, { once: true });
    }
    "#;

    admin_layout_updates_compat(
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
    assert!(html.contains("id=\"updates-compat-policy\""));
    assert!(html.contains("id=\"updates-compat-capability\""));
    assert!(html.contains("id=\"updates-compat-prefetch\""));
    assert!(html.contains("id=\"updates-compat-verify-release\""));
    assert!(html.contains("id=\"updates-compat-matrix\""));
    assert!(html.contains("id=\"updates-compat-negotiation-status\""));
    assert!(html.contains("poolai-verify-release"));
    assert!(html.contains("POOLAI_COORDINATOR_PROTOCOL_VERSION"));
    assert!(html.contains(ENV_UPDATE_POLICY));
    assert!(html.contains(ENV_RELEASE_MANIFEST_URL));
    assert!(html.contains(MIN_COORDINATOR_VERSION_DOCS_URL));
    assert!(html.contains(DEV_CAPABILITY_VERIFY_PK_HEX));
    assert!(html.contains(ENV_CAPABILITY_VERIFY_PK_HEX));
    assert!(html.contains("galaxy_capability_unsigned_rejected_total"));
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.updatesCompat.section""#));
}

#[tokio::test]
async fn admin_updates_compat_page_slim_updates_compat_i18n_patch_ph_s221() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("window.__poolaiAdminI18nRust="));
    assert!(html.contains(r#""admin.page.updatesCompat""#));
    assert!(html.contains(r#""admin.updatesCompat.protocolTitle""#));
    assert!(!html.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!html.contains(r#""admin.mon.mlTitle""#));
}

#[tokio::test]
async fn admin_updates_compat_page_wires_poolai_ui_wasm_labels() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("type=\"module\""));
    assert!(html.contains("/ui/wasm/poolai_ui_wasm.js"));
    assert!(html.contains("window.poolaiUiWasm"));
    assert!(html.contains("poolai-ui-wasm-ready"));
    assert!(html.contains("window.poolaiUiWasm.compatStatusLabel"));
    assert!(!html.contains("compatStatusLabelFallback"));
    assert!(html.contains("protocolVersionLabel"));
}

#[tokio::test]
async fn admin_updates_compat_prefetch_wasm_glue_ph_s752() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("/api/v1/grid/prefetch-metrics"));
    assert!(html.contains("renderGridPrefetchMetricsStrip"));
    assert!(html.contains("updates-compat-prefetch-strip"));
    assert!(html.contains("loadUpdatesCompatPrefetchStrip"));
}

#[tokio::test]
async fn admin_updates_compat_locality_wasm_glue_ph_s762() {
    let html = admin_updates_compat().await.0;
    assert!(html.contains("id=\"updates-compat-locality\""));
    assert!(html.contains("/api/v1/grid/locality-metrics"));
    assert!(html.contains("renderGridLocalityMetricsStrip"));
    assert!(html.contains("updates-compat-locality-strip"));
    assert!(html.contains("loadUpdatesCompatLocalityStrip"));
}
