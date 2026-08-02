//! PH-S1651: Ratio96 depth API contracts (band 101).
//! Marker: ratio96_depth_contracts
//!
//! Verifies phase-F Ratio96 slices + criteria registry totals + store wire.

use poolai_ui_core::ratio96_depth::{
    ratio96_criteria_total, ratio96_depth_stub, ratio96_phase_f_slices_met, Ratio96Depth,
    RATIO96_CASES, RATIO96_CRITERIA, RATIO96_PHASE_F_SLICES,
};
use poolai_ui_core::ratio96_store_depth::{
    ratio96_store_state, ratio96_store_wire, Ratio96StoreState,
};
use serde_json::json;

#[test]
fn ratio96_depth_registry_ph_s1649() {
    assert_eq!(RATIO96_CRITERIA.len(), 10);
    assert_eq!(ratio96_criteria_total(), 10);
    assert!(RATIO96_CASES.contains(&"loc_audit_flag"));
    assert!(RATIO96_CASES.contains(&"phase_f_slices"));
    assert_eq!(
        ratio96_depth_stub(Some(&json!({"store_wire": true}))),
        Ratio96Depth::StoreWire
    );
}

#[test]
fn ratio96_slice_docs_present_ph_s1649() {
    let canon = include_str!("../docs/development/RATIO96_DEPTH.md");
    let (met, total) = ratio96_phase_f_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all phase-F slices must be listed");
    for name in RATIO96_PHASE_F_SLICES {
        assert!(canon.contains(name), "missing phase-F slice {name}");
    }

    assert!(std::path::Path::new("docs/development/RATIO96_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/rust_ratio.json").exists());

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--ratio96"));
}

#[test]
fn ratio96_criteria_totals_consistent_ph_s1651() {
    assert_eq!(RATIO96_PHASE_F_SLICES.len(), 10);
    assert_eq!(ratio96_criteria_total(), 10);

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
}

#[test]
fn ratio96_store_wire_reads_durable_store_ph_s1650() {
    let state: Ratio96StoreState = ratio96_store_wire().expect("durable ratio store readable");
    assert!(state.stretch_spirit >= 0.95);
    assert!(state.min_ratio >= 0.9);
    let doc = json!({
        "stretch_spirit": state.stretch_spirit,
        "below_stretch_spirit": state.below_stretch_spirit,
        "stretch_spirit_gate_met": state.stretch_spirit_gate_met,
        "min_ratio": state.min_ratio,
        "meets_min_ratio": state.meets_min_ratio,
    });
    assert_eq!(ratio96_store_state(&doc), Some(state));
}
