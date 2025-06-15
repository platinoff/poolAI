use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub worker_type: String,
    pub max_tasks: u32,
    pub max_memory: u64,
    pub max_cpu: u32,
    pub active: bool,
    pub retry_count: u32,
    pub retry_delay: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub current_tasks: u32,
    pub memory_usage: u64,
    pub cpu_usage: u32,
    pub uptime: u64,
    pub last_task_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub total_retries: u64,
    pub failed_retries: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    pub config: WorkerConfig,
    pub stats: WorkerStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub worker_id: String,
    pub task_type: String,
    pub priority: u32,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub retry_count: u32,
    pub last_error: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
}

pub struct WorkerSystem {
    workers: Arc<Mutex<HashMap<String, WorkerMetrics>>>,
    tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl WorkerSystem {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(Mutex::new(HashMap::new())),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_worker(&self, config: WorkerConfig) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        if workers.contains_key(&config.id) {
            return Err(format!("Worker '{}' already exists", config.id));
        }

        // Validate worker configuration
        self.validate_worker_config(&config)?;

        let metrics = WorkerMetrics {
            config,
            stats: WorkerStats {
                total_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
                current_tasks: 0,
                memory_usage: 0,
                cpu_usage: 0,
                uptime: 0,
                last_task_time: None,
                last_error: None,
                total_retries: 0,
                failed_retries: 0,
            },
        };

        workers.insert(metrics.config.id.clone(), metrics);
        info!("Added new worker: {}", metrics.config.id);
        Ok(())
    }

    fn validate_worker_config(&self, config: &WorkerConfig) -> Result<(), String> {
        if config.max_tasks == 0 {
            return Err("max_tasks must be greater than 0".to_string());
        }
        if config.max_memory == 0 {
            return Err("max_memory must be greater than 0".to_string());
        }
        if config.max_cpu == 0 {
            return Err("max_cpu must be greater than 0".to_string());
        }
        if config.retry_delay == 0 {
            return Err("retry_delay must be greater than 0".to_string());
        }
        if config.timeout == 0 {
            return Err("timeout must be greater than 0".to_string());
        }
        Ok(())
    }

    pub async fn remove_worker(&self, id: &str) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        let mut tasks = self.tasks.lock().await;
        
        if !workers.contains_key(id) {
            return Err(format!("Worker '{}' not found", id));
        }

        // Remove associated tasks
        tasks.retain(|_, t| t.worker_id != id);
        
