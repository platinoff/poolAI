use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use chrono::{DateTime, Utc};

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

    pub async fn add_pool(&self, config: PoolConfig) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        if pools.iter().any(|p| p.config.name == config.name) {
            return Err(format!("Pool '{}' already exists", config.name));
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
            },
        };

        pools.push(metrics);
        info!("Added new pool: {}", metrics.config.name);
        Ok(())
    }

    pub async fn remove_pool(&self, name: &str) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        let initial_len = pools.len();
        pools.retain(|p| p.config.name != name);
        
        if pools.len() == initial_len {
            return Err(format!("Pool '{}' not found", name));
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
    ) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        let pool = pools
            .iter_mut()
            .find(|p| p.config.name == pool_name)
            .ok_or_else(|| format!("Pool '{}' not found", pool_name))?;

        let now = Utc::now();
        let worker_stats = WorkerStats {
            worker_id: worker_id.clone(),
            hashrate,
            shares,
            rejected_shares,
            last_share_time: Some(now),
            uptime: 0, // TODO: Calculate uptime
            memory_usage,
            gpu_usage,
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

    pub async fn get_pool(&self, name: &str) -> Result<PoolMetrics, String> {
        let pools = self.pools.lock().await;
        
        pools
            .iter()
            .find(|p| p.config.name == name)
            .cloned()
            .ok_or_else(|| format!("Pool '{}' not found", name))
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

    pub async fn set_pool_maintenance(&self, name: &str, maintenance: bool) -> Result<(), String> {
        let mut pools = self.pools.lock().await;
        
        let pool = pools
            .iter_mut()
            .find(|p| p.config.name == name)
            .ok_or_else(|| format!("Pool '{}' not found", name))?;

        pool.config.maintenance_mode = maintenance;
        info!(
            "Pool '{}' {}",
            name,
            if maintenance { "entered maintenance mode" } else { "exited maintenance mode" }
        );
        Ok(())
    }

    pub async fn get_worker_stats(&self, pool_name: &str, worker_id: &str) -> Result<WorkerStats, String> {
        let pools = self.pools.lock().await;
        
        let pool = pools
            .iter()
            .find(|p| p.config.name == pool_name)
            .ok_or_else(|| format!("Pool '{}' not found", pool_name))?;

        pool.stats
            .worker_stats
            .iter()
            .find(|w| w.worker_id == worker_id)
            .cloned()
            .ok_or_else(|| format!("Worker '{}' not found in pool '{}'", worker_id, pool_name))
    }

    pub async fn get_pool_stats(&self, name: &str) -> Result<PoolStats, String> {
        let pools = self.pools.lock().await;
        
        let pool = pools
            .iter()
            .find(|p| p.config.name == name)
            .ok_or_else(|| format!("Pool '{}' not found", name))?;

        Ok(pool.stats.clone())
    }
} 