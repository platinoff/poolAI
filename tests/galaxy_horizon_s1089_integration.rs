//! PH-S1098: Galaxy horizon close band 45 — stand smoke + RUN_LOCAL ops.

use poolai_ui_core::stand_smoke_run_local_depth::{
    stand_smoke_run_local_depth_stub, StandSmokeRunLocalDepth, FM_BAND45_ROWS,
    RUN_LOCAL_HEALTH_KEYS, RUN_LOCAL_SMOKE_CASES, STAND_SMOKE_RUN_LOCAL_BAND45_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1089_band_stand_smoke_run_local_close_ph_s1098() {
    assert_eq!(
        stand_smoke_run_local_depth_stub(Some(&json!({"health_shape": true}))),
        StandSmokeRunLocalDepth::HealthShape
    );
    assert_eq!(
        stand_smoke_run_local_depth_stub(Some(&json!({
            "run_local_smoke": true,
            "verify_dev_stand_hook": true,
            "quick_stand_smoke": true,
        }))),
        StandSmokeRunLocalDepth::FullRunLocalBand45
    );

    for key in RUN_LOCAL_HEALTH_KEYS {
        assert!(!key.is_empty());
    }
    assert_eq!(RUN_LOCAL_SMOKE_CASES.len(), 6);
    assert!(RUN_LOCAL_SMOKE_CASES.contains(&"monitoring_dashboards"));
    assert!(RUN_LOCAL_SMOKE_CASES.contains(&"jobs_store_backend"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND45_ROWS {
        assert!(fm.contains(row), "FM missing band-45 row {row}");
    }
    assert!(fm.contains("PH-S1098"));
    assert!(fm.contains("5.26"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1089") || handoff.contains("band 45"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--run-local-smoke"));
    assert!(run_local.contains("VERIFY_STAND_SMOKE"));
    assert!(run_local.contains("--stand-smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_STAND_SMOKE"));
    assert!(verify.contains("--run-local-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--stand-smoke"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("run_local_health_export_shape_ph_s1089"));
    assert!(stand_smoke.contains("smoke_monitoring_alerts_api"));
    assert!(stand_smoke.contains("stand_smoke_run_local_band45_export_shape_ph_s1095"));

    for marker in STAND_SMOKE_RUN_LOCAL_BAND45_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || stand_smoke.contains(marker),
            "band-45 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/stand_smoke_run_local_depth.rs").exists());
}
