//! Ratio 96% stretch depth band (PH-S1649…S1658, band 101 — phase F depth scaffold).
//!
//! Phase F stretch (RUST_RATIO_STRATEGY §phase F): reach `rust_ratio >= 0.96` via wasm wiring,
//! slim JS/i18n/charts, Rust stand/e2e bins. This module is the band-101 depth registry and the
//! aggregate gate for the `ratio96` slices.

use serde_json::Value;

/// Ratio96 band depth flags (registry / slices / store / contracts / glue / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio96Depth {
    None,
    DepthModule,
    SliceAggregate,
    StoreWire,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand101,
}

/// Phase-F Ratio96 slices covered by the band-101 aggregate (PH-S1649).
pub const RATIO96_PHASE_F_SLICES: &[&str] = &[
    "Ratio96Depth",
    "ratio96_store_wire",
    "ratio96_depth_contracts",
    "VERIFY_RATIO96",
    "ratio96_band101_export_shape",
    "--ratio96",
    "RATIO96_DEPTH.md",
    "RATIO96_RATIO_ADVISORY.md",
    "ratio96_store_depth",
    "PH-S1658",
];

/// Ratio96 band-101 criteria registry (PH-S1649): id · marker · doc path.
pub const RATIO96_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "ratio96_depth",
        "Ratio96Depth",
        "crates/poolai-ui-core/src/ratio96_depth.rs",
    ),
    (
        "phase_f_slices",
        "RATIO96_PHASE_F_SLICES",
        "crates/poolai-ui-core/src/ratio96_depth.rs",
    ),
    (
        "ratio96_store_wire",
        "ratio96_store_wire",
        "crates/poolai-ui-core/src/ratio96_store_depth.rs",
    ),
    (
        "ratio96_api_contracts",
        "ratio96_depth_contracts",
        "tests/ratio96_depth_contracts.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_RATIO96",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "ratio96_band101_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    ("loc_audit_flag", "--ratio96", "src/bin/poolai_loc_audit.rs"),
    (
        "docs_canon",
        "RATIO96_DEPTH.md",
        "docs/development/RATIO96_DEPTH.md",
    ),
    (
        "ratio_hold_advisory",
        "RATIO96_RATIO_ADVISORY.md",
        "docs/development/RATIO96_RATIO_ADVISORY.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1649_integration",
        "tests/galaxy_horizon_s1649_integration.rs",
    ),
];

/// `poolai-loc-audit --ratio96` case names (PH-S1654).
pub const RATIO96_CASES: &[&str] = &[
    "ratio96_depth",
    "phase_f_slices",
    "ratio96_store_wire",
    "ratio96_api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "ratio_hold_advisory",
    "band_close",
];

/// FM §5.82 band-101 marker rows.
pub const FM_BAND101_ROWS: &[&str] = &[
    "5.82",
    "Ratio96 depth scaffold",
    "PH-S1649…S1658",
    "ratio96_depth",
];

/// Ratio96 adoption markers for band 101.
pub const RATIO96_BAND101_ROWS: &[&str] = &[
    "PH-S1649",
    "ratio96_depth",
    "PH-S1650",
    "ratio96_store_wire",
    "PH-S1651",
    "ratio96_depth_contracts",
    "PH-S1652",
    "VERIFY_RATIO96",
    "PH-S1654",
    "--ratio96",
    "PH-S1658",
];

/// Production-verify stub: how many phase-F slices are referenced (PH-S1649).
pub fn ratio96_phase_f_slices_met(canon_src: &str) -> (usize, usize) {
    let total = RATIO96_PHASE_F_SLICES.len();
    let met = RATIO96_PHASE_F_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify ratio96 band depth from optional feature stub (PH-S1649).
pub fn ratio96_depth_stub(features: Option<&Value>) -> Ratio96Depth {
    let Some(f) = features else {
        return Ratio96Depth::None;
    };
    let depth = f
        .get("ratio96_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let slices = f
        .get("slice_aggregate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
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
        .get("ratio96_docs")
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

    if depth && slices && store && contracts && verify && export && loc && docs && ratio && close {
        return Ratio96Depth::FullBand101;
    }
    if close || ratio {
        return Ratio96Depth::RatioHold;
    }
    if docs {
        return Ratio96Depth::DocsCanon;
    }
    if loc {
        return Ratio96Depth::LocAuditFlag;
    }
    if export {
        return Ratio96Depth::StandSmokeExport;
    }
    if verify {
        return Ratio96Depth::VerifyDevStandHook;
    }
    if contracts {
        return Ratio96Depth::CriteriaContracts;
    }
    if store {
        return Ratio96Depth::StoreWire;
    }
    if slices {
        return Ratio96Depth::SliceAggregate;
    }
    if depth {
        return Ratio96Depth::DepthModule;
    }
    Ratio96Depth::None
}

/// Total ratio96 criteria in registry (PH-S1649).
pub fn ratio96_criteria_total() -> usize {
    RATIO96_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ratio96_depth_stub_ph_s1649() {
        assert_eq!(ratio96_depth_stub(None), Ratio96Depth::None);
        assert_eq!(
            ratio96_depth_stub(Some(&json!({"ratio96_depth": true}))),
            Ratio96Depth::DepthModule
        );
        assert_eq!(
            ratio96_depth_stub(Some(&json!({
                "ratio96_depth": true,
                "slice_aggregate": true,
                "store_wire": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "ratio96_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            Ratio96Depth::FullBand101
        );
        assert_eq!(RATIO96_CRITERIA.len(), 10);
        assert_eq!(ratio96_criteria_total(), 10);
        assert_eq!(RATIO96_PHASE_F_SLICES.len(), 10);
        assert!(FM_BAND101_ROWS.contains(&"PH-S1649…S1658"));
    }

    #[test]
    fn ratio96_phase_f_slices_met_ph_s1649() {
        let canon = "Ratio96Depth ratio96_store_wire ratio96_depth_contracts VERIFY_RATIO96 ratio96_band101_export_shape --ratio96 RATIO96_DEPTH.md RATIO96_RATIO_ADVISORY.md ratio96_store_depth PH-S1658";
        assert_eq!(ratio96_phase_f_slices_met(canon), (10, 10));
        assert_eq!(ratio96_phase_f_slices_met("Ratio96Depth"), (1, 10));
    }
}
