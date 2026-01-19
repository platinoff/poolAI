//! Rewards module for Stage 3 - Endorphin-based reward system
//!
//! Provides a comprehensive reward system with multiple reward types, user progress
//! tracking, achievement-based rewards, and reward history/statistics.
//!
//! # Features
//!
//! - **Multiple Reward Types**: Performance, Efficiency, Quality, Innovation, Collaboration, Maintenance
//! - **Reward Levels**: Bronze, Silver, Gold, Platinum, Diamond with multipliers
//! - **User Progress Tracking**: Total rewards, streaks, achievements, levels, experience
//! - **Achievement System**: Automatic achievement unlocking based on milestones
//! - **Statistics**: Reward statistics and top users leaderboard
//!
//! # Example
//!
//! ```no_run
//! use poolai::rewards::{create_reward, get_user_progress, RewardType, RewardLevel};
//! use std::collections::HashMap;
//!
//! # async fn example() {
//! // Create a reward for a user
//! let reward = create_reward(
//!     "user123".to_string(),
//!     RewardType::Performance,
//!     RewardLevel::Gold,
//!     100.0,
//!     "Outstanding performance".to_string(),
//!     HashMap::new(),
//! ).await;
//!
//! println!("Reward amount: {}", reward.amount);
//!
//! // Get user progress
//! if let Some(progress) = get_user_progress("user123").await {
//!     println!("Total rewards: {}", progress.total_rewards);
//!     println!("Level: {}", progress.level);
//!     println!("Achievements: {:?}", progress.achievements);
//! }
//! # }
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Reward types
///
/// Different types of rewards that can be awarded to users based on their activities.
///
/// # Example
///
/// ```rust
/// use poolai::rewards::RewardType;
///
/// let reward_type = RewardType::Performance;
/// match reward_type {
///     RewardType::Performance => println!("High performance reward"),
///     RewardType::Efficiency => println!("Efficient resource usage"),
///     RewardType::Quality => println!("Quality result"),
///     RewardType::Innovation => println!("Innovative solution"),
///     RewardType::Collaboration => println!("Great collaboration"),
///     RewardType::Maintenance => println!("System maintenance"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RewardType {
    Performance,   // For high performance
    Efficiency,    // For efficient resource usage
    Quality,       // For result quality
    Innovation,    // For innovative solutions
    Collaboration, // For collaboration
    Maintenance,   // For system maintenance
}

/// Reward levels
///
/// Different levels of rewards with increasing multipliers:
/// - Bronze: 1x base reward
/// - Silver: 2x base reward
/// - Gold: 3x base reward
/// - Platinum: 4x base reward
/// - Diamond: 5x base reward
///
/// # Example
///
/// ```rust
/// use poolai::rewards::RewardLevel;
///
/// let level = RewardLevel::Gold;
/// let multiplier = level as u8 as f64; // 3.0
/// println!("Level multiplier: {}", multiplier);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RewardLevel {
    Bronze = 1,   // 1x base reward
    Silver = 2,   // 2x base reward
    Gold = 3,     // 3x base reward
    Platinum = 4, // 4x base reward
    Diamond = 5,  // 5x base reward
}

/// Reward structure
///
/// Represents a single reward awarded to a user with type, level, amount,
/// description, timestamp, and optional metadata.
///
/// # Example
///
/// ```rust
/// use poolai::rewards::{Reward, RewardType, RewardLevel};
/// use chrono::Utc;
/// use std::collections::HashMap;
///
/// let reward = Reward {
///     id: "reward-123".to_string(),
///     user_id: "user-456".to_string(),
///     reward_type: RewardType::Performance,
///     level: RewardLevel::Gold,
///     amount: 450.0,
///     description: "Outstanding performance".to_string(),
///     timestamp: Utc::now(),
///     metadata: HashMap::new(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub user_id: String,
    pub reward_type: RewardType,
    pub level: RewardLevel,
    pub amount: f64,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// User progress structure
///
/// Tracks a user's progress including total rewards, streaks, achievements,
/// level, and experience points.
///
/// # Example
///
/// ```rust
/// use poolai::rewards::UserProgress;
/// use chrono::Utc;
///
/// let progress = UserProgress {
///     user_id: "user-123".to_string(),
///     total_rewards: 1000.0,
///     reward_count: 10,
///     current_streak: 5,
///     longest_streak: 7,
///     last_reward_date: Some(Utc::now()),
///     achievements: vec!["First Decade".to_string()],
///     level: 5,
///     experience: 5000,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub total_rewards: f64,
    pub reward_count: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub last_reward_date: Option<DateTime<Utc>>,
    pub achievements: Vec<String>,
    pub level: u32,
    pub experience: u64,
}