        workers.remove(id);
        info!("Removed worker: {}", id);
        Ok(())
    }

    pub async fn assign_task(
        &self,
        worker_id: &str,
        task_type: &str,
        priority: u32,
    ) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        let mut tasks = self.tasks.lock().await;
        
        let worker = workers
            .get_mut(worker_id)
            .ok_or_else(|| format!("Worker '{}' not found", worker_id))?;

        if !worker.config.active {
            return Err("Worker is not active".to_string());
        }

        if worker.stats.current_tasks >= worker.config.max_tasks {
            return Err("Worker has reached maximum tasks".to_string());
        }

        let deadline = Utc::now() + chrono::Duration::milliseconds(worker.config.timeout as i64);

        let task = Task {
            id: Uuid::new_v4().to_string(),
            worker_id: worker_id.to_string(),
            task_type: task_type.to_string(),
            priority,
            timestamp: Utc::now(),
            status: "pending".to_string(),
            retry_count: 0,
            last_error: None,
            deadline: Some(deadline),
        };

        tasks.insert(task.id.clone(), task.clone());
        worker.stats.current_tasks += 1;
        worker.stats.total_tasks += 1;

        info!(
            "Assigned task: {} to worker: {} (type: {}, priority: {})",
            task.id, worker_id, task_type, priority
        );
        Ok(())
    }

    pub async fn process_task(&self, task_id: &str) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        let mut tasks = self.tasks.lock().await;
        
        let task = tasks
            .get_mut(task_id)
            .ok_or_else(|| format!("Task '{}' not found", task_id))?;

        let worker = workers
            .get_mut(&task.worker_id)
            .ok_or_else(|| format!("Worker '{}' not found", task.worker_id))?;

        if !worker.config.active {
            return Err("Worker is not active".to_string());
        }

        let start_time = Utc::now();

        // Check if task has exceeded deadline
        if let Some(deadline) = task.deadline {
            if Utc::now() > deadline {
                task.status = "timeout".to_string();
                worker.stats.failed_tasks += 1;
                worker.stats.last_error = Some("Task exceeded deadline".to_string());
                return Err("Task exceeded deadline".to_string());
            }
        }

        match self.execute_task(task, &worker.config).await {
            Ok(_) => {
                task.status = "completed".to_string();
                worker.stats.completed_tasks += 1;
            }
            Err(e) => {
                if task.retry_count < worker.config.retry_count {
                    task.retry_count += 1;
                    task.last_error = Some(e.clone());
                    worker.stats.total_retries += 1;
                    
                    // Schedule retry
                    let retry_delay = worker.config.retry_delay * (1 << task.retry_count);
                    tokio::spawn({
                        let task_id = task.id.clone();
                        let worker_system = self.clone();
                        async move {
                            tokio::time::sleep(std::time::Duration::from_millis(retry_delay)).await;
                            if let Err(e) = worker_system.process_task(&task_id).await {
                                error!("Failed to retry task {}: {}", task_id, e);
                            }
                        }
                    });
                } else {
                    task.status = "failed".to_string();
                    worker.stats.failed_tasks += 1;
                    worker.stats.failed_retries += 1;
                    worker.stats.last_error = Some(e);
                }
            }
        }

        worker.stats.current_tasks -= 1;
        worker.stats.last_task_time = Some(start_time);
        info!("Processed task: {}", task_id);
        Ok(())
    }

    async fn execute_task(
        &self,
        task: &Task,
        config: &WorkerConfig,
    ) -> Result<(), String> {
        // Check resource limits
        if task.priority as u64 * 100 > config.max_memory {
            return Err("Task exceeds memory limit".to_string());
        }
        if task.priority as u32 * 10 > config.max_cpu {
            return Err("Task exceeds CPU limit".to_string());
        }

        // Simulate task execution
        let memory_usage = (task.priority as u64 * 100) % config.max_memory;
        let cpu_usage = (task.priority as u32 * 10) % config.max_cpu;
        
        // Simulate network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        info!(
            "Executed task: {} on worker: {} (type: {}, priority: {})",
            task.id, task.worker_id, task.task_type, task.priority
        );
        Ok(())
    }

    pub async fn update_memory_usage(&self, id: &str, memory_usage: u64) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        let worker = workers
            .get_mut(id)
            .ok_or_else(|| format!("Worker '{}' not found", id))?;

        if memory_usage > worker.config.max_memory {
            return Err("Memory usage exceeds maximum".to_string());
        }

        worker.stats.memory_usage = memory_usage;
        info!("Updated memory usage for worker: {} to {}", id, memory_usage);
        Ok(())
    }

    pub async fn update_cpu_usage(&self, id: &str, cpu_usage: u32) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        let worker = workers
            .get_mut(id)
            .ok_or_else(|| format!("Worker '{}' not found", id))?;

        if cpu_usage > worker.config.max_cpu {
            return Err("CPU usage exceeds maximum".to_string());
        }

        worker.stats.cpu_usage = cpu_usage;
        info!("Updated CPU usage for worker: {} to {}", id, cpu_usage);
        Ok(())
    }

    pub async fn get_worker(&self, id: &str) -> Result<WorkerMetrics, String> {
        let workers = self.workers.lock().await;
        
        workers
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Worker '{}' not found", id))
    }

    pub async fn get_all_workers(&self) -> Vec<WorkerMetrics> {
        let workers = self.workers.lock().await;
        workers.values().cloned().collect()
    }

    pub async fn get_active_workers(&self) -> Vec<WorkerMetrics> {
        let workers = self.workers.lock().await;
        workers
            .values()
            .filter(|w| w.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_tasks(&self, worker_id: &str) -> Vec<Task> {
        let tasks = self.tasks.lock().await;
        tasks
            .values()
            .filter(|t| t.worker_id == worker_id)
            .cloned()
            .collect()
    }

    pub async fn set_worker_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        let worker = workers
            .get_mut(id)
            .ok_or_else(|| format!("Worker '{}' not found", id))?;

        worker.config.active = active;
        info!(
            "Worker '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_worker_config(&self, id: &str, new_config: WorkerConfig) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        
        let worker = workers
            .get_mut(id)
            .ok_or_else(|| format!("Worker '{}' not found", id))?;

        worker.config = new_config;
        info!("Updated worker configuration: {}", id);
        Ok(())
    }
} 