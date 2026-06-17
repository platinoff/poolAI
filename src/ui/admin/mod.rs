//! Admin Panel module
//!
//! Provides comprehensive administrative interface with full system management capabilities.
//!
//! # Features
//!
//! - **System Overview**: Real-time system status, health, and metrics
//! - **Tenant Management**: Create, update, delete tenants, manage quotas
//! - **Security Management**: OAuth2/SAML providers, security policies
//! - **Audit Logs**: View and query audit events
//! - **Monitoring Dashboard**: Real-time metrics, alerts, custom dashboards
//! - **VM Management**: Full VM instance lifecycle management
//! - **Worker Management**: Worker pool configuration and monitoring
//! - **Library Management**: Model library administration
//! - **RAID Management**: Artifact storage and replication
//! - **User Management**: User accounts, roles, permissions
//! - **System Configuration**: Advanced settings and policies
//!
//! # Routes
//!
//! - `/ui/admin` - Admin dashboard home
//! - `/ui/admin/tenants` - Tenant management
//! - `/ui/admin/security` - Security settings
//! - `/ui/admin/audit` - Audit logs viewer
//! - `/ui/admin/monitoring` - Monitoring dashboard
//! - `/ui/admin/vm` - VM management
//! - `/ui/admin/workers` - Worker management
//! - `/ui/admin/libs` - Library management
//! - `/ui/admin/raid` - RAID management
//! - `/ui/admin/users` - User management
//! - `/ui/admin/config` - System configuration
//! - `/ui/admin/jobs` - Job queue and store backend (PH-S53)
//! - `/ui/admin/grid-pricing` - Galaxy grid pricing snapshot (PH-S82)
//! - `/ui/admin/updates-compat` - Galaxy updates & compatibility pointers (PH-S93)

pub mod audit;
pub mod config;
pub mod dashboard;
pub mod grid_pricing;
pub mod instances;
pub mod jobs;
pub mod libs;
pub mod monitoring;
pub mod raid;
pub mod security;
pub mod tenants;
pub mod topology;
pub mod updates_compat;
pub mod users;
pub mod vm;
pub mod workers;

use crate::core::state::ApiContext;
use axum::{response::Html, routing::get, Router};

/// Admin panel routes
pub fn create_admin_routes() -> Router<ApiContext> {
    Router::new()
        .route("/admin", get(dashboard::admin_dashboard))
        .route("/admin/tenants", get(tenants::admin_tenants))
        .route("/admin/security", get(security::admin_security))
        .route("/admin/audit", get(audit::admin_audit))
        .route("/admin/monitoring", get(monitoring::admin_monitoring))
        .route("/admin/vm", get(vm::admin_vm))
        .route("/admin/workers", get(workers::admin_workers))
        .route("/admin/jobs", get(jobs::admin_jobs))
        .route("/admin/grid-pricing", get(grid_pricing::admin_grid_pricing))
        .route(
            "/admin/updates-compat",
            get(updates_compat::admin_updates_compat),
        )
        .route("/admin/libs", get(libs::admin_libs))
        .route("/admin/raid", get(raid::admin_raid))
        .route("/admin/instances", get(instances::admin_instances))
        .route("/admin/topology", get(topology::admin_topology))
        .route("/admin/users", get(users::admin_users))
        .route("/admin/config", get(config::admin_config))
}

pub use crate::ui::wasm_static::POOLAI_UI_WASM_MODULE;