/// Reward system
///
/// Manages rewards, user progress, and achievements. Provides methods for
/// creating rewards, tracking progress, and generating statistics.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::RewardSystem;
///
/// # async fn example() {
/// let system = RewardSystem::new();
/// // System is automatically initialized and ready to use
/// # }
/// ```
pub struct RewardSystem {
    rewards: Arc<RwLock<HashMap<String, Reward>>>,
    user_progress: Arc<RwLock<HashMap<String, UserProgress>>>,
    _reward_multipliers: Arc<RwLock<HashMap<RewardType, f64>>>,
}

impl Default for RewardSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RewardSystem {
    pub fn new() -> Self {
        let mut multipliers = HashMap::new();
        multipliers.insert(RewardType::Performance, 1.5);
        multipliers.insert(RewardType::Efficiency, 1.3);
        multipliers.insert(RewardType::Quality, 1.4);
        multipliers.insert(RewardType::Innovation, 2.0);
        multipliers.insert(RewardType::Collaboration, 1.2);
        multipliers.insert(RewardType::Maintenance, 1.1);

        Self {
            rewards: Arc::new(RwLock::new(HashMap::new())),
            user_progress: Arc::new(RwLock::new(HashMap::new())),
            _reward_multipliers: Arc::new(RwLock::new(multipliers)),
        }
    }

    // Create a new reward
    pub async fn create_reward(
        &self,
        user_id: String,
        reward_type: RewardType,
        level: RewardLevel,
        base_amount: f64,
        description: String,
        metadata: HashMap<String, String>,
    ) -> Reward {
        let multiplier = self.get_reward_multiplier(&reward_type).await;
        let amount = base_amount * multiplier * (level as u8 as f64);

        let reward = Reward {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            reward_type: reward_type.clone(),
            level: level,
            amount,
            description,
            timestamp: Utc::now(),
            metadata,
        };

        // Save the reward
        let mut rewards = self.rewards.write().await;
        rewards.insert(reward.id.clone(), reward.clone());

        // Update user progress
        self.update_user_progress(&user_id, &reward).await;

        reward
    }

    // Get reward multiplier
    async fn get_reward_multiplier(&self, reward_type: &RewardType) -> f64 {
        let multipliers = self._reward_multipliers.read().await;
        *multipliers.get(reward_type).unwrap_or(&1.0)
    }

    // Update user progress
    async fn update_user_progress(&self, user_id: &str, reward: &Reward) {
        let mut progress_map = self.user_progress.write().await;

        let progress = progress_map
            .entry(user_id.to_string())
            .or_insert_with(|| UserProgress {
                user_id: user_id.to_string(),
                total_rewards: 0.0,
                reward_count: 0,
                current_streak: 0,
                longest_streak: 0,
                last_reward_date: None,
                achievements: Vec::new(),
                level: 1,
                experience: 0,
            });

        // Update total amount and reward count
        progress.total_rewards += reward.amount;
        progress.reward_count += 1;

        // Update streak
        let now = Utc::now();
        if let Some(last_date) = progress.last_reward_date {
            let days_diff = (now - last_date).num_days();
            if days_diff <= 1 {
                progress.current_streak += 1;
                if progress.current_streak > progress.longest_streak {
                    progress.longest_streak = progress.current_streak;
                }
            } else {
                progress.current_streak = 1;
            }
        } else {
            progress.current_streak = 1;
        }

        progress.last_reward_date = Some(now);

        // Update experience and level
        progress.experience += (reward.amount * 100.0) as u64;
        progress.level = (progress.experience / 1000) as u32 + 1;

        // Check achievements
        self.check_achievements(progress).await;
    }

    // Check achievements
    async fn check_achievements(&self, progress: &mut UserProgress) {
        let mut new_achievements = Vec::new();

        // Achievements for reward count
        if progress.reward_count >= 10
            && !progress.achievements.contains(&"First Decade".to_string())
        {
            new_achievements.push("First Decade".to_string());
        }
        if progress.reward_count >= 50
            && !progress.achievements.contains(&"Half Century".to_string())
        {
            new_achievements.push("Half Century".to_string());
        }
        if progress.reward_count >= 100 && !progress.achievements.contains(&"Century".to_string()) {
            new_achievements.push("Century".to_string());
        }

        // Achievements for streak
        if progress.current_streak >= 7
            && !progress.achievements.contains(&"Week Warrior".to_string())
        {
            new_achievements.push("Week Warrior".to_string());
        }
        if progress.current_streak >= 30
            && !progress
                .achievements
                .contains(&"Monthly Master".to_string())
        {
            new_achievements.push("Monthly Master".to_string());
        }

        // Achievements for level
        if progress.level >= 5 && !progress.achievements.contains(&"Level 10".to_string()) {
            new_achievements.push("Level 5".to_string());
        }
        if progress.level >= 10 && !progress.achievements.contains(&"Level 10".to_string()) {
            new_achievements.push("Level 10".to_string());
        }

        // Add new achievements
        progress.achievements.extend(new_achievements);
    }

