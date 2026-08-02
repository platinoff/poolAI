//! Admin aggregation API (JSON for `/ui/admin` and tools).

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::grid::galaxy_security_advisory::{
    acknowledge_security_advisory, list_security_advisories,
};
use crate::network::api::common::{check_permission, HttpAppError};
use crate::network::auth::{auth_middleware, Claims};
use crate::security::secret_rotation::{rotation_status, run_rotation, SecretKind};
use crate::services::admin_service::{AdminOverview, AdminService};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};
use serde::Deserialize;
use serde::Serialize;

async fn admin_overview_handler(
    State(ctx): State<ApiContext>,
) -> Result<AxumJson<AdminOverview>, AppError> {
    Ok(AxumJson(AdminService::overview(&ctx).await))
}

async fn admin_secrets_rotation_status_handler(
) -> AxumJson<Vec<crate::security::secret_rotation::RotationStatusEntry>> {
    AxumJson(rotation_status())
}

#[derive(Deserialize)]
struct RotateSecretRequest {
    kind: String,
}

async fn admin_secrets_rotate_handler(
    Extension(claims): Extension<Claims>,
    Json(req): Json<RotateSecretRequest>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }
    let Some(kind) = SecretKind::parse(req.kind.trim()) else {
        return HttpAppError::new(AppError::ValidationError(format!(
            "unknown secret kind: {}",
            req.kind
        )))
        .into_response();
    };
    match run_rotation(kind) {
        Ok(report) => AxumJson(report).into_response(),
        Err(e) => HttpAppError::new(e).into_response(),
    }
}

#[derive(Serialize)]
struct SecurityAdvisoryAckResponse {
    ok: bool,
    advisory_id: String,
    acknowledged: bool,
}

async fn admin_security_advisory_acknowledge_handler(
    Extension(claims): Extension<Claims>,
    Path(advisory_id): Path<String>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }
    let id = advisory_id.trim();
    if id.is_empty() {
        return HttpAppError::new(AppError::ValidationError(
            "advisory id must not be empty".into(),
        ))
        .into_response();
    }
    let first_ack = acknowledge_security_advisory(id);
    (
        StatusCode::OK,
        AxumJson(SecurityAdvisoryAckResponse {
            ok: true,
            advisory_id: id.to_string(),
            acknowledged: first_ack,
        }),
    )
        .into_response()
}

async fn admin_security_advisories_list_handler() -> impl IntoResponse {
    (StatusCode::OK, AxumJson(list_security_advisories())).into_response()
}

/// Routes under `/api/v1` (see `network::start_server` nest).
pub fn create_admin_routes() -> Router<ApiContext> {
    Router::new()
        .route("/admin/overview", get(admin_overview_handler))
        .route(
            "/admin/security-advisories",
            get(admin_security_advisories_list_handler),
        )
        .route(
            "/admin/secrets/rotation",
            get(admin_secrets_rotation_status_handler),
        )
        .route(
            "/admin/secrets/rotate",
            post(admin_secrets_rotate_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/admin/security-advisories/{id}/acknowledge",
            post(admin_security_advisory_acknowledge_handler)
                .layer(middleware::from_fn(auth_middleware)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_security_advisory::SecurityAdvisoryEntry;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn admin_overview_returns_ok_json() {
        let ctx = ApiContext::default();
        ctx.initialize().await.expect("init");
        let app = create_admin_routes().with_state(ctx);
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/admin/overview")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn admin_security_advisories_list_ph_s586() {
        let ctx = ApiContext::default();
        ctx.initialize().await.expect("init");
        let app = create_admin_routes().with_state(ctx);
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/admin/security-advisories")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("oneshot");
        assert_eq!(res.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .expect("body");
        let rows: Vec<SecurityAdvisoryEntry> = serde_json::from_slice(&bytes).expect("json array");
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().any(|r| r.id == "CVE-2026-0001"));
    }
}
