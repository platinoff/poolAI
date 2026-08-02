//! Ratio96 stand smoke depth band (PH-S1689…S1698, band 105 — phase F).
//!
//! Phase F stand smoke (RUST_RATIO_STRATEGY §phase F): live HTTP smoke for ratio96
//! store/wire/query/fixtures via `poolai-http-stand-smoke --ratio96-stand-smoke`.
//! This module is the band-105 depth registry and the aggregate gate for the
//! `ratio96 stand smoke` slices.

use serde_json::Value;

/// Ratio96 stand smoke depth flags (live smoke / export / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio96StandSmokeDepth {
    None,
    DepthModule,
    StoreWireSmoke,
    QuerySmoke,
    FieldFixtureSmoke,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand105,
}

/// Ratio96 stand smoke criteria registry (PH-S1689): id · marker · doc path.
pub const RATIO96_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "ratio96_stand_smoke_depth",
        "Ratio96StandSmokeDepth",
        "crates/poolai-ui-core/src/ratio96_stand_smoke_depth.rs",
    ),
    (
        "store_wire_smoke",
        "smoke_ratio96_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "query_smoke",
        "smoke_ratio96_query",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "field_fixture_smoke",
        "smoke_ratio96_field_fixtures",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_RATIO96_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "ratio96_stand_smoke_band105_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--ratio96-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "RATIO96_STAND_SMOKE.md",
        "docs/development/RATIO96_STAND_SMOKE.md",
    ),
    (
        "ratio_hold",
        "ratio96-stand-smoke",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1689_integration",
        "tests/galaxy_horizon_s1689_integration.rs",
    ),
];

/// `poolai-loc-audit --ratio96-stand-smoke` case names (PH-S1694).
pub const RATIO96_STAND_SMOKE_CASES: &[&str] = &[
    "ratio96_stand_smoke_depth",
    "store_wire_smoke",
    "query_smoke",
    "field_fixture_smoke",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "ratio_hold",
    "band_close",
];

/// FM §5.86 band-105 marker rows.
pub const FM_BAND105_ROWS: &[&str] = &[
    "5.86",
    "Ratio96 stand smoke",
    "PH-S1689…S1698",
    "ratio96_stand_smoke_depth",
];

/// Ratio96 stand smoke adoption markers for band 105.
pub const RATIO96_STAND_SMOKE_BAND105_ROWS: &[&str] = &[
    "PH-S1689",
    "ratio96_stand_smoke_depth",
    "PH-S1690",
    "smoke_ratio96_store_wire",
    "PH-S1691",
    "smoke_ratio96_query",
    "PH-S1693",
    "ratio96_stand_smoke_band105_export_shape",
    "PH-S1694",
    "--ratio96-stand-smoke",
    "PH-S1698",
];

/// Classify ratio96 stand smoke band depth from optional feature stub (PH-S1689).
pub fn ratio96_stand_smoke_depth_stub(features: Option<&Value>) -> Ratio96StandSmokeDepth {
    let Some(f) = features else {
        return Ratio96StandSmokeDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("ratio96_stand_smoke_depth");
    let store = enabled("store_wire_smoke");
    let query = enabled("query_smoke");
    let field = enabled("field_fixture_smoke");
    let verify = enabled("verify_dev_stand_hook");
    let smoke = enabled("stand_smoke_export");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("docs_canon");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && store && query && field && verify && smoke && loc && docs && ratio && close {
        return Ratio96StandSmokeDepth::FullBand105;
    }
    if close || ratio {
        return Ratio96StandSmokeDepth::RatioHold;
    }
    if docs {
        return Ratio96StandSmokeDepth::DocsCanon;
    }
    if loc {
        return Ratio96StandSmokeDepth::LocAuditFlag;
    }
    if smoke {
        return Ratio96StandSmokeDepth::StandSmokeExport;
    }
    if verify {
        return Ratio96StandSmokeDepth::VerifyDevStandHook;
    }
    if field {
        return Ratio96StandSmokeDepth::FieldFixtureSmoke;
    }
    if query {
        return Ratio96StandSmokeDepth::QuerySmoke;
    }
    if store {
        return Ratio96StandSmokeDepth::StoreWireSmoke;
    }
    if depth {
        return Ratio96StandSmokeDepth::DepthModule;
    }
    Ratio96StandSmokeDepth::None
}

/// Total ratio96 stand smoke criteria in registry (PH-S1689).
pub fn ratio96_stand_smoke_criteria_total() -> usize {
    RATIO96_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_stand_smoke_depth_stub_ph_s1689() {
        assert_eq!(
            ratio96_stand_smoke_depth_stub(None),
            Ratio96StandSmokeDepth::None
        );
        assert_eq!(
            ratio96_stand_smoke_depth_stub(Some(&json!({"ratio96_stand_smoke_depth": true}))),
            Ratio96StandSmokeDepth::DepthModule
        );
        assert_eq!(
            ratio96_stand_smoke_depth_stub(Some(&json!({
                "ratio96_stand_smoke_depth": true,
                "store_wire_smoke": true,
                "query_smoke": true,
                "field_fixture_smoke": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "docs_canon": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            Ratio96StandSmokeDepth::FullBand105
        );
        assert_eq!(RATIO96_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(ratio96_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND105_ROWS.contains(&"PH-S1689…S1698"));
    }
}
