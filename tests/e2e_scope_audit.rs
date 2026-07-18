//! PH-S940: e2e scope audit — browser-only `test:ci`; API-only specs archived; Rust wire canon.
//! PH-S1055: visual/axe admin route parity gate (band 41).

use std::path::Path;

/// Tier-1 visual routes added in PH-S1049 (also covered by axe).
const VISUAL_PARITY_TIER1: &[(&str, &str)] =
    &[("/ui/admin/config", "config"), ("/ui/admin/jobs", "jobs")];

/// Tier-2 grid panel visual routes (PH-S1050).
const VISUAL_PARITY_TIER2: &[(&str, &str)] = &[
    ("/ui/admin/updates-compat", "updates-compat"),
    ("/ui/admin/seed-inventory", "seed-inventory"),
    ("/ui/admin/security-advisories", "security-advisories"),
];

const RUST_WIRE_CANON: &[(&str, &str)] = &[
    ("jobs_lease.spec.ts", "tests/jobs_api_contracts.rs"),
    ("jobs_migrating.spec.ts", "tests/jobs_api_contracts.rs"),
    (
        "protocol_middleware.spec.ts",
        "tests/protocol_middleware_integration.rs",
    ),
    (
        "telegram_wallet.spec.ts",
        "tests/telegram_wallet_integration.rs",
    ),
    ("grid_pricing.spec.ts", "tests/grid_pricing_integration.rs"),
    (
        "grid_job_lease.spec.ts",
        "tests/grid_envelope_lease_integration.rs",
    ),
    (
        "grid_result_lease.spec.ts",
        "tests/grid_envelope_lease_integration.rs",
    ),
    ("jobs_raid.spec.ts", "tests/job_store_raid_persistence.rs"),
];

#[test]
fn e2e_scope_audit_ph_s940() {
    let pkg: serde_json::Value =
        serde_json::from_str(include_str!("../e2e/package.json")).expect("package.json");
    let test_ci = pkg["scripts"]["test:ci"].as_str().expect("test:ci script");
    assert_eq!(
        test_ci, "playwright test smoke admin a11y visual",
        "test:ci must be browser-only (no API specs)"
    );

    let active_specs = [
        "smoke.spec.ts",
        "admin.spec.ts",
        "a11y.spec.ts",
        "visual.spec.ts",
        "vision.spec.ts",
    ];
    for spec in active_specs {
        let path = format!("e2e/tests/{spec}");
        assert!(
            Path::new(&path).is_file(),
            "active browser spec missing: {path}"
        );
    }

    assert!(
        !Path::new("e2e/tests/jobs_raid.spec.ts").exists(),
        "jobs_raid must be archived (PH-S941); use Rust stand smoke"
    );

    for (archived, rust_canon) in RUST_WIRE_CANON {
        let archive_path = format!("e2e/archive/api-smoke/{archived}");
        assert!(
            Path::new(&archive_path).is_file(),
            "archived API spec missing: {archive_path}"
        );
        assert!(
            Path::new(rust_canon).is_file(),
            "Rust wire canon missing for {archived}: {rust_canon}"
        );
    }

    let archive_readme = include_str!("../e2e/archive/api-smoke/README.md");
    assert!(archive_readme.contains("Do not add new API-only Playwright specs"));
}

#[test]
fn e2e_visual_axe_parity_ph_s1055() {
    let visual = include_str!("../e2e/tests/visual.spec.ts");
    let a11y = include_str!("../e2e/tests/a11y.spec.ts");
    let helpers = include_str!("../e2e/tests/helpers.ts");

    assert!(
        helpers.contains("waitForVisualSnapshotReady"),
        "PH-S1054 visual snapshot helper missing"
    );
    assert!(
        helpers.contains("waitForAdminVisualReady"),
        "PH-S1050 admin visual ready helper missing"
    );
    assert!(
        a11y.contains("axe vision map (PH-S1051)"),
        "vision axe smoke missing"
    );

    for (path, name) in VISUAL_PARITY_TIER1.iter().chain(VISUAL_PARITY_TIER2.iter()) {
        assert!(
            visual.contains(path),
            "visual.spec missing route {path} ({name})"
        );
        assert!(
            a11y.contains(path),
            "a11y.spec missing route {path} ({name})"
        );
        let snapshot = format!("e2e/tests/visual.spec.ts-snapshots/{name}.png");
        assert!(
            Path::new(&snapshot).is_file(),
            "visual baseline missing: {snapshot}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/e2e_visual_axe_depth.rs").is_file());
}
