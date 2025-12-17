//! Library Manager - Main interface for library management
//!
//! Provides:
//! - Library loading and installation
//! - Lifecycle management
//! - Thread-safe operations

use crate::core::error::AppError;
use crate::libs::{
    LibraryInfo, LibraryMetadata, LibraryStatus, LibraryType,
    registry::LibraryRegistry,
    versioning::VersionManager,
    dependencies::DependencyResolver,
    download::{download_library, extract_archive},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use chrono::Utc;

/// Library Manager - Main interface for managing libraries
pub struct LibraryManager {
    registry: Arc<RwLock<LibraryRegistry>>,
    version_manager: Arc<RwLock<VersionManager>>,
    dependency_resolver: Arc<RwLock<DependencyResolver>>,
    libraries: Arc<RwLock<HashMap<String, LibraryInfo>>>,
    base_path: PathBuf,
}

impl LibraryManager {
    /// Create new library manager
    pub fn new() -> Self {
        let base_path = Self::get_default_library_path();
        
        Self {
            registry: Arc::new(RwLock::new(LibraryRegistry::new())),
            version_manager: Arc::new(RwLock::new(VersionManager::new())),
            dependency_resolver: Arc::new(RwLock::new(DependencyResolver::new())),
            libraries: Arc::new(RwLock::new(HashMap::new())),
            base_path,
        }
    }

    /// Initialize library manager
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing Library Manager");
        
        // Ensure base directory exists
        if !self.base_path.exists() {
            tokio::fs::create_dir_all(&self.base_path).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create library directory: {}", e)))?;
        }
        
        // Initialize registry
        self.registry.write().await.initialize().await?;
        
        // Load existing libraries
        self.load_existing_libraries().await?;
        
        info!("Library Manager initialized successfully");
        Ok(())
    }

    /// Get default library path
    fn get_default_library_path() -> PathBuf {
        // Use platform-specific paths
        #[cfg(target_os = "windows")]
        {
            PathBuf::from("C:\\poolai\\libs")
        }
        
        #[cfg(target_os = "linux")]
        {
            PathBuf::from("/var/lib/poolai/libs")
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            PathBuf::from("./libs")
        }
    }

    /// Load existing libraries from disk
    async fn load_existing_libraries(&self) -> Result<(), AppError> {
        // Scan library directory and load metadata
        if !self.base_path.exists() {
            return Ok(()); // Directory doesn't exist yet, nothing to load
        }
        
        let mut entries = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| AppError::ConfigError(format!("Failed to read library directory: {}", e)))?;
        
        let mut libraries = self.libraries.write().await;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| AppError::ConfigError(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_dir() {
                // Try to load library metadata
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Look for version subdirectories
                    let mut version_dirs = tokio::fs::read_dir(&path).await
                        .map_err(|e| AppError::ConfigError(format!("Failed to read version directory: {}", e)))?;
                    
                    while let Some(version_entry) = version_dirs.next_entry().await
                        .map_err(|e| AppError::ConfigError(format!("Failed to read version entry: {}", e)))?
                    {
                        let version_path = version_entry.path();
                        if version_path.is_dir() {
                            if let Some(version) = version_path.file_name().and_then(|n| n.to_str()) {
                                // Create library info from directory structure
                                let library_info = LibraryInfo {
                                    name: name.to_string(),
                                    version: version.to_string(),
                                    path: version_path.clone(),
                                    dependencies: Vec::new(),
                                    metadata: LibraryMetadata {
                                        installed_at: version_entry.metadata().await
                                            .ok()
                                            .and_then(|m| m.modified().ok())
                                            .map(|t| {
                                                // Convert SystemTime to chrono::DateTime using chrono 0.4 API
                                                let duration = t.duration_since(std::time::UNIX_EPOCH)
                                                    .unwrap_or_default();
                                                let secs = duration.as_secs() as i64;
                                                let nsecs = duration.subsec_nanos();
                                                chrono::DateTime::<Utc>::from_timestamp(secs, nsecs)
                                                    .unwrap_or_else(|| Utc::now())
                                            }),
                                        ..Default::default()
                                    },
                                };
                                
                                libraries.insert(name.to_string(), library_info);
                                break; // Use first version found
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Install a library
    pub async fn install_library(
        &self,
        name: &str,
        version: &str,
        library_type: LibraryType,
    ) -> Result<LibraryInfo, AppError> {
        info!("Installing library: {} v{}", name, version);
        
        // Check if already installed
        {
            let libraries = self.libraries.read().await;
            if let Some(lib) = libraries.get(name) {
                if lib.version == version {
                    info!("Library {} v{} already installed", name, version);
                    return Ok(lib.clone());
                }
            }
        }
        
        // Resolve dependencies
        let dependencies = self.dependency_resolver.read().await
            .resolve(name, version).await?;
        
        // Install dependencies first (using Box::pin for recursive async call)
        for dep in &dependencies {
            Box::pin(self.install_library(dep, "latest", library_type)).await?;
        }
        
        // Download and install library
        let library_path = self.download_and_install(name, version, library_type).await?;
        
        // Create library info
        let library_info = LibraryInfo {
            name: name.to_string(),
            version: version.to_string(),
            path: library_path,
            dependencies,
            metadata: LibraryMetadata {
                installed_at: Some(chrono::Utc::now()),
                ..Default::default()
            },
        };
        
        // Register library
        {
            let mut libraries = self.libraries.write().await;
            libraries.insert(name.to_string(), library_info.clone());
        }
        
        // Register in version manager
        self.version_manager.write().await
            .register_version(name, version, &library_info.path).await?;
        
        info!("Library {} v{} installed successfully", name, version);
        Ok(library_info)
    }

    /// Download and install library
    async fn download_and_install(
        &self,
        name: &str,
        version: &str,
        _library_type: LibraryType,
    ) -> Result<PathBuf, AppError> {
        info!("Downloading and installing library: {} v{}", name, version);
        
        // Create library directory
        let library_dir = self.base_path.join(name).join(version);
        tokio::fs::create_dir_all(&library_dir).await
            .map_err(|e| AppError::ConfigError(format!("Failed to create library directory: {}", e)))?;
        
        // Get download URL from registry (for now, use placeholder)
        // TODO: Get actual URL from registry
        let download_url = self.get_download_url(name, version).await?;
        
        if let Some(url) = download_url {
            // Create temporary download path
            let temp_dir = self.base_path.join(".tmp");
            tokio::fs::create_dir_all(&temp_dir).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create temp directory: {}", e)))?;
            
            let archive_name = format!("{}-{}.tar.gz", name, version);
            let archive_path = temp_dir.join(&archive_name);
            
            // Download library
            info!("Downloading from: {}", url);
            download_library(&url, &archive_path, None).await?;
            
            // Extract archive
            info!("Extracting archive to: {:?}", library_dir);
            extract_archive(&archive_path, &library_dir).await?;
            
            // Clean up temporary file
            if let Err(e) = tokio::fs::remove_file(&archive_path).await {
                warn!("Failed to remove temporary file: {}", e);
            }
            
            // Verify installation
            self.verify_installation(&library_dir, name).await?;
            
            info!("Library {} v{} installed successfully", name, version);
        } else {
            // No download URL, create placeholder structure
            info!("No download URL found, creating placeholder structure");
            let lib_file = library_dir.join("lib").join(format!("lib{}.so", name));
            if let Some(parent) = lib_file.parent() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to create lib directory: {}", e)))?;
            }
        }
        
        Ok(library_dir)
    }
    
    /// Get download URL for library
    async fn get_download_url(&self, name: &str, version: &str) -> Result<Option<String>, AppError> {
        let registry = self.registry.read().await;
        Ok(registry.get_download_url(name, version))
    }
    
    /// Verify library installation
    async fn verify_installation(&self, library_dir: &PathBuf, name: &str) -> Result<(), AppError> {
        // Check if library directory exists and is not empty
        if !library_dir.exists() {
            return Err(AppError::ConfigError(format!(
                "Library directory does not exist: {:?}",
                library_dir
            )));
        }
        
        // Check for common library files
        let lib_patterns = vec![
            format!("lib{}.so", name),
            format!("lib{}.dylib", name),
            format!("{}.dll", name),
            format!("lib{}.a", name),
        ];
        
        let mut found = false;
        let mut entries = tokio::fs::read_dir(library_dir).await
            .map_err(|e| AppError::ConfigError(format!("Failed to read library directory: {}", e)))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| AppError::ConfigError(format!("Failed to read directory entry: {}", e)))?
        {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            for pattern in &lib_patterns {
                if file_name_str.contains(pattern) {
                    found = true;
                    break;
                }
            }
            
            if found {
                break;
            }
        }
        
        if !found {
            warn!("No library files found in {:?}, but installation completed", library_dir);
        }
        
        Ok(())
    }

    /// Uninstall a library
    pub async fn uninstall_library(&self, name: &str) -> Result<(), AppError> {
        info!("Uninstalling library: {}", name);
        
        let mut libraries = self.libraries.write().await;
        
        if let Some(library_info) = libraries.remove(name) {
            // Remove files
            if library_info.path.exists() {
                tokio::fs::remove_dir_all(&library_info.path).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to remove library: {}", e)))?;
            }
            
            // Unregister from version manager
            self.version_manager.write().await
                .unregister_version(name).await?;
            
            info!("Library {} uninstalled successfully", name);
            Ok(())
        } else {
            Err(AppError::ConfigError(format!("Library {} not found", name)))
        }
    }

    /// Get library information
    pub async fn get_library(&self, name: &str) -> Option<LibraryInfo> {
        let libraries = self.libraries.read().await;
        libraries.get(name).cloned()
    }

    /// List all installed libraries
    pub async fn list_libraries(&self) -> Vec<LibraryInfo> {
        let libraries = self.libraries.read().await;
        libraries.values().cloned().collect()
    }

    /// Check library status
    pub async fn get_library_status(&self, name: &str) -> LibraryStatus {
        let libraries = self.libraries.read().await;
        if libraries.contains_key(name) {
            LibraryStatus::Installed
        } else {
            LibraryStatus::NotInstalled
        }
    }

    /// Update library to latest version
    pub async fn update_library(&self, name: &str) -> Result<LibraryInfo, AppError> {
        info!("Updating library: {}", name);
        
        // Get current version
        let current_lib = self.get_library(name).await
            .ok_or_else(|| AppError::ConfigError(format!("Library {} not found", name)))?;
        
        // Get latest version from registry
        let latest_version = {
            let registry = self.registry.read().await;
            registry.get_latest_version(name)
                .ok_or_else(|| AppError::ConfigError(format!("No versions available for {}", name)))?
        };
        
        if current_lib.version == latest_version {
            info!("Library {} already at latest version", name);
            return Ok(current_lib);
        }
        
        // Uninstall old version
        self.uninstall_library(name).await?;
        
        // Install new version
        let library_type = LibraryType::ModelLibrary; // TODO: Determine from registry
        self.install_library(name, &latest_version, library_type).await
    }

    /// Get library path
    pub async fn get_library_path(&self, name: &str) -> Option<PathBuf> {
        self.get_library(name).await.map(|lib| lib.path)
    }
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self::new()
    }
}

