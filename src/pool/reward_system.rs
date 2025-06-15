use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use thiserror::Error;
use tokio::sync::Mutex;
use uuid;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use crate::monitoring::metrics::MetricsSystem;

#[derive(Error, Debug)]
pub enum RewardError {
    #[error("Invalid performance value: {0}")]
    InvalidPerformance(f64),
    #[error("Worker not found: {0}")]
    WorkerNotFound(String),
    #[error("Invalid activity type")]
    InvalidActivityType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityType {
    TextGeneration,
    ImageGeneration,
    CodeGeneration,
    ModelTraining,
    DataProcessing,
    SystemMaintenance,
}

#[derive(Debug, Clone)]
pub struct GenerationMetrics {
    pub tokens_per_second: f64,
    pub quality_score: f64,
    pub resource_usage: f64,
    pub completion_time: f64,
}

#[derive(Debug, Clone)]
pub struct EndorphinReward {
    pub amount: f64,
    pub activity_type: ActivityType,
    pub timestamp: DateTime<Utc>,
    pub metrics: Option<GenerationMetrics>,
    pub multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reward_amount: u64,
    pub min_contributions: u32,
    pub max_contributions: u32,
    pub cooldown_period: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardStats {
    pub total_rewards: u64,
    pub total_contributions: u64,
    pub successful_rewards: u64,
    pub failed_rewards: u64,
    pub last_reward_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub current_contributions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardMetrics {
    pub config: RewardConfig,
    pub stats: RewardStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub id: String,
    pub user_id: String,
    pub reward_id: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

pub struct RewardSystem {
    rewards: Arc<Mutex<HashMap<String, RewardMetrics>>>,
    contributions: Arc<Mutex<HashMap<String, Contribution>>>,
}

impl RewardSystem {
    pub fn new() -> Self {
        Self {
            rewards: Arc::new(Mutex::new(HashMap::new())),
            contributions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_reward(&self, config: RewardConfig) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        
        if rewards.contains_key(&config.id) {
            return Err(format!("Reward '{}' already exists", config.id));
        }

        let metrics = RewardMetrics {
            config,
            stats: RewardStats {
                total_rewards: 0,
                total_contributions: 0,
                successful_rewards: 0,
                failed_rewards: 0,
                last_reward_time: None,
                last_error: None,
                current_contributions: 0,
            },
        };

        rewards.insert(metrics.config.id.clone(), metrics);
        info!("Added new reward: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_reward(&self, id: &str) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        let mut contributions = self.contributions.lock().await;
        
        if !rewards.contains_key(id) {
            return Err(format!("Reward '{}' not found", id));
        }

        // Remove associated contributions
        contributions.retain(|_, c| c.reward_id != id);
        
        rewards.remove(id);
        info!("Removed reward: {}", id);
        Ok(())
    }

    pub async fn add_contribution(
        &self,
        user_id: &str,
        reward_id: &str,
        amount: u64,
    ) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        let mut contributions = self.contributions.lock().await;
        
        let reward = rewards
            .get_mut(reward_id)
            .ok_or_else(|| format!("Reward '{}' not found", reward_id))?;

        if !reward.config.active {
            return Err("Reward is not active".to_string());
        }

        if reward.stats.current_contributions >= reward.config.max_contributions {
            return Err("Maximum contributions reached".to_string());
        }

        let contribution = Contribution {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            reward_id: reward_id.to_string(),
            amount,
            timestamp: Utc::now(),
            status: "pending".to_string(),
        };

        contributions.insert(contribution.id.clone(), contribution);
        reward.stats.current_contributions += 1;
        reward.stats.total_contributions += 1;

        info!(
            "Added contribution: {} for reward: {} (amount: {})",
            contribution.id, reward_id, amount
        );
        Ok(())
    }

    pub async fn process_reward(&self, reward_id: &str) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        let mut contributions = self.contributions.lock().await;
        
        let reward = rewards
            .get_mut(reward_id)
            .ok_or_else(|| format!("Reward '{}' not found", reward_id))?;

        if !reward.config.active {
            return Err("Reward is not active".to_string());
        }

        if reward.stats.current_contributions < reward.config.min_contributions {
            return Err("Insufficient contributions".to_string());
        }

        let start_time = Utc::now();

        // Process all pending contributions
        let pending_contributions: Vec<_> = contributions
            .values()
            .filter(|c| c.reward_id == reward_id && c.status == "pending")
            .cloned()
            .collect();

        for contribution in pending_contributions {
            match self.distribute_reward(&contribution, &reward.config).await {
                Ok(_) => {
                    if let Some(c) = contributions.get_mut(&contribution.id) {
                        c.status = "completed".to_string();
                    }
                    reward.stats.successful_rewards += 1;
                }
                Err(e) => {
                    if let Some(c) = contributions.get_mut(&contribution.id) {
                        c.status = "failed".to_string();
                    }
                    reward.stats.failed_rewards += 1;
                    reward.stats.last_error = Some(e);
                }
            }
        }

        reward.stats.total_rewards += 1;
        reward.stats.last_reward_time = Some(start_time);
        reward.stats.current_contributions = 0;

        info!("Processed reward: {}", reward_id);
        Ok(())
    }

    async fn distribute_reward(
        &self,
        contribution: &Contribution,
        config: &RewardConfig,
    ) -> Result<(), String> {
        // Simulate reward distribution
        let reward_amount = (contribution.amount as f64 * config.reward_amount as f64 / 100.0) as u64;
        
        // Simulate network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        info!(
            "Distributed reward: {} to user: {} (amount: {})",
            config.id, contribution.user_id, reward_amount
        );
        Ok(())
    }

    pub async fn get_reward(&self, id: &str) -> Result<RewardMetrics, String> {
        let rewards = self.rewards.lock().await;
        
        rewards
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Reward '{}' not found", id))
    }

    pub async fn get_all_rewards(&self) -> Vec<RewardMetrics> {
        let rewards = self.rewards.lock().await;
        rewards.values().cloned().collect()
    }

    pub async fn get_active_rewards(&self) -> Vec<RewardMetrics> {
        let rewards = self.rewards.lock().await;
        rewards
            .values()
            .filter(|r| r.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_contributions(&self, reward_id: &str) -> Vec<Contribution> {
        let contributions = self.contributions.lock().await;
        contributions
            .values()
            .filter(|c| c.reward_id == reward_id)
            .cloned()
            .collect()
    }

    pub async fn set_reward_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        
        let reward = rewards
            .get_mut(id)
            .ok_or_else(|| format!("Reward '{}' not found", id))?;

        reward.config.active = active;
        info!(
            "Reward '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_reward_config(&self, id: &str, new_config: RewardConfig) -> Result<(), String> {
        let mut rewards = self.rewards.lock().await;
        
        let reward = rewards
            .get_mut(id)
            .ok_or_else(|| format!("Reward '{}' not found", id))?;

        reward.config = new_config;
        info!("Updated reward configuration: {}", id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reward_calculation() {
        let system = RewardSystem::new();
        let reward = system.calculate_reward(ActivityType::Mining, 0.8);
        assert!(reward > 0.0);
    }

    #[test]
    fn test_reward_distribution() {
        let system = RewardSystem::new();
        let result = system.distribute_reward("test_user", ActivityType::Mining, 0.8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_update() {
        let system = RewardSystem::new();
        system.distribute_reward("test_user", ActivityType::Mining, 0.8).unwrap();
        let metrics = system.get_user_metrics("test_user");
        assert!(metrics.is_ok());
    }
} 