//! Rewards system API endpoints
//!
//! Provides endpoints for the rewards system:
//! - List all rewards statistics
//! - Get rewards for a specific user
//! - Get user progress
//! - Get top users

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::rewards::{
    get_reward_statistics, get_top_users, get_user_progress, get_user_rewards,
};

/// Create rewards system routes
pub fn create_rewards_routes() -> Router {
    Router::new()
        .route("/rewards", get(rewards_handler))
        .route("/rewards/{user_id}", get(user_rewards_handler))
        .route("/rewards/progress/{user_id}", get(user_progress_handler))
        .route("/rewards/statistics", get(rewards_statistics_handler))
        .route("/rewards/top", get(top_users_handler))
}

async fn rewards_handler() -> impl IntoResponse {
    let rewards = get_reward_statistics().await;
    Json(rewards).into_response()
}

async fn user_rewards_handler(Path(user_id): Path<String>) -> impl IntoResponse {
    let rewards = get_user_rewards(&user_id).await;
    Json(rewards).into_response()
}

async fn user_progress_handler(Path(user_id): Path<String>) -> impl IntoResponse {
    let progress = get_user_progress(&user_id).await;
    match progress {
        Some(progress) => Json(progress).into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "User not found"
            })),
        )
            .into_response(),
    }
}

async fn rewards_statistics_handler() -> impl IntoResponse {
    let stats = get_reward_statistics().await;
    Json(stats).into_response()
}

async fn top_users_handler() -> impl IntoResponse {
    let top_users = get_top_users(10).await;
    Json(top_users).into_response()
}
