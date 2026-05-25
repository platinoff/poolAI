//! Admin aggregation API (JSON for `/ui/admin` and tools).

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::network::api::common::{check_permission, HttpAppError};
use crate::network::auth::{auth_middleware, Claims};
use crate::security::secret_rotation::{rotation_status, run_rotation, SecretKind};
use crate::services::admin_service::{AdminOverview, AdminService};
use axum::{
    extract::{Extension, Json, State},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json as AxumJson, Router,
};
use serde::Deserialize;

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
    let Some(kind) = SecretKind::from_str(req.kind.trim()) else {
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

/// Routes under `/api/v1` (see `network::start_server` nest).
pub fn create_admin_routes() -> Router<ApiContext> {
    Router::new()
        .route("/admin/overview", get(admin_overview_handler))
        .route(
            "/admin/secrets/rotation",
            get(admin_secrets_rotation_status_handler),
        )
        .route(
            "/admin/secrets/rotate",
            post(admin_secrets_rotate_handler).layer(middleware::from_fn(auth_middleware)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
