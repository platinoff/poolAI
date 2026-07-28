//! Monitoring admin/ops glue band depth (PH-S1579…S1588, band 94 — enterprise phase E).

use serde_json::Value;

/// Monitoring admin/ops glue depth flags (store strip / query refresh / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringAdminOpsDepth {
    None,
    DepthModule,
    StoreStrip,
    QueryOpsGlue,
    HtmlContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand94,
}

/// Monitoring admin/ops criteria registry (PH-S1579): id · marker · doc path.
pub const MONITORING_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_admin_ops_depth",
        "MonitoringAdminOpsDepth",
        "crates/poolai-ui-core/src/monitoring_admin_ops_depth.rs",
    ),
    (
        "store_strip",
        "monitoring-store-badge",
        "src/ui/admin/monitoring.rs",
    ),
    (
        "query_ops_glue",
        "refreshMonitoring",
        "src/ui/admin/monitoring.rs",
    ),
    (
        "html_contracts",
        "monitoring_admin_ops_integration",
        "tests/monitoring_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "monitoring_admin_ops_band94_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--monitoring-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "monitoring_admin_ops_docs",
        "MONITORING_ADMIN_OPS.md",
        "docs/development/MONITORING_ADMIN_OPS.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1579_integration",
        "tests/galaxy_horizon_s1579_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-admin-ops` case names (PH-S1585).
pub const MONITORING_ADMIN_OPS_CASES: &[&str] = &[
    "monitoring_admin_ops_depth",
    "store_strip",
    "query_ops_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "monitoring_admin_ops_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.75 band-94 marker rows.
pub const FM_BAND94_ROWS: &[&str] = &[
    "5.75",
    "Monitoring admin/ops glue",
    "PH-S1579…S1588",
    "monitoring_admin_ops_depth",
];

/// Monitoring admin/ops adoption markers for band 94.
pub const MONITORING_ADMIN_OPS_BAND94_ROWS: &[&str] = &[
    "PH-S1579",
    "monitoring_admin_ops_depth",
    "PH-S1580",
    "monitoring-store-badge",
    "PH-S1581",
    "refreshMonitoring",
    "PH-S1584",
    "VERIFY_MONITORING_ADMIN_OPS",
    "PH-S1585",
    "--monitoring-admin-ops",
    "PH-S1588",
];

/// Classify monitoring admin/ops band depth from optional feature stub (PH-S1579).
pub fn monitoring_admin_ops_depth_stub(features: Option<&Value>) -> MonitoringAdminOpsDepth {
    let Some(f) = features else {
        return MonitoringAdminOpsDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("monitoring_admin_ops_depth");
    let store = enabled("store_strip");
    let query = enabled("query_ops_glue");
    let html = enabled("html_contracts");
    let verify = enabled("verify_dev_stand_hook");
    let smoke = enabled("stand_smoke_export");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("monitoring_admin_ops_docs");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && store && query && html && verify && smoke && loc && docs && ratio && close {
        return MonitoringAdminOpsDepth::FullBand94;
    }
    if close || ratio {
        return MonitoringAdminOpsDepth::RatioHold;
    }
    if docs {
        return MonitoringAdminOpsDepth::DocsCanon;
    }
    if loc {
        return MonitoringAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return MonitoringAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return MonitoringAdminOpsDepth::HtmlContracts;
    }
    if query {
        return MonitoringAdminOpsDepth::QueryOpsGlue;
    }
    if store {
        return MonitoringAdminOpsDepth::StoreStrip;
    }
    if depth {
        return MonitoringAdminOpsDepth::DepthModule;
    }
    MonitoringAdminOpsDepth::None
}

/// Total monitoring admin/ops criteria in registry (PH-S1579).
pub fn monitoring_admin_ops_criteria_total() -> usize {
    MONITORING_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_admin_ops_depth_stub_ph_s1579() {
        assert_eq!(
            monitoring_admin_ops_depth_stub(None),
            MonitoringAdminOpsDepth::None
        );
        assert_eq!(
            monitoring_admin_ops_depth_stub(Some(&json!({"monitoring_admin_ops_depth": true}))),
            MonitoringAdminOpsDepth::DepthModule
        );
        assert_eq!(
            monitoring_admin_ops_depth_stub(Some(&json!({
                "monitoring_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringAdminOpsDepth::FullBand94
        );
        assert_eq!(MONITORING_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(monitoring_admin_ops_criteria_total(), 10);
        assert!(FM_BAND94_ROWS.contains(&"PH-S1579…S1588"));
    }
}
