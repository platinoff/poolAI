//! Admin aggregation API (JSON for `/ui/admin` and tools).

use crate::core::state::ApiContext;
use crate::services::admin_service::{AdminOverview, AdminService};
use axum::{extract::State, routing::get, Json, Router};

async fn admin_overview_handler(State(ctx): State<ApiContext>) -> Json<AdminOverview> {
    Json(AdminService::overview(&ctx).await)
}

/// Routes under `/api/v1` (see `network::start_server` nest).
pub fn create_admin_routes() -> Router<ApiContext> {
    Router::new().route("/admin/overview", get(admin_overview_handler))
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
