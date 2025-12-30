//! UI Components Library
//!
//! Reusable UI components for the PoolAI dashboard.
//! Provides consistent styling and behavior across all pages.

/// Button component styles
pub const BUTTON_STYLES: &str = r#"
  .btn { 
    padding: 8px 16px; 
    border: 1px solid #262b36; 
    border-radius: 8px; 
    background: #171b22; 
    color: #e8e8e8; 
    cursor: pointer; 
    font-size: 0.95em; 
    transition: all 0.2s ease;
  }
  .btn:hover { 
    background: #1e2329; 
    border-color: #44475a; 
  }
  .btn:disabled { 
    opacity: 0.5; 
    cursor: not-allowed; 
  }
  .btn-primary { 
    background: #50fa7b; 
    color: #0f1216; 
    border-color: #50fa7b; 
  }
  .btn-primary:hover { 
    background: #67e480; 
  }
  .btn-danger { 
    background: #ff5555; 
    color: #fff; 
    border-color: #ff5555; 
  }
  .btn-danger:hover { 
    background: #ff6e6e; 
  }
  .btn-secondary {
    background: #6272a4;
    color: #fff;
    border-color: #6272a4;
  }
  .btn-secondary:hover {
    background: #7a8bc4;
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

/// Card component styles
pub const CARD_STYLES: &str = r#"
  .card { 
    background: #171b22; 
    border: 1px solid #262b36; 
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
    border-bottom: 1px solid #262b36;
  }
  .card-title {
    margin: 0;
    color: #67e480;
    font-size: 1.2em;
  }
  .card-body {
    margin-top: 12px;
  }
  .card-footer {
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid #262b36;
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
"#;

/// Form component styles
pub const FORM_STYLES: &str = r#"
  .form-group { 
    margin-bottom: 16px; 
  }
  .form-group label { 
    display: block; 
    margin-bottom: 6px; 
    color: #cfe3ff; 
    font-size: 0.9em; 
  }
  .form-group input, 
  .form-group select, 
  .form-group textarea { 
    width: 100%; 
    padding: 8px 12px; 
    border: 1px solid #262b36; 
    border-radius: 8px; 
    background: #0f1216; 
    color: #e8e8e8; 
    font-size: 0.95em; 
    box-sizing: border-box;
  }
  .form-group input:focus, 
  .form-group select:focus, 
  .form-group textarea:focus { 
    outline: none; 
    border-color: #50fa7b; 
  }
  .form-group input:invalid,
  .form-group select:invalid,
  .form-group textarea:invalid {
    border-color: #ff5555;
  }
  .form-group .help-text {
    margin-top: 4px;
    font-size: 0.85em;
    color: #a8b0bf;
  }
  .form-group .error-text {
    margin-top: 4px;
    font-size: 0.85em;
    color: #ff5555;
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

/// Badge/Pill component styles
pub const BADGE_STYLES: &str = r#"
  .pill { 
    display: inline-block; 
    padding: 2px 8px; 
    border-radius: 999px; 
    background: #0f1216; 
    border: 1px solid #262b36; 
    color: #a8b0bf; 
    font-size: 0.9em; 
  }
  .pill-success {
    background: #50fa7b;
    color: #0f1216;
    border-color: #50fa7b;
  }
  .pill-error {
    background: #ff5555;
    color: #fff;
    border-color: #ff5555;
  }
  .pill-warning {
    background: #f1fa8c;
    color: #0f1216;
    border-color: #f1fa8c;
  }
  .pill-info {
    background: #8be9fd;
    color: #0f1216;
    border-color: #8be9fd;
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

/// Get all component styles
pub fn get_component_styles() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        BUTTON_STYLES,
        CARD_STYLES,
        FORM_STYLES,
        MODAL_STYLES,
        BADGE_STYLES,
        TABLE_STYLES,
        NOTIFICATION_STYLES
    )
}

