//! Operator power endpoints (`POST /api/v1/ops/power`, PH-S1016) and ratio96 store wire
//! (`GET /api/v1/ops/ratio96`, PH-S1680).

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::network::api::common::HttpAppError;
use crate::ops::power::{apply_power_action, PowerAction, PowerRequest, PowerResponse};

pub fn create_ops_routes() -> Router<ApiContext> {
    Router::new()
        .route("/ops/power", post(power_handler))
        .route("/ops/ratio96", get(ratio96_store_wire_handler))
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

/// `GET /api/v1/ops/ratio96` — durable ratio store wire for the admin dashboard strip (PH-S1680).
async fn ratio96_store_wire_handler() -> Json<serde_json::Value> {
    Json(poolai_ui_core::ratio96_store_depth::ratio96_store_wire_json())
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

    #[tokio::test]
    async fn ops_ratio96_route_returns_wire_ph_s1680() {
        let app = create_ops_routes().with_state(ApiContext::default());
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/ops/ratio96")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .expect("ratio96 wire body");
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("ratio96 wire parses as JSON");
        assert_eq!(
            json.get("available").and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert!(json.get("stretch_gate_met").is_some());
        assert!(json.get("hold_gate_met").is_some());
    }
}
