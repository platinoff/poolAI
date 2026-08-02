//! PH-S749: Galaxy horizon close band (PH-S740…S748).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_capability_admission_depth::{
    capability_admission_depth_stub, CapabilityAdmissionDepth,
};
use poolai::grid::galaxy_capability_admission_metrics::reset_capability_admission_metrics_for_test;
use poolai::grid::galaxy_capability_doc::{
    capability_signing_message, GalaxyCapabilityDocument, DEV_CAPABILITY_VERIFY_PK_HEX,
};
use poolai::grid::stand_smoke_metrics_parity::{
    stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
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

async fn grid_app() -> Router {
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
        .route("/metrics", get(metrics_handler))
        .with_state(ctx)
}

fn signed_doc(peer_id: &str) -> GalaxyCapabilityDocument {
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    assert_eq!(
        hex::encode(sk.verifying_key().to_bytes()),
        DEV_CAPABILITY_VERIFY_PK_HEX
    );
    let unsigned = GalaxyCapabilityDocument {
        peer_id: peer_id.into(),
        capabilities: vec!["inference:edge".into()],
        signature: None,
        expires_at: Some("2027-12-31T00:00:00Z".into()),
        tee_attestation: None,
    };
    let msg = capability_signing_message(&unsigned);
    GalaxyCapabilityDocument {
        signature: Some(hex::encode(sk.sign(msg.as_bytes()).to_bytes())),
        ..unsigned
    }
}

async fn post_register(app: &Router, body: Value) -> StatusCode {
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
    response.status()
}

async fn metrics_text(app: &Router) -> String {
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

#[tokio::test]
async fn horizon_s740_band_signed_capability_admission_ph_s749() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_telegram_seats_for_test();
    reset_capability_admission_metrics_for_test();
    std::env::set_var(ENV_TELEGRAM_SEAT_LIMIT, "10");

    let app = grid_app().await;
    let peer_id = format!("horizon-s740-{}", uuid::Uuid::new_v4());
    let doc = signed_doc(&peer_id);

    assert_eq!(
        capability_admission_depth_stub(None),
        CapabilityAdmissionDepth::None
    );
    assert_eq!(
        capability_admission_depth_stub(Some(&doc)),
        CapabilityAdmissionDepth::SignedWithExpiry
    );
    assert_eq!(
        stand_smoke_metrics_parity_depth_stub(Some(&json!({"capability_admission": true}))),
        StandSmokeMetricsParityDepth::CapabilityAdmission
    );

    assert_eq!(
        post_register(
            &app,
            json!({
                "peer_id": peer_id,
                "address": "127.0.0.1",
                "port": 9104,
                "protocol_version": "1.2",
                "metadata": { "origin": "telegram_edge", "role": "virtual_node" }
            }),
        )
        .await,
        StatusCode::FORBIDDEN
    );

    assert_eq!(
        post_register(
            &app,
            json!({
                "peer_id": peer_id,
                "address": "127.0.0.1",
                "port": 9105,
                "protocol_version": "1.2",
                "metadata": { "origin": "telegram_edge", "role": "virtual_node" },
                "capability_document": doc
            }),
        )
        .await,
        StatusCode::OK
    );

    let prom = metrics_text(&app).await;
    assert!(prom.contains("galaxy_capability_unsigned_rejected_total"));
    assert!(prom.contains("galaxy_capability_signed_accepted_total"));

    std::env::remove_var(ENV_TELEGRAM_SEAT_LIMIT);
    reset_telegram_seats_for_test();
    reset_capability_admission_metrics_for_test();
}
