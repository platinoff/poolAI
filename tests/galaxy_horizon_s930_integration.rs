//! PH-S939: Galaxy horizon close band (PH-S930…S938) — admin_common table/empty wasm-only + ratio 95% gate.

use poolai_ui_core::admin_common_depth::{admin_common_depth_stub, AdminCommonDepth};
use poolai_ui_core::table::{empty_state_html, render_table_html};
use serde_json::json;

#[test]
fn horizon_s930_band_admin_common_wasm_only_ph_s939() {
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

    let empty = empty_state_html("No jobs", Some("hint"), "📋", None);
    assert!(empty.contains("admin-empty-state"));
    assert!(empty.contains("admin-empty-state-hint"));

    let table = render_table_html(r#"["Col"]"#, r#"[["a"]]"#, "{}");
    assert!(table.contains("admin-table"));
    assert!(table.contains("Col"));

    let common_js = include_str!("../src/ui/admin_common.js");
    assert!(common_js.contains("poolaiUiWasmCall('emptyStateHtml')"));
    assert!(common_js.contains("poolaiUiWasmCall('renderTableHtml')"));
    assert!(common_js.contains("poolaiUiWasmCall('tableExportButtonsHtml')"));
    assert!(!common_js.contains("function poolaiT("));
    assert!(!common_js.contains("admin-empty-state-icon"));

    let i18n_js = include_str!("../src/ui/i18n_core.js");
    assert!(i18n_js.contains("function mergeRustI18nPatch"));
    assert!(i18n_js.contains("window.poolaiT = function"));
}
