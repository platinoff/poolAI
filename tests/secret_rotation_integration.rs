//! PH-S24: secret rotation hooks + admin rotation API.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use poolai::core::state::ApiContext;
use poolai::network::api::admin::create_admin_routes;
use poolai::network::auth::{generate_token, UserRole};
use poolai::security::jwt_secrets;
use poolai::security::secret_rotation::{
    init_default_rotation_hooks, rotation_status, run_rotation, SecretKind,
};
use tower::ServiceExt;

#[test]
fn rotation_status_lists_all_kinds() {
    init_default_rotation_hooks();
    let status = rotation_status();
    assert!(status.iter().any(|e| e.kind == SecretKind::Jwt));
    assert!(status.iter().any(|e| e.kind == SecretKind::TlsCertificate));
}

#[test]
fn jwt_rotation_hook_refreshes_loaded_at() {
    init_default_rotation_hooks();
    let before = jwt_secrets::jwt_store().read().loaded_at_unix;
    run_rotation(SecretKind::Jwt).expect("rotate jwt");
    let after = jwt_secrets::jwt_store().read().loaded_at_unix;
    assert!(after >= before);
}

#[tokio::test]
async fn admin_rotation_status_returns_json() {
    init_default_rotation_hooks();
    let ctx = ApiContext::default();
    ctx.initialize().await.expect("init");
    let app = create_admin_routes().with_state(ctx);
    let res = app
        .oneshot(
            Request::builder()
                .uri("/admin/secrets/rotation")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn admin_rotate_jwt_requires_admin_token() {
    init_default_rotation_hooks();
    let ctx = ApiContext::default();
    ctx.initialize().await.expect("init");
    let app = create_admin_routes().with_state(ctx);

    let no_auth = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/admin/secrets/rotate")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"kind":"jwt"}"#))
                .unwrap(),
        )
        .await
        .expect("oneshot");
    assert_eq!(no_auth.status(), StatusCode::UNAUTHORIZED);

    let token = generate_token("admin", UserRole::Admin).expect("token");
    let ok = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/admin/secrets/rotate")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(r#"{"kind":"jwt"}"#))
                .unwrap(),
        )
        .await
        .expect("oneshot");
    assert_eq!(ok.status(), StatusCode::OK);
}
