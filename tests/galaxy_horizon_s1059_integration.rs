//! PH-S1068: Galaxy horizon close band 42 — OpenAPI/docs wire sync.

use poolai_ui_core::openapi_wire_depth::{
    openapi_wire_depth_stub, OpenapiWireDepth, FM_BAND42_ROWS, OPENAPI_WIRE_BAND42_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1059_band_openapi_wire_close_ph_s1068() {
    assert_eq!(
        openapi_wire_depth_stub(Some(&json!({"gap_audit_gate": true}))),
        OpenapiWireDepth::GapAuditGate
    );
    assert_eq!(
        openapi_wire_depth_stub(Some(&json!({
            "gap_audit_gate": true,
            "grid_contracts": true,
            "memory_contracts": true,
            "stand_smoke_ops_power": true,
            "openapi_examples": true,
            "docs_canon": true
        }))),
        OpenapiWireDepth::FullOpenapiWire
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND42_ROWS {
        assert!(fm.contains(row), "FM missing band-42 row {row}");
    }
    assert!(fm.contains("PH-S1068"));
    assert!(fm.contains("5.23"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1059") || handoff.contains("band 42"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("/ops/power"));
    assert!(openapi.contains("OpsPowerRequest"));
    assert!(openapi.contains("/memory/shards"));
    assert!(openapi.contains("GridSeedInventoryResponse"));

    let grid_contracts = include_str!("../tests/grid_openapi_contracts.rs");
    assert!(grid_contracts.contains("ph_s1060"));

    let memory_contracts = include_str!("../tests/memory_api_contracts.rs");
    assert!(memory_contracts.contains("ph_s1061"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("smoke_ops_power_openapi"));

    for marker in OPENAPI_WIRE_BAND42_ROWS {
        assert!(
            fm.contains(marker)
                || grid_contracts.contains(marker)
                || memory_contracts.contains(marker)
                || stand_smoke.contains(marker)
                || openapi.contains(marker),
            "band-42 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/openapi_wire_depth.rs").exists());
    assert!(Path::new("tests/memory_api_contracts.rs").exists());
}
