//! UI module
//!
//! Provides web dashboard interface with read/write operations, authentication,
//! theme customization, and reusable UI components.
//!
//! # Features
//!
//! - **Dashboard Pages**: Status, health, metrics, workers, libs, VM, RAID
//! - **Authentication**: JWT-based authentication with role-based access control
//! - **Write Operations**: Create/delete artifacts and workers through UI
//! - **Theme Support**: Dark, light, and high-contrast themes
//! - **Components**: Reusable UI components (buttons, cards, forms, modals)
//!
//! # Routes
//!
//! - `/ui` - Home page
//! - `/ui/status` - System status
//! - `/ui/health` - Health check
//! - `/ui/metrics` - System metrics
//! - `/ui/workers` - Worker management
//! - `/ui/libs` - Library management
//! - `/ui/vm` - VM instance management
//! - `/ui/raid` - RAID artifact management
//!
//! Concept alignment (planned in `docs/concept/poolAI_concept.txt`):
//! - Web dashboard (basic)
//! - UI components/themes/layouts (planned)

pub mod components;
pub use components::get_component_styles;

mod themes;
pub use themes::{get_all_themes, get_theme, Theme, DARK_THEME, HIGH_CONTRAST_THEME, LIGHT_THEME};

#[cfg(feature = "enterprise")]
mod admin;
#[cfg(feature = "enterprise")]
pub use admin::create_admin_routes;

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use axum::{response::Html, routing::get, Router};
use tracing::info;

pub struct UiManager;

impl Default for UiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UiManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing UI module");
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down UI module");
        Ok(())
    }
}

pub fn create_ui_routes() -> Router<ApiContext> {
    let router = Router::new()
        .route("/", get(home_handler))
        .route("/auth", get(login_page))
        .route("/login", get(login_page))
        .route("/status", get(status_page))
        .route("/health", get(health_page))
        .route("/metrics", get(metrics_page))
        .route("/workers", get(workers_page))
        .route("/libs", get(libs_page))
        .route("/vm", get(vm_page))
        .route("/raid", get(raid_page));

    // Add admin routes if enterprise feature is enabled
    #[cfg(feature = "enterprise")]
    {
        router.merge(create_admin_routes())
    }
    #[cfg(not(feature = "enterprise"))]
    {
        router
    }
}

