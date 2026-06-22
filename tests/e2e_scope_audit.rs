//! PH-S940: e2e scope audit — browser-only `test:ci`; API-only specs archived; Rust wire canon.

use std::path::Path;

const RUST_WIRE_CANON: &[(&str, &str)] = &[
    ("jobs_lease.spec.ts", "tests/jobs_api_contracts.rs"),
    ("jobs_migrating.spec.ts", "tests/jobs_api_contracts.rs"),
    (
        "protocol_middleware.spec.ts",
        "tests/protocol_middleware_integration.rs",
    ),
    (
        "telegram_wallet.spec.ts",
        "tests/virtual_node_telegram_binding_integration.rs",
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
