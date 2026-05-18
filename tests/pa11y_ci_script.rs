//! FM-019: pa11y CI script declares auth fixture (no Node required).

#[test]
fn pa11y_ci_script_has_admin_strict_auth_actions() {
    let script = include_str!("../bin/pa11y-ci.sh");
    assert!(script.contains("PA11Y_ADMIN_STRICT"));
    assert!(script.contains("run_pa11y_authenticated"));
    assert!(script.contains("write_pa11y_config"));
    assert!(script.contains("PA11Y_PASSWORD"));
}
