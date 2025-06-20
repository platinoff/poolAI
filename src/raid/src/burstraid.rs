use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::state::{AppState, NodeStatus};
use tokio::time::sleep;
use log::{info, warn, error};
use std::path::Path;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs;
use std::io;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::fs as tokio_fs;
use std::io::Write;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use reqwest;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;
use cursor_codes::network::network::NetworkSystem;

const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(10);
const NODE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Error, Debug)]
pub enum BurstRaidError {
    #[error("RAID initialization error: {0}")]
    RaidInitError(String),
    #[error("Disk error: {0}")]
    DiskError(String),
    #[error("Worker error: {0}")]
    WorkerError(String),
    #[error("Seed error: {0}")]
    SeedError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug, Clone)]
pub struct RaidConfig {
    pub raid_level: u8,
    pub min_disks: usize,
    pub stripe_size: usize,
    pub redundancy: usize,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub path: String,
    pub size: u64,
    pub status: DiskStatus,
    pub last_seen: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiskStatus {
    Active,
    Degraded,
    Failed,
    Rebuilding,
}

#[derive(Debug, Clone)]
pub struct SeedInfo {
    pub worker_id: String,
    pub path: String,
    pub size: u64,
    pub last_accessed: Instant,
    pub status: SeedStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeedStatus {
    Available,
    Unavailable,
    Migrating,
}

pub struct BurstRaidManager {
    config: RaidConfig,
    disks: Arc<RwLock<HashMap<String, DiskInfo>>>,
    seeds: Arc<RwLock<HashMap<String, SeedInfo>>>,
    model_pool: Arc<RwLock<HashMap<String, String>>>, // model_id -> raid_path
    health_check_tx: mpsc::Sender<()>,
}

impl BurstRaidManager {
    pub fn new(config: RaidConfig) -> Result<Self, BurstRaidError> {
        let (health_check_tx, _) = mpsc::channel(1);
        
        let manager = Self {
            config,
            disks: Arc::new(RwLock::new(HashMap::new())),
            seeds: Arc::new(RwLock::new(HashMap::new())),
            model_pool: Arc::new(RwLock::new(HashMap::new())),
            health_check_tx,
        };

        // Create data directory if it doesn't exist
        fs::create_dir_all("data")?;
        
        Ok(manager)
    }

    pub async fn initialize_raid(&self) -> Result<(), BurstRaidError> {
        info!("Initializing RAID array with level {}", self.config.raid_level);
        
        // Check if we have enough disks
        let disks = self.disks.read();
        if disks.len() < self.config.min_disks {
            return Err(BurstRaidError::RaidInitError(
                format!("Not enough disks. Required: {}, Available: {}", 
                        self.config.min_disks, disks.len())
            ));
        }

        // Create RAID structure
        for (disk_id, disk) in disks.iter() {
            let raid_path = format!("data/raid/{}", disk_id);
            fs::create_dir_all(&raid_path)?;
            
            info!("Initialized disk {} at {}", disk_id, raid_path);
        }

        Ok(())
    }

    pub async fn add_disk(&self, disk_id: String, path: String, size: u64) -> Result<(), BurstRaidError> {
        let mut disks = self.disks.write();
        
        disks.insert(disk_id.clone(), DiskInfo {
            path,
            size,
            status: DiskStatus::Active,
            last_seen: Instant::now(),
        });

        info!("Added disk {} to RAID array", disk_id);
        Ok(())
    }

    pub async fn register_seed(&self, worker_id: String, seed_path: String, size: u64) -> Result<(), BurstRaidError> {
        let mut seeds = self.seeds.write();
        
        seeds.insert(worker_id.clone(), SeedInfo {
            worker_id: worker_id.clone(),
            path: seed_path,
            size,
            last_accessed: Instant::now(),
            status: SeedStatus::Available,
        });

        info!("Registered seed from worker {}", worker_id);
        Ok(())
    }

