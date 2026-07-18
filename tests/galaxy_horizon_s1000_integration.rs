//! PH-S999: Galaxy horizon close band (PH-S1000…S1008) — final multi-module horizon.

use poolai_ui_core::multi_module_depth::{
    multi_module_depth_stub, MultiModuleDepth, MULTI_MODULE_BAND35_CANON, MULTI_MODULE_BAND35_ROWS,
    MULTI_MODULE_BAND35_TOP5_GRID_APIS, STAND_SMOKE_FULL_SUITE, UI_CORE_FULL_TEST_GATE,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1000_band_multi_module_horizon_close_ph_s1009() {
    assert_eq!(
        multi_module_depth_stub(Some(&json!({"wire_smoke": true}))),
        MultiModuleDepth::WireSmoke
    );
    assert_eq!(
        multi_module_depth_stub(Some(&json!({
            "wire_smoke": true,
            "admin_wasm": true,
            "stand_smoke": true,
            "test_ci_scope": true,
            "dual_gate": true
        }))),
        MultiModuleDepth::FullMultiModule
    );

    for (sprint, _, rust_canon) in MULTI_MODULE_BAND35_CANON {
        assert!(
            Path::new(rust_canon).is_file(),
            "{sprint}: missing Rust canon {rust_canon}"
        );
    }

    let roadmap = include_str!("../docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md");
    for row in MULTI_MODULE_BAND35_ROWS {
        assert!(
            roadmap.contains(row),
            "GALAXY_GRID_ROADMAP missing band-35 row {row}"
        );
    }
    assert!(roadmap.contains("PH-S1000"));
    assert!(roadmap.contains("band 35"));

    let policy = include_str!("../.cursor/rules/poolai-testing-policy.mdc");
    assert!(policy.contains("band 35"));
    assert!(policy.contains("multi_module_wire_smoke.rs"));
    assert!(policy.contains(UI_CORE_FULL_TEST_GATE));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1003"));
    assert!(handoff.contains("cargo test-ci scope"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    assert!(fm.contains("PH-S1004"));
    assert!(fm.contains("dual gate"));

    assert!(Path::new("tests/multi_module_wire_smoke.rs").exists());
    assert_eq!(MULTI_MODULE_BAND35_TOP5_GRID_APIS.len(), 5);
    assert!(STAND_SMOKE_FULL_SUITE.contains("stand-smoke"));

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert_eq!(ratio_json["sprint"].as_str().unwrap(), "PH-S1010");
    assert!(
        ratio_json["ratio_95_formal_gate_met"]
            .as_bool()
            .unwrap_or(false),
        "ratio_95_formal_gate_met expected true at band 35 close"
    );

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S1008"));
}
