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
pub mod grid_replication_pricing;
pub mod grid_verification;
pub mod instances;
pub mod jobs;
pub mod libs;
pub mod monitoring;
pub mod network_profiles;
pub mod payout_batch;
pub mod raid;
pub mod security;
pub mod security_advisories;
pub mod seed_inventory;
pub mod telegram_seats;
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
            "/admin/grid-verification",
            get(grid_verification::admin_grid_verification),
        )
        .route(
            "/admin/grid-replication-pricing",
            get(grid_replication_pricing::admin_grid_replication_pricing),
        )
        .route(
            "/admin/telegram-seats",
            get(telegram_seats::admin_telegram_seats),
        )
        .route(
            "/admin/network-profiles",
            get(network_profiles::admin_network_profiles),
        )
        .route(
            "/admin/seed-inventory",
            get(seed_inventory::admin_seed_inventory),
        )
        .route(
            "/admin/security-advisories",
            get(security_advisories::admin_security_advisories),
        )
        .route(
            "/admin/updates-compat",
            get(updates_compat::admin_updates_compat),
        )
        .route("/admin/payout-batch", get(payout_batch::admin_payout_batch))
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

/// Dashboard admin page layout — slim `admin.dash.*` Rust i18n patch only (PH-S228).
pub fn admin_layout_dashboard(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_dashboard_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Audit admin page layout — slim `admin.audit.*` Rust i18n patch only (PH-S229).
pub fn admin_layout_audit(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_audit_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Tenants admin page layout — slim `admin.tenants.*` Rust i18n patch only (PH-S230).
pub fn admin_layout_tenants(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_tenants_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Security admin page layout — slim `admin.sec.*` Rust i18n patch only (PH-S231).
pub fn admin_layout_security(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_security_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Topology admin page layout — slim `admin.topo.*` Rust i18n patch only (PH-S234).
pub fn admin_layout_topology(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_topology_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Instances admin page layout — slim `admin.inst.*` Rust i18n patch only (PH-S236).
pub fn admin_layout_instances(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_instances_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// VM admin page layout — slim `admin.vmadm.*` Rust i18n patch only (PH-S237).
pub fn admin_layout_vm(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_vm_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Users admin page layout — slim `admin.usr.*` Rust i18n patch only (PH-S238).
pub fn admin_layout_users(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_users_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Config admin page layout — slim `admin.cfg.*` Rust i18n patch only (PH-S239).
pub fn admin_layout_config(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_config_patch_script();
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

/// Monitoring admin page layout — slim `admin.mon.*` Rust i18n patch only (PH-S220).
pub fn admin_layout_monitoring(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_monitoring_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Updates-compat admin page layout — slim `admin.updatesCompat.*` Rust i18n patch only (PH-S221).
pub fn admin_layout_updates_compat(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_updates_compat_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Workers admin page layout — slim `admin.wrk.*` Rust i18n patch only (PH-S222).
pub fn admin_layout_workers(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_workers_patch_script();
    admin_layout_with_module_script(
        title_i18n_key,
        title_fallback,
        body_html,
        POOLAI_UI_WASM_MODULE,
        script_js,
        &i18n_patch,
    )
}

/// Libraries admin page layout — slim `admin.lib.*` Rust i18n patch only (PH-S223).
pub fn admin_layout_libs(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let i18n_patch = poolai_ui_core::i18n::admin_libs_patch_script();
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
    let table_patch = poolai_ui_core::i18n::admin_table_patch_script();
    let status_patch = poolai_ui_core::i18n::admin_status_patch_script();
    let err_patch = poolai_ui_core::i18n::admin_err_patch_script();
    let form_patch = poolai_ui_core::i18n::admin_form_patch_script();
    let ui_toolbar_patch = poolai_ui_core::i18n::admin_ui_toolbar_patch_script();
    let ui_common_patch = poolai_ui_core::i18n::admin_ui_common_patch_script();
    let vm_modal_patch = poolai_ui_core::i18n::vm_modal_patch_script();
    let ui_confirm_patch = poolai_ui_core::i18n::admin_ui_confirm_patch_script();
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
        <a href="/ui/admin/grid-verification" class="admin-nav-item" data-i18n="admin.nav.gridVerification">Grid verify</a>
        <a href="/ui/admin/telegram-seats" class="admin-nav-item" data-i18n="admin.nav.telegramSeats">Telegram seats</a>
        <a href="/ui/admin/network-profiles" class="admin-nav-item" data-i18n="admin.nav.networkProfiles">Network profiles</a>
        <a href="/ui/admin/seed-inventory" class="admin-nav-item" data-i18n="admin.nav.seedInventory">Seed inventory</a>
        <a href="/ui/admin/security-advisories" class="admin-nav-item" data-i18n="admin.nav.securityAdvisories">Advisories</a>
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
  <script>{table_patch}</script>
  <script>{status_patch}</script>
  <script>{err_patch}</script>
  <script>{form_patch}</script>
  <script>{ui_toolbar_patch}</script>
  <script>{ui_common_patch}</script>
  <script>{vm_modal_patch}</script>
  <script>{ui_confirm_patch}</script>
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
    let patch = poolai_ui_core::i18n::admin_monitoring_patch_script();
    assert!(patch.contains(r#""admin.page.monitoring""#));
    assert!(patch.contains(r#""admin.mon.mlTitle""#));
    assert!(patch.contains(r#""admin.mon.createDashBtn""#));
    assert!(!patch.contains(r#""admin.jobs.leaseState.active""#));
}

#[test]
fn admin_layout_default_patch_excludes_monitoring_ph_s220() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.mon.mlTitle""#));
    assert!(!patch.contains(r#""admin.page.monitoring""#));
    assert!(!patch.contains(r#""admin.dash.card.overview""#));
    assert!(!patch.contains(r#""admin.audit.sectionTitle""#));
}

#[test]
fn admin_layout_default_patch_excludes_dashboard_ph_s228() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.dash.card.overview""#));
    assert!(!patch.contains(r#""admin.page.dashboard""#));
}

#[test]
fn admin_layout_default_patch_excludes_audit_ph_s229() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.audit.sectionTitle""#));
    assert!(!patch.contains(r#""admin.page.audit""#));
}

#[test]
fn admin_layout_default_patch_excludes_tenants_ph_s230() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.tenants.section""#));
    assert!(!patch.contains(r#""admin.page.tenants""#));
    assert!(!patch.contains(r#""admin.tenants.col.name""#));
}

#[test]
fn admin_layout_default_patch_excludes_security_ph_s231() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.sec.tab.oauth""#));
    assert!(!patch.contains(r#""admin.page.security""#));
}

#[test]
fn admin_layout_default_patch_excludes_topology_ph_s234() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.topo.title""#));
    assert!(!patch.contains(r#""admin.page.topology""#));
}

#[test]
fn admin_layout_default_patch_excludes_instances_ph_s236() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.inst.title""#));
    assert!(!patch.contains(r#""admin.page.instances""#));
}

#[test]
fn admin_layout_default_patch_excludes_vm_ph_s237() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.vmadm.section""#));
    assert!(!patch.contains(r#""admin.page.vm""#));
}

#[test]
fn admin_layout_default_patch_excludes_users_ph_s238() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.usr.section""#));
    assert!(!patch.contains(r#""admin.page.users""#));
}

#[test]
fn admin_layout_default_patch_excludes_config_ph_s239() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.cfg.tab.general""#));
    assert!(!patch.contains(r#""admin.page.config""#));
}

#[test]
fn admin_layout_default_patch_excludes_updates_compat_ph_s221() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.updatesCompat.section""#));
    assert!(!patch.contains(r#""admin.page.updatesCompat""#));
}

#[test]
fn admin_layout_injects_rust_i18n_patch_ph_s154() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(patch.contains("window.__poolaiAdminI18nRust="));
    assert!(patch.contains(r#""admin.jobs.leaseState.active""#));
    assert!(!patch.contains(r#""admin.gridPricing.section""#));
}

#[test]
fn admin_layout_injects_rust_table_i18n_patch_ph_s240() {
    let patch = poolai_ui_core::i18n::admin_table_patch_script();
    assert!(patch.contains("window.__poolaiAdminTableI18nRust="));
    assert!(patch.contains(r#""admin.table.empty""#));
    assert!(patch.contains(r#""admin.table.sortedBy""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAdminTableI18nRust="));
    assert!(html.0.contains(r#""admin.table.searchPh""#));
}

#[test]
fn admin_layout_default_patch_excludes_table_ph_s240() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.table.empty""#));
    assert!(!patch.contains(r#""admin.table.exportCsv""#));
}

#[test]
fn admin_layout_injects_rust_status_i18n_patch_ph_s245() {
    let patch = poolai_ui_core::i18n::admin_status_patch_script();
    assert!(patch.contains("window.__poolaiAdminStatusI18nRust="));
    assert!(patch.contains(r#""admin.status.active""#));
    assert!(patch.contains(r#""admin.na""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAdminStatusI18nRust="));
    assert!(html.0.contains(r#""admin.btn.edit""#));
}

#[test]
fn admin_layout_default_patch_excludes_status_ph_s245() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""admin.status.active""#));
    assert!(!patch.contains(r#""admin.btn.edit""#));
}

#[test]
fn i18n_core_js_has_no_admin_status_keys_ph_s245() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'admin.status.active'"));
    assert!(!js.contains("'admin.status.inactive'"));
    assert!(!js.contains("'admin.status.yes'"));
    assert!(!js.contains("'admin.status.no'"));
    assert!(!js.contains("'admin.btn.edit'"));
    assert!(!js.contains("'admin.na'"));
}

#[test]
fn admin_layout_injects_rust_err_i18n_patch_ph_s246() {
    let patch = poolai_ui_core::i18n::admin_err_patch_script();
    assert!(patch.contains("window.__poolaiAdminErrI18nRust="));
    assert!(patch.contains(r#""err.hint403""#));
    assert!(patch.contains(r#""err.insufficientAdmin""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains("window.__poolaiAdminErrI18nRust="));
    assert!(html.0.contains(r#""admin.accessRequired""#));
}

#[test]
fn admin_layout_default_patch_excludes_err_hints_ph_s246() {
    let patch = poolai_ui_core::i18n::admin_jobs_grid_patch_script();
    assert!(!patch.contains(r#""err.hint403""#));
    assert!(!patch.contains(r#""err.insufficientAdmin""#));
}

#[test]
fn i18n_core_js_has_no_admin_err_keys_ph_s246() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'err.insufficientAdmin'"));
    assert!(!js.contains("'admin.accessRequired'"));
    assert!(!js.contains("'err.hint403'"));
    assert!(!js.contains("'err.hint503.generic'"));
    assert!(!js.contains("'err.hint503.raid'"));
    assert!(!js.contains("'err.hint503.library'"));
    assert!(!js.contains("'err.hint503.vm'"));
    assert!(!js.contains("'err.hint404.enterprise'"));
}

#[test]
fn admin_layout_injects_vm_modal_i18n_patch_ph_s248() {
    let patch = poolai_ui_core::i18n::vm_modal_patch_script();
    assert!(patch.contains("window.__poolaiVmModalI18nRust="));
    assert!(patch.contains(r#""vm.modalTitle""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""vm.confirmDelete""#));
}

#[test]
fn i18n_core_js_has_no_vm_modal_keys_ph_s248() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'vm.createBtn'"));
    assert!(!js.contains("'vm.modalTitle'"));
    assert!(!js.contains("'vm.confirmDelete'"));
}

#[test]
fn admin_layout_injects_ui_confirm_patch_ph_s252() {
    let patch = poolai_ui_core::i18n::admin_ui_confirm_patch_script();
    assert!(patch.contains("window.__poolaiAdminUiConfirmI18nRust="));
    assert!(patch.contains(r#""ui.confirmTitle""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""ui.closeDialogAria""#));
}

#[test]
fn i18n_core_js_has_no_ui_confirm_keys_ph_s252() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'ui.confirmTitle'"));
    assert!(!js.contains("'ui.confirmBtn'"));
    assert!(!js.contains("'ui.cancel'"));
    assert!(!js.contains("'ui.closeDialogAria'"));
}

#[test]
fn admin_layout_injects_form_and_ui_toolbar_patches_ph_s260() {
    let form = poolai_ui_core::i18n::admin_form_patch_script();
    let toolbar = poolai_ui_core::i18n::admin_ui_toolbar_patch_script();
    assert!(form.contains("window.__poolaiAdminFormI18nRust="));
    assert!(toolbar.contains("window.__poolaiAdminUiToolbarI18nRust="));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""ui.save""#));
    assert!(html.0.contains(r#""ui.searchTableAria""#));
    assert!(html.0.contains(r#""ui.retry""#));
}

#[test]
fn admin_layout_injects_ui_common_patch_ph_s263() {
    let patch = poolai_ui_core::i18n::admin_ui_common_patch_script();
    assert!(patch.contains("window.__poolaiUiCommonI18nRust="));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""common.loading""#));
    assert!(html.0.contains(r#""ui.create""#));
}

#[test]
fn i18n_core_js_has_no_workers_home_form_ui_toolbar_keys_ph_s257_s260() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'workers.empty'"));
    assert!(!js.contains("'home.apiTitle'"));
    assert!(!js.contains("'form.fieldRequired'"));
    assert!(!js.contains("'ui.save'"));
    assert!(!js.contains("'ui.searchTableAria'"));
    assert!(!js.contains("'ui.retry'"));
    assert!(!js.contains("'err.errorPrefix'"));
}

#[test]
fn i18n_core_js_has_no_common_or_residual_ui_keys_ph_s263() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'common.loading'"));
    assert!(!js.contains("'ui.create'"));
    assert!(!js.contains("'ui.suggestion.checkInternet'"));
}

#[test]
fn i18n_core_js_has_no_libs_or_raid_keys_ph_s264_s265() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'libs.empty'"));
    assert!(!js.contains("'raid.empty'"));
}

#[test]
fn admin_workers_patch_includes_workers_panel_keys_ph_s257() {
    let json = poolai_ui_core::i18n::admin_workers_patch_json();
    assert!(json.contains(r#""workers.modalTitle""#));
    assert!(json.contains(r#""admin.wrk.createBtn""#));
}

#[test]
fn admin_layout_injects_admin_nav_via_auth_dash_ph_s242() {
    let patch = poolai_ui_core::i18n::auth_dash_shell_patch_script();
    assert!(patch.contains(r#""admin.nav.dashboard""#));
    assert!(patch.contains(r#""admin.nav.config""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""admin.nav.jobs""#));
    assert!(html.0.contains(r#"data-i18n="admin.nav.dashboard""#));
}

#[test]
fn i18n_core_js_has_no_admin_nav_keys_ph_s242() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'admin.nav.dashboard'"));
    assert!(!js.contains("'admin.nav.config'"));
    assert!(!js.contains("'admin.nav.jobs'"));
}

#[test]
fn admin_layout_injects_admin_chrome_via_auth_dash_ph_s243() {
    let patch = poolai_ui_core::i18n::auth_dash_shell_patch_script();
    assert!(patch.contains(r#""admin.brand""#));
    assert!(patch.contains(r#""admin.skipMain""#));
    assert!(patch.contains(r#""admin.logout""#));
    let html = admin_layout("admin.test.page", "Test", "<p>body</p>", "");
    assert!(html.0.contains(r#""admin.browserSuffix""#));
    assert!(html.0.contains(r#"data-i18n="admin.brand""#));
}

#[test]
fn i18n_core_js_has_no_admin_chrome_keys_ph_s243() {
    let js = include_str!("../i18n_core.js");
    assert!(!js.contains("'admin.brand'"));
    assert!(!js.contains("'admin.skipMain'"));
    assert!(!js.contains("'admin.skipNav'"));
    assert!(!js.contains("'admin.lang.label'"));
    assert!(!js.contains("'admin.logout'"));
    assert!(!js.contains("'admin.browserSuffix'"));
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
    assert!(js.contains("renderMlPipelineMetricsPanel"));
    assert!(js.contains("renderSparklineHtml"));
    assert!(js.contains("renderLineChartHtml"));
    assert!(js.contains("groupMetricsByName"));
    assert!(js.contains("renderMetricsChartGridHtml"));
    assert!(js.contains("sanitizeChartId"));
    assert!(js.contains("renderLineChartEmptyHtml"));
    assert!(js.contains("buildMetricHistoryUrlWithHours"));
    assert!(js.contains("buildMetricsWindowUrlWithHours"));
    assert!(js.contains("PH-S1010"));
}

#[test]
#[test]
fn admin_charts_sparkline_wasm_first_ph_s275() {
    assert_admin_charts_sparkline_wasm_only();
}

fn assert_admin_charts_sparkline_wasm_only() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderSparklineHtml"));
    assert!(!js.contains("metric-sparkline-card"));
    assert!(!js.contains("<polyline"));
}

#[test]
fn admin_charts_sparkline_wasm_only_ph_s920() {
    assert_admin_charts_sparkline_wasm_only();
}

#[test]
fn admin_charts_line_chart_wasm_first_ph_s284() {
    assert_admin_charts_line_chart_wasm_only();
}

fn assert_admin_charts_line_chart_wasm_only() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderLineChartHtml"));
    assert!(js.contains("wasm.renderLineChartEmptyHtml"));
    assert!(!js.contains("escapeHtml(noData)"));
}

#[test]
fn admin_charts_line_chart_wasm_only_ph_s921() {
    assert_admin_charts_line_chart_wasm_only();
}

#[test]
fn admin_charts_sparkline_line_regression_ph_s922() {
    assert_admin_charts_sparkline_wasm_only();
    assert_admin_charts_line_chart_wasm_only();
}

#[test]
fn build_ui_wasm_script_gate_ph_s923() {
    let script = include_str!("../../../bin/build-ui-wasm.sh");
    assert!(script.contains("poolai-ui-wasm"));
    assert!(script.contains("wasm32-unknown-unknown"));
    assert!(script.contains("wasm-bindgen"));
}

#[test]
fn admin_charts_group_metrics_wasm_first_ph_s287() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.groupMetricsByName"));
}

#[test]
fn admin_charts_metrics_grid_wasm_first_ph_s294() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMetricsChartGridHtml"));
}

#[test]
fn admin_charts_sanitize_chart_id_wasm_first_ph_s297() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.sanitizeChartId"));
}

#[test]
fn admin_charts_line_chart_empty_wasm_first_ph_s304() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderLineChartEmptyHtml"));
}

#[test]
fn admin_charts_metric_history_url_wasm_first_ph_s314() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMetricHistoryUrl"));
}

#[test]
fn admin_charts_metrics_window_url_wasm_first_ph_s317() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMetricsWindowUrl"));
}

#[test]
fn admin_charts_ml_pipelines_url_wasm_first_ph_s324() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMlPipelinesUrl"));
}

#[test]
fn admin_charts_ml_pipeline_demo_url_wasm_first_ph_s327() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMlPipelineDemoUrl"));
}

#[test]
fn admin_charts_ml_pipeline_panel_wasm_first_ph_s450() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMlPipelineMetricsPanel"));
}

#[test]
fn admin_charts_monitoring_alerts_panel_wasm_first_ph_s461() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMonitoringAlertsPanel"));
    assert!(js.contains("poolaiRenderMonitoringAlertsPanel"));
    let mon = include_str!("monitoring.rs");
    assert!(mon.contains("poolaiRenderMonitoringAlertsPanel"));
}

#[test]
fn admin_charts_monitoring_dashboards_panel_wasm_first_ph_s470() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMonitoringDashboardsPanel"));
    assert!(js.contains("poolaiRenderMonitoringDashboardsPanel"));
    let mon = include_str!("monitoring.rs");
    assert!(mon.contains("poolaiRenderMonitoringDashboardsPanel"));
}

#[test]
fn admin_charts_workers_panel_wasm_first_ph_s480() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderWorkersPanel"));
    assert!(js.contains("poolaiRenderWorkersPanel"));
    let wrk = include_str!("workers.rs");
    assert!(wrk.contains("poolaiRenderWorkersPanel"));
}

#[test]
fn admin_charts_instances_panel_wasm_first_ph_s490() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderInstancesPanel"));
    assert!(js.contains("poolaiRenderInstancesPanel"));
    let inst = include_str!("instances.rs");
    assert!(inst.contains("poolaiRenderInstancesPanel"));
}

#[test]
fn admin_charts_vm_panel_wasm_first_ph_s499() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderVmPanel"));
    assert!(js.contains("poolaiRenderVmPanel"));
    let vm = include_str!("vm.rs");
    assert!(vm.contains("poolaiRenderVmPanel"));
}

#[test]
fn admin_jobs_store_badge_wasm_first_ph_s852() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderJobsStoreBadgeHtml"));
    assert!(js.contains("poolaiRenderJobsStoreBadge"));
    let jobs = include_str!("jobs.rs");
    assert!(jobs.contains("poolaiRenderJobsStoreBadge"));
}

#[test]
fn admin_seed_inventory_memory_meta_wasm_first_ph_s862() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMemorySeedMetaStripHtml"));
    assert!(js.contains("poolaiRenderMemorySeedMetaStrip"));
    assert!(js.contains("poolaiFormatSeedInventoryRamBytes"));
    let seed = include_str!("seed_inventory.rs");
    assert!(seed.contains("poolaiRenderMemorySeedMetaStrip"));
}

#[test]
fn admin_charts_metric_history_url_with_hours_wasm_first_ph_s334() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMetricHistoryUrlWithHours"));
}

#[test]
fn admin_charts_metrics_window_url_with_hours_wasm_first_ph_s337() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMetricsWindowUrlWithHours"));
}

#[test]
fn admin_charts_monitoring_alerts_url_wasm_first_ph_s344() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMonitoringAlertsUrl"));
    assert!(js.contains("poolaiFetchMonitoringAlerts"));
}

#[test]
fn admin_charts_monitoring_active_alerts_url_wasm_first_ph_s355() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMonitoringActiveAlertsUrl"));
    assert!(!js.contains("acknowledged=false"));
}

#[test]
fn admin_charts_alert_rules_url_wasm_first_ph_s347() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildAlertRulesUrl"));
    assert!(js.contains("poolaiFetchAlertRules"));
    assert!(js.contains("poolaiAlertRulesUrl"));
}

#[test]
fn admin_charts_monitoring_dashboards_url_wasm_first_ph_s353() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMonitoringDashboardsUrl"));
    assert!(js.contains("poolaiMonitoringDashboardsUrl"));
}

#[test]
fn admin_charts_monitoring_alert_ack_url_wasm_first_ph_s353() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMonitoringAlertAcknowledgeUrl"));
    assert!(js.contains("poolaiMonitoringAlertAcknowledgeUrl"));
}

#[test]
fn admin_dashboard_active_alerts_url_wasm_first_ph_s365() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("buildMonitoringActiveAlertsUrl"));
}

#[test]
fn admin_charts_monitoring_metric_latest_url_wasm_first_ph_s366() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.buildMonitoringMetricLatestUrl"));
    assert!(js.contains("poolaiMonitoringMetricLatestUrl"));
}

#[test]
fn admin_dashboard_audit_events_url_wasm_first_ph_s375() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("buildAuditEventsUrl"));
}

#[test]
fn admin_dashboard_overview_url_wasm_first_ph_s376() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("buildAdminOverviewUrl"));
}

#[test]
fn admin_dashboard_wasm_glue_tests_ph_s378() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("buildMonitoringActiveAlertsUrl"));
    assert!(dash.contains("buildAuditEventsUrl"));
    assert!(dash.contains("buildAdminOverviewUrl"));
}

#[test]
fn admin_dashboard_format_uptime_wasm_first_ph_s385() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("formatUptime"));
}

#[test]
fn admin_dashboard_metrics_window_url_wasm_first_ph_s386() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("buildDashboardMetricsWindowUrl"));
}

#[test]
fn admin_dashboard_wasm_glue_tests_ph_s388() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("formatUptime"));
    assert!(dash.contains("buildDashboardMetricsWindowUrl"));
    assert!(dash.contains("buildAdminOverviewUrl"));
}

#[test]
fn admin_dashboard_format_iso_datetime_wasm_first_ph_s396() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("formatIsoDatetime"));
    assert!(script.contains("formatAuditTimestamp"));
}

#[test]
fn admin_dashboard_wasm_glue_tests_ph_s398() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("window.poolaiUiWasm.formatIsoDatetime"));
    assert!(dash.contains("formatAuditTimestamp"));
    assert!(!dash.contains("toLocaleString()"));
    assert!(dash.contains("buildMonitoringActiveAlertsUrl"));
}

#[test]
fn admin_dashboard_alert_severity_wasm_first_ph_s406() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("alertSeverityBadgeClass"));
}

#[test]
fn admin_dashboard_wasm_glue_tests_ph_s408() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("alertSeverityBadgeClass"));
    assert!(dash.contains("buildMonitoringActiveAlertsUrl"));
    assert!(dash.contains("formatAuditTimestamp"));
}

#[test]
fn admin_dashboard_refreshed_at_wasm_first_ph_s416() {
    let script = include_str!("dashboard.rs");
    assert!(script.contains("formatLocaleTimeHms"));
    assert!(script.contains("updateDashboardRefreshedAt"));
    assert!(script.contains("dash-refreshed-at"));
}

#[test]
fn admin_dashboard_wasm_glue_tests_ph_s418() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("updateDashboardRefreshedAt"));
    assert!(dash.contains("dash-refreshed-at"));
    assert!(dash.contains("formatLocaleTimeHms"));
    assert!(dash.contains("buildAdminOverviewUrl"));
}

#[test]
fn admin_dashboard_quick_stats_wasm_first_ph_s428() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("formatPercent"));
    assert!(dash.contains("formatMegabytes"));
    assert!(dash.contains("renderQuickStats"));
}

#[test]
fn admin_dashboard_quick_stats_glue_tests_ph_s430() {
    let dash = include_str!("dashboard.rs");
    assert!(dash.contains("poolaiUiWasm"));
    assert!(dash.contains("cpu_usage_percent"));
    assert!(dash.contains("memory_usage_mb"));
}

#[tokio::test]
async fn admin_monitoring_ph_s43_ml_metrics_panel() {
    let html = monitoring::admin_monitoring().await.0;
    assert!(html.contains("id=\"ml-demo-btn\""));
    assert!(html.contains("runMlPipelineDemo"));
    assert!(html.contains("poolaiFetchMlPipelines"));
    assert!(html.contains("poolaiFetchMonitoringAlerts"));
    assert!(html.contains("acknowledged: false"));
    assert!(html.contains("poolaiFetchAlertRules"));
    assert!(html.contains("poolaiMonitoringDashboardsUrl"));
    assert!(html.contains("poolaiMonitoringAlertAcknowledgeUrl"));
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
    assert!(js.contains("poolaiUiWasmCall('emptyStateHtml')"));
}

fn assert_admin_common_table_init_wasm_only() {
    let js = include_str!("../admin_common.js");
    assert!(js.contains("poolaiUiWasmCall('tableExportButtonsHtml')"));
    assert!(js.contains("poolaiUiWasmCall('renderTableHtml')"));
    assert!(js.contains("poolaiUiWasmCall('buildTableCsv')"));
    assert!(js.contains("poolaiUiWasmCall('compareSortValues')"));
    assert!(!js.contains("headers.join(',')"));
}

fn assert_admin_common_empty_state_wasm_only() {
    let js = include_str!("../admin_common.js");
    assert!(js.contains("poolaiUiWasmCall('emptyStateHtml')"));
    assert!(!js.contains("admin-empty-state-icon"));
    assert!(!js.contains("admin-empty-state-title"));
}

#[test]
fn admin_common_table_init_wasm_only_ph_s930() {
    assert_admin_common_table_init_wasm_only();
}

#[test]
fn admin_common_empty_state_wasm_only_ph_s931() {
    assert_admin_common_empty_state_wasm_only();
}

#[test]
fn i18n_core_merge_patch_no_duplicate_poolai_t_ph_s932() {
    let i18n = include_str!("../i18n_core.js");
    let common = include_str!("../admin_common.js");
    assert!(i18n.contains("function mergeRustI18nPatch"));
    assert!(i18n.contains("window.poolaiT = function"));
    assert!(!common.contains("function poolaiT("));
}

#[test]
fn admin_common_api_error_wasm_first_ph_s273() {
    let js = include_str!("../admin_common.js");
    assert!(!js.contains("function hintFor503"));
    assert!(!js.contains("err.hint503.raid"));
    assert!(!js.contains("err.hint403"));
    assert!(js.contains("poolaiUiWasmCall('formatFetchError')"));
}

#[test]
fn admin_common_loading_error_wasm_first_ph_s274() {
    let js = include_str!("../admin_common.js");
    assert!(js.contains("poolaiUiWasmCall('adminLoadingHtml')"));
    assert!(js.contains("poolaiUiWasmCall('adminInlineErrorHtml')"));
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
fn admin_payout_batch_wasm_glue_ph_s682() {
    let page = include_str!("payout_batch.rs");
    assert!(page.contains("parsePrometheusGauge"));
    assert!(page.contains("poolaiChartsWasm"));
    assert!(page.contains("/api/v1/grid/settlement-metrics"));
    assert!(page.contains("/api/v1/grid/trust-metrics"));
}

#[test]
fn admin_grid_verification_wasm_glue_ph_s672() {
    let page = include_str!("grid_verification.rs");
    assert!(page.contains("parsePrometheusGauge"));
    assert!(page.contains("poolaiChartsWasm"));
}

#[test]
fn admin_grid_verification_wasm_glue_ph_s712() {
    let page = include_str!("grid_verification.rs");
    assert!(page.contains("/api/v1/grid/verification-metrics"));
    assert!(page.contains("renderGridVerificationMetricsStrip"));
}

#[test]
fn admin_grid_verification_wasm_complete_ph_s882() {
    let page = include_str!("grid_verification.rs");
    assert!(page.contains("poolaiRenderGridVerificationPanel"));
    assert!(page.contains("renderGridVerificationMetricsStrip"));
    assert!(page.contains("loadGridVerificationTasks"));
}

#[test]
fn admin_grid_replication_pricing_wasm_glue_ph_s692() {
    let page = include_str!("grid_replication_pricing.rs");
    assert!(page.contains("parsePrometheusGauge"));
    assert!(page.contains("poolaiChartsWasm"));
    assert!(page.contains("/api/v1/grid/replication-metrics"));
    assert!(page.contains("/api/v1/grid/pricing-metrics"));
}

#[test]
fn admin_grid_replication_pricing_wasm_slim_ph_s892() {
    let page = include_str!("grid_replication_pricing.rs");
    assert!(page.contains("poolaiRenderGridReplicationPricingPanel"));
    assert!(page.contains("replication-metrics"));
}

#[test]
fn admin_grid_replication_pricing_wasm_slim_ph_s700() {
    let page = include_str!("grid_replication_pricing.rs");
    assert!(page.contains("poolaiRenderGridReplicationPricingPanel"));
    assert!(!page.contains("admin-metrics-strip'>"));
}

#[test]
fn admin_charts_ml_pipeline_panel_wasm_only_ph_s800() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMlPipelineMetricsPanel"));
    assert!(!js.contains("pipelineId: p.id"));
    assert!(!js.contains("parts.join('')"));
}

#[test]
fn admin_charts_payout_batch_panel_wasm_only_ph_s801() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderPayoutBatchPanelHtml"));
    assert!(js.contains("poolaiRenderPayoutBatchPanel"));
    assert!(js.contains("wasm.renderGridSettlementTrustMetricsStrip"));
    assert!(!js.contains("admin.payoutBatch.latest"));
}

#[test]
fn admin_payout_batch_parse_prometheus_gauge_ph_s802() {
    let page = include_str!("payout_batch.rs");
    assert!(page.contains("parsePrometheusGauge"));
    assert!(page.contains("galaxy_trust_score"));
    let mon = include_str!("monitoring.rs");
    assert!(mon.contains("poolaiRenderMlPipelineMetricsPanel"));
}

#[test]
fn admin_payout_batch_wasm_slim_no_inline_html_ph_s802() {
    let page = include_str!("payout_batch.rs");
    assert!(page.contains("poolaiRenderPayoutBatchPanel"));
    assert!(!page.contains("admin.payoutBatch.latest', 'Latest cleared entry'"));
}

#[test]
fn admin_charts_secret_rotation_panel_wasm_only_ph_s810() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderSecretRotationPanelHtml"));
    assert!(js.contains("poolaiRenderSecretRotationPanel"));
    assert!(!js.contains("formatRotationKind(kind)"));
}

#[test]
fn admin_charts_topology_stats_strip_wasm_only_ph_s811() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderTopologyStatsStripHtml"));
    assert!(js.contains("poolaiRenderTopologyStatsStrip"));
}

#[test]
fn admin_security_topology_wasm_glue_ph_s812() {
    let sec = include_str!("security.rs");
    assert!(sec.contains("poolaiRenderSecretRotationPanel"));
    assert!(!sec.contains("secret-rotation-table"));
    let topo = include_str!("topology.rs");
    assert!(topo.contains("poolaiRenderTopologyStatsStrip"));
    assert!(topo.contains("topology-stats-strip"));
}

#[test]
fn admin_charts_vm_panel_wasm_only_ph_s820() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderVmPanel"));
    assert!(js.contains("poolaiRenderVmPanel"));
    assert!(!js.contains("admin.vmadm.empty', 'No VM instances found'),\n    { icon: '🖥' }"));
}

#[test]
fn admin_charts_workers_libs_panel_wasm_only_ph_s821() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderWorkersPanel"));
    assert!(js.contains("wasm.renderLibsPanel"));
    assert!(js.contains("poolaiRenderLibsPanel"));
    assert!(!js.contains("admin.wrk.empty', 'No workers found'),\n    { icon: '👷' }"));
    let libs = include_str!("libs.rs");
    assert!(libs.contains("poolaiRenderLibsPanel"));
    assert!(!libs.contains("admin-table"));
}

#[test]
fn admin_vm_workers_libs_wasm_glue_ph_s822() {
    let vm = include_str!("vm.rs");
    assert!(vm.contains("poolaiRenderVmPanel"));
    assert!(!vm.contains("<table class=\"admin-table\">"));
    let wrk = include_str!("workers.rs");
    assert!(wrk.contains("poolaiRenderWorkersPanel"));
    assert!(!wrk.contains("<table class=\"admin-table\">"));
    let libs = include_str!("libs.rs");
    assert!(libs.contains("poolaiRenderLibsPanel"));
    assert!(libs.contains("data-lib-action"));
    assert!(!libs.contains("<table class=\"admin-table\">"));
}

#[test]
fn admin_charts_grid_replication_pricing_wasm_first_ph_s701() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderGridReplicationPricingPanel"));
    assert!(js.contains("poolaiRenderGridReplicationPricingPanel"));
    assert!(!js.contains("parts.join('')"));
    assert!(!js.contains("pipelineId: p.id"));
}

#[test]
fn admin_grid_pricing_freshness_wasm_first_ph_s902() {
    let gp = include_str!("grid_pricing.rs");
    assert!(gp.contains("renderGridPricingFreshnessStrip"));
    assert!(!gp.contains("cache_fresh_until_secs"));
}

#[test]
fn admin_payout_batch_trust_persist_wasm_first_ph_s912() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("renderGridTrustPersistStrip"));
    let pb = include_str!("payout_batch.rs");
    assert!(pb.contains("/api/v1/grid/trust-metrics"));
}

#[test]
fn admin_charts_ml_canvas_glue_wasm_only_ph_s701() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.renderMlPipelineMetricsPanel"));
    assert!(!js.contains("parseFloat(String(val))"));
    assert!(!js.contains("metrics-charts-grid"));
}

#[test]
fn admin_charts_metric_point_values_wasm_only_ph_s701() {
    let js = include_str!("../admin_charts.js");
    assert!(js.contains("wasm.metricPointValues"));
    assert!(!js.contains("d.value != null ? Number(d.value)"));
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
