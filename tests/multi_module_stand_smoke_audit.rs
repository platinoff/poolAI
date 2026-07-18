//! PH-S1002: Multi-module stand smoke full suite — `poolai-http-stand-smoke --json` canon.

use poolai_ui_core::multi_module_depth::{
    multi_module_depth_stub, MultiModuleDepth, MULTI_MODULE_BAND35_CANON, STAND_SMOKE_FULL_SUITE,
};
use serde_json::json;
use std::path::Path;

#[test]
fn multi_module_stand_smoke_full_suite_ph_s1002() {
    assert_eq!(
        multi_module_depth_stub(Some(&json!({"stand_smoke": true}))),
        MultiModuleDepth::StandSmoke
    );

    for (sprint, _, rust_canon) in MULTI_MODULE_BAND35_CANON {
        if *sprint == "PH-S1002" {
            assert_eq!(*rust_canon, "tests/multi_module_stand_smoke_audit.rs");
        }
    }

    let stand_smoke_src = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke_src.contains("--json"));
    assert!(stand_smoke_src.contains("stand_smoke_export_shape_regression_suite_ph_s834"));
    assert!(stand_smoke_src.contains("multi_module_stand_smoke_full_suite_ph_s1002"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("poolai-http-stand-smoke"));

    assert!(STAND_SMOKE_FULL_SUITE.contains("--json"));
    assert!(Path::new("src/bin/poolai_http_stand_smoke.rs").is_file());
}
