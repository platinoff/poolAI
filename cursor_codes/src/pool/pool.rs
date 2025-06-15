use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::core::error::CursorError;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use crate::runtime::worker::WorkerManager;
use crate::runtime::scheduler::SchedulerSystem;
use crate::runtime::queue::QueueSystem;
use crate::runtime::cache::CacheSystem;
use crate::runtime::storage::StorageSystem;

#[derive(Error, Debug)]
pub enum PoolError {
    #[error("Pool not found: {0}")]
    PoolNotFound(String),
    #[error("Worker not found: {0}")]
    WorkerNotFound(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    #[error("Maintenance mode active: {0}")]
    MaintenanceMode(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub name: String,
    pub url: String,
    pub api_key: String,
    pub min_workers: u32,
    pub max_workers: u32,
    pub min_memory_gb: u32,
    pub max_memory_gb: u32,
    pub allowed_gpu_models: Vec<String>,
    pub maintenance_mode: bool,
    pub algorithm: String,
    pub difficulty: u32,
    pub payout_threshold: f64,
    pub fee_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    pub worker_id: String,
    pub hashrate: f64,
    pub shares: u64,
    pub rejected_shares: u64,
    pub last_share_time: Option<DateTime<Utc>>,
    pub uptime: u64,
    pub memory_usage: u64,
    pub gpu_usage: f64,
    pub temperature: f64,
    pub power_usage: f64,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_workers: u32,
    pub active_workers: u32,
    pub total_hashrate: f64,
    pub total_shares: u64,
    pub rejected_shares: u64,
    pub last_update: DateTime<Utc>,
    pub worker_stats: Vec<WorkerStats>,
    pub network_difficulty: u64,
    pub block_reward: f64,
    pub estimated_daily_reward: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub config: PoolConfig,
    pub stats: PoolStats,
}

pub struct PoolManager {
    pools: Arc<Mutex<Vec<PoolMetrics>>>,
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_pool(&self, config: PoolConfig) -> Result<(), PoolError> {
        let mut pools = self.pools.lock().await;
        
        if pools.iter().any(|p| p.config.name == config.name) {
            return Err(PoolError::InvalidConfig(format!("Pool '{}' already exists", config.name)));
        }

        // Validate configuration
        if config.min_workers > config.max_workers {
            return Err(PoolError::InvalidConfig("min_workers cannot be greater than max_workers".to_string()));
        }
        if config.min_memory_gb > config.max_memory_gb {
            return Err(PoolError::InvalidConfig("min_memory_gb cannot be greater than max_memory_gb".to_string()));
        }

        let metrics = PoolMetrics {
            config,
            stats: PoolStats {
                total_workers: 0,
                active_workers: 0,
                total_hashrate: 0.0,
                total_shares: 0,
                rejected_shares: 0,
                last_update: Utc::now(),
                worker_stats: Vec::new(),
                network_difficulty: 0,
                block_reward: 0.0,
                estimated_daily_reward: 0.0,
            },
        };

        pools.push(metrics);
        info!("Added new pool: {}", metrics.config.name);
        Ok(())
    }

    pub async fn remove_pool(&self, name: &str) -> Result<(), PoolError> {
        let mut pools = self.pools.lock().await;
        
        let initial_len = pools.len();
        pools.retain(|p| p.config.name != name);
        
        if pools.len() == initial_len {
            return Err(PoolError::PoolNotFound(name.to_string()));
        }

        info!("Removed pool: {}", name);
        Ok(())
    }

    pub async fn update_worker_stats(
        &self,
        pool_name: &str,
        worker_id: String,
        hashrate: f64,
        shares: u64,
        rejected_shares: u64,
        memory_usage: u64,
        gpu_usage: f64,
        temperature: f64,
        power_usage: f64,
    ) -> Result<(), PoolError> {
        let mut pools = self.pools.lock().await;
        
        let pool = pools
            .iter_mut()
            .find(|p| p.config.name == pool_name)
            .ok_or_else(|| PoolError::PoolNotFound(pool_name.to_string()))?;

        if pool.config.maintenance_mode {
            return Err(PoolError::MaintenanceMode(pool_name.to_string()));
        }

        let now = Utc::now();
        let efficiency = if power_usage > 0.0 {
            hashrate / power_usage
        } else {
            0.0
        };

        let worker_stats = WorkerStats {
            worker_id: worker_id.clone(),
            hashrate,
            shares,
            rejected_shares,
            last_share_time: Some(now),
            uptime: 0, // TODO: Calculate uptime
            memory_usage,
            gpu_usage,
            temperature,
            power_usage,
            efficiency,
        };

        // Update or add worker stats
        if let Some(existing) = pool.stats.worker_stats.iter_mut().find(|w| w.worker_id == worker_id) {
            *existing = worker_stats;
        } else {
            pool.stats.worker_stats.push(worker_stats);
        }

        // Update pool stats
        pool.stats.total_workers = pool.stats.worker_stats.len() as u32;
        pool.stats.active_workers = pool.stats.worker_stats.iter().filter(|w| w.hashrate > 0.0).count() as u32;
        pool.stats.total_hashrate = pool.stats.worker_stats.iter().map(|w| w.hashrate).sum();
        pool.stats.total_shares = pool.stats.worker_stats.iter().map(|w| w.shares).sum();
        pool.stats.rejected_shares = pool.stats.worker_stats.iter().map(|w| w.rejected_shares).sum();
        pool.stats.last_update = now;

        Ok(())
    }

    pub async fn get_pool(&self, name: &str) -> Result<PoolMetrics, PoolError> {
        let pools = self.pools.lock().await;
        
        pools
            .iter()
            .find(|p| p.config.name == name)
            .cloned()
            .ok_or_else(|| PoolError::PoolNotFound(name.to_string()))
    }

    pub async fn get_all_pools(&self) -> Vec<PoolMetrics> {
        let pools = self.pools.lock().await;
        pools.clone()
    }

    pub async fn get_active_pools(&self) -> Vec<PoolMetrics> {
        let pools = self.pools.lock().await;
        pools
            .iter()
            .filter(|p| !p.config.maintenance_mode)
            .cloned()
            .collect()
    }

    pub async fn set_pool_maintenance(&self, name: &str, maintenance: bool) -> Result<(), PoolError> {
        let mut pools = self.pools.lock().await;
        
        let pool = pools
            .iter_mut()
            .find(|p| p.config.name == name)
            .ok_or_else(|| PoolError::PoolNotFound(name.to_string()))?;

        pool.config.maintenance_mode = maintenance;
        info!(
            "Pool '{}' {}",
            name,
            if maintenance { "entered maintenance mode" } else { "exited maintenance mode" }
        );
        Ok(())
    }

    pub async fn get_worker_stats(&self, pool_name: &str, worker_id: &str) -> Result<WorkerStats, PoolError> {
        let pools = self.pools.lock().await;
        
        let pool = pools
            .iter()
            .find(|p| p.config.name == pool_name)
            .ok_or_else(|| PoolError::PoolNotFound(pool_name.to_string()))?;

        pool.stats
            .worker_stats
            .iter()
            .find(|w| w.worker_id == worker_id)
            .cloned()
            .ok_or_else(|| PoolError::WorkerNotFound(format!("Worker '{}' not found in pool '{}'", worker_id, pool_name)))
    }

    pub async fn get_pool_stats(&self, name: &str) -> Result<PoolStats, PoolError> {
        let pools = self.pools.lock().await;
        
        let pool = pools
            .iter()
            .find(|p| p.config.name == name)
            .ok_or_else(|| PoolError::PoolNotFound(name.to_string()))?;

        Ok(pool.stats.clone())
    }

    pub async fn update_network_stats(
        &self,
        pool_name: &str,
        network_difficulty: u64,
        block_reward: f64,
    ) -> Result<(), PoolError> {
        let mut pools = self.pools.lock().await;
        
        let pool = pools
            .iter_mut()
            .find(|p| p.config.name == pool_name)
            .ok_or_else(|| PoolError::PoolNotFound(pool_name.to_string()))?;

        pool.stats.network_difficulty = network_difficulty;
        pool.stats.block_reward = block_reward;

        // Calculate estimated daily reward based on pool's hashrate
        let hashrate = pool.stats.total_hashrate;
        let seconds_per_day = 86400.0;
        let blocks_per_day = (hashrate * seconds_per_day) / (network_difficulty as f64);
        pool.stats.estimated_daily_reward = blocks_per_day * block_reward;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_management() {
        let manager = PoolManager::new();

        // Test adding a pool
        let config = PoolConfig {
            name: "test_pool".to_string(),
            url: "http://test.com".to_string(),
            api_key: "test_key".to_string(),
            min_workers: 1,
            max_workers: 10,
            min_memory_gb: 4,
            max_memory_gb: 16,
            allowed_gpu_models: vec!["RTX 3080".to_string()],
            maintenance_mode: false,
            algorithm: "ethash".to_string(),
            difficulty: 1,
            payout_threshold: 0.1,
            fee_percentage: 1.0,
        };

        assert!(manager.add_pool(config.clone()).await.is_ok());
        assert!(manager.add_pool(config).await.is_err()); // Should fail - duplicate pool

        // Test getting pool
        let pool = manager.get_pool("test_pool").await.unwrap();
        assert_eq!(pool.config.name, "test_pool");

        // Test removing pool
        assert!(manager.remove_pool("test_pool").await.is_ok());
        assert!(manager.get_pool("test_pool").await.is_err());
    }

    #[tokio::test]
    async fn test_worker_stats() {
        let manager = PoolManager::new();

        // Add a pool
        let config = PoolConfig {
            name: "test_pool".to_string(),
            url: "http://test.com".to_string(),
            api_key: "test_key".to_string(),
            min_workers: 1,
            max_workers: 10,
            min_memory_gb: 4,
            max_memory_gb: 16,
            allowed_gpu_models: vec!["RTX 3080".to_string()],
            maintenance_mode: false,
            algorithm: "ethash".to_string(),
            difficulty: 1,
            payout_threshold: 0.1,
            fee_percentage: 1.0,
        };
        manager.add_pool(config).await.unwrap();

        // Test updating worker stats
        assert!(manager.update_worker_stats(
            "test_pool",
            "worker1".to_string(),
            100.0,
            1000,
            10,
            8192,
            95.0,
            75.0,
            200.0,
        ).await.is_ok());

        // Test getting worker stats
        let stats = manager.get_worker_stats("test_pool", "worker1").await.unwrap();
        assert_eq!(stats.worker_id, "worker1");
        assert_eq!(stats.hashrate, 100.0);
        assert_eq!(stats.shares, 1000);
    }
} 