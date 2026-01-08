//! Integration tests for Rewards Module

use poolai::rewards::{
    award_performance_bonus, create_reward, get_reward_statistics, get_top_users,
    get_user_progress, get_user_rewards, RewardLevel, RewardType,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_create_reward() {
    let reward = create_reward(
        "test_user_1".to_string(),
        RewardType::Performance,
        RewardLevel::Gold,
        100.0,
        "Test reward".to_string(),
        HashMap::new(),
    )
    .await;

    assert_eq!(reward.user_id, "test_user_1");
    assert_eq!(reward.reward_type, RewardType::Performance);
    assert_eq!(reward.level, RewardLevel::Gold);
    // Performance multiplier = 1.5, Gold level = 3, base = 100.0
    // amount = 100.0 * 1.5 * 3.0 = 450.0
    assert_eq!(reward.amount, 450.0);
}

#[tokio::test]
async fn test_get_user_rewards() {
    let user_id = "test_user_2".to_string();

    // Create multiple rewards
    create_reward(
        user_id.clone(),
        RewardType::Performance,
        RewardLevel::Gold,
        100.0,
        "Reward 1".to_string(),
        HashMap::new(),
    )
    .await;

    create_reward(
        user_id.clone(),
        RewardType::Efficiency,
        RewardLevel::Silver,
        50.0,
        "Reward 2".to_string(),
        HashMap::new(),
    )
    .await;

    let rewards = get_user_rewards(&user_id).await;
    assert!(rewards.len() >= 2);
    assert!(rewards.iter().all(|r| r.user_id == user_id));
}

#[tokio::test]
async fn test_get_user_progress() {
    let user_id = "test_user_3".to_string();

    // Create rewards to build progress
    for _ in 0..5 {
        create_reward(
            user_id.clone(),
            RewardType::Performance,
            RewardLevel::Gold,
            100.0,
            "Progress test".to_string(),
            HashMap::new(),
        )
        .await;
    }

    let progress = get_user_progress(&user_id).await;
    assert!(progress.is_some());
    let progress = progress.unwrap();
    assert_eq!(progress.user_id, user_id);
    assert!(progress.total_rewards > 0.0);
    assert!(progress.reward_count >= 5);
}

#[tokio::test]
async fn test_get_reward_statistics() {
    let user_id = "test_user_4".to_string();

    create_reward(
        user_id.clone(),
        RewardType::Performance,
        RewardLevel::Gold,
        100.0,
        "Stats test".to_string(),
        HashMap::new(),
    )
    .await;

    let stats = get_reward_statistics().await;
    assert!(!stats.is_empty());
    let key = "Performance_Gold".to_string();
    assert!(stats.contains_key(&key));
    // Performance multiplier = 1.5, Gold level = 3, base = 100.0
    // amount = 100.0 * 1.5 * 3.0 = 450.0
    // Note: stats may accumulate from other tests, so check >= expected
    assert!(stats[&key] >= 450.0);
}

#[tokio::test]
async fn test_get_top_users() {
    let user1 = "top_user_1".to_string();
    let user2 = "top_user_2".to_string();

    // Create more rewards for user1
    for _ in 0..3 {
        create_reward(
            user1.clone(),
            RewardType::Performance,
            RewardLevel::Gold,
            100.0,
            "Top user test".to_string(),
            HashMap::new(),
        )
        .await;
    }

    // Create fewer rewards for user2
    create_reward(
        user2.clone(),
        RewardType::Performance,
        RewardLevel::Silver,
        50.0,
        "Top user test".to_string(),
        HashMap::new(),
    )
    .await;

    let top_users = get_top_users(10).await;
    assert!(!top_users.is_empty());
    // user1 should have more rewards than user2
    let user1_total: f64 = top_users
        .iter()
        .find(|(id, _)| id == &user1)
        .map(|(_, total)| *total)
        .unwrap_or(0.0);
    let user2_total: f64 = top_users
        .iter()
        .find(|(id, _)| id == &user2)
        .map(|(_, total)| *total)
        .unwrap_or(0.0);
    assert!(user1_total > user2_total);
}

#[tokio::test]
async fn test_award_performance_bonus() {
    let user_id = "bonus_user".to_string();

    // Test high performance
    award_performance_bonus(user_id.clone(), 0.95).await;
    let rewards = get_user_rewards(&user_id).await;
    assert!(!rewards.is_empty());
    assert!(rewards
        .iter()
        .any(|r| r.description.contains("Outstanding")));

    // Test medium performance
    let user_id2 = "bonus_user_2".to_string();
    award_performance_bonus(user_id2.clone(), 0.85).await;
    let rewards2 = get_user_rewards(&user_id2).await;
    assert!(!rewards2.is_empty());
    assert!(rewards2.iter().any(|r| r.description.contains("Good")));

    // Test low performance (no bonus)
    let user_id3 = "bonus_user_3".to_string();
    award_performance_bonus(user_id3.clone(), 0.5).await;
    let rewards3 = get_user_rewards(&user_id3).await;
    assert!(rewards3.is_empty());
}

#[tokio::test]
async fn test_reward_level_ordering() {
    assert!(RewardLevel::Bronze < RewardLevel::Silver);
    assert!(RewardLevel::Silver < RewardLevel::Gold);
    assert!(RewardLevel::Gold < RewardLevel::Platinum);
    assert!(RewardLevel::Platinum < RewardLevel::Diamond);
}

#[tokio::test]
async fn test_reward_type_equality() {
    assert_eq!(RewardType::Performance, RewardType::Performance);
    assert_ne!(RewardType::Performance, RewardType::Efficiency);
}