    pub async fn load_model(&self, model_id: String, model_path: String) -> Result<(), BurstRaidError> {
        let mut model_pool = self.model_pool.write();
        
        // Calculate required space based on model size
        let model_size = fs::metadata(&model_path)?.len();
        let required_disks = (model_size as f64 / self.config.stripe_size as f64).ceil() as usize;
        
        // Check if we have enough disks
        let disks = self.disks.read();
        if disks.len() < required_disks {
            return Err(BurstRaidError::RaidInitError(
                format!("Not enough disks for model. Required: {}, Available: {}", 
                        required_disks, disks.len())
            ));
        }

        // Distribute model across RAID
        let raid_path = format!("data/raid/models/{}", model_id);
        fs::create_dir_all(&raid_path)?;
        
        // Copy model to RAID with striping
        // Implementation depends on specific RAID level
        match self.config.raid_level {
            0 => self.strip_model(&model_path, &raid_path, model_size).await?,
            1 => self.mirror_model(&model_path, &raid_path, model_size).await?,
            _ => return Err(BurstRaidError::RaidInitError(
                format!("Unsupported RAID level: {}", self.config.raid_level)
            )),
        }

        model_pool.insert(model_id, raid_path);
        info!("Loaded model into RAID array");
        Ok(())
    }

    async fn strip_model(&self, source: &str, target: &str, size: u64) -> Result<(), BurstRaidError> {
        let stripe_size = self.config.stripe_size as u64;
        let mut offset = 0;
        let mut disk_index = 0;
        
        // Calculate checksum of source file
        let source_checksum = self.calculate_checksum(source).await?;
        
        while offset < size {
            let current_stripe = std::cmp::min(stripe_size, size - offset);
            
            // Get next available disk
            let disks = self.disks.read();
            let disk_ids: Vec<_> = disks.keys().collect();
            if disk_ids.is_empty() {
                return Err(BurstRaidError::DiskError("No disks available".to_string()));
            }
            
            let disk_id = disk_ids[disk_index % disk_ids.len()];
            let disk = disks.get(disk_id).unwrap();
            
            // Create stripe file
            let stripe_path = format!("{}/stripe_{}", disk.path, offset);
            let mut stripe_file = tokio_fs::File::create(&stripe_path).await?;
            
            // Read and write stripe
            let mut source_file = tokio_fs::File::open(source).await?;
            source_file.seek(io::SeekFrom::Start(offset)).await?;
            
            let mut buffer = vec![0; current_stripe as usize];
            source_file.read_exact(&mut buffer).await?;
            stripe_file.write_all(&buffer).await?;
            
            // Verify stripe checksum
            let stripe_checksum = self.calculate_checksum(&stripe_path).await?;
            if stripe_checksum != source_checksum {
                return Err(BurstRaidError::DiskError(
                    format!("Checksum mismatch for stripe at offset {}", offset)
                ));
            }
            
            offset += current_stripe;
            disk_index += 1;
        }
        
        Ok(())
    }

    async fn mirror_model(&self, source: &str, target: &str, size: u64) -> Result<(), BurstRaidError> {
        // Calculate source checksum
        let source_checksum = self.calculate_checksum(source).await?;
        
        // Get all active disks
        let disks = self.disks.read();
        let active_disks: Vec<_> = disks.iter()
            .filter(|(_, disk)| disk.status == DiskStatus::Active)
            .collect();
            
        if active_disks.is_empty() {
            return Err(BurstRaidError::DiskError("No active disks available".to_string()));
        }
        
        // Copy to each disk
        for (disk_id, disk) in active_disks {
            let mirror_path = format!("{}/{}", target, disk_id);
            tokio_fs::create_dir_all(&mirror_path).await?;
            
            // Copy file
            tokio_fs::copy(source, &mirror_path).await?;
            
            // Verify checksum
            let mirror_checksum = self.calculate_checksum(&mirror_path).await?;
            if mirror_checksum != source_checksum {
                return Err(BurstRaidError::DiskError(
                    format!("Checksum mismatch for mirror on disk {}", disk_id)
                ));
            }
        }
        
        Ok(())
    }

