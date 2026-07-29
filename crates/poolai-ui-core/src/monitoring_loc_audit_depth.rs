//! Monitoring loc-audit aggregate band depth (PH-S1599…S1608, band 96 — enterprise phase E).
//!
//! Consolidates band 91–95 `--monitoring*` loc-audit slices under one aggregate gate.

use serde_json::Value;

/// Monitoring loc-audit depth flags (aggregate / slices / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringLocAuditDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand96,
}

/// Band 91–95 loc-audit slice flags covered by aggregate (PH-S1600).
pub const MONITORING_LOC_AUDIT_SLICES: &[&str] = &[
    "--monitoring",
    "--monitoring-store",
    "--monitoring-api",
    "--monitoring-admin-ops",
    "--monitoring-stand-smoke",
];

/// Monitoring loc-audit criteria registry (PH-S1599): id · marker · doc path.
pub const MONITORING_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_loc_audit_depth",
        "MonitoringLocAuditDepth",
        "crates/poolai-ui-core/src/monitoring_loc_audit_depth.rs",
    ),
    (
        "slice_monitoring",
        "--monitoring",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_store",
        "--monitoring-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_api",
        "--monitoring-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_admin_ops",
        "--monitoring-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_stand_smoke",
        "--monitoring-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "aggregate_flag",
        "--monitoring-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "monitoring_loc_audit_docs",
        "MONITORING_LOC_AUDIT.md",
        "docs/development/MONITORING_LOC_AUDIT.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1599_integration",
        "tests/galaxy_horizon_s1599_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-loc-audit` case names (PH-S1604).
pub const MONITORING_LOC_AUDIT_CASES: &[&str] = &[
    "monitoring_loc_audit_depth",
    "slice_monitoring",
    "slice_store",
    "slice_api",
    "slice_admin_ops",
    "slice_stand_smoke",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "monitoring_loc_audit_docs",
    "band_close",
];

/// FM §5.77 band-96 marker rows.
pub const FM_BAND96_ROWS: &[&str] = &[
    "5.77",
    "Monitoring loc-audit",
    "PH-S1599…S1608",
    "monitoring_loc_audit_depth",
];

/// Monitoring loc-audit adoption markers for band 96.
pub const MONITORING_LOC_AUDIT_BAND96_ROWS: &[&str] = &[
    "PH-S1599",
    "monitoring_loc_audit_depth",
    "PH-S1600",
    "MONITORING_LOC_AUDIT_SLICES",
    "PH-S1601",
    "monitoring_loc_audit_integration",
    "PH-S1602",
    "VERIFY_MONITORING_LOC_AUDIT",
    "PH-S1604",
    "--monitoring-loc-audit",
    "PH-S1608",
];

/// Production-verify stub: report how many of the five slice flags are present (PH-S1600).
pub fn monitoring_loc_audit_slices_met(loc_audit_src: &str) -> (usize, usize) {
    let total = MONITORING_LOC_AUDIT_SLICES.len();
    let met = MONITORING_LOC_AUDIT_SLICES
        .iter()
        .filter(|flag| loc_audit_src.contains(*flag))
        .count();
    (met, total)
}

/// Classify monitoring loc-audit band depth from optional feature stub (PH-S1599).
pub fn monitoring_loc_audit_depth_stub(features: Option<&Value>) -> MonitoringLocAuditDepth {
    let Some(f) = features else {
        return MonitoringLocAuditDepth::None;
    };
    let depth = f
        .get("monitoring_loc_audit_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let slices = f
        .get("slice_aggregate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let contracts = f
        .get("criteria_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let export = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("monitoring_loc_audit_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_hold")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let close = f
        .get("band_close")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && slices && contracts && verify && export && loc && docs && ratio && close {
        return MonitoringLocAuditDepth::FullBand96;
    }
    if close || ratio {
        return MonitoringLocAuditDepth::RatioHold;
    }
    if docs {
        return MonitoringLocAuditDepth::DocsCanon;
    }
    if loc {
        return MonitoringLocAuditDepth::LocAuditFlag;
    }
    if export {
        return MonitoringLocAuditDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringLocAuditDepth::VerifyDevStandHook;
    }
    if contracts {
        return MonitoringLocAuditDepth::CriteriaContracts;
    }
    if slices {
        return MonitoringLocAuditDepth::SliceAggregate;
    }
    if depth {
        return MonitoringLocAuditDepth::DepthModule;
    }
    MonitoringLocAuditDepth::None
}

/// Total monitoring loc-audit criteria in registry (PH-S1599).
pub fn monitoring_loc_audit_criteria_total() -> usize {
    MONITORING_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn depth_stub_none_and_full_band96() {
        assert_eq!(
            monitoring_loc_audit_depth_stub(None),
            MonitoringLocAuditDepth::None
        );
        assert_eq!(
            monitoring_loc_audit_depth_stub(Some(&json!({
                "monitoring_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringLocAuditDepth::FullBand96
        );
    }

    #[test]
    fn slices_and_criteria_counts() {
        assert_eq!(MONITORING_LOC_AUDIT_SLICES.len(), 5);
        assert_eq!(monitoring_loc_audit_criteria_total(), 10);
        assert_eq!(MONITORING_LOC_AUDIT_CASES.len(), 10);
    }
}
