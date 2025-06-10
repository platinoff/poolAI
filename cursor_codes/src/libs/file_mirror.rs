use actix_web::{web, HttpResponse, post, get};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use sha2::{Sha256, Digest};
use hex;
use log::{info, error};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMirrorResponse {
    pub status: String,
    pub checksum: String,
    pub file_path: String,
}

pub struct FileMirrorConfig {
    pub mirror_dir: PathBuf,
}

impl FileMirrorConfig {
    pub fn new(mirror_dir: PathBuf) -> Self {
        Self { mirror_dir }
    }
}

#[post("/mirror")]
async fn receive_file(
    payload: web::Payload,
    config: web::Data<FileMirrorConfig>,
) -> HttpResponse {
    let mut file_data = Vec::new();
    let mut stream = payload.into_inner();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => file_data.extend_from_slice(&bytes),
            Err(e) => {
                error!("Error receiving file chunk: {}", e);
                return HttpResponse::BadRequest().body("Error receiving file");
            }
        }
    }

    // Generate checksum
    let mut hasher = Sha256::new();
    hasher.update(&file_data);
    let checksum = hex::encode(hasher.finalize());

    // Generate unique filename
    let filename = format!("{}.bin", checksum);
    let file_path = config.mirror_dir.join(&filename);

    // Save file
    match fs::File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&file_data) {
                error!("Error saving file: {}", e);
                return HttpResponse::InternalServerError().body("Error saving file");
            }
        }
        Err(e) => {
            error!("Error creating file: {}", e);
            return HttpResponse::InternalServerError().body("Error creating file");
        }
    }

    info!("Received and saved file: {}", file_path.display());

    HttpResponse::Ok().json(FileMirrorResponse {
        status: "success".to_string(),
        checksum,
        file_path: file_path.to_string_lossy().into_owned(),
    })
}

#[get("/mirror/{checksum}")]
async fn get_file(
    checksum: web::Path<String>,
    config: web::Data<FileMirrorConfig>,
) -> HttpResponse {
    let filename = format!("{}.bin", checksum);
    let file_path = config.mirror_dir.join(&filename);

    match fs::read(&file_path) {
        Ok(data) => {
            info!("Serving file: {}", file_path.display());
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(data)
        }
        Err(e) => {
            error!("Error reading file: {}", e);
            HttpResponse::NotFound().body("File not found")
        }
    }
}

