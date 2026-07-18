//! Multi-module horizon band depth (PH-S1000, band 35).

use serde_json::Value;

/// Multi-module band depth flags (wire smoke / admin wasm / stand smoke / docs gates).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiModuleDepth {
    None,
    WireSmoke,
    AdminWasm,
    StandSmoke,
    TestCiScope,
    DualGate,
    FullMultiModule,
}

/// Top 5 grid metrics APIs — single harness (PH-S1000).
pub const MULTI_MODULE_BAND35_TOP5_GRID_APIS: &[&str] = &[
    "/api/v1/grid/verification-metrics",
    "/api/v1/grid/settlement-metrics",
    "/api/v1/grid/trust-metrics",
    "/api/v1/grid/replication-metrics",
    "/api/v1/grid/pricing-metrics",
];

/// Multi-module band canon (PH-S1000…S1002).
pub const MULTI_MODULE_BAND35_CANON: &[(&str, &str, &str)] = &[
    (
        "PH-S1000",
        "multi_module_wire_smoke",
        "tests/multi_module_wire_smoke.rs",
    ),
    (
        "PH-S1001",
        "admin_wasm_regression",
        "tests/multi_module_admin_wasm_regression.rs",
    ),
    (
        "PH-S1002",
        "stand_smoke_full_suite",
        "tests/multi_module_stand_smoke_audit.rs",
    ),
];

/// ui-core full test gate (PH-S662 / PH-S1001).
pub const UI_CORE_FULL_TEST_GATE: &str = "cargo test -p poolai-ui-core";

/// Stand smoke JSON export full suite (PH-S1002).
pub const STAND_SMOKE_FULL_SUITE: &str = "poolai-http-stand-smoke --json";

/// GALAXY_GRID_ROADMAP band-35 marker rows (PH-S1004 docs cross-link).
pub const MULTI_MODULE_BAND35_ROWS: &[&str] = &[
    "band 35 PH-S1000",
    "final multi-module horizon",
    "multi_module_wire_smoke.rs",
    "multi_module_admin_wasm_regression.rs",
    "multi_module_stand_smoke_audit.rs",
    "multi_module_depth.rs",
];

/// Classify multi-module band depth from optional feature stub (PH-S1000).
pub fn multi_module_depth_stub(features: Option<&Value>) -> MultiModuleDepth {
    let Some(f) = features else {
        return MultiModuleDepth::None;
    };
    let wire = f
        .get("wire_smoke")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let wasm = f
        .get("admin_wasm")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let stand = f
        .get("stand_smoke")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let test_ci = f
        .get("test_ci_scope")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dual = f
        .get("dual_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = wire as u8 + wasm as u8 + stand as u8 + test_ci as u8 + dual as u8;
    match flags {
        0 => MultiModuleDepth::None,
        1 if wire => MultiModuleDepth::WireSmoke,
        1 if wasm => MultiModuleDepth::AdminWasm,
        1 if stand => MultiModuleDepth::StandSmoke,
        1 if test_ci => MultiModuleDepth::TestCiScope,
        1 if dual => MultiModuleDepth::DualGate,
        5 => MultiModuleDepth::FullMultiModule,
        _ => MultiModuleDepth::FullMultiModule,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn multi_module_depth_stub_ph_s1000() {
        assert_eq!(multi_module_depth_stub(None), MultiModuleDepth::None);
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
        assert_eq!(MULTI_MODULE_BAND35_TOP5_GRID_APIS.len(), 5);
        assert_eq!(MULTI_MODULE_BAND35_CANON.len(), 3);
        assert_eq!(MULTI_MODULE_BAND35_ROWS.len(), 6);
    }
}