    // Get user rewards
    pub async fn get_user_rewards(&self, user_id: &str) -> Vec<Reward> {
        let rewards = self.rewards.read().await;
        rewards
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect()
    }

    // Get user progress
    pub async fn get_user_progress(&self, user_id: &str) -> Option<UserProgress> {
        let progress = self.user_progress.read().await;
        progress.get(user_id).cloned()
    }

    // Get reward statistics
    pub async fn get_reward_statistics(&self) -> HashMap<String, f64> {
        let rewards = self.rewards.read().await;
        let mut stats = HashMap::new();

        for reward in rewards.values() {
            let key = format!("{:?}_{:?}", reward.reward_type, reward.level);
            *stats.entry(key).or_insert(0.0) += reward.amount;
        }

        stats
    }

    // Get top users
    pub async fn get_top_users(&self, limit: usize) -> Vec<(String, f64)> {
        let progress = self.user_progress.read().await;
        let mut users: Vec<(String, f64)> = progress
            .values()
            .map(|p| (p.user_id.clone(), p.total_rewards))
            .collect();

        users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        users.truncate(limit);
        users
    }

    // Clean up old rewards
    pub async fn cleanup_old_rewards(&self, days_old: i64) {
        let cutoff_date = Utc::now() - chrono::Duration::days(days_old);
        let mut rewards = self.rewards.write().await;

        rewards.retain(|_, reward| reward.timestamp > cutoff_date);
    }
}

// Global instance of the reward system
lazy_static::lazy_static! {
    static ref REWARD_SYSTEM: RewardSystem = RewardSystem::new();
}

/// Create a new reward for a user
///
/// Creates a reward with the specified type, level, and base amount.
/// The final amount is calculated using reward type multiplier and level multiplier.
///
/// # Arguments
///
/// * `user_id` - ID of the user receiving the reward
/// * `reward_type` - Type of reward (Performance, Efficiency, etc.)
/// * `level` - Reward level (Bronze, Silver, Gold, etc.)
/// * `base_amount` - Base reward amount before multipliers
/// * `description` - Description of the reward
/// * `metadata` - Optional metadata as key-value pairs
///
/// # Returns
///
/// Returns the created `Reward` with calculated amount.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::{create_reward, RewardType, RewardLevel};
/// use std::collections::HashMap;
///
/// # async fn example() {
/// let reward = create_reward(
///     "user123".to_string(),
///     RewardType::Performance,
///     RewardLevel::Gold,
///     100.0,
///     "Outstanding performance".to_string(),
///     HashMap::new(),
/// ).await;
///
/// // Performance multiplier = 1.5, Gold level = 3
/// // Final amount = 100.0 * 1.5 * 3.0 = 450.0
/// println!("Reward amount: {}", reward.amount);
/// # }
/// ```
pub async fn create_reward(
    user_id: String,
    reward_type: RewardType,
    level: RewardLevel,
    base_amount: f64,
    description: String,
    metadata: HashMap<String, String>,
) -> Reward {
    REWARD_SYSTEM
        .create_reward(
            user_id,
            reward_type,
            level,
            base_amount,
            description,
            metadata,
        )
        .await
}

/// Get all rewards for a user
///
/// Retrieves all rewards awarded to a specific user.
///
/// # Arguments
///
/// * `user_id` - ID of the user
///
/// # Returns
///
/// Returns a vector of all rewards for the user.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::get_user_rewards;
///
/// # async fn example() {
/// let rewards = get_user_rewards("user123").await;
/// println!("User has {} rewards", rewards.len());
/// # }
/// ```
pub async fn get_user_rewards(user_id: &str) -> Vec<Reward> {
    REWARD_SYSTEM.get_user_rewards(user_id).await
}

/// Get user progress
///
/// Retrieves the progress information for a user including total rewards,
/// streaks, achievements, level, and experience.
///
/// # Arguments
///
/// * `user_id` - ID of the user
///
/// # Returns
///
/// Returns `Some(UserProgress)` if user has progress, `None` otherwise.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::get_user_progress;
///
/// # async fn example() {
/// if let Some(progress) = get_user_progress("user123").await {
///     println!("Total rewards: {}", progress.total_rewards);
///     println!("Level: {}", progress.level);
///     println!("Current streak: {}", progress.current_streak);
///     println!("Achievements: {:?}", progress.achievements);
/// }
/// # }
/// ```
pub async fn get_user_progress(user_id: &str) -> Option<UserProgress> {
    REWARD_SYSTEM.get_user_progress(user_id).await
}

