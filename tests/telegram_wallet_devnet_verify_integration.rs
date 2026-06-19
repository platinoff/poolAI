//! PH-S559: Telegram wallet devnet verify opt-in on bind.

use poolai::services::virtual_node_telegram_wallet_service::{
    devnet_verify_pubkey_stub, wallet_verify_devnet_enabled, VirtualNodeTelegramWalletService,
    ENV_WALLET_VERIFY_DEVNET,
};

const VALID_PUBKEY: &str = "7EqQdE8uK9V3mN2pL4qR5sT6uV7wX8yZ9aB1cD2eF3";

#[test]
fn devnet_verify_pubkey_stub_ph_s559() {
    assert!(devnet_verify_pubkey_stub(VALID_PUBKEY));
    assert!(!devnet_verify_pubkey_stub("bad!"));
}

#[test]
fn bind_respects_devnet_verify_env_ph_s559() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    std::env::set_var(ENV_WALLET_VERIFY_DEVNET, "1");
    assert!(wallet_verify_devnet_enabled());

    let uid = format!("ph-s559-{}", uuid::Uuid::new_v4());
    let binding =
        VirtualNodeTelegramWalletService::bind(&uid, "chat-1", VALID_PUBKEY, None).expect("bind");
    assert!(binding.verified);

    std::env::remove_var(ENV_WALLET_VERIFY_DEVNET);
}
