//! Monitoring depth scaffold (PH-S1549…S1558, band 91 — enterprise phase E).

use serde_json::Value;

/// Monitoring depth flags (production verify stub + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand91,
}

/// Monitoring criteria registry (PH-S1549): id · marker · doc path.
pub const MONITORING_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_depth",
        "MonitoringDepth",
        "crates/poolai-ui-core/src/monitoring_depth.rs",
    ),
    (
        "store_wire",
        "POOLAI_MONITORING_DATA_DIR",
        "src/enterprise/monitoring.rs",
    ),
    (
        "api_contracts",
        "monitoring_depth_audit",
        "tests/monitoring_depth_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING",
        "bin/verify-dev-stand.sh",
    ),
    ("quick_flag", "--monitoring", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "monitoring_band91_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--monitoring",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "monitoring_docs",
        "MONITORING_DEPTH.md",
        "docs/development/MONITORING_DEPTH.md",
    ),
];

/// `poolai-loc-audit --monitoring` case names (PH-S1554).
pub const MONITORING_CASES: &[&str] = &[
    "monitoring_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "quick_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "monitoring_docs",
];

/// FM §5.72 band-91 marker rows.
pub const FM_BAND91_ROWS: &[&str] = &[
    "5.72",
    "Monitoring depth",
    "PH-S1549…S1558",
    "monitoring_depth",
];

/// Monitoring adoption markers for band 91.
pub const MONITORING_BAND91_ROWS: &[&str] = &[
    "PH-S1549",
    "monitoring_depth",
    "PH-S1550",
    "POOLAI_MONITORING_DATA_DIR",
    "PH-S1552",
    "VERIFY_MONITORING",
    "PH-S1554",
    "--monitoring",
    "PH-S1558",
];

/// Classify monitoring band depth from optional feature stub (PH-S1549).
pub fn monitoring_depth_stub(features: Option<&Value>) -> MonitoringDepth {
    let Some(f) = features else {
        return MonitoringDepth::None;
    };
    let depth = f
        .get("monitoring_depth")
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
        .get("monitoring_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return MonitoringDepth::FullBand91;
    }
    if docs {
        return MonitoringDepth::DocsCanon;
    }
    if loc {
        return MonitoringDepth::LocAuditFlag;
    }
    if smoke {
        return MonitoringDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringDepth::VerifyDevStandHook;
    }
    if api {
        return MonitoringDepth::ApiContracts;
    }
    if store {
        return MonitoringDepth::StoreWire;
    }
    if depth {
        return MonitoringDepth::DepthModule;
    }
    MonitoringDepth::None
}

/// Total monitoring criteria in registry (PH-S1549).
pub fn monitoring_criteria_total() -> usize {
    MONITORING_CRITERIA.len()
}

/// Env key for durable monitoring store (PH-S1549 scaffold; sqlite when set).
pub const MONITORING_STORE_ENV: &str = "POOLAI_MONITORING_DATA_DIR";

/// Canonical store modes for band 91+ (memory default; sqlite when data dir set).
pub const MONITORING_STORE_MODES: &[&str] = &["memory", "sqlite"];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_depth_stub_ph_s1549() {
        assert_eq!(monitoring_depth_stub(None), MonitoringDepth::None);
        assert_eq!(
            monitoring_depth_stub(Some(&json!({"monitoring_depth": true}))),
            MonitoringDepth::DepthModule
        );
        assert_eq!(
            monitoring_depth_stub(Some(&json!({
                "monitoring_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_docs": true,
            }))),
            MonitoringDepth::FullBand91
        );
        assert_eq!(MONITORING_CRITERIA.len(), 8);
        assert_eq!(monitoring_criteria_total(), 8);
        assert_eq!(MONITORING_STORE_ENV, "POOLAI_MONITORING_DATA_DIR");
        assert!(MONITORING_STORE_MODES.contains(&"sqlite"));
        assert!(FM_BAND91_ROWS.contains(&"PH-S1549…S1558"));
    }
}
