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

pub mod audit;
pub mod config;
pub mod dashboard;
pub mod instances;
pub mod libs;
pub mod monitoring;
pub mod raid;
pub mod security;
pub mod tenants;
pub mod topology;
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
        .route("/admin/libs", get(libs::admin_libs))
        .route("/admin/raid", get(raid::admin_raid))
        .route("/admin/instances", get(instances::admin_instances))
        .route("/admin/topology", get(topology::admin_topology))
        .route("/admin/users", get(users::admin_users))
        .route("/admin/config", get(config::admin_config))
}

/// Admin panel layout function - shared across all admin pages
pub fn admin_layout(title: &str, body_html: &str, script_js: &str) -> Html<String> {
    let base_css = include_str!("../admin_styles.css");
    let i18n_js = include_str!("../i18n_core.js");
    let common_js = include_str!("../admin_common.js");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{title} - PoolAI Admin</title>
  <style>{base_css}</style>
</head>
<body>
  <div class="admin-wrapper">
    <aside class="admin-sidebar" role="navigation" aria-label="Admin navigation">
      <div class="admin-brand">
        <h1 data-i18n="admin.brand">PoolAI Admin</h1>
        <div class="admin-version">v0.1.0</div>
      </div>
      <nav class="admin-nav">
        <a href="/ui/admin" class="admin-nav-item" data-i18n="admin.nav.dashboard">Dashboard</a>
        <a href="/ui/admin/tenants" class="admin-nav-item" data-i18n="admin.nav.tenants">Tenants</a>
        <a href="/ui/admin/security" class="admin-nav-item" data-i18n="admin.nav.security">Security</a>
        <a href="/ui/admin/audit" class="admin-nav-item" data-i18n="admin.nav.audit">Audit Logs</a>
        <a href="/ui/admin/monitoring" class="admin-nav-item" data-i18n="admin.nav.monitoring">Monitoring</a>
        <a href="/ui/admin/vm" class="admin-nav-item" data-i18n="admin.nav.vm">VM Instances</a>
        <a href="/ui/admin/workers" class="admin-nav-item" data-i18n="admin.nav.workers">Workers</a>
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
        <h2>{title}</h2>
        <div class="admin-user-menu">
          <div id="poolai-lang-toggle" class="admin-lang-bar"></div>
          <span id="admin-user-name">Admin</span>
          <button type="button" class="btn-icon" onclick="logout()" data-i18n-aria="admin.logout" aria-label="Log out">🚪</button>
        </div>
      </header>
      
      <div class="admin-content">
        {body}
      </div>
    </main>
  </div>
  
  <script>{i18n_js}</script>
  <script>{common_js}</script>
  <script>
    (function() {{
      if (typeof PoolAiI18n !== 'undefined') {{
        PoolAiI18n.apply(document.body);
        PoolAiI18n.initAdminShell();
      }}
      if (!requireAdmin()) {{
        return;
      }}
      {script}
    }})();
  </script>
</body>
</html>"#,
        title = title,
        base_css = base_css,
        body = body_html,
        i18n_js = i18n_js,
        common_js = common_js,
        script = script_js
    );

    Html(html)
}
