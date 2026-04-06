//! Rewards system API endpoints
//!
//! Provides endpoints for the rewards system:
//! - List all rewards statistics
//! - Get rewards for a specific user
//! - Get user progress
//! - Get top users

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::api_json_error;
use crate::services::rewards_service::{RewardsService, TOP_USERS_DEFAULT_LIMIT};

/// Create rewards system routes
pub fn create_rewards_routes() -> Router<ApiContext> {
    Router::new()
        .route("/rewards", get(rewards_handler))
        .route("/rewards/{user_id}", get(user_rewards_handler))
        .route("/rewards/progress/{user_id}", get(user_progress_handler))
        .route("/rewards/statistics", get(rewards_statistics_handler))
        .route("/rewards/top", get(top_users_handler))
}

async fn rewards_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let rewards = RewardsService::reward_statistics(&ctx).await;
    Json(rewards).into_response()
}

async fn user_rewards_handler(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let rewards = RewardsService::user_rewards(&ctx, &user_id).await;
    Json(rewards).into_response()
}

async fn user_progress_handler(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    match RewardsService::user_progress(&ctx, &user_id).await {
        Some(progress) => Json(progress).into_response(),
        None => {
            let (s, j) = api_json_error(
                "NOT_FOUND",
                "User not found",
                Some(ErrorContext::new("user_progress").with_resource("user_id", &user_id)),
                StatusCode::NOT_FOUND,
            );
            (s, Json(j.0)).into_response()
        }
    }
}

async fn rewards_statistics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let stats = RewardsService::reward_statistics(&ctx).await;
    Json(stats).into_response()
}

async fn top_users_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    let top_users = RewardsService::top_users(&ctx, TOP_USERS_DEFAULT_LIMIT).await;
    Json(top_users).into_response()
}
