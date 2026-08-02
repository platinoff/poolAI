//! PH-S990: Telegram wallet HTTP wire — migrated from `e2e/archive/api-smoke/telegram_wallet.spec.ts`.
//! Canon registry: `tests/integration_gap_audit.rs` · binding extras in `virtual_node_telegram_binding_integration.rs`.

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::services::virtual_node_telegram_wallet_service::VirtualNodeTelegramWalletService;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

static TELEGRAM_WALLET_INTEGRATION_LOCK: Mutex<()> = Mutex::new(());

const VALID_PUBKEY: &str = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

async fn wallet_app() -> Router {
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18081)),
        None,
    ));
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ctx)
}

#[tokio::test]
async fn telegram_wallet_bind_verified_solana_payout_ph_s990() {
    let _lock = TELEGRAM_WALLET_INTEGRATION_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = wallet_app().await;
    let telegram_user_id = format!("ph-s990-wallet-{}", uuid::Uuid::new_v4());

    let bind = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/wallet")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "telegram_user_id": "{telegram_user_id}",
                "chat_id": "-1001234567890",
                "payout_pubkey": "{VALID_PUBKEY}",
                "chain": "solana"
            }}"#
        )))
        .unwrap();
    let res = app.oneshot(bind).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["wallet"]["telegram_user_id"], telegram_user_id);
    assert_eq!(v["wallet"]["payout_pubkey"], VALID_PUBKEY);
    assert_eq!(v["wallet"]["chain"], "solana");
    assert_eq!(v["wallet"]["verified"], true);

    VirtualNodeTelegramWalletService::clear_all();
}

#[tokio::test]
async fn telegram_wallet_bind_rejects_invalid_pubkey_ph_s990() {
    let _lock = TELEGRAM_WALLET_INTEGRATION_LOCK.lock().unwrap();
    VirtualNodeTelegramWalletService::clear_all();
    let app = wallet_app().await;
    let telegram_user_id = format!("ph-s990-wallet-bad-{}", uuid::Uuid::new_v4());

    let bind = Request::builder()
        .method("POST")
        .uri("/api/v1/virtual-nodes/telegram/wallet")
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{
                "telegram_user_id": "{telegram_user_id}",
                "chat_id": "-10099",
                "payout_pubkey": "not-valid!",
                "chain": "solana"
            }}"#
        )))
        .unwrap();
    let res = app.oneshot(bind).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    VirtualNodeTelegramWalletService::clear_all();
}
