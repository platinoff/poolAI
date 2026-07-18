//! PH-S1001: Multi-module admin wasm regression — ui-core full test gate.

use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::multi_module_depth::{
    multi_module_depth_stub, MultiModuleDepth, MULTI_MODULE_BAND35_CANON, UI_CORE_FULL_TEST_GATE,
};
use serde_json::json;
use std::path::Path;

#[test]
fn multi_module_admin_wasm_regression_ui_core_gate_ph_s1001() {
    assert_eq!(
        multi_module_depth_stub(Some(&json!({"admin_wasm": true}))),
        MultiModuleDepth::AdminWasm
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
        AdminWasmSlimDepth::PanelRenderer
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"charts_glue": true}))),
        AdminWasmSlimDepth::ChartsGlue
    );

    for (sprint, _, rust_canon) in MULTI_MODULE_BAND35_CANON {
        if *sprint == "PH-S1001" {
            assert_eq!(*rust_canon, "tests/multi_module_admin_wasm_regression.rs");
        }
    }

    let ui_core_lib = include_str!("../crates/poolai-ui-core/src/lib.rs");
    assert!(ui_core_lib.contains("multi_module_depth"));
    assert!(ui_core_lib.contains("grid_replication_pricing"));

    let cargo_toml = include_str!("../crates/poolai-ui-core/Cargo.toml");
    assert!(cargo_toml.contains("name = \"poolai-ui-core\""));

    let policy = include_str!("../.cursor/rules/poolai-testing-policy.mdc");
    assert!(policy.contains(UI_CORE_FULL_TEST_GATE));

    assert!(Path::new("crates/poolai-ui-core/src/multi_module_depth.rs").is_file());
}
