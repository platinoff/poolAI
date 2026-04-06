//! Rewards-facing operations for the HTTP API.
//!
//! Uses [`crate::core::state::AppState::rewards_engine`] after bootstrap; falls back to
//! [`crate::rewards::shared_reward_engine`] so lightweight tests with `ApiContext::default()`
//! still hit the same process-wide engine.

use crate::core::state::ApiContext;
use crate::rewards::{shared_reward_engine, Reward, RewardSystem, UserProgress};
use std::collections::HashMap;
use std::sync::Arc;

/// Default leaderboard size for `GET /rewards/top`.
pub const TOP_USERS_DEFAULT_LIMIT: usize = 10;

fn engine(ctx: &ApiContext) -> Arc<RewardSystem> {
    ctx.rewards_engine
        .get()
        .cloned()
        .unwrap_or_else(|| shared_reward_engine())
}

pub struct RewardsService;

impl RewardsService {
    pub async fn reward_statistics(ctx: &ApiContext) -> HashMap<String, f64> {
        engine(ctx).get_reward_statistics().await
    }

    pub async fn user_rewards(ctx: &ApiContext, user_id: &str) -> Vec<Reward> {
        engine(ctx).get_user_rewards(user_id).await
    }

    pub async fn user_progress(ctx: &ApiContext, user_id: &str) -> Option<UserProgress> {
        engine(ctx).get_user_progress(user_id).await
    }

    pub async fn top_users(ctx: &ApiContext, limit: usize) -> Vec<(String, f64)> {
        engine(ctx).get_top_users(limit).await
    }
}
