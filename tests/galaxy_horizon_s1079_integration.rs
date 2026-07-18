//! PH-S1088: Galaxy horizon close band 44 — Admin wasm slim panels depth.

use poolai_ui_core::admin_wasm_slim_depth::{
    admin_wasm_slim_depth_stub, AdminWasmSlimDepth, ADMIN_WASM_SLIM_BAND44_ROWS, FM_BAND44_ROWS,
};
use poolai_ui_core::galaxy_telegram_seats::render_telegram_seats_panel_html;
use poolai_ui_core::galaxy_virtual_nodes::render_galaxy_virtual_nodes_panel_html;
use poolai_ui_core::instances::render_instances_panel_html;
use poolai_ui_core::ml::{
    render_monitoring_alerts_panel_html, render_monitoring_dashboards_panel_html,
};
use poolai_ui_core::network_profiles::render_network_profiles_panel_html;
use poolai_ui_core::stand_smoke_metrics::{
    render_grid_fee_split_metrics_strip_html, render_grid_governance_metrics_strip_html,
    render_grid_locality_metrics_strip_html, render_grid_prefetch_metrics_strip_html,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1079_band_admin_wasm_slim_close_ph_s1088() {
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"monitoring_alerts_panel": true}))),
        AdminWasmSlimDepth::MonitoringAlertsPanel
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({
            "monitoring_alerts_panel": true,
            "monitoring_dashboards_panel": true,
            "instances_panel": true,
            "telegram_seats_panel": true,
            "galaxy_virtual_nodes_panel": true,
            "network_profiles_panel": true,
            "grid_prefetch_metrics_strip": true,
            "grid_locality_metrics_strip": true,
            "grid_governance_metrics_strip": true,
            "grid_fee_split_metrics_strip": true
        }))),
        AdminWasmSlimDepth::FullAdminWasmSlimBand44
    );

    let alerts_html = render_monitoring_alerts_panel_html(
        "[]",
        "N/A",
        "Ack",
        "Active",
        "Ack",
        "Sev",
        "Metric",
        "Cur",
        "Thr",
        "Trig",
        "Status",
        "Act",
        "Alerts",
        "No alerts",
    );
    assert!(alerts_html.contains("admin-empty-state"));

    let dash_html = render_monitoring_dashboards_panel_html(
        "[]",
        "Name",
        "Desc",
        "Metrics",
        "Public",
        "Created",
        "Dash",
        "—",
        "N/A",
        "Yes",
        "No",
        "{n} metrics",
        "No dashboards",
    );
    assert!(dash_html.contains("admin-empty-state"));

    let inst_html = render_instances_panel_html(
        "[]", "ID", "Model", "St", "Str", "Nodes", "Created", "Act", "Inst", "View", "Del", "Empty",
    );
    assert!(inst_html.contains("admin-empty-state"));

    let tg_html = render_telegram_seats_panel_html(
        r#"{"seat_policy":"open","seat_limit":10,"active_seats":0,"bound_wallets":[]}"#,
        "Policy",
        "Limit",
        "Active",
        "Bound",
        "Seats",
    );
    assert!(tg_html.contains("admin-table"));

    let vn_html = render_galaxy_virtual_nodes_panel_html(
        "[]", "Peer", "Origin", "Region", "Latency", "Stale", "Nodes", "Empty",
    );
    assert!(vn_html.contains("admin-empty-state"));

    let np_html = render_network_profiles_panel_html(
        "[]", "Peer", "Region", "Latency", "BW", "Profiles", "Empty",
    );
    assert!(np_html.contains("muted"));

    let prefetch_html =
        render_grid_prefetch_metrics_strip_html(r#"{"metrics":{"pull_bytes_total":1}}"#, 0);
    assert!(prefetch_html.contains("admin-metrics-strip"));

    let locality_html =
        render_grid_locality_metrics_strip_html(r#"{"metrics":{"hot_promote_total":2}}"#, 0);
    assert!(locality_html.contains("admin-metrics-strip"));

    let gov_html = render_grid_governance_metrics_strip_html(
        r#"{"metrics":{"advisory_ack_total":1}}"#,
        r#"{"mode":"advisory"}"#,
        0,
    );
    assert!(gov_html.contains("admin-metrics-strip"));

    let fee_html =
        render_grid_fee_split_metrics_strip_html(r#"{"metrics":{"applied_total":3}}"#, 0);
    assert!(fee_html.contains("admin-metrics-strip"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND44_ROWS {
        assert!(fm.contains(row), "FM missing band-44 row {row}");
    }
    assert!(fm.contains("PH-S1088"));
    assert!(fm.contains("5.25"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1079") || handoff.contains("band 44"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let admin_js = include_str!("../src/ui/admin_charts.js");
    assert!(admin_js.contains("poolaiRenderTelegramSeatsPanel"));
    assert!(admin_js.contains("poolaiRenderGalaxyVirtualNodesPanel"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("admin_wasm_slim_depth_stub_band44_export_shape_ph_s1084"));

    for marker in ADMIN_WASM_SLIM_BAND44_ROWS {
        assert!(
            fm.contains(marker) || admin_js.contains(marker) || stand_smoke.contains(marker),
            "band-44 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/admin_wasm_slim_depth.rs").exists());
}
