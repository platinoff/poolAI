//! PH-S1010: Galaxy horizon close band 36 — FM §5.15 product-complete closure.

use poolai_ui_core::product_complete_depth::{
    product_complete_depth_stub, ProductCompleteDepth, FM_BAND36_ROWS, GALAXY_BAND36_ROWS,
    HANDOFF_MAINTENANCE_ACTIVE, STABLE_BAND36_HEADERS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1010_band_product_complete_closure_ph_s1010() {
    assert_eq!(
        product_complete_depth_stub(Some(&json!({"stable_final": true}))),
        ProductCompleteDepth::StableFinal
    );
    assert_eq!(
        product_complete_depth_stub(Some(&json!({
            "stable_final": true,
            "handoff_maintenance": true,
            "fm_closure": true,
            "ratio_formal": true
        }))),
        ProductCompleteDepth::FullProductComplete
    );

    let stable = include_str!("../docs/status/STABLE_STATE_SUMMARY.md");
    for header in STABLE_BAND36_HEADERS {
        assert!(
            stable.contains(header),
            "STABLE_STATE_SUMMARY missing band-36 marker {header}"
        );
    }
    assert!(!stable.contains("Development complete (draft"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    for marker in HANDOFF_MAINTENANCE_ACTIVE {
        assert!(
            handoff.contains(marker),
            "HANDOFF missing maintenance marker {marker}"
        );
    }

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND36_ROWS {
        assert!(fm.contains(row), "FM missing band-36 row {row}");
    }
    assert!(fm.contains("PH-S1010") && fm.contains("**✅**"));

    let roadmap = include_str!("../docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md");
    for row in GALAXY_BAND36_ROWS {
        assert!(
            roadmap.contains(row),
            "GALAXY_GRID_ROADMAP missing band-36 row {row}"
        );
    }

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("Maintenance mode"));
    assert!(!next.contains("абракадабра`** (drain PH-S1010"));

    let progress = include_str!("../docs/status/DEVELOPMENT_PROGRESS_2026-05-19.md");
    assert!(progress.contains("PH-S1010"));
    assert!(progress.contains("100%"));

    assert!(Path::new("crates/poolai-ui-core/src/product_complete_depth.rs").exists());

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert_eq!(ratio_json["sprint"].as_str().unwrap(), "PH-S1010");
    let gate_met = ratio_json["ratio_95_formal_gate_met"]
        .as_bool()
        .unwrap_or(false);
    let advisory_hold = ratio_json["migration_advisory_mode"]
        .as_bool()
        .unwrap_or(false)
        && ratio_json["migration_candidate_total"]
            .as_u64()
            .unwrap_or(0)
            > 0;
    assert!(
        gate_met || advisory_hold,
        "ratio_95_formal_gate_met or band-46 migration advisory hold at PH-S1010 zriz"
    );

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S1010"));
}
