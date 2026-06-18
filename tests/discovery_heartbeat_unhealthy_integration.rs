//! PH-S522: consecutive heartbeat miss → worker unhealthy stub.

use poolai::grid::galaxy_worker_health::{
    galaxy_worker_unhealthy_total, is_peer_unhealthy, on_heartbeat_miss, on_heartbeat_success,
    reset_worker_health_for_test, ENV_HEARTBEAT_UNHEALTHY_THRESHOLD,
};

#[test]
fn heartbeat_miss_marks_unhealthy_ph_s522() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_worker_health_for_test();
    std::env::set_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD, "2");

    assert!(!on_heartbeat_miss("peer-hb-1"));
    assert!(on_heartbeat_miss("peer-hb-1"));
    assert!(is_peer_unhealthy("peer-hb-1"));
    assert_eq!(galaxy_worker_unhealthy_total(), 1);

    on_heartbeat_success("peer-hb-1");
    assert!(!is_peer_unhealthy("peer-hb-1"));

    std::env::remove_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD);
    reset_worker_health_for_test();
}