const BASE_CSS: &str = r#"
  /* Box-sizing для правильного позиціонування */
  *, *::before, *::after {
    box-sizing: border-box;
  }
  
  body { 
    font-family: Segoe UI, Arial, sans-serif; 
    font-size: var(--font-size-base, 16px);
    line-height: var(--line-height-normal, 1.5);
    font-weight: var(--font-weight-normal, 400);
    background: var(--bg, #0f1216); 
    color: var(--text, #e8e8e8); 
    margin: 0; 
    padding: 0;
    transition: background-color var(--transition-slow, 0.3s ease), color var(--transition-slow, 0.3s ease);
  }
  a { color: var(--link, #77c7ff); text-decoration: none; }
  a:hover { color: var(--link-hover, #8bd5ff); text-decoration: underline; }
  code { background: var(--bg, #0f1216); padding: 2px 6px; border-radius: 6px; border: 1px solid var(--border, #262b36); }
  
  /* Wrap контейнер з автоматичним вирівнюванням */
  .wrap { 
    max-width: 1080px; 
    margin: var(--spacing-8, 32px) auto; 
    padding: 0 var(--spacing-4, 16px); 
    width: 100%;
  }
  
  /* Topbar з правильним вирівнюванням */
  .topbar { 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    gap: var(--spacing-4, 16px); 
    padding: var(--spacing-3, 12px) var(--spacing-4, 16px); 
    border: 1px solid var(--border, #262b36); 
    border-radius: var(--radius-lg, 12px); 
    background: var(--surface, #171b22); 
    box-shadow: var(--shadow-xl, 0 12px 40px rgba(0,0,0,.20)); 
    width: 100%;
    flex-wrap: wrap;
  }
  .brand { display: flex; align-items: center; gap: var(--spacing-3, 12px); flex: 0 0 auto; }
  .brand h1 { margin: 0; font-size: var(--font-size-lg, 18px); color: var(--primary, #67e480); font-weight: var(--font-weight-semibold, 600); }
  .brand .muted { color: var(--text-muted, #a8b0bf); font-size: var(--font-size-sm, 14px); }
  
  /* Navigation з автоматичним вирівнюванням */
  .nav { 
    display: flex; 
    flex-wrap: wrap; 
    gap: var(--spacing-2, 8px); 
    align-items: center; 
    flex: 1 1 auto;
    justify-content: flex-end;
  }
  .nav a { 
    padding: var(--spacing-1, 4px) var(--spacing-2, 8px); 
    border: 1px solid var(--border, #262b36); 
    border-radius: var(--radius-md, 8px); 
    background: var(--bg, #0f1216); 
    white-space: nowrap;
    transition: all var(--transition-base, 0.2s ease);
    position: relative;
    font-size: var(--font-size-sm, 14px);
  }
  .nav a:hover {
    background: var(--surface-secondary, #1e2329);
    border-color: var(--primary, #50fa7b);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(80, 250, 123, 0.2);
  }
  .nav a:active {
    transform: translateY(0);
  }
  
  /* Content з правильним spacing */
  .content { 
    margin-top: var(--spacing-3, 12px); 
    width: 100%;
  }
  
  /* Grid з автоматичним вирівнюванням */
  .grid { 
    display: grid; 
    grid-template-columns: 1fr 1fr; 
    gap: var(--spacing-3, 12px); 
    margin-top: var(--spacing-3, 12px); 
    width: 100%;
  }
  .item { 
    padding: var(--spacing-3, 12px); 
    border-radius: var(--radius-lg, 12px); 
    border: 1px solid var(--border, #262b36); 
    background: var(--bg, #0f1216); 
    width: 100%;
    transition: all var(--transition-base, 0.2s ease);
  }
  .item:hover {
    background: var(--surface, #171b22);
    border-color: var(--border, #44475a);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  }
  .muted { color: var(--text-muted, #a8b0bf); font-size: 0.9em; }
  
  /* Row з автоматичним вирівнюванням до кордонів */
  .row { 
    display: flex; 
    align-items: center; 
    justify-content: space-between; 
    gap: var(--spacing-3, 12px); 
    width: 100%;
    flex-wrap: wrap;
  }
  .row > * {
    flex: 0 0 auto;
  }
  .row > *:last-child {
    margin-left: auto;
  }
  
  pre { 
    white-space: pre-wrap; 
    word-break: break-word; 
    background: var(--bg, #0b0d10); 
    border: 1px solid var(--border, #262b36); 
    border-radius: var(--radius-lg, 12px); 
    padding: var(--spacing-3, 12px); 
    margin: var(--spacing-3, 12px) 0 0; 
    width: 100%;
    overflow-x: auto;
    transition: all var(--transition-base, 0.2s ease);
    font-size: var(--font-size-sm, 14px);
    line-height: var(--line-height-normal, 1.5);
  }
  pre:hover {
    border-color: var(--primary, #50fa7b);
    box-shadow: 0 2px 8px rgba(80, 250, 123, 0.1);
  }
  
  @media (max-width: 860px) { 
    .grid { grid-template-columns: 1fr; } 
  }
  
  /* Responsive Design - Tablet (768px - 1024px) */
  @media (max-width: 1024px) {
    .wrap { max-width: 100%; padding: 0 16px; }
    .grid { gap: 10px; }
  }
  
  /* Responsive Design - Mobile Landscape (768px) */
  @media (max-width: 768px) {
    .wrap { padding: 0 12px; margin: 16px auto; }
    .topbar { flex-direction: column; align-items: flex-start; gap: 12px; padding: 12px; }
    .nav { display: none; } /* Hide desktop nav, show mobile drawer */
    .row { flex-direction: column; align-items: flex-start; gap: 8px; }
    .card { padding: 12px; border-radius: 10px; }
    table { font-size: 0.85em; }
    th, td { padding: 8px 6px; }
    .item { padding: 10px; }
    pre { padding: 10px; font-size: 0.85em; }
    h2 { font-size: 1.2em; }
    .muted { font-size: 0.85em; }
  }
  
  /* Responsive Design - Mobile Portrait (480px) */
  @media (max-width: 480px) {
    .wrap { padding: 0 8px; margin: 12px auto; }
    .topbar { padding: 10px; border-radius: 10px; }
    .brand h1 { font-size: 16px; }
    .brand .muted { font-size: 0.8em; display: none; } /* Hide subtitle on small screens */
    .card { padding: 10px; border-radius: 8px; }
    .btn { padding: 12px 16px; font-size: 0.9em; min-height: 44px; } /* Touch-friendly buttons */
    .item { padding: 8px; border-radius: 8px; }
    .grid { gap: 8px; margin-top: 8px; }
    pre { padding: 8px; font-size: 0.8em; border-radius: 8px; }
    h2 { font-size: 1.1em; margin-bottom: 4px; }
    .pill { font-size: 0.8em; padding: 2px 6px; }
    table { font-size: 0.8em; }
    th, td { padding: 6px 4px; }
  }
  
  /* Responsive Design - Small Mobile (360px) */
  @media (max-width: 360px) {
    .wrap { padding: 0 6px; margin: 8px auto; }
    .topbar { padding: 8px; }
    .brand h1 { font-size: 14px; }
    .card { padding: 8px; }
    .btn { padding: 10px 12px; font-size: 0.85em; }
    pre { font-size: 0.75em; }
    th, td { padding: 4px 3px; font-size: 0.75em; }
  }
  
  /* Touch Device Optimizations */
  @media (hover: none) and (pointer: coarse) {
    /* Increase touch targets */
    .btn, .nav a, button, a, select, input[type="checkbox"], input[type="radio"] {
      min-height: 44px;
      min-width: 44px;
    }
    .nav a { padding: 12px 14px; }
    .btn { padding: 12px 20px; }
    select { padding: 10px 12px; }
    input[type="text"], input[type="password"], input[type="email"], textarea {
      padding: 12px;
      font-size: 16px; /* Prevent zoom on iOS */
    }
    /* Disable hover effects on touch devices */
    .btn:hover, .nav a:hover, .card:hover, .item:hover {
      transform: none;
    }
    /* Add active states for touch feedback */
    .btn:active, .nav a:active {
      opacity: 0.8;
      transform: scale(0.98);
    }
    /* Larger tap targets for table actions */
    .action-buttons .btn { min-height: 40px; padding: 10px 14px; }
    /* Better spacing for touch */
    .form-group { margin-bottom: 20px; }
    .modal-content { padding: 20px; }
    .dropdown-item { padding: 14px 16px; min-height: 44px; }
    .tab { padding: 14px 20px; min-height: 44px; }
    .accordion-header { padding: 16px; min-height: 48px; }
  }
  
  /* Landscape orientation adjustments */
  @media (orientation: landscape) and (max-height: 500px) {
    .wrap { margin: 8px auto; }
    .topbar { padding: 8px 12px; }
    .card { padding: 10px; }
    .modal-content { max-height: 85vh; }
    .mobile-nav-drawer { width: 50%; }
  }
  
  /* High DPI / Retina displays */
  @media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
    .card, .topbar, .item { box-shadow: 0 8px 24px rgba(0,0,0,.15); }
  }
  
  /* Print styles */
  @media print {
    .topbar, .nav, .mobile-menu-toggle, .mobile-nav-drawer, .mobile-nav-overlay, .btn, .action-buttons {
      display: none !important;
    }
    .wrap { max-width: 100%; margin: 0; padding: 0; }
    .card { box-shadow: none; border: 1px solid #ccc; page-break-inside: avoid; }
    body { background: white; color: black; }
    a { color: black; text-decoration: underline; }
    pre { white-space: pre-wrap; word-break: break-word; border: 1px solid #ccc; }
  }
  /* Accessibility: Skip links */
  .skip_link {
    position: absolute;
    top: -40px;
    left: 0;
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    padding: 8px 16px;
    text-decoration: none;
    border-radius: 4px;
    z-index: 10000;
    font-weight: bold;
  }
  .skip_link:focus {
    top: 0;
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  /* Accessibility: Focus indicators */
  a:focus, button:focus, input:focus, select:focus, textarea:focus, [tabindex]:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  a:focus-visible, button:focus-visible, input:focus-visible, select:focus-visible, textarea:focus-visible, [tabindex]:focus-visible {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  /* Screen reader only content */
  .sr-only {
    position: absolute;
    left: -10000px;
    width: 1px;
    height: 1px;
    overflow: hidden;
  }
  /* Enhanced keyboard navigation for interactive elements */
  .search-result-item:focus,
  [role="option"]:focus,
  [role="button"]:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
    background: var(--surface-secondary, #1e2329);
  }
  /* Skip link positioning fix */
  .skip_link:focus {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 10000;
  }
  
  /* Enhanced UI Improvements - Smooth Transitions & Visual Polish */
  .card {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }
  .card::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(119, 199, 255, 0.1), transparent);
    transition: left 0.5s ease;
  }
  .card:hover::before {
    left: 100%;
  }
  .card:hover {
    transform: translateY(-2px);
    box-shadow: 0 16px 48px rgba(0,0,0,0.25);
    border-color: var(--primary, #50fa7b);
  }
  
  /* Smooth fade-in for content */
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
  @keyframes slideInUp {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
  }
  .content > .card {
    animation: fadeIn 0.4s ease-out;
  }
  .content > .card:nth-child(1) { animation-delay: 0.1s; }
  .content > .card:nth-child(2) { animation-delay: 0.2s; }
  .content > .card:nth-child(3) { animation-delay: 0.3s; }
  .content > .card:nth-child(4) { animation-delay: 0.4s; }
  
  /* Enhanced item hover effects */
  .item {
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }
  .item:hover {
    background: var(--surface, #171b22);
    border-color: var(--primary, #50fa7b);
    transform: translateY(-2px) scale(1.01);
    box-shadow: 0 8px 24px rgba(80, 250, 123, 0.15);
  }
  
  /* Enhanced button interactions */
  .btn {
    position: relative;
    overflow: hidden;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }
  .btn::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    transform: translate(-50%, -50%);
    transition: width 0.4s ease, height 0.4s ease;
  }
  .btn:active::after {
    width: 300px;
    height: 300px;
  }
  .btn:active {
    transform: scale(0.98);
  }
  
  /* Enhanced topbar with subtle animation */
  .topbar {
    animation: slideInUp 0.3s ease-out;
  }
  
  /* Loading state improvements */
  .loading {
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  /* Enhanced navigation link active state */
  .nav a.active {
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    font-weight: 600;
    box-shadow: 0 4px 12px rgba(80, 250, 123, 0.3);
  }
  
  /* Smooth theme transitions */
  * {
    transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
  }
  
  /* Enhanced button transitions */
  .btn {
    position: relative;
    overflow: hidden;
  }
  .btn::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.2);
    transform: translate(-50%, -50%);
    transition: width 0.6s, height 0.6s;
  }
  .btn:hover::before {
    width: 300px;
    height: 300px;
  }
  .btn:active {
    transform: scale(0.98);
  }
  
  /* Enhanced table row transitions */
  tr {
    transition: all 0.2s ease;
  }
  tr:hover {
    background: var(--surface-secondary, #1e2329) !important;
    transform: scale(1.01);
  }
  
  /* Smooth loading state transitions */
  [data-loading="true"] {
    opacity: 0.6;
    pointer-events: none;
    position: relative;
  }
  [data-loading="true"]::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(90deg, transparent, rgba(80, 250, 123, 0.1), transparent);
    animation: loading-shimmer 1.5s infinite;
  }
  @keyframes loading-shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
  
  /* Enhanced topbar transitions */
  .topbar {
    transition: all 0.3s ease;
  }
  
  /* Smooth theme transitions */
  * {
    transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
  }
  
  /* Enhanced focus states */
  input:focus, select:focus, textarea:focus {
    transform: scale(1.02);
    box-shadow: 0 0 0 3px rgba(80, 250, 123, 0.2);
  }
  
  /* Modal overlay */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: none;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }
  .modal-overlay.active {
    display: flex;
  }
  
  /* Smooth modal transitions */
  .modal {
    position: relative;
    animation: fadeIn 0.2s ease-out;
  }
  .modal-content {
    animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  @keyframes slideUp {
    from { transform: translateY(20px); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
  }
  
  /* Enhanced pill badge transitions */
  .pill {
    transition: all 0.2s ease;
  }
  .pill:hover {
    transform: scale(1.05);
  }
  
  /* Smooth grid item transitions */
  .grid .item {
    animation: fadeIn 0.3s ease-out;
    animation-fill-mode: both;
  }
  .grid .item:nth-child(1) { animation-delay: 0.05s; }
  .grid .item:nth-child(2) { animation-delay: 0.1s; }
  .grid .item:nth-child(3) { animation-delay: 0.15s; }
  .grid .item:nth-child(4) { animation-delay: 0.2s; }

  /* FM-012: language toggle (login + shared with admin .btn-lang) */
  .poolai-lang-auth {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-left: auto;
  }
  .btn-lang {
    padding: 4px 10px;
    font-size: var(--font-size-sm, 14px);
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    background: var(--surface-secondary, #1e2329);
    color: var(--text, #e8e8e8);
    cursor: pointer;
  }
  .btn-lang:hover {
    border-color: var(--primary, #67e480);
    color: var(--primary, #67e480);
  }
  .btn-lang.active {
    background: var(--primary, #67e480);
    color: #0f1216;
    border-color: var(--primary, #67e480);
  }
"#;

fn layout(
    title_i18n_key: &str,
    title_fallback: &str,
    body_html: &str,
    script_js: &str,
) -> Html<String> {
    let auth_url = "/ui/auth";
    let nav_auth_link = format!(
        r#"<a href="{}" id="authLoginBtn" data-i18n="dash.login">Login</a>"#,
        auth_url
    );
    let user_info_html = r##"<div class="user-info" id="userInfo" style="display:none;">
          <span class="role" id="userRole"></span>
          <a href="#" id="logoutBtn" data-i18n="dash.logout">Logout</a>
        </div>"##;
    let component_styles = get_component_styles();
    let theme = DARK_THEME; // Default theme
    let theme_css = theme.to_css();
    let high_contrast_value = "high-contrast";

    let nav_id = "navigation";
    let main_content_id = "main_content";
    let skip_link_class = "skip_link";
    let skip_to_main_href = format!("#{}", main_content_id);
    let skip_to_nav_href = format!("#{}", nav_id);
    let ui_base = "/ui";
    let ui_status = "/ui/status";
    let ui_health = "/ui/health";
    let ui_metrics = "/ui/metrics";
    let ui_workers = "/ui/workers";
    let ui_libs = "/ui/libs";
    let ui_vm = "/ui/vm";
    let ui_raid = "/ui/raid";
    let style_select = "padding: 4px 8px; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); color: var(--text); font-size: 0.9em; cursor: pointer;";

    let i18n_js = include_str!("i18n_core.js");
    let i18n_boot = r#"
(function(){
  if (typeof PoolAiI18n !== 'undefined') {
    document.documentElement.lang = PoolAiI18n.getLang() === 'uk' ? 'uk' : 'en';
    PoolAiI18n.apply(document.documentElement);
    PoolAiI18n.initDashboardShell();
  }
  document.addEventListener('poolai:langchange', function(){
    if (typeof PoolAiI18n !== 'undefined') PoolAiI18n.apply(document.documentElement);
  });
})();"#;
    let full_script = format!("{}\n{}\n{}", i18n_js, script_js, i18n_boot);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title data-i18n="{title_key}">{title_fallback}</title>
  <style>{base_css}
{component_css}
{theme_css}</style>
</head>
<body>
  <a href="{skip_to_main_href}" class="{skip_link_class}" data-i18n="dash.skipMain">Skip to main content</a>
  <a href="{skip_to_nav_href}" class="{skip_link_class}" data-i18n="dash.skipNav">Skip to navigation</a>
  
  <div class="wrap">
    <header class="topbar" role="banner">
      <div class="brand">
        <div>
          <h1 data-i18n="dash.brand">PoolAI UI</h1>
          <div class="muted" data-i18n="dash.subtitle">Dashboard with write operations (Stage 3)</div>
        </div>
      </div>
      <div id="poolai-lang-toggle-dash" class="poolai-lang-auth"></div>
      <button class="mobile-menu-toggle" id="mobileMenuToggle" data-i18n-aria="dash.aria.openMenu" aria-expanded="false">
        ☰
      </button>
      <nav class="nav" id="{nav_id}" role="navigation" data-i18n-aria="dash.aria.mainNav">
        <a href="{ui_base}" data-i18n="dash.nav.home" data-i18n-aria="dash.aria.home">Home</a>
        <a href="{ui_status}" data-i18n="dash.nav.status" data-i18n-aria="dash.aria.status">Status</a>
        <a href="{ui_health}" data-i18n="dash.nav.health" data-i18n-aria="dash.aria.health">Health</a>
        <a href="{ui_metrics}" data-i18n="dash.nav.metrics" data-i18n-aria="dash.aria.metrics">Metrics</a>
        <a href="{ui_workers}" data-i18n="dash.nav.workers" data-i18n-aria="dash.aria.workers">Workers</a>
        <a href="{ui_libs}" data-i18n="dash.nav.libs" data-i18n-aria="dash.aria.libs">Libs</a>
        <a href="{ui_vm}" data-i18n="dash.nav.vm" data-i18n-aria="dash.aria.vm">VM</a>
        <a href="{ui_raid}" data-i18n="dash.nav.raid" data-i18n-aria="dash.aria.raid">RAID</a>
        <select id="themeSelector" data-i18n-aria="dash.aria.theme" style="{style_select}">
          <option value="dark" data-i18n="dash.themeOptDark">🌙 Dark</option>
          <option value="light" data-i18n="dash.themeOptLight">☀️ Light</option>
          <option value="{high_contrast_value}" data-i18n="dash.themeOptHC">🔆 High Contrast</option>
        </select>
        {user_info_html}
        {nav_auth_link}
      </nav>
    </header>

    <main class="content" id="{main_content_id}" role="main">
      <div class="card">
        <div class="row">
          <div>
            <h2 style="margin:0 0 6px" data-i18n="{title_key}">{title_fallback}</h2>
            <div class="muted" data-i18n="dash.pageAutoRefresh">Auto-refresh is enabled (5s). Write operations are available for authenticated users with appropriate permissions.</div>
          </div>
          <div class="pill" id="last_updated" aria-live="polite" aria-atomic="true">—</div>
        </div>
        {body}
      </div>
    </main>
  </div>

  <div class="mobile-nav-overlay" id="mobileNavOverlay"></div>
  <div class="mobile-nav-drawer" id="mobileNavDrawer" role="navigation" data-i18n-aria="dash.aria.mobileNav">
    <div class="mobile-nav-header">
      <h2 style="margin: 0; color: var(--primary, #67e480);" data-i18n="dash.menuTitle">Menu</h2>
      <button class="mobile-nav-close" id="mobileNavClose" data-i18n-aria="dash.aria.closeMenu">×</button>
    </div>
    <div class="mobile-nav-content">
      <a href="{ui_base}" class="mobile-nav-item" data-i18n="dash.nav.home" data-i18n-aria="dash.aria.home">Home</a>
      <a href="{ui_status}" class="mobile-nav-item" data-i18n="dash.nav.status" data-i18n-aria="dash.aria.status">Status</a>
      <a href="{ui_health}" class="mobile-nav-item" data-i18n="dash.nav.health" data-i18n-aria="dash.aria.health">Health</a>
      <a href="{ui_metrics}" class="mobile-nav-item" data-i18n="dash.nav.metrics" data-i18n-aria="dash.aria.metrics">Metrics</a>
      <a href="{ui_workers}" class="mobile-nav-item" data-i18n="dash.nav.workers" data-i18n-aria="dash.aria.workers">Workers</a>
      <a href="{ui_libs}" class="mobile-nav-item" data-i18n="dash.nav.libs" data-i18n-aria="dash.aria.libs">Libs</a>
      <a href="{ui_vm}" class="mobile-nav-item" data-i18n="dash.nav.vm" data-i18n-aria="dash.aria.vm">VM</a>
      <a href="{ui_raid}" class="mobile-nav-item" data-i18n="dash.nav.raid" data-i18n-aria="dash.aria.raid">RAID</a>
      <div class="mobile-nav-item" style="flex-direction: column; align-items: flex-start; gap: 8px;">
        <label for="mobileThemeSelector" style="font-size: 0.9em; color: var(--text-muted, #a8b0bf);" data-i18n="dash.themeLabel">Theme:</label>
        <select id="mobileThemeSelector" data-i18n-aria="dash.aria.theme" style="width: 100%; padding: 8px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg); color: var(--text); font-size: 0.9em;">
          <option value="dark" data-i18n="dash.themeOptDark">🌙 Dark</option>
          <option value="light" data-i18n="dash.themeOptLight">☀️ Light</option>
          <option value="{high_contrast_value}" data-i18n="dash.themeOptHC">🔆 High Contrast</option>
        </select>
      </div>
      {user_info_html}
      {nav_auth_link}
    </div>
  </div>

  <div id="aria_live_region" aria-live="polite" aria-atomic="true" style="position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;"></div>

  <script>
  {script}
  </script>
</body>
</html>"#,
        title_key = title_i18n_key,
        title_fallback = title_fallback,
        base_css = BASE_CSS,
        component_css = component_styles,
        body = body_html,
        script = full_script,
        nav_auth_link = nav_auth_link,
        user_info_html = user_info_html,
        skip_to_main_href = skip_to_main_href,
        skip_link_class = skip_link_class,
        skip_to_nav_href = skip_to_nav_href,
        nav_id = nav_id,
        ui_base = ui_base,
        ui_status = ui_status,
        ui_health = ui_health,
        ui_metrics = ui_metrics,
        ui_workers = ui_workers,
        ui_libs = ui_libs,
        ui_vm = ui_vm,
        ui_raid = ui_raid,
        style_select = style_select,
        high_contrast_value = high_contrast_value,
        main_content_id = main_content_id
    );

    Html(html)
}

fn common_js() -> &'static str {
    r#"
// Token management
function getToken() {
  return localStorage.getItem('poolai_token');
}

function setToken(token) {
  localStorage.setItem('poolai_token', token);
}

function removeToken() {
  localStorage.removeItem('poolai_token');
  localStorage.removeItem('poolai_user');
  localStorage.removeItem('poolai_role');
  localStorage.removeItem('poolai_token_exp');
}

function getUser() {
  const user = localStorage.getItem('poolai_user');
  const role = localStorage.getItem('poolai_role');
  return user ? { username: user, role: role || 'Viewer' } : null;
}

function setUser(username, role) {
  localStorage.setItem('poolai_user', username);
  localStorage.setItem('poolai_role', role);
}

function updateUI() {
  const user = getUser();
  const userInfo = document.getElementById('userInfo');
  const loginLinkEl = document.getElementById('authLoginBtn');
  const userRole = document.getElementById('userRole');
  
  if (user) {
    if (userInfo) {
      userInfo.style.display = 'flex';
      if (userRole) userRole.textContent = user.role;
    }
    if (loginLinkEl) loginLinkEl.style.display = 'none';
  } else {
    if (userInfo) userInfo.style.display = 'none';
    if (loginLinkEl) loginLinkEl.style.display = 'inline';
  }
}

function getAuthHeaders() {
  const token = getToken();
  const headers = { 'accept': 'application/json', 'content-type': 'application/json' };
  if (token) {
    headers['authorization'] = 'Bearer ' + token;
  }
  return headers;
}

// Token validation and refresh
async function validateToken() {
  const token = getToken();
  if (!token) return false;
  
  try {
    // Decode token to check expiration (simple base64 decode for dev tokens)
    const parts = token.split('.');
    if (parts.length === 3) {
      // Real JWT format
      const payload = JSON.parse(atob(parts[1]));
      const now = Math.floor(Date.now() / 1000);
      if (payload.exp && payload.exp < now) {
        // Token expired, try to refresh
        return await refreshToken();
      }
      return true;
    } else if (token.startsWith('dev_token_')) {
      // Dev token format - check expiration from localStorage
      const tokenData = localStorage.getItem('poolai_token_exp');
      if (tokenData) {
        const exp = parseInt(tokenData, 10);
        const now = Math.floor(Date.now() / 1000);
        if (exp && exp < now) {
          return await refreshToken();
        }
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token validation error:', e);
    return false;
  }
}

async function refreshToken() {
  try {
    const token = getToken();
    if (!token) return false;
    
    // Try to refresh token via API (if endpoint exists)
    const res = await fetch('/api/v1/refresh', {
      method: 'POST',
      headers: getAuthHeaders(),
    });
    
    if (res.ok) {
      const data = await res.json();
      setToken(data.token);
      if (data.role) {
        const user = getUser();
        if (user) setUser(user.username, data.role);
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token refresh error:', e);
    return false;
  }
}

// Protected route check
async function requireAuth(requiredRole = null) {
  const user = getUser();
  if (!user) {
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  const isValid = await validateToken();
  if (!isValid) {
    removeToken();
    updateUI();
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  if (requiredRole) {
    const roleHierarchy = { 'Viewer': 1, 'Operator': 2, 'Admin': 3 };
    const userLevel = roleHierarchy[user.role] || 0;
    const requiredLevel = roleHierarchy[requiredRole] || 0;
    
    if (userLevel < requiredLevel) {
      alert('Insufficient permissions. Required role: ' + requiredRole);
      window.location.href = '/ui';
      return false;
    }
  }
  
  return true;
}

// Enhanced notification system with stacking and actions
let notificationStack = [];
let notificationIdCounter = 0;

function showNotification(message, type = 'info', duration = 3000, actions = null) {
  const notificationId = 'notification-' + (notificationIdCounter++);
  const notification = document.createElement('div');
  notification.id = notificationId;
  notification.className = 'notification notification-' + type;
  notification.setAttribute('role', 'alert');
  notification.setAttribute('aria-live', type === 'error' ? 'assertive' : 'polite');
  notification.setAttribute('aria-atomic', 'true');
  
  const notificationContent = document.createElement('div');
  notificationContent.style.cssText = 'display: flex; align-items: center; justify-content: space-between; gap: 12px;';
  
  const messageDiv = document.createElement('div');
  messageDiv.textContent = message;
  messageDiv.style.flex = '1';
  notificationContent.appendChild(messageDiv);
  
  if (actions && actions.length > 0) {
    const actionsDiv = document.createElement('div');
    actionsDiv.style.cssText = 'display: flex; gap: 8px;';
    actions.forEach(action => {
      const btn = document.createElement('button');
      btn.textContent = action.label;
      btn.style.cssText = 'padding: 4px 8px; border: none; background: rgba(255,255,255,0.2); color: inherit; border-radius: 4px; cursor: pointer; font-size: 0.85em;';
      btn.onclick = () => {
        if (action.onClick) action.onClick();
        removeNotification(notificationId);
      };
      actionsDiv.appendChild(btn);
    });
    notificationContent.appendChild(actionsDiv);
  }
  
  const closeBtn = document.createElement('button');
  closeBtn.innerHTML = '&times;';
  closeBtn.style.cssText = 'background: none; border: none; color: inherit; font-size: 20px; cursor: pointer; padding: 0; width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;';
  closeBtn.onclick = () => removeNotification(notificationId);
  closeBtn.setAttribute('aria-label', 'Close notification');
  notificationContent.appendChild(closeBtn);
  
  notification.appendChild(notificationContent);
  
  // Position notification
  const top = 20 + (notificationStack.length * 70);
  notification.style.cssText = `
    position: fixed; top: ${top}px; right: 20px; z-index: ${10000 + notificationStack.length};
    padding: 12px 20px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    background: ${type === 'success' ? '#50fa7b' : type === 'error' ? '#ff5555' : type === 'warning' ? '#f1fa8c' : '#8be9fd'};
    color: ${type === 'error' ? '#fff' : '#0f1216'};
    font-weight: 500; max-width: 400px; word-wrap: break-word;
    animation: slideIn 0.3s ease-out;
  `;
  
  document.body.appendChild(notification);
  notificationStack.push({ id: notificationId, element: notification });
  
  // Announce to screen readers
  const liveRegion = document.getElementById('aria_live_region');
  if (liveRegion) {
    liveRegion.textContent = message;
    setTimeout(() => {
      liveRegion.textContent = '';
    }, duration + 300);
  }
  
  if (duration > 0) {
    setTimeout(() => {
      removeNotification(notificationId);
    }, duration);
  }
  
  return notificationId;
}

function removeNotification(notificationId) {
  const index = notificationStack.findIndex(n => n.id === notificationId);
  if (index === -1) return;
  
  const notification = notificationStack[index].element;
  notification.style.animation = 'slideOut 0.3s ease-out';
  setTimeout(() => {
    notification.remove();
    notificationStack.splice(index, 1);
    // Reposition remaining notifications
    notificationStack.forEach((n, i) => {
      n.element.style.top = (20 + (i * 70)) + 'px';
      n.element.style.zIndex = (10000 + i).toString();
    });
  }, 300);
}

// Enhanced loading functions with skeleton support and accessibility
function showLoading(elementId, message = 'Loading...', useSkeleton = false, progress = null) {
  const el = document.getElementById(elementId);
  if (!el) return;
  el.dataset.loading = 'true';
  
  if (useSkeleton) {
    el.innerHTML = createSkeletonLoader(message);
  } else {
    const loadingId = 'loading-' + elementId + '-' + Date.now();
    let progressHtml = '';
    if (progress !== null) {
      progressHtml = `
        <div class="progress-bar" style="margin-top: 12px; max-width: 300px; margin-left: auto; margin-right: auto;">
          <div class="progress-bar-fill" style="width: ${Math.min(100, Math.max(0, progress))}%" aria-valuenow="${progress}" aria-valuemin="0" aria-valuemax="100" role="progressbar" aria-label="${message}: ${progress}%"></div>
        </div>
      `;
    }
    el.innerHTML = `
      <div role="status" aria-live="polite" aria-busy="true" id="${loadingId}" style="text-align:center; padding:20px; color:var(--text-muted, #a8b0bf);">
        <div class="spinner" aria-hidden="true"></div>
        <div style="margin-top:12px;">${escapeHtml(message)}</div>
        ${progressHtml}
      </div>
    `;
    el.setAttribute('aria-label', message);
  }
}

function hideLoading(elementId) {
  const el = document.getElementById(elementId);
  if (el && el.dataset.loading === 'true') {
    el.dataset.loading = 'false';
    el.removeAttribute('aria-label');
    el.removeAttribute('aria-busy');
  }
}

function updateLoadingProgress(elementId, progress, message = null) {
  const el = document.getElementById(elementId);
  if (!el || el.dataset.loading !== 'true') return;
  
  const progressBar = el.querySelector('.progress-bar-fill');
  const messageDiv = el.querySelector('div[style*="margin-top:12px"]');
  
  if (progressBar) {
    progressBar.style.width = `${Math.min(100, Math.max(0, progress))}%`;
    progressBar.setAttribute('aria-valuenow', progress);
    progressBar.setAttribute('aria-label', (message || 'Loading') + ': ' + progress + '%');
  }
  
  if (messageDiv && message) {
    messageDiv.textContent = message;
  }
  
  // Announce to screen readers
  if (el.querySelector('[role="status"]')) {
    const statusEl = el.querySelector('[role="status"]');
    statusEl.setAttribute('aria-label', (message || 'Loading') + ': ' + progress + '%');
  }
}

function createSkeletonLoader(type = 'table') {
  if (type === 'table') {
    return `
      <div class="skeleton-card">
        <div class="skeleton skeleton-title"></div>
        <div class="skeleton skeleton-text"></div>
        <div class="skeleton skeleton-text" style="width: 80%;"></div>
        <div style="margin-top: 16px;">
          ${Array(5).fill('<div class="skeleton skeleton-table-row"></div>').join('')}
        </div>
      </div>
    `;
  } else if (type === 'card') {
    return `
      <div class="skeleton-card">
        <div class="skeleton skeleton-title"></div>
        <div class="skeleton skeleton-text"></div>
        <div class="skeleton skeleton-text" style="width: 90%;"></div>
        <div class="skeleton skeleton-text" style="width: 70%;"></div>
      </div>
    `;
  } else if (type === 'list') {
    return `
      <div>
        ${Array(3).fill(`
          <div class="skeleton-card" style="margin-bottom: 12px;">
            <div style="display: flex; align-items: center; gap: 12px;">
              <div class="skeleton skeleton-avatar"></div>
              <div style="flex: 1;">
                <div class="skeleton skeleton-text"></div>
                <div class="skeleton skeleton-text" style="width: 60%; margin-top: 8px;"></div>
              </div>
            </div>
          </div>
        `).join('')}
      </div>
    `;
  }
  return `<div class="skeleton skeleton-text"></div>`;
}

function showSpinner(containerId, message = 'Loading...', size = 'medium') {
  const container = document.getElementById(containerId);
  if (!container) return;
  
  const sizeClass = size === 'small' ? 'spinner-small' : size === 'large' ? 'spinner-large' : '';
  container.innerHTML = `
    <div style="display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 20px;">
      <div class="spinner ${sizeClass}"></div>
      ${message ? `<div class="loading-text">${message}</div>` : ''}
    </div>
  `;
}

function showLoadingOverlay(message = 'Loading...') {
  const overlay = document.createElement('div');
  overlay.id = 'loadingOverlay';
  overlay.className = 'loading-overlay';
  overlay.innerHTML = `
    <div class="loading-spinner-container">
      <div class="spinner"></div>
      <div class="loading-text">${message}</div>
    </div>
  `;
  document.body.appendChild(overlay);
}

function hideLoadingOverlay() {
  const overlay = document.getElementById('loadingOverlay');
  if (overlay) {
    overlay.remove();
  }
}

// Enhanced error handling functions with retry support and accessibility
function showErrorBoundary(containerId, error, retryFn = null, suggestions = null) {
  const container = document.getElementById(containerId);
  if (!container) return;
  
  const errorId = 'error-' + containerId + '-' + Date.now();
  const errorMessage = error.message || String(error);
  const errorDetails = error.details || error.stack || null;
  const showDetails = errorDetails && errorDetails.length < 500; // Only show if not too long
  
  let suggestionsHtml = '';
  if (suggestions && Array.isArray(suggestions) && suggestions.length > 0) {
    suggestionsHtml = `
      <div class="error-suggestions" style="margin-top: 12px; padding-top: 12px; border-top: 1px solid var(--border, #262b36);">
        <div style="font-weight: 600; margin-bottom: 8px; color: var(--text-muted, #a8b0bf); font-size: 0.9em;">Suggestions:</div>
        <ul style="margin: 0; padding-left: 20px; color: var(--text-muted, #a8b0bf); font-size: 0.9em;">
          ${suggestions.map(s => `<li>${escapeHtml(s)}</li>`).join('')}
        </ul>
      </div>
    `;
  }
  
  container.innerHTML = `
    <div class="error-boundary" role="alert" aria-live="assertive" id="${errorId}">
      <div class="error-boundary-title">⚠️ Error</div>
      <div class="error-boundary-message">${escapeHtml(errorMessage)}</div>
      ${showDetails ? `
        <details style="margin-top: 12px;">
          <summary style="cursor: pointer; color: var(--text-muted, #a8b0bf); font-size: 0.85em;">Show details</summary>
          <pre style="margin-top: 8px; padding: 8px; background: var(--bg, #0b0d10); border: 1px solid var(--border, #262b36); border-radius: 6px; font-size: 0.8em; overflow-x: auto; max-height: 200px; overflow-y: auto;">${escapeHtml(errorDetails)}</pre>
        </details>
      ` : ''}
      ${suggestionsHtml}
      ${retryFn ? `
        <div class="error-boundary-actions">
          <button class="error-retry" type="button" onclick="(${retryFn.toString()})()" aria-label="Retry the operation that failed">Retry</button>
        </div>
      ` : ''}
    </div>
  `;
  
  // Announce error to screen readers
  const ariaLiveRegion = document.getElementById('aria_live_region');
  if (ariaLiveRegion) {
    ariaLiveRegion.textContent = 'Error: ' + errorMessage;
    setTimeout(() => {
      ariaLiveRegion.textContent = '';
    }, 1000);
  }
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

/** Read user-facing message from API JSON: legacy flat `error` string or structured `{ error: { code, message } }`. */
function apiErrorMessageFromBody(payload) {
  if (!payload || typeof payload !== 'object') return null;
  const e = payload.error;
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && typeof e.message === 'string') return e.message;
  if (typeof payload.message === 'string') return payload.message;
  return null;
}

// Enhanced fetchJson with retry support and better error handling
async function fetchJsonWithRetry(url, options = {}, maxRetries = 3, retryDelay = 1000) {
  let lastError;
  const errors = [];
  
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      const res = await fetchJson(url, options);
      // Clear any retry notifications on success
      if (attempt > 0) {
        showNotification(`Request succeeded after ${attempt} ${attempt === 1 ? 'retry' : 'retries'}`, 'success', 2000);
      }
      return res;
    } catch (error) {
      lastError = error;
      errors.push(error);
      
      if (attempt < maxRetries) {
        // Exponential backoff with notification
        const delay = retryDelay * Math.pow(2, attempt);
        showNotification(
          `Request failed. Retrying in ${(delay / 1000).toFixed(1)}s... (${attempt + 1}/${maxRetries})`,
          'warning',
          2000
        );
        await new Promise(resolve => setTimeout(resolve, delay));
        continue;
      }
    }
  }
  
  // Create enhanced error with context
  const enhancedError = {
    message: lastError.message || 'Request failed after all retries',
    originalError: lastError,
    attempts: maxRetries + 1,
    url: url,
    suggestions: [
      'Check your internet connection',
      'Verify the server is running',
      'Try refreshing the page',
      'Contact support if the problem persists'
    ]
  };
  
  throw enhancedError;
}

// Search & Filter functions
// Enhanced search filter with debounce, accessibility, and better UX
function initSearchFilter(searchInputId, tableId, filterOptions = {}) {
  const searchInput = document.getElementById(searchInputId);
  const table = document.getElementById(tableId);
  if (!searchInput || !table) return;
  
  const debounceDelay = filterOptions.debounceDelay || 300;
  let debounceTimer = null;
  let originalData = [];
  const tbody = table.querySelector('tbody');
  
  // Store original rows with metadata
  if (tbody) {
    originalData = Array.from(tbody.querySelectorAll('tr:not(.no-results-row)')).map(row => ({
      element: row,
      text: row.textContent.toLowerCase(),
      visible: true
    }));
  }
  
  // Add search icon and clear button if not present
  const container = searchInput.parentElement;
  if (container.style.position !== 'relative') {
    container.style.position = 'relative';
  }
  
  if (!container.querySelector('.search-icon')) {
    const icon = document.createElement('span');
    icon.className = 'search-icon';
    icon.setAttribute('aria-hidden', 'true');
    icon.innerHTML = '🔍';
    icon.style.cssText = 'position: absolute; right: 40px; top: 50%; transform: translateY(-50%); color: var(--text-muted, #a8b0bf); pointer-events: none;';
    container.appendChild(icon);
  }
  
  // Add clear button
  let clearButton = container.querySelector('.search-clear');
  if (!clearButton) {
    clearButton = document.createElement('button');
    clearButton.className = 'search-clear';
    clearButton.type = 'button';
    clearButton.setAttribute('aria-label', 'Clear search');
    clearButton.innerHTML = '×';
    clearButton.style.cssText = 'position: absolute; right: 8px; top: 50%; transform: translateY(-50%); background: none; border: none; color: var(--text-muted, #a8b0bf); cursor: pointer; font-size: 20px; width: 24px; height: 24px; display: none; padding: 0;';
    clearButton.onclick = () => {
      searchInput.value = '';
      searchInput.focus();
      filterTable(table, '', filterOptions);
      updateSearchStatus(table, 0, originalData.length);
      clearButton.style.display = 'none';
    };
    container.appendChild(clearButton);
  }
  
  // Enhanced accessibility
  searchInput.setAttribute('aria-label', filterOptions.ariaLabel || 'Search table');
  searchInput.setAttribute('role', 'searchbox');
  searchInput.setAttribute('aria-controls', tableId);
  
  // Debounced search with status updates
  searchInput.addEventListener('input', function(e) {
    const query = e.target.value.toLowerCase().trim();
    
    // Show/hide clear button
    if (query) {
      clearButton.style.display = 'flex';
      clearButton.style.alignItems = 'center';
      clearButton.style.justifyContent = 'center';
    } else {
      clearButton.style.display = 'none';
    }
    
    // Clear previous timer
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    
    // Show loading indicator while searching
    if (query && filterOptions.showLoading !== false) {
      searchInput.setAttribute('aria-busy', 'true');
    }
    
    // Debounce search
    debounceTimer = setTimeout(() => {
      const visibleCount = filterTable(table, query, filterOptions);
      updateSearchStatus(table, visibleCount, originalData.length, query);
      searchInput.removeAttribute('aria-busy');
      
      // Announce to screen readers
      const statusMsg = query ? `${visibleCount} of ${originalData.length} results found` : 'All results shown';
      searchInput.setAttribute('aria-label', (filterOptions.ariaLabel || 'Search table') + ': ' + statusMsg);
    }, debounceDelay);
  });
  
  // Keyboard shortcuts
  searchInput.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && searchInput.value) {
      searchInput.value = '';
      filterTable(table, '', filterOptions);
      updateSearchStatus(table, 0, originalData.length);
      clearButton.style.display = 'none';
      searchInput.focus();
    } else if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
      e.preventDefault(); // Prevent browser search
      searchInput.focus();
      searchInput.select();
    }
  });
}

function updateSearchStatus(table, visibleCount, totalCount, query = '') {
  // Remove existing status row
  const existingStatus = table.querySelector('.search-status-row');
  if (existingStatus) {
    existingStatus.remove();
  }
  
  // Add status row if there's a query
  if (query) {
    const tbody = table.querySelector('tbody');
    if (tbody) {
      const statusRow = document.createElement('tr');
      statusRow.className = 'search-status-row';
      statusRow.setAttribute('role', 'status');
      statusRow.setAttribute('aria-live', 'polite');
      const statusCell = document.createElement('td');
      statusCell.colSpan = table.querySelectorAll('th').length;
      statusCell.textContent = `${visibleCount} of ${totalCount} results`;
      statusCell.style.cssText = 'text-align: center; padding: 12px; color: var(--text-muted, #a8b0bf); font-size: 0.9em; border-top: 1px solid var(--border, #262b36);';
      statusRow.appendChild(statusCell);
      tbody.appendChild(statusRow);
    }
  }
}

// Enhanced table filtering with better matching and highlighting
function filterTable(table, query, options = {}) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return 0;
  
  const rows = tbody.querySelectorAll('tr:not(.no-results-row):not(.search-status-row)');
  let visibleCount = 0;
  const highlightMatches = options.highlightMatches !== false;
  const matchColumns = options.matchColumns || null; // Array of column indices to match, or null for all
  
  rows.forEach(row => {
    if (matchColumns && Array.isArray(matchColumns)) {
      // Only match specific columns
      const rowText = Array.from(row.cells)
        .filter((cell, index) => matchColumns.includes(index))
        .map(cell => cell.textContent)
        .join(' ')
        .toLowerCase();
      var matches = !query || rowText.includes(query.toLowerCase());
    } else {
      // Match all columns
      const rowText = row.textContent.toLowerCase();
      var matches = !query || rowText.includes(query.toLowerCase());
    }
    
    if (matches) {
      row.style.display = '';
      row.setAttribute('aria-hidden', 'false');
      visibleCount++;
      
      // Highlight matches if enabled and query exists
      if (highlightMatches && query) {
        row.querySelectorAll('td').forEach(cell => {
          const originalText = cell.dataset.originalText || cell.textContent;
          if (!cell.dataset.originalText) {
            cell.dataset.originalText = cell.textContent;
          }
          
          const regex = new RegExp(`(${escapeRegex(query)})`, 'gi');
          const highlighted = originalText.replace(regex, '<mark style="background: var(--primary, #50fa7b); color: var(--bg, #0f1216); padding: 2px 0;">$1</mark>');
          cell.innerHTML = highlighted;
        });
      } else {
        // Restore original text
        row.querySelectorAll('td').forEach(cell => {
          if (cell.dataset.originalText) {
            cell.textContent = cell.dataset.originalText;
            delete cell.dataset.originalText;
          }
        });
      }
    } else {
      row.style.display = 'none';
      row.setAttribute('aria-hidden', 'true');
    }
  });
  
  // Show "no results" message if needed
  let noResultsRow = tbody.querySelector('.no-results-row');
  if (visibleCount === 0 && query) {
    if (!noResultsRow) {
      noResultsRow = document.createElement('tr');
      noResultsRow.className = 'no-results-row';
      noResultsRow.setAttribute('role', 'status');
      noResultsRow.setAttribute('aria-live', 'polite');
      const td = document.createElement('td');
      td.colSpan = table.querySelectorAll('th').length;
      td.textContent = `No results found for "${query}"`;
      td.style.cssText = 'text-align: center; padding: 20px; color: var(--text-muted, #a8b0bf);';
      noResultsRow.appendChild(td);
      tbody.appendChild(noResultsRow);
    }
    noResultsRow.style.display = '';
    noResultsRow.querySelector('td').textContent = `No results found for "${query}"`;
  } else if (noResultsRow) {
    noResultsRow.style.display = 'none';
  }
  
  return visibleCount;
}

function escapeRegex(str) {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function sortTable(table, columnIndex, ascending = true) {
  const tbody = table.querySelector('tbody');
  if (!tbody) return;
  
  const rows = Array.from(tbody.querySelectorAll('tr:not(.no-results-row)'));
  const isNumeric = rows.every(row => {
    const cell = row.cells[columnIndex];
    return cell && !isNaN(parseFloat(cell.textContent));
  });
  
  rows.sort((a, b) => {
    const aCell = a.cells[columnIndex];
    const bCell = b.cells[columnIndex];
    
    if (!aCell || !bCell) return 0;
    
    const aValue = isNumeric ? parseFloat(aCell.textContent) : aCell.textContent.trim();
    const bValue = isNumeric ? parseFloat(bCell.textContent) : bCell.textContent.trim();
    
    if (aValue < bValue) return ascending ? -1 : 1;
    if (aValue > bValue) return ascending ? 1 : -1;
    return 0;
  });
  
  // Remove all rows
  rows.forEach(row => row.remove());
  
  // Re-append sorted rows
  rows.forEach(row => tbody.appendChild(row));
  
  // Update sort indicators
  const headers = table.querySelectorAll('th');
  headers.forEach((header, index) => {
    if (index === columnIndex) {
      header.setAttribute('data-sort', ascending ? 'asc' : 'desc');
    } else {
      header.removeAttribute('data-sort');
    }
  });
}

function initTableSorting(tableId) {
  const table = document.getElementById(tableId);
  if (!table) return;
  
  const headers = table.querySelectorAll('th');
  headers.forEach((header, index) => {
    header.style.cursor = 'pointer';
    header.style.userSelect = 'none';
    header.addEventListener('click', function() {
      const currentSort = header.getAttribute('data-sort');
      const ascending = currentSort !== 'asc';
      sortTable(table, index, ascending);
    });
  });
}

// Modal dialog functions with keyboard navigation
let activeModal = null;
let activeOverlay = null;
let modalFocusableElements = [];
let previousActiveElement = null;

function showModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) {
    console.warn('Modal not found:', modalId);
    return;
  }
  
  // Store previous active element for focus restoration
  previousActiveElement = document.activeElement;
  
  // Create or get overlay
  let overlay = document.getElementById('modal-overlay');
  if (!overlay) {
    overlay = document.createElement('div');
    overlay.id = 'modal-overlay';
    overlay.className = 'modal-overlay';
    document.body.appendChild(overlay);
    
    // Close modal on backdrop click
    overlay.addEventListener('click', function(e) {
      if (e.target === overlay && activeModal) {
        hideModal(activeModal.id);
      }
    });
  }
  
  // Move modal to overlay if not already there
  if (modal.parentElement !== overlay) {
    overlay.appendChild(modal);
  }
  
  // Set ARIA attributes
  modal.setAttribute('aria-hidden', 'false');
  modal.setAttribute('aria-modal', 'true');
  overlay.classList.add('active');
  activeModal = modal;
  activeOverlay = overlay;
  
  // Get all focusable elements in modal
  modalFocusableElements = Array.from(modal.querySelectorAll(
    'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
  ));
  
  // Focus first focusable element after a short delay to ensure modal is visible
  setTimeout(() => {
    if (modalFocusableElements.length > 0) {
      modalFocusableElements[0].focus();
    }
  }, 100);
  
  // Trap focus within modal
  modal.addEventListener('keydown', trapModalFocus);
  
  // Prevent body scroll
  document.body.style.overflow = 'hidden';
}

function hideModal(modalId) {
  const modal = document.getElementById(modalId);
  if (!modal) {
    console.warn('Modal not found:', modalId);
    return;
  }
  
  const overlay = document.getElementById('modal-overlay');
  
  // Remove ARIA attributes
  modal.setAttribute('aria-hidden', 'true');
  modal.setAttribute('aria-modal', 'false');
  
  // Hide overlay
  if (overlay) {
    overlay.classList.remove('active');
  }
  
  // Remove focus trap
  if (activeModal) {
    activeModal.removeEventListener('keydown', trapModalFocus);
  }
  
  // Restore previous focus
  if (previousActiveElement) {
    previousActiveElement.focus();
    previousActiveElement = null;
  }
  
  activeModal = null;
  activeOverlay = null;
  modalFocusableElements = [];
  
  // Restore body scroll
  document.body.style.overflow = '';
}

function trapModalFocus(e) {
  if (!activeModal || e.key !== 'Tab') return;
  
  const focusableElements = Array.from(activeModal.querySelectorAll(
    'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
  ));
  
  if (focusableElements.length === 0) return;
  
  const firstElement = focusableElements[0];
  const lastElement = focusableElements[focusableElements.length - 1];
  
  if (e.shiftKey) {
    // Shift + Tab
    if (document.activeElement === firstElement) {
      e.preventDefault();
      lastElement.focus();
    }
  } else {
    // Tab
    if (document.activeElement === lastElement) {
      e.preventDefault();
      firstElement.focus();
    }
  }
}

// Keyboard shortcuts
document.addEventListener('keydown', function(e) {
  // Esc key closes modals
  if (e.key === 'Escape' && activeModal) {
    hideModal(activeModal.id);
  }
  
  // Ctrl+K or Cmd+K for global search
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    showSearchModal();
  }
});

// Global search functionality
function showSearchModal() {
  const modalId = 'searchModal';
  let modal = document.getElementById(modalId);
  
  if (!modal) {
    modal = document.createElement('div');
    modal.id = modalId;
    modal.className = 'modal';
    modal.setAttribute('role', 'dialog');
    modal.setAttribute('aria-labelledby', 'searchModalTitle');
    modal.setAttribute('aria-modal', 'true');
    modal.setAttribute('aria-hidden', 'true');
    modal.innerHTML = `
      <div class="modal-content" style="max-width: 600px;">
        <div class="modal-header">
          <h3 id="searchModalTitle">Search</h3>
          <button class="modal-close" aria-label="Close search dialog" onclick="hideModal('${modalId}')">&times;</button>
        </div>
        <div class="modal-body">
          <div class="search-container">
            <input type="text" id="globalSearchInput" class="search-input" placeholder="Search pages, workers, artifacts..." autofocus aria-label="Search input" />
            <span class="search-icon">🔍</span>
          </div>
          <div id="searchResults" style="margin-top: 16px; max-height: 400px; overflow-y: auto;"></div>
        </div>
      </div>
    `;
    document.body.appendChild(modal);
    
    const searchInput = document.getElementById('globalSearchInput');
    const resultsContainer = document.getElementById('searchResults');
    
    // Search pages and navigation
    const searchableItems = [
      { name: 'Home', url: '/ui', category: 'Page' },
      { name: 'Status', url: '/ui/status', category: 'Page' },
      { name: 'Health', url: '/ui/health', category: 'Page' },
      { name: 'Metrics', url: '/ui/metrics', category: 'Page' },
      { name: 'Workers', url: '/ui/workers', category: 'Page' },
      { name: 'Libraries', url: '/ui/libs', category: 'Page' },
      { name: 'VM Instances', url: '/ui/vm', category: 'Page' },
      { name: 'RAID', url: '/ui/raid', category: 'Page' },
    ];
    
    let selectedIndex = -1;
    
    searchInput.addEventListener('input', function(e) {
      const query = e.target.value.toLowerCase().trim();
      selectedIndex = -1;
      
      if (query.length === 0) {
        resultsContainer.innerHTML = '<div class="muted" role="status" aria-live="polite">Type to search...</div>';
        return;
      }
      
      const results = searchableItems.filter(item => 
        item.name.toLowerCase().includes(query) || 
        item.category.toLowerCase().includes(query)
      );
      
      if (results.length === 0) {
        resultsContainer.innerHTML = '<div class="muted" role="status" aria-live="polite">No results found</div>';
        return;
      }
      
      resultsContainer.setAttribute('role', 'listbox');
      resultsContainer.setAttribute('aria-label', 'Search results');
      resultsContainer.innerHTML = results.map((item, index) => `
        <div class="search-result-item" 
             role="option" 
             id="search-result-${index}"
             tabindex="0"
             aria-label="${item.name}, ${item.category}, ${item.url}"
             aria-selected="false"
             style="padding: 12px; border: 1px solid var(--border, #262b36); border-radius: 8px; margin-bottom: 8px; cursor: pointer; transition: background 0.2s; outline: none;" 
             onclick="window.location.href='${item.url}'; hideModal('${modalId}');"
             onkeydown="if(event.key==='Enter'||event.key===' '){event.preventDefault();window.location.href='${item.url}';hideModal('${modalId}');}"
             onfocus="this.style.background='var(--surface-secondary, #1e2329)';this.setAttribute('aria-selected','true');selectedIndex=${index};"
             onblur="this.style.background='var(--bg, #0f1216)';this.setAttribute('aria-selected','false');"
             onmouseover="this.style.background='var(--surface-secondary, #1e2329)';this.setAttribute('aria-selected','true');selectedIndex=${index};"
             onmouseout="if(document.activeElement!==this){this.style.background='var(--bg, #0f1216)';this.setAttribute('aria-selected','false');}">
          <div style="font-weight: bold; color: var(--primary, #67e480);">${item.name}</div>
          <div class="muted" style="font-size: 0.85em; margin-top: 4px;">${item.category} • ${item.url}</div>
        </div>
      `).join('');
      
      // Announce results count to screen readers
      const statusMsg = results.length === 1 ? '1 result found' : results.length + ' results found';
      resultsContainer.setAttribute('aria-label', 'Search results: ' + statusMsg);
    });
    
    // Enhanced keyboard navigation for search modal
    searchInput.addEventListener('keydown', function(e) {
      if (e.key === 'Escape') {
        hideModal(modalId);
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        const items = resultsContainer.querySelectorAll('.search-result-item[role="option"]');
        if (items.length > 0) {
          selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
          items[selectedIndex].focus();
        }
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        const items = resultsContainer.querySelectorAll('.search-result-item[role="option"]');
        if (items.length > 0) {
          selectedIndex = Math.max(selectedIndex - 1, -1);
          if (selectedIndex >= 0) {
            items[selectedIndex].focus();
          } else {
            searchInput.focus();
          }
        }
      }
    });
    
    // Handle keyboard navigation in results container
    resultsContainer.addEventListener('keydown', function(e) {
      const items = Array.from(resultsContainer.querySelectorAll('.search-result-item[role="option"]'));
      const currentIndex = items.findIndex(item => item === document.activeElement);
      
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        const nextIndex = Math.min(currentIndex + 1, items.length - 1);
        items[nextIndex].focus();
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        if (currentIndex > 0) {
          items[currentIndex - 1].focus();
        } else {
          searchInput.focus();
        }
      } else if (e.key === 'Home') {
        e.preventDefault();
        if (items.length > 0) {
          items[0].focus();
        }
      } else if (e.key === 'End') {
        e.preventDefault();
        if (items.length > 0) {
          items[items.length - 1].focus();
        }
      }
    });
    
    // Focus trap
    modal.addEventListener('keydown', function(e) {
      if (e.key === 'Tab') {
        const focusableElements = modal.querySelectorAll('input, button, [tabindex]:not([tabindex="-1"])');
        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];
        
        if (e.shiftKey && document.activeElement === firstElement) {
          e.preventDefault();
          lastElement.focus();
        } else if (!e.shiftKey && document.activeElement === lastElement) {
          e.preventDefault();
          firstElement.focus();
        }
      }
    });
  }
  
  showModal(modalId);
  const searchInput = document.getElementById('globalSearchInput');
  if (searchInput) {
    searchInput.focus();
    searchInput.select();
  }
}

function confirmAction(message, onConfirm) {
  if (confirm(message)) {
    onConfirm();
  }
}

// Enhanced confirmation dialog with ARIA support
function showConfirmDialog(message, onConfirm, onCancel = null) {
  const dialogId = 'confirmDialog';
  let dialog = document.getElementById(dialogId);
  
  if (!dialog) {
    dialog = document.createElement('div');
    dialog.id = dialogId;
    dialog.className = 'modal';
    dialog.setAttribute('role', 'dialog');
    dialog.setAttribute('aria-labelledby', 'confirmDialogTitle');
    dialog.setAttribute('aria-describedby', 'confirmMessage');
    dialog.setAttribute('aria-modal', 'true');
    dialog.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h3 id="confirmDialogTitle">Confirm Action</h3>
          <button class="modal-close" aria-label="Close dialog" onclick="hideModal('${dialogId}')">&times;</button>
        </div>
        <div id="confirmMessage" style="margin-bottom:20px; color:#e8e8e8;"></div>
        <div class="modal-footer">
          <button class="btn" onclick="hideModal('${dialogId}')">Cancel</button>
          <button class="btn btn-danger" id="confirmBtn">Confirm</button>
        </div>
      </div>
    `;
    document.body.appendChild(dialog);
  }
  
  document.getElementById('confirmMessage').textContent = message;
  const confirmBtn = document.getElementById('confirmBtn');
  const oldHandler = confirmBtn.onclick;
  confirmBtn.onclick = function() {
    hideModal(dialogId);
    if (onConfirm) onConfirm();
  };
  
  const closeBtn = dialog.querySelector('.modal-close');
  closeBtn.onclick = function() {
    hideModal(dialogId);
    if (onCancel) onCancel();
  };
  
  showModal(dialogId);
}

async function fetchJson(url, options = {}) {
  const headers = getAuthHeaders();
  if (options.headers) {
    Object.assign(headers, options.headers);
  }
  const res = await fetch(url, { ...options, headers });
  if (res.status === 401) {
    const refreshed = await refreshToken();
    if (!refreshed) {
      removeToken();
      updateUI();
      if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
        window.location.href = '/ui/auth';
      }
      throw new Error('Unauthorized');
    }
    // Retry with new token
    headers['authorization'] = 'Bearer ' + getToken();
    const retryRes = await fetch(url, { ...options, headers });
    if (retryRes.status === 401) {
      removeToken();
      updateUI();
      if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
        window.location.href = '/ui/auth';
      }
      throw new Error('Unauthorized');
    }
    if (!retryRes.ok) {
      const errorData = await retryRes.json().catch(() => ({}));
      throw new Error(apiErrorMessageFromBody(errorData) || 'HTTP ' + retryRes.status);
    }
    return await retryRes.json();
  }
  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(apiErrorMessageFromBody(errorData) || 'HTTP ' + res.status);
  }
  return await res.json();
}

function setUpdated() {
  const el = document.getElementById('last_updated');
  if (!el) return;
  const p = typeof poolaiT === 'function' ? poolaiT('dash.updatedPrefix', 'Updated: ') : 'Updated: ';
  el.textContent = p + new Date().toLocaleTimeString();
}

function renderJsonPre(containerId, data) {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = '';
  const pre = document.createElement('pre');
  pre.textContent = JSON.stringify(data, null, 2);
  el.appendChild(pre);
}

function renderTable(containerId, data) {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = '';

  if (!Array.isArray(data)) {
    renderJsonPre(containerId, data);
    return;
  }

  if (data.length === 0) {
    el.innerHTML = '<div class=\"muted\">No items.</div>';
    return;
  }

  const keys = new Set();
  for (const row of data) {
    if (row && typeof row === 'object') {
      Object.keys(row).forEach(k => keys.add(k));
    }
  }
  const cols = Array.from(keys);
  if (cols.length === 0) {
    renderJsonPre(containerId, data);
    return;
  }

  const table = document.createElement('table');
  const thead = document.createElement('thead');
  const hr = document.createElement('tr');
  cols.forEach(k => {
    const th = document.createElement('th');
    th.textContent = k;
    hr.appendChild(th);
  });
  thead.appendChild(hr);
  table.appendChild(thead);

  const tbody = document.createElement('tbody');
  for (const row of data) {
    const tr = document.createElement('tr');
    cols.forEach(k => {
      const td = document.createElement('td');
      const v = row ? row[k] : null;
      td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
      tr.appendChild(td);
    });
    
    // Add action buttons for VM instances
    if (row && row.id && window.location.pathname.includes('/vm')) {
      const actionsTd = document.createElement('td');
      actionsTd.className = 'action-buttons';
      actionsTd.style.cssText = 'white-space: nowrap;';
      
      const instanceId = row.id;
      const status = row.status || '';
      
      // Start button
      if (status !== 'Running') {
        const startBtn = document.createElement('button');
        startBtn.className = 'btn btn-primary';
        startBtn.textContent = 'Start';
        startBtn.setAttribute('aria-label', `Start VM instance ${instanceId}`);
        startBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        startBtn.onclick = () => handleVmAction(instanceId, 'start');
        actionsTd.appendChild(startBtn);
      }
      
      // Stop button
      if (status === 'Running') {
        const stopBtn = document.createElement('button');
        stopBtn.className = 'btn';
        stopBtn.textContent = 'Stop';
        stopBtn.setAttribute('aria-label', `Stop VM instance ${instanceId}`);
        stopBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        stopBtn.onclick = () => handleVmAction(instanceId, 'stop');
        actionsTd.appendChild(stopBtn);
      }
      
      // Restart button
      if (status === 'Running') {
        const restartBtn = document.createElement('button');
        restartBtn.className = 'btn';
        restartBtn.textContent = 'Restart';
        restartBtn.setAttribute('aria-label', `Restart VM instance ${instanceId}`);
        restartBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
        restartBtn.onclick = () => handleVmAction(instanceId, 'restart');
        actionsTd.appendChild(restartBtn);
      }
      
      // Delete button
      const deleteBtn = document.createElement('button');
      deleteBtn.className = 'btn btn-danger';
      deleteBtn.textContent = 'Delete';
      deleteBtn.setAttribute('aria-label', `Delete VM instance ${instanceId}`);
      deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
      deleteBtn.onclick = () => handleVmDelete(instanceId, row.name || instanceId);
      actionsTd.appendChild(deleteBtn);
      
      tr.appendChild(actionsTd);
    }
    
    tbody.appendChild(tr);
  }
  table.appendChild(tbody);
  
  // Обгортаємо таблицю в container для автоматичного вирівнювання
  const container = document.createElement('div');
  container.className = 'table-container';
  container.appendChild(table);
  el.appendChild(container);
}

// Enhanced form validation with real-time feedback
function validateForm(formId) {
  const form = document.getElementById(formId);
  if (!form) return false;
  
  const inputs = form.querySelectorAll('input[required], select[required], textarea[required]');
  let isValid = true;
  
  inputs.forEach(input => {
    const fieldValid = validateField(input);
    if (!fieldValid) {
      isValid = false;
    }
  });
  
  return isValid;
}

function validateField(input) {
  let isValid = true;
  let errorMessage = '';
  
  // Remove previous error message
  const existingError = input.parentElement.querySelector('.error-text');
  if (existingError) {
    existingError.remove();
  }
  
  // Required validation
  if (input.hasAttribute('required') && !input.value.trim()) {
    isValid = false;
    errorMessage = 'This field is required';
  }
  
  // Number validation
  if (input.type === 'number' && input.value) {
    const min = input.getAttribute('min');
    const max = input.getAttribute('max');
    const value = parseFloat(input.value);
    
    if (isNaN(value)) {
      isValid = false;
      errorMessage = 'Please enter a valid number';
    } else if (min && value < parseFloat(min)) {
      isValid = false;
      errorMessage = `Value must be at least ${min}`;
    } else if (max && value > parseFloat(max)) {
      isValid = false;
      errorMessage = `Value must be at most ${max}`;
    }
  }
  
  // Email validation
  if (input.type === 'email' && input.value) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(input.value)) {
      isValid = false;
      errorMessage = 'Please enter a valid email address';
    }
  }
  
  // Pattern validation
  if (input.hasAttribute('pattern') && input.value) {
    const pattern = new RegExp(input.getAttribute('pattern'));
    if (!pattern.test(input.value)) {
      isValid = false;
      errorMessage = input.getAttribute('data-pattern-error') || 'Invalid format';
    }
  }
  
  // Update UI
  if (isValid) {
    input.style.borderColor = '';
    input.classList.remove('error');
  } else {
    input.style.borderColor = 'var(--danger, #ff5555)';
    input.classList.add('error');
    
    // Show error message
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error-text';
    errorDiv.textContent = errorMessage;
    input.parentElement.appendChild(errorDiv);
  }
  
  return isValid;
}

// Real-time form validation
function initRealTimeValidation(formId) {
  const form = document.getElementById(formId);
  if (!form) return;
  
  const inputs = form.querySelectorAll('input, select, textarea');
  inputs.forEach(input => {
    // Validate on blur
    input.addEventListener('blur', function() {
      validateField(input);
    });
    
    // Validate on input (for immediate feedback)
    input.addEventListener('input', function() {
      if (input.classList.contains('error')) {
        validateField(input);
      }
    });
  });
}

// Form auto-save functionality
function initFormAutoSave(formId, storageKey, interval = 30000) {
  const form = document.getElementById(formId);
  if (!form) return;
  
  // Load saved data
  const savedData = localStorage.getItem(storageKey);
  if (savedData) {
    try {
      const data = JSON.parse(savedData);
      Object.keys(data).forEach(key => {
        const input = form.querySelector(`[name="${key}"]`);
        if (input && !input.value) {
          input.value = data[key];
        }
      });
    } catch (e) {
      console.error('Failed to load auto-saved form data:', e);
    }
  }
  
  // Auto-save on input
  let saveTimeout;
  form.addEventListener('input', function() {
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => {
      const formData = new FormData(form);
      const data = {};
      for (const [key, value] of formData.entries()) {
        data[key] = value;
      }
      localStorage.setItem(storageKey, JSON.stringify(data));
    }, 1000); // Debounce 1 second
  });
  
  // Clear saved data on successful submit
  form.addEventListener('submit', function() {
    localStorage.removeItem(storageKey);
  });
}

// Form wizard functionality
function initFormWizard(wizardId) {
  const wizard = document.getElementById(wizardId);
  if (!wizard) return;
  
  const steps = wizard.querySelectorAll('.wizard-step');
  let currentStep = 0;
  
  steps.forEach((step, index) => {
    step.style.display = index === 0 ? 'block' : 'none';
    step.setAttribute('data-step', index.toString());
  });
  
  window.nextWizardStep = function() {
    if (currentStep < steps.length - 1) {
      steps[currentStep].style.display = 'none';
      currentStep++;
      steps[currentStep].style.display = 'block';
      updateWizardProgress(wizard, currentStep, steps.length);
    }
  };
  
  window.prevWizardStep = function() {
    if (currentStep > 0) {
      steps[currentStep].style.display = 'none';
      currentStep--;
      steps[currentStep].style.display = 'block';
      updateWizardProgress(wizard, currentStep, steps.length);
    }
  };
  
  updateWizardProgress(wizard, currentStep, steps.length);
}

function updateWizardProgress(wizard, current, total) {
  const progress = wizard.querySelector('.wizard-progress');
  if (progress) {
    const percentage = ((current + 1) / total) * 100;
    progress.style.width = percentage + '%';
  }
  
  const stepIndicator = wizard.querySelector('.wizard-step-indicator');
  if (stepIndicator) {
    stepIndicator.textContent = `Step ${current + 1} of ${total}`;
  }
}

// VM action handlers
async function handleVmAction(instanceId, action) {
  const user = getUser();
  if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
    showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
    return;
  }
  
  try {
    showLoading('data', `${action.charAt(0).toUpperCase() + action.slice(1)}ing VM instance...`);
    
    const res = await fetchJson(`/api/v1/vm/instances/${instanceId}/${action}`, {
      method: 'POST'
    });
    
    showNotification(res.message || `VM instance ${action}ed successfully`, 'success');
    setTimeout(() => {
      const refreshFn = window.refreshVmInstances || (() => location.reload());
      refreshFn();
    }, 1000);
  } catch (e) {
    showNotification('Error: ' + e.message, 'error');
    hideLoading('data');
  }
}

async function handleVmDelete(instanceId, instanceName) {
  const user = getUser();
  if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
    showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
    return;
  }
  
  showConfirmDialog(
    `Are you sure you want to delete VM instance "${instanceName}" (${instanceId})? This action cannot be undone.`,
    async () => {
      try {
        showLoading('data', 'Deleting VM instance...');
        
        await fetchJson(`/api/v1/vm/instances/${instanceId}`, {
          method: 'DELETE'
        });
        
        showNotification('VM instance deleted successfully', 'success');
        setTimeout(() => {
          const refreshFn = window.refreshVmInstances || (() => location.reload());
          refreshFn();
        }, 1000);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        hideLoading('data');
      }
    }
  );
}

// Polling with request deduplication and retry logic
const activePolls = new Map();
const pollRetries = new Map();

async function poll(url, renderFn, containerId, retries = 3) {
  // Prevent duplicate requests for the same URL
  if (activePolls.has(url)) {
    return activePolls.get(url);
  }
  
  const pollPromise = (async () => {
    try {
      const data = await fetchJson(url);
      renderFn(containerId, data);
      setUpdated();
      pollRetries.delete(url); // Reset retry count on success
      activePolls.delete(url);
      return data;
    } catch (e) {
      activePolls.delete(url);
      
      // Retry logic with exponential backoff
      const retryCount = pollRetries.get(url) || 0;
      if (retryCount < retries) {
        pollRetries.set(url, retryCount + 1);
        const delay = Math.min(1000 * Math.pow(2, retryCount), 10000); // Max 10s delay
        
        await new Promise(resolve => setTimeout(resolve, delay));
        return poll(url, renderFn, containerId, retries);
      }
      
      // Max retries reached, show error
      pollRetries.delete(url);
      const el = document.getElementById(containerId);
      if (el) {
        const errorMsg = String(e);
        el.innerHTML = '<div style="color:#ff5555; padding:12px; border:1px solid #ff5555; border-radius:8px;">Error: ' + errorMsg + '<br><button onclick="location.reload()" style="margin-top:8px; padding:6px 12px; background:#ff5555; color:white; border:none; border-radius:4px; cursor:pointer;">Retry</button></div>';
      }
      console.error('Poll error (max retries reached):', e);
    }
  })();
  
  activePolls.set(url, pollPromise);
  return pollPromise;
}

// Token validation and refresh
async function validateToken() {
  const token = getToken();
  if (!token) return false;
  
  try {
    // Decode token to check expiration (simple base64 decode for dev tokens)
    const parts = token.split('.');
    if (parts.length === 3) {
      // Real JWT format
      const payload = JSON.parse(atob(parts[1]));
      const now = Math.floor(Date.now() / 1000);
      if (payload.exp && payload.exp < now) {
        // Token expired, try to refresh
        return await refreshToken();
      }
      return true;
    } else if (token.startsWith('dev_token_')) {
      // Dev token format - check expiration from localStorage
      const tokenData = localStorage.getItem('poolai_token_exp');
      if (tokenData) {
        const exp = parseInt(tokenData, 10);
        const now = Math.floor(Date.now() / 1000);
        if (exp && exp < now) {
          return await refreshToken();
        }
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token validation error:', e);
    return false;
  }
}

async function refreshToken() {
  try {
    const token = getToken();
    if (!token) return false;
    
    // Try to refresh token via API (if endpoint exists)
    const res = await fetch('/api/v1/refresh', {
      method: 'POST',
      headers: getAuthHeaders(),
    });
    
    if (res.ok) {
      const data = await res.json();
      setToken(data.token);
      if (data.role) {
        const user = getUser();
        if (user) setUser(user.username, data.role);
      }
      if (data.expires_in) {
        const exp = Math.floor(Date.now() / 1000) + data.expires_in;
        localStorage.setItem('poolai_token_exp', exp.toString());
      }
      return true;
    }
    return false;
  } catch (e) {
    console.error('Token refresh error:', e);
    return false;
  }
}

// Protected route check
async function requireAuth(requiredRole = null) {
  const user = getUser();
  if (!user) {
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  const isValid = await validateToken();
  if (!isValid) {
    removeToken();
    updateUI();
    if (window.location.pathname !== '/ui/auth' && window.location.pathname !== '/ui/login') {
      window.location.href = '/ui/auth';
    }
    return false;
  }
  
  if (requiredRole) {
    const roleHierarchy = { 'Viewer': 1, 'Operator': 2, 'Admin': 3 };
    const userLevel = roleHierarchy[user.role] || 0;
    const requiredLevel = roleHierarchy[requiredRole] || 0;
    
    if (userLevel < requiredLevel) {
      alert('Insufficient permissions. Required role: ' + requiredRole);
      window.location.href = '/ui';
      return false;
    }
  }
  
  return true;
}

// Initialize UI on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', async function() {
    updateUI();
    // Check auth for protected routes (except login/auth pages)
    const path = window.location.pathname;
    if (path !== '/ui/auth' && path !== '/ui/login' && !path.endsWith('/auth') && !path.endsWith('/login')) {
      await requireAuth();
    }
  });
} else {
  updateUI();
  // Check auth for protected routes
  const path = window.location.pathname;
  if (path !== '/ui/auth' && path !== '/ui/login' && !path.endsWith('/auth') && !path.endsWith('/login')) {
    requireAuth();
  }
}

// Theme management
function getTheme() {
  return localStorage.getItem('poolai_theme') || 'dark';
}

function setTheme(themeName) {
  localStorage.setItem('poolai_theme', themeName);
  applyTheme(themeName);
}

function applyTheme(themeName) {
  const themes = {
    dark: {
      bg: '#0f1216', surface: '#171b22', surfaceSecondary: '#0f1216',
      text: '#e8e8e8', textMuted: '#a8b0bf', border: '#262b36',
      primary: '#50fa7b', primaryHover: '#67e480',
      danger: '#ff5555', dangerHover: '#ff6e6e',
      secondary: '#6272a4', secondaryHover: '#7a8bc4',
      success: '#50fa7b', warning: '#f1fa8c', info: '#8be9fd',
      link: '#77c7ff', linkHover: '#8bd5ff'
    },
    light: {
      bg: '#ffffff', surface: '#f5f5f5', surfaceSecondary: '#e8e8e8',
      text: '#1a1a1a', textMuted: '#666666', border: '#d0d0d0',
      primary: '#00a86b', primaryHover: '#00c47a',
      danger: '#dc3545', dangerHover: '#c82333',
      secondary: '#6c757d', secondaryHover: '#5a6268',
      success: '#28a745', warning: '#ffc107', info: '#17a2b8',
      link: '#007bff', linkHover: '#0056b3'
    },
    'high-contrast': {
      bg: '#000000', surface: '#1a1a1a', surfaceSecondary: '#000000',
      text: '#ffffff', textMuted: '#cccccc', border: '#ffffff',
      primary: '#00ff00', primaryHover: '#00cc00',
      danger: '#ff0000', dangerHover: '#cc0000',
      secondary: '#ffff00', secondaryHover: '#cccc00',
      success: '#00ff00', warning: '#ffff00', info: '#00ffff',
      link: '#00aaff', linkHover: '#0088cc'
    }
  };
  
  const theme = themes[themeName] || themes.dark;
  const root = document.documentElement;
  
  root.style.setProperty('--bg', theme.bg);
  root.style.setProperty('--surface', theme.surface);
  root.style.setProperty('--surface-secondary', theme.surfaceSecondary);
  root.style.setProperty('--text', theme.text);
  root.style.setProperty('--text-muted', theme.textMuted);
  root.style.setProperty('--border', theme.border);
  root.style.setProperty('--primary', theme.primary);
  root.style.setProperty('--primary-hover', theme.primaryHover);
  root.style.setProperty('--danger', theme.danger);
  root.style.setProperty('--danger-hover', theme.dangerHover);
  root.style.setProperty('--secondary', theme.secondary);
  root.style.setProperty('--secondary-hover', theme.secondaryHover);
  root.style.setProperty('--success', theme.success);
  root.style.setProperty('--warning', theme.warning);
  root.style.setProperty('--info', theme.info);
  root.style.setProperty('--link', theme.link);
  root.style.setProperty('--link-hover', theme.linkHover);
}

// Progress Bar functions
function updateProgressBar(barId, value, max = 100) {
  const bar = document.getElementById(barId);
  if (!bar) return;
  
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100);
  const fill = bar.querySelector('.progress-bar-fill');
  if (fill) {
    fill.style.width = percentage + '%';
  }
  
  const labelValue = bar.querySelector('.progress-bar-label-value');
  if (labelValue) {
    labelValue.textContent = Math.round(percentage) + '%';
  }
}

function updateCircularProgress(circleId, value, max = 100) {
  const circle = document.getElementById(circleId);
  if (!circle) return;
  
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100);
  const circumference = 2 * Math.PI * 30; // radius = 30
  const offset = circumference - (percentage / 100) * circumference;
  
  const fill = circle.querySelector('.progress-bar-circular-fill');
  if (fill) {
    fill.style.strokeDashoffset = offset;
  }
  
  const text = circle.querySelector('.progress-bar-circular-text');
  if (text) {
    text.textContent = Math.round(percentage) + '%';
  }
}

// Tooltip functions with enhanced accessibility
function initTooltips() {
  const tooltips = document.querySelectorAll('[data-tooltip]');
  tooltips.forEach((tooltip, index) => {
    const text = tooltip.getAttribute('data-tooltip');
    const position = tooltip.getAttribute('data-tooltip-position') || 'top';
    const delay = parseInt(tooltip.getAttribute('data-tooltip-delay')) || 0;
    const tooltipId = 'tooltip-' + index;
    
    if (!tooltip.querySelector('.tooltip-content')) {
      const content = document.createElement('div');
      content.id = tooltipId;
      content.className = 'tooltip-content';
      content.setAttribute('role', 'tooltip');
      content.textContent = text;
      tooltip.classList.add('tooltip', 'tooltip-' + position);
      tooltip.setAttribute('aria-describedby', tooltipId);
      tooltip.appendChild(content);
      
      // Show on focus for keyboard users
      tooltip.addEventListener('focus', function() {
        content.style.visibility = 'visible';
        content.style.opacity = '1';
      });
      tooltip.addEventListener('blur', function() {
        content.style.visibility = 'hidden';
        content.style.opacity = '0';
      });
      
      if (delay > 0) {
        let timeout;
        tooltip.addEventListener('mouseenter', function() {
          timeout = setTimeout(() => {
            content.style.visibility = 'visible';
            content.style.opacity = '1';
          }, delay);
        });
        tooltip.addEventListener('mouseleave', function() {
          clearTimeout(timeout);
          content.style.visibility = 'hidden';
          content.style.opacity = '0';
        });
      } else {
        tooltip.addEventListener('mouseenter', function() {
          content.style.visibility = 'visible';
          content.style.opacity = '1';
        });
        tooltip.addEventListener('mouseleave', function() {
          if (document.activeElement !== tooltip) {
            content.style.visibility = 'hidden';
            content.style.opacity = '0';
          }
        });
      }
    }
  });
}

// Dropdown functions with enhanced accessibility
function initDropdowns() {
  const dropdowns = document.querySelectorAll('.dropdown');
  dropdowns.forEach((dropdown, index) => {
    const toggle = dropdown.querySelector('.dropdown-toggle');
    const menu = dropdown.querySelector('.dropdown-menu');
    if (!toggle || !menu) return;
    
    const menuId = 'dropdown-menu-' + index;
    menu.id = menuId;
    toggle.setAttribute('aria-haspopup', 'true');
    toggle.setAttribute('aria-controls', menuId);
    toggle.setAttribute('aria-expanded', 'false');
    menu.setAttribute('role', 'menu');
    
    toggle.addEventListener('click', function(e) {
      e.stopPropagation();
      const isActive = menu.classList.contains('active');
      closeAllDropdowns();
      if (!isActive) {
        menu.classList.add('active');
        toggle.setAttribute('aria-expanded', 'true');
        // Focus first item
        const firstItem = menu.querySelector('.dropdown-item');
        if (firstItem) {
          setTimeout(() => firstItem.focus(), 0);
        }
      }
    });
    
    // Keyboard navigation
    toggle.addEventListener('keydown', function(e) {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        toggle.click();
      } else if (e.key === 'Escape') {
        closeAllDropdowns();
      }
    });
    
    // Close on outside click
    document.addEventListener('click', function(e) {
      if (!dropdown.contains(e.target)) {
        menu.classList.remove('active');
        toggle.setAttribute('aria-expanded', 'false');
      }
    });
    
    // Item selection
    const items = menu.querySelectorAll('.dropdown-item');
    items.forEach((item, index) => {
      item.setAttribute('role', 'menuitem');
      item.setAttribute('tabindex', '-1');
      
      item.addEventListener('click', function() {
        const value = item.getAttribute('data-value');
        if (value !== null) {
          toggle.textContent = item.textContent;
          toggle.setAttribute('data-value', value);
          menu.classList.remove('active');
          toggle.setAttribute('aria-expanded', 'false');
          
          // Trigger change event
          const event = new CustomEvent('dropdown-change', { detail: { value, text: item.textContent } });
          dropdown.dispatchEvent(event);
          
          // Return focus to toggle
          toggle.focus();
        }
      });
      
      // Keyboard navigation
      item.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          item.click();
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          const next = items[index + 1] || items[0];
          next.focus();
          next.setAttribute('tabindex', '0');
          item.setAttribute('tabindex', '-1');
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          const prev = items[index - 1] || items[items.length - 1];
          prev.focus();
          prev.setAttribute('tabindex', '0');
          item.setAttribute('tabindex', '-1');
        } else if (e.key === 'Home') {
          e.preventDefault();
          items[0].focus();
          items[0].setAttribute('tabindex', '0');
          item.setAttribute('tabindex', '-1');
        } else if (e.key === 'End') {
          e.preventDefault();
          items[items.length - 1].focus();
          items[items.length - 1].setAttribute('tabindex', '0');
          item.setAttribute('tabindex', '-1');
        } else if (e.key === 'Escape') {
          closeAllDropdowns();
          toggle.focus();
        }
      });
    });
    
    // Initialize first item as focusable
    if (items.length > 0) {
      items[0].setAttribute('tabindex', '0');
    }
  });
}

function closeAllDropdowns() {
  const dropdowns = document.querySelectorAll('.dropdown-menu');
  dropdowns.forEach(menu => {
    menu.classList.remove('active');
    const toggle = menu.parentElement.querySelector('.dropdown-toggle');
    if (toggle) {
      toggle.setAttribute('aria-expanded', 'false');
    }
  });
}

// Tabs functions with enhanced accessibility
function initTabs() {
  const tabContainers = document.querySelectorAll('.tabs-container');
  tabContainers.forEach((container, containerIndex) => {
    const tabs = container.querySelectorAll('.tab');
    const contents = container.querySelectorAll('.tab-content');
    const tabsId = 'tabs-' + containerIndex;
    
    container.setAttribute('role', 'tablist');
    container.setAttribute('aria-label', container.getAttribute('aria-label') || 'Tabs');
    
    tabs.forEach((tab, index) => {
      const contentId = tab.getAttribute('data-tab') || ('tab-content-' + containerIndex + '-' + index);
      const tabId = 'tab-' + containerIndex + '-' + index;
      
      tab.id = tabId;
      tab.setAttribute('role', 'tab');
      tab.setAttribute('aria-controls', contentId);
      tab.setAttribute('aria-selected', index === 0 ? 'true' : 'false');
      tab.setAttribute('tabindex', index === 0 ? '0' : '-1');
      
      if (contents[index]) {
        contents[index].id = contentId;
        contents[index].setAttribute('role', 'tabpanel');
        contents[index].setAttribute('aria-labelledby', tabId);
        contents[index].setAttribute('aria-hidden', index === 0 ? 'false' : 'true');
      }
      
      tab.addEventListener('click', function() {
        // Remove active class from all tabs and contents
        tabs.forEach(t => {
          t.classList.remove('active');
          t.setAttribute('tabindex', '-1');
        });
        contents.forEach(c => c.classList.remove('active'));
        
        // Add active class to clicked tab and corresponding content
        tab.classList.add('active');
        tab.setAttribute('tabindex', '0');
        const contentId = tab.getAttribute('data-tab') || ('tab-content-' + containerIndex + '-' + index);
        const content = container.querySelector('#' + contentId) || contents[index];
        if (content) {
          content.classList.add('active');
        }
        
        // Update ARIA attributes
        tabs.forEach((t, i) => {
          t.setAttribute('aria-selected', i === index ? 'true' : 'false');
          if (contents[i]) {
            contents[i].setAttribute('aria-hidden', i === index ? 'false' : 'true');
          }
        });
      });
      
      // Keyboard navigation (ARIA tab pattern)
      tab.addEventListener('keydown', function(e) {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
          e.preventDefault();
          const direction = e.key === 'ArrowRight' ? 1 : -1;
          const nextIndex = (index + direction + tabs.length) % tabs.length;
          tabs[nextIndex].click();
          tabs[nextIndex].focus();
        } else if (e.key === 'Home') {
          e.preventDefault();
          tabs[0].click();
          tabs[0].focus();
        } else if (e.key === 'End') {
          e.preventDefault();
          tabs[tabs.length - 1].click();
          tabs[tabs.length - 1].focus();
        }
      });
    });
  });
}

// Accordion functions with enhanced accessibility
function initAccordions() {
  const accordions = document.querySelectorAll('.accordion');
  accordions.forEach((accordion, accordionIndex) => {
    const items = accordion.querySelectorAll('.accordion-item');
    
    items.forEach((item, index) => {
      const header = item.querySelector('.accordion-header');
      const content = item.querySelector('.accordion-content');
      if (!header || !content) return;
      
      const headerId = 'accordion-header-' + accordionIndex + '-' + index;
      const contentId = 'accordion-content-' + accordionIndex + '-' + index;
      
      header.id = headerId;
      header.setAttribute('role', 'button');
      header.setAttribute('aria-expanded', 'false');
      header.setAttribute('aria-controls', contentId);
      header.setAttribute('tabindex', '0');
      
      content.id = contentId;
      content.setAttribute('role', 'region');
      content.setAttribute('aria-labelledby', headerId);
      
      header.addEventListener('click', function() {
        const isActive = item.classList.contains('active');
        
        // Close all items if not allowing multiple open
        if (!accordion.hasAttribute('data-multiple')) {
          items.forEach(i => {
            i.classList.remove('active');
            i.querySelector('.accordion-header').setAttribute('aria-expanded', 'false');
          });
        }
        
        // Toggle current item
        if (isActive) {
          item.classList.remove('active');
          header.setAttribute('aria-expanded', 'false');
        } else {
          item.classList.add('active');
          header.setAttribute('aria-expanded', 'true');
        }
      });
      
      // Keyboard navigation
      header.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          header.click();
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          const next = item.nextElementSibling;
          if (next) {
            next.querySelector('.accordion-header').focus();
          }
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          const prev = item.previousElementSibling;
          if (prev) {
            prev.querySelector('.accordion-header').focus();
          }
        } else if (e.key === 'Home') {
          e.preventDefault();
          items[0].querySelector('.accordion-header').focus();
        } else if (e.key === 'End') {
          e.preventDefault();
          items[items.length - 1].querySelector('.accordion-header').focus();
        }
      });
      
      header.setAttribute('role', 'button');
      header.setAttribute('aria-expanded', 'false');
      header.setAttribute('tabindex', '0');
      content.setAttribute('role', 'region');
    });
  });
}

// Enhanced Mobile Navigation functions with touch gestures
function initMobileNavigation() {
  const toggle = document.getElementById('mobileMenuToggle');
  const drawer = document.getElementById('mobileNavDrawer');
  const overlay = document.getElementById('mobileNavOverlay');
  const closeBtn = document.getElementById('mobileNavClose');
  const mobileThemeSelector = document.getElementById('mobileThemeSelector');
  
  // Touch gesture variables
  let touchStartX = 0;
  let touchStartY = 0;
  let touchEndX = 0;
  let touchEndY = 0;
  const swipeThreshold = 50; // Minimum swipe distance in pixels
  const swipeAngleThreshold = 30; // Maximum angle from horizontal for swipe detection
  
  if (!toggle || !drawer || !overlay) return;
  
  function openDrawer() {
    drawer.classList.add('active');
    overlay.classList.add('active');
    toggle.setAttribute('aria-expanded', 'true');
    document.body.style.overflow = 'hidden';
  }
  
  function closeDrawer() {
    drawer.classList.remove('active');
    overlay.classList.remove('active');
    toggle.setAttribute('aria-expanded', 'false');
    document.body.style.overflow = '';
  }
  
  toggle.addEventListener('click', openDrawer);
  if (closeBtn) closeBtn.addEventListener('click', closeDrawer);
  overlay.addEventListener('click', closeDrawer);
  
  // Close on escape key
  document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && drawer.classList.contains('active')) {
      closeDrawer();
    }
  });
  
  // Close drawer when clicking on nav item
  const navItems = drawer.querySelectorAll('.mobile-nav-item');
  navItems.forEach(item => {
    item.addEventListener('click', function() {
      setTimeout(closeDrawer, 100);
    });
  });
  
  // Sync mobile theme selector with main theme selector
  if (mobileThemeSelector) {
    const mainThemeSelector = document.getElementById('themeSelector');
    if (mainThemeSelector) {
      mobileThemeSelector.value = mainThemeSelector.value;
      mobileThemeSelector.addEventListener('change', function(e) {
        const newTheme = e.target.value;
        setTheme(newTheme);
        if (mainThemeSelector) {
          mainThemeSelector.value = newTheme;
        }
      });
    }
  }
  
  // Swipe to open drawer from left edge
  document.addEventListener('touchstart', function(e) {
    touchStartX = e.touches[0].clientX;
    touchStartY = e.touches[0].clientY;
  }, { passive: true });
  
  document.addEventListener('touchend', function(e) {
    if (!e.changedTouches || e.changedTouches.length === 0) return;
    
    touchEndX = e.changedTouches[0].clientX;
    touchEndY = e.changedTouches[0].clientY;
    
    const diffX = touchEndX - touchStartX;
    const diffY = touchEndY - touchStartY;
    const angle = Math.abs(Math.atan2(diffY, diffX) * 180 / Math.PI);
    
    // Check if it's a horizontal swipe (within angle threshold)
    if (angle > swipeAngleThreshold && angle < (180 - swipeAngleThreshold)) return;
    
    // Swipe right from left edge to open drawer
    if (touchStartX < 30 && diffX > swipeThreshold && !drawer.classList.contains('active')) {
      openDrawer();
    }
    
    // Swipe left to close drawer
    if (diffX < -swipeThreshold && drawer.classList.contains('active')) {
      closeDrawer();
    }
  }, { passive: true });
  
  // Swipe on drawer to close
  drawer.addEventListener('touchstart', function(e) {
    touchStartX = e.touches[0].clientX;
    touchStartY = e.touches[0].clientY;
  }, { passive: true });
  
  drawer.addEventListener('touchend', function(e) {
    if (!e.changedTouches || e.changedTouches.length === 0) return;
    
    touchEndX = e.changedTouches[0].clientX;
    touchEndY = e.changedTouches[0].clientY;
    
    const diffX = touchEndX - touchStartX;
    const diffY = touchEndY - touchStartY;
    const angle = Math.abs(Math.atan2(diffY, diffX) * 180 / Math.PI);
    
    // Check if it's a horizontal swipe
    if (angle > swipeAngleThreshold && angle < (180 - swipeAngleThreshold)) return;
    
    // Swipe left to close
    if (diffX < -swipeThreshold) {
      closeDrawer();
    }
  }, { passive: true });
}

// Touch Gesture functions
function initTouchGestures() {
  // Swipe detection for swipeable elements
  const swipeables = document.querySelectorAll('.swipeable');
  swipeables.forEach(element => {
    let startX = 0;
    let startY = 0;
    let currentX = 0;
    let isSwiping = false;
    
    element.addEventListener('touchstart', function(e) {
      startX = e.touches[0].clientX;
      startY = e.touches[0].clientY;
      isSwiping = false;
    });
    
    element.addEventListener('touchmove', function(e) {
      if (!startX || !startY) return;
      
      currentX = e.touches[0].clientX;
      const currentY = e.touches[0].clientY;
      const diffX = startX - currentX;
      const diffY = startY - currentY;
      
      if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 10) {
        isSwiping = true;
        const content = element.querySelector('.swipeable-content');
        if (content) {
          const translateX = Math.max(-100, Math.min(0, -diffX));
          content.style.transform = 'translateX(' + translateX + 'px)';
        }
      }
    });
    
    element.addEventListener('touchend', function(e) {
      if (!isSwiping) return;
      
      const diffX = startX - currentX;
      const threshold = 50;
      
      if (Math.abs(diffX) > threshold) {
        if (diffX > 0) {
          element.classList.add('swiped');
          const content = element.querySelector('.swipeable-content');
          if (content) {
            content.style.transform = 'translateX(-80px)';
          }
        } else {
          element.classList.remove('swiped');
          const content = element.querySelector('.swipeable-content');
          if (content) {
            content.style.transform = 'translateX(0)';
          }
        }
      } else {
        const content = element.querySelector('.swipeable-content');
        if (content) {
          content.style.transform = 'translateX(0)';
        }
      }
      
      startX = 0;
      startY = 0;
      currentX = 0;
      isSwiping = false;
    });
  });
  
  // Touch feedback for buttons
  const touchElements = document.querySelectorAll('.btn, .nav a, .dropdown-toggle, .tab, .accordion-header');
  touchElements.forEach(element => {
    element.classList.add('touch-feedback');
    element.addEventListener('touchstart', function() {
      this.classList.add('touch-active');
    });
    element.addEventListener('touchend', function() {
      const self = this;
      setTimeout(function() {
        self.classList.remove('touch-active');
      }, 150);
    });
  });
}

// Responsive Tables functions
function initResponsiveTables() {
  if (window.innerWidth <= 768) {
    const tables = document.querySelectorAll('table');
    tables.forEach(table => {
      if (table.classList.contains('responsive-table-card')) return;
      
      const container = table.parentElement;
      if (!container || container.classList.contains('responsive-table-container')) return;
      
      const wrapper = document.createElement('div');
      wrapper.className = 'responsive-table-container';
      table.parentElement.insertBefore(wrapper, table);
      wrapper.appendChild(table);
      
      const headers = table.querySelectorAll('th');
      const headerTexts = Array.from(headers).map(function(th) {
        return th.textContent.trim();
      });
      
      table.classList.add('responsive-table-card');
      const rows = table.querySelectorAll('tbody tr');
      rows.forEach(function(row) {
        const cells = row.querySelectorAll('td');
        cells.forEach(function(cell, index) {
          if (headerTexts[index]) {
            cell.setAttribute('data-label', headerTexts[index]);
          }
        });
      });
    });
  }
}

// Handle window resize
let resizeTimeout;
window.addEventListener('resize', function() {
  clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(function() {
    initResponsiveTables();
  }, 250);
});

// Setup logout link and theme selector
document.addEventListener('DOMContentLoaded', function() {
  const logoutLink = document.getElementById('logoutBtn');
  if (logoutLink) {
    logoutLink.addEventListener('click', function(e) {
      e.preventDefault();
      removeToken();
      updateUI();
      window.location.href = '/ui/auth';
    });
  }
  
  // Setup theme selector
  const themeSelector = document.getElementById('themeSelector');
  if (themeSelector) {
    const currentTheme = getTheme();
    themeSelector.value = currentTheme;
    applyTheme(currentTheme);
    
    themeSelector.addEventListener('change', function(e) {
      const newTheme = e.target.value;
      setTheme(newTheme);
    });
  }
  
  // Initialize new UI components
  initTooltips();
  initDropdowns();
  initTabs();
  initAccordions();
  initMobileNavigation();
  initTouchGestures();
  initResponsiveTables();
});
"#
}

async fn login_page() -> Html<String> {
    let login_js = r#"
    function showAlert(message, type = 'error') {
      const alert = document.getElementById('alert');
      if (!alert) return;
      alert.className = 'alert alert-' + type;
      alert.textContent = message;
      alert.style.display = 'block';
    }
    
    function hideAlert() {
      const alert = document.getElementById('alert');
      if (alert) alert.style.display = 'none';
    }

    function apiErrMsg(p) {
      if (!p || typeof p !== 'object') return null;
      const e = p.error;
      if (typeof e === 'string') return e;
      if (e && typeof e === 'object' && typeof e.message === 'string') return e.message;
      if (typeof p.message === 'string') return p.message;
      return null;
    }
    
    async function handleLogin(event) {
      event.preventDefault();
      hideAlert();
      
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      const btn = document.getElementById('loginBtn');
      
      btn.disabled = true;
      btn.textContent = poolaiT('auth.loggingIn', 'Logging in…');
      
      try {
        const res = await fetch('/api/v1/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username, password })
        });
        
        if (!res.ok) {
          const data = await res.json();
          throw new Error(apiErrMsg(data) || poolaiT('auth.loginFailed', 'Login failed'));
        }
        
        const data = await res.json();
        setToken(data.token);
        setUser(username, data.role);
        
        // Store token expiration time
        if (data.expires_in) {
          const exp = Math.floor(Date.now() / 1000) + data.expires_in;
          localStorage.setItem('poolai_token_exp', exp.toString());
        }
        
        updateUI();
        
        window.location.href = '/ui';
      } catch (e) {
        showAlert(e.message || poolaiT('auth.loginFailed', 'Login failed'), 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = poolaiT('auth.submit', 'Login');
      }
    }
    
    async function handleOAuth2Login(provider) {
      hideAlert();
      
      try {
        // Redirect to OAuth2 provider
        window.location.href = '/api/enterprise/auth/' + provider;
      } catch (e) {
        showAlert(poolaiT('auth.oauthStartFail', 'Failed to start OAuth2 login: ') + e.message, 'error');
      }
    }
    
    // Check for OAuth2 callback (token in URL or response)
    async function checkOAuth2Callback() {
      const urlParams = new URLSearchParams(window.location.search);
      const token = urlParams.get('token');
      const error = urlParams.get('error');
      
      if (error) {
        showAlert(poolaiT('auth.oauthFail', 'OAuth2 authentication failed: ') + error, 'error');
        // Clean URL
        window.history.replaceState({}, document.title, '/ui/auth');
        return;
      }
      
      if (token) {
        try {
          // Token received from OAuth2 callback
          setToken(token);
          
          // Get user info if available
          const username = urlParams.get('username') || 'oauth2_user';
          const role = urlParams.get('role') || 'Viewer';
          
          setUser(username, role);
          
          // Store token expiration if provided
          const expiresIn = urlParams.get('expires_in');
          if (expiresIn) {
            const exp = Math.floor(Date.now() / 1000) + parseInt(expiresIn, 10);
            localStorage.setItem('poolai_token_exp', exp.toString());
          } else {
            // Default 1 hour
            const exp = Math.floor(Date.now() / 1000) + 3600;
            localStorage.setItem('poolai_token_exp', exp.toString());
          }
          
          updateUI();
          
          // Clean URL and redirect
          window.history.replaceState({}, document.title, '/ui');
          window.location.href = '/ui';
        } catch (e) {
          showAlert(poolaiT('auth.oauthTokenFail', 'Failed to process OAuth2 token: ') + e.message, 'error');
        }
      }
    }
    
    // Load available OAuth2 providers and show buttons
    async function loadOAuth2Providers() {
      try {
        const res = await fetch('/api/enterprise/security/oauth2/providers');
        if (!res.ok) return; // OAuth2 not available or not configured
        
        const providers = await res.json();
        const container = document.getElementById('oauth2-providers');
        if (!container) return;
        
        const availableProviders = Array.isArray(providers) 
          ? providers.filter(p => p.enabled && (p.name === 'github' || p.name === 'google' || p.name === 'telegram'))
          : [];
        
        if (availableProviders.length === 0) {
          container.style.display = 'none';
          return;
        }
        
        const oauthOr = poolaiT('auth.oauthOr', 'Or sign in with:');
        let buttonsHtml = '<div style="text-align: center; margin-top: 20px; padding-top: 20px; border-top: 1px solid var(--border, #262b36);"><div style="margin-bottom: 12px; color: var(--text-muted, #a8b0bf); font-size: 0.9em;">' + oauthOr + '</div><div style="display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;">';
        
        availableProviders.forEach(provider => {
          const providerName = provider.name.toLowerCase();
          let icon = '';
          let label = providerName.charAt(0).toUpperCase() + providerName.slice(1);
          
          if (providerName === 'github') {
            icon = '🔷';
          } else if (providerName === 'google') {
            icon = '🔴';
          } else if (providerName === 'telegram') {
            icon = '✈️';
          }
          
          buttonsHtml += `<button type="button" class="btn oauth2-btn" style="min-width: 120px;" onclick="handleOAuth2Login('${providerName}')" aria-label="Sign in with ${label}">${icon} ${label}</button>`;
        });
        
        buttonsHtml += '</div></div>';
        container.innerHTML = buttonsHtml;
        container.style.display = 'block';
      } catch (e) {
        // OAuth2 providers endpoint not available or failed
        const container = document.getElementById('oauth2-providers');
        if (container) container.style.display = 'none';
      }
    }
    
    function logout() {
      removeToken();
      updateUI();
      window.location.href = '/ui/auth';
    }
    "#;

    let i18n_js = include_str!("i18n_core.js");
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title data-i18n="auth.pageTitle">Login - PoolAI</title>
  <style>{css}</style>
</head>
<body>
  <div class="wrap">
    <div class="topbar">
      <div class="brand">
        <h1>PoolAI</h1>
      </div>
      <div id="poolai-lang-toggle-auth" class="poolai-lang-auth"></div>
    </div>
    <div class="content">
      <div class="card" style="max-width: 400px; margin: 40px auto;">
        <h2 style="margin:0 0 20px" data-i18n="auth.cardTitle">Login</h2>
        <div id="alert"></div>
        <form id="loginForm">
          <div class="form-group">
            <label for="username" data-i18n="auth.username">Username</label>
            <input type="text" id="username" name="username" required autocomplete="username" />
          </div>
          <div class="form-group">
            <label for="password" data-i18n="auth.password">Password</label>
            <input type="password" id="password" name="password" required autocomplete="current-password" />
          </div>
          <button type="submit" class="btn" id="loginBtn" data-i18n="auth.submit">Login</button>
        </form>
        <div id="oauth2-providers" style="display: none;"></div>
        <div style="margin-top: 20px; font-size: 0.9em; color:#a8b0bf;">
          <div><strong data-i18n="auth.testAccounts">Test accounts:</strong></div>
          <div data-i18n="auth.testAdmin">Admin: admin / admin123</div>
          <div data-i18n="auth.testOperator">Operator: operator / op123</div>
          <div data-i18n="auth.testViewer">Viewer: viewer / view123</div>
        </div>
      </div>
    </div>
  </div>
  <script>
    {i18n_js}
    {common_js}
    {login_js}
    (function() {{
      if (getUser()) {{
        window.location.href = '/ui';
        return;
      }}
      checkOAuth2Callback();
      if (typeof PoolAiI18n !== 'undefined') {{
        PoolAiI18n.apply(document.documentElement);
        PoolAiI18n.initAuthPage();
      }}
      loadOAuth2Providers();
      document.addEventListener('poolai:langchange', function() {{ loadOAuth2Providers(); }});
      var _lf = document.getElementById('loginForm');
      if (_lf) _lf.addEventListener('submit', handleLogin);
    }})();
  </script>
</body>
</html>"#,
        css = BASE_CSS,
        i18n_js = i18n_js,
        common_js = common_js(),
        login_js = login_js
    );
    Html(html)
}

async fn home_handler() -> Html<String> {
    let script = format!(
        r#"{}
// Protected route check for home
(async function() {{
  await requireAuth();
  setUpdated();
}})();
"#,
        common_js()
    );
    layout(
        "dash.title.home",
        "Home",
        r#"
<div class="grid">
  <div class="item">
    <div><b data-i18n="home.apiTitle">API</b></div>
    <div class="muted"><span data-i18n="home.apiBase">Base:</span> <code>/api/v1</code></div>
    <div style="margin-top:8px"><a href="/api/v1/status">/api/v1/status</a></div>
  </div>
  <div class="item">
    <div><b data-i18n="home.uiTitle">UI</b></div>
    <div class="muted"><span data-i18n="home.uiHint">Pages under</span> <code>/ui</code></div>
    <div style="margin-top:8px"><a href="/ui/status" data-i18n="home.openDashboard">Open read-only dashboard</a></div>
  </div>
</div>

<div class="grid">
  <div class="item"><b data-i18n="home.quickLinks">Quick links</b><div style="margin-top:8px">
    <a href="/ui/metrics" data-i18n="dash.nav.metrics">Metrics</a> ·
    <a href="/ui/workers" data-i18n="dash.nav.workers">Workers</a> ·
    <a href="/ui/libs" data-i18n="dash.nav.libs">Libs</a> ·
    <a href="/ui/vm" data-i18n="dash.nav.vm">VM</a> ·
    <a href="/ui/raid" data-i18n="dash.nav.raid">RAID</a>
  </div></div>
  <div class="item">
    <div><b data-i18n="home.notesTitle">Notes</b></div>
          <div class="muted" data-i18n="home.notesBody">Write operations are available for authenticated users with appropriate permissions.</div>
  </div>
</div>
"#,
        &script,
    )
}

async fn status_page() -> Html<String> {
    layout(
        "dash.title.status",
        "Status",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/status', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn health_page() -> Html<String> {
    layout(
        "dash.title.health",
        "Health",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/health', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn metrics_page() -> Html<String> {
    layout(
        "dash.title.metrics",
        "Metrics",
        r#"<div id="data"></div>"#,
        &format!(
            "{}\nasync function refresh(){{ await poll('/api/v1/metrics', renderJsonPre, 'data'); }}\nrefresh(); setInterval(refresh, 5000);",
            common_js()
        ),
    )
}

async fn workers_page() -> Html<String> {
    let workers_js = r#"
    window.refreshWorkers = async function() {
      await poll('/api/v1/workers', renderWorkersTable, 'data');
    };
    
    function renderWorkersTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No workers available.</div>';
        return;
      }
      
      const table = document.createElement('table');
      table.setAttribute('role', 'table');
      table.setAttribute('aria-label', 'Workers list');
      table.setAttribute('aria-describedby', 'workers-table-desc');
      const tableDesc = document.createElement('div');
      tableDesc.id = 'workers-table-desc';
      tableDesc.className = 'sr-only';
      tableDesc.textContent = 'Table showing workers: id, health, state, task, metrics, actions';
      tableDesc.style.cssText = 'position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;';
      
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      hr.setAttribute('role', 'row');
      ['ID', 'Health', 'State', 'Current task', 'Requests', 'Queue', 'Actions'].forEach(label => {
        const th = document.createElement('th');
        th.setAttribute('role', 'columnheader');
        th.setAttribute('scope', 'col');
        th.textContent = label;
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      tbody.setAttribute('role', 'rowgroup');
      for (const worker of data) {
        const tr = document.createElement('tr');
        tr.setAttribute('role', 'row');
        const workerId = worker ? worker.id : 'unknown';
        tr.setAttribute('aria-label', 'Worker ' + workerId);
        
        const tdId = document.createElement('td');
        tdId.setAttribute('role', 'cell');
        tdId.textContent = String(workerId);
        tr.appendChild(tdId);
        
        const healthy = worker && typeof worker.is_healthy === 'boolean'
          ? worker.is_healthy
          : (worker && worker.status !== 'error');
        const tdHealth = document.createElement('td');
        tdHealth.setAttribute('role', 'cell');
        const healthBadge = document.createElement('span');
        healthBadge.className = 'status-badge ' + (healthy ? 'active' : 'error');
        healthBadge.textContent = healthy ? 'Healthy' : 'Unhealthy';
        tdHealth.appendChild(healthBadge);
        tr.appendChild(tdHealth);
        
        const tdState = document.createElement('td');
        tdState.setAttribute('role', 'cell');
        tdState.textContent = worker && worker.status ? String(worker.status) : '—';
        tr.appendChild(tdState);
        
        const tdTask = document.createElement('td');
        tdTask.setAttribute('role', 'cell');
        tdTask.textContent = (worker && worker.current_task) ? String(worker.current_task) : '—';
        tr.appendChild(tdTask);
        
        const tdReq = document.createElement('td');
        tdReq.setAttribute('role', 'cell');
        tdReq.textContent = worker && typeof worker.total_requests_processed === 'number'
          ? String(worker.total_requests_processed)
          : '0';
        tr.appendChild(tdReq);
        
        const tdQueue = document.createElement('td');
        tdQueue.setAttribute('role', 'cell');
        tdQueue.textContent = worker && typeof worker.queue_size === 'number'
          ? String(worker.queue_size)
          : '0';
        tr.appendChild(tdQueue);
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.setAttribute('role', 'cell');
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const workerId = worker ? worker.id : 'unknown';
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Delete button
          const deleteBtn = document.createElement('button');
          deleteBtn.className = 'btn btn-danger';
          deleteBtn.textContent = 'Delete';
          deleteBtn.setAttribute('type', 'button');
          deleteBtn.setAttribute('aria-label', `Delete worker ${workerId}`);
          deleteBtn.setAttribute('aria-describedby', `worker-${workerId}-desc`);
          deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          deleteBtn.onclick = () => handleWorkerDelete(workerId);
          deleteBtn.onkeydown = (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              handleWorkerDelete(workerId);
            }
          };
          actionsTd.appendChild(deleteBtn);
          
          // Hidden description for screen readers
          const desc = document.createElement('span');
          desc.id = `worker-${workerId}-desc`;
          desc.className = 'sr-only';
          desc.textContent = `Permanently delete worker ${workerId}`;
          desc.style.cssText = 'position: absolute; left: -10000px; width: 1px; height: 1px; overflow: hidden;';
          actionsTd.appendChild(desc);
        } else {
          actionsTd.setAttribute('aria-label', 'No actions available for your role');
          actionsTd.textContent = '—';
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
    }
    
    async function showCreateWorkerModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createWorkerModal');
    }
    
    async function handleCreateWorker(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createWorkerForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const payload = {
          worker_id: document.getElementById('workerId').value,
          max_concurrent_requests: parseInt(document.getElementById('workerMaxConcurrent').value, 10) || 10,
          request_timeout_ms: parseInt(document.getElementById('workerTimeout').value, 10) || 5000,
          health_check_interval_ms: parseInt(document.getElementById('workerHealthInterval').value, 10) || 1000,
          enable_caching: document.getElementById('workerEnableCache').checked,
          cache_size: parseInt(document.getElementById('workerCacheSize').value, 10) || 1000,
          max_memory_mb: parseInt(document.getElementById('workerMaxMemory').value, 10) || 2048,
          cpu_priority: parseInt(document.getElementById('workerCpuPriority').value, 10) || 5,
          gpu_device: document.getElementById('workerGpuDevice').value ? parseInt(document.getElementById('workerGpuDevice').value, 10) : null,
          auto_restart: document.getElementById('workerAutoRestart').checked,
          resource_monitoring: document.getElementById('workerResourceMonitoring').checked
        };
        
        const result = await fetchJson('/api/v1/workers', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('Worker created successfully', 'success');
        hideModal('createWorkerModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshWorkers();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleWorkerDelete(workerId) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to delete worker "${workerId}"? This action cannot be undone.`,
        async () => {
          try {
            showLoading('data', 'Deleting worker...');
            
            await fetchJson(`/api/v1/workers/${workerId}`, {
              method: 'DELETE'
            });
            
            showNotification('Worker deleted successfully', 'success');
            setTimeout(() => {
              window.refreshWorkers();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('data');
          }
        }
      );
    }
    
    async function refresh() {
      await window.refreshWorkers();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "dash.title.workers",
        "Workers",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/workers</code></div>
  <button class="btn btn-primary" onclick="showCreateWorkerModal()" aria-label="Create new worker">Create Worker</button>
</div>
<div id="data"></div>

<!-- Create Worker Modal -->
<div id="createWorkerModal" class="modal" role="dialog" aria-labelledby="createWorkerModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createWorkerModalTitle">Create Worker</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createWorkerModal')">&times;</button>
    </div>
    <form id="createWorkerForm" onsubmit="handleCreateWorker(event)">
      <div class="form-group">
        <label for="workerId">Worker ID</label>
        <input type="text" id="workerId" name="worker_id" required placeholder="worker-1" />
      </div>
      <div class="form-group">
        <label for="workerMaxConcurrent">Max Concurrent Requests</label>
        <input type="number" id="workerMaxConcurrent" name="max_concurrent_requests" min="1" max="100" value="10" />
      </div>
      <div class="form-group">
        <label for="workerTimeout">Request Timeout (ms)</label>
        <input type="number" id="workerTimeout" name="request_timeout_ms" min="1000" max="60000" value="5000" />
      </div>
      <div class="form-group">
        <label for="workerHealthInterval">Health Check Interval (ms)</label>
        <input type="number" id="workerHealthInterval" name="health_check_interval_ms" min="100" max="10000" value="1000" />
      </div>
      <div class="form-group">
        <label for="workerMaxMemory">Max Memory (MB)</label>
        <input type="number" id="workerMaxMemory" name="max_memory_mb" min="256" max="131072" value="2048" />
      </div>
      <div class="form-group">
        <label for="workerCpuPriority">CPU Priority (1-10)</label>
        <input type="number" id="workerCpuPriority" name="cpu_priority" min="1" max="10" value="5" />
      </div>
      <div class="form-group">
        <label for="workerGpuDevice">GPU Device ID (optional)</label>
        <input type="number" id="workerGpuDevice" name="gpu_device" min="0" placeholder="Leave empty for no GPU" />
      </div>
      <div class="form-group">
        <label for="workerEnableCache">
          <input type="checkbox" id="workerEnableCache" name="enable_caching" checked />
          Enable Caching
        </label>
      </div>
      <div class="form-group">
        <label for="workerCacheSize">Cache Size</label>
        <input type="number" id="workerCacheSize" name="cache_size" min="100" max="10000" value="1000" />
      </div>
      <div class="form-group">
        <label for="workerAutoRestart">
          <input type="checkbox" id="workerAutoRestart" name="auto_restart" checked />
          Auto Restart on Failure
        </label>
      </div>
      <div class="form-group">
        <label for="workerResourceMonitoring">
          <input type="checkbox" id="workerResourceMonitoring" name="resource_monitoring" checked />
          Resource Monitoring
        </label>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createWorkerModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), workers_js),
    )
}

async fn libs_page() -> Html<String> {
    let libs_js = r#"
    window.refreshLibraries = async function() {
      await poll('/api/v1/libraries', renderLibrariesTable, 'data');
    };
    
    function renderLibrariesTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No libraries installed.</div>';
        return;
      }
      
      const table = document.createElement('table');
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      ['name', 'version', 'type', 'status', 'actions'].forEach(k => {
        const th = document.createElement('th');
        th.textContent = k.charAt(0).toUpperCase() + k.slice(1);
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      for (const lib of data) {
        const tr = document.createElement('tr');
        
        ['name', 'version', 'type', 'status'].forEach(k => {
          const td = document.createElement('td');
          const v = lib ? lib[k] : null;
          td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
          tr.appendChild(td);
        });
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const libName = lib.name;
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Update button
          const updateBtn = document.createElement('button');
          updateBtn.className = 'btn';
          updateBtn.textContent = 'Update';
          updateBtn.setAttribute('aria-label', `Update library ${libName}`);
          updateBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          updateBtn.onclick = () => handleLibraryAction(libName, 'update');
          actionsTd.appendChild(updateBtn);
          
          // Uninstall button
          const uninstallBtn = document.createElement('button');
          uninstallBtn.className = 'btn btn-danger';
          uninstallBtn.textContent = 'Uninstall';
          uninstallBtn.setAttribute('aria-label', `Uninstall library ${libName}`);
          uninstallBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          uninstallBtn.onclick = () => handleLibraryUninstall(libName);
          actionsTd.appendChild(uninstallBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
    }
    
    async function showInstallLibraryModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('installLibraryModal');
    }
    
    async function handleInstallLibrary(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('installLibraryForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Installing...';
      
      try {
        const libName = document.getElementById('libName').value;
        const version = document.getElementById('libVersion').value || 'latest';
        
        const result = await fetchJson(`/api/v1/libraries/${libName}/install`, {
          method: 'POST',
          body: JSON.stringify({ version })
        });
        
        showNotification('Library installed successfully', 'success');
        hideModal('installLibraryModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshLibraries();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleLibraryAction(libName, action) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      try {
        showLoading('data', `${action.charAt(0).toUpperCase() + action.slice(1)}ing library...`);
        
        const result = await fetchJson(`/api/v1/libraries/${libName}/${action}`, {
          method: 'POST'
        });
        
        showNotification(result.message || `Library ${action}d successfully`, 'success');
        setTimeout(() => {
          window.refreshLibraries();
        }, 1000);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        hideLoading('data');
      }
    }
    
    async function handleLibraryUninstall(libName) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to uninstall library "${libName}"? This action cannot be undone.`,
        async () => {
          try {
            showLoading('data', 'Uninstalling library...');
            
            await fetchJson(`/api/v1/libraries/${libName}/uninstall`, {
              method: 'POST'
            });
            
            showNotification('Library uninstalled successfully', 'success');
            setTimeout(() => {
              window.refreshLibraries();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('data');
          }
        }
      );
    }
    
    async function refresh() {
      await window.refreshLibraries();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "dash.title.libraries",
        "Libraries",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/libraries</code></div>
  <button class="btn btn-primary" onclick="showInstallLibraryModal()">Install Library</button>
</div>
<div id="data"></div>

<!-- Install Library Modal -->
<div id="installLibraryModal" class="modal">
  <div class="modal-content">
    <div class="modal-header">
      <h3>Install Library</h3>
      <button class="modal-close" onclick="hideModal('installLibraryModal')">&times;</button>
    </div>
    <form id="installLibraryForm" onsubmit="handleInstallLibrary(event)">
      <div class="form-group">
        <label for="libName">Library Name</label>
        <input type="text" id="libName" name="name" required placeholder="libtorch" />
      </div>
      <div class="form-group">
        <label for="libVersion">Version (optional, defaults to latest)</label>
        <input type="text" id="libVersion" name="version" placeholder="1.13.0" />
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('installLibraryModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Install</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), libs_js),
    )
}

async fn vm_page() -> Html<String> {
    let vm_js = r#"
    window.refreshVmInstances = async function() {
      await poll('/api/v1/vm/instances', renderTable, 'data');
    };
    
    async function showCreateVmModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createVmModal');
    }
    
    async function handleCreateVm(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createVmForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const payload = {
          name: document.getElementById('vmName').value,
          resources: {
            cpu_cores: parseInt(document.getElementById('vmCpuCores').value, 10),
            memory_mb: parseInt(document.getElementById('vmMemoryMb').value, 10),
            gpu_required: document.getElementById('vmGpuRequired').checked
          },
          isolation: document.getElementById('vmIsolation').value
        };
        
        const result = await fetchJson('/api/v1/vm/instances', {
          method: 'POST',
          body: JSON.stringify(payload)
        });
        
        showNotification('VM instance created successfully', 'success');
        hideModal('createVmModal');
        form.reset();
        
        setTimeout(() => {
          window.refreshVmInstances();
        }, 500);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
      } finally {
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function refresh() {
      await window.refreshVmInstances();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "dash.title.vm",
        "VM Instances",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Source: <code>/api/v1/vm/instances</code></div>
  <button class="btn btn-primary" onclick="showCreateVmModal()" aria-label="Create new VM instance">Create VM Instance</button>
</div>
<div id="data"></div>

<!-- Create VM Modal -->
<div id="createVmModal" class="modal" role="dialog" aria-labelledby="createVmModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createVmModalTitle">Create VM Instance</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createVmModal')">&times;</button>
    </div>
    <form id="createVmForm" onsubmit="handleCreateVm(event)">
      <div class="form-group">
        <label for="vmName">Instance Name</label>
        <input type="text" id="vmName" name="name" required placeholder="my-vm-instance" />
      </div>
      <div class="form-group">
        <label for="vmCpuCores">CPU Cores</label>
        <input type="number" id="vmCpuCores" name="cpu_cores" required min="1" max="64" value="2" />
      </div>
      <div class="form-group">
        <label for="vmMemoryMb">Memory (MB)</label>
        <input type="number" id="vmMemoryMb" name="memory_mb" required min="256" max="131072" value="2048" />
      </div>
      <div class="form-group">
        <label for="vmGpuRequired">
          <input type="checkbox" id="vmGpuRequired" name="gpu_required" />
          GPU Required
        </label>
      </div>
      <div class="form-group">
        <label for="vmIsolation">Isolation Type</label>
        <select id="vmIsolation" name="isolation" required>
          <option value="ProcessSandbox">Process Sandbox</option>
          <option value="HardwareVm">Hardware VM</option>
        </select>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createVmModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), vm_js),
    )
}

async fn raid_page() -> Html<String> {
    let raid_js = r#"
    window.refreshRaidArtifacts = async function() {
      await poll('/api/v1/raid/artifacts', renderRaidArtifactsTable, 'artifacts');
    };
    
    function renderRaidArtifactsTable(containerId, data) {
      const el = document.getElementById(containerId);
      if (!el) return;
      el.innerHTML = '';
      
      if (!Array.isArray(data)) {
        renderJsonPre(containerId, data);
        return;
      }
      
      if (data.length === 0) {
        el.innerHTML = '<div class="muted">No artifacts stored.</div>';
        return;
      }
      
      const table = document.createElement('table');
      const thead = document.createElement('thead');
      const hr = document.createElement('tr');
      ['id', 'name', 'stored_at', 'actions'].forEach(k => {
        const th = document.createElement('th');
        th.textContent = k.charAt(0).toUpperCase() + k.slice(1).replace('_', ' ');
        hr.appendChild(th);
      });
      thead.appendChild(hr);
      table.appendChild(thead);
      
      const tbody = document.createElement('tbody');
      for (const artifact of data) {
        const tr = document.createElement('tr');
        
        ['id', 'name', 'stored_at'].forEach(k => {
          const td = document.createElement('td');
          let v = artifact ? artifact[k] : null;
          if (k === 'stored_at' && v) {
            try {
              const date = new Date(v);
              v = date.toLocaleString();
            } catch (e) {
              // Keep original value
            }
          }
          td.textContent = (typeof v === 'object') ? JSON.stringify(v) : String(v ?? '');
          tr.appendChild(td);
        });
        
        // Action buttons
        const actionsTd = document.createElement('td');
        actionsTd.className = 'action-buttons';
        actionsTd.style.cssText = 'white-space: nowrap;';
        
        const artifactId = artifact.id;
        const artifactName = artifact.name || artifactId;
        const user = getUser();
        const canWrite = user && (user.role === 'Admin' || user.role === 'Operator');
        
        if (canWrite) {
          // Delete button
          const deleteBtn = document.createElement('button');
          deleteBtn.className = 'btn btn-danger';
          deleteBtn.textContent = 'Delete';
          deleteBtn.setAttribute('aria-label', `Delete artifact ${artifactName}`);
          deleteBtn.style.cssText = 'padding: 4px 8px; font-size: 0.85em;';
          deleteBtn.onclick = () => handleArtifactDelete(artifactId, artifactName);
          actionsTd.appendChild(deleteBtn);
        }
        
        tr.appendChild(actionsTd);
        tbody.appendChild(tr);
      }
      table.appendChild(tbody);
      
      // Обгортаємо таблицю в container для автоматичного вирівнювання
      const container = document.createElement('div');
      container.className = 'table-container';
      container.appendChild(table);
      el.appendChild(container);
    }
    
    async function showCreateArtifactModal() {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      showModal('createArtifactModal');
    }
    
    async function handleCreateArtifact(event) {
      event.preventDefault();
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions.', 'error');
        return;
      }
      
      if (!validateForm('createArtifactForm')) {
        showNotification('Please fill in all required fields correctly.', 'error');
        return;
      }
      
      const form = event.target;
      const btn = form.querySelector('button[type="submit"]');
      const originalText = btn.textContent;
      
      btn.disabled = true;
      btn.textContent = 'Creating...';
      
      try {
        const name = document.getElementById('artifactName').value;
        const fileInput = document.getElementById('artifactFile');
        
        if (!fileInput.files || fileInput.files.length === 0) {
          showNotification('Please select a file to upload.', 'error');
          btn.disabled = false;
          btn.textContent = originalText;
          return;
        }
        
        const file = fileInput.files[0];
        const reader = new FileReader();
        
        reader.onload = async function(e) {
          try {
            const arrayBuffer = e.target.result;
            const base64 = btoa(String.fromCharCode(...new Uint8Array(arrayBuffer)));
            
            const result = await fetchJson('/api/v1/raid/artifacts', {
              method: 'POST',
              body: JSON.stringify({
                name: name,
                data: base64
              })
            });
            
            showNotification('Artifact created successfully', 'success');
            hideModal('createArtifactModal');
            form.reset();
            
            setTimeout(() => {
              window.refreshRaidArtifacts();
            }, 500);
          } catch (err) {
            showNotification('Error: ' + err.message, 'error');
          } finally {
            btn.disabled = false;
            btn.textContent = originalText;
          }
        };
        
        reader.onerror = function() {
          showNotification('Error reading file', 'error');
          btn.disabled = false;
          btn.textContent = originalText;
        };
        
        reader.readAsArrayBuffer(file);
      } catch (e) {
        showNotification('Error: ' + e.message, 'error');
        btn.disabled = false;
        btn.textContent = originalText;
      }
    }
    
    async function handleArtifactDelete(artifactId, artifactName) {
      const user = getUser();
      if (!user || (user.role !== 'Admin' && user.role !== 'Operator')) {
        showNotification('Insufficient permissions. Admin or Operator role required.', 'error');
        return;
      }
      
      showConfirmDialog(
        `Are you sure you want to delete artifact "${artifactName}" (${artifactId})? This action cannot be undone.`,
        async () => {
          try {
            showLoading('artifacts', 'Deleting artifact...');
            
            await fetchJson(`/api/v1/raid/artifacts/${artifactId}`, {
              method: 'DELETE'
            });
            
            showNotification('Artifact deleted successfully', 'success');
            setTimeout(() => {
              window.refreshRaidArtifacts();
            }, 1000);
          } catch (e) {
            showNotification('Error: ' + e.message, 'error');
            hideLoading('artifacts');
          }
        }
      );
    }
    
    async function refresh() {
      await poll('/api/v1/raid/nodes', renderTable, 'nodes');
      await window.refreshRaidArtifacts();
    }
    
    refresh();
    setInterval(refresh, 5000);
    "#;

    layout(
        "dash.title.raid",
        "RAID",
        r#"
<div class="row" style="margin-bottom:16px;">
  <div class="muted">Artifacts: <code>/api/v1/raid/artifacts</code></div>
  <button class="btn btn-primary" onclick="showCreateArtifactModal()" aria-label="Create new artifact">Create Artifact</button>
</div>

<div class="grid">
  <div class="item">
    <div class="muted">Nodes: <code>/api/v1/raid/nodes</code></div>
    <div id="nodes"></div>
  </div>
  <div class="item">
    <div class="muted">Artifacts: <code>/api/v1/raid/artifacts</code></div>
    <div id="artifacts"></div>
  </div>
</div>

<!-- Create Artifact Modal -->
<div id="createArtifactModal" class="modal" role="dialog" aria-labelledby="createArtifactModalTitle" aria-modal="true" aria-hidden="true">
  <div class="modal-content">
    <div class="modal-header">
      <h3 id="createArtifactModalTitle">Create Artifact</h3>
      <button class="modal-close" aria-label="Close dialog" onclick="hideModal('createArtifactModal')">&times;</button>
    </div>
    <form id="createArtifactForm" onsubmit="handleCreateArtifact(event)">
      <div class="form-group">
        <label for="artifactName">Artifact Name</label>
        <input type="text" id="artifactName" name="name" required placeholder="my-artifact" />
      </div>
      <div class="form-group">
        <label for="artifactFile">File</label>
        <input type="file" id="artifactFile" name="file" required />
      </div>
      <div class="modal-footer">
        <button type="button" class="btn" onclick="hideModal('createArtifactModal')">Cancel</button>
        <button type="submit" class="btn btn-primary">Create</button>
      </div>
    </form>
  </div>
</div>
"#,
        &format!("{}\n{}", common_js(), raid_js),
    )
}

pub async fn initialize() -> Result<(), AppError> {
    UiManager::new().initialize().await
}

pub async fn shutdown() -> Result<(), AppError> {
    UiManager::new().shutdown().await
}
