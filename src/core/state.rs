use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::core::error::AppError;
use crate::core::config::PoolAIConfig;
use crate::core::model_interface::{ModelState, ModelStatus};
use tracing::info;

/// Состояние воркера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    /// Уникальный ID воркера
    pub id: String,
    /// Адрес воркера
    pub address: String,
    /// Вычислительная мощность
    pub mining_power: f64,
    /// Статус воркера
    pub status: WorkerStatus,
    /// Время последней активности
    pub last_seen: DateTime<Utc>,
    /// Метрики производительности
    pub metrics: WorkerMetrics,
    /// Обрабатываемые модели
    pub active_models: Vec<String>,
}

/// Метрики воркера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    /// Загрузка CPU (%)
    pub cpu_utilization: f32,
    /// Использование памяти (MB)
    pub memory_usage_mb: f32,
    /// Утилизация GPU (%)
    pub gpu_utilization: f32,
    /// Температура GPU (°C)
    pub gpu_temperature: f32,
    /// Количество обработанных запросов
    pub requests_processed: u64,
    /// Среднее время обработки (мс)
    pub avg_processing_time_ms: f32,
    /// Количество ошибок
    pub error_count: u64,
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_usage_mb: 0.0,
            gpu_utilization: 0.0,
            gpu_temperature: 0.0,
            requests_processed: 0,
            avg_processing_time_ms: 0.0,
            error_count: 0,
        }
    }
}

/// Статус воркера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Active,
    Inactive,
    Error,
    Maintenance,
    Shutdown,
}

/// Статус узла для распределенных систем
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Degraded,
    Failed,
    Maintenance,
}

/// Состояние системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// Статус системы
    pub status: SystemStatus,
    /// Время запуска
    pub start_time: DateTime<Utc>,
    /// Время последней активности
    pub last_activity: DateTime<Utc>,
    /// Количество активных воркеров
    pub active_workers: usize,
    /// Общее количество воркеров
    pub total_workers: usize,
    /// Количество активных моделей
    pub active_models: usize,
    /// Метрики системы
    pub system_metrics: SystemMetrics,
}

/// Статус системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Initializing,
    Running,
    Degraded,
    Error,
    Shutdown,
    Maintenance,
}

/// Метрики системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Общая загрузка CPU (%)
    pub total_cpu_utilization: f32,
    /// Общее использование памяти (MB)
    pub total_memory_usage_mb: f32,
    /// Общая утилизация GPU (%)
    pub total_gpu_utilization: f32,
    /// Общее количество запросов
    pub total_requests: u64,
    /// Средняя задержка (мс)
    pub avg_latency_ms: f32,
    /// Пропускная способность (запросов/сек)
    pub throughput_rps: f32,
    /// Количество ошибок
    pub error_count: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            total_cpu_utilization: 0.0,
            total_memory_usage_mb: 0.0,
            total_gpu_utilization: 0.0,
            total_requests: 0,
            avg_latency_ms: 0.0,
            throughput_rps: 0.0,
            error_count: 0,
        }
    }
}

/// Основное состояние приложения
pub struct AppState {
    /// Воркеры
    pub workers: Arc<RwLock<HashMap<String, Worker>>>,
    /// Конфигурация
    pub config: Arc<RwLock<PoolAIConfig>>,
    /// Состояние системы
    pub system_state: Arc<RwLock<SystemState>>,
    /// Состояния моделей
    pub model_states: Arc<RwLock<HashMap<String, ModelState>>>,
    /// Флаг инициализации
    pub is_initialized: Arc<RwLock<bool>>,
    /// Мьютекс для синхронизации
    pub state_mutex: Arc<Mutex<()>>,
}

