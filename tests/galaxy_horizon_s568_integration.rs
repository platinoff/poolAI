//! PH-S577: Galaxy horizon wire integration band (PH-S568…S576).

#![allow(clippy::await_holding_lock)]
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, routing::get, Router};
use poolai::core::discovery_handle::DiscoveryHandle;
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_fraud_proof::{
    fraud_proof_pending_total, reset_fraud_proof_metrics_for_test, ENV_FRAUD_PROOF,
};
use poolai::grid::galaxy_network_profile_store::{
    persist_peer_network_profile, reset_network_profile_store_for_test,
};
use poolai::grid::galaxy_security_advisory::{
    advisory_acknowledged_total, reset_security_advisory_for_test,
};
use poolai::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use poolai::grid::galaxy_settlement::SettlementStatus;
use poolai::grid::galaxy_settlement_mode::ENV_SETTLEMENT_ON_CHAIN;
use poolai::grid::galaxy_settlement_onchain::{
    emit_settlement_job_rewarded, last_onchain_rpc_signature_len,
    reset_onchain_submit_metrics_for_test,
};
use poolai::grid::galaxy_verify_sampling::{
    evaluate_checker_timeout_policy, reset_verify_sampling_metrics_for_test, VerifySamplingConfig,
    VerifySamplingVerdict,
};
use poolai::grid::protocol_compat::ENV_PROTOCOL_SUNSET_MIN;
use poolai::grid::{
    ingest_envelope, GridEnvelope, GridIngestKind, GridJobBody, GridMessage, GridResultBody,
    GridResultStatus,
};
use poolai::job::JobStore;
use poolai::memory::MemoryShardStore;
use poolai::network::api::admin::create_admin_routes;
use poolai::network::api::create_api_routes;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::discovery::{DiscoveryConfig, DiscoveryService};
use poolai::observability::{self, metrics_handler};
use serde_json::json;
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
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 18083));
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

async fn get_text(app: &Router, uri: &str) -> (StatusCode, String) {
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
        String::from_utf8(bytes.to_vec()).unwrap_or_default(),
    )
}

#[tokio::test]
async fn horizon_s568_band_onchain_network_profiles_metrics_ph_s577() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    reset_network_profile_store_for_test();
    reset_onchain_submit_metrics_for_test();
    reset_verify_sampling_metrics_for_test();
    reset_fraud_proof_metrics_for_test();
    reset_security_advisory_for_test();

    persist_peer_network_profile("peer-list-a", r#"{"region":"eu"}"#).expect("persist");
    let app = grid_app();
    let (status, body) = get_text(&app, "/api/v1/grid/network-profiles").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("peer-list-a"));

    let tmp = tempfile::TempDir::new().expect("tempdir");
    std::env::set_var(
        "POOLAI_ONCHAIN_EVENTS_DIR",
        tmp.path().to_string_lossy().as_ref(),
    );
    std::env::set_var(ENV_SETTLEMENT_ON_CHAIN, "1");
    std::env::set_var("POOLAI_SOLANA_MOCK_RPC", "1");
    let entry = PayoutBatchLedgerEntry {
        job_id: "horizon-s568-job".into(),
        cleared_at: "2026-06-19T12:00:00Z".into(),
        gross_lamports: Some(9_000),
        ..PayoutBatchLedgerEntry::minimal("", "")
    };
    emit_settlement_job_rewarded(&entry, "peer-onchain");
    assert!(last_onchain_rpc_signature_len() > 0);

    assert_eq!(
        evaluate_checker_timeout_policy(
            "job-timeout",
            Some(&json!({"checker_timeout": true, "checker_retry_count": 1})),
            &VerifySamplingConfig::default_stub(),
        ),
        VerifySamplingVerdict::VerificationInconclusive
    );
    let (_, metrics) = get_text(&app, "/metrics").await;
    assert!(metrics.contains("galaxy_verification_checker_timeout_inconclusive_total"));

    std::env::set_var(ENV_FRAUD_PROOF, "1");
    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);
    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "horizon-s571-job".into(),
            task_kind: "inference".into(),
            verification_policy: None,
            input_artifact_ids: vec![],
            required_shard_ids: vec![],
            deadline: None,
        }),
        Some("tg-edge".into()),
    );
    ingest_envelope(job_env, &jobs, &memory).expect("job ingest");
    let lease_epoch = jobs
        .get("horizon-s571-job")
        .expect("get")
        .expect("row")
        .lease_epoch;
    let result_env = GridEnvelope::new(
        GridMessage::Result(GridResultBody {
            job_id: "horizon-s571-job".into(),
            status: GridResultStatus::Completed,
            output_artifact_ids: vec![],
            proof: None,
            metrics: Some(json!({
                "trust_score": 90,
                "verification_verdict": "mismatch"
            })),
            lease_epoch,
        }),
        Some("tg-edge".into()),
    );
    let out = ingest_envelope(result_env, &jobs, &memory).expect("result ingest");
    match out.kind {
        GridIngestKind::Result {
            settlement_status, ..
        } => assert_eq!(settlement_status, SettlementStatus::PendingVerification),
        other => panic!("unexpected outcome: {other:?}"),
    }
    assert!(fraud_proof_pending_total() >= 1);

    std::env::set_var(ENV_PROTOCOL_SUNSET_MIN, "1.1");
    let disc_app = app_with_discovery().await;
    let sunset = disc_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/discovery/register-remote")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "peer_id": "sunset-peer",
                        "address": "127.0.0.1",
                        "port": 9102,
                        "protocol_version": "1.0"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(sunset.status(), StatusCode::UPGRADE_REQUIRED);

    let ctx = ApiContext::default();
    ctx.initialize().await.expect("init");
    let admin_app = create_admin_routes().with_state(ctx);
    let token = generate_token("admin", UserRole::Admin).expect("token");
    let ack = admin_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/admin/security-advisories/CVE-2026-577/acknowledge")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ack");
    assert_eq!(ack.status(), StatusCode::OK);
    assert_eq!(advisory_acknowledged_total(), 1);

    std::env::remove_var(ENV_SETTLEMENT_ON_CHAIN);
    std::env::remove_var("POOLAI_SOLANA_MOCK_RPC");
    std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    std::env::remove_var(ENV_FRAUD_PROOF);
    std::env::remove_var(ENV_PROTOCOL_SUNSET_MIN);
    reset_network_profile_store_for_test();
}
