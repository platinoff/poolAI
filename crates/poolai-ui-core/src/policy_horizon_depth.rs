//! Policies horizon-close band depth (PH-S1539…S1548, band 90 — enterprise phase D close).
//!
//! Aggregates bands 81–89 `--policy*` + `--policy-ratio-advisory` slices under one horizon gate.

use serde_json::Value;

/// Policies horizon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyHorizonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand90,
}

/// Phase-D Policies loc-audit / canon slices covered by horizon aggregate (PH-S1540).
pub const POLICY_HORIZON_SLICES: &[&str] = &[
    "--policy",
    "--policy-store",
    "--policy-api",
    "--policy-admin-ops",
    "--policy-stand-smoke",
    "--policy-loc-audit",
    "--policy-docs-canon",
    "--policy-vision-sync",
    "--policy-ratio-advisory",
    "POLICIES_RATIO_ADVISORY.md",
];

/// Policies horizon criteria registry (PH-S1539): id · marker · doc path.
pub const POLICY_HORIZON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_horizon_depth",
        "PolicyHorizonDepth",
        "crates/poolai-ui-core/src/policy_horizon_depth.rs",
    ),
    (
        "phase_d_slices",
        "POLICY_HORIZON_SLICES",
        "crates/poolai-ui-core/src/policy_horizon_depth.rs",
    ),
    (
        "prior_policy_store",
        "--policy-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_policy_horizon",
        "POLICIES_HORIZON.md",
        "docs/development/POLICIES_HORIZON.md",
    ),
    (
        "doc_ratio_advisory",
        "POLICIES_RATIO_ADVISORY.md",
        "docs/development/POLICIES_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--policy-horizon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_HORIZON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "policy_horizon_integration",
        "tests/policy_horizon_integration.rs",
    ),
    (
        "ratio_advisory_prior",
        "policy_ratio_advisory_depth",
        "crates/poolai-ui-core/src/policy_ratio_advisory_depth.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1539_integration",
        "tests/galaxy_horizon_s1539_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-horizon` case names (PH-S1544).
pub const POLICY_HORIZON_CASES: &[&str] = &[
    "policy_horizon_depth",
    "phase_d_slices",
    "prior_policy_store",
    "doc_policy_horizon",
    "doc_ratio_advisory",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "ratio_advisory_prior",
    "band_close",
];

/// FM §5.71 band-90 marker rows.
pub const FM_BAND90_ROWS: &[&str] = &[
    "5.71",
    "Policies horizon close",
    "PH-S1539…S1548",
    "policy_horizon_depth",
];

/// Policies horizon adoption markers for band 90.
pub const POLICY_HORIZON_BAND90_ROWS: &[&str] = &[
    "PH-S1539",
    "policy_horizon_depth",
    "PH-S1540",
    "POLICY_HORIZON_SLICES",
    "PH-S1541",
    "policy_horizon_integration",
    "PH-S1542",
    "VERIFY_POLICY_HORIZON",
    "PH-S1544",
    "--policy-horizon",
    "PH-S1548",
];

/// Production-verify stub: how many horizon slices are referenced (PH-S1540).
pub fn policy_horizon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = POLICY_HORIZON_SLICES.len();
    let met = POLICY_HORIZON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify policies horizon band depth from optional feature stub (PH-S1539).
pub fn policy_horizon_depth_stub(features: Option<&Value>) -> PolicyHorizonDepth {
    let Some(f) = features else {
        return PolicyHorizonDepth::None;
    };
    let depth = f
        .get("policy_horizon_depth")
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
        .get("policy_horizon_docs")
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
        return PolicyHorizonDepth::FullBand90;
    }
    if close || ratio {
        return PolicyHorizonDepth::RatioHold;
    }
    if docs {
        return PolicyHorizonDepth::DocsCanon;
    }
    if loc {
        return PolicyHorizonDepth::LocAuditFlag;
    }
    if export {
        return PolicyHorizonDepth::StandSmokeExport;
    }
    if verify {
        return PolicyHorizonDepth::VerifyDevStandHook;
    }
    if contracts {
        return PolicyHorizonDepth::CriteriaContracts;
    }
    if slices {
        return PolicyHorizonDepth::SliceAggregate;
    }
    if depth {
        return PolicyHorizonDepth::DepthModule;
    }
    PolicyHorizonDepth::None
}

/// Total policies horizon criteria in registry (PH-S1539).
pub fn policy_horizon_criteria_total() -> usize {
    POLICY_HORIZON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_horizon_depth_stub_ph_s1539() {
        assert_eq!(policy_horizon_depth_stub(None), PolicyHorizonDepth::None);
        assert_eq!(
            policy_horizon_depth_stub(Some(&json!({"policy_horizon_depth": true}))),
            PolicyHorizonDepth::DepthModule
        );
        assert_eq!(
            policy_horizon_depth_stub(Some(&json!({
                "policy_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyHorizonDepth::FullBand90
        );
        assert_eq!(POLICY_HORIZON_CRITERIA.len(), 10);
        assert_eq!(policy_horizon_criteria_total(), 10);
        assert_eq!(POLICY_HORIZON_SLICES.len(), 10);
        assert!(FM_BAND90_ROWS.contains(&"PH-S1539…S1548"));
    }

    #[test]
    fn policy_horizon_slices_met_ph_s1540() {
        let src = "--policy --policy-store --policy-api --policy-admin-ops --policy-stand-smoke --policy-loc-audit --policy-docs-canon --policy-vision-sync --policy-ratio-advisory POLICIES_RATIO_ADVISORY.md";
        assert_eq!(policy_horizon_slices_met(src), (10, 10));
        assert_eq!(policy_horizon_slices_met("--policy"), (1, 10));
    }
}
