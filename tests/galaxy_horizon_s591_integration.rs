//! PH-S599: Galaxy horizon close band (PH-S591…S598).

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::dispatch::{
    with_prefetch_peer, ENV_PREFETCH_COORDINATOR_REGION, ENV_PREFETCH_MIN_BANDWIDTH_MBPS,
};
use poolai::grid::galaxy_capability_admission::{
    record_peer_capabilities, record_raid_artifact_probe_success, reset_peer_capabilities_for_test,
    reset_probe_success_for_test,
};
use poolai::grid::galaxy_capability_doc::{
    capability_signing_message, GalaxyCapabilityDocument, DEV_CAPABILITY_VERIFY_PK_HEX,
    ENV_TEE_ATTEST_REQUIRED,
};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_prefetch_metrics::{
    prefetch_backpressure_total, prefetch_egress_blocked_total, reset_prefetch_metrics_for_test,
    METRIC_PREFETCH_BACKPRESSURE_TOTAL, METRIC_PREFETCH_EGRESS_BLOCKED_TOTAL,
};
use poolai::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN;
use poolai::grid::galaxy_settlement_onchain::{
    last_onchain_rpc_signature_len, reset_onchain_submit_metrics_for_test,
};
use poolai::grid::galaxy_verify_sampling::ENV_VERIFY_BASE_SAMPLE_RATE;
use poolai::network::api::create_api_routes;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::observability::{self, metrics_handler};
use poolai::services::virtual_node_telegram_wallet_service::{
    reset_wallet_rebind_override_for_test, VirtualNodeTelegramWalletService,
    ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS,
};
use serde_json::{json, Value};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use tower::ServiceExt;

fn grid_app() -> Router {
    observability::init_prometheus();
    Router::new()
        .nest("/api/v1", create_api_routes())
        .route("/metrics", get(metrics_handler))
        .with_state(ApiContext::default())
}

async fn app_with_discovery() -> Router {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18084));
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
        .with_state(ctx)
}

async fn request_json(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    auth: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = auth {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let req_body = if let Some(v) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&v).unwrap())
    } else {
        Body::empty()
    };
    let response = app
        .clone()
        .oneshot(builder.body(req_body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, v)
}

fn signed_gpu_capability(peer_id: &str, tee: Option<&str>) -> Value {
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    assert_eq!(
        hex::encode(sk.verifying_key().to_bytes()),
        DEV_CAPABILITY_VERIFY_PK_HEX
    );
    let expires_at = "2027-12-31T00:00:00Z";
    let mut caps = vec!["inference:gpu".to_string()];
    if tee.is_some() {
        caps.push("gpu_passthrough".into());
    }
    let unsigned = GalaxyCapabilityDocument {
        peer_id: peer_id.into(),
        capabilities: caps.clone(),
        signature: None,
        expires_at: Some(expires_at.into()),
        tee_attestation: tee.map(str::to_string),
    };
    let msg = capability_signing_message(&unsigned);
    let mut doc = json!({
        "peer_id": peer_id,
        "capabilities": caps,
        "expires_at": expires_at,
        "signature": hex::encode(sk.sign(msg.as_bytes()).to_bytes()),
    });
    if let Some(t) = tee {
        doc["tee_attestation"] = json!(t);
    }
    doc
}

