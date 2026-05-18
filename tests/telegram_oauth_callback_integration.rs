//! FM-012: HTTP wire for Telegram Login Widget OAuth callback (`/api/enterprise/auth/telegram/callback`).

#![cfg(feature = "enterprise")]

use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use axum::{body::Body, Router};
use poolai::core::state::{ApiContext, AppState};
use poolai::enterprise::security::{sign_telegram_login_widget_query, OAuth2Config};
use poolai::network::enterprise_api::create_enterprise_api_routes;
use std::collections::HashMap;
use std::sync::Arc;
use tower::ServiceExt;

async fn enterprise_app(ctx: ApiContext) -> Router {
    Router::new()
        .nest("/api/enterprise", create_enterprise_api_routes())
        .with_state(ctx)
}

async fn register_telegram_provider(
    ctx: &ApiContext,
    token: &str,
    allow: Vec<String>,
) -> Result<(), poolai::core::error::AppError> {
    ctx.security_manager.initialize().await?;
    let _ = ctx
        .security_manager
        .delete_oauth2_provider("telegram")
        .await;
    let config = OAuth2Config {
        client_id: "poolai_test_bot".to_string(),
        client_secret: token.to_string(),
        authorization_url: "https://example.com/auth".to_string(),
        token_url: "https://example.com/token".to_string(),
        redirect_uri: "http://127.0.0.1:8080/api/enterprise/auth/telegram/callback".to_string(),
        scopes: vec![],
        telegram_allow_user_ids: allow,
    };
    ctx.security_manager
        .register_oauth2_provider("telegram".to_string(), config)
        .await
}

fn signed_query(token: &str, id: u64) -> HashMap<String, String> {
    let mut q = HashMap::new();
    q.insert(
        "auth_date".into(),
        chrono::Utc::now().timestamp().to_string(),
    );
    q.insert("id".into(), id.to_string());
    sign_telegram_login_widget_query(token, q).expect("sign widget query")
}

#[tokio::test]
async fn telegram_oauth_callback_redirects_on_success() {
    let token = "7000001:AAH_test_token_for_integration";
    let ctx: ApiContext = Arc::new(AppState::new());
    register_telegram_provider(&ctx, token, vec![])
        .await
        .expect("register telegram");
    let app = enterprise_app(ctx).await;

    let q = signed_query(token, 4242);
    let uri = format!(
        "/api/enterprise/auth/telegram/callback?id={}&auth_date={}&hash={}",
        q["id"],
        q["auth_date"],
        urlencoding::encode(&q["hash"])
    );

    let res = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
    let location = res.headers().get("location").unwrap().to_str().unwrap();
    assert!(location.starts_with("/ui/auth?token="));
}

#[tokio::test]
async fn telegram_oauth_callback_denied_when_not_on_allowlist() {
    let token = "7000002:AAH_test_token_allowlist";
    let ctx: ApiContext = Arc::new(AppState::new());
    register_telegram_provider(&ctx, token, vec!["99".to_string()])
        .await
        .expect("register telegram");
    let app = enterprise_app(ctx).await;

    let q = signed_query(token, 4242);
    let uri = format!(
        "/api/enterprise/auth/telegram/callback?id={}&auth_date={}&hash={}",
        q["id"],
        q["auth_date"],
        urlencoding::encode(&q["hash"])
    );

    let res = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["error"]["code"], "TELEGRAM_USER_NOT_ALLOWED");
}
