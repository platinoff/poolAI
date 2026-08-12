//! PH-S1888: Galaxy horizon close band 124 — GPULimits admin/ops glue.
//! Suite: `galaxy_horizon_s1879_integration`.

use poolai_ui_core::gpu_limits_admin_ops_depth::{
    gpu_limits_admin_ops_criteria_total, gpu_limits_admin_ops_depth_stub, GpuLimitsAdminOpsDepth,
    FM_BAND124_ROWS, GPU_LIMITS_ADMIN_OPS_BAND124_ROWS, GPU_LIMITS_ADMIN_OPS_CASES,
    GPU_LIMITS_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::gpu_limits_store::{gpu_limits_store_wire_json, GPU_LIMITS_STORE_PATH};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1879_band_gpu_limits_admin_ops_close_ph_s1888() {
    assert_eq!(
        gpu_limits_admin_ops_depth_stub(Some(&json!({"gpu_limits_admin_ops_depth": true}))),
        GpuLimitsAdminOpsDepth::DepthModule
    );
    assert_eq!(
        gpu_limits_admin_ops_depth_stub(Some(&json!({
            "gpu_limits_admin_ops_depth": true,
            "store_strip": true,
            "query_ops_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "docs_canon": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        GpuLimitsAdminOpsDepth::FullBand124
    );

    assert_eq!(GPU_LIMITS_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(gpu_limits_admin_ops_criteria_total(), 10);
    assert!(GPU_LIMITS_ADMIN_OPS_CASES.contains(&"docs_canon"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND124_ROWS {
        assert!(fm.contains(row), "FM missing band-124 row {row}");
    }
    assert!(fm.contains("PH-S1888"));
    assert!(fm.contains("5.105"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1879") || handoff.contains("band 124"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 124"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--gpu-limits-admin-ops"));
    assert!(run_local.contains("VERIFY_GPU_LIMITS_ADMIN_OPS"));

    let canon_doc = include_str!("../docs/development/GPU_LIMITS_ADMIN_OPS.md");
    assert!(canon_doc.contains("gpu-limits-store-badge"));
    assert!(canon_doc.contains("/api/v1/gpu-limits"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_GPU_LIMITS_ADMIN_OPS"));
    assert!(verify.contains("--gpu-limits-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--gpu-limits-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("gpu_limits_admin_ops_mode"));
    assert!(loc_audit.contains("gpu_limits_admin_ops_criteria_met_count"));

    let dash = include_str!("../src/ui/admin/dashboard.rs");
    assert!(dash.contains("gpu-limits-store-badge"));
    assert!(dash.contains("refreshGpuLimits"));

    for marker in GPU_LIMITS_ADMIN_OPS_BAND124_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || dash.contains(marker)
                || verify.contains(marker)
                || canon_doc.contains(marker),
            "band-124 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/gpu_limits_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/GPU_LIMITS_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/gpu_limits_admin_ops_integration.rs").exists());

    let wire = gpu_limits_store_wire_json();
    assert!(wire.get("mode").is_some());
    assert!(wire.get("admission_active").is_some());
    assert!(Path::new(GPU_LIMITS_STORE_PATH).exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("gpu_limits_admin_ops_mode").is_some());
    assert!(ratio.get("gpu_limits_admin_ops_criteria_total").is_some());
}
