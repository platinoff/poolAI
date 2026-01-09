//! System Configuration page
//!
//! Provides system configuration interface with tabs for different settings.

use axum::response::Html;
use crate::ui::admin::admin_layout;

/// System configuration page
pub async fn admin_config() -> Html<String> {
    let script = r#"
    function showTab(tabName) {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
      loadConfigTab(tabName);
    }
    
    async function loadConfigTab(tabName) {
      const el = document.getElementById('config-content');
      if (!el) return;
      el.innerHTML = '<div class="muted">Configuration for ' + tabName + ' - to be implemented</div>';
    }
    
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => showTab(tab.dataset.tab));
    });
    
    loadConfigTab('general');
    "#;

    admin_layout(
        "System Configuration",
        r#"
        <div class="admin-section">
          <div class="admin-tabs">
            <button class="tab active" data-tab="general">General</button>
            <button class="tab" data-tab="performance">Performance</button>
            <button class="tab" data-tab="security">Security</button>
            <button class="tab" data-tab="monitoring">Monitoring</button>
          </div>
          <div id="config-content"></div>
        </div>
        "#,
        script,
    )
}
