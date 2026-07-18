//! PH-S1038: Galaxy horizon close band 39 — admin tables/forms polish (FM-019).

use poolai_ui_core::admin_tables_forms_depth::{
    admin_tables_forms_depth_stub, AdminTablesFormsDepth, ADMIN_TABLES_FORMS_BAND39_ROWS,
    FM_BAND39_ROWS,
};
use poolai_ui_core::table::{empty_state_html, form_field_html};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1029_band_tables_forms_close_ph_s1038() {
    assert_eq!(
        admin_tables_forms_depth_stub(Some(&json!({"empty_state_parity": true}))),
        AdminTablesFormsDepth::EmptyStateParity
    );
    assert_eq!(
        admin_tables_forms_depth_stub(Some(&json!({
            "empty_state_parity": true,
            "security_tables_polish": true,
            "tenants_jobs_tables": true,
            "instances_topology_tables": true,
            "grid_panel_tables": true,
            "raid_artifacts_table": true,
            "modal_form_a11y": true,
            "config_dashboard_forms": true
        }))),
        AdminTablesFormsDepth::FullTablesForms
    );

    let empty = empty_state_html("No tenants found", None, "📋", None);
    assert!(empty.contains("role=\"status\""));
    assert!(empty.contains("No tenants found"));

    let field = form_field_html(
        r#"{"id":"tenantName","name":"name","label":"Tenant Name","required":true}"#,
        "tenantName",
    );
    assert!(field.contains("aria-required=\"true\""));
    assert!(field.contains("tenantName"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND39_ROWS {
        assert!(fm.contains(row), "FM missing band-39 row {row}");
    }
    assert!(fm.contains("PH-S1038"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1029") || handoff.contains("band 39"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let tenants = include_str!("../src/ui/admin/tenants.rs");
    assert!(tenants.contains("adminEmptyStateHtml"));
    assert!(tenants.contains("adminInitTablesIn"));
    assert!(tenants.contains("aria-required=\"true\""));

    let security = include_str!("../src/ui/admin/security.rs");
    assert!(security.contains("admin-table-container"));
    assert!(security.contains("adminEmptyStateHtml"));

    let jobs = include_str!("../src/ui/admin/jobs.rs");
    assert!(jobs.contains("aria-label="));

    for marker in ADMIN_TABLES_FORMS_BAND39_ROWS {
        assert!(
            tenants.contains(marker)
                || security.contains(marker)
                || jobs.contains(marker)
                || fm.contains(marker),
            "band-39 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/admin_tables_forms_depth.rs").exists());
}
