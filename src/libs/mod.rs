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
        // Инициализация менеджера библиотек
        self.lib_manager.initialize().await?;
        
        // Сканирование доступных библиотек
        self.scan_libraries().await?;
        
        // Загрузка стандартных библиотек
        self.load_standard_libraries().await?;
        
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        // Выгрузка всех библиотек
        self.unload_all_libraries().await?;
        
        // Выключение менеджера
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
                // Загрузка библиотеки через менеджер
                self.lib_manager.load_library(&library.path).await?;
                
                // Обновление статуса
                let mut libraries = self.libraries.write().await;
                if let Some(lib) = libraries.get_mut(name) {
                    lib.is_loaded = true;
                }
            }
        } else {
            return Err(AppError::LibraryNotFound);
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
                // Выгрузка библиотеки через менеджер
                self.lib_manager.unload_library(&library.path).await?;
                
                // Обновление статуса
                let mut libraries = self.libraries.write().await;
                if let Some(lib) = libraries.get_mut(name) {
                    lib.is_loaded = false;
                }
            }
        } else {
            return Err(AppError::LibraryNotFound);
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
        // Получение модели
        let model_library = {
            let model_libraries = self.model_libraries.read().await;
            model_libraries.get(model_name).cloned()
        };
        
        if let Some(model_lib) = model_library {
            // Создание оптимизатора
            let optimizer = tuning::ModelOptimizer::new(model_lib, optimization_level)?;
            
            // Выполнение оптимизации
            optimizer.optimize().await?;
        } else {
            return Err(AppError::ModelNotFound);
        }
        
        Ok(())
    }

    pub async fn update_library(&self, name: &str) -> Result<(), AppError> {
        // Проверка обновлений через менеджер
        let has_update = self.lib_manager.check_for_updates(name).await?;
        
        if has_update {
            // Выгрузка старой версии
            self.unload_library(name).await?;
            
            // Обновление библиотеки
            self.lib_manager.update_library(name).await?;
            
            // Загрузка новой версии
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
            Err(AppError::LibraryNotFound)
        }
    }

    async fn scan_libraries(&self) -> Result<(), AppError> {
        // Сканирование директории библиотек
        let libs_path = std::path::Path::new(&self.config.libs_directory);
        
        if !libs_path.exists() {
            std::fs::create_dir_all(libs_path)?;
        }
        
        // Сканирование файлов библиотек
        for entry in std::fs::read_dir(libs_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "so" || ext == "dll") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                
                let library_info = LibraryInfo {
                    name: name.clone(),
                    version: "1.0.0".to_string(), // Будет извлечено из метаданных
                    path: path.to_string_lossy().to_string(),
                    size_mb: path.metadata()?.len() as f32 / (1024.0 * 1024.0),
                    dependencies: Vec::new(), // Будет заполнено из метаданных
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
        // Загрузка стандартных библиотек
        let standard_libs = vec![
            "cuda",
            "cudnn",
            "tensorrt",
            "onnxruntime",
        ];
        
        for lib_name in standard_libs {
            if let Some(library) = self.get_library_info(lib_name).await {
                if !library.is_loaded {
                    self.load_library(lib_name).await?;
                }
            }
        }
        
        Ok(())
    }

    async fn unload_all_libraries(&self) -> Result<(), AppError> {
        let libraries = self.libraries.read().await;
        let loaded_libraries: Vec<String> = libraries
            .iter()
            .filter(|(_, lib)| lib.is_loaded)
            .map(|(name, _)| name.clone())
            .collect();
        
        for lib_name in loaded_libraries {
            self.unload_library(&lib_name).await?;
        }
        
        Ok(())
    }
} 