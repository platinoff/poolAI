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
    dependencies::{DependencyResolver, ResolvedDependency},
    download::{download_library, extract_archive},
    manifest::InstalledLibrariesManifest,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;
use flate2::Compression;
use flate2::write::GzEncoder;

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
        // Prefer loading from manifest (production-min persistence).
        let manifest_path = self.manifest_path();
        if let Some(manifest) = InstalledLibrariesManifest::load(&manifest_path).await? {
            let mut libraries = self.libraries.write().await;
            libraries.clear();

            // Keep only entries that still exist on disk.
            for (name, info) in manifest.libraries {
                if info.path.exists() {
                    libraries.insert(name, info);
                }
            }

            return Ok(());
        }

        // Fallback: scan library directory and load metadata (legacy).
        if !self.base_path.exists() {
            return Ok(());
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
                                        installed_at: Some(chrono::Utc::now()),
                                        ..Default::default()
                                    },
                                    artifact_ref: None, // Not stored in RAID for loaded libraries
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

    fn manifest_path(&self) -> PathBuf {
        self.base_path.join("manifest.json")
    }

    async fn persist_manifest(&self) -> Result<(), AppError> {
        let libraries = self.libraries.read().await;
        let manifest = InstalledLibrariesManifest::new(libraries.clone());
        manifest.save_atomic(&self.manifest_path()).await
    }

    /// Install a library
    pub async fn install_library(
        &self,
        name: &str,
        version: &str,
        library_type: LibraryType,
    ) -> Result<LibraryInfo, AppError> {
        // Resolve version "latest" (production-min behavior).
        let resolved_version = if version == "latest" {
            let registry = self.registry.read().await;
            registry.get_latest_version(name).unwrap_or_else(|| "latest".to_string())
        } else {
            version.to_string()
        };

        info!("Installing library: {} v{}", name, resolved_version);
        
        // Check if already installed
        {
            let libraries = self.libraries.read().await;
            if let Some(lib) = libraries.get(name) {
                if lib.version == resolved_version {
                    info!("Library {} v{} already installed", name, resolved_version);
                    return Ok(lib.clone());
                }
            }
        }
        
        // Resolve dependencies + choose versions using registry (production-min).
        let dep_plan: Vec<ResolvedDependency> = {
            let resolver = self.dependency_resolver.read().await;
            let registry = self.registry.read().await;
            resolver.resolve_versions(name, &resolved_version, &registry)?
        };
        
        // Install dependencies first (using Box::pin for recursive async call)
        for dep in &dep_plan {
            Box::pin(self.install_library(&dep.name, &dep.version, library_type)).await?;
        }
        
        // Download and install library
        let (library_path, artifact_ref) = self
            .download_and_install(name, &resolved_version, library_type)
            .await?;

        // Create library info
        let dependencies = dep_plan.iter().map(|d| d.name.clone()).collect();
        let library_info = LibraryInfo {
            name: name.to_string(),
            version: resolved_version.clone(),
            path: library_path,
            dependencies,
            metadata: LibraryMetadata {
                installed_at: Some(chrono::Utc::now()),
                ..Default::default()
            },
            artifact_ref,
        };
        
        // Register library
        {
            let mut libraries = self.libraries.write().await;
            libraries.insert(name.to_string(), library_info.clone());
        }
        
        // Register in version manager
        self.version_manager.write().await
            .register_version(name, &resolved_version, &library_info.path).await?;

        // Persist manifest after successful install.
        self.persist_manifest().await?;
        
        info!("Library {} v{} installed successfully", name, resolved_version);
        Ok(library_info)
    }

    /// Download and install library
    /// Returns (library_path, artifact_ref)
    async fn download_and_install(
        &self,
        name: &str,
        version: &str,
        _library_type: LibraryType,
    ) -> Result<(PathBuf, Option<crate::raid::ArtifactRef>), AppError> {
        info!("Downloading and installing library: {} v{}", name, version);
        
        let library_dir = self.base_path.join(name).join(version);
        let library_parent = self.base_path.join(name);
        tokio::fs::create_dir_all(&library_parent).await
            .map_err(|e| AppError::ConfigError(format!("Failed to create library directory: {}", e)))?;
        
        // Get download URL from registry (for now, use placeholder)
        // TODO: Get actual URL from registry
        let download_url = self.get_download_url(name, version).await?;
        
        if let Some(url) = download_url {
            // Production-min atomic install: download+extract to temp, then rename into place.
            let tmp_root = self.base_path.join(".tmp");
            tokio::fs::create_dir_all(&tmp_root).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create temp directory: {}", e)))?;

            let session_dir = tmp_root.join(format!("{}-{}-{}", name, version, Uuid::new_v4()));
            tokio::fs::create_dir_all(&session_dir).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create temp session directory: {}", e)))?;

            let archive_path = session_dir.join("archive");
            let extract_dir = session_dir.join("extract");

            // Optional expected checksum from registry metadata
            let expected_checksum = {
                let registry = self.registry.read().await;
                registry
                    .get_metadata(name, version)
                    .and_then(|m| m.metadata.checksum.as_deref())
                    .map(|s| s.to_string())
            };

            info!("Downloading from: {}", url);
            download_library(&url, &archive_path, expected_checksum.as_deref()).await?;

            info!("Extracting archive to: {:?}", extract_dir);
            extract_archive(&archive_path, &extract_dir).await?;

            self.verify_installation(&extract_dir, name).await?;

            // Store extracted library as artifact in RAID before finalizing install
            let artifact_ref = {
                // Create a tar.gz archive of the extracted library for storage
                let artifact_archive = session_dir.join("artifact.tar.gz");
                self.create_artifact_archive(&extract_dir, &artifact_archive).await?;
                
                // Read archive bytes
                let artifact_bytes = tokio::fs::read(&artifact_archive).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to read artifact archive: {}", e)))?;
                
                // Store in RAID
                let raid_manager = crate::raid::get_global_manager();
                let artifact_name = format!("{}-{}.tar.gz", name, version);
                raid_manager.put_artifact(&artifact_name, &artifact_bytes).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to store artifact in RAID: {}", e)))?
            };

            // Replace existing target if present (update/reinstall)
            if library_dir.exists() {
                tokio::fs::remove_dir_all(&library_dir).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to remove existing library dir: {}", e)))?;
            }

            tokio::fs::rename(&extract_dir, &library_dir).await
                .map_err(|e| AppError::ConfigError(format!("Failed to finalize install (rename): {}", e)))?;

            // Cleanup temp session (archive + empty session dir)
            if let Err(e) = tokio::fs::remove_dir_all(&session_dir).await {
                warn!("Failed to remove temp session dir: {}", e);
            }

            info!("Library {} v{} installed successfully (atomic) and stored as artifact in RAID", name, version);
            Ok((library_dir, Some(artifact_ref)))
        } else {
            // No download URL, create placeholder structure
            info!("No download URL found, creating placeholder structure");
            tokio::fs::create_dir_all(&library_dir).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create library directory: {}", e)))?;
            let lib_file = library_dir.join("lib").join(format!("lib{}.so", name));
            if let Some(parent) = lib_file.parent() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| AppError::ConfigError(format!("Failed to create lib directory: {}", e)))?;
            }
            Ok((library_dir, None))
        }
    }

    /// Create artifact archive (tar.gz) from extracted library directory
    /// Uses spawn_blocking for CPU-intensive tar creation
    async fn create_artifact_archive(
        &self,
        source_dir: &Path,
        output_path: &Path,
    ) -> Result<(), AppError> {
        use std::fs::File;
        use std::io::BufWriter;

        let source_dir = source_dir.to_path_buf();
        let output_path = output_path.to_path_buf();

        // Use spawn_blocking for CPU-intensive tar creation
        tokio::task::spawn_blocking(move || -> Result<(), AppError> {
            let file = File::create(&output_path)
                .map_err(|e| AppError::ConfigError(format!("Failed to create artifact archive: {}", e)))?;
            let gz = GzEncoder::new(BufWriter::new(file), Compression::default());
            let mut tar = tar::Builder::new(gz);

            // Recursively add all files from source_dir to tar archive
            fn add_dir_to_tar(tar: &mut tar::Builder<GzEncoder<BufWriter<File>>>, dir: &Path, base: &Path) -> Result<(), AppError> {
                let entries = std::fs::read_dir(dir)
                    .map_err(|e| AppError::ConfigError(format!("Failed to read directory {:?}: {}", dir, e)))?;

                for entry in entries {
                    let entry = entry.map_err(|e| AppError::ConfigError(format!("Failed to read directory entry: {}", e)))?;
                    let path = entry.path();
                    let relative_path = path.strip_prefix(base)
                        .map_err(|e| AppError::ConfigError(format!("Failed to get relative path: {}", e)))?;

                    if path.is_file() {
                        let mut file = File::open(&path)
                            .map_err(|e| AppError::ConfigError(format!("Failed to open file {:?}: {}", path, e)))?;
                        tar.append_file(relative_path, &mut file)
                            .map_err(|e| AppError::ConfigError(format!("Failed to add file to tar: {}", e)))?;
                    } else if path.is_dir() {
                        tar.append_dir_all(relative_path, &path)
                            .map_err(|e| AppError::ConfigError(format!("Failed to add directory to tar: {}", e)))?;
                        add_dir_to_tar(tar, &path, base)?;
                    }
                }
                Ok(())
            }

            add_dir_to_tar(&mut tar, &source_dir, &source_dir)?;
            tar.finish()
                .map_err(|e| AppError::ConfigError(format!("Failed to finish tar archive: {}", e)))?;

            Ok(())
        })
        .await
        .map_err(|e| AppError::ConfigError(format!("Task join error: {}", e)))?
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

            // Persist manifest after uninstall.
            self.persist_manifest().await?;
            
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

    /// Get library path, loading from RAID if needed
    /// This is the runtime integration point - if library is stored as artifact in RAID
    /// and local path doesn't exist, it will be extracted from RAID
    pub async fn get_library_path_or_load_from_raid(&self, name: &str) -> Result<Option<PathBuf>, AppError> {
        let lib_info = self.get_library(name).await;
        
        match lib_info {
            Some(lib) => {
                // Check if local path exists
                if lib.path.exists() {
                    return Ok(Some(lib.path));
                }
                
                // If path doesn't exist but we have artifact_ref, load from RAID
                if let Some(ref artifact_ref) = lib.artifact_ref {
                    info!("Library {} not found at local path, loading from RAID artifact {}", name, artifact_ref.id);
                    return self.load_library_from_raid(&lib, artifact_ref).await;
                }
                
                // No artifact_ref, return None
                warn!("Library {} path doesn't exist and no artifact_ref available", name);
                Ok(None)
            }
            None => Ok(None)
        }
    }

    /// Load library from RAID artifact
    async fn load_library_from_raid(
        &self,
        lib_info: &LibraryInfo,
        artifact_ref: &crate::raid::ArtifactRef,
    ) -> Result<Option<PathBuf>, AppError> {
        use crate::libs::download::extract_archive;
        
        // Get artifact from RAID using artifact_ref.path
        let raid_manager = crate::raid::get_global_manager();
        let artifact_bytes = raid_manager.get_artifact(&artifact_ref.path).await
            .map_err(|e| AppError::ConfigError(format!("Failed to get artifact from RAID: {}", e)))?;
        
        // Create temp directory for extraction
        let tmp_root = self.base_path.join(".tmp");
        tokio::fs::create_dir_all(&tmp_root).await
            .map_err(|e| AppError::ConfigError(format!("Failed to create temp directory: {}", e)))?;

        let session_dir = tmp_root.join(format!("{}-{}-{}", lib_info.name, lib_info.version, Uuid::new_v4()));
        tokio::fs::create_dir_all(&session_dir).await
            .map_err(|e| AppError::ConfigError(format!("Failed to create temp session directory: {}", e)))?;

        // Write artifact bytes to temp file
        let artifact_archive = session_dir.join("artifact.tar.gz");
        tokio::fs::write(&artifact_archive, &artifact_bytes).await
            .map_err(|e| AppError::ConfigError(format!("Failed to write artifact archive: {}", e)))?;

        // Extract archive
        let extract_dir = session_dir.join("extract");
        extract_archive(&artifact_archive, &extract_dir).await?;

        // Verify extraction
        self.verify_installation(&extract_dir, &lib_info.name).await?;

        // Move extracted library to final location (atomic)
        if lib_info.path.exists() {
            tokio::fs::remove_dir_all(&lib_info.path).await
                .map_err(|e| AppError::ConfigError(format!("Failed to remove existing library dir: {}", e)))?;
        }

        // Create parent directory if needed
        if let Some(parent) = lib_info.path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| AppError::ConfigError(format!("Failed to create library parent directory: {}", e)))?;
        }

        tokio::fs::rename(&extract_dir, &lib_info.path).await
            .map_err(|e| AppError::ConfigError(format!("Failed to finalize library load from RAID: {}", e)))?;

        // Cleanup temp session
        if let Err(e) = tokio::fs::remove_dir_all(&session_dir).await {
            warn!("Failed to remove temp session dir: {}", e);
        }

        info!("Library {} v{} loaded from RAID artifact successfully", lib_info.name, lib_info.version);
        Ok(Some(lib_info.path.clone()))
    }
}

impl Default for LibraryManager {
    fn default() -> Self {
        Self::new()
    }
}

