use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use log::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryInfo {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub status: LibraryStatus,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LibraryStatus {
    Installed,
    Downloading,
    Failed(String),
    NotInstalled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryConfig {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub gpu_required: bool,
    pub memory_required: u64,
}

pub struct LibraryManager {
    libraries: Arc<RwLock<HashMap<String, LibraryInfo>>>,
    config: Arc<Mutex<HashMap<String, LibraryConfig>>>,
    download_path: PathBuf,
    status: Arc<Mutex<LibraryStatus>>,
}

impl LibraryManager {
    pub fn new(download_path: PathBuf) -> Self {
        let mut config = HashMap::new();
        
        // LibTorch configuration
        config.insert("libtorch".to_string(), LibraryConfig {
            name: "libtorch".to_string(),
            version: "2.1.0".to_string(),
            path: download_path.join("libtorch"),
            dependencies: vec![
                "CUDA".to_string(),
                "cuDNN".to_string(),
            ],
            gpu_required: true,
            memory_required: 8 * 1024 * 1024 * 1024, // 8GB
        });

        Self {
            libraries: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(Mutex::new(config)),
            download_path,
            status: Arc::new(Mutex::new(LibraryStatus::NotInstalled)),
        }
    }

    pub async fn check_libtorch(&self) -> Result<LibraryStatus, String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        let lib_path = &libtorch_config.path;
        
        if !lib_path.exists() {
            return Ok(LibraryStatus::NotInstalled);
        }

        // Check if it's a valid LibTorch installation
        let torch_lib = lib_path.join("lib");
        let torch_include = lib_path.join("include");
        
        if !torch_lib.exists() || !torch_include.exists() {
            return Ok(LibraryStatus::Failed("Invalid installation".to_string()));
        }

        Ok(LibraryStatus::Installed)
    }

    pub async fn download_libtorch(&self) -> Result<(), String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        // Update status
        let mut libraries = self.libraries.write();
        libraries.insert("libtorch".to_string(), LibraryInfo {
            name: "libtorch".to_string(),
            version: libtorch_config.version.clone(),
            path: libtorch_config.path.clone(),
            status: LibraryStatus::Downloading,
            dependencies: libtorch_config.dependencies.clone(),
            size: 0,
            last_updated: chrono::Utc::now(),
        });

        // Create download directory if it doesn't exist
        fs::create_dir_all(&self.download_path)
            .map_err(|e| format!("Failed to create download directory: {}", e))?;

        let zip_path = self.download_path.join("libtorch.zip");

        // Download LibTorch
        info!("Downloading LibTorch...");
        let response = reqwest::get(&libtorch_config.path.to_string_lossy())
            .await
            .map_err(|e| format!("Failed to download LibTorch: {}", e))?;

        let mut file = File::create(&zip_path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        let content = response.bytes()
            .await
            .map_err(|e| format!("Failed to get response bytes: {}", e))?;

        file.write_all(&content)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        // Extract LibTorch
        info!("Extracting LibTorch...");
        let output = Command::new("unzip")
            .arg(&zip_path)
            .arg("-d")
            .arg(&self.download_path)
            .output()
            .map_err(|e| format!("Failed to extract LibTorch: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to extract LibTorch: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Move to final location
        let extracted_path = self.download_path.join("libtorch");
        if extracted_path.exists() {
            fs::remove_dir_all(&libtorch_config.path)
                .map_err(|e| format!("Failed to remove existing installation: {}", e))?;
        }

        fs::rename(extracted_path, &libtorch_config.path)
            .map_err(|e| format!("Failed to move LibTorch to final location: {}", e))?;

        // Clean up
        fs::remove_file(zip_path)
            .map_err(|e| format!("Failed to remove zip file: {}", e))?;

        // Update status
        let mut libraries = self.libraries.write();
        if let Some(info) = libraries.get_mut("libtorch") {
            info.status = LibraryStatus::Installed;
            info.last_updated = chrono::Utc::now();
        }

        info!("LibTorch installation completed successfully");
        Ok(())
    }

    pub async fn verify_libtorch(&self) -> Result<bool, String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        let lib_path = &libtorch_config.path;
        
        // Check required files
        let required_files = vec![
            "lib/libtorch.so",
            "lib/libc10.so",
            "include/torch/torch.h",
            "include/torch/csrc/api/include/torch/nn.h",
        ];

        for file in required_files {
            let file_path = lib_path.join(file);
            if !file_path.exists() {
                return Ok(false);
            }
        }

        // Check version
        let version_file = lib_path.join("version.txt");
        if version_file.exists() {
            let version = fs::read_to_string(version_file)
                .map_err(|e| format!("Failed to read version file: {}", e))?;
            
            if !version.contains(&libtorch_config.version) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn setup_environment(&self) -> Result<(), String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        // Set environment variables
        std::env::set_var("TORCH_HOME", libtorch_config.path.to_string_lossy());
        std::env::set_var("LD_LIBRARY_PATH", format!("{}:{}", 
            libtorch_config.path.join("lib").to_string_lossy(),
            std::env::var("LD_LIBRARY_PATH").unwrap_or_default()
        ));

        info!("Environment variables set for LibTorch");
        Ok(())
    }

    pub fn get_library_info(&self, name: &str) -> Option<LibraryInfo> {
        self.libraries.read().get(name).cloned()
    }

    pub async fn update_library(&self) -> Result<(), String> {
        // Check current version and update if needed
        let current_status = self.check_libtorch().await?;
        
        match current_status {
            LibraryStatus::NotInstalled => {
                self.download_libtorch().await?;
            }
            LibraryStatus::Failed(_) => {
                warn!("LibTorch installation is corrupted, reinstalling...");
                self.download_libtorch().await?;
            }
            LibraryStatus::Installed => {
                info!("LibTorch is already installed and up to date");
            }
            LibraryStatus::Downloading => {
                return Err("LibTorch is currently being downloaded".to_string());
            }
        }

        Ok(())
    }

    pub async fn load_library(&self) -> Result<(), String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        // Check dependencies
        self.check_dependencies(libtorch_config).await?;

        // Load LibTorch
        self.load_libtorch(libtorch_config).await?;

        // Update status
        let mut status = self.status.lock().await;
        *status = LibraryStatus::Installed;

        info!("LibTorch library loaded successfully");
        Ok(())
    }

    pub async fn unload_library(&self) -> Result<(), String> {
        let config = self.config.lock().await;
        let libtorch_config = config.get("libtorch")
            .ok_or_else(|| "LibTorch configuration not found".to_string())?;

        // Unload LibTorch
        self.unload_libtorch(libtorch_config).await?;

        // Update status
        let mut status = self.status.lock().await;
        *status = LibraryStatus::NotInstalled;

        info!("LibTorch library unloaded successfully");
        Ok(())
    }

    pub async fn get_status(&self) -> LibraryStatus {
        self.status.lock().await.clone()
    }

    pub async fn update_status(&self, new_status: LibraryStatus) {
        let mut status = self.status.lock().await;
        *status = new_status;
    }

    async fn check_dependencies(&self, config: &LibraryConfig) -> Result<(), String> {
        // Check GPU requirements
        if config.gpu_required {
            // Check CUDA availability
            let cuda_check = Command::new("nvidia-smi")
                .output()
                .map_err(|_| "CUDA not available".to_string())?;

            if !cuda_check.status.success() {
                return Err("CUDA is required but not available".to_string());
            }
        }

        // Check memory requirements
        let available_memory = sysinfo::System::new_all().total_memory() * 1024; // Convert to bytes
        if available_memory < config.memory_required {
            return Err(format!(
                "Insufficient memory. Required: {}GB, Available: {}GB",
                config.memory_required / (1024 * 1024 * 1024),
                available_memory / (1024 * 1024 * 1024)
            ));
        }

        Ok(())
    }

    async fn load_libtorch(&self, config: &LibraryConfig) -> Result<(), String> {
        // Set up environment variables
        self.setup_environment().await?;

        // Verify installation
        if !self.verify_libtorch().await? {
            return Err("LibTorch installation verification failed".to_string());
        }

        Ok(())
    }

    async fn unload_libtorch(&self, _config: &LibraryConfig) -> Result<(), String> {
        // Clear environment variables
        std::env::remove_var("TORCH_HOME");
        
        // Note: LD_LIBRARY_PATH modification is more complex and should be handled carefully
        // For now, we'll just log the action
        info!("LibTorch environment variables cleared");
        
        Ok(())
    }
} 