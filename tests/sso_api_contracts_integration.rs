//! PH-S1270…S1274: SSO HTTP API contracts (band 63).
//! Marker: sso_api_contracts_integration

#![cfg(feature = "enterprise")]

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use chrono::{Duration, Utc};
use poolai::core::state::ApiContext;
use poolai::enterprise::security::validate_saml_audience_and_time;
use poolai::network::auth::{generate_token, UserRole};
use poolai::network::enterprise_api::create_enterprise_api_routes;
use poolai_ui_core::sso_api_contracts_depth::{
    sso_api_contracts_depth_stub, sso_api_criteria_total, SsoApiContractsDepth,
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

#[tokio::test]
async fn sso_oauth2_http_crud_lifecycle_ph_s1270() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("oauth-api-{}", uuid::Uuid::new_v4());

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/oauth2/providers",
        Some(oauth2_create_body(&name)),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED, "create: {created}");
    assert_eq!(
        created.get("message").and_then(|m| m.as_str()),
        Some("OAuth2 provider registered successfully")
    );

    let (get_status, got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK, "get: {got}");
    assert_eq!(
        got.get("name").and_then(|n| n.as_str()),
        Some(name.as_str())
    );

    let (list_status, list) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/oauth2/providers",
        None,
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK, "list: {list}");
    let arr = list.as_array().expect("list array");
    assert!(arr
        .iter()
        .any(|p| p.get("name").and_then(|n| n.as_str()) == Some(name.as_str())));

    let (upd_status, updated) = request_json(
        &app,
        "PUT",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        Some(json!({
            "enabled": false,
            "config": {
                "client_id": "cid-updated",
                "client_secret": "csecret",
                "authorization_url": "https://oauth.example.com/authorize",
                "token_url": "https://oauth.example.com/token",
                "redirect_uri": "https://poolai.example.com/callback",
                "scopes": ["openid"],
                "telegram_allow_user_ids": []
            }
        })),
        Some(&auth),
    )
    .await;
    assert_eq!(upd_status, StatusCode::OK, "update: {updated}");

    let (got2_status, got2) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        None,
        None,
    )
    .await;
    assert_eq!(got2_status, StatusCode::OK, "get2: {got2}");
    assert_eq!(got2.get("enabled").and_then(|e| e.as_bool()), Some(false));
    assert_eq!(
        got2.get("config")
            .and_then(|c| c.get("client_id"))
            .and_then(|c| c.as_str()),
        Some("cid-updated")
    );

    let (del_status, deleted) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK, "delete: {deleted}");

    let (missing_status, missing) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        None,
        None,
    )
    .await;
    assert_eq!(missing_status, StatusCode::NOT_FOUND, "missing: {missing}");

    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({"oauth2_http_crud": true}))),
        SsoApiContractsDepth::Oauth2HttpCrud
    );
    assert_eq!(sso_api_criteria_total(), 10);
}

#[tokio::test]
async fn sso_saml_http_crud_lifecycle_ph_s1271() {
    let app = enterprise_app().await;
    let auth = admin_bearer();
    let name = format!("saml-api-{}", uuid::Uuid::new_v4());

    let (create_status, created) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/saml/providers",
        Some(saml_create_body(&name)),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED, "create: {created}");

    let (get_status, got) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/saml/providers/{name}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK, "get: {got}");
    assert_eq!(
        got.get("name").and_then(|n| n.as_str()),
        Some(name.as_str())
    );

    let (upd_status, updated) = request_json(
        &app,
        "PUT",
        &format!("/api/enterprise/security/saml/providers/{name}"),
        Some(json!({ "enabled": false })),
        Some(&auth),
    )
    .await;
    assert_eq!(upd_status, StatusCode::OK, "update: {updated}");

    let (del_status, deleted) = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/security/saml/providers/{name}"),
        None,
        Some(&auth),
    )
    .await;
    assert_eq!(del_status, StatusCode::OK, "delete: {deleted}");

    let (missing_status, missing) = request_json(
        &app,
        "GET",
        &format!("/api/enterprise/security/saml/providers/{name}"),
        None,
        None,
    )
    .await;
    assert_eq!(missing_status, StatusCode::NOT_FOUND, "missing: {missing}");

    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({"saml_http_crud": true}))),
        SsoApiContractsDepth::SamlHttpCrud
    );
}

