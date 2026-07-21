//! Tenant loc-audit aggregate band depth (PH-S1199…S1208, band 56 — enterprise phase A).
//!
//! Consolidates band 51–55 `--tenant-*` loc-audit slices under one aggregate gate.

use serde_json::Value;

/// Tenant loc-audit depth flags (aggregate / slices / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantLocAuditDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand56,
}

/// Band 51–55 loc-audit slice flags covered by aggregate (PH-S1200).
pub const TENANT_LOC_AUDIT_SLICES: &[&str] = &[
    "--tenant-persist",
    "--tenant-store",
    "--tenant-api",
    "--tenant-admin-ops",
    "--tenant-stand-smoke",
];

/// Tenant loc-audit criteria registry (PH-S1199): id · marker · doc path.
pub const TENANT_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_loc_audit_depth",
        "TenantLocAuditDepth",
        "crates/poolai-ui-core/src/tenant_loc_audit_depth.rs",
    ),
    (
        "slice_persist",
        "--tenant-persist",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_store",
        "--tenant-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    ("slice_api", "--tenant-api", "src/bin/poolai_loc_audit.rs"),
    (
        "slice_admin_ops",
        "--tenant-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "slice_stand_smoke",
        "--tenant-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "aggregate_flag",
        "--tenant-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "tenant_loc_audit_docs",
        "TENANT_LOC_AUDIT.md",
        "docs/development/TENANT_LOC_AUDIT.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1199_integration",
        "tests/galaxy_horizon_s1199_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-loc-audit` case names (PH-S1204).
pub const TENANT_LOC_AUDIT_CASES: &[&str] = &[
    "tenant_loc_audit_depth",
    "slice_persist",
    "slice_store",
    "slice_api",
    "slice_admin_ops",
    "slice_stand_smoke",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "tenant_loc_audit_docs",
    "band_close",
];

/// FM §5.37 band-56 marker rows.
pub const FM_BAND56_ROWS: &[&str] = &[
    "5.37",
    "Tenant loc-audit",
    "PH-S1199…S1208",
    "tenant_loc_audit_depth",
];

/// Tenant loc-audit adoption markers for band 56.
pub const TENANT_LOC_AUDIT_BAND56_ROWS: &[&str] = &[
    "PH-S1199",
    "tenant_loc_audit_depth",
    "PH-S1200",
    "TENANT_LOC_AUDIT_SLICES",
    "PH-S1201",
    "tenant_loc_audit_integration",
    "PH-S1202",
    "VERIFY_TENANT_LOC_AUDIT",
    "PH-S1204",
    "--tenant-loc-audit",
    "PH-S1208",
];

/// Production-verify stub: report how many of the five slice flags are present (PH-S1200).
pub fn tenant_loc_audit_slices_met(loc_audit_src: &str) -> (usize, usize) {
    let total = TENANT_LOC_AUDIT_SLICES.len();
    let met = TENANT_LOC_AUDIT_SLICES
        .iter()
        .filter(|flag| loc_audit_src.contains(*flag))
        .count();
    (met, total)
}

/// Classify tenant loc-audit band depth from optional feature stub (PH-S1199).
pub fn tenant_loc_audit_depth_stub(features: Option<&Value>) -> TenantLocAuditDepth {
    let Some(f) = features else {
        return TenantLocAuditDepth::None;
    };
    let depth = f
        .get("tenant_loc_audit_depth")
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
        .get("tenant_loc_audit_docs")
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
        return TenantLocAuditDepth::FullBand56;
    }
    if close || ratio {
        return TenantLocAuditDepth::RatioHold;
    }
    if docs {
        return TenantLocAuditDepth::DocsCanon;
    }
    if loc {
        return TenantLocAuditDepth::LocAuditFlag;
    }
    if export {
        return TenantLocAuditDepth::StandSmokeExport;
    }
    if verify {
        return TenantLocAuditDepth::VerifyDevStandHook;
    }
    if contracts {
        return TenantLocAuditDepth::CriteriaContracts;
    }
    if slices {
        return TenantLocAuditDepth::SliceAggregate;
    }
    if depth {
        return TenantLocAuditDepth::DepthModule;
    }
    TenantLocAuditDepth::None
}

/// Total tenant loc-audit criteria in registry (PH-S1199).
pub fn tenant_loc_audit_criteria_total() -> usize {
    TENANT_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_loc_audit_depth_stub_ph_s1199() {
        assert_eq!(tenant_loc_audit_depth_stub(None), TenantLocAuditDepth::None);
        assert_eq!(
            tenant_loc_audit_depth_stub(Some(&json!({"tenant_loc_audit_depth": true}))),
            TenantLocAuditDepth::DepthModule
        );
        assert_eq!(
            tenant_loc_audit_depth_stub(Some(&json!({
                "tenant_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantLocAuditDepth::FullBand56
        );
        assert_eq!(TENANT_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(tenant_loc_audit_criteria_total(), 10);
        assert_eq!(TENANT_LOC_AUDIT_SLICES.len(), 5);
        assert!(FM_BAND56_ROWS.contains(&"PH-S1199…S1208"));
    }

    #[test]
    fn tenant_loc_audit_slices_met_ph_s1200() {
        let src =
            "--tenant-persist --tenant-store --tenant-api --tenant-admin-ops --tenant-stand-smoke";
        assert_eq!(tenant_loc_audit_slices_met(src), (5, 5));
        assert_eq!(tenant_loc_audit_slices_met("--tenant-persist"), (1, 5));
    }
}
