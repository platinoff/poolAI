//! Monitoring store-wire band depth (PH-S1559…S1568, band 92 — enterprise phase E).

use serde_json::Value;

/// Monitoring store-wire depth flags (durable path wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringStoreDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand92,
}

/// Monitoring store-wire criteria registry (PH-S1559): id · marker · doc path.
pub const MONITORING_STORE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_store_depth",
        "MonitoringStoreDepth",
        "crates/poolai-ui-core/src/monitoring_store_depth.rs",
    ),
    (
        "store_wire",
        "monitoring_store_wire",
        "src/enterprise/monitoring.rs",
    ),
    (
        "api_contracts",
        "monitoring_store_wire_integration",
        "tests/monitoring_store_wire_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_STORE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "monitoring_store_band92_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--monitoring-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "monitoring_store_docs",
        "MONITORING_STORE.md",
        "docs/development/MONITORING_STORE.md",
    ),
];

/// `poolai-loc-audit --monitoring-store` case names (PH-S1564).
pub const MONITORING_STORE_CASES: &[&str] = &[
    "monitoring_store_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "monitoring_store_docs",
];

/// FM §5.73 band-92 marker rows.
pub const FM_BAND92_ROWS: &[&str] = &[
    "5.73",
    "Monitoring store wire",
    "PH-S1559…S1568",
    "monitoring_store_depth",
];

/// Monitoring store-wire adoption markers for band 92.
pub const MONITORING_BAND92_ROWS: &[&str] = &[
    "PH-S1559",
    "monitoring_store_depth",
    "PH-S1560",
    "monitoring_store_wire",
    "PH-S1562",
    "VERIFY_MONITORING_STORE",
    "PH-S1564",
    "--monitoring-store",
    "PH-S1568",
];

/// Classify monitoring store-wire band depth from optional feature stub (PH-S1559).
pub fn monitoring_store_depth_stub(features: Option<&Value>) -> MonitoringStoreDepth {
    let Some(f) = features else {
        return MonitoringStoreDepth::None;
    };
    let depth = f
        .get("monitoring_store_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let api = f
        .get("api_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("monitoring_store_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return MonitoringStoreDepth::FullBand92;
    }
    if docs {
        return MonitoringStoreDepth::DocsCanon;
    }
    if loc {
        return MonitoringStoreDepth::LocAuditFlag;
    }
    if smoke {
        return MonitoringStoreDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringStoreDepth::VerifyDevStandHook;
    }
    if api {
        return MonitoringStoreDepth::ApiContracts;
    }
    if store {
        return MonitoringStoreDepth::StoreWire;
    }
    if depth {
        return MonitoringStoreDepth::DepthModule;
    }
    MonitoringStoreDepth::None
}

/// Total monitoring store-wire criteria in registry (PH-S1559).
pub fn monitoring_store_criteria_total() -> usize {
    MONITORING_STORE_CRITERIA.len()
}

/// Env key for monitoring store backend (band 92 wire + band 91 data-dir compat).
pub const MONITORING_STORE_ENV: &str = "POOLAI_MONITORING_STORE";

/// Env key for durable monitoring data directory (band 91+ / band 92 wire).
pub const MONITORING_DATA_DIR_ENV: &str = "POOLAI_MONITORING_DATA_DIR";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_store_depth_stub_ph_s1559() {
        assert_eq!(
            monitoring_store_depth_stub(None),
            MonitoringStoreDepth::None
        );
        assert_eq!(
            monitoring_store_depth_stub(Some(&json!({"monitoring_store_depth": true}))),
            MonitoringStoreDepth::DepthModule
        );
        assert_eq!(
            monitoring_store_depth_stub(Some(&json!({
                "monitoring_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_store_docs": true,
            }))),
            MonitoringStoreDepth::FullBand92
        );
        assert_eq!(MONITORING_STORE_CRITERIA.len(), 7);
        assert_eq!(monitoring_store_criteria_total(), 7);
        assert_eq!(MONITORING_STORE_ENV, "POOLAI_MONITORING_STORE");
        assert_eq!(MONITORING_DATA_DIR_ENV, "POOLAI_MONITORING_DATA_DIR");
        assert!(FM_BAND92_ROWS.contains(&"PH-S1559…S1568"));
    }
}
