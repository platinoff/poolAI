//! PH-S1048: Galaxy horizon close band 40 — Vision map/a11y/perf.

use poolai_ui_core::vision_map_depth::{
    vision_map_depth_stub, VisionMapDepth, FM_BAND40_ROWS, VISION_MAP_BAND40_ROWS,
};
use std::path::Path;

#[test]
fn horizon_s1039_band_vision_map_close_ph_s1048() {
    assert_eq!(
        vision_map_depth_stub(Some(&serde_json::json!({"skip_links_landmarks": true}))),
        VisionMapDepth::SkipLinksLandmarks
    );
    assert_eq!(
        vision_map_depth_stub(Some(&serde_json::json!({
            "skip_links_landmarks": true,
            "icon_aria_labels": true,
            "explorer_tree_keyboard": true,
            "link_graph_a11y": true,
            "map_filter_incremental": true,
            "dense_map_lod": true,
            "background_tab_perf": true
        }))),
        VisionMapDepth::FullVisionMap
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND40_ROWS {
        assert!(fm.contains(row), "FM missing band-40 row {row}");
    }
    assert!(fm.contains("PH-S1048"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1039") || handoff.contains("band 40"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let index = include_str!("../GSV/docs/vision/index.html");
    assert!(index.contains("superseded by GSV"));
    assert!(!index.contains("vision-skip-link"));
    assert!(!index.contains("role=\"tree\""));

    let vision_js = include_str!("../GSV/docs/vision/vision.js");
    assert!(vision_js.contains("updateMapSprintDim"));
    assert!(vision_js.contains("initTreeKeyboardNav"));
    assert!(vision_js.contains("link-neighbor"));
    assert!(vision_js.contains("initVisionVisibilityPerf"));

    for marker in VISION_MAP_BAND40_ROWS {
        assert!(
            index.contains(marker) || vision_js.contains(marker) || fm.contains(marker),
            "band-40 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/vision_map_depth.rs").exists());
}
