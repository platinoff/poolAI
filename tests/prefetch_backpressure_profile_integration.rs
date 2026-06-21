//! PH-S751: Prefetch backpressure gate from persisted network profile bandwidth.

use poolai::grid::dispatch::{
    prefetch_backpressure_skip, with_prefetch_peer, ENV_PREFETCH_MIN_BANDWIDTH_MBPS,
};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_backpressure_total, reset_prefetch_metrics_for_test,
};

#[test]
fn prefetch_backpressure_from_profile_bandwidth_ph_s751() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
    std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");

    persist_peer_network_profile(
        "peer-s751",
        r#"{"region":"eu-west","latency_ms_p50":12,"bandwidth_mbps":10,"egress_policy":"direct"}"#,
    )
    .expect("persist");

    with_prefetch_peer(Some("peer-s751"), || {
        assert!(prefetch_backpressure_skip());
    });
    assert_eq!(prefetch_backpressure_total(), 1);

    std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
}
