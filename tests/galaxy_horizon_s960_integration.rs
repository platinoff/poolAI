//! PH-S969: Galaxy horizon close band (PH-S960…S968) — DOCS_LEGACY audit close.

use poolai_ui_core::docs_legacy_depth::{
    docs_legacy_depth_stub, DocsLegacyDepth, FLAT_LEGACY_DOC_SAMPLES, LEGACY_AUDIT_BAND31_ROWS,
};
use serde_json::json;
use std::path::Path;

const STALE_BANNER: &str = "Stale / не канон (2026-07-17, PH-S961)";

#[test]
fn horizon_s960_band_docs_legacy_close_ph_s969() {
    assert_eq!(
        docs_legacy_depth_stub(Some(&json!({"legacy_audit": true}))),
        DocsLegacyDepth::LegacyAudit
    );
    assert_eq!(
        docs_legacy_depth_stub(Some(&json!({
            "legacy_audit": true,
            "flat_banners": true,
            "concept_dehype": true,
            "architect_sync": true
        }))),
        DocsLegacyDepth::FullLegacy
    );

    let legacy_audit = include_str!("../docs/development/DOCS_LEGACY_AUDIT_2026-05-19.md");
    assert!(legacy_audit.contains("PH-S960"));
    assert!(legacy_audit.contains("band 31"));
    for row in LEGACY_AUDIT_BAND31_ROWS {
        assert!(
            legacy_audit.contains(row),
            "DOCS_LEGACY_AUDIT missing band-31 row {row}"
        );
    }

    for doc in FLAT_LEGACY_DOC_SAMPLES {
        let path = format!("docs/{doc}");
        assert!(Path::new(&path).exists(), "missing flat legacy doc {path}");
        let content = std::fs::read_to_string(&path).expect("read flat legacy doc");
        assert!(
            content.contains(STALE_BANNER),
            "{path} missing PH-S961 stale banner"
        );
    }

    let concept = include_str!("../docs/concept/poolAI_concept_root.txt");
    assert!(concept.contains("PH-S962"));
    assert!(concept.contains("de-hype"));

    let architect = include_str!("../docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md");
    assert!(architect.contains("PH-S963"));
    assert!(architect.contains("FM §5.1"));

    let index = include_str!("../docs/INDEX_2026-03-17.md");
    assert!(index.contains("FUNCTION_MANAGEMENT.md"));
    assert!(index.contains("§5.12"));

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    let sprint = ratio_json["sprint"].as_str().unwrap();
    assert!(
        sprint == "PH-S1010"
            || sprint == "PH-S1005"
            || sprint == "PH-S995"
            || sprint == "PH-S985"
            || sprint == "PH-S975"
            || sprint == "PH-S965",
        "rust_ratio sprint should reflect band 31–35 loc-audit zriz, got {sprint}"
    );
    assert!(
        ratio_json["in_formal_band"].as_bool().unwrap_or(false)
            || ratio_json["ratio_95_formal_gate_met"]
                .as_bool()
                .unwrap_or(false)
    );

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S968"));
}
