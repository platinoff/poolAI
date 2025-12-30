//! UI Components Library
//!
//! Reusable UI components for the PoolAI dashboard.
//! Provides consistent styling and behavior across all pages.

/// Button component styles (uses CSS variables for theming)
pub const BUTTON_STYLES: &str = r#"
  .btn { 
    padding: 8px 16px; 
    border: 1px solid var(--border, #262b36); 
    border-radius: 8px; 
    background: var(--surface, #171b22); 
    color: var(--text, #e8e8e8); 
    cursor: pointer; 
    font-size: 0.95em; 
    transition: all 0.2s ease;
  }
  .btn:hover { 
    background: var(--surface-secondary, #1e2329); 
    border-color: var(--border, #44475a); 
  }
  .btn:disabled { 
    opacity: 0.5; 
    cursor: not-allowed; 
  }
  .btn-primary { 
    background: var(--primary, #50fa7b); 
    color: var(--bg, #0f1216); 
    border-color: var(--primary, #50fa7b); 
  }
  .btn-primary:hover { 
    background: var(--primary-hover, #67e480); 
  }
  .btn-danger { 
    background: var(--danger, #ff5555); 
    color: #fff; 
    border-color: var(--danger, #ff5555); 
  }
  .btn-danger:hover { 
    background: var(--danger-hover, #ff6e6e); 
  }
  .btn-secondary {
    background: var(--secondary, #6272a4);
    color: #fff;
    border-color: var(--secondary, #6272a4);
  }
  .btn-secondary:hover {
    background: var(--secondary-hover, #7a8bc4);
  }
  .btn-small {
    padding: 4px 8px;
    font-size: 0.85em;
  }
  .btn-large {
    padding: 12px 24px;
    font-size: 1.1em;
  }
"#;

/// Card component styles (uses CSS variables for theming)
pub const CARD_STYLES: &str = r#"
  .card { 
    background: var(--surface, #171b22); 
    border: 1px solid var(--border, #262b36); 
    border-radius: 14px; 
    padding: 16px; 
    box-shadow: 0 12px 40px rgba(0,0,0,.20); 
  }
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border, #262b36);
  }
  .card-title {
    margin: 0;
    color: var(--primary, #67e480);
    font-size: 1.2em;
  }
  .card-body {
    margin-top: 12px;
  }
  .card-footer {
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid var(--border, #262b36);
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
"#;

/// Form component styles (uses CSS variables for theming)
pub const FORM_STYLES: &str = r#"
  .form-group { 
    margin-bottom: 16px; 
  }
  .form-group label { 
    display: block; 
    margin-bottom: 6px; 
    color: var(--text, #cfe3ff); 
    font-size: 0.9em; 
  }
  .form-group input, 
  .form-group select, 
  .form-group textarea { 
    width: 100%; 
    padding: 8px 12px; 
    border: 1px solid var(--border, #262b36); 
    border-radius: 8px; 
    background: var(--bg, #0f1216); 
    color: var(--text, #e8e8e8); 
    font-size: 0.95em; 
    box-sizing: border-box;
  }
  .form-group input:focus, 
  .form-group select:focus, 
  .form-group textarea:focus { 
    outline: none; 
    border-color: var(--primary, #50fa7b); 
  }
  .form-group input:invalid,
  .form-group select:invalid,
  .form-group textarea:invalid {
    border-color: var(--danger, #ff5555);
  }
  .form-group .help-text {
    margin-top: 4px;
    font-size: 0.85em;
    color: var(--text-muted, #a8b0bf);
  }
  .form-group .error-text {
    margin-top: 4px;
    font-size: 0.85em;
    color: var(--danger, #ff5555);
  }
"#;

/// Modal component styles
pub const MODAL_STYLES: &str = r#"
  .modal { 
    display: none; 
    position: fixed; 
    top: 0; 
    left: 0; 
    right: 0; 
    bottom: 0; 
    background: rgba(0,0,0,0.7); 
    z-index: 1000; 
    align-items: center; 
    justify-content: center; 
  }
  .modal.active { 
    display: flex; 
  }
  .modal-content { 
    background: #171b22; 
    border: 1px solid #262b36; 
    border-radius: 14px; 
    padding: 24px; 
    max-width: 500px; 
    width: 90%; 
    max-height: 90vh; 
    overflow-y: auto; 
  }
  .modal-header { 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    margin-bottom: 20px; 
  }
  .modal-header h3 { 
    margin: 0; 
    color: #67e480; 
  }
  .modal-close { 
    background: none; 
    border: none; 
    color: #a8b0bf; 
    font-size: 24px; 
    cursor: pointer; 
    padding: 0; 
    width: 30px; 
    height: 30px; 
    transition: color 0.2s ease;
  }
  .modal-close:hover { 
    color: #e8e8e8; 
  }
  .modal-footer { 
    display: flex; 
    gap: 12px; 
    justify-content: flex-end; 
    margin-top: 20px; 
  }
"#;

/// Badge/Pill component styles (uses CSS variables for theming)
pub const BADGE_STYLES: &str = r#"
  .pill { 
    display: inline-block; 
    padding: 2px 8px; 
    border-radius: 999px; 
    background: var(--bg, #0f1216); 
    border: 1px solid var(--border, #262b36); 
    color: var(--text-muted, #a8b0bf); 
    font-size: 0.9em; 
  }
  .pill-success {
    background: var(--success, #50fa7b);
    color: var(--bg, #0f1216);
    border-color: var(--success, #50fa7b);
  }
  .pill-error {
    background: var(--danger, #ff5555);
    color: #fff;
    border-color: var(--danger, #ff5555);
  }
  .pill-warning {
    background: var(--warning, #f1fa8c);
    color: var(--bg, #0f1216);
    border-color: var(--warning, #f1fa8c);
  }
  .pill-info {
    background: var(--info, #8be9fd);
    color: var(--bg, #0f1216);
    border-color: var(--info, #8be9fd);
  }
"#;

/// Table component styles
pub const TABLE_STYLES: &str = r#"
  table { 
    width: 100%; 
    border-collapse: collapse; 
    margin-top: 12px; 
  }
  th, td { 
    border: 1px solid #262b36; 
    padding: 8px; 
    text-align: left; 
    vertical-align: top; 
  }
  th { 
    background: #0f1216; 
    color: #cfe3ff; 
  }
  tr:hover {
    background: #1e2329;
  }
  .action-buttons { 
    display: flex; 
    gap: 8px; 
    flex-wrap: wrap; 
    white-space: nowrap;
  }
"#;

/// Notification component styles
pub const NOTIFICATION_STYLES: &str = r#"
  @keyframes slideIn { 
    from { transform: translateX(100%); opacity: 0; } 
    to { transform: translateX(0); opacity: 1; } 
  }
  @keyframes slideOut { 
    from { transform: translateX(0); opacity: 1; } 
    to { transform: translateX(100%); opacity: 0; } 
  }
  .notification {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 10000;
    padding: 12px 20px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    font-weight: 500;
    max-width: 400px;
    word-wrap: break-word;
    animation: slideIn 0.3s ease-out;
  }
  .notification-success {
    background: #50fa7b;
    color: #0f1216;
  }
  .notification-error {
    background: #ff5555;
    color: #fff;
  }
  .notification-info {
    background: #8be9fd;
    color: #0f1216;
  }
  .notification-warning {
    background: #f1fa8c;
    color: #0f1216;
  }
"#;

/// Progress Bar component styles (uses CSS variables for theming)
pub const PROGRESS_BAR_STYLES: &str = r#"
  .progress-bar {
    width: 100%;
    height: 8px;
    background: var(--bg, #0f1216);
    border: 1px solid var(--border, #262b36);
    border-radius: 4px;
    overflow: hidden;
    position: relative;
  }
  .progress-bar-fill {
    height: 100%;
    background: var(--primary, #50fa7b);
    border-radius: 4px;
    transition: width 0.3s ease;
    position: relative;
  }
  .progress-bar-fill::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
    animation: progress-shimmer 1.5s infinite;
  }
  @keyframes progress-shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
  .progress-bar-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
    font-size: 0.9em;
    color: var(--text, #e8e8e8);
  }
  .progress-bar-label-text {
    font-weight: 500;
  }
  .progress-bar-label-value {
    color: var(--text-muted, #a8b0bf);
  }
  .progress-bar-circular {
    width: 64px;
    height: 64px;
    position: relative;
    display: inline-block;
  }
  .progress-bar-circular svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }
  .progress-bar-circular circle {
    fill: none;
    stroke-width: 4;
    stroke-linecap: round;
  }
  .progress-bar-circular-bg {
    stroke: var(--bg, #0f1216);
  }
  .progress-bar-circular-fill {
    stroke: var(--primary, #50fa7b);
    stroke-dasharray: 188.5;
    stroke-dashoffset: 188.5;
    transition: stroke-dashoffset 0.3s ease;
  }
  .progress-bar-circular-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 0.9em;
    font-weight: 500;
    color: var(--text, #e8e8e8);
  }
"#;

/// Tooltip component styles (uses CSS variables for theming)
pub const TOOLTIP_STYLES: &str = r#"
  .tooltip {
    position: relative;
    display: inline-block;
  }
  .tooltip-content {
    visibility: hidden;
    opacity: 0;
    position: absolute;
    z-index: 1000;
    padding: 6px 12px;
    background: var(--surface, #171b22);
    color: var(--text, #e8e8e8);
    border: 1px solid var(--border, #262b36);
    border-radius: 6px;
    font-size: 0.85em;
    white-space: nowrap;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    transition: opacity 0.2s ease, visibility 0.2s ease;
    pointer-events: none;
  }
  .tooltip-content::after {
    content: '';
    position: absolute;
    border: 6px solid transparent;
  }
  .tooltip:hover .tooltip-content,
  .tooltip:focus .tooltip-content {
    visibility: visible;
    opacity: 1;
  }
  .tooltip-top .tooltip-content {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-bottom: 8px;
  }
  .tooltip-top .tooltip-content::after {
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-top-color: var(--surface, #171b22);
  }
  .tooltip-bottom .tooltip-content {
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-top: 8px;
  }
  .tooltip-bottom .tooltip-content::after {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    border-bottom-color: var(--surface, #171b22);
  }
  .tooltip-left .tooltip-content {
    right: 100%;
    top: 50%;
    transform: translateY(-50%);
    margin-right: 8px;
  }
  .tooltip-left .tooltip-content::after {
    left: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-left-color: var(--surface, #171b22);
  }
  .tooltip-right .tooltip-content {
    left: 100%;
    top: 50%;
    transform: translateY(-50%);
    margin-left: 8px;
  }
  .tooltip-right .tooltip-content::after {
    right: 100%;
    top: 50%;
    transform: translateY(-50%);
    border-right-color: var(--surface, #171b22);
  }
"#;

/// Dropdown component styles (uses CSS variables for theming)
pub const DROPDOWN_STYLES: &str = r#"
  .dropdown {
    position: relative;
    display: inline-block;
  }
  .dropdown-toggle {
    padding: 8px 16px;
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    background: var(--surface, #171b22);
    color: var(--text, #e8e8e8);
    cursor: pointer;
    font-size: 0.95em;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.2s ease;
  }
  .dropdown-toggle:hover {
    background: var(--surface-secondary, #1e2329);
    border-color: var(--border, #44475a);
  }
  .dropdown-toggle:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  .dropdown-menu {
    display: none;
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 4px;
    min-width: 200px;
    background: var(--surface, #171b22);
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    z-index: 1000;
    max-height: 300px;
    overflow-y: auto;
  }
  .dropdown-menu.active {
    display: block;
  }
  .dropdown-item {
    padding: 8px 16px;
    color: var(--text, #e8e8e8);
    cursor: pointer;
    transition: background 0.2s ease;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .dropdown-item:hover {
    background: var(--surface-secondary, #1e2329);
  }
  .dropdown-item:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: -2px;
  }
  .dropdown-item.selected {
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
  }
  .dropdown-search {
    padding: 8px 16px;
    border-bottom: 1px solid var(--border, #262b36);
  }
  .dropdown-search input {
    width: 100%;
    padding: 6px 12px;
    border: 1px solid var(--border, #262b36);
    border-radius: 6px;
    background: var(--bg, #0f1216);
    color: var(--text, #e8e8e8);
    font-size: 0.9em;
  }
  .dropdown-search input:focus {
    outline: none;
    border-color: var(--primary, #50fa7b);
  }
"#;

/// Tabs component styles (uses CSS variables for theming)
pub const TABS_STYLES: &str = r#"
  .tabs {
    display: flex;
    flex-wrap: wrap;
    border-bottom: 1px solid var(--border, #262b36);
    margin-bottom: 16px;
  }
  .tab {
    padding: 12px 20px;
    border: none;
    background: transparent;
    color: var(--text-muted, #a8b0bf);
    cursor: pointer;
    font-size: 0.95em;
    border-bottom: 2px solid transparent;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 8px;
    position: relative;
  }
  .tab:hover {
    color: var(--text, #e8e8e8);
    background: var(--surface-secondary, #1e2329);
  }
  .tab:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: -2px;
  }
  .tab.active {
    color: var(--primary, #50fa7b);
    border-bottom-color: var(--primary, #50fa7b);
  }
  .tab-badge {
    padding: 2px 6px;
    border-radius: 10px;
    background: var(--bg, #0f1216);
    border: 1px solid var(--border, #262b36);
    font-size: 0.75em;
    color: var(--text-muted, #a8b0bf);
  }
  .tab.active .tab-badge {
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    border-color: var(--primary, #50fa7b);
  }
  .tab-content {
    display: none;
  }
  .tab-content.active {
    display: block;
  }
  @media (max-width: 768px) {
    .tabs {
      flex-direction: column;
    }
    .tab {
      width: 100%;
      justify-content: flex-start;
    }
  }
"#;

/// Skeleton Loader component styles (uses CSS variables for theming)
pub const SKELETON_STYLES: &str = r#"
  .skeleton {
    background: linear-gradient(90deg, var(--bg, #0f1216) 25%, var(--surface-secondary, #1e2329) 50%, var(--bg, #0f1216) 75%);
    background-size: 200% 100%;
    animation: skeleton-loading 1.5s ease-in-out infinite;
    border-radius: 4px;
  }
  @keyframes skeleton-loading {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
  .skeleton-text {
    height: 1em;
    margin-bottom: 8px;
  }
  .skeleton-text:last-child {
    margin-bottom: 0;
  }
  .skeleton-title {
    height: 1.5em;
    width: 60%;
    margin-bottom: 12px;
  }
  .skeleton-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
  }
  .skeleton-button {
    height: 36px;
    width: 120px;
    border-radius: 8px;
  }
  .skeleton-card {
    padding: 16px;
    border-radius: 14px;
    background: var(--surface, #171b22);
    border: 1px solid var(--border, #262b36);
  }
  .skeleton-table-row {
    height: 48px;
    margin-bottom: 8px;
  }
"#;

/// Spinner/Loading component styles (uses CSS variables for theming)
pub const SPINNER_STYLES: &str = r#"
  .spinner {
    display: inline-block;
    width: 20px;
    height: 20px;
    border: 3px solid var(--border, #262b36);
    border-top-color: var(--primary, #50fa7b);
    border-radius: 50%;
    animation: spinner-spin 0.8s linear infinite;
  }
  @keyframes spinner-spin {
    to { transform: rotate(360deg); }
  }
  .spinner-small {
    width: 16px;
    height: 16px;
    border-width: 2px;
  }
  .spinner-large {
    width: 32px;
    height: 32px;
    border-width: 4px;
  }
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
  }
  .loading-spinner-container {
    background: var(--surface, #171b22);
    padding: 24px;
    border-radius: 12px;
    border: 1px solid var(--border, #262b36);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }
  .loading-spinner-container .spinner {
    width: 40px;
    height: 40px;
    border-width: 4px;
  }
  .loading-text {
    color: var(--text, #e8e8e8);
    font-size: 0.95em;
  }
"#;

/// Error Boundary component styles (uses CSS variables for theming)
pub const ERROR_BOUNDARY_STYLES: &str = r#"
  .error-boundary {
    padding: 20px;
    border: 1px solid var(--danger, #ff5555);
    border-radius: 8px;
    background: var(--surface, #171b22);
    margin: 16px 0;
  }
  .error-boundary-title {
    color: var(--danger, #ff5555);
    font-size: 1.1em;
    font-weight: 600;
    margin-bottom: 8px;
  }
  .error-boundary-message {
    color: var(--text, #e8e8e8);
    font-size: 0.9em;
    margin-bottom: 12px;
  }
  .error-boundary-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }
  .error-retry {
    padding: 6px 12px;
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9em;
    transition: background 0.2s ease;
  }
  .error-retry:hover {
    background: var(--primary-hover, #67e480);
  }
"#;

/// Mobile Navigation component styles (uses CSS variables for theming)
pub const MOBILE_NAV_STYLES: &str = r#"
  .mobile-menu-toggle {
    display: none;
    background: none;
    border: none;
    color: var(--text, #e8e8e8);
    font-size: 24px;
    cursor: pointer;
    padding: 8px;
    width: 40px;
    height: 40px;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    transition: background 0.2s ease;
  }
  .mobile-menu-toggle:hover {
    background: var(--surface-secondary, #1e2329);
  }
  .mobile-menu-toggle:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: 2px;
  }
  .mobile-nav-drawer {
    position: fixed;
    top: 0;
    left: -100%;
    width: 280px;
    height: 100vh;
    background: var(--surface, #171b22);
    border-right: 1px solid var(--border, #262b36);
    z-index: 10001;
    transition: left 0.3s ease;
    overflow-y: auto;
    box-shadow: 2px 0 12px rgba(0,0,0,0.3);
  }
  .mobile-nav-drawer.active {
    left: 0;
  }
  .mobile-nav-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: none;
  }
  .mobile-nav-overlay.active {
    display: block;
  }
  .mobile-nav-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--border, #262b36);
  }
  .mobile-nav-close {
    background: none;
    border: none;
    color: var(--text, #e8e8e8);
    font-size: 24px;
    cursor: pointer;
    padding: 4px;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: background 0.2s ease;
  }
  .mobile-nav-close:hover {
    background: var(--surface-secondary, #1e2329);
  }
  .mobile-nav-content {
    padding: 16px;
  }
  .mobile-nav-item {
    display: block;
    padding: 12px 16px;
    color: var(--text, #e8e8e8);
    text-decoration: none;
    border-radius: 8px;
    margin-bottom: 8px;
    transition: background 0.2s ease;
    min-height: 44px;
    display: flex;
    align-items: center;
  }
  .mobile-nav-item:hover {
    background: var(--surface-secondary, #1e2329);
  }
  .mobile-nav-item:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: -2px;
  }
  @media (max-width: 768px) {
    .mobile-menu-toggle {
      display: flex;
    }
    .nav {
      display: none;
    }
  }
"#;

/// Responsive Layout component styles (uses CSS variables for theming)
pub const RESPONSIVE_LAYOUT_STYLES: &str = r#"
  .responsive-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 16px;
  }
  .responsive-table-container {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }
  .responsive-table {
    min-width: 100%;
  }
  @media (max-width: 768px) {
    .responsive-grid {
      grid-template-columns: 1fr;
    }
    .responsive-table-container {
      display: block;
    }
    .responsive-table-card {
      display: block;
    }
    .responsive-table-card .table-row {
      display: flex;
      flex-direction: column;
      padding: 12px;
      margin-bottom: 12px;
      border: 1px solid var(--border, #262b36);
      border-radius: 8px;
      background: var(--surface, #171b22);
    }
    .responsive-table-card .table-cell {
      padding: 6px 0;
      border: none;
    }
    .responsive-table-card .table-cell::before {
      content: attr(data-label) ': ';
      font-weight: 600;
      color: var(--text-muted, #a8b0bf);
      margin-right: 8px;
    }
  }
  .touch-target {
    min-height: 44px;
    min-width: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .touch-friendly {
    padding: 12px 16px;
    font-size: 1em;
  }
  @media (hover: none) and (pointer: coarse) {
    .btn, .nav a, .dropdown-toggle, .tab, .accordion-header {
      min-height: 44px;
      padding: 12px 16px;
    }
    .modal-content {
      width: 95%;
      max-width: none;
      margin: 10px;
    }
    .dropdown-menu {
      width: 100%;
      max-width: 100vw;
    }
  }
"#;

/// Touch Gesture component styles (uses CSS variables for theming)
pub const TOUCH_GESTURE_STYLES: &str = r#"
  .swipeable {
    position: relative;
    overflow: hidden;
    touch-action: pan-y;
  }
  .swipeable-content {
    transition: transform 0.3s ease;
  }
  .swipeable-actions {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    background: var(--danger, #ff5555);
    padding: 0 16px;
    transform: translateX(100%);
    transition: transform 0.3s ease;
  }
  .swipeable.swiped .swipeable-actions {
    transform: translateX(0);
  }
  .touch-feedback {
    -webkit-tap-highlight-color: rgba(80, 250, 123, 0.3);
    tap-highlight-color: rgba(80, 250, 123, 0.3);
  }
  .touch-active {
    opacity: 0.7;
    transform: scale(0.98);
  }
"#;

/// Form Wizard component styles (uses CSS variables for theming)
pub const FORM_WIZARD_STYLES: &str = r#"
  .wizard {
    position: relative;
  }
  .wizard-progress-bar {
    height: 4px;
    background: var(--bg, #0f1216);
    border-radius: 2px;
    margin-bottom: 24px;
    overflow: hidden;
  }
  .wizard-progress {
    height: 100%;
    background: var(--primary, #50fa7b);
    border-radius: 2px;
    transition: width 0.3s ease;
  }
  .wizard-step-indicator {
    text-align: center;
    color: var(--text-muted, #a8b0bf);
    font-size: 0.9em;
    margin-bottom: 16px;
  }
  .wizard-step {
    display: none;
  }
  .wizard-step.active {
    display: block;
  }
  .wizard-actions {
    display: flex;
    justify-content: space-between;
    margin-top: 24px;
    gap: 12px;
  }
  .wizard-nav-btn {
    padding: 8px 16px;
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    background: var(--surface, #171b22);
    color: var(--text, #e8e8e8);
    cursor: pointer;
    font-size: 0.95em;
    transition: all 0.2s ease;
  }
  .wizard-nav-btn:hover {
    background: var(--surface-secondary, #1e2329);
    border-color: var(--primary, #50fa7b);
  }
  .wizard-nav-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .wizard-nav-btn-primary {
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    border-color: var(--primary, #50fa7b);
  }
  .wizard-nav-btn-primary:hover:not(:disabled) {
    background: var(--primary-hover, #67e480);
  }
"#;

/// Search & Filter component styles (uses CSS variables for theming)
pub const SEARCH_FILTER_STYLES: &str = r#"
  .search-container {
    position: relative;
    margin-bottom: 16px;
  }
  .search-input {
    width: 100%;
    padding: 10px 40px 10px 12px;
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    background: var(--bg, #0f1216);
    color: var(--text, #e8e8e8);
    font-size: 0.95em;
    box-sizing: border-box;
  }
  .search-input:focus {
    outline: none;
    border-color: var(--primary, #50fa7b);
  }
  .search-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted, #a8b0bf);
    pointer-events: none;
  }
  .filter-container {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-bottom: 16px;
  }
  .filter-chip {
    padding: 6px 12px;
    border: 1px solid var(--border, #262b36);
    border-radius: 16px;
    background: var(--surface, #171b22);
    color: var(--text, #e8e8e8);
    font-size: 0.85em;
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  .filter-chip:hover {
    background: var(--surface-secondary, #1e2329);
    border-color: var(--primary, #50fa7b);
  }
  .filter-chip.active {
    background: var(--primary, #50fa7b);
    color: var(--bg, #0f1216);
    border-color: var(--primary, #50fa7b);
  }
  .filter-chip-remove {
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    padding: 0;
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: background 0.2s ease;
  }
  .filter-chip-remove:hover {
    background: rgba(0, 0, 0, 0.2);
  }
"#;

/// Accordion component styles (uses CSS variables for theming)
pub const ACCORDION_STYLES: &str = r#"
  .accordion {
    border: 1px solid var(--border, #262b36);
    border-radius: 8px;
    overflow: hidden;
  }
  .accordion-item {
    border-bottom: 1px solid var(--border, #262b36);
  }
  .accordion-item:last-child {
    border-bottom: none;
  }
  .accordion-header {
    padding: 12px 16px;
    background: var(--surface, #171b22);
    border: none;
    width: 100%;
    text-align: left;
    cursor: pointer;
    color: var(--text, #e8e8e8);
    font-size: 0.95em;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    transition: background 0.2s ease;
  }
  .accordion-header:hover {
    background: var(--surface-secondary, #1e2329);
  }
  .accordion-header:focus {
    outline: 2px solid var(--primary, #50fa7b);
    outline-offset: -2px;
  }
  .accordion-header-icon {
    transition: transform 0.3s ease;
    color: var(--text-muted, #a8b0bf);
  }
  .accordion-item.active .accordion-header-icon {
    transform: rotate(180deg);
  }
  .accordion-content {
    max-height: 0;
    overflow: hidden;
    transition: max-height 0.3s ease;
    background: var(--bg, #0f1216);
  }
  .accordion-item.active .accordion-content {
    max-height: 1000px;
  }
  .accordion-body {
    padding: 16px;
    color: var(--text, #e8e8e8);
  }
"#;

/// Get all component styles
pub fn get_component_styles() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        BUTTON_STYLES,
        CARD_STYLES,
        FORM_STYLES,
        MODAL_STYLES,
        BADGE_STYLES,
        TABLE_STYLES,
        NOTIFICATION_STYLES,
        PROGRESS_BAR_STYLES,
        TOOLTIP_STYLES,
        DROPDOWN_STYLES,
        TABS_STYLES,
        ACCORDION_STYLES,
        SKELETON_STYLES,
        SPINNER_STYLES,
        ERROR_BOUNDARY_STYLES,
        SEARCH_FILTER_STYLES,
        FORM_WIZARD_STYLES,
        MOBILE_NAV_STYLES,
        RESPONSIVE_LAYOUT_STYLES,
        TOUCH_GESTURE_STYLES
    )
}