/// Get reward statistics
///
/// Returns aggregated statistics for all rewards grouped by reward type and level.
///
/// # Returns
///
/// Returns a HashMap with keys in format "{RewardType}_{RewardLevel}" and
/// values as total reward amounts.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::get_reward_statistics;
///
/// # async fn example() {
/// let stats = get_reward_statistics().await;
/// for (key, amount) in stats {
///     println!("{}: {}", key, amount);
/// }
/// # }
/// ```
pub async fn get_reward_statistics() -> HashMap<String, f64> {
    REWARD_SYSTEM.get_reward_statistics().await
}

/// Get top users by total rewards
///
/// Returns a list of top users sorted by total rewards in descending order.
///
/// # Arguments
///
/// * `limit` - Maximum number of users to return
///
/// # Returns
///
/// Returns a vector of tuples (user_id, total_rewards) sorted by total rewards.
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::get_top_users;
///
/// # async fn example() {
/// let top_users = get_top_users(10).await;
/// for (user_id, total) in top_users {
///     println!("User {}: {} total rewards", user_id, total);
/// }
/// # }
/// ```
pub async fn get_top_users(limit: usize) -> Vec<(String, f64)> {
    REWARD_SYSTEM.get_top_users(limit).await
}

/// Award performance bonus automatically
///
/// Automatically awards a performance bonus to a user based on their performance score.
/// Awards Gold level for scores >= 0.9 and Silver level for scores >= 0.8.
///
/// # Arguments
///
/// * `user_id` - ID of the user
/// * `performance_score` - Performance score (0.0 - 1.0)
///
/// # Example
///
/// ```no_run
/// use poolai::rewards::award_performance_bonus;
///
/// # async fn example() {
/// // Award bonus for high performance
/// award_performance_bonus("user123".to_string(), 0.95).await;
/// # }
/// ```
pub async fn award_performance_bonus(user_id: String, performance_score: f64) {
    if performance_score >= 0.9 {
        let metadata = HashMap::new();
        create_reward(
            user_id,
            RewardType::Performance,
            RewardLevel::Gold,
            100.0,
            "Outstanding Performance Bonus".to_string(),
            metadata,
        )
        .await;
    } else if performance_score >= 0.8 {
        let metadata = HashMap::new();
        create_reward(
            user_id,
            RewardType::Performance,
            RewardLevel::Silver,
            50.0,
            "Good Performance Bonus".to_string(),
            metadata,
        )
        .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_reward() {
        let user_id = "test_user".to_string();
        let reward = create_reward(
            user_id.clone(),
            RewardType::Performance,
            RewardLevel::Gold,
            100.0,
            "Test reward".to_string(),
            HashMap::new(),
        )
        .await;

        assert_eq!(reward.user_id, user_id);
        assert_eq!(reward.reward_type, RewardType::Performance);
        assert_eq!(reward.level, RewardLevel::Gold);
        // Performance multiplier = 1.5, Gold level = 3, base = 100.0
        // amount = 100.0 * 1.5 * 3.0 = 450.0
        assert_eq!(reward.amount, 450.0);
        assert_eq!(reward.description, "Test reward");
    }

    #[tokio::test]
    async fn test_reward_level_multipliers() {
        let base_amount = 100.0;
        // Performance multiplier = 1.5
        let performance_multiplier = 1.5;
        let test_cases = vec![
            (RewardLevel::Bronze, 1.0),
            (RewardLevel::Silver, 2.0),
            (RewardLevel::Gold, 3.0),
            (RewardLevel::Platinum, 4.0),
            (RewardLevel::Diamond, 5.0),
        ];

        for (level, level_multiplier) in test_cases {
            let reward = create_reward(
                "test_user".to_string(),
                RewardType::Performance,
                level,
                base_amount,
                format!("Test {:?}", level),
                HashMap::new(),
            )
            .await;

            // amount = base_amount * performance_multiplier * level_multiplier
            let expected = base_amount * performance_multiplier * level_multiplier;
            assert_eq!(reward.amount, expected);
        }
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
        assert_eq!(rewards.len(), 2);
        assert!(rewards.iter().all(|r| r.user_id == user_id));
    }

    #[tokio::test]
    async fn test_user_progress_tracking() {
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
    }

    #[tokio::test]
    async fn test_reward_statistics() {
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
    async fn test_top_users() {
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

    #[test]
    fn test_reward_level_ordering() {
        assert!(RewardLevel::Bronze < RewardLevel::Silver);
        assert!(RewardLevel::Silver < RewardLevel::Gold);
        assert!(RewardLevel::Gold < RewardLevel::Platinum);
        assert!(RewardLevel::Platinum < RewardLevel::Diamond);
    }

    #[test]
    fn test_reward_type_equality() {
        assert_eq!(RewardType::Performance, RewardType::Performance);
        assert_ne!(RewardType::Performance, RewardType::Efficiency);
    }
}
