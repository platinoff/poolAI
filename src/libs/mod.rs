pub mod model;
pub mod tuning;
pub mod lib_manager;

use crate::core::error::AppError;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LibConfig {
    pub libs_directory: String,
    pub auto_update: bool,
    pub version_check_interval_hours: u64,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
}

#[derive(Debug, Clone)]
pub struct LibraryInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub size_mb: f32,
    pub dependencies: Vec<String>,
    pub is_loaded: bool,
    pub last_updated: std::time::Instant,
}

#[derive(Debug, Clone)]
pub struct ModelLibrary {
    pub name: String,
    pub version: String,
    pub model_type: ModelType,
    pub file_size_mb: f32,
    pub parameters_count: u64,
    pub supported_features: Vec<String>,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone)]
pub enum ModelType {
    GPT,
    BERT,
    T5,
    LLaMA,
    Custom,
}

#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    None,
    Basic,
    Advanced,
    Maximum,
}

pub struct Libs {
    config: LibConfig,
    libraries: Arc<RwLock<HashMap<String, LibraryInfo>>>,
    model_libraries: Arc<RwLock<HashMap<String, ModelLibrary>>>,
    lib_manager: Arc<lib_manager::LibManager>,
}

impl Libs {
    pub fn new(config: LibConfig) -> Result<Self, AppError> {
        let lib_manager = Arc::new(lib_manager::LibManager::new(config.clone())?);
        
        Ok(Self {
            config,
            libraries: Arc::new(RwLock::new(HashMap::new())),
            model_libraries: Arc::new(RwLock::new(HashMap::new())),
            lib_manager,
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Initialize library manager
        self.lib_manager.initialize().await?;
        
        // Scan available libraries
        self.scan_libraries().await?;
        
        // Load standard libraries
        self.load_standard_libraries().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Unload all libraries
        self.unload_all_libraries().await?;
        
        // Shutdown manager
        self.lib_manager.shutdown().await?;
        
        Ok(())
    }

    pub async fn load_library(&self, name: &str) -> Result<(), AppError> {
        let library_info = {
            let libraries = self.libraries.read().await;
            libraries.get(name).cloned()
        };
        
        if let Some(library) = library_info {
            if !library.is_loaded {
                // Load library through manager
                self.lib_manager.load_library(&library.path).await?;
                
                // Update status
                let mut libraries = self.libraries.write().await;
                if let Some(lib) = libraries.get_mut(name) {
                    lib.is_loaded = true;
                }
            }
        } else {
            return Err(AppError::Model(format!("Library '{}' not found", name)));
        }
        
        Ok(())
    }

    pub async fn unload_library(&self, name: &str) -> Result<(), AppError> {
        let library_info = {
            let libraries = self.libraries.read().await;
            libraries.get(name).cloned()
        };
        
        if let Some(library) = library_info {
            if library.is_loaded {
                // Unload library through manager
                self.lib_manager.unload_library(&library.path).await?;
                
                // Update status
                let mut libraries = self.libraries.write().await;
                if let Some(lib) = libraries.get_mut(name) {
                    lib.is_loaded = false;
                }
            }
        } else {
            return Err(AppError::Model(format!("Library '{}' not found", name)));
        }
        
        Ok(())
    }

    pub async fn get_library_info(&self, name: &str) -> Option<LibraryInfo> {
        let libraries = self.libraries.read().await;
        libraries.get(name).cloned()
    }

    pub async fn list_libraries(&self) -> Vec<LibraryInfo> {
        let libraries = self.libraries.read().await;
        libraries.values().cloned().collect()
    }

    pub async fn add_model_library(&self, model_lib: ModelLibrary) -> Result<(), AppError> {
        let mut model_libraries = self.model_libraries.write().await;
        model_libraries.insert(model_lib.name.clone(), model_lib);
        
        Ok(())
    }

    pub async fn get_model_library(&self, name: &str) -> Option<ModelLibrary> {
        let model_libraries = self.model_libraries.read().await;
        model_libraries.get(name).cloned()
    }

    pub async fn list_model_libraries(&self) -> Vec<ModelLibrary> {
        let model_libraries = self.model_libraries.read().await;
        model_libraries.values().cloned().collect()
    }

    pub async fn optimize_model(&self, model_name: &str, optimization_level: OptimizationLevel) -> Result<(), AppError> {
        // Get model
        let model_library = {
            let model_libraries = self.model_libraries.read().await;
            model_libraries.get(model_name).cloned()
        };
        
        if let Some(model_lib) = model_library {
            // Create optimizer
            let optimizer = tuning::ModelOptimizer::new(model_lib, optimization_level)?;
            
            // Perform optimization
            optimizer.optimize().await?;
        } else {
            return Err(AppError::Model(format!("Model '{}' not found", model_name)));
        }
        
        Ok(())
    }

    pub async fn update_library(&self, name: &str) -> Result<(), AppError> {
        // Check for updates through manager
        let has_update = self.lib_manager.check_for_updates(name).await?;
        
        if has_update {
            // Unload old version
            self.unload_library(name).await?;
            
            // Download and install new version
            self.lib_manager.update_library(name).await?;
            
            // Load new version
            self.load_library(name).await?;
        }
        
        Ok(())
    }

    pub async fn get_dependencies(&self, library_name: &str) -> Result<Vec<String>, AppError> {
        let library_info = {
            let libraries = self.libraries.read().await;
            libraries.get(library_name).cloned()
        };
        
        if let Some(library) = library_info {
            Ok(library.dependencies)
        } else {
            Err(AppError::Model(format!("Library '{}' not found", library_name)))
        }
    }

    async fn scan_libraries(&self) -> Result<(), AppError> {
        // Scan libraries directory
        let libs_dir = std::path::Path::new(&self.config.libs_directory);
        
        if !libs_dir.exists() {
            std::fs::create_dir_all(libs_dir)?;
        }
        
        // Scan for library files
        for entry in std::fs::read_dir(libs_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "so" || ext == "dll") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let library_info = LibraryInfo {
                    name: name.clone(),
                    version: "1.0.0".to_string(),
                    path: path.to_string_lossy().to_string(),
                    size_mb: path.metadata()?.len() as f32 / 1024.0 / 1024.0,
                    dependencies: Vec::new(),
                    is_loaded: false,
                    last_updated: std::time::Instant::now(),
                };
                
                let mut libraries = self.libraries.write().await;
                libraries.insert(name, library_info);
            }
        }
        
        Ok(())
    }

    async fn load_standard_libraries(&self) -> Result<(), AppError> {
        // Load essential libraries
        let standard_libs = vec!["core", "utils", "math"];
        
        for lib_name in standard_libs {
            if let Some(_) = self.get_library_info(lib_name).await {
                self.load_library(lib_name).await?;
            }
        }
        
        Ok(())
    }

    async fn unload_all_libraries(&self) -> Result<(), AppError> {
        let libraries = self.libraries.read().await;
        let loaded_libs: Vec<String> = libraries.iter()
            .filter(|(_, lib)| lib.is_loaded)
            .map(|(name, _)| name.clone())
            .collect();
        
        for lib_name in loaded_libs {
            self.unload_library(&lib_name).await?;
        }
        
        Ok(())
    }
} 