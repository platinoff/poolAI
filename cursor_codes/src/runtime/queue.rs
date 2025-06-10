use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub queue_type: String,
    pub max_size: u32,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub total_items: u64,
    pub processed_items: u64,
    pub failed_items: u64,
    pub current_items: u32,
    pub retried_items: u64,
    pub average_processing_time: Duration,
    pub last_operation_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMetrics {
    pub config: QueueConfig,
    pub stats: QueueStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub queue_id: String,
    pub data: String,
    pub priority: u32,
    pub created_at: DateTime<Utc>,
    pub retry_count: u32,
    pub status: String,
}

pub struct QueueSystem {
    queues: Arc<Mutex<HashMap<String, QueueMetrics>>>,
    items: Arc<Mutex<HashMap<String, QueueItem>>>,
}

impl QueueSystem {
    pub fn new() -> Self {
        Self {
            queues: Arc::new(Mutex::new(HashMap::new())),
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_queue(&self, config: QueueConfig) -> Result<(), String> {
        let mut queues = self.queues.lock().await;
        
        if queues.contains_key(&config.id) {
            return Err(format!("Queue '{}' already exists", config.id));
        }

        let metrics = QueueMetrics {
            config,
            stats: QueueStats {
                total_items: 0,
                processed_items: 0,
                failed_items: 0,
                current_items: 0,
                retried_items: 0,
                average_processing_time: Duration::from_secs(0),
                last_operation_time: None,
                last_error: None,
            },
        };

        queues.insert(metrics.config.id.clone(), metrics);
        info!("Added new queue: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_queue(&self, id: &str) -> Result<(), String> {
        let mut queues = self.queues.lock().await;
        let mut items = self.items.lock().await;
        
        if !queues.contains_key(id) {
            return Err(format!("Queue '{}' not found", id));
        }

        // Remove associated items
        items.retain(|_, i| i.queue_id != id);
        
        queues.remove(id);
        info!("Removed queue: {}", id);
        Ok(())
    }

    pub async fn enqueue_item(
        &self,
        queue_id: &str,
        data: &str,
        priority: u32,
    ) -> Result<String, String> {
        let mut queues = self.queues.lock().await;
        let mut items = self.items.lock().await;
        
        let queue = queues
            .get_mut(queue_id)
            .ok_or_else(|| format!("Queue '{}' not found", queue_id))?;

        if !queue.config.active {
            return Err("Queue is not active".to_string());
        }

        if queue.stats.current_items >= queue.config.max_size {
            return Err("Queue has reached maximum size".to_string());
        }

        let item = QueueItem {
            id: uuid::Uuid::new_v4().to_string(),
            queue_id: queue_id.to_string(),
            data: data.to_string(),
            priority,
            created_at: Utc::now(),
            retry_count: 0,
            status: "pending".to_string(),
        };

        items.insert(item.id.clone(), item.clone());
        queue.stats.current_items += 1;
        queue.stats.total_items += 1;

        info!(
            "Enqueued item: {} in queue: {} (priority: {})",
            item.id, queue_id, priority
        );
        Ok(item.id)
    }

    pub async fn dequeue_item(&self, queue_id: &str) -> Result<Option<QueueItem>, String> {
        let mut queues = self.queues.lock().await;
        let mut items = self.items.lock().await;
        
        let queue = queues
            .get_mut(queue_id)
            .ok_or_else(|| format!("Queue '{}' not found", queue_id))?;

        if !queue.config.active {
            return Err("Queue is not active".to_string());
        }

        // Find the highest priority item
        let item = items
            .values()
            .filter(|i| i.queue_id == queue_id && i.status == "pending")
            .max_by_key(|i| i.priority)
            .cloned();

        if let Some(item) = item {
            items.remove(&item.id);
            queue.stats.current_items -= 1;
            info!("Dequeued item: {} from queue: {}", item.id, queue_id);
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub async fn process_item(&self, item_id: &str) -> Result<(), String> {
        let mut queues = self.queues.lock().await;
        let mut items = self.items.lock().await;
        
        let item = items
            .get_mut(item_id)
            .ok_or_else(|| format!("Item '{}' not found", item_id))?;

        let queue = queues
            .get_mut(&item.queue_id)
            .ok_or_else(|| format!("Queue '{}' not found", item.queue_id))?;

        if !queue.config.active {
            return Err("Queue is not active".to_string());
        }

        let start_time = Utc::now();

        match self.process_item_data(item, &queue.config).await {
            Ok(_) => {
                item.status = "processed".to_string();
                queue.stats.processed_items += 1;
            }
            Err(e) => {
                if item.retry_count < queue.config.max_retries {
                    item.retry_count += 1;
                    item.status = "pending".to_string();
                    queue.stats.retried_items += 1;
                    info!(
                        "Retrying item: {} in queue: {} (attempt: {})",
                        item_id, item.queue_id, item.retry_count
                    );
                } else {
                    item.status = "failed".to_string();
                    queue.stats.failed_items += 1;
                    queue.stats.last_error = Some(e);
                }
            }
        }

        let processing_time = start_time.signed_duration_since(item.created_at);
        queue.stats.average_processing_time = Duration::from_secs(
            (queue.stats.average_processing_time.as_secs() + processing_time.num_seconds() as u64) / 2,
        );
        queue.stats.last_operation_time = Some(start_time);

        info!("Processed item: {}", item_id);
        Ok(())
    }

    async fn process_item_data(&self, item: &QueueItem, config: &QueueConfig) -> Result<(), String> {
        // Simulate item processing
        let is_successful = true;
        
        // Simulate processing delay
        tokio::time::sleep(config.retry_delay).await;
        
        if !is_successful {
            return Err("Item processing failed".to_string());
        }

        info!(
            "Processed item data: {} in queue: {}",
            item.id, item.queue_id
        );
        Ok(())
    }

    pub async fn get_queue(&self, id: &str) -> Result<QueueMetrics, String> {
        let queues = self.queues.lock().await;
        
        queues
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Queue '{}' not found", id))
    }

    pub async fn get_all_queues(&self) -> Vec<QueueMetrics> {
        let queues = self.queues.lock().await;
        queues.values().cloned().collect()
    }

    pub async fn get_active_queues(&self) -> Vec<QueueMetrics> {
        let queues = self.queues.lock().await;
        queues
            .values()
            .filter(|q| q.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_items(&self, queue_id: &str) -> Vec<QueueItem> {
        let items = self.items.lock().await;
        items
            .values()
            .filter(|i| i.queue_id == queue_id)
            .cloned()
            .collect()
    }

    pub async fn set_queue_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut queues = self.queues.lock().await;
        
        let queue = queues
            .get_mut(id)
            .ok_or_else(|| format!("Queue '{}' not found", id))?;

        queue.config.active = active;
        info!(
            "Queue '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_queue_config(&self, id: &str, new_config: QueueConfig) -> Result<(), String> {
        let mut queues = self.queues.lock().await;
        
        let queue = queues
            .get_mut(id)
            .ok_or_else(|| format!("Queue '{}' not found", id))?;

        queue.config = new_config;
        info!("Updated queue configuration: {}", id);
        Ok(())
    }
} 