//! Tenant docs-canon band depth (PH-S1209…S1218, band 57 — enterprise phase A).
//!
//! Consolidates band 51–56 `TENANT_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// Tenant docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantDocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand57,
}

/// Band 51–56 tenant canon doc filenames covered by aggregate (PH-S1210).
pub const TENANT_DOCS_CANON_SLICES: &[&str] = &[
    "TENANT_PERSIST.md",
    "TENANT_STORE.md",
    "TENANT_API.md",
    "TENANT_ADMIN_OPS.md",
    "TENANT_STAND_SMOKE.md",
    "TENANT_LOC_AUDIT.md",
];

/// Tenant docs-canon criteria registry (PH-S1209): id · marker · doc path.
pub const TENANT_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_docs_canon_depth",
        "TenantDocsCanonDepth",
        "crates/poolai-ui-core/src/tenant_docs_canon_depth.rs",
    ),
    (
        "doc_persist",
        "TENANT_PERSIST.md",
        "docs/development/TENANT_PERSIST.md",
    ),
    (
        "doc_store",
        "TENANT_STORE.md",
        "docs/development/TENANT_STORE.md",
    ),
    ("doc_api", "TENANT_API.md", "docs/development/TENANT_API.md"),
    (
        "doc_admin_ops",
        "TENANT_ADMIN_OPS.md",
        "docs/development/TENANT_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "TENANT_STAND_SMOKE.md",
        "docs/development/TENANT_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "TENANT_LOC_AUDIT.md",
        "docs/development/TENANT_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--tenant-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1209_integration",
        "tests/galaxy_horizon_s1209_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-docs-canon` case names (PH-S1214).
pub const TENANT_DOCS_CANON_CASES: &[&str] = &[
    "tenant_docs_canon_depth",
    "doc_persist",
    "doc_store",
    "doc_api",
    "doc_admin_ops",
    "doc_stand_smoke",
    "doc_loc_audit",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "band_close",
];

/// FM §5.38 band-57 marker rows.
pub const FM_BAND57_ROWS: &[&str] = &[
    "5.38",
    "Tenant docs canon",
    "PH-S1209…S1218",
    "tenant_docs_canon_depth",
];

/// Tenant docs-canon adoption markers for band 57.
pub const TENANT_DOCS_CANON_BAND57_ROWS: &[&str] = &[
    "PH-S1209",
    "tenant_docs_canon_depth",
    "PH-S1210",
    "TENANT_DOCS_CANON_SLICES",
    "PH-S1211",
    "tenant_docs_canon_integration",
    "PH-S1212",
    "VERIFY_TENANT_DOCS_CANON",
    "PH-S1214",
    "--tenant-docs-canon",
    "PH-S1218",
];

/// Production-verify stub: how many of the six tenant canon docs are referenced (PH-S1210).
pub fn tenant_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = TENANT_DOCS_CANON_SLICES.len();
    let met = TENANT_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify tenant docs-canon band depth from optional feature stub (PH-S1209).
pub fn tenant_docs_canon_depth_stub(features: Option<&Value>) -> TenantDocsCanonDepth {
    let Some(f) = features else {
        return TenantDocsCanonDepth::None;
    };
    let depth = f
        .get("tenant_docs_canon_depth")
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
        .get("tenant_docs_canon_docs")
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
        return TenantDocsCanonDepth::FullBand57;
    }
    if close || ratio {
        return TenantDocsCanonDepth::RatioHold;
    }
    if docs {
        return TenantDocsCanonDepth::DocsCanon;
    }
    if loc {
        return TenantDocsCanonDepth::LocAuditFlag;
    }
    if export {
        return TenantDocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return TenantDocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return TenantDocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return TenantDocsCanonDepth::SliceAggregate;
    }
    if depth {
        return TenantDocsCanonDepth::DepthModule;
    }
    TenantDocsCanonDepth::None
}

/// Total tenant docs-canon criteria in registry (PH-S1209).
pub fn tenant_docs_canon_criteria_total() -> usize {
    TENANT_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_docs_canon_depth_stub_ph_s1209() {
        assert_eq!(
            tenant_docs_canon_depth_stub(None),
            TenantDocsCanonDepth::None
        );
        assert_eq!(
            tenant_docs_canon_depth_stub(Some(&json!({"tenant_docs_canon_depth": true}))),
            TenantDocsCanonDepth::DepthModule
        );
        assert_eq!(
            tenant_docs_canon_depth_stub(Some(&json!({
                "tenant_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantDocsCanonDepth::FullBand57
        );
        assert_eq!(TENANT_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(tenant_docs_canon_criteria_total(), 10);
        assert_eq!(TENANT_DOCS_CANON_SLICES.len(), 6);
        assert!(FM_BAND57_ROWS.contains(&"PH-S1209…S1218"));
    }

    #[test]
    fn tenant_docs_canon_slices_met_ph_s1210() {
        let src = "TENANT_PERSIST.md TENANT_STORE.md TENANT_API.md TENANT_ADMIN_OPS.md TENANT_STAND_SMOKE.md TENANT_LOC_AUDIT.md";
        assert_eq!(tenant_docs_canon_slices_met(src), (6, 6));
        assert_eq!(tenant_docs_canon_slices_met("TENANT_PERSIST.md"), (1, 6));
    }
}
