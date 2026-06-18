//! PH-S505: telegram seat coordinator read API.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::network::api::create_api_routes;
use poolai::services::telegram_seat_service::{
    reset_telegram_seats_for_test, try_admit_telegram_edge, ENV_TELEGRAM_SEAT_LIMIT,
    ENV_TELEGRAM_SEAT_POLICY,
};
use poolai::services::virtual_node_telegram_wallet_service::VirtualNodeTelegramWalletService;
use tower::ServiceExt;

fn grid_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({})),
    )
}

#[tokio::test]
async fn grid_telegram_seats_read_api_ph_s505() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_telegram_seats_for_test();
    VirtualNodeTelegramWalletService::clear_all();
    std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "5");
    std::env::set_var(ENV_TELEGRAM_SEAT_POLICY, "bound_wallet_session");

    let pubkey = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";
    VirtualNodeTelegramWalletService::bind("9001", "-100123", pubkey, None).expect("bind");
    let _ = try_admit_telegram_edge("peer-seat-1");

    let app = grid_app();
    let (status, body) = get_json(&app, "/api/v1/grid/telegram-seats").await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(body.get("ok").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(
        body.get("seat_policy").and_then(|v| v.as_str()),
        Some("bound_wallet_session")
    );
    assert_eq!(
        body.get("admin_max_seats").and_then(|v| v.as_u64()),
        Some(5)
    );
    assert_eq!(
        body.get("bound_wallets_count").and_then(|v| v.as_u64()),
        Some(1)
    );
    assert_eq!(body.get("seat_limit").and_then(|v| v.as_u64()), Some(1));
    assert_eq!(
        body.get("active_telegram_edge_workers")
            .and_then(|v| v.as_u64()),
        Some(1)
    );

    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
    std::env::remove_var(ENV_TELEGRAM_SEAT_POLICY);
    reset_telegram_seats_for_test();
    VirtualNodeTelegramWalletService::clear_all();
}
