//! Admin tables/forms polish band depth (PH-S1029…S1038, band 39).

use serde_json::Value;

/// Band-39 admin tables/forms FM-019 adoption depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminTablesFormsDepth {
    None,
    EmptyStateParity,
    SecurityTablesPolish,
    TenantsJobsTables,
    InstancesTopologyTables,
    GridPanelTables,
    RaidArtifactsTable,
    ModalFormA11y,
    ConfigDashboardForms,
    FullTablesForms,
}

/// FM §5.20 band-39 marker rows.
pub const FM_BAND39_ROWS: &[&str] = &["5.20", "Admin tables/forms", "PH-S1029…S1038", "FM-019"];

/// RUN_LOCAL / admin adoption markers for band 39.
pub const ADMIN_TABLES_FORMS_BAND39_ROWS: &[&str] = &[
    "PH-S1029",
    "adminEmptyStateHtml",
    "adminInitTablesIn",
    "aria-required",
    "PH-S1038",
];

pub fn admin_tables_forms_depth_stub(features: Option<&Value>) -> AdminTablesFormsDepth {
    let Some(f) = features else {
        return AdminTablesFormsDepth::None;
    };
    let empty = f
        .get("empty_state_parity")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let security = f
        .get("security_tables_polish")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tenants_jobs = f
        .get("tenants_jobs_tables")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let instances_topo = f
        .get("instances_topology_tables")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let grid_panels = f
        .get("grid_panel_tables")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let raid = f
        .get("raid_artifacts_table")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let modal_forms = f
        .get("modal_form_a11y")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let config_dash = f
        .get("config_dashboard_forms")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [
        empty,
        security,
        tenants_jobs,
        instances_topo,
        grid_panels,
        raid,
        modal_forms,
        config_dash,
    ];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => AdminTablesFormsDepth::None,
        8 => AdminTablesFormsDepth::FullTablesForms,
        _ if empty && !security => AdminTablesFormsDepth::EmptyStateParity,
        _ if security => AdminTablesFormsDepth::SecurityTablesPolish,
        _ if tenants_jobs => AdminTablesFormsDepth::TenantsJobsTables,
        _ if instances_topo => AdminTablesFormsDepth::InstancesTopologyTables,
        _ if grid_panels => AdminTablesFormsDepth::GridPanelTables,
        _ if raid => AdminTablesFormsDepth::RaidArtifactsTable,
        _ if modal_forms => AdminTablesFormsDepth::ModalFormA11y,
        _ if config_dash => AdminTablesFormsDepth::ConfigDashboardForms,
        _ => AdminTablesFormsDepth::FullTablesForms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn admin_tables_forms_depth_stub_ph_s1038() {
        assert_eq!(
            admin_tables_forms_depth_stub(None),
            AdminTablesFormsDepth::None
        );
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
    }
}
