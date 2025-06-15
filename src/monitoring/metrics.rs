use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_type: String,
    pub unit: String,
    pub aggregation: String,
    pub retention: Duration,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStats {
    pub total_samples: u64,
    pub current_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub sum_value: f64,
    pub average_value: f64,
    pub last_sample_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricMetrics {
    pub config: MetricConfig,
    pub stats: MetricStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub id: String,
    pub metric_id: String,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

pub struct MetricsSystem {
    metrics: Arc<Mutex<HashMap<String, MetricMetrics>>>,
    samples: Arc<Mutex<HashMap<String, Sample>>>,
}

impl MetricsSystem {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            samples: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_metric(&self, config: MetricConfig) -> Result<(), String> {
        let mut metrics = self.metrics.lock().await;
        
        if metrics.contains_key(&config.id) {
            return Err(format!("Metric '{}' already exists", config.id));
        }

        let metrics_data = MetricMetrics {
            config,
            stats: MetricStats {
                total_samples: 0,
                current_value: 0.0,
                min_value: f64::MAX,
                max_value: f64::MIN,
                sum_value: 0.0,
                average_value: 0.0,
                last_sample_time: None,
                last_error: None,
            },
        };

        metrics.insert(metrics_data.config.id.clone(), metrics_data);
        info!("Added new metric: {}", metrics_data.config.id);
        Ok(())
    }

    pub async fn remove_metric(&self, id: &str) -> Result<(), String> {
        let mut metrics = self.metrics.lock().await;
        let mut samples = self.samples.lock().await;
        
        if !metrics.contains_key(id) {
            return Err(format!("Metric '{}' not found", id));
        }

        // Remove associated samples
        samples.retain(|_, s| s.metric_id != id);
        
        metrics.remove(id);
        info!("Removed metric: {}", id);
        Ok(())
    }

    pub async fn record_sample(
        &self,
        metric_id: &str,
        value: f64,
        labels: HashMap<String, String>,
    ) -> Result<String, String> {
        let mut metrics = self.metrics.lock().await;
        let mut samples = self.samples.lock().await;
        
        let metric = metrics
            .get_mut(metric_id)
            .ok_or_else(|| format!("Metric '{}' not found", metric_id))?;

        if !metric.config.active {
            return Err("Metric is not active".to_string());
        }

        let sample = Sample {
            id: uuid::Uuid::new_v4().to_string(),
            metric_id: metric_id.to_string(),
            timestamp: Utc::now(),
            value,
            labels,
        };

        samples.insert(sample.id.clone(), sample.clone());
        metric.stats.total_samples += 1;
        metric.stats.current_value = value;
        metric.stats.min_value = metric.stats.min_value.min(value);
        metric.stats.max_value = metric.stats.max_value.max(value);
        metric.stats.sum_value += value;
        metric.stats.average_value = metric.stats.sum_value / metric.stats.total_samples as f64;
        metric.stats.last_sample_time = Some(sample.timestamp);

        info!(
            "Recorded sample: {} for metric: {} (value: {})",
            sample.id, metric_id, value
        );
        Ok(sample.id)
    }

    pub async fn get_metric(&self, id: &str) -> Result<MetricMetrics, String> {
        let metrics = self.metrics.lock().await;
        
        metrics
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Metric '{}' not found", id))
    }

    pub async fn get_all_metrics(&self) -> Vec<MetricMetrics> {
        let metrics = self.metrics.lock().await;
        metrics.values().cloned().collect()
    }

    pub async fn get_active_metrics(&self) -> Vec<MetricMetrics> {
        let metrics = self.metrics.lock().await;
        metrics
            .values()
            .filter(|m| m.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_samples(
        &self,
        metric_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Vec<Sample> {
        let samples = self.samples.lock().await;
        samples
            .values()
            .filter(|s| {
                s.metric_id == metric_id
                    && start_time.map_or(true, |t| s.timestamp >= t)
                    && end_time.map_or(true, |t| s.timestamp <= t)
            })
            .cloned()
            .collect()
    }

    pub async fn set_metric_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut metrics = self.metrics.lock().await;
        
        let metric = metrics
            .get_mut(id)
            .ok_or_else(|| format!("Metric '{}' not found", id))?;

        metric.config.active = active;
        info!(
            "Metric '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_metric_config(&self, id: &str, new_config: MetricConfig) -> Result<(), String> {
        let mut metrics = self.metrics.lock().await;
        
        let metric = metrics
            .get_mut(id)
            .ok_or_else(|| format!("Metric '{}' not found", id))?;

        metric.config = new_config;
        info!("Updated metric configuration: {}", id);
        Ok(())
    }

    pub async fn aggregate_metric(
        &self,
        metric_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<HashMap<String, f64>, String> {
        let samples = self.get_samples(metric_id, Some(start_time), Some(end_time)).await;
        
        if samples.is_empty() {
            return Err("No samples found for the specified time range".to_string());
        }

        let mut result = HashMap::new();
        result.insert("count".to_string(), samples.len() as f64);
        result.insert("sum".to_string(), samples.iter().map(|s| s.value).sum());
        result.insert(
            "average".to_string(),
            samples.iter().map(|s| s.value).sum::<f64>() / samples.len() as f64,
        );
        result.insert(
            "min".to_string(),
            samples.iter().map(|s| s.value).fold(f64::MAX, f64::min),
        );
        result.insert(
            "max".to_string(),
            samples.iter().map(|s| s.value).fold(f64::MIN, f64::max),
        );

        Ok(result)
    }
} 