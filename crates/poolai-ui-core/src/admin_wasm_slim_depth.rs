//! Admin wasm slim panel depth classification (PH-S704…PH-S1088).

use serde_json::Value;

/// Admin wasm slim depth classification from panel feature flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminWasmSlimDepth {
    None,
    PanelRenderer,
    ChartsGlue,
    /// ML pipeline metrics panel wasm renderer (PH-S804).
    MlPipelinePanel,
    /// Payout batch admin panel wasm renderer (PH-S804).
    PayoutBatchPanel,
    /// Secret rotation admin panel wasm renderer (PH-S814).
    SecurityRotationPanel,
    /// Topology stats strip wasm renderer (PH-S814).
    TopologyStatsStrip,
    /// VM instances admin panel wasm renderer (PH-S824).
    VmPanel,
    /// Workers admin panel wasm renderer (PH-S824).
    WorkersPanel,
    /// Libraries admin panel wasm renderer (PH-S824).
    LibsPanel,
    /// Jobs store backend badge wasm renderer (PH-S852).
    JobsStoreBadge,
    /// Memory / seed inventory meta strip wasm renderer (PH-S862).
    MemorySeedMetaStrip,
    /// Grid verification checker panel wasm renderer (PH-S882).
    GridVerificationPanel,
    /// Grid verification metrics strip wasm renderer (PH-S882).
    GridVerificationMetricsStrip,
    /// Grid replication-pricing rate cap strip wasm renderer (PH-S892).
    GridReplicationPricingRateCapStrip,
    /// Grid pricing L1 freshness metadata strip wasm renderer (PH-S902).
    GridPricingFreshnessStrip,
    /// Grid trust persist + gate metrics strip wasm renderer (PH-S912).
    GridTrustPersistStrip,
    /// Monitoring active alerts panel wasm renderer (PH-S1079).
    MonitoringAlertsPanel,
    /// Monitoring dashboards panel wasm renderer (PH-S1080).
    MonitoringDashboardsPanel,
    /// Instances admin panel wasm renderer (PH-S1081).
    InstancesPanel,
    /// Telegram seats panel wasm renderer (PH-S1081).
    TelegramSeatsPanel,
    /// Galaxy virtual nodes panel wasm renderer (PH-S1082).
    GalaxyVirtualNodesPanel,
    /// Network profiles panel wasm renderer (PH-S1082).
    NetworkProfilesPanel,
    /// Grid prefetch metrics strip wasm renderer (PH-S1083).
    GridPrefetchMetricsStrip,
    /// Grid locality metrics strip wasm renderer (PH-S1083).
    GridLocalityMetricsStrip,
    /// Grid governance metrics strip wasm renderer (PH-S1084).
    GridGovernanceMetricsStrip,
    /// Grid fee-split metrics strip wasm renderer (PH-S1084).
    GridFeeSplitMetricsStrip,
    /// All band-44 wasm slim depth flags active (PH-S1088).
    FullAdminWasmSlimBand44,
}

/// FM §5.25 band-44 marker rows.
pub const FM_BAND44_ROWS: &[&str] = &[
    "5.25",
    "Admin wasm slim",
    "PH-S1079…S1088",
    "admin_wasm_slim_depth",
];

/// Admin wasm slim adoption markers for band 44.
pub const ADMIN_WASM_SLIM_BAND44_ROWS: &[&str] = &[
    "PH-S1079",
    "monitoring_alerts_panel",
    "PH-S1080",
    "monitoring_dashboards_panel",
    "PH-S1083",
    "grid_prefetch_metrics_strip",
    "PH-S1084",
    "grid_fee_split_metrics_strip",
    "PH-S1088",
];

