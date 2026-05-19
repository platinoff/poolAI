//! FM-019: pa11y CI script declares auth fixture (no Node required).

#[test]
fn pa11y_ci_script_has_admin_strict_auth_actions() {
    let script = include_str!("../bin/pa11y-ci.sh");
    assert!(script.contains("PA11Y_ADMIN_STRICT"));
    assert!(script.contains("run_pa11y_authenticated"));
    assert!(script.contains("write_pa11y_config"));
    assert!(script.contains("write_pa11y_simple_config"));
    assert!(script.contains("PA11Y_PASSWORD"));
    assert!(script.contains("${BASE}/ui/admin/config"));
    let admin_urls = script
        .split("ADMIN_URLS=(")
        .nth(1)
        .and_then(|s| s.split(')').next())
        .expect("ADMIN_URLS block");
    assert!(admin_urls.contains("\"${BASE}/ui\""));
    for path in ["/ui/libs", "/ui/vm", "/ui/raid"] {
        assert!(
            admin_urls.contains(&format!("\"${{BASE}}{path}\"")),
            "missing ADMIN_URLS entry for {path}"
        );
    }
}
