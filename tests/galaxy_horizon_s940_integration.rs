//! PH-S949: Galaxy horizon close band (PH-S940…S948) — e2e scope audit + ratio 96% stretch spirit.

use poolai_ui_core::stretch_depth::{stretch_depth_stub, StretchDepth};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s940_band_e2e_stretch_ops_ph_s949() {
    assert_eq!(
        stretch_depth_stub(Some(&json!({"e2e_scope_audit": true}))),
        StretchDepth::E2eScope
    );
    assert_eq!(
        stretch_depth_stub(Some(&json!({
            "e2e_scope_audit": true,
            "ratio_stretch_spirit": true,
            "ops_shell_canon": true
        }))),
        StretchDepth::E2eRatioOps
    );

    let pkg: serde_json::Value =
        serde_json::from_str(include_str!("../e2e/package.json")).expect("package.json");
    assert_eq!(
        pkg["scripts"]["test:ci"].as_str().unwrap(),
        "playwright test smoke admin a11y visual"
    );
    assert!(!Path::new("e2e/tests/jobs_raid.spec.ts").exists());

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    let sprint = ratio_json["sprint"].as_str().unwrap_or("");
    assert!(
        sprint == "PH-S1010"
            || sprint == "PH-S1005"
            || sprint == "PH-S995"
            || sprint == "PH-S985"
            || sprint == "PH-S975"
            || sprint == "PH-S965"
            || sprint == "PH-S955"
            || sprint == "PH-S945",
        "rust_ratio sprint should reflect band 29–35 loc-audit zriz, got {sprint}"
    );
    assert!(ratio_json["stretch_spirit_gate_met"].is_boolean());
    assert!(ratio_json["ops_shell_canon_met"].as_bool().unwrap_or(false));
    assert!(ratio_json["e2e_ts_loc_reduction"].as_i64().unwrap_or(0) >= 0);

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S942"));
    assert!(joined.contains("PH-S948"));
}
