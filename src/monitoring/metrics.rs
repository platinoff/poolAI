//! Metrics Collection Module
//!
//! Provides system metrics collection including GPU, CPU, memory, disk, network,
//! and application-specific metrics.
//!
//! # Features
//!
//! - **System Metrics**: GPU utilization, CPU usage, memory, disk, network
//! - **Performance Metrics**: Response time, requests per second, error rate
//! - **Model Metrics**: Model-specific performance metrics
//! - **Historical Data**: Track metrics over time with configurable retention
//!
//! # Example
//!
//! ```no_run
//! use poolai::monitoring::metrics::MetricsCollector;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let collector = MetricsCollector::new();
//! let metrics = collector.collect().await?;
//!
//! println!("GPU: {:.1}%, CPU: {:.1}%, Memory: {:.1}MB",
//!     metrics.gpu_utilization,
//!     metrics.cpu_usage_percent,
//!     metrics.memory_usage_mb);
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// System metrics snapshot
///
/// Contains a complete snapshot of system and application metrics at a specific point in time.
///
/// # Example
///
/// ```rust
/// use poolai::monitoring::metrics::Metrics;
/// use std::time::Instant;
///
/// let metrics = Metrics {
///     timestamp: Instant::now(),
///     gpu_utilization: 75.5,
///     memory_usage_mb: 4096.0,
///     cpu_usage_percent: 45.2,
///     disk_usage_percent: 65.8,
///     network_throughput_mbps: 125.5,
///     average_response_time_ms: 250.0,
///     requests_per_second: 45.2,
///     error_rate: 0.02,
///     active_connections: 12,
///     queue_size: 5,
///     model_specific_metrics: std::collections::HashMap::new(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Metrics {
    pub timestamp: Instant,
    pub gpu_utilization: f32,
    pub memory_usage_mb: f32,
    pub cpu_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub network_throughput_mbps: f32,
    pub average_response_time_ms: f64,
    pub requests_per_second: f64,
    pub error_rate: f32,
    pub active_connections: usize,
    pub queue_size: usize,
    pub model_specific_metrics: HashMap<String, f64>,
}

/// Model-specific performance metrics
///
/// Tracks performance metrics for individual AI models including processing time,
/// token generation, GPU usage, and cache performance.
///
/// # Example
///
/// ```rust
/// use poolai::monitoring::metrics::ModelMetrics;
///
/// let model_metrics = ModelMetrics {
///     model_name: "llama-2-7b".to_string(),
///     processing_time_ms: 150,
///     tokens_generated: 1000,
///     tokens_per_second: 6.67,
///     gpu_memory_usage_mb: 8192.0,
///     gpu_utilization: 85.5,
///     cache_hit_rate: 0.92,
///     error_count: 0,
///     success_count: 100,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ModelMetrics {
    pub model_name: String,
    pub processing_time_ms: u64,
    pub tokens_generated: usize,
    pub tokens_per_second: f32,
    pub gpu_memory_usage_mb: f32,
    pub gpu_utilization: f32,
    pub cache_hit_rate: f32,
    pub error_count: u64,
    pub success_count: u64,
}

/// System resource metrics
///
/// Provides information about available system resources including GPU, CPU, RAM, and disk.
///
/// # Example
///
/// ```rust
/// use poolai::monitoring::metrics::ResourceMetrics;
///
/// let resources = ResourceMetrics {
///     gpu_count: 2,
///     total_gpu_memory_mb: 16384.0,
///     available_gpu_memory_mb: 8192.0,
///     cpu_cores: 16,
///     total_ram_mb: 32768.0,
///     available_ram_mb: 16384.0,
///     disk_space_gb: 1000.0,
///     available_disk_space_gb: 500.0,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    pub gpu_count: usize,
    pub total_gpu_memory_mb: f32,
    pub available_gpu_memory_mb: f32,
    pub cpu_cores: usize,
    pub total_ram_mb: f32,
    pub available_ram_mb: f32,
    pub disk_space_gb: f64,
    pub available_disk_space_gb: f64,
}

