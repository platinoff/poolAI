//! PH-S849: Galaxy horizon close band (PH-S840…S848) — OpenAPI gap 0 + contract band.

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::ApiContext;
use poolai::grid::galaxy_security_advisory::{
    list_security_advisories, reset_security_advisory_for_test,
};
use poolai::network::api::create_api_routes;
use serde_json::Value;
use tower::ServiceExt;

fn api_app() -> Router {
    Router::new()
        .nest("/api/v1", create_api_routes())
        .with_state(ApiContext::default())
}

async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[tokio::test]
async fn horizon_s840_band_openapi_gap_routes_ph_s849() {
    reset_security_advisory_for_test();
    let advisories = list_security_advisories();
    assert_eq!(advisories.len(), 3);
    assert!(advisories.iter().any(|r| r.id == "CVE-2026-0001"));

    let app = api_app();

    let (status, list) = get_json(&app, "/api/v1/grid/network-profiles").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list["ok"], true);
    assert!(list["peer_ids"].is_array());

    let (status, rows) = get_json(&app, "/api/v1/admin/security-advisories").await;
    assert_eq!(status, StatusCode::OK);
    let arr = rows.as_array().expect("advisories");
    assert_eq!(arr.len(), 3);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/virtual-nodes/telegram/wallet/rebind-override")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"telegram_user_id":"9001","chat_id":"1","payout_pubkey":"7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let (status, governance) = get_json(&app, "/api/v1/grid/governance-metrics").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(governance["ok"], true);
    assert!(governance["metrics"].is_object());

    reset_security_advisory_for_test();
}