/// Admin panel layout function - shared across all admin pages
pub fn admin_layout(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Jobs admin page layout — slim `admin.jobs.*` Rust i18n patch only (PH-S211).
pub fn admin_layout_jobs(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_jobs_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// RAID admin page layout — slim `admin.raidadm.*` Rust i18n patch only (PH-S214).
pub fn admin_layout_raid(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_raid_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Grid-pricing admin page layout — slim `admin.gridPricing.*` Rust i18n patch only (PH-S217).
pub fn admin_layout_grid_pricing(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_grid_pricing_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Like [`admin_layout`] but inserts a `<script type="module">` before the page IIFE (PH-S151 wasm wiring).
pub fn admin_layout_with_module_script(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    module_script: &str,
    script_js: &str,
    i18n_patch_script: &str,
) -> Html<String> {
    let base_css = format!(
        "{}{}",
        poolai_ui_core::design_tokens::admin_base_css(),
        include_str!("../admin_styles.css"),
    );
    let i18n_js = include_str!("../i18n_core.js");
    let theme_js = include_str!("../admin_theme.js");
    let common_js = include_str!("../admin_common.js");
    let modal_js = include_str!("../admin_modal_a11y.js");
    let charts_js = include_str!("../admin_charts.js");
    let i18n_patch = i18n_patch_script;
    let auth_dash_patch = poolai_ui_core::i18n::auth_dash_shell_patch_script();
    let theme_patch = poolai_ui_core::theme::admin_theme_patch_script();
    let modal_patch = poolai_ui_core::modal::admin_modal_patch_script();
    let module_block = if module_script.is_empty() {
        String::new()
    } else {
        format!(
            r#"<script type="module">
{module_script}
</script>"#
        )
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>PoolAI Admin</title>
  <style>{base_css}</style>
</head>
<body>
  <a href="{skip_admin_main_href}" class="skip_link" data-i18n="admin.skipMain">Skip to main content</a>
  <a href="{skip_admin_nav_href}" class="skip_link" data-i18n="admin.skipNav">Skip to navigation</a>
  <div class="admin-wrapper">
    <aside class="admin-sidebar" role="navigation" aria-label="Admin navigation">
      <div class="admin-brand">
        <h1 data-i18n="admin.brand">PoolAI Admin</h1>
        <div class="admin-version">v0.1.0</div>
      </div>
      <nav class="admin-nav" id="admin_nav">
        <a href="/ui/admin" class="admin-nav-item" data-i18n="admin.nav.dashboard">Dashboard</a>
        <a href="/ui/admin/tenants" class="admin-nav-item" data-i18n="admin.nav.tenants">Tenants</a>
        <a href="/ui/admin/security" class="admin-nav-item" data-i18n="admin.nav.security">Security</a>
        <a href="/ui/admin/audit" class="admin-nav-item" data-i18n="admin.nav.audit">Audit Logs</a>
        <a href="/ui/admin/monitoring" class="admin-nav-item" data-i18n="admin.nav.monitoring">Monitoring</a>
        <a href="/ui/admin/vm" class="admin-nav-item" data-i18n="admin.nav.vm">VM Instances</a>
        <a href="/ui/admin/workers" class="admin-nav-item" data-i18n="admin.nav.workers">Workers</a>
        <a href="/ui/admin/jobs" class="admin-nav-item" data-i18n="admin.nav.jobs">Jobs</a>
        <a href="/ui/admin/grid-pricing" class="admin-nav-item" data-i18n="admin.nav.gridPricing">Grid pricing</a>
        <a href="/ui/admin/updates-compat" class="admin-nav-item" data-i18n="admin.nav.updatesCompat">Updates</a>
        <a href="/ui/admin/libs" class="admin-nav-item" data-i18n="admin.nav.libs">Libraries</a>
        <a href="/ui/admin/raid" class="admin-nav-item" data-i18n="admin.nav.raid">RAID</a>
        <a href="/ui/admin/instances" class="admin-nav-item" data-i18n="admin.nav.instances">Model Instances</a>
        <a href="/ui/admin/topology" class="admin-nav-item" data-i18n="admin.nav.topology">Topology</a>
        <a href="/ui/admin/users" class="admin-nav-item" data-i18n="admin.nav.users">Users</a>
        <a href="/ui/admin/config" class="admin-nav-item" data-i18n="admin.nav.config">Configuration</a>
      </nav>
    </aside>
    
    <main class="admin-main" role="main">
      <header class="admin-header-bar">
        <h2 data-i18n="{title_key}">{title_fallback}</h2>
        <div class="admin-user-menu">
          <div id="poolai-lang-toggle" class="admin-lang-bar"></div>
          <span id="admin-user-name">Admin</span>
          <button type="button" class="btn-icon" onclick="logout()" data-i18n-aria="admin.logout" aria-label="Log out">🚪</button>
        </div>
      </header>
      
      <div id="admin_main_content" class="admin-content" tabindex="-1">
        <div id="poolai-bootstrap-banner-host" class="poolai-bootstrap-banner-host" hidden></div>
        {body}
      </div>
    </main>
  </div>
  <div id="admin-aria-live" class="sr-only" aria-live="polite" aria-atomic="true"></div>
  
  <script>{i18n_patch}</script>
  <script>{auth_dash_patch}</script>
  <script>{theme_patch}</script>
  <script>{modal_patch}</script>
  <script>{i18n_js}</script>
  <script>{theme_js}</script>
  <script>{common_js}</script>
  <script>{modal_js}</script>
  <script>{charts_js}</script>
  {module_block}
  <script>
    (function() {{
      function adminSyncDocTitle() {{
        var h2 = document.querySelector('.admin-header-bar h2');
        if (h2 && typeof PoolAiI18n !== 'undefined') {{
          document.title = h2.textContent.trim() + PoolAiI18n.t('admin.browserSuffix');
        }}
      }}
      if (typeof PoolAiI18n !== 'undefined') {{
        if (typeof poolaiInitThemeFromStorage === 'function') poolaiInitThemeFromStorage();
        document.documentElement.lang = PoolAiI18n.getLang() === 'uk' ? 'uk' : 'en';
        PoolAiI18n.apply(document.body);
        PoolAiI18n.initAdminShell();
        adminSyncDocTitle();
        if (typeof adminMarkCurrentNav === 'function') adminMarkCurrentNav();
        document.addEventListener('poolai:langchange', function() {{
          PoolAiI18n.apply(document.body);
          adminSyncDocTitle();
          if (typeof adminMarkCurrentNav === 'function') adminMarkCurrentNav();
        }});
      }}
      if (!requireAdmin()) {{
        return;
      }}
      {script}
    }})();
  </script>
</body>
</html>"#,
        title_key = title_i18n_key,
        title_fallback = title_fallback,
        skip_admin_main_href = "#admin_main_content",
        skip_admin_nav_href = "#admin_nav",
        base_css = base_css,
        body = body_html,
        i18n_patch = i18n_patch,
        auth_dash_patch = auth_dash_patch,
        theme_patch = theme_patch,
        modal_patch = modal_patch,
        i18n_js = i18n_js,
        common_js = common_js,
        charts_js = charts_js,
        module_block = module_block,
        script = script_js
    );

    Html(html)
}

#[test]
fn admin_layout_injects_rust_theme_patch_ph_s160() {
    let patch = poolai_ui_core::theme::admin_theme_patch_script();
    assert!(patch.contains("window.__poolaiAdminThemesRust="));
    assert!(patch.contains(r#""high-contrast""#));
    assert!(patch.contains("\"surfaceSecondary\":\"#1e2329\""));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAdminThemesRust="));
}

#[test]
fn admin_layout_injects_monitoring_i18n_ph_s207() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(patch.contains(r#""admin.page.monitoring""#));
    assert!(patch.contains(r#""admin.mon.mlTitle""#));
    assert!(patch.contains(r#""admin.mon.createDashBtn""#));
}

#[test]
fn admin_layout_injects_rust_i18n_patch_ph_s154() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(patch.contains("window.__poolaiAdminI18nRust="));
    assert!(patch.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!patch.contains(r#""admin.gridPricing.section""#));
}

#[test]
fn admin_layout_injects_rust_auth_dash_i18n_patch_ph_s162() {
    let patch = poolai_ui_core::i18n::auth_dash_shell_patch_script();
    assert!(patch.contains("window.__poolaiAuthDashI18nRust="));
    assert!(patch.contains(r#""auth.pageTitle""#));
    assert!(patch.contains(r#""dash.nav.home""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAuthDashI18nRust="));
}

#[test]
fn admin_charts_layer_exports() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("function poolaiFetchMetricHistory"));
    assert!(js.contains("function poolaiRenderLineChart"));
    assert!(js.contains("function poolaiRenderSparkline"));
    assert!(js.contains("function poolaiRenderMetricsChartGrid"));
    assert!(js.contains("function poolaiStartMetricsPolling"));
    assert!(js.contains("function poolaiRenderMlPipelineMetricsPanel"));
    assert!(js.contains("function poolaiFetchMlPipelines"));
    assert!(js.contains("poolaiChartsWasm"));
    assert!(js.contains("chartScale"));
    assert!(js.contains("flattenMlStepRows"));
}

#[tokio::test]
async fn admin_monitoring_ph_s43_ml_metrics_panel() {
    let html = monitoring::admin_monitoring().await.0;
    assert!(html.contains("id=\"ml-demo-btn\""));
    assert!(html.contains("runMlPipelineDemo"));
    assert!(html.contains("poolaiFetchMlPipelines"));
    assert!(html.contains("poolaiRenderMlPipelineMetricsPanel"));
    assert!(html.contains("ml-pipeline-metrics-panel"));
}

#[test]
fn admin_layout_includes_charts_script() {
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("function poolaiRenderLineChart"));
}

#[test]
fn admin_common_fm019_modal_a11y_helpers() {
    let js = concat!(
        include_str!("../admin_common.js"),
        include_str!("../admin_modal_a11y.js"),
    );
    assert!(js.contains("function keepFocusInModal"));
    assert!(js.contains("function showModalContent"));
    assert!(js.contains("ADMIN_DYNAMIC_MODAL_ID"));
    assert!(js.contains("function handleModalEscape"));
    assert!(js.contains("function adminSyncTabA11y"));
    assert!(js.contains("__poolaiAdminModalRust"));
    assert!(js.contains("function poolaiTrapTabAction"));
    assert!(js.contains("function adminEnhanceFormA11y"));
    assert!(js.contains("function adminEnhanceTablesA11y"));
    assert!(js.contains("function adminObserveDynamicA11y"));
    assert!(js.contains("function adminApplyDesignSystem"));
    assert!(js.contains("function adminRenderTable"));
    assert!(js.contains("function adminFormFieldHtml"));
}

#[test]
fn admin_layout_injects_rust_modal_patch_ph_s161() {
    let patch = poolai_ui_core::modal::admin_modal_patch_script();
    assert!(patch.contains("window.__poolaiAdminModalRust="));
    assert!(patch.contains("focusable_selector"));
    assert!(patch.contains("adminDynamicModal"));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAdminModalRust="));
}

#[test]
fn admin_common_ph_s42_table_ux_helpers() {
    let js = include_str!("../admin_common.js");
    assert!(js.contains("function adminEmptyStateHtml"));
    assert!(js.contains("function adminFilterTable"));
    assert!(js.contains("function adminSortTable"));
    assert!(js.contains("function adminExportTableCsv"));
    assert!(js.contains("function adminExportTableJson"));
    assert!(js.contains("function adminEnhanceAdminTable"));
    assert!(js.contains("function adminInitTablesIn"));
    assert!(js.contains("admin-table-toolbar"));
    assert!(js.contains("admin-empty-state"));
}

#[test]
fn admin_common_ph_s14_high_contrast_theme() {
    let js = include_str!("../admin_theme.js");
    assert!(js.contains("'high-contrast'"));
    assert!(js.contains("function poolaiNormalizeTheme"));
    assert!(js.contains("__poolaiAdminThemesRust"));
    assert!(!js.contains("const POOLAI_UI_THEMES"));
}

#[test]
fn admin_layout_includes_design_tokens_css_ph_s166() {
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    let body = html.0;
    assert!(body.contains("--spacing-1: 4px"));
    assert!(body.contains("--admin-sidebar-width: 260px"));
    assert!(body.contains("admin-table--striped"));
    assert!(body.contains("admin-table-toolbar"));
    assert!(body.contains("admin-empty-state"));
}

#[test]
fn admin_styles_btn_primary_contrast_rules() {
    let css = include_str!("../admin_styles.css");
    assert!(
        css.contains(".btn:not(.btn-primary):not(.btn-danger):not(.btn-secondary):hover"),
        "generic .btn:hover must not recolor solid variants"
    );
    assert!(
        css.contains(".btn.btn-primary:hover"),
        "btn-primary hover must pin foreground to --bg"
    );
    assert!(
        css.contains(".btn-primary::before"),
        "btn-primary ripple overlay disabled for axe contrast"
    );
    assert!(
        css.contains(".modal[aria-hidden=\"true\"]"),
        "closed modals hidden from layout/axe"
    );
}

#[cfg(all(test, feature = "enterprise"))]
mod a11y_tests {
    use super::admin_layout;
    use crate::ui::admin::config::admin_config;
    use crate::ui::admin::security::admin_security;
    use crate::ui::admin::users::admin_users;

    #[test]
    fn admin_layout_includes_skip_links_and_live_region() {
        let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
        let body = html.0;
        assert!(body.contains("class=\"skip_link\""));
        assert!(body.contains("id=\"admin_main_content\""));
        assert!(body.contains("id=\"admin_nav\""));
        assert!(body.contains("id=\"admin-aria-live\""));
        assert!(body.contains("role=\"navigation\""));
        assert!(body.contains("role=\"main\""));
    }

    #[tokio::test]
    async fn users_modals_closed_aria_state() {
        let html = admin_users().await.0;
        assert!(html.contains("id=\"createUserModal\""));
        assert!(html.contains("id=\"editUserModal\""));
        assert!(html.contains("role=\"dialog\""));
        assert!(html.contains("aria-modal=\"false\" aria-hidden=\"true\""));
    }

    #[tokio::test]
    async fn users_create_form_a11y_attributes() {
        let html = admin_users().await.0;
        assert!(html.contains("id=\"userUsername\""));
        assert!(html.contains("aria-required=\"true\""));
        assert!(html.contains("autocomplete=\"username\""));
        assert!(html.contains("autocomplete=\"new-password\""));
        assert!(html.contains("class=\"required\" aria-hidden=\"true\""));
    }

    #[tokio::test]
    async fn security_tablist_semantic_roles() {
        let html = admin_security().await.0;
        assert!(html.contains("role=\"tablist\""));
        assert!(html.contains("role=\"tabpanel\""));
        assert!(html.contains("id=\"security-tab-oauth2\""));
        assert!(html.contains("id=\"security-tab-rotation\""));
        assert!(html.contains("aria-controls=\"security-content\""));
    }

    #[tokio::test]
    async fn config_tablist_semantic_roles() {
        let html = admin_config().await.0;
        assert!(html.contains("role=\"tablist\""));
        assert!(html.contains("id=\"config-tab-general\""));
        assert!(html.contains("aria-labelledby=\"config-tab-general\""));
    }

    #[tokio::test]
    async fn security_modals_closed_aria_state() {
        let html = admin_security().await.0;
        assert!(html.contains("id=\"createOAuth2Modal\""));
        assert!(html.contains("id=\"editPolicyModal\""));
        assert!(html.contains("aria-modal=\"false\" aria-hidden=\"true\""));
    }

    #[tokio::test]
    async fn workers_modal_closed_aria_state() {
        use crate::ui::admin::workers::admin_workers;
        let html = admin_workers().await.0;
        assert!(html.contains("id=\"createWorkerModal\""));
        assert!(!html.contains("aria-modal=\"true\" aria-hidden=\"true\""));
        assert!(html.contains("aria-modal=\"false\" aria-hidden=\"true\""));
    }
}
