//! Audit docs-canon band depth (PH-S1409…S1418, band 77 — enterprise phase C).
//!
//! Consolidates band 71–76 `AUDIT_*.md` canon docs under one docs-canon gate.

use serde_json::Value;

/// Audit docs-canon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDocsCanonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand77,
}

/// Band 71–76 Audit canon doc filenames covered by aggregate (PH-S1410).
pub const AUDIT_DOCS_CANON_SLICES: &[&str] = &[
    "AUDIT_DEPTH.md",
    "AUDIT_STORE.md",
    "AUDIT_API.md",
    "AUDIT_ADMIN_OPS.md",
    "AUDIT_STAND_SMOKE.md",
    "AUDIT_LOC_AUDIT.md",
];

/// Audit docs-canon criteria registry (PH-S1409): id · marker · doc path.
pub const AUDIT_DOCS_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_docs_canon_depth",
        "AuditDocsCanonDepth",
        "crates/poolai-ui-core/src/audit_docs_canon_depth.rs",
    ),
    (
        "doc_depth",
        "AUDIT_DEPTH.md",
        "docs/development/AUDIT_DEPTH.md",
    ),
    (
        "doc_store",
        "AUDIT_STORE.md",
        "docs/development/AUDIT_STORE.md",
    ),
    ("doc_api", "AUDIT_API.md", "docs/development/AUDIT_API.md"),
    (
        "doc_admin_ops",
        "AUDIT_ADMIN_OPS.md",
        "docs/development/AUDIT_ADMIN_OPS.md",
    ),
    (
        "doc_stand_smoke",
        "AUDIT_STAND_SMOKE.md",
        "docs/development/AUDIT_STAND_SMOKE.md",
    ),
    (
        "doc_loc_audit",
        "AUDIT_LOC_AUDIT.md",
        "docs/development/AUDIT_LOC_AUDIT.md",
    ),
    (
        "aggregate_flag",
        "--audit-docs-canon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_DOCS_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1409_integration",
        "tests/galaxy_horizon_s1409_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-docs-canon` case names (PH-S1414).
pub const AUDIT_DOCS_CANON_CASES: &[&str] = &[
    "audit_docs_canon_depth",
    "doc_depth",
    "doc_store",
    "doc_api",
    "doc_admin_ops",
    "doc_stand_smoke",
    "doc_loc_audit",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "band_close",
];

/// FM §5.58 band-77 marker rows.
pub const FM_BAND77_ROWS: &[&str] = &[
    "5.58",
    "Audit docs canon",
    "PH-S1409…S1418",
    "audit_docs_canon_depth",
];

/// Audit docs-canon adoption markers for band 77.
pub const AUDIT_DOCS_CANON_BAND77_ROWS: &[&str] = &[
    "PH-S1409",
    "audit_docs_canon_depth",
    "PH-S1410",
    "AUDIT_DOCS_CANON_SLICES",
    "PH-S1411",
    "audit_docs_canon_integration",
    "PH-S1412",
    "VERIFY_AUDIT_DOCS_CANON",
    "PH-S1414",
    "--audit-docs-canon",
    "PH-S1418",
];

/// Production-verify stub: how many of the six Audit canon docs are referenced (PH-S1410).
pub fn audit_docs_canon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = AUDIT_DOCS_CANON_SLICES.len();
    let met = AUDIT_DOCS_CANON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Audit docs-canon band depth from optional feature stub (PH-S1409).
pub fn audit_docs_canon_depth_stub(features: Option<&Value>) -> AuditDocsCanonDepth {
    let Some(f) = features else {
        return AuditDocsCanonDepth::None;
    };
    let depth = f
        .get("audit_docs_canon_depth")
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
        .get("audit_docs_canon_docs")
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
        return AuditDocsCanonDepth::FullBand77;
    }
    if close || ratio {
        return AuditDocsCanonDepth::RatioHold;
    }
    if docs {
        return AuditDocsCanonDepth::DocsCanon;
    }
    if loc {
        return AuditDocsCanonDepth::LocAuditFlag;
    }
    if export {
        return AuditDocsCanonDepth::StandSmokeExport;
    }
    if verify {
        return AuditDocsCanonDepth::VerifyDevStandHook;
    }
    if contracts {
        return AuditDocsCanonDepth::CriteriaContracts;
    }
    if slices {
        return AuditDocsCanonDepth::SliceAggregate;
    }
    if depth {
        return AuditDocsCanonDepth::DepthModule;
    }
    AuditDocsCanonDepth::None
}

/// Total Audit docs-canon criteria in registry (PH-S1409).
pub fn audit_docs_canon_criteria_total() -> usize {
    AUDIT_DOCS_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_docs_canon_depth_stub_ph_s1409() {
        assert_eq!(audit_docs_canon_depth_stub(None), AuditDocsCanonDepth::None);
        assert_eq!(
            audit_docs_canon_depth_stub(Some(&json!({"audit_docs_canon_depth": true}))),
            AuditDocsCanonDepth::DepthModule
        );
        assert_eq!(
            audit_docs_canon_depth_stub(Some(&json!({
                "audit_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditDocsCanonDepth::FullBand77
        );
        assert_eq!(AUDIT_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(audit_docs_canon_criteria_total(), 10);
        assert_eq!(AUDIT_DOCS_CANON_SLICES.len(), 6);
        assert!(FM_BAND77_ROWS.contains(&"PH-S1409…S1418"));
    }

    #[test]
    fn audit_docs_canon_slices_met_ph_s1410() {
        let src = "AUDIT_DEPTH.md AUDIT_STORE.md AUDIT_API.md AUDIT_ADMIN_OPS.md AUDIT_STAND_SMOKE.md AUDIT_LOC_AUDIT.md";
        assert_eq!(audit_docs_canon_slices_met(src), (6, 6));
        assert_eq!(audit_docs_canon_slices_met("AUDIT_DEPTH.md"), (1, 6));
    }
}
