//! Rewards-facing operations for the HTTP API.
//!
//! Currently delegates to the in-process [`crate::rewards`] module (lazy-static engine).
//! `ApiContext` is passed for alignment with the service layer and a future move of the engine onto [`crate::core::state::AppState`].

use crate::core::state::ApiContext;
use crate::rewards::{
    get_reward_statistics, get_top_users, get_user_progress, get_user_rewards, Reward, UserProgress,
};
use std::collections::HashMap;

/// Default leaderboard size for `GET /rewards/top`.
pub const TOP_USERS_DEFAULT_LIMIT: usize = 10;

pub struct RewardsService;

impl RewardsService {
    pub async fn reward_statistics(_ctx: &ApiContext) -> HashMap<String, f64> {
        get_reward_statistics().await
    }

    pub async fn user_rewards(_ctx: &ApiContext, user_id: &str) -> Vec<Reward> {
        get_user_rewards(user_id).await
    }

    pub async fn user_progress(_ctx: &ApiContext, user_id: &str) -> Option<UserProgress> {
        get_user_progress(user_id).await
    }

    pub async fn top_users(_ctx: &ApiContext, limit: usize) -> Vec<(String, f64)> {
        get_top_users(limit).await
    }
}
