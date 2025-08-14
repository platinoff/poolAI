// rewards/mod.rs
// Система нагород для Stage 3 (ендорфін-базировані нагороди)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

// Типи нагород
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RewardType {
    Performance,    // За високу продуктивність
    Efficiency,     // За ефективність використання ресурсів
    Quality,        // За якість результатів
    Innovation,     // За інноваційні рішення
    Collaboration,  // За співпрацю
    Maintenance,    // За обслуговування системи
}

// Рівні нагород
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RewardLevel {
    Bronze = 1,    // 1x базова нагорода
    Silver = 2,    // 2x базова нагорода
    Gold = 3,      // 3x базова нагорода
    Platinum = 4,  // 4x базова нагорода
    Diamond = 5,   // 5x базова нагорода
}

// Структура нагороди
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

// Структура прогресу користувача
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

// Система нагород
pub struct RewardSystem {
    rewards: Arc<RwLock<HashMap<String, Reward>>>,
    user_progress: Arc<RwLock<HashMap<String, UserProgress>>>,
    reward_multipliers: Arc<RwLock<HashMap<RewardType, f64>>>,
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
            reward_multipliers: Arc::new(RwLock::new(multipliers)),
        }
    }

    // Створення нової нагороди
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
            level: level.clone(),
            amount,
            description,
            timestamp: Utc::now(),
            metadata,
        };

        // Зберігаємо нагороду
        let mut rewards = self.rewards.write().await;
        rewards.insert(reward.id.clone(), reward.clone());

        // Оновлюємо прогрес користувача
        self.update_user_progress(&user_id, &reward).await;

        reward
    }

    // Отримання множника нагороди
    async fn get_reward_multiplier(&self, reward_type: &RewardType) -> f64 {
        let multipliers = self.reward_multipliers.read().await;
        *multipliers.get(reward_type).unwrap_or(&1.0)
    }

    // Оновлення прогресу користувача
    async fn update_user_progress(&self, user_id: &str, reward: &Reward) {
        let mut progress_map = self.user_progress.write().await;
        
        let progress = progress_map.entry(user_id.to_string()).or_insert_with(|| UserProgress {
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

        // Оновлюємо загальну суму та кількість нагород
        progress.total_rewards += reward.amount;
        progress.reward_count += 1;

        // Оновлюємо streak
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

        // Оновлюємо досвід та рівень
        progress.experience += (reward.amount * 100.0) as u64;
        progress.level = (progress.experience / 1000) as u32 + 1;

        // Перевіряємо досягнення
        self.check_achievements(progress).await;
    }

    // Перевірка досягнень
    async fn check_achievements(&self, progress: &mut UserProgress) {
        let mut new_achievements = Vec::new();

        // Досягнення за кількість нагород
        if progress.reward_count >= 10 && !progress.achievements.contains(&"First Decade".to_string()) {
            new_achievements.push("First Decade".to_string());
        }
        if progress.reward_count >= 50 && !progress.achievements.contains(&"Half Century".to_string()) {
            new_achievements.push("Half Century".to_string());
        }
        if progress.reward_count >= 100 && !progress.achievements.contains(&"Century".to_string()) {
            new_achievements.push("Century".to_string());
        }

        // Досягнення за streak
        if progress.current_streak >= 7 && !progress.achievements.contains(&"Week Warrior".to_string()) {
            new_achievements.push("Week Warrior".to_string());
        }
        if progress.current_streak >= 30 && !progress.achievements.contains(&"Monthly Master".to_string()) {
            new_achievements.push("Monthly Master".to_string());
        }

        // Досягнення за рівень
        if progress.level >= 5 && !progress.achievements.contains(&"Level 5".to_string()) {
            new_achievements.push("Level 5".to_string());
        }
        if progress.level >= 10 && !progress.achievements.contains(&"Level 10".to_string()) {
            new_achievements.push("Level 10".to_string());
        }

        // Додаємо нові досягнення
        progress.achievements.extend(new_achievements);
    }

    // Отримання нагород користувача
    pub async fn get_user_rewards(&self, user_id: &str) -> Vec<Reward> {
        let rewards = self.rewards.read().await;
        rewards
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect()
    }

    // Отримання прогресу користувача
    pub async fn get_user_progress(&self, user_id: &str) -> Option<UserProgress> {
        let progress = self.user_progress.read().await;
        progress.get(user_id).cloned()
    }

    // Отримання статистики нагород
    pub async fn get_reward_statistics(&self) -> HashMap<String, f64> {
        let rewards = self.rewards.read().await;
        let mut stats = HashMap::new();

        for reward in rewards.values() {
            let key = format!("{:?}_{:?}", reward.reward_type, reward.level);
            *stats.entry(key).or_insert(0.0) += reward.amount;
        }

        stats
    }

    // Отримання топ користувачів
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

    // Очищення старих нагород
    pub async fn cleanup_old_rewards(&self, days_old: i64) {
        let cutoff_date = Utc::now() - chrono::Duration::days(days_old);
        let mut rewards = self.rewards.write().await;
        
        rewards.retain(|_, reward| reward.timestamp > cutoff_date);
    }
}

// Глобальний екземпляр системи нагород
lazy_static::lazy_static! {
    static ref REWARD_SYSTEM: RewardSystem = RewardSystem::new();
}

// Публічні функції для доступу до системи нагород
pub async fn create_reward(
    user_id: String,
    reward_type: RewardType,
    level: RewardLevel,
    base_amount: f64,
    description: String,
    metadata: HashMap<String, String>,
) -> Reward {
    REWARD_SYSTEM.create_reward(user_id, reward_type, level, base_amount, description, metadata).await
}

pub async fn get_user_rewards(user_id: &str) -> Vec<Reward> {
    REWARD_SYSTEM.get_user_rewards(user_id).await
}

pub async fn get_user_progress(user_id: &str) -> Option<UserProgress> {
    REWARD_SYSTEM.get_user_progress(user_id).await
}

pub async fn get_reward_statistics() -> HashMap<String, f64> {
    REWARD_SYSTEM.get_reward_statistics().await
}

pub async fn get_top_users(limit: usize) -> Vec<(String, f64)> {
    REWARD_SYSTEM.get_top_users(limit).await
}

// Функція для автоматичного нагородження за продуктивність
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
        ).await;
    } else if performance_score >= 0.8 {
        let metadata = HashMap::new();
        create_reward(
            user_id,
            RewardType::Performance,
            RewardLevel::Silver,
            50.0,
            "Good Performance Bonus".to_string(),
            metadata,
        ).await;
    }
}
