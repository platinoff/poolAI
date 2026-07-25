//! Audit horizon-close band depth (PH-S1439…S1448, band 80 — enterprise phase C close).
//!
//! Aggregates bands 71–79 `--audit*` loc-audit slices under one horizon gate,
//! closing Audit phase C before Policies (band 81).

use serde_json::Value;

/// Audit horizon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditHorizonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand80,
}

/// Phase-C Audit loc-audit / canon slices covered by horizon aggregate (PH-S1440).
pub const AUDIT_HORIZON_SLICES: &[&str] = &[
    "--audit",
    "--audit-store",
    "--audit-api",
    "--audit-admin-ops",
    "--audit-stand-smoke",
    "--audit-loc-audit",
    "--audit-docs-canon",
    "--audit-vision-sync",
    "--audit-ratio-advisory",
    "AUDIT_RATIO_ADVISORY.md",
];

/// Audit horizon criteria registry (PH-S1439): id · marker · doc path.
pub const AUDIT_HORIZON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_horizon_depth",
        "AuditHorizonDepth",
        "crates/poolai-ui-core/src/audit_horizon_depth.rs",
    ),
    (
        "phase_c_slices",
        "AUDIT_HORIZON_SLICES",
        "crates/poolai-ui-core/src/audit_horizon_depth.rs",
    ),
    (
        "prior_audit_store",
        "--audit-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_audit_horizon",
        "AUDIT_HORIZON.md",
        "docs/development/AUDIT_HORIZON.md",
    ),
    (
        "doc_ratio_advisory",
        "AUDIT_RATIO_ADVISORY.md",
        "docs/development/AUDIT_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--audit-horizon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_HORIZON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "audit_horizon_integration",
        "tests/audit_horizon_integration.rs",
    ),
    (
        "ratio_advisory_prior",
        "audit_ratio_advisory_depth",
        "crates/poolai-ui-core/src/audit_ratio_advisory_depth.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1439_integration",
        "tests/galaxy_horizon_s1439_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-horizon` case names (PH-S1444).
pub const AUDIT_HORIZON_CASES: &[&str] = &[
    "audit_horizon_depth",
    "phase_c_slices",
    "prior_audit_store",
    "doc_audit_horizon",
    "doc_ratio_advisory",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "ratio_advisory_prior",
    "band_close",
];

/// FM §5.61 band-80 marker rows.
pub const FM_BAND80_ROWS: &[&str] = &[
    "5.61",
    "Audit horizon close",
    "PH-S1439…S1448",
    "audit_horizon_depth",
];

/// Audit horizon adoption markers for band 80.
pub const AUDIT_HORIZON_BAND80_ROWS: &[&str] = &[
    "PH-S1439",
    "audit_horizon_depth",
    "PH-S1440",
    "AUDIT_HORIZON_SLICES",
    "PH-S1441",
    "audit_horizon_integration",
    "PH-S1442",
    "VERIFY_AUDIT_HORIZON",
    "PH-S1444",
    "--audit-horizon",
    "PH-S1448",
];

/// Production-verify stub: how many horizon slices are referenced (PH-S1440).
pub fn audit_horizon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = AUDIT_HORIZON_SLICES.len();
    let met = AUDIT_HORIZON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Audit horizon band depth from optional feature stub (PH-S1439).
pub fn audit_horizon_depth_stub(features: Option<&Value>) -> AuditHorizonDepth {
    let Some(f) = features else {
        return AuditHorizonDepth::None;
    };
    let depth = f
        .get("audit_horizon_depth")
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
        .get("audit_horizon_docs")
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
        return AuditHorizonDepth::FullBand80;
    }
    if close || ratio {
        return AuditHorizonDepth::RatioHold;
    }
    if docs {
        return AuditHorizonDepth::DocsCanon;
    }
    if loc {
        return AuditHorizonDepth::LocAuditFlag;
    }
    if export {
        return AuditHorizonDepth::StandSmokeExport;
    }
    if verify {
        return AuditHorizonDepth::VerifyDevStandHook;
    }
    if contracts {
        return AuditHorizonDepth::CriteriaContracts;
    }
    if slices {
        return AuditHorizonDepth::SliceAggregate;
    }
    if depth {
        return AuditHorizonDepth::DepthModule;
    }
    AuditHorizonDepth::None
}

/// Total Audit horizon criteria in registry (PH-S1439).
pub fn audit_horizon_criteria_total() -> usize {
    AUDIT_HORIZON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_horizon_depth_stub_ph_s1439() {
        assert_eq!(audit_horizon_depth_stub(None), AuditHorizonDepth::None);
        assert_eq!(
            audit_horizon_depth_stub(Some(&json!({
                "audit_horizon_depth": true
            }))),
            AuditHorizonDepth::DepthModule
        );
        assert_eq!(
            audit_horizon_depth_stub(Some(&json!({
                "audit_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditHorizonDepth::FullBand80
        );
        assert_eq!(AUDIT_HORIZON_CRITERIA.len(), 10);
        assert_eq!(audit_horizon_criteria_total(), 10);
        assert_eq!(AUDIT_HORIZON_SLICES.len(), 10);
        assert!(FM_BAND80_ROWS.contains(&"PH-S1439…S1448"));
    }

    #[test]
    fn audit_horizon_slices_met_ph_s1440() {
        let src = "--audit --audit-store --audit-api --audit-admin-ops --audit-stand-smoke --audit-loc-audit --audit-docs-canon --audit-vision-sync --audit-ratio-advisory AUDIT_RATIO_ADVISORY.md";
        assert_eq!(audit_horizon_slices_met(src), (10, 10));
        assert_eq!(audit_horizon_slices_met("--audit"), (1, 10));
    }
}
