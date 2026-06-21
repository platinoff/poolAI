//! PH-S741: dev fixture signed capability pass path on register-remote.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_capability_admission_metrics::{
    capability_signed_accepted_total, capability_unsigned_rejected_total,
    reset_capability_admission_metrics_for_test,
};
use poolai::grid::galaxy_capability_doc::{
    capability_signing_message, GalaxyCapabilityDocument, DEV_CAPABILITY_VERIFY_PK_HEX,
};
use poolai::network::api::create_api_routes;
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::observability::{self, metrics_handler};
use poolai::services::telegram_seat_service::{
    reset_telegram_seats_for_test, ENV_TELEGRAM_SEAT_LIMIT,
};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

async fn app_with_discovery() -> Router {
    observability::init_prometheus();
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18082));
    let discovery = Arc::new(DiscoveryService::new(
        DiscoveryConfig::default(),
        addr,
        None,
    ));
    let ctx = ApiContext::default();
    {
        let mut slot = ctx.discovery.write().await;
        *slot = Some(discovery as Arc<dyn DiscoveryHandle>);
    }
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", axum::routing::get(metrics_handler))
        .with_state(ctx)
}

async fn post_register(app: &Router, body: Value) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/discovery/register-remote")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, v)
}

async fn get_metrics(app: &Router) -> String {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap_or_default()
}

fn signed_capability_doc(peer_id: &str) -> Value {
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    assert_eq!(
        hex::encode(sk.verifying_key().to_bytes()),
        DEV_CAPABILITY_VERIFY_PK_HEX
    );
    let expires_at = "2027-12-31T00:00:00Z";
    let unsigned = GalaxyCapabilityDocument {
        peer_id: peer_id.into(),
        capabilities: vec!["inference:edge".into()],
        signature: None,
        expires_at: Some(expires_at.into()),
        tee_attestation: None,
    };
    let msg = capability_signing_message(&unsigned);
    json!({
        "peer_id": peer_id,
        "capabilities": ["inference:edge"],
        "expires_at": expires_at,
        "signature": hex::encode(sk.sign(msg.as_bytes()).to_bytes()),
    })
}

#[tokio::test]
async fn register_remote_signed_capability_dev_fixture_ph_s741() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_telegram_seats_for_test();
    reset_capability_admission_metrics_for_test();
    std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "10");

    let app = app_with_discovery().await;
    let peer_id = format!("ph-s741-{}", uuid::Uuid::new_v4());

    let (status, body) = post_register(
        &app,
        json!({
            "peer_id": peer_id,
            "address": "127.0.0.1",
            "port": 9102,
            "protocol_version": "1.2",
            "metadata": { "origin": "telegram_edge", "role": "virtual_node" }
        }),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "body={body}");
    assert_eq!(capability_unsigned_rejected_total(), 1);

    let (status, body) = post_register(
        &app,
        json!({
            "peer_id": peer_id,
            "address": "127.0.0.1",
            "port": 9103,
            "protocol_version": "1.2",
            "metadata": { "origin": "telegram_edge", "role": "virtual_node" },
            "capability_document": signed_capability_doc(&peer_id)
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "body={body}");
    assert_eq!(capability_signed_accepted_total(), 1);

    let metrics = get_metrics(&app).await;
    assert!(metrics.contains("galaxy_capability_unsigned_rejected_total"));
    assert!(metrics.contains("galaxy_capability_signed_accepted_total"));

    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
    reset_telegram_seats_for_test();
    reset_capability_admission_metrics_for_test();
}
