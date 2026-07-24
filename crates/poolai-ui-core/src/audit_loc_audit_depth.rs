//! Audit loc-audit aggregate band depth (PH-S1399…S1408, band 76 — enterprise phase C).
//!
//! Consolidates band 71–75 `--audit*` loc-audit slices under one aggregate gate.

use serde_json::Value;

/// Audit loc-audit depth flags (aggregate / slices / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLocAuditDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand76,
}

/// Band 71–75 loc-audit slice flags covered by aggregate (PH-S1400).
pub const AUDIT_LOC_AUDIT_SLICES: &[&str] = &[
    "--audit",
    "--audit-store",
    "--audit-api",
    "--audit-admin-ops",
    "--audit-stand-smoke",
];

/// Audit loc-audit criteria registry (PH-S1399): id · marker · doc path.
pub const AUDIT_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_loc_audit_depth",
        "AuditLocAuditDepth",
        "crates/poolai-ui-core/src/audit_loc_audit_depth.rs",
    ),
    ("slice_audit", "--audit", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_store",
        "--audit-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    ("slice_api", "--audit-api", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_admin_ops",
        "--audit-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_stand_smoke",
        "--audit-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "aggregate_flag",
        "--audit-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "audit_loc_audit_docs",
        "AUDIT_LOC_AUDIT.md",
        "docs/development/AUDIT_LOC_AUDIT.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1399_integration",
        "tests/galaxy_horizon_s1399_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-loc-audit` case names (PH-S1404).
pub const AUDIT_LOC_AUDIT_CASES: &[&str] = &[
    "audit_loc_audit_depth",
    "slice_audit",
    "slice_store",
    "slice_api",
    "slice_admin_ops",
    "slice_stand_smoke",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "audit_loc_audit_docs",
    "band_close",
];

/// FM §5.57 band-76 marker rows.
pub const FM_BAND76_ROWS: &[&str] = &[
    "5.57",
    "Audit loc-audit",
    "PH-S1399…S1408",
    "audit_loc_audit_depth",
];

/// Audit loc-audit adoption markers for band 76.
pub const AUDIT_LOC_AUDIT_BAND76_ROWS: &[&str] = &[
    "PH-S1399",
    "audit_loc_audit_depth",
    "PH-S1400",
    "AUDIT_LOC_AUDIT_SLICES",
    "PH-S1401",
    "audit_loc_audit_integration",
    "PH-S1402",
    "VERIFY_AUDIT_LOC_AUDIT",
    "PH-S1404",
    "--audit-loc-audit",
    "PH-S1408",
];

/// Production-verify stub: report how many of the five slice flags are present (PH-S1400).
pub fn audit_loc_audit_slices_met(loc_audit_src: &str) -> (usize, usize) {
    let total = AUDIT_LOC_AUDIT_SLICES.len();
    let met = AUDIT_LOC_AUDIT_SLICES
        .iter()
        .filter(|flag| loc_audit_src.contains(*flag))
        .count();
    (met, total)
}

/// Classify audit loc-audit band depth from optional feature stub (PH-S1399).
pub fn audit_loc_audit_depth_stub(features: Option<&Value>) -> AuditLocAuditDepth {
    let Some(f) = features else {
        return AuditLocAuditDepth::None;
    };
    let depth = f
        .get("audit_loc_audit_depth")
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
        .get("audit_loc_audit_docs")
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
        return AuditLocAuditDepth::FullBand76;
    }
    if close || ratio {
        return AuditLocAuditDepth::RatioHold;
    }
    if docs {
        return AuditLocAuditDepth::DocsCanon;
    }
    if loc {
        return AuditLocAuditDepth::LocAuditFlag;
    }
    if export {
        return AuditLocAuditDepth::StandSmokeExport;
    }
    if verify {
        return AuditLocAuditDepth::VerifyDevStandHook;
    }
    if contracts {
        return AuditLocAuditDepth::CriteriaContracts;
    }
    if slices {
        return AuditLocAuditDepth::SliceAggregate;
    }
    if depth {
        return AuditLocAuditDepth::DepthModule;
    }
    AuditLocAuditDepth::None
}

/// Total audit loc-audit criteria in registry (PH-S1399).
pub fn audit_loc_audit_criteria_total() -> usize {
    AUDIT_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_loc_audit_depth_stub_ph_s1399() {
        assert_eq!(audit_loc_audit_depth_stub(None), AuditLocAuditDepth::None);
        assert_eq!(
            audit_loc_audit_depth_stub(Some(&json!({"audit_loc_audit_depth": true}))),
            AuditLocAuditDepth::DepthModule
        );
        assert_eq!(
            audit_loc_audit_depth_stub(Some(&json!({
                "audit_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditLocAuditDepth::FullBand76
        );
        assert_eq!(AUDIT_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(audit_loc_audit_criteria_total(), 10);
        assert_eq!(AUDIT_LOC_AUDIT_SLICES.len(), 5);
        assert!(FM_BAND76_ROWS.contains(&"PH-S1399…S1408"));
    }

    #[test]
    fn audit_loc_audit_slices_met_ph_s1400() {
        let src = "--audit --audit-store --audit-api --audit-admin-ops --audit-stand-smoke";
        assert_eq!(audit_loc_audit_slices_met(src), (5, 5));
        assert_eq!(audit_loc_audit_slices_met("--audit"), (1, 5));
    }
}
