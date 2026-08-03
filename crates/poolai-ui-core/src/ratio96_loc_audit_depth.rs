//! Ratio96 loc-audit depth band (PH-S1699…S1708, band 106 — phase F loc-audit).
//!
//! Phase F loc-audit (RUST_RATIO_STRATEGY §phase F): loc-audit for ratio96
//! store/wire/migration via `poolai-loc-audit --ratio96-loc-audit`.
//! This module is the band-106 depth registry and the aggregate gate for the
//! `ratio96 loc-audit` slices.

use serde_json::Value;

/// Ratio96 loc-audit depth flags (registry / smoke / export / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio96LocAuditDepth {
    None,
    DepthModule,
    LocAuditSmoke,
    MigrationAdvisory,
    ExportShape,
    LocAuditFlag,
    DocsCanon,
    VisionSync,
    RatioHold,
    FullBand106,
}

/// Ratio96 loc-audit criteria registry (PH-S1699): id · marker · doc path.
pub const RATIO96_LOC_AUDIT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "ratio96_loc_audit_depth",
        "Ratio96LocAuditDepth",
        "crates/poolai-ui-core/src/ratio96_loc_audit_depth.rs",
    ),
    (
        "loc_audit_smoke",
        "smoke_ratio96_loc_audit",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "migration_advisory",
        "smoke_ratio96_migration_advisory",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "export_shape",
        "ratio96_loc_audit_band106_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--ratio96-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "RATIO96_LOC_AUDIT.md",
        "docs/development/RATIO96_LOC_AUDIT.md",
    ),
    (
        "vision_sync",
        "poolai-vision-sync",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "verify_hook",
        "VERIFY_RATIO96_LOC_AUDIT",
        "bin/verify-dev-stand.sh",
    ),
    (
        "ratio_hold",
        "ratio96-loc-audit",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1699_integration",
        "tests/galaxy_horizon_s1699_integration.rs",
    ),
];

/// `poolai-loc-audit --ratio96-loc-audit` case names (PH-S1703).
pub const RATIO96_LOC_AUDIT_CASES: &[&str] = &[
    "ratio96_loc_audit_depth",
    "loc_audit_smoke",
    "migration_advisory",
    "export_shape",
    "loc_audit_flag",
    "docs_canon",
    "vision_sync",
    "verify_hook",
    "ratio_hold",
    "band_close",
];

/// FM §5.87 band-106 marker rows.
pub const FM_BAND106_ROWS: &[&str] = &[
    "5.87",
    "Ratio96 loc-audit",
    "PH-S1699…S1708",
    "ratio96_loc_audit_depth",
];

/// Ratio96 loc-audit adoption markers for band 106.
pub const RATIO96_LOC_AUDIT_BAND106_ROWS: &[&str] = &[
    "PH-S1699",
    "ratio96_loc_audit_depth",
    "PH-S1700",
    "smoke_ratio96_loc_audit",
    "PH-S1701",
    "smoke_ratio96_migration_advisory",
    "PH-S1702",
    "ratio96_loc_audit_band106_export_shape",
    "PH-S1703",
    "--ratio96-loc-audit",
    "PH-S1708",
];

/// Classify ratio96 loc-audit band depth from optional feature stub (PH-S1699).
pub fn ratio96_loc_audit_depth_stub(features: Option<&Value>) -> Ratio96LocAuditDepth {
    let Some(f) = features else {
        return Ratio96LocAuditDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("ratio96_loc_audit_depth");
    let smoke = enabled("loc_audit_smoke");
    let migration = enabled("migration_advisory");
    let export = enabled("export_shape");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("docs_canon");
    let vision = enabled("vision_sync");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && smoke && migration && export && loc && docs && vision && ratio && close {
        return Ratio96LocAuditDepth::FullBand106;
    }
    if close || ratio {
        return Ratio96LocAuditDepth::RatioHold;
    }
    if vision {
        return Ratio96LocAuditDepth::VisionSync;
    }
    if docs {
        return Ratio96LocAuditDepth::DocsCanon;
    }
    if loc {
        return Ratio96LocAuditDepth::LocAuditFlag;
    }
    if export {
        return Ratio96LocAuditDepth::ExportShape;
    }
    if migration {
        return Ratio96LocAuditDepth::MigrationAdvisory;
    }
    if smoke {
        return Ratio96LocAuditDepth::LocAuditSmoke;
    }
    if depth {
        return Ratio96LocAuditDepth::DepthModule;
    }
    Ratio96LocAuditDepth::None
}

/// Total ratio96 loc-audit criteria in registry (PH-S1699).
pub fn ratio96_loc_audit_criteria_total() -> usize {
    RATIO96_LOC_AUDIT_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_loc_audit_depth_stub_ph_s1699() {
        assert_eq!(
            ratio96_loc_audit_depth_stub(None),
            Ratio96LocAuditDepth::None
        );
        assert_eq!(
            ratio96_loc_audit_depth_stub(Some(&json!({"ratio96_loc_audit_depth": true}))),
            Ratio96LocAuditDepth::DepthModule
        );
        assert_eq!(
            ratio96_loc_audit_depth_stub(Some(&json!({
                "ratio96_loc_audit_depth": true,
                "loc_audit_smoke": true,
                "migration_advisory": true,
                "export_shape": true,
                "loc_audit_flag": true,
                "docs_canon": true,
                "vision_sync": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            Ratio96LocAuditDepth::FullBand106
        );
        assert_eq!(RATIO96_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(ratio96_loc_audit_criteria_total(), 10);
        assert!(FM_BAND106_ROWS.contains(&"PH-S1699…S1708"));
    }
}