#[tokio::test]
async fn horizon_s591_band_profile_prefetch_gpu_tee_wallet_onchain_ph_s599() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
    reset_peer_capabilities_for_test();
    reset_probe_success_for_test();
    VirtualNodeTelegramWalletService::clear_all();
    reset_wallet_rebind_override_for_test();
    reset_onchain_submit_metrics_for_test();
    std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0");

    persist_peer_network_profile(
        "peer-s591",
        r#"{"region":"eu-west","latency_ms_p50":12,"bandwidth_mbps":10,"egress_policy":"direct"}"#,
    )
    .expect("persist");
    std::env::set_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS, "100");
    with_prefetch_peer(Some("peer-s591"), || {
        poolai::grid::dispatch::prefetch_backpressure_skip();
    });
    assert!(prefetch_backpressure_total() >= 1);

    persist_peer_network_profile(
        "peer-s592",
        r#"{"region":"us-east","latency_ms_p50":12,"egress_policy":"lan_only"}"#,
    )
    .expect("persist");
    std::env::set_var(ENV_PREFETCH_COORDINATOR_REGION, "eu-west");
    with_prefetch_peer(Some("peer-s592"), || {
        poolai::grid::dispatch::prefetch_egress_blocked_skip();
    });
    assert!(prefetch_egress_blocked_total() >= 1);

    let app = grid_app();
    record_raid_artifact_probe_success("tg-edge");
    record_peer_capabilities("tg-edge", &["inference:gpu".into()]);
    let job_id = format!("ph-s593-gpu-{}", uuid::Uuid::new_v4());
    let (gpu_status, gpu_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:00Z",
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference:gpu",
            "input_artifact_ids": [],
            "required_shard_ids": ["w:emb-1"],
            "source_peer_id": "tg-edge"
        })),
        None,
    )
    .await;
    assert_eq!(gpu_status, StatusCode::FORBIDDEN);
    assert_eq!(
        gpu_body["error"]["code"].as_str(),
        Some("gpu_passthrough_required")
    );

    std::env::set_var(ENV_TEE_ATTEST_REQUIRED, "1");
    let disc_app = app_with_discovery().await;
    let tee_peer = format!("ph-s594-{}", uuid::Uuid::new_v4());
    let (tee_status, _) = request_json(
        &disc_app,
        "POST",
        "/api/v1/discovery/register-remote",
        Some(json!({
            "peer_id": tee_peer,
            "address": "127.0.0.1",
            "port": 9104,
            "protocol_version": "1.2",
            "metadata": { "origin": "telegram_edge", "role": "virtual_node" },
            "capability_document": signed_gpu_capability("edge-gpu-1", None)
        })),
        None,
    )
    .await;
    assert_eq!(tee_status, StatusCode::FORBIDDEN);
    std::env::remove_var(ENV_TEE_ATTEST_REQUIRED);

    std::env::set_var(ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS, "86400");
    let pk1 = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";
    let pk2 = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
    VirtualNodeTelegramWalletService::bind("900591", "-100591", pk1, None).expect("bind");
    let admin_token = generate_token("admin", UserRole::Admin).expect("token");
    let (ov_status, ov_body) = request_json(
        &app,
        "POST",
        "/api/v1/virtual-nodes/telegram/wallet/rebind-override",
        Some(json!({
            "telegram_user_id": "900591",
            "chat_id": "-100591",
            "payout_pubkey": pk2,
            "chain": "solana"
        })),
        Some(&admin_token),
    )
    .await;
    assert_eq!(ov_status, StatusCode::OK);
    assert_eq!(ov_body["admin_override"].as_bool(), Some(true));
    assert_eq!(
        VirtualNodeTelegramWalletService::lookup("900591")
            .expect("wallet")
            .payout_pubkey,
        pk2
    );

    let peer_put = format!("ph-s596-{}", uuid::Uuid::new_v4());
    let (put_status, put_body) = request_json(
        &app,
        "PUT",
        &format!("/api/v1/grid/network-profiles/{peer_put}"),
        Some(json!({
            "network_profile": {
                "region": "eu-central",
                "latency_ms_p50": 15,
                "bandwidth_mbps": 250,
                "egress_policy": "direct"
            }
        })),
        None,
    )
    .await;
    assert_eq!(put_status, StatusCode::OK);
    assert_eq!(put_body["peer_id"].as_str(), Some(peer_put.as_str()));

    std::env::set_var(ENV_SETTLEMENT_ON_CHAIN, "1");
    std::env::set_var("POOLAI_SOLANA_MOCK_RPC", "1");
    let tmp = tempfile::TempDir::new().expect("tempdir");
    std::env::set_var(
        "POOLAI_ONCHAIN_EVENTS_DIR",
        tmp.path().to_string_lossy().as_ref(),
    );
    record_raid_artifact_probe_success("tg-edge");
    record_peer_capabilities(
        "tg-edge",
        &["gpu_passthrough".into(), "inference:gpu".into()],
    );
    let onchain_job = format!("ph-s597-{}", uuid::Uuid::new_v4());
    let (job_status, _) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:00Z",
            "type": "job",
            "job_id": onchain_job,
            "task_kind": "inference:text",
            "input_artifact_ids": [format!("artifact-{onchain_job}")],
            "source_peer_id": "tg-edge"
        })),
        None,
    )
    .await;
    assert_eq!(job_status, StatusCode::OK);
    let (_, get_body) = request_json(
        &app,
        "GET",
        &format!("/api/v1/jobs/{onchain_job}"),
        None,
        None,
    )
    .await;
    let epoch = get_body["job"]["lease_epoch"].as_u64().expect("epoch");
    let (result_status, _result_body) = request_json(
        &app,
        "POST",
        "/api/v1/grid/envelope",
        Some(json!({
            "v": 1,
            "sent_at": "2026-06-19T12:00:01Z",
            "type": "result",
            "job_id": onchain_job,
            "status": "completed",
            "output_artifact_ids": [format!("out-{onchain_job}")],
            "lease_epoch": epoch,
            "source_peer_id": "tg-edge",
            "metrics": { "trust_score": 85 }
        })),
        None,
    )
    .await;
    assert_eq!(result_status, StatusCode::OK);
    assert!(last_onchain_rpc_signature_len() > 0);

    let metrics_status = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status();
    assert_eq!(metrics_status, StatusCode::OK);
    let metrics_bytes = to_bytes(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
            .into_body(),
        usize::MAX,
    )
    .await
    .unwrap();
    let metrics_text = String::from_utf8(metrics_bytes.to_vec()).unwrap_or_default();
    for name in [
        METRIC_PREFETCH_BACKPRESSURE_TOTAL,
        METRIC_PREFETCH_EGRESS_BLOCKED_TOTAL,
        "galaxy_settlement_cleared_total",
    ] {
        assert!(metrics_text.contains(name), "missing {name}");
    }

    let (adv_status, adv_body) =
        request_json(&app, "GET", "/api/v1/admin/security-advisories", None, None).await;
    assert_eq!(adv_status, StatusCode::OK);
    assert!(adv_body.to_string().contains("CVE-2026-0001"));

    std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    std::env::remove_var(ENV_PREFETCH_MIN_BANDWIDTH_MBPS);
    std::env::remove_var(ENV_PREFETCH_COORDINATOR_REGION);
    std::env::remove_var(ENV_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS);
    std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
    std::env::remove_var("POOLAI_SOLANA_MOCK_RPC");
    std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    reset_network_profile_store_for_test();
    reset_prefetch_metrics_for_test();
    reset_peer_capabilities_for_test();
    reset_probe_success_for_test();
    VirtualNodeTelegramWalletService::clear_all();
    reset_wallet_rebind_override_for_test();
    reset_onchain_submit_metrics_for_test();
}
