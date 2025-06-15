use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorphinConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub base_amount: u64,
    pub multiplier: f64,
    pub cooldown_period: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorphinStats {
    pub total_rewards: u64,
    pub total_amount: u64,
    pub successful_rewards: u64,
    pub failed_rewards: u64,
    pub last_reward_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub current_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndorphinMetrics {
    pub config: EndorphinConfig,
    pub stats: EndorphinStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub user_id: String,
    pub endorphin_id: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

pub struct EndorphinSystem {
    endorphins: Arc<Mutex<HashMap<String, EndorphinMetrics>>>,
    rewards: Arc<Mutex<HashMap<String, Reward>>>,
}

impl EndorphinSystem {
    pub fn new() -> Self {
        Self {
            endorphins: Arc::new(Mutex::new(HashMap::new())),
            rewards: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_endorphin(&self, config: EndorphinConfig) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        
        if endorphins.contains_key(&config.id) {
            return Err(format!("Endorphin '{}' already exists", config.id));
        }

        let metrics = EndorphinMetrics {
            config,
            stats: EndorphinStats {
                total_rewards: 0,
                total_amount: 0,
                successful_rewards: 0,
                failed_rewards: 0,
                last_reward_time: None,
                last_error: None,
                current_multiplier: 1.0,
            },
        };

        endorphins.insert(metrics.config.id.clone(), metrics);
        info!("Added new endorphin: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_endorphin(&self, id: &str) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        let mut rewards = self.rewards.lock().await;
        
        if !endorphins.contains_key(id) {
            return Err(format!("Endorphin '{}' not found", id));
        }

        // Remove associated rewards
        rewards.retain(|_, r| r.endorphin_id != id);
        
        endorphins.remove(id);
        info!("Removed endorphin: {}", id);
        Ok(())
    }

    pub async fn distribute_reward(
        &self,
        user_id: &str,
        endorphin_id: &str,
        amount: u64,
    ) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        let mut rewards = self.rewards.lock().await;
        
        let endorphin = endorphins
            .get_mut(endorphin_id)
            .ok_or_else(|| format!("Endorphin '{}' not found", endorphin_id))?;

        if !endorphin.config.active {
            return Err("Endorphin is not active".to_string());
        }

        let reward = Reward {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            endorphin_id: endorphin_id.to_string(),
            amount: (amount as f64 * endorphin.stats.current_multiplier) as u64,
            timestamp: Utc::now(),
            status: "pending".to_string(),
        };

        rewards.insert(reward.id.clone(), reward.clone());
        endorphin.stats.total_rewards += 1;
        endorphin.stats.total_amount += reward.amount;

        info!(
            "Distributed reward: {} for endorphin: {} (amount: {})",
            reward.id, endorphin_id, reward.amount
        );
        Ok(())
    }

    pub async fn process_reward(&self, reward_id: &str) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        let mut rewards = self.rewards.lock().await;
        
        let reward = rewards
            .get_mut(reward_id)
            .ok_or_else(|| format!("Reward '{}' not found", reward_id))?;

        let endorphin = endorphins
            .get_mut(&reward.endorphin_id)
            .ok_or_else(|| format!("Endorphin '{}' not found", reward.endorphin_id))?;

        if !endorphin.config.active {
            return Err("Endorphin is not active".to_string());
        }

        let start_time = Utc::now();

        match self.execute_reward(reward, &endorphin.config).await {
            Ok(_) => {
                reward.status = "completed".to_string();
                endorphin.stats.successful_rewards += 1;
            }
            Err(e) => {
                reward.status = "failed".to_string();
                endorphin.stats.failed_rewards += 1;
                endorphin.stats.last_error = Some(e);
            }
        }

        endorphin.stats.last_reward_time = Some(start_time);
        info!("Processed reward: {}", reward_id);
        Ok(())
    }

    async fn execute_reward(
        &self,
        reward: &Reward,
        config: &EndorphinConfig,
    ) -> Result<(), String> {
        // Simulate reward execution
        let reward_amount = (reward.amount as f64 * config.multiplier) as u64;
        
        // Simulate network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        info!(
            "Executed reward: {} for user: {} (amount: {})",
            reward.id, reward.user_id, reward_amount
        );
        Ok(())
    }

    pub async fn get_endorphin(&self, id: &str) -> Result<EndorphinMetrics, String> {
        let endorphins = self.endorphins.lock().await;
        
        endorphins
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Endorphin '{}' not found", id))
    }

    pub async fn get_all_endorphins(&self) -> Vec<EndorphinMetrics> {
        let endorphins = self.endorphins.lock().await;
        endorphins.values().cloned().collect()
    }

    pub async fn get_active_endorphins(&self) -> Vec<EndorphinMetrics> {
        let endorphins = self.endorphins.lock().await;
        endorphins
            .values()
            .filter(|e| e.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_rewards(&self, endorphin_id: &str) -> Vec<Reward> {
        let rewards = self.rewards.lock().await;
        rewards
            .values()
            .filter(|r| r.endorphin_id == endorphin_id)
            .cloned()
            .collect()
    }

    pub async fn set_endorphin_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        
        let endorphin = endorphins
            .get_mut(id)
            .ok_or_else(|| format!("Endorphin '{}' not found", id))?;

        endorphin.config.active = active;
        info!(
            "Endorphin '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_endorphin_config(&self, id: &str, new_config: EndorphinConfig) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        
        let endorphin = endorphins
            .get_mut(id)
            .ok_or_else(|| format!("Endorphin '{}' not found", id))?;

        endorphin.config = new_config;
        info!("Updated endorphin configuration: {}", id);
        Ok(())
    }

    pub async fn update_multiplier(&self, id: &str, multiplier: f64) -> Result<(), String> {
        let mut endorphins = self.endorphins.lock().await;
        
        let endorphin = endorphins
            .get_mut(id)
            .ok_or_else(|| format!("Endorphin '{}' not found", id))?;

        if multiplier <= 0.0 {
            return Err("Multiplier must be greater than 0".to_string());
        }

        endorphin.stats.current_multiplier = multiplier;
        info!("Updated multiplier for endorphin: {} to {}", id, multiplier);
        Ok(())
    }
} 