pub fn init_file_mirror(cfg: &mut web::ServiceConfig, mirror_dir: PathBuf) {
    let config = FileMirrorConfig::new(mirror_dir);
    cfg.app_data(web::Data::new(config))
        .service(receive_file)
        .service(get_file);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub sync_interval: u64,
    pub last_sync: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total_files: u64,
    pub total_size: u64,
    pub synced_files: u64,
    pub failed_files: u64,
    pub last_sync_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorMetrics {
    pub config: MirrorConfig,
    pub stats: FileStats,
}

pub struct FileMirror {
    mirrors: Arc<Mutex<HashMap<String, MirrorMetrics>>>,
}

impl FileMirror {
    pub fn new() -> Self {
        Self {
            mirrors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_mirror(&self, name: String, config: MirrorConfig) -> Result<(), String> {
        let mut mirrors = self.mirrors.lock().await;
        
        if mirrors.contains_key(&name) {
            return Err(format!("Mirror '{}' already exists", name));
        }

        if !config.source_path.exists() {
            return Err(format!("Source path does not exist: {:?}", config.source_path));
        }

        if !config.destination_path.exists() {
            fs::create_dir_all(&config.destination_path)
                .map_err(|e| format!("Failed to create destination directory: {}", e))?;
        }

        let metrics = MirrorMetrics {
            config,
            stats: FileStats {
                total_files: 0,
                total_size: 0,
                synced_files: 0,
                failed_files: 0,
                last_sync_time: None,
                last_error: None,
            },
        };

        mirrors.insert(name.clone(), metrics);
        info!("Added new mirror: {}", name);
        Ok(())
    }

    pub async fn remove_mirror(&self, name: &str) -> Result<(), String> {
        let mut mirrors = self.mirrors.lock().await;
        
        if !mirrors.contains_key(name) {
            return Err(format!("Mirror '{}' not found", name));
        }

        mirrors.remove(name);
        info!("Removed mirror: {}", name);
        Ok(())
    }

    pub async fn sync_mirror(&self, name: &str) -> Result<(), String> {
        let mut mirrors = self.mirrors.lock().await;
        
        let mirror = mirrors
            .get_mut(name)
            .ok_or_else(|| format!("Mirror '{}' not found", name))?;

        if !mirror.config.active {
            return Err("Mirror is not active".to_string());
        }

        let now = Utc::now();
        if let Some(last_sync) = mirror.config.last_sync {
            let time_diff = now.signed_duration_since(last_sync).num_seconds() as u64;
            if time_diff < mirror.config.sync_interval {
                return Err("Too soon for next sync".to_string());
            }
        }

        match self.sync_directory(
            &mirror.config.source_path,
            &mirror.config.destination_path,
        ) {
            Ok(stats) => {
                mirror.stats = stats;
                mirror.config.last_sync = Some(now);
                info!("Successfully synced mirror: {}", name);
                Ok(())
            }
            Err(e) => {
                mirror.stats.last_error = Some(e.clone());
                error!("Failed to sync mirror {}: {}", name, e);
                Err(e)
            }
        }
    }

    fn sync_directory(
        &self,
        source: &Path,
        destination: &Path,
    ) -> Result<FileStats, String> {
        let mut stats = FileStats {
            total_files: 0,
            total_size: 0,
            synced_files: 0,
            failed_files: 0,
            last_sync_time: Some(Utc::now()),
            last_error: None,
        };

        for entry in fs::read_dir(source)
            .map_err(|e| format!("Failed to read source directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            let dest_path = destination.join(path.strip_prefix(source).unwrap());

            if path.is_dir() {
                if !dest_path.exists() {
                    fs::create_dir_all(&dest_path)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }
                let sub_stats = self.sync_directory(&path, &dest_path)?;
                stats.total_files += sub_stats.total_files;
                stats.total_size += sub_stats.total_size;
                stats.synced_files += sub_stats.synced_files;
                stats.failed_files += sub_stats.failed_files;
            } else {
                stats.total_files += 1;
                stats.total_size += path
                    .metadata()
                    .map_err(|e| format!("Failed to get file metadata: {}", e))?
                    .len();

                if self.sync_file(&path, &dest_path).is_ok() {
                    stats.synced_files += 1;
                } else {
                    stats.failed_files += 1;
                }
            }
        }

        Ok(stats)
    }

    fn sync_file(&self, source: &Path, destination: &Path) -> Result<(), String> {
        if !destination.exists()
            || source
                .metadata()
                .map_err(|e| format!("Failed to get source metadata: {}", e))?
                .modified()
                .map_err(|e| format!("Failed to get source modification time: {}", e))?
                > destination
                    .metadata()
                    .map_err(|e| format!("Failed to get destination metadata: {}", e))?
                    .modified()
                    .map_err(|e| format!("Failed to get destination modification time: {}", e))?
        {
            fs::copy(source, destination)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
        Ok(())
    }

    pub async fn get_mirror(&self, name: &str) -> Result<MirrorMetrics, String> {
        let mirrors = self.mirrors.lock().await;
        
        mirrors
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Mirror '{}' not found", name))
    }

    pub async fn get_all_mirrors(&self) -> Vec<MirrorMetrics> {
        let mirrors = self.mirrors.lock().await;
        mirrors.values().cloned().collect()
    }

    pub async fn get_active_mirrors(&self) -> Vec<MirrorMetrics> {
        let mirrors = self.mirrors.lock().await;
        mirrors
            .values()
            .filter(|m| m.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_mirror_active(&self, name: &str, active: bool) -> Result<(), String> {
        let mut mirrors = self.mirrors.lock().await;
        
        let mirror = mirrors
            .get_mut(name)
            .ok_or_else(|| format!("Mirror '{}' not found", name))?;

        mirror.config.active = active;
        info!(
            "Mirror '{}' {}",
            name,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }
} 