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
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
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
        ACCORDION_STYLES
    )
}

