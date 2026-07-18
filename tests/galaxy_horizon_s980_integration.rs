//! PH-S989: Galaxy horizon close band (PH-S980…S988) — STABLE product-complete draft.

use poolai_ui_core::stable_depth::{
    stable_depth_stub, StableDepth, HANDOFF_MAINTENANCE_MARKERS, INDEX_BAND33_ROWS,
    STABLE_BAND33_HEADERS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s980_band_stable_product_complete_close_ph_s989() {
    assert_eq!(
        stable_depth_stub(Some(&json!({"stable_draft": true}))),
        StableDepth::StableDraft
    );
    assert_eq!(
        stable_depth_stub(Some(&json!({
            "stable_draft": true,
            "index_zriz": true,
            "readme_maintenance": true,
            "handoff_template": true,
            "progress_scope": true
        }))),
        StableDepth::FullStable
    );

    let stable = include_str!("../docs/status/STABLE_STATE_SUMMARY.md");
    for header in STABLE_BAND33_HEADERS {
        assert!(
            stable.contains(header),
            "STABLE_STATE_SUMMARY missing band-33 marker {header}"
        );
    }
    assert!(stable.contains("PH-S980"));
    assert!(stable.contains("§5.15"));

    for row in INDEX_BAND33_ROWS {
        let index = include_str!("../docs/INDEX_2026-03-17.md");
        assert!(index.contains(row), "INDEX missing band-33 row {row}");
    }

    let readme = include_str!("../README.md");
    assert!(readme.contains("PH-S982"));
    assert!(readme.contains("maintenance"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    for marker in HANDOFF_MAINTENANCE_MARKERS {
        assert!(
            handoff.contains(marker),
            "HANDOFF missing maintenance marker {marker}"
        );
    }

    let progress = include_str!("../docs/status/DEVELOPMENT_PROGRESS_2026-05-19.md");
    assert!(progress.contains("PH-S984"));
    assert!(progress.contains("100%"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    assert!(fm.contains("§5.15"));
    assert!(fm.contains("product-complete"));

    assert!(Path::new("crates/poolai-ui-core/src/stable_depth.rs").exists());

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    let sprint = ratio_json["sprint"].as_str().unwrap();
    assert!(
        sprint == "PH-S1005" || sprint == "PH-S995",
        "rust_ratio sprint should reflect band 33–35 loc-audit zriz, got {sprint}"
    );
    assert!(ratio_json["in_formal_band"].as_bool().unwrap_or(false));

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S988"));
    assert!(joined.contains("PH-S998") || joined.contains("PH-S988"));
}