    async fn calculate_checksum(&self, path: &str) -> Result<String, BurstRaidError> {
        let mut file = tokio_fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 1024 * 1024]; // 1MB buffer
        
        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub async fn handle_worker_failure(&self, worker_id: String) -> Result<(), BurstRaidError> {
        let mut seeds = self.seeds.write();
        
        if let Some(seed) = seeds.get_mut(&worker_id) {
            seed.status = SeedStatus::Unavailable;
            warn!("Worker {} failed, marking seed as unavailable", worker_id);
            
            // Start migration process if needed
            self.migrate_seed(worker_id).await?;
        }
        
        Ok(())
    }

    async fn migrate_seed(&self, worker_id: String) -> Result<(), BurstRaidError> {
        let mut seeds = self.seeds.write();
        
        if let Some(seed) = seeds.get_mut(&worker_id) {
            seed.status = SeedStatus::Migrating;
            
            // Find available worker with enough space
            let available_workers: Vec<_> = seeds.iter()
                .filter(|(id, s)| 
                    id != &worker_id && 
                    s.status == SeedStatus::Available &&
                    s.size >= seed.size
                )
                .collect();
                
            if available_workers.is_empty() {
                return Err(BurstRaidError::WorkerError(
                    "No available workers for migration".to_string()
                ));
            }
            
            // Choose worker with most free space
            let target_worker = available_workers.iter()
                .max_by_key(|(_, s)| s.size)
                .unwrap();
                
            // Copy seed data
            let target_path = format!("{}/migrated_{}", target_worker.1.path, worker_id);
            tokio_fs::copy(&seed.path, &target_path).await?;
            
            // Verify checksum
            let source_checksum = self.calculate_checksum(&seed.path).await?;
            let target_checksum = self.calculate_checksum(&target_path).await?;
            
            if source_checksum != target_checksum {
                return Err(BurstRaidError::SeedError(
                    "Checksum mismatch during migration".to_string()
                ));
            }
            
            // Update seed info
            seed.path = target_path;
            seed.status = SeedStatus::Available;
            info!("Successfully migrated seed from worker {}", worker_id);
        }
        
        Ok(())
    }

    pub async fn monitor_health(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            
            let disks = self.disks.read();
            let seeds = self.seeds.read();
            
            // Check disk health
            for (disk_id, disk) in disks.iter() {
                if disk.last_seen.elapsed() > Duration::from_secs(300) {
                    warn!("Disk {} has not been seen for 5 minutes", disk_id);
                }
            }
            
            // Check seed health
            for (worker_id, seed) in seeds.iter() {
                if seed.last_accessed.elapsed() > Duration::from_secs(300) {
                    warn!("Seed from worker {} has not been accessed for 5 minutes", worker_id);
                }
            }
        }
    }