fn flag(f: &Value, key: &str) -> bool {
    f.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

/// Classify admin wasm slim depth from optional feature stub (PH-S704).
pub fn admin_wasm_slim_depth_stub(features: Option<&Value>) -> AdminWasmSlimDepth {
    let Some(f) = features else {
        return AdminWasmSlimDepth::None;
    };

    let band44_flags = [
        flag(f, "monitoring_alerts_panel"),
        flag(f, "monitoring_dashboards_panel"),
        flag(f, "instances_panel"),
        flag(f, "telegram_seats_panel"),
        flag(f, "galaxy_virtual_nodes_panel"),
        flag(f, "network_profiles_panel"),
        flag(f, "grid_prefetch_metrics_strip"),
        flag(f, "grid_locality_metrics_strip"),
        flag(f, "grid_governance_metrics_strip"),
        flag(f, "grid_fee_split_metrics_strip"),
    ];
    let band44_count = band44_flags.iter().filter(|&&b| b).count();
    if band44_count == 10 {
        return AdminWasmSlimDepth::FullAdminWasmSlimBand44;
    }
    if flag(f, "monitoring_alerts_panel") {
        return AdminWasmSlimDepth::MonitoringAlertsPanel;
    }
    if flag(f, "monitoring_dashboards_panel") {
        return AdminWasmSlimDepth::MonitoringDashboardsPanel;
    }
    if flag(f, "instances_panel") {
        return AdminWasmSlimDepth::InstancesPanel;
    }
    if flag(f, "telegram_seats_panel") {
        return AdminWasmSlimDepth::TelegramSeatsPanel;
    }
    if flag(f, "galaxy_virtual_nodes_panel") {
        return AdminWasmSlimDepth::GalaxyVirtualNodesPanel;
    }
    if flag(f, "network_profiles_panel") {
        return AdminWasmSlimDepth::NetworkProfilesPanel;
    }
    if flag(f, "grid_prefetch_metrics_strip") {
        return AdminWasmSlimDepth::GridPrefetchMetricsStrip;
    }
    if flag(f, "grid_locality_metrics_strip") {
        return AdminWasmSlimDepth::GridLocalityMetricsStrip;
    }
    if flag(f, "grid_governance_metrics_strip") {
        return AdminWasmSlimDepth::GridGovernanceMetricsStrip;
    }
    if flag(f, "grid_fee_split_metrics_strip") {
        return AdminWasmSlimDepth::GridFeeSplitMetricsStrip;
    }
    if flag(f, "libs_panel") {
        return AdminWasmSlimDepth::LibsPanel;
    }
    if flag(f, "jobs_store_badge") {
        return AdminWasmSlimDepth::JobsStoreBadge;
    }
    if flag(f, "memory_seed_meta_strip") {
        return AdminWasmSlimDepth::MemorySeedMetaStrip;
    }
    if flag(f, "grid_verification_metrics_strip") {
        return AdminWasmSlimDepth::GridVerificationMetricsStrip;
    }
    if flag(f, "grid_replication_pricing_rate_cap_strip") {
        return AdminWasmSlimDepth::GridReplicationPricingRateCapStrip;
    }
    if flag(f, "grid_trust_persist_strip") {
        return AdminWasmSlimDepth::GridTrustPersistStrip;
    }
    if flag(f, "grid_pricing_freshness_strip") {
        return AdminWasmSlimDepth::GridPricingFreshnessStrip;
    }
    if flag(f, "grid_verification_panel") {
        return AdminWasmSlimDepth::GridVerificationPanel;
    }
    if flag(f, "workers_panel") {
        return AdminWasmSlimDepth::WorkersPanel;
    }
    if flag(f, "vm_panel") {
        return AdminWasmSlimDepth::VmPanel;
    }
    if flag(f, "charts_glue") {
        return AdminWasmSlimDepth::ChartsGlue;
    }
    if flag(f, "topology_stats_strip") {
        return AdminWasmSlimDepth::TopologyStatsStrip;
    }
    if flag(f, "security_rotation_panel") {
        return AdminWasmSlimDepth::SecurityRotationPanel;
    }
    if flag(f, "payout_batch_panel") {
        return AdminWasmSlimDepth::PayoutBatchPanel;
    }
    if flag(f, "ml_pipeline_panel") {
        return AdminWasmSlimDepth::MlPipelinePanel;
    }
    if flag(f, "panel_renderer") {
        return AdminWasmSlimDepth::PanelRenderer;
    }
    AdminWasmSlimDepth::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s704() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
            AdminWasmSlimDepth::PanelRenderer
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"charts_glue": true}))),
            AdminWasmSlimDepth::ChartsGlue
        );
        assert_eq!(admin_wasm_slim_depth_stub(None), AdminWasmSlimDepth::None);
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s804() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"ml_pipeline_panel": true}))),
            AdminWasmSlimDepth::MlPipelinePanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"payout_batch_panel": true}))),
            AdminWasmSlimDepth::PayoutBatchPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(
                &json!({"charts_glue": true, "ml_pipeline_panel": true})
            )),
            AdminWasmSlimDepth::ChartsGlue
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s814() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"security_rotation_panel": true}))),
            AdminWasmSlimDepth::SecurityRotationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"topology_stats_strip": true}))),
            AdminWasmSlimDepth::TopologyStatsStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s824() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"workers_panel": true}))),
            AdminWasmSlimDepth::WorkersPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"libs_panel": true}))),
            AdminWasmSlimDepth::LibsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true, "workers_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s852() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"jobs_store_badge": true}))),
            AdminWasmSlimDepth::JobsStoreBadge
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s862() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"memory_seed_meta_strip": true}))),
            AdminWasmSlimDepth::MemorySeedMetaStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s882() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_panel": true}))),
            AdminWasmSlimDepth::GridVerificationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_metrics_strip": true}))),
            AdminWasmSlimDepth::GridVerificationMetricsStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s892() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(
                &json!({"grid_replication_pricing_rate_cap_strip": true})
            )),
            AdminWasmSlimDepth::GridReplicationPricingRateCapStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s902() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_pricing_freshness_strip": true}))),
            AdminWasmSlimDepth::GridPricingFreshnessStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s912() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_trust_persist_strip": true}))),
            AdminWasmSlimDepth::GridTrustPersistStrip
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_ph_s1086() {
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"monitoring_alerts_panel": true}))),
            AdminWasmSlimDepth::MonitoringAlertsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"monitoring_dashboards_panel": true}))),
            AdminWasmSlimDepth::MonitoringDashboardsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"instances_panel": true}))),
            AdminWasmSlimDepth::InstancesPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"telegram_seats_panel": true}))),
            AdminWasmSlimDepth::TelegramSeatsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"galaxy_virtual_nodes_panel": true}))),
            AdminWasmSlimDepth::GalaxyVirtualNodesPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"network_profiles_panel": true}))),
            AdminWasmSlimDepth::NetworkProfilesPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_prefetch_metrics_strip": true}))),
            AdminWasmSlimDepth::GridPrefetchMetricsStrip
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_locality_metrics_strip": true}))),
            AdminWasmSlimDepth::GridLocalityMetricsStrip
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_governance_metrics_strip": true}))),
            AdminWasmSlimDepth::GridGovernanceMetricsStrip
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_fee_split_metrics_strip": true}))),
            AdminWasmSlimDepth::GridFeeSplitMetricsStrip
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
    }
}
