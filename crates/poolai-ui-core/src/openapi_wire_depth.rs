//! OpenAPI/docs wire sync band depth (PH-S1059…S1068, band 42).

use serde_json::Value;

/// Band-42 OpenAPI wire sync depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenapiWireDepth {
    None,
    GapAuditGate,
    GridContracts,
    MemoryContracts,
    StandSmokeOpsPower,
    OpenapiExamples,
    DocsCanon,
    FullOpenapiWire,
}

/// FM §5.23 band-42 marker rows.
pub const FM_BAND42_ROWS: &[&str] = &["5.23", "OpenAPI/docs wire", "PH-S1059…S1068", "contracts"];

/// OpenAPI wire adoption markers for band 42.
pub const OPENAPI_WIRE_BAND42_ROWS: &[&str] = &[
    "PH-S1059",
    "poolai-openapi-gap-audit",
    "PH-S1060",
    "grid_openapi_contracts",
    "PH-S1061",
    "memory_api_contracts",
    "PH-S1062",
    "ops_power_openapi",
    "PH-S1068",
];

pub fn openapi_wire_depth_stub(features: Option<&Value>) -> OpenapiWireDepth {
    let Some(f) = features else {
        return OpenapiWireDepth::None;
    };
    let gap = f
        .get("gap_audit_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let grid = f
        .get("grid_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let memory = f
        .get("memory_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_ops_power")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let examples = f
        .get("openapi_examples")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("docs_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [gap, grid, memory, smoke, examples, docs];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => OpenapiWireDepth::None,
        6 => OpenapiWireDepth::FullOpenapiWire,
        _ if gap && !grid => OpenapiWireDepth::GapAuditGate,
        _ if grid => OpenapiWireDepth::GridContracts,
        _ if memory => OpenapiWireDepth::MemoryContracts,
        _ if smoke => OpenapiWireDepth::StandSmokeOpsPower,
        _ if examples => OpenapiWireDepth::OpenapiExamples,
        _ if docs => OpenapiWireDepth::DocsCanon,
        _ => OpenapiWireDepth::FullOpenapiWire,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn openapi_wire_depth_stub_ph_s1067() {
        assert_eq!(openapi_wire_depth_stub(None), OpenapiWireDepth::None);
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
    }
}