    pub async fn verify_data_integrity(&self) -> Result<(), BurstRaidError> {
        let disks = self.disks.read();
        let model_pool = self.model_pool.read();
        
        for (model_id, raid_path) in model_pool.iter() {
            info!("Verifying integrity for model {}", model_id);
            
            match self.config.raid_level {
                0 => {
                    // Verify all stripes
                    let mut offset = 0;
                    while let Ok(stripe_path) = tokio_fs::read_dir(&raid_path).await {
                        let mut entries = stripe_path.into_iter();
                        while let Some(entry) = entries.next().await {
                            let entry = entry?;
                            let stripe_checksum = self.calculate_checksum(entry.path().to_str().unwrap()).await?;
                            // Compare with original checksum
                            // Implementation depends on how checksums are stored
                        }
                    }
                },
                1 => {
                    // Verify all mirrors
                    for (disk_id, _) in disks.iter() {
                        let mirror_path = format!("{}/{}", raid_path, disk_id);
                        let mirror_checksum = self.calculate_checksum(&mirror_path).await?;
                        // Compare with original checksum
                        // Implementation depends on how checksums are stored
                    }
                },
                _ => return Err(BurstRaidError::RaidInitError(
                    format!("Unsupported RAID level: {}", self.config.raid_level)
                )),
            }
        }
        
        Ok(())
    }
}

pub async fn monitor_health(app_state: Arc<AppState>) {
    info!("Starting RAID health monitoring");
    
    loop {
        let now = Instant::now();
        let mut failed_nodes = Vec::new();
        
        // Check all nodes
        {
            let raid_status = app_state.raid_status.lock();
            for (node_id, node) in raid_status.iter() {
                if now.duration_since(node.last_heartbeat) > NODE_TIMEOUT {
                    warn!("Node {} has timed out", node_id);
                    failed_nodes.push(*node_id);
                }
            }
        }
        
        // Update status for failed nodes
        for node_id in failed_nodes {
            app_state.update_raid_status(node_id, NodeStatus::Failed);
            // TODO: Implement resource redistribution logic
        }
        
        sleep(HEALTH_CHECK_INTERVAL).await;
    }
}

pub fn redistribute_resources(app_state: &Arc<AppState>, failed_node: solana_sdk::pubkey::Pubkey) {
    // Get all active nodes
    let active_nodes: Vec<_> = {
        let raid_status = app_state.raid_status.lock();
        raid_status.iter()
            .filter(|(_, node)| node.status == NodeStatus::Active)
            .map(|(id, _)| *id)
            .collect()
    };
    
    if active_nodes.is_empty() {
        error!("No active nodes available for resource redistribution");
        return;
    }
    
    // Calculate new resource distribution
    let resources_per_node = 1.0 / active_nodes.len() as f64;
    
    // Update resource allocation
    for node_id in active_nodes {
        // TODO: Implement actual resource redistribution logic
        info!("Redistributing resources to node {}", node_id);
    }
}

pub async fn handle_node_failure(app_state: Arc<AppState>, node_id: solana_sdk::pubkey::Pubkey) {
    warn!("Handling failure for node {}", node_id);
    
    // Update node status
    app_state.update_raid_status(node_id, NodeStatus::Failed);
    
    // Redistribute resources
    redistribute_resources(&app_state, node_id);
    
    // TODO: Implement recovery procedures
    info!("Node failure handling completed for {}", node_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_raid_initialization() {
        let config = RaidConfig {
            raid_level: 1,
            min_disks: 2,
            stripe_size: 1024 * 1024, // 1MB
            redundancy: 1,
        };
        
        let manager = BurstRaidManager::new(config).unwrap();
        
        // Add test disks
        manager.add_disk("disk1".to_string(), "data/disk1".to_string(), 1024 * 1024 * 1024).await.unwrap();
        manager.add_disk("disk2".to_string(), "data/disk2".to_string(), 1024 * 1024 * 1024).await.unwrap();
        
        assert!(manager.initialize_raid().await.is_ok());
    }

    #[tokio::test]
    async fn test_seed_registration() {
        let config = RaidConfig {
            raid_level: 1,
            min_disks: 2,
            stripe_size: 1024 * 1024,
            redundancy: 1,
        };
        
        let manager = BurstRaidManager::new(config).unwrap();
        
        assert!(manager.register_seed(
            "worker1".to_string(),
            "data/seeds/worker1".to_string(),
            1024 * 1024
        ).await.is_ok());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstConfig {
    pub id: String,
    pub target_url: String,
    pub concurrent_requests: u32,
    pub request_timeout: u64,
    pub max_retries: u32,
    pub retry_delay: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub current_concurrent_requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstMetrics {
    pub config: BurstConfig,
    pub stats: BurstStats,
}

pub struct BurstRaid {
    bursts: Arc<Mutex<HashMap<String, BurstMetrics>>>,
}

impl BurstRaid {
    pub fn new() -> Self {
        Self {
            bursts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_burst(&self, config: BurstConfig) -> Result<(), String> {
        let mut bursts = self.bursts.lock().await;
        
        if bursts.contains_key(&config.id) {
            return Err(format!("Burst '{}' already exists", config.id));
        }

        let metrics = BurstMetrics {
            config,
            stats: BurstStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
                current_concurrent_requests: 0,
            },
        };

        bursts.insert(metrics.config.id.clone(), metrics);
        info!("Added new burst: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_burst(&self, id: &str) -> Result<(), String> {
        let mut bursts = self.bursts.lock().await;
        
        if !bursts.contains_key(id) {
            return Err(format!("Burst '{}' not found", id));
        }

        bursts.remove(id);
        info!("Removed burst: {}", id);
        Ok(())
    }

    pub async fn execute_burst(&self, id: &str) -> Result<(), String> {
        let mut bursts = self.bursts.lock().await;
        
        let burst = bursts
            .get_mut(id)
            .ok_or_else(|| format!("Burst '{}' not found", id))?;

        if !burst.config.active {
            return Err("Burst is not active".to_string());
        }

        if burst.stats.current_concurrent_requests >= burst.config.concurrent_requests {
            return Err("Maximum concurrent requests reached".to_string());
        }

        burst.stats.current_concurrent_requests += 1;
        let start_time = Utc::now();

        // Simulate request execution
        let result = self.execute_request(&burst.config).await;
        
        let end_time = Utc::now();
        let response_time = (end_time - start_time).num_milliseconds() as f64;

        match result {
            Ok(_) => {
                burst.stats.successful_requests += 1;
                burst.stats.last_error = None;
            }
            Err(e) => {
                burst.stats.failed_requests += 1;
                burst.stats.last_error = Some(e);
            }
        }

        burst.stats.total_requests += 1;
        let total_time = burst.stats.average_response_time * (burst.stats.total_requests - 1) as f64;
        burst.stats.average_response_time = (total_time + response_time) / burst.stats.total_requests as f64;
        
        burst.stats.last_request_time = Some(end_time);
        burst.stats.current_concurrent_requests -= 1;

        Ok(())
    }

    async fn execute_request(&self, config: &BurstConfig) -> Result<(), String> {
        // Simulate HTTP request
        let client = reqwest::Client::new();
        let mut retries = 0;

        while retries < config.max_retries {
            match client
                .get(&config.target_url)
                .timeout(std::time::Duration::from_millis(config.request_timeout))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(());
                    } else {
                        return Err(format!("Request failed with status: {}", response.status()));
                    }
                }
                Err(e) => {
                    retries += 1;
                    if retries < config.max_retries {
                        tokio::time::sleep(std::time::Duration::from_millis(config.retry_delay)).await;
                        continue;
                    }
                    return Err(format!("Request failed after {} retries: {}", retries, e));
                }
            }
        }

        Err("Maximum retries exceeded".to_string())
    }

    pub async fn get_burst(&self, id: &str) -> Result<BurstMetrics, String> {
        let bursts = self.bursts.lock().await;
        
        bursts
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Burst '{}' not found", id))
    }

    pub async fn get_all_bursts(&self) -> Vec<BurstMetrics> {
        let bursts = self.bursts.lock().await;
        bursts.values().cloned().collect()
    }

    pub async fn get_active_bursts(&self) -> Vec<BurstMetrics> {
        let bursts = self.bursts.lock().await;
        bursts
            .values()
            .filter(|b| b.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_burst_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut bursts = self.bursts.lock().await;
        
        let burst = bursts
            .get_mut(id)
            .ok_or_else(|| format!("Burst '{}' not found", id))?;

        burst.config.active = active;
        info!(
            "Burst '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_burst_config(&self, id: &str, new_config: BurstConfig) -> Result<(), String> {
        let mut bursts = self.bursts.lock().await;
        
        let burst = bursts
            .get_mut(id)
            .ok_or_else(|| format!("Burst '{}' not found", id))?;

        burst.config = new_config;
        info!("Updated burst configuration: {}", id);
        Ok(())
    }
} 