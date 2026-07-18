//! Operator power endpoints (`POST /api/v1/ops/power`, PH-S1016).

use axum::{http::StatusCode, routing::post, Json, Router};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::network::api::common::HttpAppError;
use crate::ops::power::{apply_power_action, PowerAction, PowerRequest, PowerResponse};

pub fn create_ops_routes() -> Router<ApiContext> {
    Router::new().route("/ops/power", post(power_handler))
}

async fn power_handler(
    Json(body): Json<PowerRequest>,
) -> Result<(StatusCode, Json<PowerResponse>), HttpAppError> {
    let action = PowerAction::parse(body.action.trim()).ok_or_else(|| {
        HttpAppError::new(AppError::ValidationError(
            "action must be shutdown or reboot".into(),
        ))
    })?;
    let response = apply_power_action(action);
    Ok((StatusCode::ACCEPTED, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn ops_power_route_accepts_shutdown_ph_s1016() {
        let app = create_ops_routes().with_state(ApiContext::default());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/ops/power")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"action":"shutdown"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
}
