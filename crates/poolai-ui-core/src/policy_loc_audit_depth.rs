//! Policies loc-audit aggregate band depth (PH-S1499…S1508, band 86 — enterprise phase D).
//!
//! Consolidates band 81–85 `--policy*` loc-audit slices under one aggregate gate.

use serde_json::Value;

/// Policies loc-audit depth flags (aggregate / slices / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyLocAuditDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand86,
}

/// Band 81–85 loc-audit slice flags covered by aggregate (PH-S1500).
pub const POLICY_LOC_AUDIT_SLICES: &[&str] = &[
    "--policy",
    "--policy-store",
    "--policy-api",
    "--policy-admin-ops",
    "--policy-stand-smoke",
];

/// Policies loc-audit criteria registry (PH-S1499): id · marker · doc path.
pub const POLICY_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_loc_audit_depth",
        "PolicyLocAuditDepth",
        "crates/poolai-ui-core/src/policy_loc_audit_depth.rs",
    ),
    ("slice_policy", "--policy", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_store",
        "--policy-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    ("slice_api", "--policy-api", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_admin_ops",
        "--policy-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_stand_smoke",
        "--policy-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "aggregate_flag",
        "--policy-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "policy_loc_audit_docs",
        "POLICIES_LOC_AUDIT.md",
        "docs/development/POLICIES_LOC_AUDIT.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1499_integration",
        "tests/galaxy_horizon_s1499_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-loc-audit` case names (PH-S1504).
pub const POLICY_LOC_AUDIT_CASES: &[&str] = &[
    "policy_loc_audit_depth",
    "slice_policy",
    "slice_store",
    "slice_api",
    "slice_admin_ops",
    "slice_stand_smoke",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "policy_loc_audit_docs",
    "band_close",
];

/// FM §5.67 band-86 marker rows.
pub const FM_BAND86_ROWS: &[&str] = &[
    "5.67",
    "Policies loc-audit",
    "PH-S1499…S1508",
    "policy_loc_audit_depth",
];

/// Policies loc-audit adoption markers for band 86.
pub const POLICY_LOC_AUDIT_BAND86_ROWS: &[&str] = &[
    "PH-S1499",
    "policy_loc_audit_depth",
    "PH-S1500",
    "POLICY_LOC_AUDIT_SLICES",
    "PH-S1501",
    "policy_loc_audit_integration",
    "PH-S1502",
    "VERIFY_POLICY_LOC_AUDIT",
    "PH-S1504",
    "--policy-loc-audit",
    "PH-S1508",
];

/// Production-verify stub: report how many of the five slice flags are present (PH-S1500).
pub fn policy_loc_audit_slices_met(loc_audit_src: &str) -> (usize, usize) {
    let total = POLICY_LOC_AUDIT_SLICES.len();
    let met = POLICY_LOC_AUDIT_SLICES
        .iter()
        .filter(|flag| loc_audit_src.contains(*flag))
        .count();
    (met, total)
}

/// Classify policies loc-audit band depth from optional feature stub (PH-S1499).
pub fn policy_loc_audit_depth_stub(features: Option<&Value>) -> PolicyLocAuditDepth {
    let Some(f) = features else {
        return PolicyLocAuditDepth::None;
    };
    let depth = f
        .get("policy_loc_audit_depth")
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
        .get("policy_loc_audit_docs")
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
        return PolicyLocAuditDepth::FullBand86;
    }
    if close || ratio {
        return PolicyLocAuditDepth::RatioHold;
    }
    if docs {
        return PolicyLocAuditDepth::DocsCanon;
    }
    if loc {
        return PolicyLocAuditDepth::LocAuditFlag;
    }
    if export {
        return PolicyLocAuditDepth::StandSmokeExport;
    }
    if verify {
        return PolicyLocAuditDepth::VerifyDevStandHook;
    }
    if contracts {
        return PolicyLocAuditDepth::CriteriaContracts;
    }
    if slices {
        return PolicyLocAuditDepth::SliceAggregate;
    }
    if depth {
        return PolicyLocAuditDepth::DepthModule;
    }
    PolicyLocAuditDepth::None
}

/// Total policies loc-audit criteria in registry (PH-S1499).
pub fn policy_loc_audit_criteria_total() -> usize {
    POLICY_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_loc_audit_depth_stub_ph_s1499() {
        assert_eq!(policy_loc_audit_depth_stub(None), PolicyLocAuditDepth::None);
        assert_eq!(
            policy_loc_audit_depth_stub(Some(&json!({"policy_loc_audit_depth": true}))),
            PolicyLocAuditDepth::DepthModule
        );
        assert_eq!(
            policy_loc_audit_depth_stub(Some(&json!({
                "policy_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyLocAuditDepth::FullBand86
        );
        assert_eq!(POLICY_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(policy_loc_audit_criteria_total(), 10);
        assert_eq!(POLICY_LOC_AUDIT_SLICES.len(), 5);
        assert!(FM_BAND86_ROWS.contains(&"PH-S1499…S1508"));
    }

    #[test]
    fn policy_loc_audit_slices_met_ph_s1500() {
        let src = "--policy --policy-store --policy-api --policy-admin-ops --policy-stand-smoke";
        assert_eq!(policy_loc_audit_slices_met(src), (5, 5));
        assert_eq!(policy_loc_audit_slices_met("--policy"), (1, 5));
    }
}