impl AppState {
    /// Создание нового состояния приложения
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(PoolAIConfig::default())),
            system_state: Arc::new(RwLock::new(SystemState {
                status: SystemStatus::Initializing,
                start_time: Utc::now(),
                last_activity: Utc::now(),
                active_workers: 0,
                total_workers: 0,
                active_models: 0,
                system_metrics: SystemMetrics::default(),
            })),
            model_states: Arc::new(RwLock::new(HashMap::new())),
            is_initialized: Arc::new(RwLock::new(false)),
            state_mutex: Arc::new(Mutex::new(())),
        }
    }

    /// Инициализация состояния
    pub async fn initialize(&self) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut initialized = self.is_initialized.write();
        
        if *initialized {
            return Ok(());
        }

        info!("Initializing application state...");
        
        // Загрузка конфигурации
        let config = PoolAIConfig::default();
        *self.config.write() = config;
        
        // Обновление состояния системы
        let mut system_state = self.system_state.write();
        system_state.status = SystemStatus::Running;
        system_state.last_activity = Utc::now();
        
        *initialized = true;
        info!("Application state initialized successfully");
        Ok(())
    }

    /// Очистка состояния
    pub async fn cleanup(&self) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        info!("Cleaning up application state...");
        
        // Очистка воркеров
        self.workers.write().clear();
        
        // Очистка состояний моделей
        self.model_states.write().clear();
        
        // Обновление состояния системы
        let mut system_state = self.system_state.write();
        system_state.status = SystemStatus::Shutdown;
        system_state.active_workers = 0;
        system_state.total_workers = 0;
        system_state.active_models = 0;
        
        // Сброс флага инициализации
        *self.is_initialized.write() = false;
        
        info!("Application state cleanup complete");
        Ok(())
    }

    /// Добавление воркера
    pub fn add_worker(&self, worker: Worker) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();
        
        workers.insert(worker.id.clone(), worker.clone());
        system_state.total_workers += 1;
        
        if matches!(worker.status, WorkerStatus::Active) {
            system_state.active_workers += 1;
        }
        
        system_state.last_activity = Utc::now();
        
        info!("Added worker: {} (status: {:?})", worker.id, worker.status);
        Ok(())
    }

    /// Удаление воркера
    pub fn remove_worker(&self, worker_id: &str) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();
        
        if let Some(worker) = workers.remove(worker_id) {
            system_state.total_workers -= 1;
            
            if matches!(worker.status, WorkerStatus::Active) {
                system_state.active_workers = system_state.active_workers.saturating_sub(1);
            }
            
            system_state.last_activity = Utc::now();
            
            info!("Removed worker: {}", worker_id);
            Ok(())
        } else {
            Err(AppError::ResourceError(format!("Worker '{}' not found", worker_id)))
        }
    }

    /// Получение воркера
    pub fn get_worker(&self, worker_id: &str) -> Option<Worker> {
        self.workers.read().get(worker_id).cloned()
    }

    /// Получение всех воркеров
    pub fn get_all_workers(&self) -> Vec<Worker> {
        self.workers.read().values().cloned().collect()
    }

    /// Обновление статуса воркера
    pub fn update_worker_status(&self, worker_id: &str, status: WorkerStatus) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut workers = self.workers.write();
        let mut system_state = self.system_state.write();
        
        if let Some(worker) = workers.get_mut(worker_id) {
            let was_active = matches!(worker.status, WorkerStatus::Active);
            let is_active = matches!(status, WorkerStatus::Active);
            
            worker.status = status.clone();
            worker.last_seen = Utc::now();
            
            // Обновление счетчиков активных воркеров
            if was_active && !is_active {
                system_state.active_workers = system_state.active_workers.saturating_sub(1);
            } else if !was_active && is_active {
                system_state.active_workers += 1;
            }
            
            system_state.last_activity = Utc::now();
            
            info!("Updated worker {} status to {:?}", worker_id, status);
            Ok(())
        } else {
            Err(AppError::ResourceError(format!("Worker '{}' not found", worker_id)))
        }
    }

    /// Обновление метрик воркера
    pub fn update_worker_metrics(&self, worker_id: &str, metrics: WorkerMetrics) -> Result<(), AppError> {
        let mut workers = self.workers.write();
        
        if let Some(worker) = workers.get_mut(worker_id) {
            worker.metrics = metrics;
            worker.last_seen = Utc::now();
            Ok(())
        } else {
            Err(AppError::ResourceError(format!("Worker '{}' not found", worker_id)))
        }
    }

    /// Добавление состояния модели
    pub fn add_model_state(&self, model_name: String, state: ModelState) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut model_states = self.model_states.write();
        let mut system_state = self.system_state.write();
        
        model_states.insert(model_name.clone(), state.clone());
        
        if matches!(state.status, ModelStatus::Ready) {
            system_state.active_models += 1;
        }
        
        system_state.last_activity = Utc::now();
        
        info!("Added model state: {} (status: {:?})", model_name, state.status);
        Ok(())
    }

    /// Обновление состояния модели
    pub fn update_model_state(&self, model_name: &str, state: ModelState) -> Result<(), AppError> {
        let _lock = self.state_mutex.lock();
        let mut model_states = self.model_states.write();
        let mut system_state = self.system_state.write();
        
        if let Some(existing_state) = model_states.get(model_name) {
            let was_ready = matches!(existing_state.status, ModelStatus::Ready);
            let is_ready = matches!(state.status, ModelStatus::Ready);
            
            // Обновление счетчиков активных моделей
            if was_ready && !is_ready {
                system_state.active_models = system_state.active_models.saturating_sub(1);
            } else if !was_ready && is_ready {
                system_state.active_models += 1;
            }
        }
        
        model_states.insert(model_name.to_string(), state.clone());
        system_state.last_activity = Utc::now();
        
        info!("Updated model state: {} (status: {:?})", model_name, state.status);
        Ok(())
    }

    /// Получение состояния модели
    pub fn get_model_state(&self, model_name: &str) -> Option<ModelState> {
        self.model_states.read().get(model_name).cloned()
    }

    /// Получение всех состояний моделей
    pub fn get_all_model_states(&self) -> HashMap<String, ModelState> {
        self.model_states.read().clone()
    }

    /// Получение времени работы
    pub fn get_uptime(&self) -> std::time::Duration {
        let system_state = self.system_state.read();
        let now = Utc::now();
        (now - system_state.start_time).to_std().unwrap_or_default()
    }

    /// Проверка готовности системы
    pub fn is_ready(&self) -> bool {
        *self.is_initialized.read()
    }

    /// Получение состояния системы
    pub fn get_system_state(&self) -> SystemState {
        self.system_state.read().clone()
    }

    /// Обновление метрик системы
    pub fn update_system_metrics(&self, metrics: SystemMetrics) -> Result<(), AppError> {
        let mut system_state = self.system_state.write();
        system_state.system_metrics = metrics;
        system_state.last_activity = Utc::now();
        Ok(())
    }
} 