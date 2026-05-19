//! FM-019: pa11y CI script declares auth fixture (no Node required).

#[test]
fn pa11y_ci_script_has_admin_strict_auth_actions() {
    let script = include_str!("../bin/pa11y-ci.sh");
    assert!(script.contains("PA11Y_ADMIN_STRICT"));
    assert!(script.contains("run_pa11y_authenticated"));
    assert!(script.contains("write_pa11y_config"));
    assert!(script.contains("write_pa11y_simple_config"));
    assert!(script.contains("PA11Y_PASSWORD"));
    assert!(script.contains("validate_pa11y_standard"));
    assert!(script.contains("PA11Y_WCAG22"));
    assert!(script.contains("wcag22aa"));
    assert!(script.contains("${BASE}/ui/admin/config"));
    let admin_urls = script
        .split("ADMIN_URLS=(")
        .nth(1)
        .and_then(|s| s.split(')').next())
        .expect("ADMIN_URLS block");
    assert!(admin_urls.contains("\"${BASE}/ui\""));
    const ADMIN_PATHS: &[&str] = &[
        "/ui",
        "/ui/status",
        "/ui/health",
        "/ui/metrics",
        "/ui/admin",
        "/ui/admin/users",
        "/ui/admin/security",
        "/ui/admin/config",
        "/ui/admin/tenants",
        "/ui/admin/audit",
        "/ui/admin/monitoring",
        "/ui/admin/instances",
        "/ui/admin/topology",
        "/ui/workers",
        "/ui/libs",
        "/ui/vm",
        "/ui/raid",
    ];
    for path in ADMIN_PATHS {
        assert!(
            admin_urls.contains(&format!("\"${{BASE}}{path}\"")),
            "missing ADMIN_URLS entry for {path}"
        );
    }
    let url_lines = admin_urls.lines().filter(|l| l.contains("${BASE}")).count();
    assert_eq!(
        url_lines,
        ADMIN_PATHS.len(),
        "ADMIN_URLS must list exactly {} auth paths (runbook §3.1)",
        ADMIN_PATHS.len()
    );
}
