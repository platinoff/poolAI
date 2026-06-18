//! PH-S529: startup hydrate persisted network_profile into discovery peers.

use poolai::core::discovery_types::{PeerCapabilities, PeerInfo};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reopen_network_profile_store_for_test,
    reset_network_profile_store_for_test, ENV_NETWORK_PROFILE_DATA_DIR,
};
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn hydrate_network_profiles_on_discovery_start_ph_s529() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("poolai-np-hydrate-{stamp}"));
    std::env::set_var(ENV_NETWORK_PROFILE_DATA_DIR, dir.to_string_lossy().as_ref());
    reopen_network_profile_store_for_test();
    reset_network_profile_store_for_test();

    let json = r#"{"region":"eu-west","latency_ms_p50":22}"#;
    persist_peer_network_profile("peer-hydrate-1", json).expect("persist");

    let service = Arc::new(DiscoveryService::new(
        DiscoveryConfig {
            enabled: true,
            ..Default::default()
        },
        "127.0.0.1:0".parse().unwrap(),
        None,
    ));
    service
        .register_remote_peer(PeerInfo {
            peer_id: "peer-hydrate-1".into(),
            address: "127.0.0.1".into(),
            port: 9100,
            last_seen: chrono::Utc::now(),
            capabilities: PeerCapabilities::default(),
            metadata: HashMap::new(),
        })
        .await
        .expect("register");

    let updated = service.hydrate_persisted_network_profiles().await;
    assert_eq!(updated, 1);
    let peer = service.get_peer("peer-hydrate-1").await.expect("peer");
    assert_eq!(
        peer.metadata.get("network_profile").map(String::as_str),
        Some(json)
    );

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(ENV_NETWORK_PROFILE_DATA_DIR);
    reset_network_profile_store_for_test();
}
