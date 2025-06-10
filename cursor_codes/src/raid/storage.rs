use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::fs;
use std::io;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use tokio::fs as tokio_fs;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::storage::StorageSystem;

const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(10);
const NODE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Error, Debug)]
pub enum Error {
    #[error("RAID initialization error: {0}")]
    RaidInitError(String),
    #[error("Disk error: {0}")]
    DiskError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidConfig {
    pub raid_level: u8,
    pub min_disks: usize,
    pub stripe_size: usize,
    pub redundancy: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub path: String,
    pub size: u64,
    pub status: DiskStatus,
    pub last_seen: Instant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiskStatus {
    Active,
    Degraded,
    Failed,
    Rebuilding,
}

pub struct StorageManager {
    config: RaidConfig,
    disks: Arc<RwLock<HashMap<String, DiskInfo>>>,
    model_pool: Arc<RwLock<HashMap<String, String>>>, // model_id -> raid_path
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            config: RaidConfig {
                raid_level: 0,
                min_disks: 2,
                stripe_size: 1024 * 1024, // 1MB
                redundancy: 1,
            },
            disks: Arc::new(RwLock::new(HashMap::new())),
            model_pool: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        // Create data directory if it doesn't exist
        fs::create_dir_all("data")?;
        Ok(())
    }

    pub async fn add_disk(&self, disk_id: String, path: String, size: u64) -> Result<(), Error> {
        let mut disks = self.disks.write().await;
        
        disks.insert(disk_id.clone(), DiskInfo {
            path,
            size,
            status: DiskStatus::Active,
            last_seen: Instant::now(),
        });

        Ok(())
    }

    pub async fn load_model(&self, model_id: String, model_path: String) -> Result<(), Error> {
        let mut model_pool = self.model_pool.write().await;
        
        // Calculate required space based on model size
        let model_size = fs::metadata(&model_path)?.len();
        let required_disks = (model_size as f64 / self.config.stripe_size as f64).ceil() as usize;
        
        // Check if we have enough disks
        let disks = self.disks.read().await;
        if disks.len() < required_disks {
            return Err(Error::RaidInitError(
                format!("Not enough disks for model. Required: {}, Available: {}", 
                        required_disks, disks.len())
            ));
        }

        // Distribute model across RAID
        let raid_path = format!("data/raid/models/{}", model_id);
        fs::create_dir_all(&raid_path)?;
        
        // Copy model to RAID with striping
        match self.config.raid_level {
            0 => self.strip_model(&model_path, &raid_path, model_size).await?,
            1 => self.mirror_model(&model_path, &raid_path, model_size).await?,
            _ => return Err(Error::RaidInitError(
                format!("Unsupported RAID level: {}", self.config.raid_level)
            )),
        }

        model_pool.insert(model_id, raid_path);
        Ok(())
    }

    async fn strip_model(&self, source: &str, target: &str, size: u64) -> Result<(), Error> {
        let stripe_size = self.config.stripe_size as u64;
        let mut offset = 0;
        let mut disk_index = 0;
        
        // Calculate checksum of source file
        let source_checksum = self.calculate_checksum(source).await?;
        
        while offset < size {
            let current_stripe = std::cmp::min(stripe_size, size - offset);
            
            // Get next available disk
            let disks = self.disks.read().await;
            let disk_ids: Vec<_> = disks.keys().collect();
            if disk_ids.is_empty() {
                return Err(Error::DiskError("No disks available".to_string()));
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
                return Err(Error::DiskError(
                    format!("Checksum mismatch for stripe at offset {}", offset)
                ));
            }
            
            offset += current_stripe;
            disk_index += 1;
        }
        
        Ok(())
    }

    async fn mirror_model(&self, source: &str, target: &str, size: u64) -> Result<(), Error> {
        // Calculate source checksum
        let source_checksum = self.calculate_checksum(source).await?;
        
        // Get all active disks
        let disks = self.disks.read().await;
        let active_disks: Vec<_> = disks.iter()
            .filter(|(_, disk)| disk.status == DiskStatus::Active)
            .collect();
            
        if active_disks.is_empty() {
            return Err(Error::DiskError("No active disks available".to_string()));
        }

        // Mirror to each active disk
        for (disk_id, disk) in active_disks {
            let mirror_path = format!("{}/mirror_{}", disk.path, target);
            fs::copy(source, &mirror_path)?;
            
            // Verify mirror checksum
            let mirror_checksum = self.calculate_checksum(&mirror_path).await?;
            if mirror_checksum != source_checksum {
                return Err(Error::DiskError(
                    format!("Checksum mismatch for mirror on disk {}", disk_id)
                ));
            }
        }
        
        Ok(())
    }

    async fn calculate_checksum(&self, path: &str) -> Result<String, Error> {
        let mut file = tokio_fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Cleanup and unmount all disks
        let mut disks = self.disks.write().await;
        disks.clear();
        Ok(())
    }
} 