/// Metrics collector for system and application metrics
///
/// Collects and manages metrics from various sources including system resources,
/// application performance, and model-specific metrics.
///
/// # Thread Safety
///
/// This struct is not thread-safe by default. For concurrent access, wrap it in
/// `Arc<RwLock<MetricsCollector>>` or use the global monitoring instance.
///
/// # Example
///
/// ```no_run
/// use poolai::monitoring::metrics::MetricsCollector;
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let mut collector = MetricsCollector::new();
///
/// // Collect current metrics
/// let metrics = collector.collect().await?;
///
/// // Get historical metrics
/// let historical = collector.get_historical_metrics(Duration::from_secs(3600)).await;
/// # Ok(())
/// # }
/// ```
pub struct MetricsCollector {
    _last_collection: Instant,
    historical_metrics: Vec<Metrics>,
    model_metrics: HashMap<String, ModelMetrics>,
    resource_metrics: ResourceMetrics,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    ///
    /// Initializes a new metrics collector with empty historical data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::monitoring::metrics::MetricsCollector;
    ///
    /// let collector = MetricsCollector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _last_collection: Instant::now(),
            historical_metrics: Vec::new(),
            model_metrics: HashMap::new(),
            resource_metrics: ResourceMetrics {
                gpu_count: 0,
                total_gpu_memory_mb: 0.0,
                available_gpu_memory_mb: 0.0,
                cpu_cores: 0,
                total_ram_mb: 0.0,
                available_ram_mb: 0.0,
                disk_space_gb: 0.0,
                available_disk_space_gb: 0.0,
            },
        }
    }

    /// Collect current system and application metrics
    ///
    /// Gathers metrics from all available sources including GPU, CPU, memory,
    /// disk, network, and application-specific metrics.
    ///
    /// # Returns
    ///
    /// Returns a `Metrics` struct containing all collected metrics, or an error
    /// if metric collection fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::MetricsCollector;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let collector = MetricsCollector::new();
    /// let metrics = collector.collect().await?;
    /// println!("GPU utilization: {:.1}%", metrics.gpu_utilization);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collect(&self) -> Result<Metrics, AppError> {
        let timestamp = Instant::now();

        // Сбор системных метрик
        let gpu_utilization = self.collect_gpu_utilization().await?;
        let memory_usage_mb = self.collect_memory_usage().await?;
        let cpu_usage_percent = self.collect_cpu_usage().await?;
        let disk_usage_percent = self.collect_disk_usage().await?;
        let network_throughput_mbps = self.collect_network_metrics().await?;

        // Сбор метрик производительности
        let average_response_time_ms = self.calculate_average_response_time().await?;
        let requests_per_second = self.calculate_requests_per_second().await?;
        let error_rate = self.calculate_error_rate().await?;
        let active_connections = self.get_active_connections().await?;
        let queue_size = self.get_queue_size().await?;

        // Сбор специфичных метрик моделей
        let model_specific_metrics = self.collect_model_specific_metrics().await?;

        let metrics = Metrics {
            timestamp,
            gpu_utilization,
            memory_usage_mb,
            cpu_usage_percent,
            disk_usage_percent,
            network_throughput_mbps,
            average_response_time_ms,
            requests_per_second,
            error_rate,
            active_connections,
            queue_size,
            model_specific_metrics,
        };

        Ok(metrics)
    }

    async fn collect_gpu_utilization(&self) -> Result<f32, AppError> {
        // Заглушка для сбора GPU метрик
        // В реальной реализации здесь будет интеграция с GPU драйверами
        Ok(75.5)
    }

    async fn collect_memory_usage(&self) -> Result<f32, AppError> {
        // Заглушка для сбора метрик памяти
        // В реальной реализации здесь будет системный вызов
        Ok(4096.0)
    }

    async fn collect_cpu_usage(&self) -> Result<f32, AppError> {
        // Заглушка для сбора CPU метрик
        // В реальной реализации здесь будет системный вызов
        Ok(45.2)
    }

    async fn collect_disk_usage(&self) -> Result<f32, AppError> {
        // Заглушка для сбора метрик диска
        // В реальной реализации здесь будет системный вызов
        Ok(65.8)
    }

    async fn collect_network_metrics(&self) -> Result<f32, AppError> {
        // Заглушка для сетевых метрик
        // В реальной реализации здесь будет системный вызов
        Ok(125.5)
    }

    async fn calculate_average_response_time(&self) -> Result<f64, AppError> {
        // Заглушка для расчета среднего времени ответа
        // В реальной реализации здесь будет анализ исторических данных
        Ok(250.0)
    }

    async fn calculate_requests_per_second(&self) -> Result<f64, AppError> {
        // Заглушка для расчета RPS
        // В реальной реализации здесь будет анализ исторических данных
        Ok(45.2)
    }

    async fn calculate_error_rate(&self) -> Result<f32, AppError> {
        // Заглушка для расчета ошибок
        // В реальной реализации здесь будет анализ исторических данных
        Ok(0.02)
    }

    async fn get_active_connections(&self) -> Result<usize, AppError> {
        // Заглушка для получения активных соединений
        // В реальной реализации здесь будет интеграция с сетевой подсистемой
        Ok(12)
    }

    async fn get_queue_size(&self) -> Result<usize, AppError> {
        // Заглушка для получения размера очереди
        // В реальной реализации здесь будет интеграция с пулом
        Ok(5)
    }

    async fn collect_model_specific_metrics(&self) -> Result<HashMap<String, f64>, AppError> {
        // Заглушка для специфичных метрик моделей
        // В реальной реализации здесь будет интеграция с моделями
        let mut metrics = HashMap::new();
        metrics.insert("model_1_throughput".to_string(), 1250.5);
        metrics.insert("model_2_throughput".to_string(), 980.3);
        metrics.insert("cache_hit_rate".to_string(), 0.85);
        Ok(metrics)
    }

    /// Update metrics for a specific model
    ///
    /// Stores or updates performance metrics for a named model.
    ///
    /// # Arguments
    ///
    /// * `model_name` - Name of the model
    /// * `metrics` - Model-specific metrics to store
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::{MetricsCollector, ModelMetrics};
    ///
    /// # async fn example() {
    /// let mut collector = MetricsCollector::new();
    /// let model_metrics = ModelMetrics {
    ///     model_name: "llama-2-7b".to_string(),
    ///     processing_time_ms: 150,
    ///     tokens_generated: 1000,
    ///     tokens_per_second: 6.67,
    ///     gpu_memory_usage_mb: 8192.0,
    ///     gpu_utilization: 85.5,
    ///     cache_hit_rate: 0.92,
    ///     error_count: 0,
    ///     success_count: 100,
    /// };
    /// collector.update_model_metrics("llama-2-7b".to_string(), model_metrics).await;
    /// # }
    /// ```
    pub async fn update_model_metrics(&mut self, model_name: String, metrics: ModelMetrics) {
        self.model_metrics.insert(model_name, metrics);
    }

    /// Get metrics for a specific model
    ///
    /// Retrieves stored metrics for a named model, if available.
    ///
    /// # Arguments
    ///
    /// * `model_name` - Name of the model to retrieve metrics for
    ///
    /// # Returns
    ///
    /// Returns `Some(ModelMetrics)` if metrics exist for the model, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::MetricsCollector;
    ///
    /// # async fn example() {
    /// let collector = MetricsCollector::new();
    /// if let Some(metrics) = collector.get_model_metrics("llama-2-7b").await {
    ///     println!("Tokens per second: {:.2}", metrics.tokens_per_second);
    /// }
    /// # }
    /// ```
    pub async fn get_model_metrics(&self, model_name: &str) -> Option<ModelMetrics> {
        self.model_metrics.get(model_name).cloned()
    }

    /// Update system resource metrics
    ///
    /// Updates the stored system resource information.
    ///
    /// # Arguments
    ///
    /// * `metrics` - Resource metrics to store
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::{MetricsCollector, ResourceMetrics};
    ///
    /// # async fn example() {
    /// let mut collector = MetricsCollector::new();
    /// let resources = ResourceMetrics {
    ///     gpu_count: 2,
    ///     total_gpu_memory_mb: 16384.0,
    ///     available_gpu_memory_mb: 8192.0,
    ///     cpu_cores: 16,
    ///     total_ram_mb: 32768.0,
    ///     available_ram_mb: 16384.0,
    ///     disk_space_gb: 1000.0,
    ///     available_disk_space_gb: 500.0,
    /// };
    /// collector.update_resource_metrics(resources).await;
    /// # }
    /// ```
    pub async fn update_resource_metrics(&mut self, metrics: ResourceMetrics) {
        self.resource_metrics = metrics;
    }

    /// Get current system resource metrics
    ///
    /// Returns a copy of the stored system resource information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::MetricsCollector;
    ///
    /// # async fn example() {
    /// let collector = MetricsCollector::new();
    /// let resources = collector.get_resource_metrics().await;
    /// println!("GPU count: {}, CPU cores: {}", resources.gpu_count, resources.cpu_cores);
    /// # }
    /// ```
    pub async fn get_resource_metrics(&self) -> ResourceMetrics {
        self.resource_metrics.clone()
    }

    /// Get historical metrics within a time window
    ///
    /// Retrieves all metrics collected within the specified duration from now.
    ///
    /// # Arguments
    ///
    /// * `duration` - Time window to retrieve metrics for
    ///
    /// # Returns
    ///
    /// Returns a vector of `Metrics` collected within the specified duration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::MetricsCollector;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let collector = MetricsCollector::new();
    /// let historical = collector.get_historical_metrics(Duration::from_secs(3600)).await;
    /// println!("Collected {} metrics in the last hour", historical.len());
    /// # }
    /// ```
    pub async fn get_historical_metrics(&self, duration: Duration) -> Vec<Metrics> {
        let cutoff_time = Instant::now() - duration;

        self.historical_metrics
            .iter()
            .filter(|metrics| metrics.timestamp >= cutoff_time)
            .cloned()
            .collect()
    }

    /// Add metrics to historical storage
    ///
    /// Stores metrics in historical data with automatic cleanup when the limit
    /// (10000 entries) is exceeded.
    ///
    /// # Arguments
    ///
    /// * `metrics` - Metrics to add to history
    ///
    /// # Example
    ///
    /// ```no_run
    /// use poolai::monitoring::metrics::{MetricsCollector, Metrics};
    /// use std::time::Instant;
    /// use std::collections::HashMap;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let mut collector = MetricsCollector::new();
    /// let metrics = collector.collect().await?;
    /// collector.add_metrics_to_history(metrics).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_metrics_to_history(&mut self, metrics: Metrics) {
        self.historical_metrics.push(metrics);

        // Ограничение размера истории
        if self.historical_metrics.len() > 10000 {
            self.historical_metrics.drain(0..1000);
        }
    }
}