#[tokio::test]
async fn sso_store_wire_http_ph_s1272() {
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

    std::env::set_var("POOLAI_SSO_STORE", "sqlite");
    std::env::set_var("POOLAI_SSO_DATA_DIR", "data/dev/sso");
    let (status2, wire2) = request_json(
        &app,
        "GET",
        "/api/enterprise/security/sso/store",
        None,
        None,
    )
    .await;
    assert_eq!(status2, StatusCode::OK, "sqlite wire: {wire2}");
    assert_eq!(wire2.get("mode").and_then(|m| m.as_str()), Some("sqlite"));
    assert_eq!(
        wire2.get("configured").and_then(|c| c.as_bool()),
        Some(true)
    );
    let path = wire2
        .get("durable_path")
        .and_then(|p| p.as_str())
        .expect("durable_path");
    assert!(path.contains("sso"), "path={path}");

    std::env::remove_var("POOLAI_SSO_STORE");
    std::env::remove_var("POOLAI_SSO_DATA_DIR");

    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
        SsoApiContractsDepth::StoreWireHttp
    );
}

#[tokio::test]
async fn sso_callback_fixtures_http_ph_s1274() {
    let app = enterprise_app().await;
    let auth = admin_bearer();

    // OAuth callback missing code — deterministic fixture (no live IdP).
    let (oauth_status, oauth_body) = request_json(
        &app,
        "GET",
        "/api/enterprise/auth/github/callback",
        None,
        None,
    )
    .await;
    assert_eq!(oauth_status, StatusCode::BAD_REQUEST, "oauth: {oauth_body}");
    let oauth_code = oauth_body
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .or_else(|| oauth_body.get("code").and_then(|c| c.as_str()));
    assert!(
        oauth_code == Some("OAUTH2_MISSING_CODE")
            || oauth_body.to_string().contains("OAUTH2_MISSING_CODE")
            || oauth_body
                .to_string()
                .contains("Missing authorization code"),
        "oauth body: {oauth_body}"
    );

    // SAML callback for missing provider — deterministic fixture.
    let (saml_status, saml_body) = request_form(
        &app,
        "POST",
        "/api/enterprise/auth/saml/missing-provider-band63/callback",
        "SAMLResponse=dGVzdA%3D%3D&RelayState=x",
    )
    .await;
    assert_eq!(saml_status, StatusCode::BAD_REQUEST, "saml: {saml_body}");
    assert!(
        saml_body.to_string().contains("SAML_ASSERTION_INVALID")
            || saml_body.to_string().contains("Failed to validate"),
        "saml body: {saml_body}"
    );

    // Audience/time unit stub (band 61) still green under API contracts suite.
    let now = Utc::now();
    let future = (now + Duration::hours(1)).to_rfc3339();
    let ok_xml = format!(
        r#"<saml:Audience>https://poolai.example.com</saml:Audience><saml:Conditions NotOnOrAfter="{future}"/>"#
    );
    assert!(validate_saml_audience_and_time(&ok_xml, "https://poolai.example.com", now).is_ok());
    let bad = validate_saml_audience_and_time(&ok_xml, "https://other.example.com", now);
    assert!(bad.is_err());

    // Ensure provider CRUD still reachable after callback fixtures.
    let name = format!("oauth-fx-{}", uuid::Uuid::new_v4());
    let (create_status, _) = request_json(
        &app,
        "POST",
        "/api/enterprise/security/oauth2/providers",
        Some(oauth2_create_body(&name)),
        Some(&auth),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let _ = request_json(
        &app,
        "DELETE",
        &format!("/api/enterprise/security/oauth2/providers/{name}"),
        None,
        Some(&auth),
    )
    .await;

    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({"callback_fixtures": true}))),
        SsoApiContractsDepth::CallbackFixtures
    );
}
