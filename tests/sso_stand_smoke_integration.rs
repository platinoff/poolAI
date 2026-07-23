//! PH-S1290…S1292: SSO live stand-smoke contracts (band 65).
//! Marker: sso_stand_smoke_integration
//!
//! CI canon uses in-process axum (no live stand). Live HTTP runners live in
//! `poolai-http-stand-smoke --sso-stand-smoke`.

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use poolai::core::state::ApiContext;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::sso_stand_smoke_depth::{
    sso_stand_smoke_criteria_total, sso_stand_smoke_depth_stub, SsoStandSmokeDepth,
    SSO_STAND_SMOKE_CASES, SSO_STAND_SMOKE_CRITERIA,
};
use serde_json::{json, Value};
use tower::ServiceExt;

fn admin_bearer() -> String {
    let token = generate_token("admin", UserRole::Admin).expect("admin token");
    format!("Bearer {token}")
}

async fn enterprise_app() -> Router {
    let ctx = ApiContext::default();
    ctx.security_manager
        .initialize()
        .await
        .expect("security init");
    Router::new()
        .nest("/api/enterprise", create_enterprise_api_routes())
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
    if let Some(bearer) = auth {
        builder = builder.header("authorization", bearer);
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
    let v: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("JSON body")
    };
    (status, v)
}

async fn request_form(app: &Router, method: &str, uri: &str, form: &str) -> (StatusCode, Value) {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/x-www-form-urlencoded");
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(form.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or(Value::String(String::from_utf8_lossy(&bytes).into_owned()))
    };
    (status, v)
}

fn oauth2_create_body(name: &str) -> Value {
    json!({
        "name": name,
        "config": {
            "client_id": "cid",
            "client_secret": "csecret",
            "authorization_url": "https://oauth.example.com/authorize",
            "token_url": "https://oauth.example.com/token",
            "redirect_uri": "https://poolai.example.com/callback",
            "scopes": ["openid", "profile"],
            "telegram_allow_user_ids": []
        },
        "enabled": true
    })
}

fn saml_create_body(name: &str) -> Value {
    json!({
        "name": name,
        "config": {
            "entity_id": "https://idp.example.com/entity",
            "sso_url": "https://idp.example.com/sso",
            "acs_url": "https://poolai.example.com/acs",
            "slo_url": null,
            "certificate": "TEST_CERT",
            "attribute_mapping": {
                "email": "email",
                "username": "username"
            }
        },
        "enabled": true
    })
}

#[test]
fn sso_stand_smoke_depth_registry_ph_s1289() {
    assert_eq!(SSO_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(sso_stand_smoke_criteria_total(), 10);
    assert!(SSO_STAND_SMOKE_CASES.contains(&"live_store"));
    assert!(SSO_STAND_SMOKE_CASES.contains(&"live_crud"));
    assert_eq!(
        sso_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
        SsoStandSmokeDepth::LiveStore
    );
}

#[tokio::test]
async fn sso_stand_smoke_store_wire_ph_s1290() {
    std::env::remove_var("POOLAI_SSO_STORE");
    std::env::remove_var("POOLAI_SSO_DATA_DIR");

    let app = enterprise_app().await;
    let (status, wire) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/sso/store",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "store wire: {wire}");
    let obj = wire.as_object().expect("wire object");
    for key in ["mode", "durable_path", "configured"] {
        assert!(obj.contains_key(key), "wire missing `{key}`: {obj:?}");
    }
    assert_eq!(obj.get("mode").and_then(|m| m.as_str()), Some("memory"));
    assert_eq!(obj.get("configured").and_then(|c| c.as_bool()), Some(false));
}

#[tokio::test]
async fn sso_stand_smoke_oauth2_saml_crud_ph_s1291() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let oauth_name = format!("stand-oauth-{}", uuid::Uuid::new_v4());
    let saml_name = format!("stand-saml-{}", uuid::Uuid::new_v4());

    let (list_status, list) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/oauth2/providers",
        None,
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK, "list: {list}");
    assert!(list.as_array().is_some(), "list array: {list}");

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/oauth2/providers",
        Some(oauth2_create_body(&oauth_name)),
        Some(&auth),
    )
    .await;
    assert_eq!(
        create_status,
        StatusCode::CREATED,
        "oauth create: {created}"
    );

    let (get_status, got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/oauth2/providers/{oauth_name}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK, "oauth get: {got}");
    assert_eq!(
        got.get("name").and_then(|n| n.as_str()),
        Some(oauth_name.as_str())
    );

    let (del_status, del) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/security/oauth2/providers/{oauth_name}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK, "oauth delete: {del}");

    let (saml_create_status, saml_created) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/saml/providers",
        Some(saml_create_body(&saml_name)),
        Some(&auth),
    )
    .await;
    assert_eq!(
        saml_create_status,
        StatusCode::CREATED,
        "saml create: {saml_created}"
    );

    let (saml_get_status, saml_got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/saml/providers/{saml_name}"),
        None,
        None,
    )
    .await;
    assert_eq!(saml_get_status, StatusCode::OK, "saml get: {saml_got}");

    let (saml_del_status, saml_del) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/security/saml/providers/{saml_name}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(saml_del_status, StatusCode::OK, "saml delete: {saml_del}");
}

#[tokio::test]
async fn sso_stand_smoke_callback_fixtures_ph_s1292() {
    let app = enterprise_app().await;

    let (oauth_status, oauth_body) = request_json(
        &app,
        "GET",
        "/api/enterprise/auth/github/callback",
        None,
        None,
    )
    .await;
    assert_eq!(oauth_status, StatusCode::BAD_REQUEST, "oauth: {oauth_body}");
    let oauth_text = oauth_body.to_string();
    assert!(
        oauth_text.contains("OAUTH2_MISSING_CODE")
            || oauth_text.contains("Missing authorization code"),
        "oauth body: {oauth_body}"
    );

    let (saml_status, saml_body) = request_form(
        &app,
        "POST",
        "/api/enterprise/auth/saml/missing-provider-band65/callback",
        "SAMLResponse=dGVzdA%3D%3D&RelayState=x",
    )
    .await;
    assert_eq!(saml_status, StatusCode::BAD_REQUEST, "saml: {saml_body}");
    assert!(
        saml_body.to_string().contains("SAML_ASSERTION_INVALID")
            || saml_body.to_string().contains("Failed to validate"),
        "saml body: {saml_body}"
    );

    assert_eq!(
        sso_stand_smoke_depth_stub(Some(&json!({"live_callback_fixtures": true}))),
        SsoStandSmokeDepth::LiveCallbackFixtures
    );
}
