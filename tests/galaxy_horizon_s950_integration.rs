//! PH-S959: Galaxy horizon close band (PH-S950…S958) — FUNCTIONALITY_DIGEST full sync.

use poolai_ui_core::digest_depth::{
    digest_depth_stub, DigestDepth, GRID_MODULE_STEMS, JOB_MODULE_STEMS,
};
use serde_json::json;

#[test]
fn horizon_s950_band_digest_full_sync_ph_s959() {
    assert_eq!(
        digest_depth_stub(Some(&json!({"grid_digest": true}))),
        DigestDepth::Grid
    );
    assert_eq!(
        digest_depth_stub(Some(&json!({
            "grid_digest": true,
            "job_digest": true,
            "ui_wasm_digest": true,
            "bins_digest": true
        }))),
        DigestDepth::FullDigest
    );

    let digest = include_str!("../docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md");

    for stem in GRID_MODULE_STEMS {
        let path = format!("src/grid/{stem}.rs");
        assert!(
            digest.contains(&path),
            "FUNCTIONALITY_DIGEST missing grid module {path}"
        );
    }
    for stem in JOB_MODULE_STEMS {
        if *stem == "mod" {
            continue;
        }
        let path = format!("src/job/{stem}.rs");
        assert!(
            digest.contains(&path),
            "FUNCTIONALITY_DIGEST missing job module {path}"
        );
    }

    assert!(digest.contains("crates/poolai-ui-core"));
    assert!(digest.contains("crates/poolai-ui-wasm"));
    assert!(digest.contains("poolai-openapi-gap-audit"));
    assert!(digest.contains("poolai-http-stand-smoke"));
    assert!(digest.contains("poolai-loc-audit"));
    assert!(digest.contains("poolai-vision-sync"));

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    let sprint = ratio_json["sprint"].as_str().unwrap_or("");
    assert!(
        sprint == "PH-S975" || sprint == "PH-S965" || sprint == "PH-S955",
        "rust_ratio sprint should reflect band 30–32 loc-audit zriz, got {sprint}"
    );
    assert!(ratio_json["in_formal_band"].as_bool().unwrap_or(false));

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S958"));
}
