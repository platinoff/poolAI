//! Admin common wasm depth classification (PH-S930/S931, band 28).

use serde_json::Value;

/// Admin common table/empty-state wasm glue depth (band 28).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminCommonDepth {
    None,
    TableInitGlue,
    EmptyStateGlue,
    TableInitEmptyGlue,
}

/// Classify admin common wasm depth from optional feature stub (PH-S930/S931).
pub fn admin_common_depth_stub(features: Option<&Value>) -> AdminCommonDepth {
    let Some(f) = features else {
        return AdminCommonDepth::None;
    };
    let table = f
        .get("table_init_glue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let empty = f
        .get("empty_state_glue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    match (table, empty) {
        (true, true) => AdminCommonDepth::TableInitEmptyGlue,
        (true, false) => AdminCommonDepth::TableInitGlue,
        (false, true) => AdminCommonDepth::EmptyStateGlue,
        (false, false) => AdminCommonDepth::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn admin_common_depth_stub_ph_s930_s931() {
        assert_eq!(admin_common_depth_stub(None), AdminCommonDepth::None);
        assert_eq!(
            admin_common_depth_stub(Some(&json!({"table_init_glue": true}))),
            AdminCommonDepth::TableInitGlue
        );
        assert_eq!(
            admin_common_depth_stub(Some(&json!({"empty_state_glue": true}))),
            AdminCommonDepth::EmptyStateGlue
        );
        assert_eq!(
            admin_common_depth_stub(Some(&json!({
                "table_init_glue": true,
                "empty_state_glue": true
            }))),
            AdminCommonDepth::TableInitEmptyGlue
        );
    }
}
