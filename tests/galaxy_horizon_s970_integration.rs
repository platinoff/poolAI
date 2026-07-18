//! PH-S979: Galaxy horizon close band (PH-S970…S978) — concept implemented markers.

use poolai_ui_core::concept_markers_depth::{
    concept_markers_depth_stub, ConceptMarkersDepth, CONCEPT_MARKER_BAND32_HEADERS,
    CONCEPT_ROADMAP_BAND32_ROWS, CONCEPT_TBD_BAND32_NOTES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s970_band_concept_markers_close_ph_s979() {
    assert_eq!(
        concept_markers_depth_stub(Some(&json!({"sections_1_3": true}))),
        ConceptMarkersDepth::Sections1To3
    );
    assert_eq!(
        concept_markers_depth_stub(Some(&json!({
            "sections_1_3": true,
            "sections_4_6": true,
            "sections_7_9": true
        }))),
        ConceptMarkersDepth::FullConcept
    );

    let galaxy = include_str!("../docs/concept/POOLAI_GALAXY_GRID.md");
    for header in CONCEPT_MARKER_BAND32_HEADERS {
        assert!(
            galaxy.contains(header),
            "POOLAI_GALAXY_GRID missing marker header {header}"
        );
    }
    assert!(galaxy.contains("PH-S970"));
    assert!(galaxy.contains("PH-S971"));
    assert!(galaxy.contains("PH-S972"));

    for note in CONCEPT_TBD_BAND32_NOTES {
        assert!(
            galaxy.contains(note),
            "POOLAI_GALAXY_GRID missing §8 TBD note {note}"
        );
    }

    let roadmap = include_str!("../docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md");
    assert!(roadmap.contains("PH-S970"));
    assert!(roadmap.contains("band 32"));
    for row in CONCEPT_ROADMAP_BAND32_ROWS {
        assert!(
            roadmap.contains(row),
            "GALAXY_GRID_ROADMAP missing band-32 row {row}"
        );
    }

    let index = include_str!("../docs/INDEX_2026-03-17.md");
    assert!(index.contains("POOLAI_GALAXY_GRID.md"));
    assert!(index.contains("PH-S976"));
    assert!(index.contains("band 32"));

    assert!(Path::new("crates/poolai-ui-core/src/concept_markers_depth.rs").exists());

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    let sprint = ratio_json["sprint"].as_str().unwrap();
    assert!(
        sprint == "PH-S1005" || sprint == "PH-S995",
        "rust_ratio sprint should reflect band 32–35 loc-audit zriz, got {sprint}"
    );
    assert!(ratio_json["in_formal_band"].as_bool().unwrap_or(false));

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S978"));
}
